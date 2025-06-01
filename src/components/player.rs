use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use super::{ToolType, MaterialType, EquipmentSlots, SpecialAbility};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    // Movement (dual axis)
    #[actionlike(DualAxis)]
    Move,
    
    // Camera control
    Look,           // Mouse movement
    RotateCamera,   // Right-click to enable rotation
    
    // Equipment slots (1-4 and Shift+1-4)
    EquipSlot1,
    EquipSlot2,
    EquipSlot3,
    EquipSlot4,
    EquipSlot5,
    EquipSlot6,
    EquipSlot7,
    EquipSlot8,
    
    // Actions
    Attack,              // Left mouse - attack/harvest/use
    Interact,            // F key
    SpecialAbility1,     // Q key (empty initially)
    SpecialAbility2,     // E key (empty initially)
    
    // Build mode
    ToggleBuildMode,     // B key
}

#[derive(Component)]
pub struct Player {
    pub movement_speed: f32,
    pub interaction_range: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            movement_speed: 5.0,
            interaction_range: 3.0,
        }
    }
}

#[derive(Component)]
pub struct PlayerController {
    pub equipment: EquipmentSlots,
    pub special_abilities: [Option<SpecialAbility>; 2], // Q and E slots
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            equipment: EquipmentSlots::default(),
            special_abilities: [None, None],
        }
    }
}

#[derive(Component)]
pub struct CameraTarget;

#[derive(Component)]
pub struct PlayerCamera {
    pub offset: Vec3,
    pub sensitivity: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub is_rotating: bool,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            offset: Vec3::new(0.0, 5.0, 10.0), // Behind and above player
            sensitivity: 0.003,
            pitch: -0.4, // Look down slightly
            yaw: 0.0,
            is_rotating: false,
        }
    }
}

#[derive(Component)]
pub struct PlayerInput {
    pub movement: Vec3,
    pub mouse_delta: Vec2,
    pub interact: bool,
    pub build_mode: bool,
    pub tool_switch: Option<usize>,
}

impl Default for PlayerInput {
    fn default() -> Self {
        Self {
            movement: Vec3::ZERO,
            mouse_delta: Vec2::ZERO,
            interact: false,
            build_mode: false,
            tool_switch: None,
        }
    }
}
