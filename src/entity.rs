use modscript::{Callable, Value, FuncMap, ExprRes, Error};

use std::rc::Rc;

pub struct Entity {
    name: String,
    init: Callable,
    pre_action: Callable,
    action: Callable,
    post_action: Callable,
    delete: Callable,
    key: char,
    source: Rc<FuncMap>,
}

#[derive(Clone)]
pub struct EntityInst {
    pub id: u64,
    entity: Rc<Entity>,
    fields: Value, // Obj
}

impl Entity {
    pub fn new(name: &str, key: char,
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
            key: key,
            init: init,
            pre_action: pre_action,
            action: action,
            post_action: post_action,
            delete: delete,
            source: source,
        }
    }

    /*pub fn create_instance(&self, id: u64/*, inst_store: &InstanceStore*/) -> Result<EntityInst, String> {
        let fields = self.init.call(&self.source, &[])?;
        Ok(EntityInst {
            entity: Rc::clone(self),
            id: id,
            fields: fields,
        })
    }*/
}

impl EntityInst {
    pub fn new(entity: Rc<Entity>, id: u64) -> Result<Self, Error> {
        let fields = entity.init.call(&entity.source, &[])?;
        Ok(EntityInst {
            entity: entity.clone(),
            id: id,
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

    pub fn get_fields(&self) -> Value {
        self.fields.clone()
    }
}

impl Drop for EntityInst {
    fn drop(&mut self) {
        self.entity.delete.call(&self.entity.source, &[self.fields.clone()]).unwrap();
    }
}
