use modscript::{Value, VType, ExprRes, Error};

use super::entity::EntityInst;
use super::level::LevelInst;
use super::global::Global;
use super::Coord;
use super::textrender::MapCommand;

use std::collections::HashMap;
use std::cmp;
use std::sync::mpsc::Sender;

// Runtime data for game
pub struct State {
    glob_obj: Value,
    current_layout: String,
    glob_instances: HashMap<u64, EntityInst>,
    level_instances: HashMap<u64, LevelInst>,

    id_count: u64,
    active_level: u64,
    active_entity: u64,

    map_data_sender: Sender<MapCommand>,
}

impl State {
    pub fn new(map_data_sender: Sender<MapCommand>) -> Self {
        State {
            glob_obj: Value::Null,
            current_layout: String::new(),
            glob_instances: HashMap::new(),
            level_instances: HashMap::new(),

            id_count: 0,
            active_level: 0,
            active_entity: 0,

            map_data_sender: map_data_sender,
        }
    }

    pub fn set_glob_obj(&mut self, val: Value) {
        self.glob_obj = val;
    }

    pub fn get_glob_obj(&self) -> Value {
        self.glob_obj.clone()
    }

    pub fn set_current_layout(&mut self, c_l: &str) {
        self.current_layout = c_l.to_string()
    }

    pub fn get_current_layout<'a>(&'a self) -> &'a str {
        &self.current_layout
    }

    pub fn set_active_level(&mut self, id: u64) {
        self.active_level = id;
        if id != 0 {
            let level = self.level_instances.get(&self.active_level).expect(&format!("No active level {}", id));
            self.map_data_sender.send(MapCommand::NewLevel(level.x_size(), level.y_size())).unwrap();
        }
    }

    pub fn get_active_level(&self) -> u64 {
        self.active_level
    }

    pub fn set_active_entity(&mut self, id: u64) {
        self.active_entity = id;
    }

    pub fn clear_active_entity(&mut self) {
        self.active_entity = 0;
    }

    pub fn get_active_entity(&self) -> u64 {
        self.active_entity
    }

    pub fn prepare_render(&self) {
        let level = self.level_instances.get(&self.active_level).expect("No active level");

        level.send_text_map_data(&self.map_data_sender, &self.glob_instances);
    }
}

// LEVEL
impl State {
    pub fn create_level(&mut self, glob: &Global, name: &str) -> Result<u64, Error> {
        let instance = glob.new_level_instance(name)?;
        self.id_count += 1; // TODO (?): more robust id generation
        self.level_instances.insert(self.id_count, instance);
        Ok(self.id_count)
    }

    pub fn delete_level(&mut self, id: i64) -> ExprRes {
        let id = id as u64;
        self.level_instances.remove(&id);
        Ok(Value::Null)
    }

    pub fn clone_level(&mut self, id: i64) -> ExprRes {
        let instance = self.level_instances.get(&(id as u64)).unwrap().clone();
        self.id_count += 1;
        self.level_instances.insert(self.id_count, instance);
        Ok(Value::Val(VType::I(self.id_count as i64)))
    }

    pub fn set_level_obj(&mut self, val: Value) {
        self.level_instances.get_mut(&self.active_level).unwrap().set_data(val);
    }

    pub fn get_level_obj(&self) -> ExprRes {
        Ok(self.level_instances.get(&self.active_level).unwrap().get_data())
    }

    pub fn instance_at(&self, at: Coord) -> ExprRes {
        Ok(match self.level_instances.get(&self.active_level).unwrap().instance_at(at) {
            Some(i) => Value::Val(VType::I(i as i64)),
            None    => Value::Null,
        })
    }

    pub fn location_of(&self, id: i64) -> ExprRes {
        Ok(match self.level_instances.get(&self.active_level).unwrap().location_of(id as u64) {
            Some(_l) => {
                // TODO: create object
                Value::Null
            },
            None    => Value::Null,
        })
    }
}

// ENTITY
impl State {
    pub fn create_glob_entity(&mut self, glob: &Global, name: &str) -> Result<u64, Error> {
        let instance = glob.new_entity_instance(name)?;
        self.id_count += 1; // TODO (?): more robust id generation
        self.glob_instances.insert(self.id_count, instance);
        Ok(self.id_count)
    }

    pub fn create_local_entity(&mut self, glob: &Global, name: &str) -> Result<u64, Error> {
        let instance = glob.new_entity_instance(name)?;
        self.id_count += 1; // TODO (?): more robust id generation
        self.level_instances.get_mut(&self.active_level).unwrap()
            .add_instance(self.id_count, instance);
        Ok(self.id_count)
    }

    pub fn delete_entity(&mut self, id: i64) -> ExprRes {
        let id = id as u64;
        self.glob_instances.remove(&id);

        let level = self.level_instances.get_mut(&self.active_level).unwrap();
        level.remove_instance(id);
        level.despawn_instance(id);
        Ok(Value::Null)
    }

    pub fn set_entity_obj(&mut self, id: u64, val: Value) {
        match self.glob_instances.get_mut(&id) {
            Some(e) => e.set_data(val),
            None    => self.level_instances.get_mut(&self.active_level)
                           .unwrap()
                           .set_entity_data(id, val),
        }
    }

    pub fn get_entity_obj(&self, id: u64) -> ExprRes {
        match self.glob_instances.get(&id) {
            Some(e) => Ok(e.get_data()),
            None    => Ok(self.level_instances.get(&self.active_level)
                              .unwrap()
                              .get_entity_data(id)),
        }
    }

    pub fn active_entity_obj(&self) -> ExprRes {
        if self.active_entity != 0 {
            self.get_entity_obj(self.active_entity)
        } else {
            Ok(Value::Null) // or custom err?
        }
    }

    pub fn get_entity_name(&self, id: i64) -> String {
        match self.glob_instances.get(&(id as u64)) {
            Some(e) => e.get_name(),
            None    => self.level_instances.get(&self.active_level)
                              .unwrap()
                              .get_entity_name(id as u64),
        }
    }

    pub fn run_actions(&mut self) -> ExprRes {
        // run all entities actions
        // all entities post-actions
        Ok(Value::Null)
    }

    // TODO: below should accept closure or callable as argument
    /*pub fn run_actions_ordered(&mut self, comp: &Value) -> ExprRes {
        use modscript::Value::*;
        let compare = |a,b| match *comp {
            Func(ref p, ref n)      => {
                self.source.call_fn(&p.borrow(), &n.borrow(), &vec![a,b])
            },
            Closure(ref f, ref a)   => {
                f.borrow().call(&vec![a,b], &*self.source, Some(a.borrow()))
            },
            _ => return mserr(Type::RunTime(RunCode::TypeError)),
        };

        // sort all entities
        // run actions
        // run post-actions
        Ok(Value::Null)
    }*/

}

// LEVEL MAP
impl State {
    pub fn fill_tiles(&mut self, tile_name: &str, tl: Coord, br: Coord) -> ExprRes {
        let level = self.level_instances.get_mut(&self.active_level).unwrap();
        let tile = level.get_tile_id(tile_name).unwrap();
        let y_start = cmp::min(tl.1, br.1);
        let x_start = cmp::min(tl.0, br.0);
        let y_range = (tl.1 as isize - br.1 as isize).abs() as usize;
        let x_range = (tl.0 as isize - br.0 as isize).abs() as usize;

        for y in y_start..(y_start + y_range) {
            for x in x_start..(x_start + x_range) {
                level.set_tile(tile, (x,y));
            }
        }
        Ok(Value::Null)
    }

    pub fn draw_line(&mut self, tile_name: &str, s: Coord, e: Coord) -> ExprRes {
        let level = self.level_instances.get_mut(&self.active_level).unwrap();
        let tile = level.get_tile_id(tile_name).unwrap();
        let y_start = cmp::min(s.1, e.1);
        let x_start = cmp::min(s.0, e.0);
        let y_range = (s.1 as isize - e.1 as isize).abs() as usize;
        let x_range = (s.0 as isize - e.0 as isize).abs() as usize;

        if x_range != 0 {
            let gradient = y_range as f64 / x_range as f64;
            let mut last_y = y_start;
            for x in x_start..=(x_start + x_range) {
                let new_y = (gradient * (x - x_start) as f64) as usize + y_start;
                for y in last_y..=new_y {
                    level.set_tile(tile, (x,y));
                }

                //level.set_tile(tile, (x,new_y));
                last_y = new_y;
            }
        } else {
            for y in y_start..=(y_start + y_range) {
                level.set_tile(tile, (x_start,y));
            }
        }

        Ok(Value::Null)
    }

    pub fn spawn_entity(&mut self, entity: i64, loc: Coord) -> ExprRes {
        let level = self.level_instances.get_mut(&self.active_level).unwrap();
        let spawned = level.spawn_instance(entity as u64, loc);
        Ok(Value::Val(VType::B(spawned)))
    }

    pub fn despawn_entity(&mut self, entity: i64) -> ExprRes {
        let level = self.level_instances.get_mut(&self.active_level).unwrap();
        let despawned = level.despawn_instance(entity as u64);
        Ok(Value::Val(VType::B(despawned)))
    }
}
