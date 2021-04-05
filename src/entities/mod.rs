use serde::{Deserialize, Serialize};

pub mod backgrounds;
pub mod barriers;
pub mod blast;
pub mod boss;
pub mod consumable;
pub mod defense;
pub mod enemy_spawner;
pub mod planet;
pub mod side_panels;
pub mod spaceship;
pub mod spawn;
pub mod status_bar;
pub mod status_unit;
pub mod store_icons;

pub use self::{
    backgrounds::initialize_background,
    barriers::initialize_arena_barriers,
    blast::spawn_blasts,
    boss::spawn_repeater,
    consumable::spawn_random_consumable,
    defense::initialize_defense,
    enemy_spawner::initialize_enemy_spawner,
    planet::initialize_planet,
    side_panels::initialize_side_panels,
    spaceship::initialize_spaceship,
    spawn::{spawn_consumable, spawn_effect, spawn_enemy, spawn_item, spawn_spawnable},
    status_bar::initialize_status_bars,
    status_unit::spawn_status_unit,
    store_icons::initialize_store_icons,
};

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum SpawnableType {
    Enemy(EnemyType),
    Consumable(ConsumableType),
    Item(ItemType),
    Effect(EffectType),
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum EnemyType {
    Pawn,
    Drone,
    StraferRight,
    StraferLeft,
    Hauler, //ally
    MissileLauncher,
    Missile,
    RepeaterBody,
    RepeaterHead,
    RepeaterLeftShoulder,
    RepeaterRightShoulder,
    RepeaterLeftArm,
    RepeaterRightArm,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum ConsumableType {
    DefenseWrench,
    Money1,
    Money5,
    HealthWrench,
    Armor,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum ItemType {
    SteelBarrel,
    PlasmaBlasts,
    HazardousReactor,
    WarpThruster,
    Tentaclover,
    DefenseSatellite,
    DoubleBarrel,
    YithianPlague,
    Spice,
    EnhancedPlating,
    StructureReinforcement,
    BlasterSizeEnhancer,
    FrequencyAugmentor,
}
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum EffectType {
    AllyBlastExplosion,
    EnemyBlastExplosion,
    PoisonBlastExplosion,
    CriticalBlastExplosion,
    EnemyExplosion,
    Star, //TODO: implement background stars
    Giblets(EnemyType),
}
