use bevy::prelude::*;
use super::MaterialType;
use std::collections::HashMap;

#[derive(Component, Debug, Clone, PartialEq)]
pub enum BuildingType {
    WorkBench,
    Apex,
    Armoury,
    Temple,
}

impl BuildingType {
    pub fn get_cost(&self) -> HashMap<MaterialType, u32> {
        let mut cost = HashMap::new();
        match self {
            BuildingType::WorkBench => {
                cost.insert(MaterialType::Wood, 250);
            }
            BuildingType::Apex => {
                cost.insert(MaterialType::Wood, 1000);
                cost.insert(MaterialType::Stone, 1000);
            }
            BuildingType::Armoury => {
                cost.insert(MaterialType::Wood, 250);
                cost.insert(MaterialType::Stone, 250);
                cost.insert(MaterialType::Metal, 500);
            }
            BuildingType::Temple => {
                cost.insert(MaterialType::Wood, 250);
                cost.insert(MaterialType::Stone, 250);
                cost.insert(MaterialType::Metal, 250);
                cost.insert(MaterialType::Flux, 1000);
            }
        }
        cost
    }

    pub fn get_name(&self) -> &str {
        match self {
            BuildingType::WorkBench => "Work Bench",
            BuildingType::Apex => "Apex - Level 1",
            BuildingType::Armoury => "Armoury - Level 1",
            BuildingType::Temple => "Temple - Level 1",
        }
    }
}

#[derive(Component)]
pub struct Building {
    pub building_type: BuildingType,
    pub level: u32,
    pub is_functional: bool,
}

impl Building {
    pub fn new(building_type: BuildingType) -> Self {
        Self {
            building_type,
            level: 1,
            is_functional: true,
        }
    }
}

#[derive(Component)]
pub struct BuildingGhost {
    pub building_type: BuildingType,
    pub is_valid_placement: bool,
}

#[derive(Component)]
pub struct Constructible {
    pub build_time: f32,
    pub current_build_time: f32,
    pub is_complete: bool,
}

impl Constructible {
    pub fn new(build_time: f32) -> Self {
        Self {
            build_time,
            current_build_time: 0.0,
            is_complete: false,
        }
    }

    pub fn progress(&mut self, delta_time: f32) {
        if !self.is_complete {
            self.current_build_time += delta_time;
            if self.current_build_time >= self.build_time {
                self.is_complete = true;
            }
        }
    }

    pub fn get_progress(&self) -> f32 {
        (self.current_build_time / self.build_time).min(1.0)
    }
}
