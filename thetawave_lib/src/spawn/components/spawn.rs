use crate::{
    entities::{ConsumableType, EffectType, ItemType, MobType, SpawnableType},
    resources::SpriteSheetsResource,
    spawnable::resources::{ConsumablesResource, EffectsResource, ItemsResource, MobsResource},
};

use amethyst::{
    core::{math::Vector2, transform::Transform},
    ecs::prelude::{Component, DenseVecStorage, Entities, LazyUpdate, ReadExpect},
};

use serde::{Deserialize, Serialize};

/// Used for periodically spawning spawnable entities
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoSpawnerComponent {
    /// Type of spawnable entity to spawn
    pub child_entity_type: SpawnableType,
    /// Offset position from entity's Transform translation
    pub offset: Vector2<f32>,
    /// Time period in seconds between spawns
    period: f32,
    /// Stores current time until next spawn
    timer: f32,
}

impl Component for AutoSpawnerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl AutoSpawnerComponent {
    /// Spawn `child_entity_type` with `period` interval (call every frame)
    pub fn spawn_when_ready(
        &mut self,
        delta_time: f32,
        spawn_transform: Transform,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        mobs_resource: &ReadExpect<MobsResource>,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        items_resource: &ReadExpect<ItemsResource>,
        effects_resource: &ReadExpect<EffectsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        // update timer
        self.timer -= delta_time;

        // reset timer to period and spawn entity when timer is less than 0.0
        if self.timer < 0.0 {
            // reset timer to period
            self.timer = self.period;

            // add offset position to translation position of entity
            let mut adjusted_transform = spawn_transform;
            adjusted_transform.prepend_translation_x(self.offset.x);
            adjusted_transform.prepend_translation_y(self.offset.y);

            // call spawn function for spawning child_entity_type
            match &self.child_entity_type {
                SpawnableType::Mob(mob_type) => {
                    mobs_resource.spawn_mob(
                        mob_type,
                        adjusted_transform,
                        spritesheets_resource,
                        entities,
                        lazy_update,
                    );
                }

                SpawnableType::Consumable(consumable_type) => {
                    consumables_resource.spawn_consumable(
                        consumable_type,
                        false,
                        adjusted_transform,
                        spritesheets_resource,
                        entities,
                        lazy_update,
                    );
                }

                SpawnableType::Item(item_type) => {
                    items_resource.spawn_item(
                        item_type,
                        false,
                        adjusted_transform,
                        spritesheets_resource,
                        entities,
                        lazy_update,
                    );
                }

                SpawnableType::Effect(effect_type) => {
                    effects_resource.spawn_effect(
                        effect_type,
                        adjusted_transform,
                        spritesheets_resource,
                        entities,
                        lazy_update,
                    );
                }
            }
        }
    }
}

/// Used for periodically spawning mob entities
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoMobSpawnerComponent {
    /// Type of mob entity to spawn
    pub child_mob_type: MobType,
    /// Offset position from entity's Transform translation
    pub offset: Vector2<f32>,
    /// Time period in seconds between spawns
    period: f32,
    /// Stores current time until next spawn
    timer: f32,
}

impl Component for AutoMobSpawnerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl AutoMobSpawnerComponent {
    /// Spawn `child_mob_type` with `period` interval (call every frame)
    #[allow(dead_code)]
    pub fn spawn_when_ready(
        &mut self,
        delta_time: f32,
        spawn_transform: Transform,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        mobs_resource: &ReadExpect<MobsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        // update timer
        self.timer -= delta_time;

        // reset timer to period and spawn entity when timer is less than 0.0
        if self.timer < 0.0 {
            // reset timer to period
            self.timer = self.period;

            // add offset position to translation position of entity
            let mut adjusted_transform = spawn_transform;
            adjusted_transform.prepend_translation_x(self.offset.x);
            adjusted_transform.prepend_translation_y(self.offset.y);

            // call mob spawn function
            mobs_resource.spawn_mob(
                &self.child_mob_type,
                adjusted_transform,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }
    }
}

/// Used for periodically spawning consumable entities
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoConsumableSpawnerComponent {
    /// Type of consumable entity to spawn
    pub child_consumable_type: ConsumableType,
    /// Offset position from entity's Transform translation
    pub offset: Vector2<f32>,
    /// Time period in seconds between spawns
    period: f32,
    /// Stores current time until next spawn
    timer: f32,
}

impl Component for AutoConsumableSpawnerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl AutoConsumableSpawnerComponent {
    /// Spawn `child_consumable_type` with `period` interval (call every frame)
    #[allow(dead_code)]
    pub fn spawn_when_ready(
        &mut self,
        delta_time: f32,
        spawn_transform: Transform,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        // update timer
        self.timer -= delta_time;

        // reset timer to period and spawn entity when timer is less than 0.0
        if self.timer < 0.0 {
            // reset timer to period
            self.timer = self.period;

            // add offset position to translation position of entity
            let mut adjusted_transform = spawn_transform;
            adjusted_transform.prepend_translation_x(self.offset.x);
            adjusted_transform.prepend_translation_y(self.offset.y);

            // call consumable spawn function
            consumables_resource.spawn_consumable(
                &self.child_consumable_type,
                false,
                adjusted_transform,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }
    }
}

/// Used for periodically spawning item entities
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoItemSpawnerComponent {
    /// Type of item entity to spawn
    pub child_item_type: ItemType,
    /// Offset position from entity's Transform translation
    pub offset: Vector2<f32>,
    /// Time period in seconds between spawns
    period: f32,
    /// Stores current time until next spawn
    timer: f32,
}

impl Component for AutoItemSpawnerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl AutoItemSpawnerComponent {
    /// Spawn `child_item_type` with `period` interval (call every frame)
    #[allow(dead_code)]
    pub fn spawn_when_ready(
        &mut self,
        delta_time: f32,
        spawn_transform: Transform,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        items_resource: &ReadExpect<ItemsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        // update timer
        self.timer -= delta_time;

        // reset timer to period and spawn entity when timer is less than 0.0
        if self.timer < 0.0 {
            // reset timer to period
            self.timer = self.period;

            // add offset position to translation position of entity
            let mut adjusted_transform = spawn_transform;
            adjusted_transform.prepend_translation_x(self.offset.x);
            adjusted_transform.prepend_translation_y(self.offset.y);

            // call item spawn function
            items_resource.spawn_item(
                &self.child_item_type,
                false,
                adjusted_transform,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }
    }
}

/// Used for periodically spawning effect entities
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoEffectSpawnerComponent {
    /// Type of effect entity to spawn
    pub child_effect_type: EffectType,
    /// Offset position from entity's Transform translation
    pub offset: Vector2<f32>,
    /// Time period in seconds between spawns
    period: f32,
    /// Stores current time until next spawn
    timer: f32,
}

impl Component for AutoEffectSpawnerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl AutoEffectSpawnerComponent {
    /// Spawn `child_effect_type` with `period` interval (call every frame)
    #[allow(dead_code)]
    pub fn spawn_when_ready(
        &mut self,
        delta_time: f32,
        spawn_transform: Transform,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        effects_resource: &ReadExpect<EffectsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        // update timer
        self.timer -= delta_time;

        // reset timer to period and spawn entity when timer is less than 0.0
        if self.timer < 0.0 {
            // reset timer to period
            self.timer = self.period;

            // add offset position to translation position of entity
            let mut adjusted_transform = spawn_transform;
            adjusted_transform.prepend_translation_x(self.offset.x);
            adjusted_transform.prepend_translation_y(self.offset.y);

            // call effect spawn function
            effects_resource.spawn_effect(
                &self.child_effect_type,
                adjusted_transform,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }
    }
}
