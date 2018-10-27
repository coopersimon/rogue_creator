use modscript::Value;

use super::entity::EntityInst;
use super::level::LevelInst;

use std::collections::HashMap;

// Runtime data for game
pub struct State {
    pub glob_obj: Value,
    pub current_layout: String,
    //glob_data:
    pub id_count: u64,
    pub glob_instances: HashMap<u64, EntityInst>,
    pub level_instances: HashMap<u64, LevelInst>,
}

impl State {
    pub fn new() -> Self {
        State {
            glob_obj: Value::Null,
            current_layout: String::new(),
            id_count: 0,
            glob_instances: HashMap::new(),
            level_instances: HashMap::new(),
        }
    }
}
