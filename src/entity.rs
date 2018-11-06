use modscript::{ScriptExpr, Value, FuncMap, ExprRes, Error};

use textitem::TextItem;

use std::rc::Rc;

pub struct Entity {
    name: String,   // TODO: way of getting this out
    init: ScriptExpr,
    action: ScriptExpr,
    post_action: ScriptExpr,
    delete: ScriptExpr,
    tile: TextItem,
    source: Rc<FuncMap>,
}

#[derive(Clone)]
pub struct EntityInst {
    entity: Rc<Entity>,
    fields: Value, // Obj
}

impl Entity {
    pub fn new(name: &str, tile: &str,
        init: ScriptExpr,
        action: ScriptExpr,
        post_action: ScriptExpr,
        delete: ScriptExpr,
        source: Rc<FuncMap>
        ) -> Self
    {
        Entity {
            name: name.to_string(),
            tile: TextItem::new_tile(tile.to_string()),
            init: init,
            action: action,
            post_action: post_action,
            delete: delete,
            source: source,
        }
    }
}

impl EntityInst {
    pub fn new(entity: Rc<Entity>) -> Result<Self, Error> {
        let fields = entity.init.run(&entity.source)?;
        Ok(EntityInst {
            entity: entity.clone(),
            fields: fields,
        })
    }

    pub fn action(&self) -> ExprRes {
        self.entity.action.run(&self.entity.source)
    }

    pub fn post_action(&self) -> ExprRes {
        self.entity.post_action.run(&self.entity.source)
    }

    pub fn delete(&self) -> ExprRes {
        self.entity.delete.run(&self.entity.source)
    }

    pub fn get_data(&self) -> Value {
        self.fields.clone()
    }

    pub fn get_tile(&self) -> TextItem {
        self.entity.tile.clone()
    }
}
