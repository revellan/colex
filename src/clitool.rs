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
        let mut company: HashMap<&str, &str> = HashMap::new();
        company.contains_key::<str>(&self.name);
        company.insert(&self.name, dep);
    }
    pub fn list_dep(self) -> Vec<String> {
        let deps: Vec<String> = Vec::new();
        deps
    }
}
