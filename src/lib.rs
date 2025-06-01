use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use std::collections::HashMap;

pub mod components;
pub mod systems;
pub mod resources;
pub mod ui;

use components::*;
use systems::*;
use resources::*;
use ui::*;

// Function to create default tech tree for testing
fn create_default_tech_tree() -> TechnologyTree {
    let mut categories = HashMap::new();
    categories.insert("survival".to_string(), TechCategory {
        name: "Survival".to_string(),
        color: "#8B4513".to_string(),
        description: "Basic survival skills and tools".to_string(),
    });
    categories.insert("crafting".to_string(), TechCategory {
        name: "Crafting".to_string(),
        color: "#FF6B35".to_string(),
        description: "Advanced crafting and manufacturing".to_string(),
    });

    let mut technologies = HashMap::new();
    
    // Basic survival tech (starting tech)
    technologies.insert("basic_survival".to_string(), TechnologyNode {
        id: "basic_survival".to_string(),
        name: "Basic Survival".to_string(),
        description: "Learn to gather basic materials with your hands".to_string(),
        category: "survival".to_string(),
        tier: 1,
        position: TechPosition { x: 0, y: 0 },
        prerequisites: vec![],
        research_cost: 0,
        material_costs: HashMap::new(),
        unlocks: TechUnlocks {
            items: vec![],
            abilities: vec!["hand_gathering".to_string()],
            recipes: vec![],
            buildings: vec![],
            research_bonus: Some(1),
        },
    });

    // Tool making tech
    let mut tool_costs = HashMap::new();
    tool_costs.insert(MaterialType::Wood, 5);
    tool_costs.insert(MaterialType::Stone, 5);
    
    technologies.insert("tool_making".to_string(), TechnologyNode {
        id: "tool_making".to_string(),
        name: "Tool Making".to_string(),
        description: "Craft basic tools from wood and stone".to_string(),
        category: "survival".to_string(),
        tier: 2,
        position: TechPosition { x: 1, y: 0 },
        prerequisites: vec!["basic_survival".to_string()],
        research_cost: 10,
        material_costs: tool_costs,
        unlocks: TechUnlocks {
            items: vec!["wooden_axe".to_string(), "stone_pick".to_string()],
            abilities: vec![],
            recipes: vec!["wooden_axe_recipe".to_string()],
            buildings: vec![],
            research_bonus: None,
        },
    });

    TechnologyTree {
        technologies,
        categories,
        version: "1.0".to_string(),
        starting_techs: vec!["basic_survival".to_string()],
    }
}

pub struct SkywardPlugin;

impl Plugin for SkywardPlugin {
    fn build(&self, app: &mut App) {
        app
            // Insert game resources
            .insert_resource(GameState::Playing)
            .insert_resource(Inventory::default())
            .insert_resource(create_default_tech_tree())
            
            // Add input manager plugin
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            
            // Add startup systems
            .add_systems(Startup, (
                setup_lighting,
                setup_world,
                systems::spawn_player,
            ))
            
            // Add update systems
            .add_systems(Update, (
                systems::handle_player_input,
                systems::update_camera_follow,
                systems::handle_equipment_switching,
                systems::handle_attack_action,
                systems::handle_special_abilities,
                systems::handle_interactions,
                systems::building_system,
                // Technology systems
                systems::handle_technology_input,
                systems::update_technology_availability,
                systems::passive_research_generation,
                systems::auto_unlock_technologies,
                systems::display_research_status,
                systems::initialize_starting_technologies,
                ui::ui_update,
            ));
    }
}

#[derive(Resource, Debug, Clone, PartialEq)]
pub enum GameState {
    MainMenu,
    Playing,
    Paused,
}

fn setup_camera(mut commands: Commands) {
    // 3rd person camera setup
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_lighting(mut commands: Commands) {
    // Directional light (sun)
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -std::f32::consts::FRAC_PI_4)),
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 300.0,
        affects_lightmapped_meshes: true,
    });
}

fn setup_world(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // This will be expanded to generate the procedural world
    // For now, just add a ground plane
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(100.0, 100.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 1.0, 0.0),
            ..default()
        })),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
}
