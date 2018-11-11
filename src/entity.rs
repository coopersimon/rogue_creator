use modscript::{ScriptExpr, Value, FuncMap, ExprRes, Error};

use textitem::TextItem;

use std::rc::Rc;

pub struct Entity {
    //name: String,   // TODO: way of getting this out
    init: ScriptExpr,
    action: ScriptExpr,
    post_action: ScriptExpr,
    delete: ScriptExpr,
    tile: TextItem,
}

#[derive(Clone)]
pub struct EntityInst {
    name: String,
    tile: TextItem,
    fields: Value, // Obj
}

impl Entity {
    pub fn new(/*name: &str, */tile: &str,
        init: ScriptExpr,
        action: ScriptExpr,
        post_action: ScriptExpr,
        delete: ScriptExpr,
        ) -> Self
    {
        Entity {
            //name: name.to_string(),
            tile: TextItem::new_tile(tile.to_string()),
            init: init,
            action: action,
            post_action: post_action,
            delete: delete,
        }
    }

    pub fn new_instance(&self, name: &str) -> EntityInst {
        EntityInst {
            name: name.to_string(),
            tile: self.tile.clone(),
            fields: Value::Null,
        }
    }

    pub fn init(&self, source: &FuncMap) -> ExprRes {
        self.init.run(source)
    }

    pub fn action(&self, source: &FuncMap) -> ExprRes {
        self.action.run(source)
    }

    pub fn post_action(&self, source: &FuncMap) -> ExprRes {
        self.post_action.run(source)
    }

    pub fn delete(&self, source: &FuncMap) -> ExprRes {
        self.delete.run(source)
    }
}

impl EntityInst {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_data(&mut self, data: Value) {
        self.fields = data;
    }

    pub fn get_data(&self) -> Value {
        self.fields.clone()
    }

    pub fn get_tile(&self) -> TextItem {
        self.tile.clone()
    }
}
