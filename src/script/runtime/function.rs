use super::{Value, GlobState, ExprSig};
use std::collections::BTreeMap;

// not very nice
//use script::ast::FuncRoot;

/*pub enum Package {
    Ext(String),
    Script(String),
}*/

//pub type ExtFunc = fn(&str, &[Value], &mut GlobState, &FuncMap) -> ExprSig;
pub type PackageRoot = Box<Fn(&str, &[Value], &mut GlobState, &FuncMap) -> ExprSig>;

//pub type ScriptFunc = fn(&[Value], &mut GlobState, &FuncMap) -> ExprSig;
//pub type ScriptFunc = Box<Fn(&[Value], &mut GlobState, &FuncMap) -> ExprSig>;


pub struct FuncMap {
    //ext_funcs: BTreeMap<String, ExtFunc>,
    //script_funcs: BTreeMap<String, BTreeMap<String, ScriptFunc>>,
    //script_funcs: BTreeMap<String, BTreeMap<String, FuncRoot>>,
    packages: BTreeMap<String, PackageRoot>,
}

impl FuncMap {
    pub fn new() -> Self {
        FuncMap {
            //ext_funcs: BTreeMap::new(),
            //script_funcs: BTreeMap::new(),
            packages: BTreeMap::new(),
        }
    }

    /*pub fn attach_ext_package(&mut self, package_name: &str, package: ExtFunc) {
        self.ext_funcs.insert(package_name.to_string(), package);
    }*/

    pub fn attach_package(&mut self, package_name: &str, package: PackageRoot) {
        self.packages.insert(package_name.to_string(), package);
    }

    //pub fn attach_script_package(&mut self, package_name: &str, package: BTreeMap<String, ScriptFunc>) {
    //pub fn attach_script_package(&mut self, package_name: &str, package: BTreeMap<String, FuncRoot>) {
    //    self.script_funcs.insert(package_name.to_string(), package);
    //}

    //pub fn call_fn(&self, package: &Package, name: &str, args: &[Value], data: &mut GlobState) -> ExprSig {
    pub fn call_fn(&self, package: &str, name: &str, args: &[Value], data: &mut GlobState) -> ExprSig {
        /*match package {
            &Package::Ext(ref n) => match self.ext_funcs.get(n) {
                Some(p) => p(name, args, data, self),
                None => ExprSig::Error("Couldn't find extension package.".to_string()),
            },
            &Package::Script(ref n) => match self.script_funcs.get(n) {
                Some(p) => match p.get(name) {
                    Some(f) => f.call(args, data, self),
                    None => ExprSig::Error("Couldn't find script function.".to_string()),
                },
                None => ExprSig::Error("Couldn't find script package.".to_string()),
            },
        }*/

        match self.packages.get(package) {
            Some(p) => p(name, args, data, self),
            None => ExprSig::Error("Couldn't find package.".to_string()),
        }
    }
}
