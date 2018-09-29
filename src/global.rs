// Global info for the game and JSON parsing
use modscript;
use modscript::Value as msValue;
use serde_json;
use serde_json::Value as jsonValue;
use modscript::{Callable, FuncMap, expr_from_text};

use super::entity::{Entity, EntityInst};
use super::level::{Level, LevelInst, TileInfo};

use std::collections::HashMap;
use std::rc::Rc;
use std::fs::File;
use std::io::{BufReader, Read};

pub struct Global {
    // Code
    pub source: Rc<FuncMap>,

    // Main functions
    //tick: Callable,
    //end: Callable,

    // Constructors
    entities: HashMap<String, Rc<Entity>>,
    levels: HashMap<String, Rc<Level>>,
    //layouts: HashMap<String, Layout>,

    // Runtime data
    //glob_data:
    //pub glob_obj: msValue,
    id_count: u64,
    glob_instances: HashMap<u64, EntityInst>,
    level_instances: HashMap<u64, LevelInst>,
}

impl Global {
    pub fn new() -> Self {
        Global {
            source: Rc::new(FuncMap::new()),

            entities: HashMap::new(),
            levels: HashMap::new(),

            id_count: 0,
            glob_instances: HashMap::new(),
            level_instances: HashMap::new(),
        }
    }


    // WARNING: horrible function TODO: make less horrible
    pub fn init_game(&mut self, hub_file_name: &str, source: FuncMap) -> Result<(), serde_json::Error> {
        // parse JSON
            // .mod source code (compile)
            // entities (delayed object create?)
            // levels (delayed object create?)
            // layouts
            // global object
            // global data
            // global funcs


        let hub_file = read_file(hub_file_name);
        let hub_data: jsonValue = serde_json::from_str(&hub_file)?;

        // TODO: better error handling
        for src in hub_data["source"].as_array().unwrap().iter() {
            let package_name = src.as_str().expect("Source file not a string!");
            let package = modscript::package_from_file(package_name).unwrap();
            source.attach_package(package_name, package.call_ref());
        }

        self.source 

        /* ENTITIES */
        // TODO: support single entity file
        for entity_file_name in hub_data["entities"].as_array().unwrap().iter() {
            let entity_file = read_file(entity_file_name.as_str().unwrap());
            let entity_data: jsonValue = serde_json::from_str(&entity_file)?;

            let packs = match entity_data.get("imports") {
                Some(i) => {
                    let mut p = Vec::new();
                    for (ref k, ref v) in i.as_object().unwrap().iter() {
                        let value = v.as_str().unwrap();
                        p.push((k.clone(), value.to_string()));
                    }
                    p
                },
                None => Vec::new(),
            };

            for (ref name, ref ent) in entity_data["entities"].as_object().unwrap().iter() {
                self.entities.insert(name.clone(), Rc::new(Entity::new(
                    &name,
                    ent["key"].as_str().unwrap().chars().next().unwrap(),
                    eval_snippet(&packs, ent.get("init"), &self.source).unwrap(),
                    eval_snippet(&packs, ent.get("pre_action"), &self.source).unwrap(),
                    eval_snippet(&packs, ent.get("action"), &self.source).unwrap(),
                    eval_snippet(&packs, ent.get("post_action"), &self.source).unwrap(),
                    eval_snippet(&packs, ent.get("delete"), &self.source).unwrap(),
                    self.source.clone()
                )));
            }
        }
        /* ENTITIES */

        /* LEVELS */
        // TODO: support single level file
        for level_file_name in hub_data["levels"].as_array().unwrap().iter() {
            let level_file = read_file(level_file_name.as_str().unwrap());
            let level_data: jsonValue = serde_json::from_str(&level_file)?;

            let packs = match level_data.get("imports") {
                Some(i) => {
                    let mut p = Vec::new();
                    for (ref k, ref v) in i.as_object().unwrap().iter() {
                        let value = v.as_str().unwrap();
                        p.push((k.clone(), value.to_string()));
                    }
                    p
                },
                None => Vec::new(),
            };

            let tile = level_data["tiles"];
            let default_tile = tile["default"].as_str().unwrap().chars().next();
            let mut collide_tiles = HashMap::new();
            for v in tile["collide"].as_array().unwrap().iter() {
                let ch = v.as_str().unwrap().chars().next();
                collide_tiles.insert(ch.unwrap(), true);
            }
            for v in tile["nocollide"].as_array().unwrap().iter() {
                let ch = v.as_str().unwrap().chars().next();
                collide_tiles.insert(ch.unwrap(), false);
            }

            let tile_info = Rc::new(TileInfo::new(default_tile.unwrap(), collide_tiles));

            for (ref name, ref lev) in level_data["levels"].as_object().unwrap().iter() {
                self.levels.insert(name.clone(), Rc::new(Level::new(
                    lev["x"].as_u64().unwrap(),
                    lev["y"].as_u64().unwrap(),
                    tile_info.clone(),
                    eval_snippet(&packs, lev.get("init"), &self.source).unwrap(),
                    eval_snippet(&packs, lev.get("delete"), &self.source).unwrap(),
                    self.source.clone()
                )));
            }
        }
        /* LEVELS */

        /* LAYOUTS */

        /* LAYOUTS */

        // run init -> glob_obj
        Ok(())
    }

}

// TODO: move this somewhere better
fn read_file(file_name: &str) -> String {
    let file = File::open(file_name).expect("Couldn't open file.");

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents).expect("Error reading file into string.");
    contents
}

fn eval_snippet(imports: &[(String, String)], script: Option<&jsonValue>, libs: &FuncMap) -> Result<Callable, String> {
    match script {
        Some(s) => {
            let script_str = s.as_str().unwrap();
            let expr = expr_from_text(imports, script_str)?;
            let val = expr.run(libs)?;
            Ok(Callable::new(Some(val)))
        },
        None => Ok(Callable::new(None)),
    }
}
