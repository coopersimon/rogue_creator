use std::collections::HashMap;
use textitem::TextItem;

pub type TileID = u32;

pub struct TileItem {
    pub text: TextItem,
    pub collide: bool,
}

impl TileItem {
    // TODO: add colours, effects, error handling
    pub fn new(text: String, collide: bool) -> Self {
        TileItem {
            text: TextItem::new_tile(text),
            collide: collide,
        }
    }
}

pub struct TileInfo {
    default: TileID,
    data: HashMap<TileID, TileItem>,
    ids: HashMap<String, TileID>,
    id_gen: TileID,
}

impl TileInfo {
    pub fn new() -> Self {
        TileInfo {
            default: 0,
            data: HashMap::new(),
            ids: HashMap::new(),
            id_gen: 0,
        }
    }

    pub fn add_tile(&mut self, name: &str, data: TileItem) {
        self.id_gen += 1;
        self.ids.insert(name.to_string(), self.id_gen);
        self.data.insert(self.id_gen, data);
    }

    pub fn set_default(&mut self, name: &str) {
        self.default = *self.ids.get(name).unwrap();
    }

    pub fn get_default(&self) -> TileID {
        self.default
    }

    pub fn get_id(&self, name: &str) -> Option<TileID> {
        match self.ids.get(name) {
            Some(ref v) => Some(*v.clone()),
            None => None,
        }
    }

    pub fn get_item(&self, id: TileID) -> Option<&TileItem> {
        self.data.get(&id)
    }
}
