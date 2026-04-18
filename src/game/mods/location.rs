use std::time::{Duration, Instant};
use crate::game::bully::GameData;
use crate::game::mods::health;
use crate::memory::coordinates_vector::CoordinatesVector;
use crate::windows::input;

pub async fn sisyphus(data: &GameData) {
    // get starting location
    let starting_location = CoordinatesVector::read(&data);

    // get starting time & a random duration
    let start_time = Instant::now();
    let duration = rand::random_range(10..18);

    loop {
        // check if the random duration has passed
        if start_time.elapsed().as_secs() >= duration {
            break;
        }

        // check if the player is moving
        if !input::is_moving() {
            tokio::time::sleep(Duration::from_millis(5)).await;
            continue;
        }

        // get current location
        let current_location = CoordinatesVector::read(&data);

        // check if the player has moved more than 5 units in any direction
        if starting_location.has_moved(&current_location, 3.0) {
            // teleport back to starting location
            CoordinatesVector::write(&data, starting_location.clone());
        }

        // sleep for cpu usage
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
}

pub async fn sonar_sisyphus(data: &GameData) {
    // get starting location
    let starting_location = CoordinatesVector::read(&data);

    // get starting time & a random duration
    let start_time = Instant::now();
    let duration = rand::random_range(10..18);

    // variable for how long to sleep for
    let mut sleep_ms = 100;

    loop {
        // check if the random duration has passed
        if start_time.elapsed().as_secs() >= duration {
            break;
        }

        // check if the player is moving
        if !input::is_moving() {
            tokio::time::sleep(Duration::from_millis(5)).await;
            continue;
        }

        // sleep for sonar effect
        tokio::time::sleep(Duration::from_millis(sleep_ms)).await;

        // update sleep ms
        if sleep_ms < 1000 {
            sleep_ms += (sleep_ms / 5).max(50).min(1000);
        }

        // teleport to starting location
        CoordinatesVector::write(&data, starting_location.clone());
    }
}

pub async fn speed(data: &GameData) {
    // get starting time
    let start_time = Instant::now();

    loop {
        // check if 30 seconds has passed
        if start_time.elapsed().as_secs() >= 30 {
            break;
        }

        // check if the player is moving
        if !input::is_moving() {
            tokio::time::sleep(Duration::from_millis(5)).await;
            continue;
        }

        // get current location
        let current_location = CoordinatesVector::read(&data);

        // sleep for difference calculation
        tokio::time::sleep(Duration::from_millis(20)).await;

        // get new location
        let mut new_location = CoordinatesVector::read(&data);

        // get displacement between locations
        let displacement = current_location.get_displacement(&new_location);

        // add difference to new location
        new_location.add(displacement);

        // update location
        CoordinatesVector::write(&data, new_location);
    }
}

pub async fn speed_faster(data: &GameData) {
    // get starting time
    let start_time = Instant::now();

    loop {
        // check if 15 seconds has passed
        if start_time.elapsed().as_secs() >= 15 {
            break;
        }

        // check if the player is moving
        if !input::is_moving() {
            tokio::time::sleep(Duration::from_millis(5)).await;
            continue;
        }

        // get current location
        let current_location = CoordinatesVector::read(&data);

        // sleep for difference calculation
        tokio::time::sleep(Duration::from_millis(10)).await;

        // get new location
        let mut new_location = CoordinatesVector::read(&data);

        // get displacement between locations
        let mut displacement = current_location.get_displacement(&new_location);

        // add extra displacement
        displacement.multiply_horizontal(2f32);

        // add difference to new location
        new_location.add(displacement);

        // update location
        CoordinatesVector::write(&data, new_location);
    }
}

pub async fn max_jump(data: &GameData) {
    // get starting time
    let start_time = Instant::now();

    loop {
        // check if 30 seconds has passed
        if start_time.elapsed().as_secs() >= 30 {
            break;
        }

        // check if the player has pressed space
        if !input::has_jumped() && !input::is_jumping() {
            tokio::time::sleep(Duration::from_millis(5)).await;
            continue;
        }

        // get current health (we will reset to it)
        let original_health = health::get_health(&data);

        // get original location
        let original_location = CoordinatesVector::read(&data);

        // give an unreasonable amount of health to prevent dying
        health::update_health(&data, 999999f32);

        // apply vertical motion
        for _ in 0..5 {
            // get current location
            let mut current_location = CoordinatesVector::read(&data);

            // add to z coordinate
            current_location.z += 3f32;

            // update coordinates
            CoordinatesVector::write(&data, current_location);

            // sleep
            tokio::time::sleep(Duration::from_millis(30)).await;
        }

        // wait for player to get back on-ground
        loop {
            // get current location
            let current_location = CoordinatesVector::read(&data);

            // check if we're back on the ground
            if current_location.z as i32 <= original_location.z as i32 {
                // wait for us to be fully on-ground (prevent accidental death)
                tokio::time::sleep(Duration::from_millis(500)).await;

                // reset health
                match original_health {
                    Some(original_health) => {
                        health::update_health(&data, original_health);
                        break;
                    },
                    None => health::heal(&data)
                }

                break;
            }

            // sleep
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }
}

pub async fn no_jumping(data: &GameData) {
    // get starting time
    let start_time = Instant::now();

    // get original location
    let original_location = CoordinatesVector::read(&data);

    // variable for storing the last z value before space was pressed
    let mut last_ground_z: f32 = original_location.z;
    let mut has_jumped = false;
    let mut jump_time: Instant = Instant::now();

    loop {
        // check if 30 seconds has passed
        if start_time.elapsed().as_secs() >= 30 {
            break;
        }

        // get current location
        let mut current_location = CoordinatesVector::read(&data);

        // check if the player isn't jumping
        if !input::has_jumped() && !input::is_jumping() {
            // make sure we aren't currently changing the z position
            if !has_jumped {
                // update last ground z
                last_ground_z = current_location.z;
            }
        } else {
            // set has jumped
            has_jumped = true;
            jump_time = Instant::now();
        }

        // check if the player has jumped
        if has_jumped {
            // check if the player is back on the ground
            if current_location.z as i32 <= original_location.z as i32 {
                // make sure the player has had enough time to land
                if jump_time.elapsed().as_secs() >= 4 {
                    has_jumped = false;
                }
            }

            // update z location
            current_location.z = last_ground_z;

            // update location
            CoordinatesVector::write(&data, current_location);
        }

        // sleep
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
}

pub async fn freeze(data: &GameData) {
    // get starting time
    let start_time = Instant::now();

    loop {
        // check if 10 seconds has passed
        if start_time.elapsed().as_secs() >= 10 {
            break;
        }

        // get current location
        let current_location = CoordinatesVector::read(&data);

        // sleep for difference calculation
        tokio::time::sleep(Duration::from_millis(10)).await;

        // set to old location
        CoordinatesVector::write(&data, current_location);
    }
}

pub fn teleport_dorms(data: &GameData) {
    CoordinatesVector::write(&data, CoordinatesVector { x: 271.06076, y: -115.08667, z: /*6*/11.1845984 });
}

pub fn random_tp(data: &GameData) {
    // vector for random coordinates
    let mut random_coordinates = CoordinatesVector::empty();

    // set random horizontal coordinates
    random_coordinates.x = rand::random_range(-450f32..450f32);
    random_coordinates.y = rand::random_range(-450f32..450f32);

    // teleport up a little bit to prevent most clipping
    random_coordinates.z = 15f32;

    // teleport to random coordinates
    CoordinatesVector::write(&data, random_coordinates);
}

pub async fn fake_random_tp(data: &GameData) {
    // get current location
    let original_location = CoordinatesVector::read(&data);

    // get current health (we will reset to it)
    let original_health = health::get_health(&data);

    // give an unreasonable amount of health to prevent dying
    health::update_health(&data, 999999f32);

    // teleport to random location
    random_tp(&data);

    // sleep for 5 seconds
    tokio::time::sleep(Duration::from_secs(5)).await;

    // teleport back to original location
    CoordinatesVector::write(&data, original_location);

    // reset health
    match original_health {
        Some(original_health) => health::update_health(&data, original_health),
        None => health::heal(&data)
    }
}

pub fn sky_tp(data: &GameData) {
    // get current location
    let mut current_location = CoordinatesVector::read(&data);

    // add to vertical coordinate
    current_location.z += 100f32;

    // update location
    CoordinatesVector::write(&data, current_location);
}

pub async fn fake_sky_tp(data: &GameData) {
    // get current health (we will reset to it)
    let original_health = health::get_health(&data);

    // give an unreasonable amount of health to prevent dying
    health::update_health(&data, 999999f32);

    // get current location
    let original_location = CoordinatesVector::read(&data);

    // teleport to sky
    sky_tp(&data);

    loop {
        // get current location
        let current_location = CoordinatesVector::read(&data);

        // check if we're back on the ground
        if current_location.z as i32 <= original_location.z as i32 {
            // wait for us to be fully on-ground (prevent accidental death)
            tokio::time::sleep(Duration::from_millis(500)).await;

            // reset health
            match original_health {
                Some(original_health) => health::update_health(&data, original_health),
                None => health::heal(&data)
            }

            break;
        }

        // sleep
        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    /*// get current location
    let mut original_location = CoordinatesVector::read(&data);
    let original_z = original_location.z;

    // add to vertical coordinate
    original_location.z += 100f32;

    // update location
    CoordinatesVector::write(&data, original_location.clone());

    // sleep for half a second
    tokio::time::sleep(Duration::from_millis(500)).await;

    // reset z coordinate
    original_location.z = original_z;

    // reset location
    CoordinatesVector::write(&data, original_location);*/
}

pub fn hell_tp(data: &GameData) {
    // get current location
    let mut current_location = CoordinatesVector::read(&data);

    // teleport to beneath the map
    current_location.z = -50f32;

    // update location
    CoordinatesVector::write(&data, current_location);
}

pub async fn reverse_gravity(data: &GameData) {
    /*// determine gravity
    let mut starting_position = CoordinatesVector::read(&data);

    // add some height
    starting_position.z += 1f32;

    // update position
    CoordinatesVector::write(&data, starting_position.clone());

    // sleep for 1ms & get updated position
    tokio::time::sleep(Duration::from_millis(1)).await;

    // get new position
    let gravity_position = CoordinatesVector::read(&data);

    // determine gravity
    let gravity = gravity_position.z - starting_position.z;
    //debug!("gravity_position.z: {} | starting_position.z: {} | gravity: {}", gravity_position.z, starting_position.z, gravity);*/

    // get starting time & a random duration
    let start_time = Instant::now();

    // variables for increasing gravity
    let mut gravity_multiplier: f32 = 1.025569439;
    let mut frame_counter: usize = 0;

    loop {
        // check if the 10 seconds has passed
        if start_time.elapsed().as_secs() >= 5 {
            break;
        }

        // get current position
        let mut current_location = CoordinatesVector::read(&data);

        // apply gravity, stop at 150z
        current_location.z = (current_location.z * gravity_multiplier).min(150f32);

        // check if we should increment gravity
        if frame_counter % 23 == 0 {
            // add gravity to multiplier & make sure we don't go up too fast
            // -0.025569439
            gravity_multiplier = (gravity_multiplier + 0.025569439).min(1.041f32); // 1.0389f32
        }

        // update position
        CoordinatesVector::write(&data, current_location);

        // sleep for 1ms
        tokio::time::sleep(Duration::from_millis(1)).await;

        // add frame
        frame_counter += 1;
    }
}

pub async fn phoon(data: &GameData) {
    // get starting time & a random duration
    let start_time = Instant::now();

    loop {
        // check if the 30 seconds has passed
        if start_time.elapsed().as_secs() >= 30 {
            break;
        }

        // get current position
        let mut current_location = CoordinatesVector::read(&data);

        // multiply z for bouncing affect
        current_location.z *= 1.025;

        // update position
        CoordinatesVector::write(&data, current_location);

        // sleep for 1ms
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
}

/*pub fn get_location(data: &GameData) {
    let coordinates = CoordinatesVector::read(data);
    debug!("{:?}", coordinates);
}*/