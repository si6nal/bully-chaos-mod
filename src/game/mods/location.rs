use std::time::{Duration, Instant};
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_LCONTROL, VK_LSHIFT};
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

pub async fn slowness(data: &GameData) {
    // get starting time
    let start_time = Instant::now();

    loop {
        // check if 20 seconds has passed
        if start_time.elapsed().as_secs() >= 20 {
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

        // half displacement
        displacement.divide_horizontal(2f32);

        // subtract half the displacement from the current location
        new_location.subtract(displacement);

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

    // todo: set location to random tp location continuously for 5 seconds to prevent some death scenarios

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

pub fn bus_tp(data: &GameData) {
    // this doesn't include bus stop locations from the north-east island or the west most bus stop
    let bus_stop_locations: Vec<CoordinatesVector> = vec![
        CoordinatesVector::from(304.43604f32, 269.8308f32, 5.436815f32), // { x: 304.43604, y: 269.8308, z: 5.436815 }
        CoordinatesVector::from(459.7717f32, 244.46739f32, 10.634422f32), // { x: 459.7717, y: 244.46739, z: 10.634422 }
        CoordinatesVector::from(537.21747f32, 417.62842f32, 17.077328f32), // { x: 537.21747, y: 417.62842, z: 17.077328 }
        CoordinatesVector::from(586.89514f32, -47.421276f32, 5.915363f32), // { x: 586.89514, y: -47.421276, z: 5.915363 }
        CoordinatesVector::from(125.42762f32, -377.4589f32, 2.2570353f32), // { x: 125.42762, y: -377.4589, z: 2.2570353 }
        CoordinatesVector::from(256.95206f32, -418.27426f32, 2.6757145f32), // { x: 256.95206, y: -418.27426, z: 2.6757145 }
    ];

    // get current location
    let current_location = CoordinatesVector::read(&data);

    // determine the closest bus stop
    let mut closest_bus_stop_location = current_location.clone();
    let mut best_distance = f32::MAX;
    for bus_stop_location in bus_stop_locations {
        // get distance to bus stop
        let distance = current_location.distance_to(&bus_stop_location);

        // check if it's the closest
        if distance < best_distance {
            closest_bus_stop_location = bus_stop_location;
            best_distance = distance;
        }
    }

    // move bus stop location slightly up to prevent potential clipping
    closest_bus_stop_location.z += 0.5f32;

    // teleport to the closest bus stop
    CoordinatesVector::write(&data, closest_bus_stop_location);
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

pub async fn opposite_input(data: &GameData) {
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
        let mut displacement = current_location.get_displacement(&new_location);

        // duplicate displacement
        displacement.multiply_horizontal(2f32);

        // subtract difference from new location
        new_location.subtract(displacement);

        // update location
        CoordinatesVector::write(&data, new_location);
    }
}

pub async fn flight(data: &GameData) {
    // get current z coordinate so we can modify it
    let mut flight_z = CoordinatesVector::read(&data).z;
    let mut is_flying = false;

    // get starting time
    let start_time = Instant::now();

    loop {
        // check if 30 seconds has passed
        if start_time.elapsed().as_secs() >= 30 {
            break;
        }

        // check if we should modify z coordinate
        if input::is_key_down(VK_LSHIFT) {
            flight_z += 0.1f32;
            is_flying = true;
        } else if input::is_key_down(VK_LCONTROL) {
            flight_z -= 0.1f32;
            is_flying = true;
        }

        // get current coordinates
        let mut coords = CoordinatesVector::read(&data);

        // check if the player is moving for resetting flight variables or enabling flight
        if input::is_moving() {
            // set z coordinate to flight z if the player is changing z or has changed z
            if is_flying {
                // update z coordinate
                coords.z = flight_z;
            }
        } else {
            // reset flight z value
            flight_z = coords.z;
            is_flying = false;
        }

        // update coordinates
        CoordinatesVector::write(&data, coords);

        // sleep for cpu usage
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
}

pub async fn drunk_speed(data: &GameData) {
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
        let mut displacement = current_location.get_displacement(&new_location);

        // add extra x/z displacement
        if displacement.x <= 0.0f32 {
            displacement.x = displacement.y;
        }

        if displacement.y <= 0.0f32 {
            displacement.y = displacement.x;
        }

        // add difference to new location
        new_location.add(displacement);

        // update location
        CoordinatesVector::write(&data, new_location);
    }
}

pub async fn random_force(data: &GameData) {
    // variables for tracking forced movement
    let mut last_input_time = Instant::now();
    let mut xy_inverse = false;

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

        // check if we have recently added movement
        if last_input_time.elapsed().as_millis() >= 325 {
            // sleep
            let sleep_time = rand::random_range(2..8);
            tokio::time::sleep(Duration::from_secs(sleep_time)).await;

            // reset last input time & flip xy inverse
            last_input_time = Instant::now();
            xy_inverse = !xy_inverse;
        }

        // get current location
        let current_location = CoordinatesVector::read(&data);

        // sleep for difference calculation
        tokio::time::sleep(Duration::from_millis(20)).await;

        // get new location
        let mut new_location = CoordinatesVector::read(&data);

        // get displacement between locations
        let displacement = current_location.get_displacement(&new_location);

        // move in different direction
        if xy_inverse {
            new_location.x -= displacement.x * 1.5f32;
        } else {
            new_location.y -= displacement.y * 1.5f32;
        }

        // add difference to new location
        //new_location.subtract_horizontal(displacement);

        // update location
        CoordinatesVector::write(&data, new_location);
    }
}

pub async fn disabled_movement_axis(data: &GameData) {
    // choose random axis to disable
    let disabled_axis = rand::random_range(0..2);

    // get starting location
    let starting_location = CoordinatesVector::read(&data);

    // get starting time
    let start_time = Instant::now();

    loop {
        // check if 15 seconds has passed
        if start_time.elapsed().as_secs() >= 15 {
            break;
        }

        // get current location
        let mut current_location = CoordinatesVector::read(&data);

        // disable axis
        match disabled_axis {
            0 => current_location.x = starting_location.x,
            1 => current_location.y = starting_location.y,
            2 => current_location.z = starting_location.z,
            _ => unreachable!(),
        }

        // update location
        CoordinatesVector::write(&data, current_location);

        // sleep
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

/*pub async fn get_location(data: &GameData) {
    loop {
        let coordinates = CoordinatesVector::read(data);
        log::debug!("{:?}", coordinates);
        tokio::time::sleep(Duration::from_millis(500)).await;

        {
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
            displacement.multiply_horizontal(3f32);

            // add difference to new location
            new_location.add(displacement);

            // update location
            CoordinatesVector::write(&data, new_location);
        }
    }
}*/