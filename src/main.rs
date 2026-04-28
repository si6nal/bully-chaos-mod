use std::collections::HashMap;
use std::ops::Div;
use std::time::{Duration, Instant};
use log::{error, info, warn};
use strum::IntoEnumIterator;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use crate::game::bully::GameData;
use crate::game::events::{ChaosEvents, TwitchClientData};
use crate::settings::event_settings::EventSettings;
use crate::settings::twitch_settings::TwitchSettings;
use crate::windows::{processes, window};

mod windows;
mod game;
mod memory;
mod settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize logger
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();

    // load event settings
    let event_settings = EventSettings::get();

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

    // create twitch client data struct for events
    let twitch_client_data = TwitchClientData {
        irc_client: client.clone(),
        channel: twitch_settings.username.clone(),
    };

    // join channel & send connection message
    client.join(twitch_settings.username.clone()).expect("failed to join channel.");
    client.say(twitch_settings.username.clone(), format!(
        "[Bully Chaos Mod] Successfully connected. Enabled events: {}, version: {}",
        ChaosEvents::iter().len(),
        env!("CARGO_PKG_VERSION"),
    )).await.expect("Failed to say connection message.");

    // wrap getting game data in a loop, so if the game crashes this won't have to be re-opened
    loop {
        // get game data
        let game_data = GameData::get().await;

        // counter for how many events have been executed
        let mut event_counter: u32 = 0;

        // time for when reduced voting timer started
        let mut reduced_voting_timer_start: Option<Instant> = None;

        // start chaos voting
        loop {
            // get chaos events
            let events = ChaosEvents::get_events();

            // pause voting
            if event_counter > 0 && event_counter % event_settings.meta_no_chaos_roo == 0 {
                let _ = client.say(twitch_settings.username.clone(), String::from("Voting disabled for 1 minute (meta).")).await;
                tokio::time::sleep(Duration::from_secs(60)).await;
            }

            // announce chaos events
            let _ = client.say(twitch_settings.username.clone(), String::from("Chaos voting started...")).await;
            for (idx, event) in events.iter().enumerate() {
                let _ = client.say(twitch_settings.username.clone(), format!("{}. {}", idx + 1, event.as_str())).await;
                tokio::time::sleep(Duration::from_millis(90)).await;
            }

            // create hashmap for each chatter for storing votes
            let mut votes: HashMap<String, u8> = HashMap::new();

            /* testing */ {
                //tokio::time::sleep(Duration::from_secs(5)).await;
                //game::mods::health::update_health(&game_data, 99999999f32);
                //game::mods::ammo::give_all_ammo(&game_data);
                //game::mods::location::slowness(&game_data).await;
                //game::mods::trouble_meter::trouble_health(&game_data).await;
            }

            // check if we should apply the reduced voting timer
            if event_counter > 0 && event_counter % event_settings.meta_extra_chaos_roo == 0 {
                reduced_voting_timer_start = Some(Instant::now());
                let _ = client.say(twitch_settings.username.clone(), String::from("More chaos enabled for 1 minute (meta).")).await;
            }

            // sleep for voting period, account for meta time modifier
            let adjusted_voting_time = if let Some(timer_start) = reduced_voting_timer_start {
                // check if we should disable the reduced timer
                if timer_start.elapsed().as_secs() >= 60 {
                    reduced_voting_timer_start = None;
                    let _ = client.say(twitch_settings.username.clone(), String::from("More chaos disabled.")).await;
                }

                voting_time.div(2)
            } else {
                voting_time
            };
            tokio::time::sleep(adjusted_voting_time).await;

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
                            // todo: get first character & parse that, allows for emotes
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
            
            // check if the game is out of focus
            if game_data.process_id != window::get_focused_window_process_id() {
                loop {
                    // sleep indefinitely until the game is focused again
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    info!("game is out-of-focus, event execution paused.");
                    
                    // check if the game is focused
                    if game_data.process_id != window::get_focused_window_process_id() {
                        break;
                    }
                }
            }

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
                event.execute(&game_data, None).await;

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
            match event {
                ChaosEvents::FakeCrash | ChaosEvents::RealCrash => unreachable!(),
                ChaosEvents::MetaMoreChaos => reduced_voting_timer_start = Some(Instant::now()),
                _ => event.execute(&game_data, Some(&twitch_client_data)).await,
            }

            // increment event counter
            event_counter += 1;
        }

        // warn that chaos loop exited
        warn!("Bully game handle closed.");
        let _ = client.say(twitch_settings.username.clone(), String::from("Bully game handle closed, voting paused.")).await;
        tokio::time::sleep(Duration::from_secs(4)).await;
    }
}
