use script::ast::expr::ExprSignal;
use std::collections::BTreeMap;

pub enum Package {
    Ext(&str),
    Script(&str),
}

type ExtFunc: fn(&str, &[Value], &mut GlobalState) -> ExprSignal;

type ScriptFunc: fn(&[Value], &mut GlobalState, &FuncMap) -> ExprSignal;

pub struct FuncMap {
    ext_funcs: BTreeMap<String, ExtFunc>,
    script_funcs: BTreeMap<String, BTreeMap<String, ScriptFunc>>,
}

impl FuncMap {
    pub fn new() -> Self {
        FuncMap {
            ext_funcs: BTreeMap::new(),
            script_funcs: BTreeMap::new(),
        }
    }

    pub fn attach_ext_package(&mut self, package_name: &str, package: ExtFunc) {
        ext_funcs.insert(package_name.to_string(), package);
    }

    pub fn attach_script_package(&mut self, package_name: &str, package: BTreeMap<String, ScriptFunc>) {
        script_funcs.insert(package_name.to_string(), package);
    }

    pub fn call_fn(&self, package: Package, name: &str, args: &[Value], data: &mut GlobalState) -> ExprSignal {
        match package {
            Ext(n) => match self.ext_funcs.get(n) {
                Some(p) => p(name, args, data),
                None => ExprSignal::Error("Couldn't find extension package.".to_string()),
            },
            Script(n) => match self.script_funcs.get(n) {
                Some(p) => match p.get(name) {
                    Some(f) => f(args, data, self),
                    None => ExprSignal::Error("Couldn't find script function.".to_string()),
                },
                None => ExprSignal::Error("Couldn't find script package.".to_string()),
            },
        }
    }
}
