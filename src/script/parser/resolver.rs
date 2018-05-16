use std::collections::HashMap;

// used by compiler to resolve references & ids
pub struct Resolver {
    current_package: Option<String>,
    package_refs: HashMap<String, String>,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            current_package: None,
            package_refs: HashMap::new(),
        }
    }

    pub fn set_package(&mut self, package_name: &str) {
        self.current_package = Some(package_name.to_string());
    }

    pub fn add_package_ref(&mut self, package_ref: &str, package_name: &str) {
        self.package_refs.insert(package_ref.to_string(), package_name.to_string());
    }

    pub fn get_package_ref(&self, package_ref: Option<&str>) -> Option<String> {
        match package_ref {
            Some(ref s) => match self.package_refs.get(*s) {
                Some(s) => Some(s.clone()),
                None => None,
            },
            None => self.current_package.clone(),
        }
    }

    pub fn reset_package_refs(&mut self) {
        self.current_package = None;
        self.package_refs.clear();
    }
}
