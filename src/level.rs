use modscript::{Value, ScriptExpr, FuncMap, Error, ExprRes};

use Coord;
use tile::{TileInfo, TileID};
use super::entity::EntityInst;
use textrender::MapCommand;

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::mpsc::Sender;

pub struct Level {
    x: usize,
    y: usize,
    tile_info: Rc<TileInfo>,
    init: ScriptExpr,
    delete: ScriptExpr,
}

#[derive(Clone)]
pub struct LevelInst {
    tile_info: Rc<TileInfo>,
    tile_map: Vec<Vec<TileID>>,
    local_instances: HashMap<u64, EntityInst>,
    instance_locs: HashMap<u64, Coord>,
    data: Value,
}


impl Level {
    pub fn new(x_size: u64,
        y_size: u64,
        tile_info: Rc<TileInfo>,
        init: ScriptExpr,
        delete: ScriptExpr,
        ) -> Self
    {
        Level {
            x: x_size as usize,
            y: y_size as usize,
            tile_info: tile_info,
            init: init,
            delete: delete,
        }
    }

    pub fn new_instance(&self) -> LevelInst {
        LevelInst {
            tile_info: self.tile_info.clone(),
            tile_map: vec![vec![self.tile_info.get_default(); self.x]; self.y],
            local_instances: HashMap::new(),
            instance_locs: HashMap::new(),
            data: Value::Null,
        }
    }

    pub fn init(&self, source: &FuncMap) -> ExprRes {
        self.init.run(source)
    }

    pub fn delete(&self, source: &FuncMap) -> ExprRes {
        self.delete.run(source)
    }
}

impl LevelInst {
    // TODO: Error handling here?
    pub fn get_tile_id(&self, tile_name: &str) -> Option<TileID> {
        self.tile_info.get_id(tile_name)
    }

    pub fn set_tile(&mut self, tile: TileID, loc: Coord) -> bool {
        let (x,y) = loc;
        if y >= self.tile_map.len() {
            false
        } else if x >= self.tile_map[0].len() {
            false
        } else {
            // Assuming ID is valid
            self.tile_map[y][x] = tile;
            true
        }
    }

    /* trait InstanceStore */
    pub fn add_instance(&mut self, id: u64, instance: EntityInst) {
        self.local_instances.insert(id, instance);
    }

    pub fn remove_instance(&mut self, id: u64) {
        self.local_instances.remove(&id);
    }
    /* trait InstanceStore */

    pub fn spawn_instance(&mut self, id: u64, loc: Coord) -> bool {
        let (x,y) = loc;
        if y >= self.tile_map.len() {
            false
        } else if x >= self.tile_map[0].len() {
            false
        } else if self.tile_info.get_item(self.tile_map[y][x]).unwrap().collide {
            false
        } else {
            self.instance_locs.insert(id, loc);
            true
        }
    }

    pub fn despawn_instance(&mut self, id: u64) -> bool {
        self.instance_locs.remove(&id).is_some()
    }

    pub fn instance_at(&self, loc: Coord) -> Option<u64> {
        for (&k, &v) in self.instance_locs.iter() {
            if v == loc {
                return Some(k.clone());
            }
        }
        None
    }

    pub fn location_of(&self, id: u64) -> Option<Coord> {
        match self.instance_locs.get(&id) {
            Some(i) => Some(i.clone()),
            None => None,
        }
    }

    pub fn set_data(&mut self, val: Value) {
        self.data = val;
    }

    pub fn get_data(&self) -> Value {
        self.data.clone()
    }

    pub fn x_size(&self) -> usize {
        match self.tile_map.len() {
            0 => 0,
            _ => self.tile_map[0].len()
        }
    }

    pub fn y_size(&self) -> usize {
        self.tile_map.len()
    }

    pub fn set_entity_data(&mut self, id: u64, val: Value) {
        self.local_instances.get_mut(&id).unwrap().set_data(val);
    }

    pub fn get_entity_data(&self, id: u64) -> Value {
        self.local_instances.get(&id).unwrap().get_data()
    }

    pub fn get_entity_name(&self, id: u64) -> String {
        self.local_instances.get(&id).unwrap().get_name()
    }

    pub fn send_text_map_data(&self, sender: &Sender<MapCommand>, glob_instances: &HashMap<u64, EntityInst>) {
        let tile_info = self.tile_info.clone();
        sender.send(MapCommand::TileInfo(tile_info)).unwrap();
        sender.send(MapCommand::TileData(self.tile_map.clone())).unwrap();
        for (&k, &v) in self.instance_locs.iter() {
            let tile = match self.local_instances.get(&k) {
                Some(e) => e.get_tile(),
                None    => match glob_instances.get(&k) {
                    Some(e) => e.get_tile(),
                    None    => panic!("Handle this error better"),
                },
            };
            sender.send(MapCommand::Sprite(v.clone(), tile)).unwrap();
        }
    }
}
