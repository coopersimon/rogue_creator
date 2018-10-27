use modscript::Value;

use super::entity::EntityInst;
use super::level::LevelInst;

use std::collections::HashMap;

// Runtime data for game
pub struct State {
    pub glob_obj: Value,
    pub current_layout: String,
    //glob_data:
    pub glob_instances: HashMap<u64, EntityInst>,
    pub level_instances: HashMap<u64, LevelInst>,

    id_count: u64,
    active_level: u64,
}

impl State {
    pub fn new() -> Self {
        State {
            glob_obj: Value::Null,
            current_layout: String::new(),
            glob_instances: HashMap::new(),
            level_instances: HashMap::new(),
            id_count: 0,
            active_level: 0,
        }
    }
}
