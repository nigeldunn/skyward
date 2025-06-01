use bevy::prelude::*;

pub mod player_controller;
pub mod world_generation;
pub mod resource_gathering;
pub mod building;
pub mod technology;

pub use player_controller::*;
pub use world_generation::*;
pub use resource_gathering::*;
pub use building::*;
pub use technology::*;

// System function placeholders that will be implemented in their respective modules
pub fn player_movement(/* parameters will be added */) {
    // Player movement system will be implemented in player_controller.rs
}

pub fn camera_follow(/* parameters will be added */) {
    // Camera follow system will be implemented in player_controller.rs
}

pub fn resource_interaction(/* parameters will be added */) {
    // Resource interaction system will be implemented in resource_gathering.rs
}

pub fn building_system(/* parameters will be added */) {
    // Building system will be implemented in building.rs
}
