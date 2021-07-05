use crate::{
    constants::{ARENA_MAX_X, ARENA_MAX_Y, ARENA_MIN_X, ARENA_SPAWN_OFFSET, SPAWNER_Y_OFFSET},
    entities::SpawnableType,
    resources::{InvasionFormationPool, InvasionRandomPool, SpriteSheetsResource},
    spawnable::resources::{
        spawn_spawnable, ConsumablesResource, EffectsResource, ItemsResource, MobsResource,
    },
};
use amethyst::{
    core::{math::Vector2, transform::Transform},
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
};

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Used for storing formation spawnable data
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FormationSpawnable {
    /// Spawnable entity type
    pub spawnable_type: SpawnableType,
    /// Position in the formation
    pub position: Vector2<f32>,
}

/// Used for spawning an organized formation of spawnables
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Formation {
    /// Vector of formation spawnables (positions and entity types)
    pub formation_spawnables: Vec<FormationSpawnable>,
    /// Random weight (higher number = more likely to be spawned)
    pub weight: f32,
    /// Time until next wave is spawned
    pub period: f32,
}

impl Formation {
    /// Spawn all entities in formation at their positions
    pub fn spawn_formation(
        &self,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        mobs_resource: &ReadExpect<MobsResource>,
        items_resource: &ReadExpect<ItemsResource>,
        effects_resource: &ReadExpect<EffectsResource>,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        for formation_spawnable in self.formation_spawnables.iter() {
            let mut spawn_transform = Transform::default();
            spawn_transform.set_translation_xyz(
                formation_spawnable.position.x,
                formation_spawnable.position.y,
                0.0,
            );

            spawn_spawnable(
                &formation_spawnable.spawnable_type,
                false,
                spawn_transform,
                consumables_resource,
                mobs_resource,
                items_resource,
                effects_resource,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }
    }
}

/// Used for storing data about a spawnable entity in a random pool
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RandomSpawnable {
    /// Optional spawnable entity type
    pub spawnable_type: Option<SpawnableType>,
    /// Random weight (higher number = more likely to be spawned)
    pub weight: f32,
    /// Time until next wave is spawned
    pub period: f32,
}

/// Used for spawning entities/formations from pools
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SpawnerResource {
    /// Pools of entities to be spawned randomly
    pub random_pools: HashMap<InvasionRandomPool, Vec<RandomSpawnable>>,
    /// Pools of formations to be spawned randomly
    pub formation_pools: HashMap<InvasionFormationPool, Vec<Formation>>,
    /// Counts down time between spawns
    pub timer: f32,
}

impl SpawnerResource {
    /// Choose a random X position within the boundaries of the arena
    fn choose_spawn_position() -> f32 {
        let max_width = ARENA_MAX_X - ARENA_SPAWN_OFFSET;
        let min_width = ARENA_MIN_X + ARENA_SPAWN_OFFSET;
        ARENA_MIN_X + ARENA_SPAWN_OFFSET + rand::thread_rng().gen::<f32>() * (max_width - min_width)
    }

    /// Choose a random spawnable type from the `random_pool` of entities
    fn choose_random_spawnable(&self, random_pool_type: &InvasionRandomPool) -> &RandomSpawnable {
        let random_pool = &self.random_pools[random_pool_type];

        let prob_space = random_pool
            .iter()
            .fold(0.0, |sum, rand_spawnable| sum + rand_spawnable.weight);

        let pos = rand::thread_rng().gen::<f32>() * prob_space;
        let mut sum = 0.0;
        for random_spawnable in random_pool.iter() {
            sum += random_spawnable.weight;
            if sum > pos {
                return random_spawnable;
            }
        }
        unreachable!("Error in probabilities of random spawnable pool.");
    }

    /// Spawn a random spawnable from `random_pools` given a type of pool
    pub fn spawn_random_spawnable_when_ready(
        &mut self,
        random_pool_type: &InvasionRandomPool,
        dt: f32,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        mobs_resource: &ReadExpect<MobsResource>,
        items_resource: &ReadExpect<ItemsResource>,
        effects_resource: &ReadExpect<EffectsResource>,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        self.timer -= dt;

        if self.timer <= 0.0 {
            let random_spawnable = self.choose_random_spawnable(random_pool_type);
            let mut spawn_transform = Transform::default();
            spawn_transform.set_translation_xyz(
                Self::choose_spawn_position(),
                ARENA_MAX_Y + SPAWNER_Y_OFFSET,
                0.0,
            );

            if let Some(spawnable_type) = &random_spawnable.spawnable_type {
                spawn_spawnable(
                    spawnable_type,
                    false,
                    spawn_transform,
                    consumables_resource,
                    mobs_resource,
                    items_resource,
                    effects_resource,
                    spritesheets_resource,
                    entities,
                    lazy_update,
                );
            }

            self.timer = random_spawnable.period;
        }
    }

    /// Choose a random formation from the formation pool of the given type from `formation_pools`
    fn choose_random_formation(&self, formation_pool_type: &InvasionFormationPool) -> &Formation {
        let formation_pool = &self.formation_pools[formation_pool_type];

        let prob_space = formation_pool
            .iter()
            .fold(0.0, |sum, formation| sum + formation.weight);

        let pos = rand::thread_rng().gen::<f32>() * prob_space;
        let mut sum = 0.0;
        for formation in formation_pool.iter() {
            sum += formation.weight;
            if sum > pos {
                return formation;
            }
        }
        unreachable!("Error in probabilities of formation pool.");
    }

    /// Spawn a random formation from `formation_pools` given a type of pool
    pub fn spawn_random_formation_when_ready(
        &mut self,
        formation_pool_type: &InvasionFormationPool,
        dt: f32,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        mobs_resource: &ReadExpect<MobsResource>,
        items_resource: &ReadExpect<ItemsResource>,
        effects_resource: &ReadExpect<EffectsResource>,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        self.timer -= dt;

        if self.timer <= 0.0 {
            let formation = self.choose_random_formation(formation_pool_type);
            formation.spawn_formation(
                consumables_resource,
                mobs_resource,
                items_resource,
                effects_resource,
                spritesheets_resource,
                entities,
                lazy_update,
            );

            self.timer = formation.period;
        }
    }
}
