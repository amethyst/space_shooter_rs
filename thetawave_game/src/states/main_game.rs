use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::{Dispatcher, DispatcherBuilder, Entity},
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::debug_drawing::{DebugLines, DebugLinesParams},
    renderer::formats::texture::ImageFormat,
    renderer::{Camera, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
};
use std::{collections::HashMap, f32::consts::FRAC_PI_3};
use thetawave_lib::{
    audio::initialize_audio,
    constants::{
        ARENA_HEIGHT, ARENA_MAX_X, ARENA_MIN_X, ARENA_MIN_Y, ARENA_WIDTH, CAMERA_X, CAMERA_Y,
        CAMERA_Z,
    },
    entities::{initialize_arena_barriers, initialize_spaceship},
    motion::systems::{
        BlastMotion2DSystem, CollisionDetectionSystem, CollisionHandlerSystem,
        ConsumableMotion2DSystem, ItemMotion2DSystem, MobArenaBorderCollisionSystem,
        MobBlastCollisionSystem, MobMobCollisionSystem, MobMotion2DSystem,
        MobPlayerCollisionSystem, MobTargetSystem, Motion2DSystem,
        PlayerArenaBorderCollisionSystem, PlayerBlastCollisionSystem,
        PlayerConsumableCollisionSystem, PlayerItemCollisionSystem, PlayerMobCollisionSystem,
        PlayerMotion2DSystem,
    },
    resources::{DebugLinesConfig, SpriteSheetsConfig, SpriteSheetsResource},
    spawn::systems::{
        AutoSpawnerSystem, DespawnAtBorderSystem, DespawnTimeLimitSystem, SpawnerSystem,
    },
    spawnable::{
        systems::ModifiersSystem,
        systems::{MobBehaviorSystem, MobDestroyedSystem},
    },
    systems,
    visual::{
        entities::{
            initialize_background, initialize_planet, initialize_side_panels,
            initialize_status_bars, initialize_store_icons,
        },
        systems::{
            AnimationSystem, FadeSystem, PlanetsSystem, StatTrackerSystem, StatusBarSystem,
            TrackedStats,
        },
    },
    weapons::systems::{AutoFireSystem, ManualBlasterSystem},
};

use crate::states::PausedState;

pub struct MainGameState {
    is_paused: bool,
    pause_display: Option<Entity>,
    dispatcher: Dispatcher<'static, 'static>,
}

impl Default for MainGameState {
    fn default() -> Self {
        MainGameState {
            is_paused: false,
            pause_display: None,
            dispatcher: DispatcherBuilder::new()
                .with(AnimationSystem, "animation_system", &[])
                .with(PlanetsSystem, "planets_system", &[])
                .with(systems::PhaseManagerSystem, "phase_manager_system", &[])
                .with(MobBehaviorSystem, "mob_behavior_system", &[])
                .with(systems::BossSystem, "boss_system", &[])
                .with(SpawnerSystem, "spawner_system", &[])
                .with(DespawnTimeLimitSystem, "timelimit_system", &[])
                .with(Motion2DSystem, "motion_2d_system", &[])
                .with(MobTargetSystem, "mob_target_system", &[])
                .with(AutoSpawnerSystem, "auto_spawner_system", &[])
                .with(systems::AttractorSystem, "attractor_system", &[])
                .with(
                    ItemMotion2DSystem::default(),
                    "item_motion_2d_system",
                    &["attractor_system"],
                )
                .with(
                    ConsumableMotion2DSystem::default(),
                    "consumable_motion_2d_system",
                    &["attractor_system"],
                )
                .with(
                    BlastMotion2DSystem::default(),
                    "blast_motion_2d_system",
                    &["attractor_system"],
                )
                .with(
                    systems::BarrelRollAbilitySystem::default(),
                    "barrel_roll_ability_system",
                    &[],
                )
                .with(
                    MobMotion2DSystem,
                    "mob_motion_2d_system",
                    &["mob_target_system"],
                )
                .with(DespawnAtBorderSystem, "despawn_at_border_system", &[])
                .with(PlayerMotion2DSystem, "spaceship_movement_system", &[])
                .with(StatusBarSystem, "status_bar_system", &[])
                .with(CollisionDetectionSystem, "collision_detection_system", &[])
                .with(
                    CollisionHandlerSystem::default(),
                    "collision_handler_system",
                    &["collision_detection_system"],
                )
                .with(
                    MobArenaBorderCollisionSystem::default(),
                    "mob_arena_border_collsion_system",
                    &["collision_handler_system"],
                )
                .with(
                    PlayerArenaBorderCollisionSystem::default(),
                    "spaceship_arena_border_collision_system",
                    &["collision_handler_system"],
                )
                .with(
                    PlayerMobCollisionSystem::default(),
                    "spaceship_mob_collision_system",
                    &["collision_handler_system"],
                )
                .with(
                    PlayerBlastCollisionSystem::default(),
                    "spaceship_blast_collision_system",
                    &["collision_handler_system"],
                )
                .with(
                    PlayerItemCollisionSystem::default(),
                    "spaceship_item_collision_system",
                    &["collision_handler_system"],
                )
                .with(
                    ModifiersSystem::default(),
                    "modifiers_system",
                    &["spaceship_item_collision_system"],
                )
                .with(
                    PlayerConsumableCollisionSystem::default(),
                    "spaceship_consumable_collision_system",
                    &["collision_handler_system"],
                )
                .with(
                    MobPlayerCollisionSystem::default(),
                    "mob_player_collision",
                    &["collision_handler_system"],
                )
                .with(
                    MobMobCollisionSystem::default(),
                    "mob_mob_collision",
                    &["collision_handler_system"],
                )
                .with(
                    MobBlastCollisionSystem::default(),
                    "mob_blast_collision",
                    &["collision_handler_system"],
                )
                .with(
                    systems::DefenseSystem::default(),
                    "defense_system",
                    &["spaceship_item_collision_system"],
                )
                .with(systems::SpaceshipSystem::default(), "spaceship_system", &[])
                .with(systems::StoreSystem, "store_system", &[])
                .with(
                    StatTrackerSystem,
                    "stat_tracker_system",
                    &["store_system", "spaceship_system"],
                )
                .with(AutoFireSystem, "autoblaster_system", &[])
                .with(ManualBlasterSystem, "manualblaster_system", &[])
                .with(
                    MobDestroyedSystem::default(),
                    "mob_destroyed_system",
                    &["mob_behavior_system"],
                )
                .with(
                    systems::PlayAudioSystem::default(),
                    "play_audio_system",
                    &[],
                )
                .with(FadeSystem, "fade_system", &[])
                .build(),
        }
    }
}

impl SimpleState for MainGameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let spritesheets = init_spritesheets(world);

        self.dispatcher.setup(world);

        initialize_audio(world);
        initialise_ui(world);
        initialize_status_bars(world);
        initialize_side_panels(world, spritesheets.spritesheets["side_panels"].clone());
        initialize_planet(
            world,
            "earth_planet.glb",
            ARENA_MIN_X + (ARENA_WIDTH / 2.0),
            -1100.0,
            -1010.0,
            1000.0,
            100.0,
            0.01,
        );
        initialize_planet(
            world,
            "sol_star.glb",
            ARENA_MIN_X + (ARENA_WIDTH / 2.0) - 5000.0,
            (ARENA_HEIGHT / 2.0) + 3000.0,
            -15000.0,
            800.0,
            0.0,
            0.005,
        );
        initialize_background(world, spritesheets.spritesheets["backgrounds"].clone());
        initialize_spaceship(world, spritesheets.spritesheets["characters"].clone());
        initialize_arena_barriers(world);
        initialize_store_icons(world, spritesheets.spritesheets["items"].clone());
        initialise_camera(world);

        world.insert(DebugLines::new());
        let debug_lines_params = {
            let debug_lines_config = world.read_resource::<DebugLinesConfig>();

            DebugLinesParams {
                line_width: debug_lines_config.line_width,
            }
        };
        world.insert(debug_lines_params);
    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.is_paused = true;

        let paused_text_entity = get_paused_text(data.world);
        self.pause_display = Some(paused_text_entity);
    }

    fn on_resume(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        self.is_paused = false;
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        self.dispatcher.dispatch(data.world);

        // Handle paused state here.
        if let Some(pause_text_entity) = self.pause_display {
            data.world
                .delete_entity(pause_text_entity)
                .expect("Failed to remove Paused text.");

            self.pause_display = None;
        }

        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(PausedState));
            }
        }
        Trans::None
    }
}

fn init_spritesheets(world: &mut World) -> SpriteSheetsResource {
    let mut spritesheets = HashMap::new();
    {
        let spritesheets_config = world.read_resource::<SpriteSheetsConfig>();
        for (spritesheet_name, spritesheet_data) in spritesheets_config.iter() {
            let texture_handle = {
                let loader = world.read_resource::<Loader>();
                let texture_storage = world.read_resource::<AssetStorage<Texture>>();
                loader.load(
                    format!("texture/{}", spritesheet_data.image),
                    ImageFormat::default(),
                    (),
                    &texture_storage,
                )
            };

            let loader = world.read_resource::<Loader>();
            let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
            let spritesheet_handle = loader.load(
                format!("texture/{}", spritesheet_data.data),
                SpriteSheetFormat(texture_handle),
                (),
                &sprite_sheet_store,
            );

            spritesheets.insert(spritesheet_name.clone(), spritesheet_handle);
        }
    }
    let spritesheets = SpriteSheetsResource { spritesheets };
    world.insert(spritesheets.clone());
    spritesheets
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(CAMERA_X, CAMERA_Y, CAMERA_Z);

    world
        .create_entity()
        .with(Camera::perspective(1.3, FRAC_PI_3, 0.1))
        .with(transform)
        .build();
}
/*
pub struct TrackedStats {
    pub currency: Entity,
    pub shields: Entity,
    pub item_price_1: Entity,
    pub item_price_2: Entity,
    pub item_price_3: Entity,
}
*/

fn initialise_ui(world: &mut World) {
    let item_slots_texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/item_slots.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let item_slots_sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "texture/item_slots.ron",
            SpriteSheetFormat(item_slots_texture_handle),
            (),
            &sprite_sheet_store,
        )
    };

    let item_slots_sprite_render = SpriteRender {
        sprite_sheet: item_slots_sprite_sheet_handle,
        sprite_number: 0,
    };

    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_MAX_X + 10.0 + 2.0, ARENA_MIN_Y + 29.0 + 24.0, 0.9);

    world
        .create_entity()
        .with(item_slots_sprite_render)
        .with(local_transform)
        .build();

    let currency_texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/currency_ui.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let currency_sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "texture/currency_ui.ron",
            SpriteSheetFormat(currency_texture_handle),
            (),
            &sprite_sheet_store,
        )
    };

    let consumables_texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/consumables_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let consumables_sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "texture/consumables_spritesheet.ron",
            SpriteSheetFormat(consumables_texture_handle),
            (),
            &sprite_sheet_store,
        )
    };

    let currency_sprite_render = SpriteRender {
        sprite_sheet: currency_sprite_sheet_handle,
        sprite_number: 0,
    };

    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_MAX_X + 10.0, ARENA_MIN_Y + 12.5, 0.9);

    world
        .create_entity()
        .with(currency_sprite_render)
        .with(local_transform)
        .build();

    let shield_sprite_render = SpriteRender {
        sprite_sheet: consumables_sprite_sheet_handle,
        sprite_number: 4,
    };

    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_MAX_X + 10.0, ARENA_MIN_Y + 158.0, 0.9);

    world
        .create_entity()
        .with(shield_sprite_render)
        .with(local_transform)
        .build();

    let font = world.read_resource::<Loader>().load(
        "font/SpaceMadness.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let currency_count_transform = UiTransform::new(
        "currency_count".to_string(),
        Anchor::BottomRight,
        Anchor::BottomRight,
        -6.0,
        10.0,
        0.9,
        50.0,
        45.0,
    );
    let currency_count = world
        .create_entity()
        .with(currency_count_transform)
        .with(UiText::new(
            font.clone(),
            "x0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            20.0,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    let shields_count_transform = UiTransform::new(
        "shields_count".to_string(),
        Anchor::MiddleRight,
        Anchor::MiddleRight,
        -6.0,
        48.0,
        0.9,
        50.0,
        45.0,
    );

    let shields_count = world
        .create_entity()
        .with(shields_count_transform)
        .with(UiText::new(
            font.clone(),
            "x0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            20.0,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    let item_price_1_transform = UiTransform::new(
        "item_price_0".to_string(),
        Anchor::BottomRight,
        Anchor::BottomRight,
        -6.0,
        130.0,
        0.9,
        50.0,
        45.0,
    );
    let item_price_1 = world
        .create_entity()
        .with(item_price_1_transform)
        .with(UiText::new(
            font.clone(),
            "$0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            15.0,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    let item_price_2_transform = UiTransform::new(
        "item_price_1".to_string(),
        Anchor::BottomRight,
        Anchor::BottomRight,
        -6.0,
        90.0,
        0.9,
        50.0,
        45.0,
    );
    let item_price_2 = world
        .create_entity()
        .with(item_price_2_transform)
        .with(UiText::new(
            font.clone(),
            "$0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            15.0,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    let item_price_3_transform = UiTransform::new(
        "item_price_2".to_string(),
        Anchor::BottomRight,
        Anchor::BottomRight,
        -6.0,
        50.0,
        0.9,
        50.0,
        45.0,
    );
    let item_price_3 = world
        .create_entity()
        .with(item_price_3_transform)
        .with(UiText::new(
            font,
            "$0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            15.0,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    world.insert(TrackedStats {
        currency: currency_count,
        shields: shields_count,
        item_price_1,
        item_price_2,
        item_price_3,
    });
}

fn get_paused_text(world: &mut World) -> Entity {
    let font_handle = world.read_resource::<Loader>().load(
        "font/SpaceMadness.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let ui_transform = UiTransform::new(
        String::from("paused_text"),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        0.0,
        0.0,
        100.0,
        30.0,
    );
    let ui_text = UiText::new(
        font_handle,
        String::from("paused"),
        [1.0, 1.0, 1.0, 1.0],
        25.0,
        LineMode::Single,
        Anchor::Middle,
    );

    world
        .create_entity()
        .with(ui_transform)
        .with(ui_text)
        .build()
}
