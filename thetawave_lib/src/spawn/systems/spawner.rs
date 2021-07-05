use crate::{
    entities::spawn_repeater,
    resources::{BossType, PhaseManagerResource, PhaseType, SpriteSheetsResource},
    spawn::{components::AutoSpawnerComponent, resources::SpawnerResource},
    spawnable::resources::{ConsumablesResource, EffectsResource, ItemsResource, MobsResource},
};
use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::{
        Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, Write, WriteExpect,
        WriteStorage,
    },
};

/// Handles spawning of entities given the type of the current phase
pub struct SpawnerSystem;

impl<'s> System<'s> for SpawnerSystem {
    /// System game logic
    type SystemData = (
        Entities<'s>,
        Read<'s, Time>,
        WriteExpect<'s, SpawnerResource>,
        ReadExpect<'s, SpriteSheetsResource>,
        Write<'s, PhaseManagerResource>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, ConsumablesResource>,
        ReadExpect<'s, ItemsResource>,
        ReadExpect<'s, EffectsResource>,
        ReadExpect<'s, MobsResource>,
    );

    /// System game logic
    fn run(
        &mut self,
        (
            entities,
            time,
            mut spawner_resource,
            spritesheets_resource,
            mut phase_manager,
            lazy_update,
            consumables_resource,
            items_resource,
            effects_resource,
            mobs_resource,
        ): Self::SystemData,
    ) {
        match phase_manager.get_current_phase_type() {
            Some(PhaseType::InvasionRandom(random_pool_type)) => spawner_resource
                .spawn_random_spawnable_when_ready(
                    random_pool_type,
                    time.delta_seconds(),
                    &consumables_resource,
                    &mobs_resource,
                    &items_resource,
                    &effects_resource,
                    &spritesheets_resource,
                    &entities,
                    &lazy_update,
                ),

            Some(PhaseType::InvasionFormation(formation_pool_type)) => spawner_resource
                .spawn_random_formation_when_ready(
                    formation_pool_type,
                    time.delta_seconds(),
                    &consumables_resource,
                    &mobs_resource,
                    &items_resource,
                    &effects_resource,
                    &spritesheets_resource,
                    &entities,
                    &lazy_update,
                ),

            Some(PhaseType::Boss) => {
                match phase_manager.phase_map[phase_manager.phase_idx].boss_type {
                    BossType::Repeater => {
                        // spawn repeater boss
                        if !phase_manager.phase_map[phase_manager.phase_idx].boss_spawned {
                            spawn_repeater(
                                &spritesheets_resource,
                                &mobs_resource,
                                &entities,
                                &lazy_update,
                            );
                            let phase_idx = phase_manager.phase_idx;
                            phase_manager.phase_map[phase_idx].boss_spawned = true;
                        }
                    }

                    BossType::None => {}
                }
            }

            Some(PhaseType::Rest) => {}

            _ => {}
        }
    }
}

/// Handles automatic spawning of entities using auto-spawner components
pub struct AutoSpawnerSystem;

impl<'s> System<'s> for AutoSpawnerSystem {
    /// Data used by the system
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, AutoSpawnerComponent>,
        Read<'s, Time>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, MobsResource>,
        ReadExpect<'s, ConsumablesResource>,
        ReadExpect<'s, ItemsResource>,
        ReadExpect<'s, EffectsResource>,
        ReadExpect<'s, SpriteSheetsResource>,
        Entities<'s>,
    );

    /// System game logic
    fn run(
        &mut self,
        (
            transforms,
            mut auto_child_entity_spawners,
            time,
            lazy_update,
            mobs_resource,
            consumables_resource,
            items_resource,
            effects_resource,
            sprite_sheets_resource,
            entities,
        ): Self::SystemData,
    ) {
        for (transform, auto_child_entity_spawner) in
            (&transforms, &mut auto_child_entity_spawners).join()
        {
            auto_child_entity_spawner.spawn_when_ready(
                time.delta_seconds(),
                transform.clone(),
                &sprite_sheets_resource,
                &mobs_resource,
                &consumables_resource,
                &items_resource,
                &effects_resource,
                &entities,
                &lazy_update,
            );
        }
    }
}
