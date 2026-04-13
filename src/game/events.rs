use std::time::Duration;
use log::info;
use rand::prelude::IteratorRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::game::bully::GameData;
use crate::game::mods::{ammo, health, location, money, trouble_meter};
use crate::windows::processes;

// events that are commented out aren't implemented
#[derive(EnumIter, Clone, PartialEq)]
pub enum ChaosEvents {
    Nothing, // does nothing (skips vote)
    Nothing1Min, // does nothing for 1 minute
    RandomEvent, // selects another event

    /* =========== */
    /* MEMORY MODS */
    /* =========== */
    RemoveMoney, // money = 0
    MaxMoney, // money = i32 max ~ capped at 1m

    Invincibility, // 30sec setting health to 100
    Heal, // health = 100
    NoHealth, // health = 1
    Suicide, // health = 0

    MostWanted, // max trouble meter (30 sec)
    RemoveWanted, // wanted level = 0

    RemoveAllAmmo, // removes all ammo
    GiveAllAmmo, // gives max ammo for weapons

    Sisyphus, // teleports the player back to their original pos if they move more than 3 units (10-18 sec)
    SonarSisyphus, // teleports the player back to their original pos with an increasing delay (10-18 sec)

    Speed, // duplicates moving distance (15-20 sec)
    Freeze, // stops the player from moving (10 sec)

    HomeSweetHome, // teleport back to the dorms
    RandomTp, // teleports to random **pre-defined** coordinates of in-game locations | coordinates aren't pre-defined, too lazy to get list of coordinates
    FakeRandomTp, // teleports player back to original location after a random tp (5 sec delay)
    SkyTp, // teleports the player into the sky
    FakeSkyTp, // teleports the player into the sky & back onto the ground before dying
    HellTp, // teleports the player into the ground

    /* ============ */
    /* WINDOWS MODS */
    /* ============ */
    FakeCrash, // suspends the game for 3 seconds
    RealCrash, // closes the game

    //Schizophrenia, // randomly presses movement keys & moves the mouse, 2-6sec input delay (30 sec)
    // todo: try to implement opposite input as a memory mod, get difference between location every 5/10ms & apply distance in opposite direction
    //OppositeInput, // causes the player to move opposite of their key presses (send release & press+release messages) (30 sec)
    //CameraSpin, // sends mouse movement messages to rotate the camera (20 sec)
    //ConstantAttacking, // sends key presses to attack (15 sec)
    //Phoon, // sends key presses to jump (15 sec)
    //TakeYourMeds, // mutes the game (15 sec)
}

impl ChaosEvents {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChaosEvents::Nothing        => "Nothing ever happens",
            ChaosEvents::Nothing1Min    => "Nothing for 1 minute",
            ChaosEvents::RandomEvent    => "Random event",
            ChaosEvents::RemoveMoney    => "Remove all money",
            ChaosEvents::MaxMoney       => "Max money",
            ChaosEvents::Invincibility  => "Invincibility (30 seconds)",
            ChaosEvents::Heal           => "Heal",
            ChaosEvents::NoHealth       => "I need a doctor (1hp)",
            ChaosEvents::Suicide        => "You serve zero purpose",
            ChaosEvents::MostWanted     => "Most wanted (30 seconds)",
            ChaosEvents::RemoveWanted   => "Remove trouble",
            ChaosEvents::RemoveAllAmmo  => "Remove all ammo",
            ChaosEvents::GiveAllAmmo    => "Give all ammo",
            ChaosEvents::Sisyphus       => "Sisyphus (10-18 seconds)",
            ChaosEvents::SonarSisyphus  => "Sonar Sisyphus (10-18 seconds)",
            ChaosEvents::Speed          => "Supa Sprinter (10-15 seconds)",
            ChaosEvents::Freeze         => "Freeze (10 seconds)",
            ChaosEvents::HomeSweetHome  => "Home Sweet Home",
            ChaosEvents::RandomTp       => "Random TP",
            ChaosEvents::FakeRandomTp   => "Fake random TP",
            ChaosEvents::SkyTp          => "Sky TP (Suicide)",
            ChaosEvents::FakeSkyTp      => "Fake Sky TP",
            ChaosEvents::HellTp         => "Mole POV (Suicide)",
            ChaosEvents::FakeCrash      => "Fake crash",
            ChaosEvents::RealCrash      => "Real crash",
            //ChaosEvents::Phoon          => "Phoon",
        }
    }

    pub async fn execute(&self, data: &GameData) {
        match self {
            ChaosEvents::Nothing => info!("nothing (event)"),
            ChaosEvents::Nothing1Min => tokio::time::sleep(Duration::from_secs(60)).await,
            ChaosEvents::RandomEvent => {
                loop {
                    let next_event = Self::rand();
                    if next_event == ChaosEvents::RandomEvent {
                        continue;
                    }

                    Box::pin(next_event.execute(&data)).await;
                    break;
                }
            },
            ChaosEvents::RemoveMoney => money::remove_money(&data),
            ChaosEvents::MaxMoney => money::give_max_money(&data),
            ChaosEvents::Invincibility => health::give_invincibility(&data).await,
            ChaosEvents::Heal => health::heal(&data),
            ChaosEvents::NoHealth => health::no_health(&data),
            ChaosEvents::Suicide => health::suicide(&data),
            ChaosEvents::MostWanted => trouble_meter::max_trouble(&data).await,
            ChaosEvents::RemoveWanted => trouble_meter::remove_trouble(&data),
            ChaosEvents::RemoveAllAmmo => ammo::remove_all_ammo(&data),
            ChaosEvents::GiveAllAmmo => ammo::give_all_ammo(&data),
            ChaosEvents::Sisyphus => location::sisyphus(&data).await,
            ChaosEvents::SonarSisyphus => location::sonar_sisyphus(&data).await,
            ChaosEvents::Speed => location::speed(&data).await,
            ChaosEvents::Freeze => location::freeze(&data).await,
            ChaosEvents::HomeSweetHome => location::teleport_dorms(&data),
            ChaosEvents::RandomTp => location::random_tp(&data),
            ChaosEvents::FakeRandomTp => location::fake_random_tp(&data).await,
            ChaosEvents::SkyTp => location::sky_tp(&data),
            ChaosEvents::FakeSkyTp => location::fake_sky_tp(&data).await,
            ChaosEvents::HellTp => location::hell_tp(&data),
            ChaosEvents::FakeCrash => processes::pause_process(data.process_id, 4).await,
            ChaosEvents::RealCrash => processes::terminate_process(data.handle),
            //ChaosEvents::Phoon => input::phoon(&data).await,
        }
    }

    pub fn rand() -> ChaosEvents {
        let mut rng = rand::rng();
        ChaosEvents::iter().choose(&mut rng).unwrap_or(ChaosEvents::Nothing)
    }

    pub fn get_events() -> Vec<ChaosEvents> {
        vec![ChaosEvents::rand(), ChaosEvents::rand(), ChaosEvents::rand(), ChaosEvents::rand()]
    }
}