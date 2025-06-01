use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod player;
pub mod resources;
pub mod buildings;

pub use player::*;
pub use resources::*;
pub use buildings::*;

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaterialType {
    Wood,
    Stone,
    Metal,
    Flux,
}

#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ToolType {
    Axe,
    Pick,
    Scoop,
}

impl ToolType {
    pub fn can_harvest(&self, material: &MaterialType) -> bool {
        match (self, material) {
            (ToolType::Axe, MaterialType::Wood) => true,
            (ToolType::Pick, MaterialType::Stone) => true,
            (ToolType::Pick, MaterialType::Metal) => true,
            (ToolType::Scoop, MaterialType::Flux) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,
    Bow,
    Spear,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConsumableType {
    Food,
    Potion,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemType {
    Tool(ToolType),
    Weapon(WeaponType),
    Consumable(ConsumableType),
    Material(MaterialType),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub item_type: ItemType,
    pub useable: bool,
}

#[derive(Debug, Clone)]
pub struct EquippedItem {
    pub item: Item,
    pub durability: Option<f32>,
    pub stack_count: Option<u32>,
}

#[derive(Component, Debug)]
pub struct EquipmentSlots {
    pub slots: [Option<EquippedItem>; 8],
    pub active_slot: usize, // 0-7 (corresponds to slots 1-8)
}

impl Default for EquipmentSlots {
    fn default() -> Self {
        Self {
            slots: [None, None, None, None, None, None, None, None],
            active_slot: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialAbility {
    pub id: String,
    pub name: String,
    pub cooldown: f32,
    pub last_used: f32,
}

// Technology Tree System
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnologyNode {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub tier: u32,
    pub position: TechPosition,
    pub prerequisites: Vec<String>,
    pub research_cost: u32,
    pub material_costs: std::collections::HashMap<MaterialType, u32>,
    pub unlocks: TechUnlocks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechUnlocks {
    pub items: Vec<String>, // Item IDs to unlock
    pub abilities: Vec<String>, // Ability IDs to unlock
    pub recipes: Vec<String>, // Recipe IDs to unlock
    pub buildings: Vec<String>, // Building type IDs to unlock
    pub research_bonus: Option<u32>, // Bonus research points per second
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechCategory {
    pub name: String,
    pub color: String, // Hex color code
    pub description: String,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct TechnologyProgress {
    pub unlocked_techs: std::collections::HashSet<String>,
    pub available_techs: std::collections::HashSet<String>,
    pub research_points: u32,
    pub total_points_earned: u32,
    pub current_research: Option<String>,
    pub research_progress: f32, // 0.0 to 1.0
}

impl Default for TechnologyProgress {
    fn default() -> Self {
        let mut unlocked = std::collections::HashSet::new();
        unlocked.insert("basic_survival".to_string());
        
        Self {
            unlocked_techs: unlocked,
            available_techs: std::collections::HashSet::new(),
            research_points: 0,
            total_points_earned: 0,
            current_research: None,
            research_progress: 0.0,
        }
    }
}

#[derive(Component)]
pub struct Interactable {
    pub interaction_range: f32,
}

#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}
