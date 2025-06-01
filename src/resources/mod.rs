use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::components::MaterialType;

pub mod inventory;
pub mod game_state;
pub mod technology;

pub use inventory::*;
pub use game_state::*;
pub use technology::*;

#[derive(Resource, Default, Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub materials: HashMap<MaterialType, u32>,
}

impl Inventory {
    pub fn new() -> Self {
        let mut materials = HashMap::new();
        materials.insert(MaterialType::Wood, 0);
        materials.insert(MaterialType::Stone, 0);
        materials.insert(MaterialType::Metal, 0);
        materials.insert(MaterialType::Flux, 0);
        
        Self { materials }
    }

    pub fn add_material(&mut self, material_type: MaterialType, amount: u32) {
        *self.materials.entry(material_type).or_insert(0) += amount;
    }

    pub fn remove_material(&mut self, material_type: MaterialType, amount: u32) -> bool {
        if let Some(current) = self.materials.get_mut(&material_type) {
            if *current >= amount {
                *current -= amount;
                return true;
            }
        }
        false
    }

    pub fn get_amount(&self, material_type: &MaterialType) -> u32 {
        *self.materials.get(material_type).unwrap_or(&0)
    }

    pub fn can_afford(&self, costs: &HashMap<MaterialType, u32>) -> bool {
        costs.iter().all(|(material, cost)| {
            self.get_amount(material) >= *cost
        })
    }

    pub fn spend_materials(&mut self, costs: &HashMap<MaterialType, u32>) -> bool {
        if self.can_afford(costs) {
            for (material, cost) in costs {
                self.remove_material(material.clone(), *cost);
            }
            true
        } else {
            false
        }
    }
}
