pub struct IncludeData<'a> {
    pub items: &'a [u8],
    pub enemies: &'a [u8],
    pub consumables: &'a [u8],
    pub effects: &'a [u8],
    pub players: &'a [u8],
    pub phases: &'a [u8],
    pub store: &'a [u8],
    pub game_parameters: &'a [u8],
    pub formations: &'a [u8],
}

pub fn load_include_data() -> IncludeData<'static> {
    IncludeData {
        items: include_bytes!("items.ron"),
        enemies: include_bytes!("enemies.ron"),
        consumables: include_bytes!("consumables.ron"),
        effects: include_bytes!("effects.ron"),
        players: include_bytes!("players.ron"),
        phases: include_bytes!("phases.ron"),
        store: include_bytes!("store.ron"),
        game_parameters: include_bytes!("game_parameters.ron"),
        formations: include_bytes!("formations.ron"),
    }
}

