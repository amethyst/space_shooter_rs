use crate::{
    components::{HealthComponent, PlayerComponent},
    entities::SpawnableType,
    resources::StoreResource,
    spawnable::resources::{ConsumablesResource, ItemsResource},
};
use amethyst::{
    ecs::prelude::{Entity, Join, ReadExpect, ReadStorage, System, WriteStorage},
    ui::UiText,
};

/// Holds variable values that are displayed
// TODO: this doesn't belong here
pub struct TrackedStats {
    pub currency: Entity,
    pub shields: Entity,
    pub item_price_1: Entity,
    pub item_price_2: Entity,
    pub item_price_3: Entity,
}

/// Handles and displays tracked stats
pub struct StatTrackerSystem;

impl<'s> System<'s> for StatTrackerSystem {
    /// Data used by the system
    type SystemData = (
        ReadStorage<'s, PlayerComponent>,
        ReadStorage<'s, HealthComponent>,
        ReadExpect<'s, StoreResource>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, TrackedStats>,
        ReadExpect<'s, ItemsResource>,
        ReadExpect<'s, ConsumablesResource>,
    );

    /// System game logic
    fn run(
        &mut self,
        (
            players,
            healths,
            store_resource,
            mut ui_text,
            tracked_stats,
            items_resource,
            consumables_resource,
        ): Self::SystemData,
    ) {
        for character in (&players).join() {
            if let Some(text) = ui_text.get_mut(tracked_stats.currency) {
                text.text = format!("x{}", character.money.to_string());
            }
        }

        for (_player, health) in (&players, &healths).join() {
            if let Some(text) = ui_text.get_mut(tracked_stats.shields) {
                text.text = format!("x{}", health.armor.to_string());
            }
        }

        if let Some(text) = ui_text.get_mut(tracked_stats.item_price_1) {
            text.text = format!(
                "${}",
                inventory_price(
                    &store_resource.inventory[0],
                    &items_resource,
                    &consumables_resource
                )
            );
        }

        if let Some(text) = ui_text.get_mut(tracked_stats.item_price_2) {
            text.text = format!(
                "${}",
                inventory_price(
                    &store_resource.inventory[1],
                    &items_resource,
                    &consumables_resource
                )
            );
        }

        if let Some(text) = ui_text.get_mut(tracked_stats.item_price_3) {
            text.text = format!(
                "${}",
                inventory_price(
                    &store_resource.inventory[2],
                    &items_resource,
                    &consumables_resource
                )
            );
        }
    }
}

/// Get the price of an item or consumable in the store
fn inventory_price(
    inventory_entity: &Option<SpawnableType>,
    items_resource: &ReadExpect<ItemsResource>,
    consumables_resource: &ReadExpect<ConsumablesResource>,
) -> usize {
    match inventory_entity {
        Some(SpawnableType::Item(item_type)) => {
            items_resource.item_entities[item_type]
                .item_component
                .price
        }
        Some(SpawnableType::Consumable(consumable_type)) => {
            consumables_resource.consumable_entities[consumable_type]
                .consumable_component
                .price
        }
        _ => 0,
    }
}
