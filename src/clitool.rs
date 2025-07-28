use std::collections::HashMap;
pub struct Employee {
    pub name: String,
}
impl Employee {
    pub fn new(name: &str) -> Employee {
        Employee {
            name: String::from(name),
        }
    }
    pub fn add(self, dep: &str) {
        let mut company: HashMap<&str, Vec<String>> = HashMap::new();
        let mut deps: Vec<String>;
        match company.remove::<str>(&self.name) {
            Some(v) => deps = v,
            None => deps = Vec::new(),
        }
        deps.push(dep.to_string());
        company.insert(&self.name, deps);
    }
    pub fn list_dep(self) -> Vec<String> {
        let deps: Vec<String> = Vec::new();

        deps
    }
}
