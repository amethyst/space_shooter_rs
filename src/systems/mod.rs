mod blast;
mod collision_detection;
mod collision_handler;
mod consumable;
mod defense;
mod enemy;
mod enemy_hit;
mod enemy_spawn;
mod explosion;
mod gamemaster;
mod item;
mod planets;
mod player_hit;
mod spaceship;
mod spaceship_movement;
mod stat_tracker;
mod status_bar;
mod store;

pub use self::{
    blast::BlastSystem, collision_detection::CollisionDetectionSystem,
    collision_handler::CollisionHandlerSystem, consumable::ConsumableSystem,
    defense::DefenseSystem, enemy::EnemySystem, enemy_hit::EnemyHitSystem,
    enemy_spawn::SpawnerSystem, explosion::ExplosionSystem, gamemaster::GameMasterSystem,
    item::ItemSystem, planets::PlanetsSystem, player_hit::PlayerHitSystem,
    spaceship::SpaceshipSystem, spaceship_movement::SpaceshipMovementSystem,
    stat_tracker::StatTrackerSystem, status_bar::StatusBarSystem, store::StoreSystem,
};

pub fn hitbox_collide(
    mut x1: f32,
    mut y1: f32,
    mut x2: f32,
    mut y2: f32,
    hitbox_width_1: f32,
    hitbox_height_1: f32,
    hitbox_width_2: f32,
    hitbox_height_2: f32,
) -> bool {
    x1 -= hitbox_width_1 / 2.0;
    y1 -= hitbox_height_1 / 2.0;
    x2 -= hitbox_width_2 / 2.0;
    y2 -= hitbox_height_2 / 2.0;

    x1 < (x2 + hitbox_width_2)
        && (x1 + hitbox_width_1) > x2
        && y1 < (y2 + hitbox_height_2)
        && (y1 + hitbox_height_1) > y2
}
