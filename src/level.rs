use modscript::{Value, Callable, FuncMap};

use Coord;
use super::entity::EntityInst;
use super::global::Global;

use std::collections::HashMap;
use std::rc::Rc;

pub struct TileInfo {
    default_tile: char,
    collide_tiles: HashMap<char, bool>,
}

impl TileInfo {
    pub fn new(default_tile: char, collide_tiles: HashMap<char, bool>) -> Self {
        TileInfo {
            default_tile: default_tile,
            collide_tiles: collide_tiles,
        }
    }
}

pub struct Level {
    x: usize,
    y: usize,
    tile_info: Rc<TileInfo>,
    init: Callable,
    delete: Callable,
    source: Rc<FuncMap>,
}

impl Level {
    pub fn new(x_size: u64,
        y_size: u64,
        tile_info: Rc<TileInfo>,
        init: Callable,
        delete: Callable,
        source: Rc<FuncMap>
        ) -> Self
    {
        Level {
            x: x_size as usize,
            y: y_size as usize,
            tile_info: tile_info,
            init: init,
            delete: delete,
            source: source,
        }
    }

    /*pub fn create_instance(&self) -> LevelInst {
        LevelInst {
            level: Rc::clone(self),
            tile_map: vec![vec![self.tile_info.default_tile; self.x]; self.y],
            local_instances: HashMap::new(),
            instance_locs: HashMap::new(),
            data: Value::Null,
        }
    }*/
}

pub struct LevelInst {
    level: Rc<Level>,
    tile_map: Vec<Vec<char>>,
    local_instances: HashMap<u64, EntityInst>,
    instance_locs: HashMap<Coord, u64>,
    data: Value,
}

impl LevelInst {
    pub fn new(level: Rc<Level>) -> Self {
        LevelInst {
            level: level.clone(),
            tile_map: vec![vec![level.tile_info.default_tile; level.x]; level.y],
            local_instances: HashMap::new(),
            instance_locs: HashMap::new(),
            data: Value::Null,
        }
    }

    pub fn init(&mut self) {
        self.data = self.level.init.call(&self.level.source, &[]).unwrap();
    }

    pub fn set_tile(&mut self, tile: char, loc: Coord) -> bool {
        let (x,y) = loc;
        if y >= self.tile_map.len() {
            false
        } else if x >= self.tile_map[0].len() {
            false
        } else if self.level.tile_info.collide_tiles.contains_key(&tile) {
            self.tile_map[y][x] = tile;
            true
        } else {
            false
        }
    }

    /* trait InstanceStore */
    pub fn add_instance(&mut self, instance: EntityInst) {
        let id = instance.id;
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
        } else if self.level.tile_info.collide_tiles[&self.tile_map[y][x]] {
            false
        } else {
            self.instance_locs.insert(loc, id);
            true
        }
    }

    pub fn despawn_instance(&mut self, id: u64) -> bool {
        let start_len = self.instance_locs.len();
        self.instance_locs.retain(|_, v| *v != id);
        start_len != self.instance_locs.len()
    }

    // TODO: cleaner way of doing this?
    pub fn instance_at(&self, loc: Coord, glob: &mut Global) -> Option<u64> {
        match self.instance_locs.get(&loc) {
            Some(i) => Some(i.clone()),
            None => None,
        }
    }

    pub fn location_of(&self, id: u64) -> Option<Coord> {
        for (&k, &v) in self.instance_locs.iter() {
            if v == id {
                return Some(k.clone());
            }
        }
        None
    }

    pub fn get_data(&self) -> Value {
        self.data.clone()
    }
}

impl Drop for LevelInst {
    fn drop(&mut self) {
        self.level.delete.call(&self.level.source, &[self.data.clone()]).unwrap();
    }
}
