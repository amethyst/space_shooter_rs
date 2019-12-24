use crate::{
    components::{EnemySpawnerTag, Spawner},
    constants::{
        ARENA_MAX_Y, ARENA_MIN_X, ARENA_WIDTH, ENEMY_DRONE_RATIO, ENEMY_HAULER_RATIO,
        ENEMY_PAWN_RATIO, ENEMY_SPAWN_INTERVAL, ENEMY_STRAFER_RATIO, SPAWNER_Y_OFFSET,
    },
};
use amethyst::{
    core::transform::Transform,
    ecs::{World, WorldExt},
    prelude::Builder,
};

pub fn initialise_enemy_spawner(world: &mut World) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(
        ARENA_MIN_X + (ARENA_WIDTH / 2.0),
        ARENA_MAX_Y + SPAWNER_Y_OFFSET,
        0.0,
    );
    world
        .create_entity()
        .with(Spawner::new(
            vec![
                ("pawn".to_string(), ENEMY_PAWN_RATIO),
                ("drone".to_string(), ENEMY_DRONE_RATIO),
                ("hauler".to_string(), ENEMY_HAULER_RATIO),
                ("strafer".to_string(), ENEMY_STRAFER_RATIO),
            ],
            ENEMY_SPAWN_INTERVAL,
        ))
        .with(EnemySpawnerTag)
        .with(local_transform)
        .build();
}
