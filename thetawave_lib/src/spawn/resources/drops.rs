use crate::{
    spawnable::{spawn_spawnable, SpawnableResources, SpawnableType},
    tools::weighted_rng,
    visual::SpriteSheetsResource,
};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, LazyUpdate},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type DropTablesResource = HashMap<DropTableType, DropProbabilities>;
pub type DropProbabilities = Vec<(SpawnableType, f32)>;
pub type RollProbabilities = Vec<(Option<DropTableType>, f32)>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DropRolls {
    pub roll_count: u8,
    pub roll_probs: RollProbabilities,
}

impl DropRolls {
    fn choose_drop_table(roll_probs: &RollProbabilities) -> &Option<DropTableType> {
        let probs = roll_probs.iter().map(|roll_prob| roll_prob.1).collect();
        &roll_probs[weighted_rng(probs)].0
    }

    fn choose_drop(drop_probs: &DropProbabilities) -> &SpawnableType {
        let probs = drop_probs.iter().map(|drop_prob| drop_prob.1).collect();
        &drop_probs[weighted_rng(probs)].0
    }

    pub fn spawn(
        &self,
        spawn_transform: &Transform,
        drop_tables_resource: &DropTablesResource,
        spawnable_resources: &SpawnableResources,
        spritesheets_resource: &SpriteSheetsResource,
        entities: &Entities,
        lazy_update: &LazyUpdate,
    ) {
        for _ in 0..self.roll_count {
            // pick a drop table
            if let Some(drop_table) = Self::choose_drop_table(&self.roll_probs) {
                // spawn a drop from the table
                spawn_spawnable(
                    Self::choose_drop(&drop_tables_resource[drop_table]),
                    true,
                    spawn_transform,
                    spawnable_resources,
                    spritesheets_resource,
                    entities,
                    lazy_update,
                )
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum DropTableType {
    Standard,
    Boss,
    MoneyAsteroid,
}
