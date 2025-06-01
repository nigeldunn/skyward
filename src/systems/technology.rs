use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::components::{PlayerController, TechnologyProgress, PlayerAction};
use crate::resources::{TechnologyTree, Inventory, ResearchSource};

// System to handle technology research input
pub fn handle_technology_input(
    mut player_query: Query<(&ActionState<PlayerAction>, &mut TechnologyProgress)>,
    _tech_tree: Res<TechnologyTree>,
    mut inventory: ResMut<Inventory>,
) {
    let Ok((action_state, mut progress)) = player_query.single_mut() else {
        return;
    };

    // For now, use B key to convert materials to research points
    if action_state.just_pressed(&PlayerAction::ToggleBuildMode) {
        // Convert 5 wood to 1 research point as a test
        if inventory.get_amount(&crate::components::MaterialType::Wood) >= 5 {
            inventory.remove_material(crate::components::MaterialType::Wood, 5);
            progress.add_research_points(1, ResearchSource::MaterialConversion);
        } else {
            info!("Not enough wood to convert to research points (need 5 wood)");
        }
    }
}

// System to update available technologies based on unlocked prerequisites
pub fn update_technology_availability(
    mut player_query: Query<&mut TechnologyProgress>,
    tech_tree: Res<TechnologyTree>,
) {
    let Ok(mut progress) = player_query.single_mut() else {
        return;
    };

    // Update available techs whenever the tree changes
    if tech_tree.is_changed() || progress.is_changed() {
        tech_tree.update_available_techs(&mut progress);
    }
}

// System for passive research point generation
pub fn passive_research_generation(
    mut player_query: Query<&mut TechnologyProgress>,
    time: Res<Time>,
) {
    let Ok(mut progress) = player_query.single_mut() else {
        return;
    };

    // Generate 1 research point every 10 seconds as base rate
    let research_rate = 0.1; // 0.1 points per second = 1 point per 10 seconds
    let points_to_add = (research_rate * time.delta_secs()).round() as u32;
    
    if points_to_add > 0 {
        progress.add_research_points(points_to_add, ResearchSource::TimeBasedResearch);
    }
}

// System to handle automatic technology unlocking
pub fn auto_unlock_technologies(
    mut player_query: Query<&mut TechnologyProgress>,
    tech_tree: Res<TechnologyTree>,
    mut inventory: ResMut<Inventory>,
) {
    let Ok(mut progress) = player_query.single_mut() else {
        return;
    };

    // Auto-unlock available technologies if player has enough research points and materials
    let available_techs: Vec<String> = progress.available_techs.iter().cloned().collect();
    
    for tech_id in available_techs {
        if let Some(tech) = tech_tree.get_technology(&tech_id) {
            // Check if player can afford the research
            if progress.research_points >= tech.research_cost && 
               inventory.can_afford(&tech.material_costs) {
                
                // Spend materials
                if inventory.spend_materials(&tech.material_costs) {
                    // Unlock the technology
                    tech_tree.unlock_technology(&tech_id, &mut progress);
                }
            }
        }
    }
}

// System to display current research status
pub fn display_research_status(
    player_query: Query<&TechnologyProgress>,
    _tech_tree: Res<TechnologyTree>,
    inventory: Res<Inventory>,
    time: Res<Time>,
) {
    let Ok(progress) = player_query.single() else {
        return;
    };

    // Display research info every 5 seconds (for debugging)
    static mut LAST_UPDATE: f32 = 0.0;
    let current_time = time.elapsed_secs();
    
    unsafe {
        if current_time - LAST_UPDATE > 5.0 {
            LAST_UPDATE = current_time;
            
            info!("=== RESEARCH STATUS ===");
            info!("Research Points: {}", progress.research_points);
            info!("Total Points Earned: {}", progress.total_points_earned);
            info!("Unlocked Technologies: {:?}", progress.unlocked_techs);
            info!("Available Technologies: {:?}", progress.available_techs);
            
            info!("=== INVENTORY ===");
            for (material, amount) in &inventory.materials {
                if *amount > 0 {
                    info!("{:?}: {}", material, amount);
                }
            }
            
            info!("Press B to convert 5 Wood → 1 Research Point");
            info!("=======================");
        }
    }
}

// System to handle experience-based research point generation
pub fn experience_research_generation(
    mut _player_query: Query<&mut TechnologyProgress>,
    _controller_query: Query<&PlayerController>,
) {
    // Award research points for using tools/equipment
    // This would be triggered by other systems when tools are used
    // For now, this is a placeholder for the mechanic
}

// System to initialize starting technologies
pub fn initialize_starting_technologies(
    mut player_query: Query<&mut TechnologyProgress, Added<TechnologyProgress>>,
    tech_tree: Res<TechnologyTree>,
) {
    for mut progress in player_query.iter_mut() {
        // Ensure starting technologies are unlocked
        for tech_id in &tech_tree.starting_techs {
            progress.unlocked_techs.insert(tech_id.clone());
        }
        
        // Update available techs based on starting techs
        tech_tree.update_available_techs(&mut progress);
        
        info!("Technology system initialized with starting tech: {:?}", tech_tree.starting_techs);
    }
}
