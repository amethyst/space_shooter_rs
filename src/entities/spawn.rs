use crate::{
    entities::{ConsumableType, EffectType, EnemyType, ItemType, SpawnableType},
    resources::{
        ConsumablesResource, EffectsResource, EnemiesResource, ItemsResource, SpriteSheetsResource,
    },
};
use amethyst::{
    core::{transform::Transform, Parent},
    ecs::prelude::{Builder, Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{palette::Srgba, resources::Tint, SpriteRender, Transparent},
};

use rand::{thread_rng, Rng};

pub fn spawn_consumable(
    consumable_type: &ConsumableType,
    spawn_transform: Transform,
    consumables_resource: &ReadExpect<ConsumablesResource>,
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let consumable_data = consumables_resource.consumable_entities[consumable_type].clone();

    let sprite_render = SpriteRender {
        sprite_sheet: spritesheets_resource.spritesheets
            [&consumable_data.sprite_render_data.spritesheet]
            .clone(),
        sprite_number: consumable_data.sprite_render_data.initial_index,
    };

    lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(consumable_data.hitbox_component.clone())
        .with(consumable_data.consumable_component)
        .with(consumables_resource.motion2d_component.clone())
        .with(spawn_transform)
        .with(Transparent)
        .with(consumables_resource.despawn_border_component.clone())
        .build();
}

pub fn spawn_enemy(
    enemy_type: &EnemyType,
    spawn_transform: Transform,
    enemies_resource: &ReadExpect<EnemiesResource>,
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) -> Entity {
    let enemy_data = enemies_resource[enemy_type].clone();

    let enemy_sprite_render = SpriteRender {
        sprite_sheet: spritesheets_resource.spritesheets
            [&enemy_data.sprite_render_data.spritesheet]
            .clone(),
        sprite_number: enemy_data.sprite_render_data.initial_index,
    };

    let enemy_entity = lazy_update
        .create_entity(entities)
        .with(enemy_sprite_render)
        .with(enemy_data.animation_component)
        .with(enemy_data.enemy_component)
        .with(enemy_data.hitbox_component)
        .with(enemy_data.motion2d_component)
        .with(enemy_data.health_component)
        .with(enemy_data.despawn_component)
        .with(spawn_transform)
        .with(Transparent)
        .build();

    if let Some(blaster_component) = enemy_data.blaster_component {
        lazy_update.insert(enemy_entity, blaster_component);
    }
    if let Some(autofire_component) = enemy_data.autofire_component {
        lazy_update.insert(enemy_entity, autofire_component);
    }
    if let Some(auto_child_entity_spawner_component) = enemy_data.auto_spawner_component {
        lazy_update.insert(enemy_entity, auto_child_entity_spawner_component);
    }

    // Spawn thruster entity as child of enemy entity

    if let Some(thruster_data) = enemy_data.thruster_data {
        let thruster_parent = Parent::new(enemy_entity);

        let thruster_sprite_render = SpriteRender {
            sprite_sheet: spritesheets_resource.spritesheets["thrusters"].clone(),
            sprite_number: thruster_data.animation_component.start_idx,
        };

        let mut thruster_transform = Transform::default();
        thruster_transform.set_translation_y(thruster_data.y_offset);

        lazy_update
            .create_entity(entities)
            .with(thruster_parent)
            .with(thruster_transform)
            .with(thruster_sprite_render)
            .with(thruster_data.animation_component)
            .with(Transparent)
            .build();
    }

    enemy_entity
}

pub fn spawn_item(
    item_type: &ItemType,
    spawn_transform: Transform,
    items_resource: &ReadExpect<ItemsResource>,
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let item_data = items_resource.item_entities[item_type].clone();

    let sprite_render = SpriteRender {
        sprite_sheet: spritesheets_resource.spritesheets[&item_data.sprite_render_data.spritesheet]
            .clone(),
        sprite_number: item_data.sprite_render_data.initial_index,
    };

    let item_entity = lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(item_data.item_component)
        .with(items_resource.hitbox2d_component.clone())
        .with(items_resource.motion2d_component.clone())
        .with(spawn_transform)
        .with(Transparent)
        .with(items_resource.despawn_border_component.clone())
        .build();

    if let Some(animation_component) = item_data.animation_component {
        lazy_update.insert(item_entity, animation_component);
    }
}

pub fn spawn_effect(
    effect_type: &EffectType,
    spawn_transform: Transform,
    effects_resource: &ReadExpect<EffectsResource>,
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let effect_data = &effects_resource[effect_type];

    for sprite_render_data in effect_data.sprite_render_data.iter() {
        // for each spriterender data in the array create an entity with a unique sprite render and the same other properties

        let sprite_render = SpriteRender {
            sprite_sheet: spritesheets_resource.spritesheets[&sprite_render_data.spritesheet]
                .clone(),
            sprite_number: sprite_render_data.initial_index,
        };

        let effect_entity = lazy_update
            .create_entity(entities)
            .with(sprite_render)
            .with(spawn_transform.clone())
            .with(Transparent)
            .build();

        // TODO: scale to be a vector (like with Sprite render)
        if let Some(animation_component) = effect_data.animation_component.clone() {
            lazy_update.insert(effect_entity, animation_component);
        }

        if let Some(time_limit_component) = effect_data.time_limit_component.clone() {
            lazy_update.insert(effect_entity, time_limit_component);
        }

        if let Some(mut motion2d_component) = effect_data.motion2d_component.clone() {
            if let Some(random_initial_motion) = effect_data.random_initial_motion.clone() {
                motion2d_component.velocity.x = thread_rng().gen_range(
                    random_initial_motion.linear.x.0,
                    random_initial_motion.linear.x.1,
                );
                motion2d_component.velocity.y = thread_rng().gen_range(
                    random_initial_motion.linear.y.0,
                    random_initial_motion.linear.y.1,
                );
                motion2d_component.angular_velocity = thread_rng().gen_range(
                    random_initial_motion.angular.0,
                    random_initial_motion.angular.1,
                );
            }
            lazy_update.insert(effect_entity, motion2d_component);
        } else if effect_data.random_initial_motion.is_some() {
            panic!("Effects with RandomInitialMotion must also have a motion component.")
        }

        if let Some(fade_component) = effect_data.fade_component.clone() {
            let tint_component = Tint(Srgba::new(
                if let Some(red_change) = fade_component.red_change.clone() {
                    red_change.value
                } else {
                    1.0
                },
                if let Some(green_change) = fade_component.green_change.clone() {
                    green_change.value
                } else {
                    1.0
                },
                if let Some(blue_change) = fade_component.blue_change.clone() {
                    blue_change.value
                } else {
                    1.0
                },
                if let Some(alpha_change) = fade_component.alpha_change.clone() {
                    alpha_change.value
                } else {
                    1.0
                },
            ));

            lazy_update.insert(effect_entity, tint_component);
            lazy_update.insert(effect_entity, fade_component);
        }
    }
}

pub fn spawn_spawnable(
    spawnable_type: &SpawnableType,
    spawn_transform: Transform,
    consumables_resource: &ReadExpect<ConsumablesResource>,
    enemies_resource: &ReadExpect<EnemiesResource>,
    items_resource: &ReadExpect<ItemsResource>,
    effects_resource: &ReadExpect<EffectsResource>,
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    match spawnable_type {
        SpawnableType::Consumable(consumable_type) => {
            spawn_consumable(
                consumable_type,
                spawn_transform,
                consumables_resource,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }

        SpawnableType::Enemy(enemy_type) => {
            spawn_enemy(
                enemy_type,
                spawn_transform,
                enemies_resource,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }

        SpawnableType::Item(item_type) => {
            spawn_item(
                item_type,
                spawn_transform,
                items_resource,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }

        SpawnableType::Effect(effect_type) => {
            spawn_effect(
                effect_type,
                spawn_transform,
                effects_resource,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }
    }
}
