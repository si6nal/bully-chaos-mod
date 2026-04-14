use std::collections::HashMap;
use std::time::Duration;
use log::{error, info, warn};
use strum::IntoEnumIterator;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use crate::game::bully::GameData;
use crate::game::events::ChaosEvents;
use crate::game::mods::{health, location};
use crate::settings::twitch_settings::TwitchSettings;
use crate::windows::processes;

mod windows;
mod game;
mod memory;
mod settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize logger
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();

    // load twitch settings
    let twitch_settings = TwitchSettings::get().expect("update twitch settings file.");
    let voting_time = Duration::from_secs(twitch_settings.voting_time);

    // verify length of oauth token
    if twitch_settings.oauth_token.len() != 30 {
        error!("invalid oauth token length, the oauth token length must be 30.");
        return Ok(());
    }

    // create twitch config
    // https://twitchtokengenerator.com/
    let cfg = ClientConfig::new_simple(
        StaticLoginCredentials::new(
            twitch_settings.username.clone(),
            Some(twitch_settings.oauth_token)
        )
    );
    let (mut incoming_messages, client) = TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(cfg);

    /*// create channel for incoming messages
    let (tx_messages, mut rx_messages): (UnboundedSender<ServerMessage>, UnboundedReceiver<ServerMessage>) = mpsc::unbounded_channel();

    // create task that consumes messages & sends them to the local sender & receiver channels
    tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            let _ = tx_messages.send(message);
        }
    });*/

    // join channel & send connection message
    client.join(twitch_settings.username.clone()).expect("failed to join channel.");
    client.say(twitch_settings.username.clone(), format!("[Bully Chaos Mod] Successfully connected. Enabled events: {}", ChaosEvents::iter().len())).await.expect("Failed to say connection message.");

    // wrap getting game data in a loop, so if the game crashes this won't have to be re-opened
    loop {
        // get game data
        let game_data = GameData::get().await;

        // start chaos voting
        // todo: (meta) no chaos 1 minute every x(30) events
        // todo: (meta) reduced voting timer for 1 minute every x(50) events
        loop {
            // get chaos events
            let events = ChaosEvents::get_events();

            // announce chaos events
            let _ = client.say(twitch_settings.username.clone(), String::from("Chaos voting started...")).await;
            for (idx, event) in events.iter().enumerate() {
                let _ = client.say(twitch_settings.username.clone(), format!("{}. {}", idx + 1, event.as_str())).await;
                tokio::time::sleep(Duration::from_millis(90)).await;
            }

            // create hashmap for each chatter for storing votes
            let mut votes: HashMap<String, u8> = HashMap::new();

            /* testing */ {
                //info!("{:?}", memory::memory::read_float(game_data.handle, game_offsets::get_offset(game_data.handle, game_data.player_offset, game_offsets::PLAYER_HEALTH_OFFSET).unwrap()));
                //health::heal(&game_data);
                //ammo::give_all_ammo(&game_data);
                //location::fake_sky_tp(&game_data).await;
                //trouble_meter::max_trouble(&game_data).await;
                //input::phoon(&game_data).await;
            }

            // sleep for voting period
            tokio::time::sleep(voting_time).await;

            // read messages since last read period
            while let Ok(msg) = incoming_messages.try_recv() {
                // handle messages
                match msg {
                    ServerMessage::Privmsg(msg) => {
                        // get message text
                        let msg_txt = msg.message_text;

                        // check if message is a voting option
                        if msg_txt.len() == 1 {
                            // convert message to u8
                            if let Ok(vote_option) = msg_txt.parse::<u8>() {
                                // make sure the vote option exists
                                if vote_option > 0 && vote_option <= 4 {
                                    // add or update sender's vote option
                                    *votes.entry(msg.sender.login).or_insert(vote_option) = vote_option;
                                }
                            }
                        }
                    },
                    _ => {} // ignore messages that aren't chat messages
                }
            }

            // todo: sleep while game isn't focused

            // check if votes is empty, if so just choose a random event
            let (event, event_votes): (ChaosEvents, Option<usize>) = if votes.is_empty() {
                info!("choosing random event, no votes were cast.");
                match events.get(rand::random_range(0..3)) {
                    Some(event) => (event.clone(), None),
                    None => (ChaosEvents::Nothing, None)
                }
            } else {
                // create hashmap for event vote counting
                let mut vote_counts: HashMap<u8, usize> = HashMap::new();

                // count votes
                for (_, vote) in votes {
                    // add vote or increment
                    *vote_counts.entry(vote).or_insert(1) += 1;
                }

                // get event with the highest votes
                match vote_counts.iter().max_by_key(|&(_, v)| v).map(|(&k, _)| k) {
                    Some(event_winner) => {
                        // get event
                        match events.get((event_winner as usize - 1).max(0)) {
                            Some(event) => {
                                info!("selected event: {}", event.as_str());

                                // get votes for event
                                match vote_counts.get(&event_winner) {
                                    Some(votes) => (event.clone(), Some(*votes - 1)),
                                    None => (event.clone(), None)
                                }
                            },
                            None => {
                                warn!("failed to select event: {}", event_winner);
                                (ChaosEvents::Nothing, None)
                            }
                        }
                    },
                    None => {
                        warn!("failed to get the event with the highest votes.");
                        (ChaosEvents::Nothing, None)
                    }
                }
            };

            // check if game is still running before applying event
            if !processes::is_process_active(game_data.handle) {
                break;
            }

            // apply crash events before message
            if event == ChaosEvents::FakeCrash || event == ChaosEvents::RealCrash {
                // execute crash event
                info!("executing crash event...");
                event.execute(&game_data).await;

                // break from game loop if it was a real crash event, this prevents an extra voting phase beginning
                if event == ChaosEvents::RealCrash {
                    break;
                }
            }

            // send message of selected event
            let event_msg = match event_votes {
                Some(event_vote_count) => format!("Chosen event: {} ({} votes)", event.as_str(), event_vote_count),
                None => format!("Chosen event: {}", event.as_str()),
            };
            info!("{}", event_msg);
            let _ = client.say(twitch_settings.username.clone(), event_msg).await;

            // apply other events
            if event != ChaosEvents::FakeCrash && event != ChaosEvents::RealCrash {
                event.execute(&game_data).await;
            }
        }

        // warn that chaos loop exited
        warn!("Bully game handle closed.");
        let _ = client.say(twitch_settings.username.clone(), String::from("Bully game handle closed, voting paused.")).await;
    }
}
