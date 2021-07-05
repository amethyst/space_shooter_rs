use crate::{
    components::{AttractorCategory, PlayerComponent},
    constants::{ARENA_HEIGHT, ARENA_MIN_Y},
    entities::{AllyType, EnemyType, MobType, NeutralType},
    events::AttractionEvent,
    motion::components::{Hitbox2DComponent, Motion2DComponent},
    spawnable::components::{BlastComponent, ConsumableComponent, ItemComponent, MobComponent},
    tools::distance,
    weapons::BlastType,
};
use amethyst::{
    core::{math::{Vector2, Vector3}, timing::Time, transform::Transform},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    ecs::*,
    shrev::{EventChannel, ReaderId},
};

/// Handles core motion
pub struct Motion2DSystem;

impl<'s> System<'s> for Motion2DSystem {
    /// Data used by the system
    type SystemData = (
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    /// System game logic
    fn run(&mut self, (mut motion_2ds, mut transforms, time): Self::SystemData) {
        for (motion_2d, transform) in (&mut motion_2ds, &mut transforms).join() {
            let dt = time.delta_seconds();

            // update translation based on velocity and delta time
            transform.set_translation_xyz(
                transform.translation().x + motion_2d.velocity.x * dt,
                transform.translation().y + motion_2d.velocity.y * dt,
                transform.translation().z,
            );

            // update angle based on angular velocity and time
            transform.append_rotation_z_axis(motion_2d.angular_velocity * dt);

            // limit angular speed to max angular speed
            if motion_2d.angular_velocity.abs() > motion_2d.angular_speed {
                if motion_2d.angular_velocity > 0.0 {
                    motion_2d.angular_velocity = motion_2d.angular_speed;
                } else {
                    motion_2d.angular_velocity = -motion_2d.angular_speed;
                }
            }

            // limit speed in the x direction to the max speed
            if motion_2d.velocity.x.abs() > motion_2d.max_speed.x {
                if motion_2d.velocity.x > 0.0 {
                    motion_2d.velocity.x = motion_2d.max_speed.x;
                } else {
                    motion_2d.velocity.x = -motion_2d.max_speed.x;
                }
            }

            // limit speed in the y direction to the max speed
            if motion_2d.velocity.y.abs() > motion_2d.max_speed.y {
                if motion_2d.velocity.y > 0.0 {
                    motion_2d.velocity.y = motion_2d.max_speed.y;
                } else {
                    motion_2d.velocity.y = -motion_2d.max_speed.y;
                }
            }
        }
    }
}

/// Handles motion of blasts
#[derive(Default)]
pub struct BlastMotion2DSystem {
    /// Reads from the attraction event channel
    event_reader: Option<ReaderId<AttractionEvent>>,
}

impl<'s> System<'s> for BlastMotion2DSystem {
    /// Data used by the system
    type SystemData = (
        WriteStorage<'s, Motion2DComponent>,
        ReadStorage<'s, BlastComponent>,
        WriteStorage<'s, Transform>,
        Read<'s, EventChannel<AttractionEvent>>,
    );

    /// Sets up event readers
    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<AttractionEvent>>()
                .register_reader(),
        );
    }

    /// System game logic
    fn run(
        &mut self,
        (mut motion_2ds, blasts, mut transforms, attraction_channel): Self::SystemData,
    ) {
        let mut attracted = false;
        for event in attraction_channel.read(self.event_reader.as_mut().unwrap()) {
            for (blast, motion_2d, transform) in (&blasts, &mut motion_2ds, &mut transforms).join()
            {
                if let Some(attract_data) = event.affected_spawnables.get(&AttractorCategory::Blast)
                {
                    // skip if the target's attract data isn't active.
                    if !attract_data.is_active {
                        continue;
                    }
                    // check if spawnable is in area of influence
                    // TODO simplify distance() to take just the transform and event data
                    // components

                    if distance(
                        *transform.translation(),
                        Vector3::new(event.target_position.x, event.target_position.y, 0.0),
                        true,
                    ) < attract_data.radius
                    {
                        // accelerate towards attractor
                        attracted = true;
                        if let BlastType::Enemy = blast.blast_type {
                                motion_2d.target_position = Some(event.target_position);
                                motion_2d.move_towards_target(
                                    Vector2::new(
                                        transform.translation().x,
                                        transform.translation().y,
                                    ),
                                    Vector2::new(
                                        attract_data.acceleration,
                                        attract_data.acceleration,
                                    ),
                                    attract_data.should_repel,
                                );
                        }
                        break;
                    }
                }
            }
        }

        if !attracted {
            for (_blast, motion_2d) in (&blasts, &mut motion_2ds).join() {
                motion_2d.target_position = None;
                motion_2d.move_down();
                motion_2d.brake_horizontal();
            }
        }
    }
}

/// Handles motion of consumables
#[derive(Default)]
pub struct ConsumableMotion2DSystem {
    /// Reads from the attraction event channel
    event_reader: Option<ReaderId<AttractionEvent>>,
}

impl<'s> System<'s> for ConsumableMotion2DSystem {
    /// Data used by the system
    type SystemData = (
        WriteStorage<'s, Motion2DComponent>,
        ReadStorage<'s, ConsumableComponent>,
        WriteStorage<'s, Transform>,
        Read<'s, EventChannel<AttractionEvent>>,
    );

    /// Sets up event readers
    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<AttractionEvent>>()
                .register_reader(),
        );
    }

    /// System game logic
    fn run(
        &mut self,
        (mut motion_2ds, consumables, mut transforms, attraction_channel): Self::SystemData,
    ) {
        let mut attracted = false;
        for event in attraction_channel.read(self.event_reader.as_mut().unwrap()) {
            for (_consumable, motion_2d, transform) in
                (&consumables, &mut motion_2ds, &mut transforms).join()
            {
                if let Some(attract_data) = event
                    .affected_spawnables
                    .get(&AttractorCategory::Consumable)
                {
                    // check if spawnable is in area of influence
                    if distance(
                        *transform.translation(),
                        Vector3::new(event.target_position.x, event.target_position.y, 0.0),
                        true
                    ) < attract_data.radius
                    {
                        // accelerate towards attractor
                        attracted = true;
                        motion_2d.target_position = Some(event.target_position);
                        motion_2d.move_towards_target(
                            Vector2::new(transform.translation().x, transform.translation().y),
                            Vector2::new(attract_data.acceleration, attract_data.acceleration),
                            attract_data.should_repel,
                        );
                        break;
                    }
                }
            }
        }

        if !attracted {
            for (_consumable, motion_2d) in (&consumables, &mut motion_2ds).join() {
                motion_2d.target_position = None;
                motion_2d.move_down();
                motion_2d.brake_horizontal();
            }
        }
    }
}

/// Handles motion of items
#[derive(Default)]
pub struct ItemMotion2DSystem {
    /// Reads from the attraction event channel
    event_reader: Option<ReaderId<AttractionEvent>>,
}

impl<'s> System<'s> for ItemMotion2DSystem {
    /// Data used by the system
    type SystemData = (
        WriteStorage<'s, Motion2DComponent>,
        ReadStorage<'s, ItemComponent>,
        WriteStorage<'s, Transform>,
        Read<'s, EventChannel<AttractionEvent>>,
    );

    /// Sets up event readers
    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<AttractionEvent>>()
                .register_reader(),
        );
    }

    /// System game logic
    fn run(
        &mut self,
        (mut motion_2ds, items, mut transforms, attraction_channel): Self::SystemData,
    ) {
        let mut attracted = false;
        for event in attraction_channel.read(self.event_reader.as_mut().unwrap()) {
            for (_item, motion_2d, transform) in (&items, &mut motion_2ds, &mut transforms).join() {
                if let Some(attract_data) = event.affected_spawnables.get(&AttractorCategory::Item)
                {
                    // check if spawnable is in area of influence
                    if distance(
                        *transform.translation(),
                        Vector3::new(event.target_position.x, event.target_position.y, 0.0),
                        true
                    ) < attract_data.radius
                    {
                        // accelerate towards attractor
                        attracted = true;
                        motion_2d.target_position = Some(event.target_position);
                        motion_2d.move_towards_target(
                            Vector2::new(transform.translation().x, transform.translation().y),
                            Vector2::new(attract_data.acceleration, attract_data.acceleration),
                            attract_data.should_repel,
                        );
                        break;
                    }
                }
            }
        }

        if !attracted {
            for (_item, motion_2d) in (&items, &mut motion_2ds).join() {
                motion_2d.target_position = None;
                motion_2d.move_down();
                motion_2d.brake_horizontal();
            }
        }
    }
}

/// Handles motion of mobs
pub struct MobMotion2DSystem;

impl<'s> System<'s> for MobMotion2DSystem {
    /// Data used by the system
    type SystemData = (
        ReadStorage<'s, MobComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Hitbox2DComponent>,
    );

    /// System game logic
    fn run(&mut self, (mobs, mut motion_2ds, mut transforms, mut hitbox_2ds): Self::SystemData) {
        for (mob, motion_2d, hitbox_2d, transform) in
            (&mobs, &mut motion_2ds, &mut hitbox_2ds, &mut transforms).join()
        {
            move_mob(mob, transform, motion_2d, hitbox_2d);
        }
    }
}

/// Handles mob targeting
pub struct MobTargetSystem;

impl<'s> System<'s> for MobTargetSystem {
    /// Data used by the system
    type SystemData = (
        WriteStorage<'s, MobComponent>,
        WriteStorage<'s, Motion2DComponent>,
        ReadStorage<'s, PlayerComponent>,
        ReadStorage<'s, Transform>,
    );

    /// System game logic
    fn run(&mut self, (mut mobs, mut motion_2ds, players, transforms): Self::SystemData) {
        for (mob, transform, motion_2d) in (&mut mobs, &transforms, &mut motion_2ds).join() {
            if let MobType::Enemy(EnemyType::Missile) = mob.mob_type {
                let mut closest_player_position: Option<Vector2<f32>> = None;

                for (_player, player_transform) in (&players, &transforms).join() {
                    if let Some(closest_position) = closest_player_position {
                        if distance(
                            *player_transform.translation(),
                            *transform.translation(),
                            true,
                        ) < distance(
                            Vector3::new(closest_position.x, closest_position.y, 0.0),
                            *transform.translation(),
                            true,
                        ) {
                            closest_player_position = Some(Vector2::new(
                                player_transform.translation().x,
                                player_transform.translation().y,
                            ));
                        }
                    } else {
                        closest_player_position = Some(Vector2::new(
                            player_transform.translation().x,
                            player_transform.translation().y,
                        ));
                    }
                }

                motion_2d.target_position = closest_player_position;
            }
        }
    }
}

/// Handle movement of mobs by MobType
fn move_mob(
    mob: &MobComponent,
    transform: &mut Transform,
    motion_2d: &mut Motion2DComponent,
    hitbox_2d: &mut Hitbox2DComponent,
) {
    match mob.mob_type {
        MobType::Neutral(NeutralType::MoneyAsteroid) => {
            motion_2d.move_down();
            motion_2d.brake_horizontal();
        }
        MobType::Enemy(EnemyType::Pawn) => {
            motion_2d.move_down();
            motion_2d.brake_horizontal();
        }
        MobType::Enemy(EnemyType::Drone) => {
            motion_2d.move_down();
            motion_2d.brake_horizontal();
        }
        MobType::Ally(AllyType::Hauler) => {
            motion_2d.move_down();
            motion_2d.brake_horizontal();
        }
        MobType::Enemy(EnemyType::StraferRight) => {
            motion_2d.move_down();

            // accelerate to speed stat in the x direction
            if motion_2d.velocity.x.abs() < motion_2d.speed.x {
                if motion_2d.velocity.x >= 0.0 {
                    motion_2d.velocity.x += motion_2d.acceleration.x;
                } else {
                    motion_2d.velocity.x -= motion_2d.acceleration.x;
                }
            } else if motion_2d.velocity.x.abs() >= motion_2d.speed.x {
                if motion_2d.velocity.x > 0.0 {
                    motion_2d.velocity.x -= motion_2d.deceleration.x;
                } else {
                    motion_2d.velocity.x += motion_2d.deceleration.x;
                }
            }
        }
        MobType::Enemy(EnemyType::StraferLeft) => {
            motion_2d.move_down();

            // accelerate to speed stat in the x direction
            if motion_2d.velocity.x.abs() < motion_2d.speed.x {
                if motion_2d.velocity.x <= 0.0 {
                    motion_2d.velocity.x -= motion_2d.acceleration.x;
                } else {
                    motion_2d.velocity.x += motion_2d.acceleration.x;
                }
            } else if motion_2d.velocity.x.abs() >= motion_2d.speed.x {
                if motion_2d.velocity.x > 0.0 {
                    motion_2d.velocity.x -= motion_2d.deceleration.x;
                } else {
                    motion_2d.velocity.x += motion_2d.deceleration.x;
                }
            }
        }
        MobType::Enemy(EnemyType::MissileLauncher) => {
            motion_2d.move_down();
            motion_2d.brake_horizontal();
        }
        MobType::Enemy(EnemyType::Missile) => {
            if motion_2d.target_position.is_some() {
                //turn towards target
                motion_2d.turn_towards_target(
                    Vector2::new(transform.translation().x, transform.translation().y),
                    transform.euler_angles().2.to_degrees() + 180.0,
                );
                hitbox_2d.set_offset_rotation(transform.euler_angles().2);

                motion_2d.move_forward(transform.euler_angles().2);
            } else {
                motion_2d.move_down();
                motion_2d.brake_horizontal();
            }
        }
        MobType::Enemy(EnemyType::RepeaterBody) => {
            // move down to position and then accelerate backwards
            if transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 30.0 {
                motion_2d.move_down();
            } else {
                motion_2d.move_up();
            }
        }
        MobType::Enemy(EnemyType::RepeaterHead) => {
            // move down to position and then accelerate backwards
            if transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 67.0 {
                motion_2d.move_down();
            } else {
                motion_2d.move_up();
            }
        }
        MobType::Enemy(EnemyType::RepeaterRightShoulder)
        | MobType::Enemy(EnemyType::RepeaterLeftShoulder) => {
            // move down to position and then accelerate backwards
            if transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 32.0 {
                motion_2d.move_down();
            } else {
                motion_2d.move_up();
            }

            // rotate back and forth
            if transform.euler_angles().2 > 0.1 {
                motion_2d.angular_velocity = 0.05;
            } else if transform.euler_angles().2 < -0.1 {
                motion_2d.angular_velocity = -0.05;
            }
        }
        MobType::Enemy(EnemyType::RepeaterRightArm)
        | MobType::Enemy(EnemyType::RepeaterLeftArm) => {
            // move down to position and then accelerate backwards
            if transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 32.0 {
                motion_2d.move_down();
            } else {
                motion_2d.move_up();
            }
        }
    }
}
