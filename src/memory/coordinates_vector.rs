use log::warn;
use crate::game::bully::GameData;
use crate::memory::{game_offsets, memory};

#[derive(Debug, Clone)]
pub struct CoordinatesVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl CoordinatesVector {
    pub fn read(data: &GameData) -> CoordinatesVector {
        // create vec for read coordinate values
        let mut coordinates = CoordinatesVector { x: 0.0, y: 0.0, z: 0.0 };

        // read x offset
        match Self::read_coordinate(&data, 0x00) {
            Some(x) => coordinates.x = x,
            None => warn!("failed to read x coordinate"),
        }

        // read y offset
        match Self::read_coordinate(&data, 0x04) {
            Some(y) => coordinates.y = y,
            None => warn!("failed to read y coordinate"),
        }

        // read z offset
        match Self::read_coordinate(&data, 0x08) {
            Some(z) => coordinates.z = z,
            None => warn!("failed to read z coordinate"),
        }

        coordinates
    }

    pub fn write(data: &GameData, coordinates: CoordinatesVector) {
        Self::write_coordinate(&data, 0x00, coordinates.x);
        Self::write_coordinate(&data, 0x04, coordinates.y);
        Self::write_coordinate(&data, 0x08, coordinates.z);
    }
    
    pub fn empty() -> CoordinatesVector {
        CoordinatesVector { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn has_moved(&self, other_coordinates: &CoordinatesVector, distance: f32) -> bool {
        // get difference between coordinates
        let difference = self.get_abs_delta(&other_coordinates);

        // return true if the difference between any coordinates is greater than the max distance
        difference.x >= distance || difference.y >= distance || difference.z >= distance
    }

    pub fn get_abs_delta(&self, other_coordinates: &CoordinatesVector) -> CoordinatesVector {
        CoordinatesVector {
            x: (self.x - other_coordinates.x).abs(),
            y: (self.y - other_coordinates.y).abs(),
            z: (self.z - other_coordinates.z).abs(),
        }
    }

    pub fn get_displacement(&self, other_coordinates: &CoordinatesVector) -> CoordinatesVector {
        CoordinatesVector {
            x: other_coordinates.x - self.x,
            y: other_coordinates.y - self.y,
            z: other_coordinates.z - self.z,
        }
    }

    pub fn add(&mut self, other_coordinates: CoordinatesVector) {
        self.x = self.x + other_coordinates.x;
        self.y = self.y + other_coordinates.y;
        self.z = self.z + other_coordinates.z;
    }
    
    pub fn multiply_horizontal(&mut self, multiplier: f32) {
        self.x = self.x * multiplier;
        self.y = self.y * multiplier;
    }

    fn read_coordinate(data: &GameData, padding: usize) -> Option<f32> {
        match game_offsets::get_offset(data.handle, data.player_coordinates_offset, game_offsets::PLAYER_COORDINATES_OFFSET + padding) {
            Some(coord_offset) => return memory::read::<f32>(data.handle, coord_offset),
            None => warn!("[r] failed to get player coordinate offset with {} padding.", padding)
        }

        None
    }

    fn write_coordinate(data: &GameData, padding: usize, pos: f32) {
        match game_offsets::get_offset(data.handle, data.player_coordinates_offset, game_offsets::PLAYER_COORDINATES_OFFSET + padding) {
            Some(coord_offset) => {
                if !memory::write::<f32>(data.handle, coord_offset, pos) {
                    warn!("failed to write player coordinate");
                }
            },
            None => warn!("[w] failed to get player coordinate offset with {} padding.", padding)
        }
    }
}