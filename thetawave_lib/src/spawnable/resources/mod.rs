//! Resources for spawnable entities
use crate::{
    entities::SpawnableType,
    motion::components::{Hitbox2DComponent, Motion2DComponent},
    resources::SpriteSheetsResource,
    spawn::components::{DespawnAtBorderComponent, DespawnTimeLimitComponent},
    spawnable::components::BlastComponent,
};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Builder, Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
};

mod modifiers;
mod spawnables;

pub use self::{
    modifiers::{ConsumableModifiersResource, ItemModifiersResource, Modifier},
    spawnables::{
        ConsumableEntityData, ConsumablesResource, EffectEntityData, EffectsResource,
        ItemEntityData, ItemsResource, MobEntityData, MobsResource, RandomMotionRange2D,
        ThrusterEntityData,
    },
};

/// Spawn any kind of spawnable entity
pub fn spawn_spawnable(
    spawnable_type: &SpawnableType,
    is_drop: bool,
    spawn_transform: Transform,
    consumables_resource: &ReadExpect<ConsumablesResource>,
    mobs_resource: &ReadExpect<MobsResource>,
    items_resource: &ReadExpect<ItemsResource>,
    effects_resource: &ReadExpect<EffectsResource>,
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    match spawnable_type {
        SpawnableType::Consumable(consumable_type) => {
            consumables_resource.spawn_consumable(
                consumable_type,
                is_drop,
                spawn_transform,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }

        SpawnableType::Mob(mob_type) => {
            mobs_resource.spawn_mob(
                mob_type,
                spawn_transform,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }

        SpawnableType::Item(item_type) => {
            items_resource.spawn_item(
                item_type,
                is_drop,
                spawn_transform,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }

        SpawnableType::Effect(effect_type) => {
            effects_resource.spawn_effect(
                effect_type,
                spawn_transform,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }
    }
}

/// Spawn blast entities
pub fn spawn_blasts(
    blast_count: usize,
    blast_spacing: f32,
    blast_sprite_render: SpriteRender,
    blast_component: BlastComponent,
    blast_hitbox: Hitbox2DComponent,
    blast_motion2d: Motion2DComponent,
    despawn_time: f32,
    mut blast_transform: Transform,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    for _ in 0..blast_count {
        lazy_update
            .create_entity(entities)
            .with(blast_component.clone())
            .with(blast_hitbox.clone())
            .with(blast_motion2d.clone())
            .with(blast_sprite_render.clone())
            .with(blast_transform.clone())
            .with(Transparent)
            .with(DespawnTimeLimitComponent {
                duration: despawn_time,
            })
            // Also despawn at the border to avoid tracking entities that left the screen
            .with(DespawnAtBorderComponent {
                top_offset: Some(2.0),
                bottom_offset: Some(-2.0),
                left_offset: Some(-2.0),
                right_offset: Some(2.0),
            })
            .build();

        blast_transform.prepend_translation_x(blast_spacing);
    }
}
