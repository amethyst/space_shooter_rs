use crate::{
    constants::{ARENA_MAX_X, ARENA_MAX_Y, ARENA_MIN_X, ARENA_MIN_Y},
    events::MobReachedBottomEvent,
    spawn::components::{DespawnAtBorderComponent, DespawnTimeLimitComponent},
    spawnable::components::MobComponent,
};
use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    ecs::*,
    shrev::EventChannel,
};

/// Handles despawning of entities at arena borders
pub struct DespawnAtBorderSystem;

impl<'s> System<'s> for DespawnAtBorderSystem {
    /// Data used by the system
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, DespawnAtBorderComponent>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, MobComponent>,
        Write<'s, EventChannel<MobReachedBottomEvent>>,
    );

    /// System game logic
    fn run(
        &mut self,
        (entities, despawn_borders, transforms, mobs, mut mob_reached_bottom_event_channel): Self::SystemData,
    ) {
        for (entity, despawn_border, transform) in
            (&*entities, &despawn_borders, &transforms).join()
        {
            if let Some(top_border_offset) = despawn_border.top_offset {
                if transform.translation().y > ARENA_MAX_Y + top_border_offset {
                    entities.delete(entity).expect("unable to delete entity");
                }
            }

            if let Some(bottom_border_offset) = despawn_border.bottom_offset {
                if transform.translation().y < ARENA_MIN_Y + bottom_border_offset {
                    if let Some(mob) = mobs.get(entity) {
                        mob_reached_bottom_event_channel
                            .single_write(MobReachedBottomEvent::new(mob.defense_damage));
                    }
                    entities.delete(entity).expect("unable to delete entity");
                }
            }

            if let Some(left_border_offset) = despawn_border.left_offset {
                if transform.translation().x < ARENA_MIN_X + left_border_offset {
                    entities.delete(entity).expect("unable to delete entity");
                }
            }

            if let Some(right_border_offset) = despawn_border.right_offset {
                if transform.translation().x > ARENA_MAX_X + right_border_offset {
                    entities.delete(entity).expect("unable to delete entity");
                }
            }
        }
    }
}

/// Handles despawning of entities after a time limit
pub struct DespawnTimeLimitSystem;

impl<'s> System<'s> for DespawnTimeLimitSystem {
    /// Data used by the system
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, DespawnTimeLimitComponent>,
        Read<'s, Time>,
    );

    /// System game logic
    fn run(&mut self, (entities, mut timelimits, time): Self::SystemData) {
        for (timed_entity, time_component) in (&*entities, &mut timelimits).join() {
            if time_component.duration > 0.0 {
                time_component.duration -= time.delta_seconds();
            } else {
                entities
                    .delete(timed_entity)
                    .expect("unable to delete entity");
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use amethyst::{
        ecs::prelude::{Builder, Entity, WorldExt},
        Error,
    };
    use amethyst_test::prelude::*;

    use crate::spawn::components::DespawnTimeLimitComponent;

    #[test]
    fn test_timelimit_system() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(DespawnTimeLimitSystem, "timelimit_system", &[])
            .with_effect(|world| {
                let entity = world
                    .create_entity()
                    .with(DespawnTimeLimitComponent { duration: -1.0 })
                    .build();
                world.insert(EffectReturn(entity));
            })
            .with_assertion(|world| {
                let entity = world.read_resource::<EffectReturn<Entity>>().0;
                world.maintain();
                assert!(!world.is_alive(entity));
            })
            .run()
    }
}
