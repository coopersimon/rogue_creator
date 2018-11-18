// Global info for the game and JSON parsing
use modscript;
use modscript::Value as msValue;
use serde_json;
use serde_json::Value as jsonValue;
use modscript::{ScriptExpr, FuncMap, expr_from_text, ExprRes, VType, mserr};
use pancurses::Input;

use super::entity::{Entity, EntityInst};
use super::level::{Level, LevelInst};
use super::layout::Layout;
use super::tile::{TileItem, TileInfo};
use super::error::{engerr, EngErr, RunCode};
use Coord;

use std::collections::HashMap;
use std::rc::Rc;
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::mpsc::Sender;

pub struct Global {
    // Code
    pub source: FuncMap,

    // Main functions
    init: ScriptExpr,
    tick: ScriptExpr,
    end: ScriptExpr,

    // Entity and level snippets
    entities: HashMap<String, Entity>,
    levels: HashMap<String, Level>,

    // Layout
    layouts: HashMap<String, Layout>,

    // Database
    // glob_data
}

impl Global {
    pub fn new() -> Self {
        Global {
            source: FuncMap::new(),

            init: ScriptExpr::new(None),
            tick: ScriptExpr::new(None),
            end: ScriptExpr::new(None),

            entities: HashMap::new(),
            levels: HashMap::new(),
            layouts: HashMap::new(),
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
            self.source.attach_package(&(root_dir.to_owned() + package_name), package.call_ref());
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
                        p.push((value.to_string(), root_dir.to_owned() + k));
                    }
                    p
                },
                None => Vec::new(),
            };

            for (ref name, ref ent) in entity_data["entities"].as_object().unwrap().iter() {
                self.entities.insert(name.to_string(), Entity::new(
                    /*&name,*/
                    ent["tile"].as_str().unwrap(),
                    eval_snippet(&packs, ent.get("init"), &self.source).unwrap(),
                    eval_snippet(&packs, ent.get("action"), &self.source).unwrap(),
                    eval_snippet(&packs, ent.get("post_action"), &self.source).unwrap(),
                    eval_snippet(&packs, ent.get("delete"), &self.source).unwrap()
                ));
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
                        p.push((value.to_string(), root_dir.to_owned() + k));
                    }
                    p
                },
                None => Vec::new(),
            };

            let tile = level_data["tiles"].as_object().unwrap();
            let mut tile_info = TileInfo::new();
            for (ref name, ref t) in tile["collide"].as_object().unwrap().iter() {
                let i = TileItem::new(t.as_str().unwrap().to_string(), true);
                tile_info.add_tile(name, i);
            }
            for (ref name, ref t) in tile["nocollide"].as_object().unwrap().iter() {
                let i = TileItem::new(t.as_str().unwrap().to_string(), false);
                tile_info.add_tile(name, i);
            }
            tile_info.set_default(tile["default"].as_str().unwrap());

            let tile_info = Rc::new(tile_info);

            for (ref name, ref lev) in level_data["levels"].as_object().unwrap().iter() {
                self.levels.insert(name.to_string(), Level::new(
                    lev["x"].as_u64().unwrap(),
                    lev["y"].as_u64().unwrap(),
                    tile_info.clone(),
                    eval_snippet(&packs, lev.get("init"), &self.source).unwrap(),
                    eval_snippet(&packs, lev.get("delete"), &self.source).unwrap()
                ));
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
                        p.push((value.to_string(), root_dir.to_owned() + k));
                    }
                    p
                },
                None => Vec::new(),
            };

            for (ref name, ref layout) in layout_data["layouts"].as_object().unwrap().iter() {
                let mut default = None;
                let inputs = match layout.get("inputs") {
                    Some(i) => {
                        let mut m = HashMap::new();
                        for (ref k, ref v) in i.as_object().unwrap().iter() {
                            let key = match k.as_str() {
                                "default"   => {
                                    default = Some(eval_snippet(&packs, Some(v), &self.source).unwrap());
                                    continue;
                                },
                                "enter"     => Input::Character('\n'),
                                "space"     => Input::Character(' '),
                                "backspace" => Input::KeyBackspace,
                                "tab"       => Input::Character('\t'),
                                "left"      => Input::KeyLeft,
                                "right"     => Input::KeyRight,
                                "up"        => Input::KeyUp,
                                "down"      => Input::KeyDown,
                                ch          => Input::Character(ch.to_string().chars().next().unwrap()),
                            };
                            m.insert(key, eval_snippet(&packs, Some(v), &self.source).unwrap());
                        }
                        m
                    },
                    None => HashMap::new(),
                };

                self.layouts.insert(name.to_string(), Layout::new(
                    inputs,
                    default,
                    eval_snippet(&packs, layout.get("render"), &self.source).unwrap()
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
                    p.push((value.to_string(), root_dir.to_owned() + k));
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

    // Better deal with different inputs
    pub fn run_input(&self, current_layout: &str, key: &Input) -> ExprRes {
        self.layouts.get(current_layout).expect("Unrecognised layout.").run_input(key, &self.source)
    }

    pub fn run_render(&self, current_layout: &str) -> ExprRes {
        self.layouts.get(current_layout).expect("Unrecognised layout.").render(&self.source)
    }

    pub fn init(&self) -> ExprRes {
        self.init.run(&self.source)
    }

    pub fn tick(&self) -> ExprRes {
        self.tick.run(&self.source)
    }

    pub fn end(&self) -> ExprRes {
        self.end.run(&self.source)
    }

    // call fns on these?
    pub fn new_entity_instance(&self, name: &str) -> Result<EntityInst, modscript::Error> {
        match self.entities.get(name) {
            Some(e) => Ok(e.new_instance(name)),
            None    => Err(engerr(EngErr::RunTime(RunCode::EntityClassNotFound))),
        }
    }

    pub fn new_level_instance(&self, name: &str) -> Result<LevelInst, modscript::Error> {
        match self.levels.get(name) {
            Some(l) => Ok(l.new_instance()),
            None    => Err(engerr(EngErr::RunTime(RunCode::LevelClassNotFound))),
        }
    }


    pub fn level_init(&self, name: &str) -> ExprRes {
        match self.levels.get(name) {
            Some(l) => l.init(&self.source),
            None    => Err(engerr(EngErr::RunTime(RunCode::LevelClassNotFound))), // critical?
        }
    }

    pub fn level_delete(&self, name: &str) -> ExprRes {
        match self.levels.get(name) {
            Some(l) => l.delete(&self.source),
            None    => Err(engerr(EngErr::RunTime(RunCode::LevelClassNotFound))), // critical?
        }
    }


    pub fn entity_init(&self, name: &str) -> ExprRes {
        match self.entities.get(name) {
            Some(e) => e.init(&self.source),
            None    => Err(engerr(EngErr::RunTime(RunCode::EntityClassNotFound))), // critical?
        }
    }

    pub fn entity_action(&self, name: &str) -> ExprRes {
        match self.entities.get(name) {
            Some(e) => e.action(&self.source),
            None    => Err(engerr(EngErr::RunTime(RunCode::EntityClassNotFound))), // critical?
        }
    }

    pub fn entity_post_action(&self, name: &str) -> ExprRes {
        match self.entities.get(name) {
            Some(e) => e.post_action(&self.source),
            None    => Err(engerr(EngErr::RunTime(RunCode::EntityClassNotFound))), // critical?
        }
    }

    pub fn entity_delete(&self, name: &str) -> ExprRes {
        match self.entities.get(name) {
            Some(e) => e.delete(&self.source),
            None    => Err(engerr(EngErr::RunTime(RunCode::EntityClassNotFound))), // critical?
        }
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

// TODO: rename this function and reconsider code snippets
fn eval_snippet(imports: &[(String, String)], script: Option<&jsonValue>, libs: &FuncMap) -> Result<ScriptExpr, modscript::Error> {
    match script {
        Some(s) => {
            let script_str = s.as_str().unwrap();
            let expr = expr_from_text(imports, script_str)?;
            Ok(expr)
        },
        None => Ok(ScriptExpr::new(None)),
    }
}
