use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use leafwing_input_manager::prelude::*;
use crate::components::{Player, PlayerController, PlayerCamera, CameraTarget, PlayerAction, ToolType, ItemType, Item, EquippedItem, WeaponType, ConsumableType, TechnologyProgress};

// Input map configuration
pub fn default_input_map() -> InputMap<PlayerAction> {
    let mut input_map = InputMap::default();

    // Movement - WASD
    input_map.insert_dual_axis(PlayerAction::Move, VirtualDPad::wasd());
    
    // Camera rotation - Right mouse button
    input_map.insert(PlayerAction::RotateCamera, MouseButton::Right);
    
    // Equipment slots
    input_map.insert(PlayerAction::EquipSlot1, KeyCode::Digit1);
    input_map.insert(PlayerAction::EquipSlot2, KeyCode::Digit2);
    input_map.insert(PlayerAction::EquipSlot3, KeyCode::Digit3);
    input_map.insert(PlayerAction::EquipSlot4, KeyCode::Digit4);
    
    // Slots 5-8 using Z,X,C,V for now (will implement shift+numbers later)
    input_map.insert(PlayerAction::EquipSlot5, KeyCode::KeyZ);
    input_map.insert(PlayerAction::EquipSlot6, KeyCode::KeyX);
    input_map.insert(PlayerAction::EquipSlot7, KeyCode::KeyC);
    input_map.insert(PlayerAction::EquipSlot8, KeyCode::KeyV);
    
    // Actions
    input_map.insert(PlayerAction::Attack, MouseButton::Left);
    input_map.insert(PlayerAction::Interact, KeyCode::KeyF);
    input_map.insert(PlayerAction::SpecialAbility1, KeyCode::KeyQ);
    input_map.insert(PlayerAction::SpecialAbility2, KeyCode::KeyE);
    input_map.insert(PlayerAction::ToggleBuildMode, KeyCode::KeyB);

    input_map
}

// Spawn player system
pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn player entity with a simple capsule mesh
    let _player_entity = commands.spawn((
        Player::default(),
        PlayerController::default(),
        TechnologyProgress::default(),
        CameraTarget,
        Mesh3d(meshes.add(Capsule3d::new(0.5, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.0, 1.0),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.0, 0.0),
        ActionState::<PlayerAction>::default(),
        default_input_map(),
    )).id();

    // Spawn camera as a separate entity
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 6.0, 10.0)
            .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        PlayerCamera::default(),
    ));
}

// Handle player input and movement
pub fn handle_player_input(
    time: Res<Time>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut player_query: Query<(
        &ActionState<PlayerAction>,
        &mut Transform,
        &Player,
    ), With<Player>>,
    mut camera_query: Query<(&mut PlayerCamera, &mut Transform), (With<PlayerCamera>, Without<Player>)>,
) {
    let Ok((action_state, mut player_transform, player)) = player_query.single_mut() else {
        return;
    };

    let Ok((mut camera, _camera_transform)) = camera_query.single_mut() else {
        return;
    };

    let delta_time = time.delta_secs();

    // Handle camera rotation when right-click is held
    camera.is_rotating = action_state.pressed(&PlayerAction::RotateCamera);

    // Process mouse motion for camera rotation
    if camera.is_rotating {
        for mouse_event in mouse_motion.read() {
            camera.yaw -= mouse_event.delta.x * camera.sensitivity;
            camera.pitch -= mouse_event.delta.y * camera.sensitivity;
            
            // Clamp pitch to prevent camera flipping
            camera.pitch = camera.pitch.clamp(-1.5, 1.5);
        }
    }

    // Handle movement input
    let movement_vector = action_state.clamped_axis_pair(&PlayerAction::Move);
    
    if movement_vector.length() > 0.1 {
        // Get camera forward and right vectors (ignoring pitch for movement)
        let camera_yaw = Quat::from_rotation_y(camera.yaw);
        let forward = camera_yaw * Vec3::NEG_Z;
        let right = camera_yaw * Vec3::X;
        
        // Calculate movement direction relative to camera
        let move_direction = (forward * movement_vector.y + right * movement_vector.x).normalize();
        
        // Move player
        player_transform.translation += move_direction * player.movement_speed * delta_time;
        
        // Rotate player to face movement direction
        player_transform.rotation = Quat::from_rotation_y(move_direction.z.atan2(move_direction.x) - std::f32::consts::FRAC_PI_2);
    }
}

// Update camera to follow player
pub fn update_camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
    mut camera_query: Query<(&PlayerCamera, &mut Transform), (With<PlayerCamera>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let Ok((camera, mut camera_transform)) = camera_query.single_mut() else {
        return;
    };

    // Calculate camera position based on player position and camera angles
    let rotation = Quat::from_rotation_y(camera.yaw) * Quat::from_rotation_x(camera.pitch);
    let offset = rotation * camera.offset;
    
    camera_transform.translation = player_transform.translation + offset;
    camera_transform.look_at(player_transform.translation + Vec3::Y, Vec3::Y);
}

// Handle equipment slot switching
pub fn handle_equipment_switching(
    mut player_query: Query<(&ActionState<PlayerAction>, &mut PlayerController)>,
) {
    let Ok((action_state, mut controller)) = player_query.single_mut() else {
        return;
    };

    // Check for equipment slot switching
    if action_state.just_pressed(&PlayerAction::EquipSlot1) {
        controller.equipment.active_slot = 0;
        info!("Switched to equipment slot 1");
    } else if action_state.just_pressed(&PlayerAction::EquipSlot2) {
        controller.equipment.active_slot = 1;
        info!("Switched to equipment slot 2");
    } else if action_state.just_pressed(&PlayerAction::EquipSlot3) {
        controller.equipment.active_slot = 2;
        info!("Switched to equipment slot 3");
    } else if action_state.just_pressed(&PlayerAction::EquipSlot4) {
        controller.equipment.active_slot = 3;
        info!("Switched to equipment slot 4");
    } else if action_state.just_pressed(&PlayerAction::EquipSlot5) {
        controller.equipment.active_slot = 4;
        info!("Switched to equipment slot 5");
    } else if action_state.just_pressed(&PlayerAction::EquipSlot6) {
        controller.equipment.active_slot = 5;
        info!("Switched to equipment slot 6");
    } else if action_state.just_pressed(&PlayerAction::EquipSlot7) {
        controller.equipment.active_slot = 6;
        info!("Switched to equipment slot 7");
    } else if action_state.just_pressed(&PlayerAction::EquipSlot8) {
        controller.equipment.active_slot = 7;
        info!("Switched to equipment slot 8");
    }

    // Display current active item
    let active_slot = controller.equipment.active_slot;
    if let Some(equipped_item) = &controller.equipment.slots[active_slot] {
        if action_state.just_pressed(&PlayerAction::EquipSlot1) ||
           action_state.just_pressed(&PlayerAction::EquipSlot2) ||
           action_state.just_pressed(&PlayerAction::EquipSlot3) ||
           action_state.just_pressed(&PlayerAction::EquipSlot4) ||
           action_state.just_pressed(&PlayerAction::EquipSlot5) ||
           action_state.just_pressed(&PlayerAction::EquipSlot6) ||
           action_state.just_pressed(&PlayerAction::EquipSlot7) ||
           action_state.just_pressed(&PlayerAction::EquipSlot8) {
            info!("Equipped: {}", equipped_item.item.name);
        }
    } else {
        if action_state.just_pressed(&PlayerAction::EquipSlot1) ||
           action_state.just_pressed(&PlayerAction::EquipSlot2) ||
           action_state.just_pressed(&PlayerAction::EquipSlot3) ||
           action_state.just_pressed(&PlayerAction::EquipSlot4) ||
           action_state.just_pressed(&PlayerAction::EquipSlot5) ||
           action_state.just_pressed(&PlayerAction::EquipSlot6) ||
           action_state.just_pressed(&PlayerAction::EquipSlot7) ||
           action_state.just_pressed(&PlayerAction::EquipSlot8) {
            info!("Empty slot - bare hands");
        }
    }
}

// Handle attack and use actions
pub fn handle_attack_action(
    mut player_query: Query<(&ActionState<PlayerAction>, &PlayerController)>,
) {
    let Ok((action_state, controller)) = player_query.single_mut() else {
        return;
    };

    if action_state.just_pressed(&PlayerAction::Attack) {
        let active_slot = controller.equipment.active_slot;
        
        match &controller.equipment.slots[active_slot] {
            Some(equipped_item) => {
                match &equipped_item.item.item_type {
                    ItemType::Tool(tool_type) => {
                        info!("Using tool: {:?} to harvest", tool_type);
                        // TODO: Implement harvesting logic
                    },
                    ItemType::Weapon(weapon_type) => {
                        info!("Attacking with weapon: {:?}", weapon_type);
                        // TODO: Implement weapon attack logic
                    },
                    ItemType::Consumable(consumable_type) => {
                        info!("Using consumable: {:?}", consumable_type);
                        // TODO: Implement consumable use logic
                    },
                    ItemType::Material(material_type) => {
                        info!("Using material: {:?}", material_type);
                        // TODO: Implement material use logic
                    },
                }
            },
            None => {
                info!("Unarmed attack - punching with bare hands!");
                // TODO: Implement unarmed attack logic
            }
        }
    }
}

// Handle special abilities
pub fn handle_special_abilities(
    mut player_query: Query<(&ActionState<PlayerAction>, &PlayerController)>,
) {
    let Ok((action_state, controller)) = player_query.single_mut() else {
        return;
    };

    if action_state.just_pressed(&PlayerAction::SpecialAbility1) {
        if let Some(ability) = &controller.special_abilities[0] {
            info!("Using special ability 1: {}", ability.name);
            // TODO: Implement ability use logic
        } else {
            info!("No special ability assigned to Q key");
        }
    }

    if action_state.just_pressed(&PlayerAction::SpecialAbility2) {
        if let Some(ability) = &controller.special_abilities[1] {
            info!("Using special ability 2: {}", ability.name);
            // TODO: Implement ability use logic
        } else {
            info!("No special ability assigned to E key");
        }
    }
}

// Handle interactions and build mode
pub fn handle_interactions(
    mut player_query: Query<(&ActionState<PlayerAction>, &PlayerController)>,
) {
    let Ok((action_state, _controller)) = player_query.single_mut() else {
        return;
    };

    if action_state.just_pressed(&PlayerAction::Interact) {
        info!("Interact action pressed (F key)");
        // TODO: Implement interaction logic
    }

    if action_state.just_pressed(&PlayerAction::ToggleBuildMode) {
        info!("Build mode toggled (B key)");
        // TODO: Implement build mode toggle logic
    }
}

// Legacy function names for compatibility with existing lib.rs
pub fn handle_tool_switching(
    player_query: Query<(&ActionState<PlayerAction>, &mut PlayerController)>,
) {
    handle_equipment_switching(player_query);
}

pub fn resource_interaction(
    player_query: Query<(&ActionState<PlayerAction>, &PlayerController)>,
) {
    handle_attack_action(player_query);
}
