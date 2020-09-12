use crate::{
    components::{
        AutoBlasterComponent, BlastComponent, BlastType, Hitbox2DComponent, Motion2DComponent,
    },
    constants::{
        BLAST_HITBOX_DIAMETER, BLAST_Z, CRIT_BLAST_SPRITE_INDEX, ENEMY_BLAST_SPRITE_INDEX,
        PLAYER_BLAST_SPRITE_INDEX, POISON_BLAST_SPRITE_INDEX,
    },
    entities::spawn_blasts,
    resources::SpriteResource,
};

use amethyst::{
    core::{
        math::{Vector2, Vector3},
        timing::Time,
        transform::Transform,
    },
    ecs::prelude::{
        Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage,
    },
    renderer::SpriteRender,
};

use rand::{thread_rng, Rng};

pub struct AutoBlasterSystem;

impl<'s> System<'s> for AutoBlasterSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, Time>,
        ReadExpect<'s, LazyUpdate>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, AutoBlasterComponent>,
        ReadStorage<'s, Motion2DComponent>,
        ReadExpect<'s, SpriteResource>,
    );

    fn run(
        &mut self,
        (entities, time, lazy_update, transforms, mut autoblasters, motion2ds, sprite_resource): Self::SystemData,
    ) {
        for (transform, autoblaster, motion2d) in
            (&transforms, &mut autoblasters, &motion2ds).join()
        {
            fire_when_ready(
                autoblaster,
                motion2d,
                transform,
                time.delta_seconds(),
                &entities,
                &sprite_resource,
                &lazy_update,
            );
        }
    }
}

fn fire_when_ready(
    autoblaster: &mut AutoBlasterComponent,
    source_motion2d: &Motion2DComponent,
    source_transform: &Transform,
    delta_time: f32,
    entities: &Entities,
    sprite_resource: &ReadExpect<SpriteResource>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    if autoblaster.fire_timer > 0.0 {
        autoblaster.fire_timer -= delta_time;
    } else {
        autoblaster.fire_timer = autoblaster.fire_period;
        let fire_position = Vector3::new(
            source_transform.translation().x + autoblaster.offset.x,
            source_transform.translation().y + autoblaster.offset.y,
            BLAST_Z,
        );

        let mut blast_type = if !autoblaster.allied {
            BlastType::Enemy // TODO: remove BlastType or "allied" bool. They store redundant info.
        } else {
            BlastType::Ally
        };

        let blast_damage = autoblaster.damage
            * if thread_rng().gen::<f32>() < autoblaster.crit_chance {
                blast_type = BlastType::AllyCritical;
                2.0
            } else {
                1.0
            };

        let blast_poison_damage = if thread_rng().gen::<f32>() < autoblaster.poison_chance {
            blast_type = BlastType::AllyPoison;
            autoblaster.poison_damage
        } else {
            0.0
        };

        let blast_sprite_render = SpriteRender {
            sprite_sheet: sprite_resource.blasts_sprite_sheet.clone(),
            sprite_number: match blast_type {
                BlastType::Ally => PLAYER_BLAST_SPRITE_INDEX,
                BlastType::Enemy => ENEMY_BLAST_SPRITE_INDEX,
                BlastType::AllyCritical => CRIT_BLAST_SPRITE_INDEX,
                BlastType::AllyPoison => POISON_BLAST_SPRITE_INDEX,
            },
        };

        let blast_hitbox = Hitbox2DComponent {
            width: BLAST_HITBOX_DIAMETER * autoblaster.size_multiplier,
            height: BLAST_HITBOX_DIAMETER * autoblaster.size_multiplier,
            offset_x: 0.0,
            offset_y: 0.0,
            offset_rotation: 0.0,
        };

        let blast_motion2d = Motion2DComponent {
            velocity: Vector2::new(
                (source_motion2d.velocity.x * autoblaster.velocity_multiplier)
                    + autoblaster.shot_velocity.x,
                (source_motion2d.velocity.y * autoblaster.velocity_multiplier)
                    + autoblaster.shot_velocity.y,
            ),
            acceleration: Vector2::new(0.0, 0.0),
            deceleration: Vector2::new(0.0, 0.0),
            max_speed: Vector2::new(1000.0, 1000.0),
            knockback_max_speed: Vector2::new(1000.0, 1000.0),
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            angular_deceleration: 0.0,
        };

        let blast_component = BlastComponent {
            damage: blast_damage,
            poison_damage: blast_poison_damage,
            blast_type,
        };

        let blast_spawn_x = fire_position.x
            - if autoblaster.count % 2 == 0 {
                (autoblaster.spacing * (autoblaster.count - 1) as f32) / 2.0
            } else {
                autoblaster.spacing * (autoblaster.count / 2) as f32
            };

        let mut blast_transform = Transform::default();
        blast_transform.set_translation(Vector3::new(
            blast_spawn_x,
            fire_position.y,
            fire_position.z,
        ));
        blast_transform.set_scale(Vector3::new(
            autoblaster.size_multiplier,
            autoblaster.size_multiplier,
            1.0,
        ));

        spawn_blasts(
            autoblaster.count,
            autoblaster.spacing,
            blast_sprite_render,
            blast_component,
            blast_hitbox,
            blast_motion2d,
            blast_transform,
            entities,
            lazy_update,
        );
    }
}
