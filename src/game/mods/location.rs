use std::time::{Duration, Instant};
use crate::game::bully::GameData;
use crate::memory::coordinates_vector::CoordinatesVector;

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
    // get starting time & a random duration
    let start_time = Instant::now();
    let duration = rand::random_range(15..20);

    loop {
        // check if the random duration has passed
        if start_time.elapsed().as_secs() >= duration {
            break;
        }

        // get current location
        let current_location = CoordinatesVector::read(&data);

        // sleep for difference calculation
        tokio::time::sleep(Duration::from_millis(50)).await;

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
    CoordinatesVector::write(&data, CoordinatesVector { x: 271.06076, y: -115.08667, z: 6.1845984 });
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

pub fn sky_tp(data: &GameData) {
    // get current location
    let mut current_location = CoordinatesVector::read(&data);

    // add to vertical coordinate
    current_location.z += 100f32;

    // update location
    CoordinatesVector::write(&data, current_location);
}

pub fn hell_tp(data: &GameData) {
    // get current location
    let mut current_location = CoordinatesVector::read(&data);

    // teleport to beneath the map
    current_location.z = -50f32;

    // update location
    CoordinatesVector::write(&data, current_location);
}

/*pub fn get_location(data: &GameData) {
    let coordinates = CoordinatesVector::read(data);
    debug!("{:?}", coordinates);
}*/