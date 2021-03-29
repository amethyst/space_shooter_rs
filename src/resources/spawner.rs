use crate::{
    constants::{ARENA_MAX_X, ARENA_MAX_Y, ARENA_MIN_X, ARENA_SPAWN_OFFSET, SPAWNER_Y_OFFSET},
    entities::{spawn::spawn_spawnable, SpawnableType},
    resources::{
        ConsumablesResource, EffectsResource, EnemiesResource, ItemsResource, SpriteSheetsResource,
    },
};
use amethyst::{
    core::{math::Vector2, transform::Transform},
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
};

use rand::{seq::SliceRandom, thread_rng, Rng};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FormationSpawnable {
    pub spawnable_type: SpawnableType,
    pub position: Vector2<f32>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Formation {
    pub formation_spawnables: Vec<FormationSpawnable>,
}

impl Formation {
    pub fn spawn_formation(
        &self,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        enemies_resource: &ReadExpect<EnemiesResource>,
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
                spawn_transform,
                consumables_resource,
                enemies_resource,
                items_resource,
                effects_resource,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RandomSpawnable {
    pub spawnable_type: Option<SpawnableType>,
    pub weight: f32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SpawnerResource {
    pub random_pool: Vec<RandomSpawnable>, //TODO: change to HashMap of Vec<SpawnableType> for AsteroidField, Drones, etc.
    pub formation_pool: Vec<Formation>, //TODO: change to HashMap of Vec<Formation> for Level1Formations, Level2Formations, etc.
    pub period: f32,
    pub timer: f32,
}

impl SpawnerResource {
    fn choose_spawn_position() -> f32 {
        let max_width = ARENA_MAX_X - ARENA_SPAWN_OFFSET;
        let min_width = ARENA_MIN_X + ARENA_SPAWN_OFFSET;
        ARENA_MIN_X + ARENA_SPAWN_OFFSET + rand::thread_rng().gen::<f32>() * (max_width - min_width)
    }

    fn choose_random_spawnable(&self) -> &Option<SpawnableType> {
        let prob_space = self
            .random_pool
            .iter()
            .fold(0.0, |sum, rand_spawnable| sum + rand_spawnable.weight);

        let pos = rand::thread_rng().gen::<f32>() * prob_space;
        let mut sum = 0.0;
        for random_spawnable in self.random_pool.iter() {
            sum += random_spawnable.weight;
            if sum > pos {
                return &random_spawnable.spawnable_type;
            }
        }

        &None
    }

    pub fn spawn_random_spawnable_when_ready(
        &mut self,
        dt: f32,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        enemies_resource: &ReadExpect<EnemiesResource>,
        items_resource: &ReadExpect<ItemsResource>,
        effects_resource: &ReadExpect<EffectsResource>,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        self.timer -= dt;

        if self.timer <= 0.0 {
            self.timer = self.period;

            if let Some(spawnable_type) = self.choose_random_spawnable() {
                let mut spawn_transform = Transform::default();
                spawn_transform.set_translation_xyz(
                    Self::choose_spawn_position(),
                    ARENA_MAX_Y + SPAWNER_Y_OFFSET,
                    0.0,
                );

                spawn_spawnable(
                    spawnable_type,
                    spawn_transform,
                    consumables_resource,
                    enemies_resource,
                    items_resource,
                    effects_resource,
                    spritesheets_resource,
                    entities,
                    lazy_update,
                );
            }
        }
    }

    pub fn spawn_random_formation_when_ready(
        &mut self,
        dt: f32,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        enemies_resource: &ReadExpect<EnemiesResource>,
        items_resource: &ReadExpect<ItemsResource>,
        effects_resource: &ReadExpect<EffectsResource>,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        self.timer -= dt;

        if self.timer <= 0.0 {
            self.timer = self.period;

            self.formation_pool
                .choose(&mut rand::thread_rng())
                .unwrap()
                .spawn_formation(
                    consumables_resource,
                    enemies_resource,
                    items_resource,
                    effects_resource,
                    spritesheets_resource,
                    entities,
                    lazy_update,
                );
        }
    }
}