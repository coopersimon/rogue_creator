// Global info for the game and JSON parsing
use modscript;
use modscript::Value as msValue;
use serde_json;
use serde_json::Value as jsonValue;
use modscript::{Callable, FuncMap, expr_from_text, ExprRes, VType};

use super::entity::{Entity, EntityInst};
use super::level::{Level, LevelInst, TileInfo};
use super::layout::Layout;

use std::collections::HashMap;
use std::rc::Rc;
use std::fs::File;
use std::io::{BufReader, Read};

pub struct Global {
    // Code
    pub source: Rc<FuncMap>,

    // Main functions
    init: Callable,
    tick: Callable,
    end: Callable,

    // Constructors
    entities: HashMap<String, Rc<Entity>>,
    levels: HashMap<String, Rc<Level>>,

    // Layout
    layouts: HashMap<String, Layout>,

    // Database
    // glob_data

    // Mutable data
    pub glob_obj: msValue,
    pub current_layout: String,
    pub glob_instances: HashMap<u64, EntityInst>,
    pub level_instances: HashMap<u64, LevelInst>,

    id_count: u64,
    active_level: u64,
}

impl Global {
    pub fn new() -> Self {
        Global {
            source: Rc::new(FuncMap::new()),

            init: Callable::new(None),
            tick: Callable::new(None),
            end: Callable::new(None),

            entities: HashMap::new(),
            levels: HashMap::new(),
            layouts: HashMap::new(),

            glob_obj: msValue::Null,
            current_layout: String::new(),
            glob_instances: HashMap::new(),
            level_instances: HashMap::new(),

            id_count: 0,
            active_level: 0,
        }
    }


    // WARNING: horrible function TODO: make less horrible
    pub fn init_game(&mut self, hub_file_name: &str) -> Result<(), serde_json::Error> {
        // parse JSON
            // .mod source code (compile)
            // entities (delayed object create?)
            // levels (delayed object create?)
            // layouts
            // global object
            // global data
            // global funcs

        let root_dir = hub_file_name.split('/').next().unwrap().to_owned() + "/";

        let hub_file = read_file(hub_file_name);
        let hub_data: jsonValue = serde_json::from_str(&hub_file)?;

        // TODO: better error handling
        for src in hub_data["source"].as_array().unwrap().iter() {
            let package_name = src.as_str().expect("Source file not a string!");
            let package = modscript::package_from_file(&(root_dir.to_owned() + package_name)).unwrap();
            Rc::get_mut(&mut self.source).unwrap().attach_package(package_name, package.call_ref());
        }

        /* ENTITIES */
        // TODO: support single entity file
        for entity_file_name in hub_data["entities"].as_array().unwrap().iter() {
            let entity_file = read_file(&(root_dir.to_owned() + entity_file_name.as_str().unwrap()));
            let entity_data: jsonValue = serde_json::from_str(&entity_file)?;

            let packs = match entity_data.get("imports") {
                Some(i) => {
                    let mut p = Vec::new();
                    for (ref k, ref v) in i.as_object().unwrap().iter() {
                        let value = v.as_str().unwrap();
                        p.push((k.to_string(), value.to_string()));
                    }
                    p
                },
                None => Vec::new(),
            };

            for (ref name, ref ent) in entity_data["entities"].as_object().unwrap().iter() {
                self.entities.insert(name.to_string(), Rc::new(Entity::new(
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
            let level_file = read_file(&(root_dir.to_owned() + level_file_name.as_str().unwrap()));
            let level_data: jsonValue = serde_json::from_str(&level_file)?;

            let packs = match level_data.get("imports") {
                Some(i) => {
                    let mut p = Vec::new();
                    for (ref k, ref v) in i.as_object().unwrap().iter() {
                        let value = v.as_str().unwrap();
                        p.push((k.to_string(), value.to_string()));
                    }
                    p
                },
                None => Vec::new(),
            };

            let tile = level_data["tiles"].as_object().unwrap();
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
                self.levels.insert(name.to_string(), Rc::new(Level::new(
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
        // TODO: support single layout file
        for layout_file_name in hub_data["layouts"].as_array().unwrap().iter() {
            let layout_file = read_file(&(root_dir.to_owned() + layout_file_name.as_str().unwrap()));
            let layout_data: jsonValue = serde_json::from_str(&layout_file)?;

            let packs = match layout_data.get("imports") {
                Some(i) => {
                    let mut p = Vec::new();
                    for (ref k, ref v) in i.as_object().unwrap().iter() {
                        let value = v.as_str().unwrap();
                        p.push((k.to_string(), value.to_string()));
                    }
                    p
                },
                None => Vec::new(),
            };

            for (ref name, ref layout) in layout_data["layouts"].as_object().unwrap().iter() {
                let inputs = match layout.get("inputs") {
                    Some(i) => {
                        let mut m = HashMap::new();
                        for (ref k, ref v) in i.as_object().unwrap().iter() {
                            let ch = k.to_string().chars().next();
                            m.insert(ch.unwrap(), eval_snippet(&packs, Some(v), &self.source).unwrap());
                        }
                        m
                    },
                    None => HashMap::new(),
                };

                self.layouts.insert(name.to_string(), Layout::new(
                    inputs,
                    eval_snippet(&packs, layout.get("render"), &self.source).unwrap(),
                    self.source.clone()
                ));
            }
        }
        /* LAYOUTS */

        // TODO: Global data

        /* SCRIPTS */
        let packs = match hub_data.get("imports") {
            Some(i) => {
                let mut p = Vec::new();
                for (ref k, ref v) in i.as_object().unwrap().iter() {
                    let value = v.as_str().unwrap();
                    p.push((k.to_string(), value.to_string()));
                }
                p
            },
            None => Vec::new(),
        };

        // TODO: check init exists
        // TODO: check end exists
        self.init = eval_snippet(&packs, hub_data.get("init"), &self.source).unwrap();
        self.end = eval_snippet(&packs, hub_data.get("end"), &self.source).unwrap();
        self.tick = eval_snippet(&packs, hub_data.get("tick"), &self.source).unwrap();
        /* SCRIPTS */

        Ok(())
    }

    pub fn run_input(&self, /*current_layout: &str, */key: char) -> ExprRes {
        self.layouts.get(&self.current_layout).expect("Unrecognised layout.").run_input(key)
    }

    pub fn init(&self) -> ExprRes {
        self.init.call(&self.source, &[])
    }

    pub fn tick(&self) -> ExprRes {
        self.tick.call(&self.source, &[])
    }

    pub fn end(&self) -> ExprRes {
        self.end.call(&self.source, &[])
    }
}

// LEVEL
impl Global {
    pub fn create_level(&mut self, name: &str) -> ExprRes {
        let level = self.levels.get(name).unwrap().clone();
        let mut instance = LevelInst::new(level);
        instance.init()?;
        self.id_count += 1; // TODO (?): more robust id generation
        self.level_instances.insert(self.id_count, instance);
        Ok(msValue::Val(VType::I(self.id_count as i64)))
    }

    pub fn delete_level(&mut self, id: i64) -> ExprRes {
        self.level_instances.remove(&(id as u64));
        Ok(msValue::Null)
    }

    pub fn set_active_level(&mut self, id: i64) -> ExprRes {
        self.active_level = id as u64;
        Ok(msValue::Null)
    }

    pub fn clone_level(&mut self, id: i64) -> ExprRes {
        self.id_count += 1;
        let instance = self.level_instances.get(&(id as u64)).unwrap().clone();
        self.level_instances.insert(self.id_count, instance);
        Ok(msValue::Val(VType::I(self.id_count as i64)))
    }

    pub fn level_obj(&self) -> ExprRes {
        Ok(self.level_instances.get(&self.active_level).unwrap().get_data())
    }
}

// ENTITY
impl Global {
    pub fn create_glob_entity(&mut self, name: &str) -> ExprRes {
        let entity = self.entities.get(name).unwrap().clone();
        let instance = EntityInst::new(entity)?;
        self.id_count += 1; // TODO (?): more robust id generation
        self.glob_instances.insert(self.id_count, instance);
        Ok(msValue::Val(VType::I(self.id_count as i64)))
    }

    pub fn create_local_entity(&mut self, name: &str) -> ExprRes {
        let entity = self.entities.get(name).unwrap().clone();
        let instance = EntityInst::new(entity)?;
        self.id_count += 1; // TODO (?): more robust id generation
        self.level_instances.get_mut(&self.active_level).unwrap()
            .add_instance(self.id_count, instance);
        Ok(msValue::Val(VType::I(self.id_count as i64)))
    }

    pub fn delete_entity(&mut self, id: i64) -> ExprRes {
        self.glob_instances.remove(&(id as u64));
        self.level_instances.get_mut(&self.active_level).unwrap()
            .remove_instance(id as u64);
        Ok(msValue::Null)
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

fn eval_snippet(imports: &[(String, String)], script: Option<&jsonValue>, libs: &FuncMap) -> Result<Callable, modscript::Error> {
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
