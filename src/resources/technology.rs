use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::components::{TechnologyNode, TechCategory, TechnologyProgress, MaterialType};

#[derive(Resource, Debug, Clone)]
pub struct TechnologyTree {
    pub technologies: HashMap<String, TechnologyNode>,
    pub categories: HashMap<String, TechCategory>,
    pub version: String,
    pub starting_techs: Vec<String>,
}

impl Default for TechnologyTree {
    fn default() -> Self {
        Self {
            technologies: HashMap::new(),
            categories: HashMap::new(),
            version: "1.0".to_string(),
            starting_techs: vec!["basic_survival".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechTreeData {
    pub version: String,
    pub starting_techs: Vec<String>,
    pub categories: HashMap<String, TechCategory>,
    pub technologies: Vec<TechnologyNode>,
}

impl TechnologyTree {
    pub fn from_data(data: TechTreeData) -> Self {
        let mut tech_map = HashMap::new();
        for tech in data.technologies {
            tech_map.insert(tech.id.clone(), tech);
        }

        Self {
            technologies: tech_map,
            categories: data.categories,
            version: data.version,
            starting_techs: data.starting_techs,
        }
    }

    pub fn get_technology(&self, id: &str) -> Option<&TechnologyNode> {
        self.technologies.get(id)
    }

    pub fn get_category(&self, id: &str) -> Option<&TechCategory> {
        self.categories.get(id)
    }

    pub fn can_research(&self, tech_id: &str, progress: &TechnologyProgress) -> bool {
        if let Some(tech) = self.get_technology(tech_id) {
            // Check if already unlocked
            if progress.unlocked_techs.contains(tech_id) {
                return false;
            }

            // Check prerequisites
            tech.prerequisites.iter().all(|prereq| {
                progress.unlocked_techs.contains(prereq)
            })
        } else {
            false
        }
    }

    pub fn get_research_cost(&self, tech_id: &str) -> Option<u32> {
        self.get_technology(tech_id).map(|tech| tech.research_cost)
    }

    pub fn get_material_costs(&self, tech_id: &str) -> Option<&HashMap<MaterialType, u32>> {
        self.get_technology(tech_id).map(|tech| &tech.material_costs)
    }

    pub fn update_available_techs(&self, progress: &mut TechnologyProgress) {
        progress.available_techs.clear();
        
        for (tech_id, _tech) in &self.technologies {
            if !progress.unlocked_techs.contains(tech_id) && self.can_research(tech_id, progress) {
                progress.available_techs.insert(tech_id.clone());
            }
        }
    }

    pub fn unlock_technology(&self, tech_id: &str, progress: &mut TechnologyProgress) -> bool {
        if self.can_research(tech_id, progress) {
            if let Some(tech) = self.get_technology(tech_id) {
                // Check if player has enough research points
                if progress.research_points >= tech.research_cost {
                    progress.research_points -= tech.research_cost;
                    progress.unlocked_techs.insert(tech_id.to_string());
                    
                    // Update available techs after unlocking
                    self.update_available_techs(progress);
                    
                    info!("Technology unlocked: {} - {}", tech.name, tech.description);
                    
                    // Log what was unlocked
                    if !tech.unlocks.items.is_empty() {
                        info!("  Unlocked items: {:?}", tech.unlocks.items);
                    }
                    if !tech.unlocks.abilities.is_empty() {
                        info!("  Unlocked abilities: {:?}", tech.unlocks.abilities);
                    }
                    if !tech.unlocks.recipes.is_empty() {
                        info!("  Unlocked recipes: {:?}", tech.unlocks.recipes);
                    }
                    if !tech.unlocks.buildings.is_empty() {
                        info!("  Unlocked buildings: {:?}", tech.unlocks.buildings);
                    }
                    
                    return true;
                } else {
                    info!("Not enough research points to unlock {}", tech.name);
                }
            }
        } else {
            info!("Cannot research technology: prerequisites not met");
        }
        false
    }

    pub fn get_technologies_by_category(&self, category: &str) -> Vec<&TechnologyNode> {
        self.technologies
            .values()
            .filter(|tech| tech.category == category)
            .collect()
    }

    pub fn get_technologies_by_tier(&self, tier: u32) -> Vec<&TechnologyNode> {
        self.technologies
            .values()
            .filter(|tech| tech.tier == tier)
            .collect()
    }
}

// Research point generation sources
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResearchSource {
    MaterialConversion,
    TimeBasedResearch,
    ExperienceGain,
    Discovery,
}

impl TechnologyProgress {
    pub fn add_research_points(&mut self, amount: u32, source: ResearchSource) {
        self.research_points += amount;
        self.total_points_earned += amount;
        
        match source {
            ResearchSource::MaterialConversion => {
                info!("Gained {} research points from material conversion", amount);
            }
            ResearchSource::TimeBasedResearch => {
                info!("Gained {} research points from passive research", amount);
            }
            ResearchSource::ExperienceGain => {
                info!("Gained {} research points from experience", amount);
            }
            ResearchSource::Discovery => {
                info!("Gained {} research points from discovery!", amount);
            }
        }
    }

    pub fn can_afford_research(&self, tech_tree: &TechnologyTree, tech_id: &str) -> bool {
        if let Some(cost) = tech_tree.get_research_cost(tech_id) {
            self.research_points >= cost
        } else {
            false
        }
    }

    pub fn start_research(&mut self, tech_id: String) {
        self.current_research = Some(tech_id.clone());
        self.research_progress = 0.0;
        info!("Started researching: {}", tech_id);
    }

    pub fn cancel_research(&mut self) {
        if let Some(tech_id) = &self.current_research {
            info!("Cancelled research: {}", tech_id);
        }
        self.current_research = None;
        self.research_progress = 0.0;
    }

    pub fn complete_research(&mut self) -> Option<String> {
        if let Some(tech_id) = self.current_research.take() {
            self.research_progress = 0.0;
            Some(tech_id)
        } else {
            None
        }
    }
}
