use crate::{
    audio::{play_sfx, Sounds},
    components::{choose_random_name, Consumable, Defense, Enemy, EnemyType, Fires, Rigidbody},
    constants::{ARENA_MIN_Y, EXPLOSION_Z},
    entities::{fire_blast, spawn_consumable, spawn_explosion},
    resources::SpriteResource,
};
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::{math::Vector3, timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, LazyUpdate, Read, ReadExpect, System, WriteStorage},
};
use std::{collections::HashMap, ops::Deref};

pub struct EnemySystem;

impl<'s> System<'s> for EnemySystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Defense>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
        ReadExpect<'s, HashMap<String, Consumable>>, // should create alias ConsumablePool
    );

    fn run(
        &mut self,
        (
            entities,
            mut enemys,
            mut defenses,
            mut transforms,
            time,
            sprite_resource,
            lazy_update,
            storage,
            sounds,
            audio_output,
            consumable_pool,
        ): Self::SystemData,
    ) {
        for (enemy_entity, enemy_component, enemy_transform) in
            (&*entities, &mut enemys, &mut transforms).join()
        {
            //constrain in arena
            enemy_component.constrain_to_arena(enemy_transform);

            //accelerate in -y direction
            enemy_component.accelerate(0.0, -1.0);

            //transform the spaceship in x and y by the currrent velocity in x and y
            enemy_component.update_position(enemy_transform, time.delta_seconds());

            enemy_component.health -= enemy_component.poison;

            //conditions for despawning
            if enemy_transform.translation()[1] + enemy_component.height / 2.0 < ARENA_MIN_Y {
                //defense is damage is enemy gets past
                for defense in (&mut defenses).join() {
                    defense.defense -= enemy_component.defense_damage;
                }
                let _result = entities.delete(enemy_entity);
            } else if enemy_component.health < 0.0 {
                //enemy us deleted, explosion is spawned and item dropped
                let death_position = Vector3::new(
                    enemy_transform.translation()[0],
                    enemy_transform.translation()[1],
                    EXPLOSION_Z,
                );

                let _result = entities.delete(enemy_entity);

                spawn_explosion(
                    &entities,
                    &sprite_resource,
                    enemy_component.explosion_sprite_idx,
                    death_position,
                    &lazy_update,
                );

                play_sfx(
                    &sounds.explosion_sfx,
                    &storage,
                    audio_output.as_ref().map(|o| o.deref()),
                );

                let name = choose_random_name(&enemy_component.collectables_probs);
                if !name.is_empty() {
                    spawn_consumable(
                        &entities,
                        &sprite_resource,
                        consumable_pool[name].clone(),
                        death_position,
                        &lazy_update,
                    );
                }
            }

            //behavior for enemies based on its enemy_type attribute
            match enemy_component.enemy_type {
                EnemyType::Pawn => {
                    if let Some(fire_position) = enemy_component.fire_cooldown(
                        enemy_transform,
                        -1.0 * enemy_component.height / 2.0,
                        true,
                        time.delta_seconds(),
                    ) {
                        fire_blast(
                            &entities,
                            &sprite_resource,
                            enemy_component,
                            fire_position,
                            &lazy_update,
                        )
                    }
                }

                EnemyType::Drone => {}

                EnemyType::Hauler => {}

                EnemyType::Strafer => {
                    if let Some(fire_position) = enemy_component.fire_cooldown(
                        enemy_transform,
                        -1.0 * enemy_component.height / 2.0,
                        true,
                        time.delta_seconds(),
                    ) {
                        fire_blast(
                            &entities,
                            &sprite_resource,
                            enemy_component,
                            fire_position,
                            &lazy_update,
                        )
                    }

                    enemy_component.accelerate(1.0, 0.0);
                }
            }
        }
    }
}
