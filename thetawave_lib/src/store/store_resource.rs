use crate::{
    constants::{ARENA_MAX_Y, ITEM_SPAWN_Y_OFFSET},
    player::components::PlayerComponent,
    spawnable::resources::{ConsumablesResource, ItemsResource, SpawnableType},
    tools::Timer,
    visual::resources::SpriteSheetsResource,
};
use amethyst::prelude::Builder;
use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::SpriteRender,
};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::convert::From;

pub type StockProbabilities = Vec<(SpawnableType, f32)>;

#[derive(Serialize, Deserialize)]
pub struct StoreConfig {
    restock_period: f32,
    stock: StockProbabilities,
}

pub struct StoreResource {
    pub inventory: Vec<Option<SpawnableType>>,
    pub timer: Timer,
    stock: StockProbabilities,
    icon_positions: Vec<Vector3<f32>>,
    icon_entities: Vec<Option<Entity>>,
}

impl From<StoreConfig> for StoreResource {
    fn from(config: StoreConfig) -> Self {
        StoreResource::new(config.restock_period, config.stock)
    }
}

impl StoreResource {
    pub fn new(restock_period: f32, stock: StockProbabilities) -> Self {
        StoreResource {
            inventory: vec![None, None, None],
            timer: Timer::new(restock_period),
            stock,
            icon_positions: vec![
                Vector3::new(327.0, 72.0, 0.9),
                Vector3::new(327.0, 53.0, 0.9),
                Vector3::new(327.0, 34.0, 0.9),
            ],
            icon_entities: vec![None, None, None],
        }
    }

    fn destroy_icon(&mut self, index: usize, entities: &Entities) {
        if let Some(icon_entity) = self.icon_entities[index] {
            entities
                .delete(icon_entity)
                .expect("Unable to delete store icon entity.");
            self.icon_entities[index] = None;
        }
    }

    fn create_icon(
        &mut self,
        index: usize,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        items_resource: &ReadExpect<ItemsResource>,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        if let Some(spawnable_type) = self.inventory[index].clone() {
            self.destroy_icon(index, entities);

            let sprite_render = SpriteRender {
                sprite_sheet: spritesheets_resource.spritesheets
                    [&spawnable_type.get_spritesheet_name()].clone(),
                sprite_number: match spawnable_type {
                    SpawnableType::Consumable(consumable_type) => {
                        consumables_resource.consumable_entities[&consumable_type].sprite_render_data.initial_index
                    },
                    SpawnableType::Item(item_type) => {
                        items_resource.item_entities[&item_type].sprite_render_data.initial_index
                    },
                    _ => {panic!("Attempted to set icon to invalid Spawnable. Icons must be sent to items or consumables.")}
                },
            };

            let mut transform = Transform::default();
            transform.set_translation(self.icon_positions[index].into());

            let icon_entity = lazy_update
                .create_entity(entities)
                .with(sprite_render)
                .with(transform)
                .build();

            self.icon_entities[index] = Some(icon_entity);
        }
    }

    pub fn update(
        &mut self,
        delta_time: f32,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        items_resource: &ReadExpect<ItemsResource>,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) -> bool {
        if self.timer.update(delta_time) {
            self.choose_stock(
                spritesheets_resource,
                items_resource,
                consumables_resource,
                entities,
                lazy_update,
            );
            return true;
        }
        false
    }

    fn choose_stock(
        &mut self,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        items_resource: &ReadExpect<ItemsResource>,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        self.inventory = vec![None, None, None];
        let mut choose_pool = self.stock.clone();

        // choose 3 items
        for i in 0..3 {
            let total_probs = choose_pool.iter().fold(0.0, |sum, item| sum + item.1);

            // choose an item
            let pos = thread_rng().gen::<f32>() * total_probs;
            let mut sum = 0.0;

            for (entity_type, value) in choose_pool.clone() {
                sum += value;
                if sum > pos {
                    //let item_to_add = &items_resource.item_entities[&item_type];
                    choose_pool.retain(|element| element != &(entity_type.clone(), value)); // remove chosen item from cloned choose pool

                    self.inventory[i] = Some(entity_type.clone());
                    self.create_icon(
                        i,
                        spritesheets_resource,
                        items_resource,
                        consumables_resource,
                        entities,
                        lazy_update,
                    );

                    let entity_index = self
                        .stock
                        .iter()
                        .position(|element| element == &(entity_type.clone(), value))
                        .unwrap();

                    if let SpawnableType::Item(_) = entity_type {
                        self.stock[entity_index].1 /= 2.0; // divide probability of item appearing in store by 2
                    }

                    break;
                }
            }
        }
    }

    pub fn purchase(
        &mut self,
        inventory_index: usize,
        entities: &Entities,
        player: &mut PlayerComponent,
        transform: &Transform,
        items_resource: &ReadExpect<ItemsResource>,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) -> bool {
        if let Some(entity_type) = self.inventory[inventory_index].clone() {
            self.destroy_icon(inventory_index, entities);
            match &entity_type {
                SpawnableType::Item(item_type) => {
                    let item_data = items_resource.item_entities[&item_type].clone();
                    if player.money >= item_data.item_component.price {
                        player.money -= item_data.item_component.price;

                        let mut spawn_transform = Transform::default();
                        spawn_transform.set_translation_xyz(
                            transform.translation().x,
                            ARENA_MAX_Y + ITEM_SPAWN_Y_OFFSET,
                            0.0,
                        );

                        items_resource.spawn_item(
                            &item_type,
                            false,
                            spawn_transform,
                            spritesheets_resource,
                            entities,
                            lazy_update,
                        );

                        for (i, e_type) in self.stock.iter().enumerate() {
                            if e_type.0 == entity_type {
                                self.stock[i].1 = 0.0; //set probability of appearing again to 0
                                break;
                            }
                        }

                        self.inventory[inventory_index] = None; //change item slot data to None
                        return true;
                    }
                }
                SpawnableType::Consumable(consumable_type) => {
                    let consumable_data =
                        consumables_resource.consumable_entities[&consumable_type].clone();
                    if player.money >= consumable_data.consumable_component.price {
                        player.money -= consumable_data.consumable_component.price;

                        let mut spawn_transform = Transform::default();
                        spawn_transform.set_translation_xyz(
                            transform.translation().x,
                            ARENA_MAX_Y + ITEM_SPAWN_Y_OFFSET,
                            0.0,
                        );

                        consumables_resource.spawn_consumable(
                            &consumable_type,
                            false,
                            spawn_transform,
                            spritesheets_resource,
                            entities,
                            lazy_update,
                        );

                        self.inventory[inventory_index] = None; //change item slot data to None
                        return true;
                    }
                }
                _ => panic!("Only items and consumables can be purchased in the store."),
            }
        }
        false
    }
}
