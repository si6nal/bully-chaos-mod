use std::time::Duration;
use log::info;
use rand::prelude::IteratorRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use twitch_irc::{SecureTCPTransport, TwitchIRCClient};
use twitch_irc::login::StaticLoginCredentials;
use crate::game::bully::GameData;
use crate::game::mods::{ammo, health, location, money, trouble_meter, win_api};
use crate::windows::{processes, window};

// events that are commented out aren't implemented
// todo: add weight to events
#[derive(EnumIter, Clone, PartialEq, Debug)]
pub enum ChaosEvents {
    /* =========== */
    /* META EVENTS */
    /* =========== */
    Nothing, // does nothing (skips vote)
    RandomEvent, // selects another event
    MetaNoChaos, // no chaos for 1 minute
    MetaMoreChaos, // more chaos for 1 minute
    //MetaExecutePreviousEvents, // executes the last 3 events

    /* ============= */
    /* MEMORY EVENTS */
    /* ============= */
    RemoveMoney, // money = 0
    CheckBounced, // money = 25
    SpareChange, // adds a random amount of money ($.01-$10.00)
    MaxMoney, // money = i32 max ~ capped at 1m

    Invincibility, // 30sec setting health to 100
    Heal, // health = 100
    NoHealth, // health = 1
    Suicide, // health = 0

    MostWanted, // max trouble meter (30 sec)
    WantedHealth, // sets trouble meter to health (30 sec)
    RemoveWanted, // wanted level = 0

    //ExtraKnockback, // checks if the player has taken damage & applies extra knockback

    RemoveAllAmmo, // removes all ammo
    GiveAllAmmo, // gives max ammo for weapons

    Sisyphus, // teleports the player back to their original pos if they move more than 3 units (10-18 sec)
    SonarSisyphus, // teleports the player back to their original pos with an increasing delay (10-18 sec)

    Speed, // duplicates moving distance (30 sec)
    SpeedFaster, // duplicates moving distance at a faster rate (15 sec)
    Slowness, // slightly teleports the player backwards
    MaxJump, // the Speed event but vertical, gives invincibility in the air until the player is back on the ground (30 sec)
    NoJumping, // sets z value to the last z value before space was pressed (30 sec)
    Freeze, // stops the player from moving (10 sec)

    HomeSweetHome, // teleport back to the dorms
    RandomTp, // teleports to random **pre-defined** coordinates of in-game locations | coordinates aren't pre-defined, too lazy to get list of coordinates
    FakeRandomTp, // teleports player back to original location after a random tp (5 sec delay)
    SkyTp, // teleports the player into the sky
    FakeSkyTp, // teleports the player into the sky & back onto the ground before dying
    HellTp, // teleports the player into the ground
    BusTp, // teleports to the closest bus stop
    ReverseGravity, // determines the current gravity & applies it oppositely (10 sec)
    Phoon, // makes the player jump (30 sec)
    OppositeInput, // applies movement in the opposite direction (30 sec)
    Flight, // sets z value to 25, allows x,y movement (30 sec)
    //MapDriveBy, // teleports the player around the map in a circle, pressing space ends the circle pattern and lets the player fall (and live)
    //OppositeMapSideTp, // teleports the player to the opposite side of the map
    DrunkSpeed, // speed but opposite direction (30 sec)
    RandomForce, // randomly moves the character, 2-8sec input delay (30 sec)
    DisabledMovementAxis, // disables a random axis the player can move along (15 sec)

    /* ================ */
    /* RENDERING EVENTS */
    /* ================ */
    //TiredJimmy, // adds a blinking effect (30 sec)
    //RandomPicture, // choose a random picture to render in-game from a pictures folder alongside the exe, maybe make the image bounce around the screen

    /* ============== */
    /* WINDOWS EVENTS */
    /* ============== */
    FakeCrash, // suspends the game for 4 seconds
    RealCrash, // closes the game
    MinimizeGame, // minimizes the game window
    Lag, // suspends game to mimic lag (30 sec)
    LagStutter, // briefly suspends game (30 sec)
    //RepeatedMinimizing, // randomly minimizes the game then maximizes it randomly (20 sec)
    //TakeYourMeds, // mutes the game (15 sec)
}

impl ChaosEvents {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChaosEvents::Nothing => "Nothing ever happens",
            ChaosEvents::RandomEvent => "Random event",
            ChaosEvents::MetaNoChaos => "No chaos (meta)",
            ChaosEvents::MetaMoreChaos => "More chaos (meta)",
            ChaosEvents::RemoveMoney => "Remove all money",
            ChaosEvents::CheckBounced => "Check bounced",
            ChaosEvents::SpareChange => "Spare change",
            ChaosEvents::MaxMoney => "Max money",
            ChaosEvents::Invincibility => "Invincibility (30 seconds)",
            ChaosEvents::Heal => "Heal",
            ChaosEvents::NoHealth => "I need a doctor (1hp)",
            ChaosEvents::Suicide => "You serve zero purpose",
            ChaosEvents::MostWanted => "Most wanted (30 seconds)",
            ChaosEvents::WantedHealth => "Health is trouble (30 seconds)",
            ChaosEvents::RemoveWanted  => "Remove trouble",
            ChaosEvents::RemoveAllAmmo => "Remove all ammo",
            ChaosEvents::GiveAllAmmo => "Give all ammo",
            ChaosEvents::Sisyphus => "Sisyphus (10-18 seconds)",
            ChaosEvents::SonarSisyphus => "Sonar Sisyphus (10-18 seconds)",
            ChaosEvents::Speed => "Speed (30 seconds)",
            ChaosEvents::SpeedFaster => "Ugandan speed (15 seconds)",
            ChaosEvents::Slowness => "Slowness (20 seconds)",
            ChaosEvents::MaxJump => "Max jump (30 seconds)",
            ChaosEvents::NoJumping => "No jumping (30 seconds)",
            ChaosEvents::Freeze => "Freeze (10 seconds)",
            ChaosEvents::HomeSweetHome => "Home Sweet Home",
            ChaosEvents::RandomTp => "Random TP",
            ChaosEvents::FakeRandomTp => "Fake random TP",
            ChaosEvents::SkyTp => "Sky TP (Suicide)",
            ChaosEvents::FakeSkyTp => "Fake Sky TP",
            ChaosEvents::HellTp => "Mole POV (Suicide)",
            ChaosEvents::BusTp => "BUS",
            ChaosEvents::ReverseGravity => "Reverse gravity (10 seconds)",
            ChaosEvents::Phoon => "Phoon (30 seconds)",
            ChaosEvents::OppositeInput => "Opposite input (30 seconds)",
            ChaosEvents::Flight => "Flight (30 seconds)",
            ChaosEvents::DrunkSpeed => "Drunk speed (30 seconds)",
            ChaosEvents::RandomForce => "Random force (30 seconds)",
            ChaosEvents::DisabledMovementAxis => "Disable random axis (30 seconds)",
            ChaosEvents::FakeCrash => "Fake crash",
            ChaosEvents::RealCrash => "Real crash",
            ChaosEvents::MinimizeGame => "Minimize game",
            ChaosEvents::Lag => "Favela PC (30 seconds)",
            ChaosEvents::LagStutter => "Game stutter (30 seconds)",
        }
    }

    pub async fn execute(&self, data: &GameData, twitch_client_data: Option<&TwitchClientData>) {
        match self {
            ChaosEvents::Nothing => info!("nothing (event)"),
            ChaosEvents::RandomEvent => {
                // get all event options
                let mut events = ChaosEvents::iter().collect::<Vec<ChaosEvents>>();

                // remove random event option
                events.retain(|e| *e != ChaosEvents::RandomEvent);

                // get random event
                let event = ChaosEvents::rand_vec(&events);

                // send message for the random event
                if let Some(twitch_client_data) = twitch_client_data {
                    let _ = twitch_client_data.irc_client.say(twitch_client_data.channel.clone(), format!("Random event: {}", event.as_str())).await;
                }

                // execute random event
                Box::pin(event.execute(&data, None)).await;
            },
            ChaosEvents::MetaNoChaos => {
                if let Some(twitch_client_data) = twitch_client_data {
                    let _ = twitch_client_data.irc_client.say(twitch_client_data.channel.clone(), String::from("Voting disabled for 1 minute (meta).")).await;
                }

                tokio::time::sleep(Duration::from_secs(60)).await
            },
            ChaosEvents::MetaMoreChaos => { }, // handled in main.rs
            ChaosEvents::RemoveMoney => money::remove_money(&data),
            ChaosEvents::CheckBounced => money::check_bounced(&data),
            ChaosEvents::SpareChange => money::spare_change(&data),
            ChaosEvents::MaxMoney => money::give_max_money(&data),
            ChaosEvents::Invincibility => health::give_invincibility(&data).await,
            ChaosEvents::Heal => health::heal(&data),
            ChaosEvents::NoHealth => health::no_health(&data),
            ChaosEvents::Suicide => health::suicide(&data),
            ChaosEvents::MostWanted => trouble_meter::max_trouble(&data).await,
            ChaosEvents::WantedHealth => trouble_meter::trouble_health(&data).await,
            ChaosEvents::RemoveWanted => trouble_meter::remove_trouble(&data),
            ChaosEvents::RemoveAllAmmo => ammo::remove_all_ammo(&data),
            ChaosEvents::GiveAllAmmo => ammo::give_all_ammo(&data),
            ChaosEvents::Sisyphus => location::sisyphus(&data).await,
            ChaosEvents::SonarSisyphus => location::sonar_sisyphus(&data).await,
            ChaosEvents::Speed => location::speed(&data).await,
            ChaosEvents::SpeedFaster => location::speed_faster(&data).await,
            ChaosEvents::Slowness => location::slowness(&data).await,
            ChaosEvents::MaxJump => location::max_jump(&data).await,
            ChaosEvents::NoJumping => location::no_jumping(&data).await,
            ChaosEvents::Freeze => location::freeze(&data).await,
            ChaosEvents::HomeSweetHome => location::teleport_dorms(&data),
            ChaosEvents::RandomTp => location::random_tp(&data),
            ChaosEvents::FakeRandomTp => location::fake_random_tp(&data).await,
            ChaosEvents::SkyTp => location::sky_tp(&data),
            ChaosEvents::FakeSkyTp => location::fake_sky_tp(&data).await,
            ChaosEvents::HellTp => location::hell_tp(&data),
            ChaosEvents::BusTp => location::bus_tp(&data),
            ChaosEvents::ReverseGravity => location::reverse_gravity(&data).await,
            ChaosEvents::Phoon => location::phoon(&data).await,
            ChaosEvents::OppositeInput => location::opposite_input(&data).await,
            ChaosEvents::Flight => location::flight(&data).await,
            ChaosEvents::DrunkSpeed => location::drunk_speed(&data).await,
            ChaosEvents::RandomForce => location::random_force(&data).await,
            ChaosEvents::DisabledMovementAxis => location::disabled_movement_axis(&data).await,
            ChaosEvents::FakeCrash => win_api::fake_crash(&data).await,
            ChaosEvents::RealCrash => win_api::real_crash(&data).await,
            ChaosEvents::MinimizeGame => win_api::minimize_game(&data).await,
            ChaosEvents::Lag => win_api::lag(&data).await,
            ChaosEvents::LagStutter => win_api::lag_stutter(&data).await,
        }
    }

    pub fn rand() -> ChaosEvents {
        let mut rng = rand::rng();
        ChaosEvents::iter().choose(&mut rng).unwrap_or(ChaosEvents::Nothing)
    }

    pub fn rand_vec(events: &Vec<ChaosEvents>) -> ChaosEvents {
        let mut rng = rand::rng();
        events.iter().choose(&mut rng).unwrap().clone()
    }

    pub fn get_events() -> Vec<ChaosEvents> {
        // get all event options
        let mut events = ChaosEvents::iter().collect::<Vec<ChaosEvents>>();

        // remove random event option (we always add this as an option)
        events.retain(|e| *e != ChaosEvents::RandomEvent);

        // create vec for selected events
        let mut chaos_events: Vec<ChaosEvents> = Vec::new();

        // add 3 unique random events
        for _ in 0..3 {
            // get random event
            let event = ChaosEvents::rand_vec(&events);

            // add random event
            chaos_events.push(event.clone());

            // remove event from vec
            events.retain(|e| *e != event);
        }

        // add random event
        chaos_events.push(ChaosEvents::RandomEvent);

        // return events
        chaos_events
    }
}

pub struct TwitchClientData {
    pub irc_client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
    pub channel: String,
}