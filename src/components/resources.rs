use bevy::prelude::*;
use super::MaterialType;

#[derive(Component)]
pub struct ResourceNode {
    pub material_type: MaterialType,
    pub amount: u32,
    pub max_amount: u32,
    pub regeneration_rate: f32,
    pub last_harvest_time: f32,
}

impl ResourceNode {
    pub fn new(material_type: MaterialType, amount: u32) -> Self {
        Self {
            material_type,
            amount,
            max_amount: amount,
            regeneration_rate: 1.0, // 1 unit per second
            last_harvest_time: 0.0,
        }
    }

    pub fn can_harvest(&self) -> bool {
        self.amount > 0
    }

    pub fn harvest(&mut self, amount: u32) -> u32 {
        let harvested = amount.min(self.amount);
        self.amount -= harvested;
        harvested
    }

    pub fn regenerate(&mut self, time: f32) {
        if self.amount < self.max_amount {
            let time_since_harvest = time - self.last_harvest_time;
            if time_since_harvest >= 1.0 / self.regeneration_rate {
                self.amount = (self.amount + 1).min(self.max_amount);
                self.last_harvest_time = time;
            }
        }
    }
}

#[derive(Component)]
pub struct Harvestable {
    pub harvest_time: f32,
    pub yield_amount: u32,
}

impl Default for Harvestable {
    fn default() -> Self {
        Self {
            harvest_time: 2.0, // 2 seconds to harvest
            yield_amount: 10,   // 10 units per harvest
        }
    }
}

#[derive(Component)]
pub struct WorldPosition {
    pub x: i32,
    pub z: i32,
}
