use modscript::{Callable, Value, FuncMap, ExprRes, Error};

use textitem::TextItem;

use std::rc::Rc;

pub struct Entity {
    name: String,
    init: Callable,
    pre_action: Callable,
    action: Callable,
    post_action: Callable,
    delete: Callable,
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
        init: Callable,
        pre_action: Callable,
        action: Callable,
        post_action: Callable,
        delete: Callable,
        source: Rc<FuncMap>
        ) -> Self
    {
        Entity {
            name: name.to_string(),
            tile: TextItem::new_tile(tile.to_string()),
            init: init,
            pre_action: pre_action,
            action: action,
            post_action: post_action,
            delete: delete,
            source: source,
        }
    }
}

impl EntityInst {
    pub fn new(entity: Rc<Entity>) -> Result<Self, Error> {
        let fields = entity.init.call(&entity.source, &[])?;
        Ok(EntityInst {
            entity: entity.clone(),
            fields: fields,
        })
    }

    pub fn set_action(&mut self, new_action: Value) {
        // Move action into entity?
        //self.action.set_value(new_action);
    }

    pub fn call_action(&mut self) -> ExprRes {
        // TODO: pass output of pre into action
        // TODO: this might require allowing functions to accept variable amounts of arguments (modscript change)
        self.entity.pre_action.call(&self.entity.source, &[self.fields.clone()])?;
        self.entity.action.call(&self.entity.source, &[self.fields.clone()])?;
        self.entity.post_action.call(&self.entity.source, &[self.fields.clone()])
    }

    pub fn get_data(&self) -> Value {
        self.fields.clone()
    }

    pub fn get_tile(&self) -> TextItem {
        self.entity.tile.clone()
    }
}

impl Drop for EntityInst {
    fn drop(&mut self) {
        self.entity.delete.call(&self.entity.source, &[self.fields.clone()]).unwrap();
    }
}
