use std::{collections::HashMap, io, io::Write};
fn get_in(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().expect("Failed to flush stdout!");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}
fn get_num(msg: &str, max_num: u8) -> u8 {
    let num = loop {
        let input = get_in(msg);
        match input.trim().parse() {
            Ok(n) if n <= max_num && n > 0 => break n,
            Ok(n) => {
                println!("{} is not a valid Option!", n);
            }
            Err(_) => {
                println!("Enter a valid Number!");
            }
        }
    };
    num
}
pub struct Company {
    pub employees: HashMap<String, String>,
}
impl Company {
    pub fn new() -> Company {
        Company {
            employees: HashMap::new(),
        }
    }
    pub fn add(&mut self, employee: &str, dep: &str) {
        self.employees.insert(employee.to_string(), dep.to_string());
    }
    pub fn remove_employee(&mut self, employee: &str) {
        self.employees.remove::<str>(employee);
    }
    pub fn list_deps(&self) -> Vec<String> {
        let mut deps: Vec<String> = Vec::new();
        for (_, dep) in self.employees.iter() {
            let mut exists = false;
            for i in &deps {
                if i == dep {
                    exists = true;
                }
            }
            if !exists {
                deps.push(dep.clone());
            }
        }
        deps
    }
    pub fn list_employees_by_dep(&self) -> Vec<(String, String)> {
        let mut deps: Vec<String> = Vec::new();
        for (_, dep) in self.employees.iter() {
            let mut exists = false;
            for i in &deps {
                if i == dep {
                    exists = true;
                }
            }
            if !exists {
                deps.push(dep.clone());
            }
        }
        deps.sort();
        let mut rv: Vec<(String, String)> = Vec::new();
        for i in &deps {
            let mut employees: Vec<String> = Vec::new();
            for (employee, dep) in self.employees.iter() {
                if i == dep {
                    employees.push(employee.clone());
                }
            }
            employees.sort();
            for s in employees {
                rv.push((i.clone(), s));
            }
        }
        rv
    }
    pub fn list_employees(&self) -> Vec<String> {
        let mut employees: Vec<String> = Vec::new();
        for (i, _) in self.employees.iter() {
            employees.push(i.clone());
        }
        employees.sort();
        employees
    }
    pub fn cli(&mut self) {
        loop {
            let msg = format!(
                "\n1. Add Employee to Department\n2. Remove Employee\n3. List Each Department with It's employees (A-Z)\n4. List All Employees (A-Z)\n>>> "
            );
            match get_num(&msg, 4) {
                1 => {
                    let mut employee = get_in("\nEnter the Name of the Employee: ");
                    employee.pop();
                    match self.employees.get(&employee) {
                        None => (),
                        Some(n) => {
                            print!(
                                "The Employee '{}' is currently in the '{}' Department, do you want to overwrite that value? (Y/n)",
                                employee, n
                            );
                            // I left the following line, in case "Yes" will become the non-default choice.
                            //let _true_vals: [&'static str; 11] = ["y", "Y", "YES", "yes", "Yes", "YEs", "yEs", "yES", "Ye", "YE","yE",];
                            let false_vals: [&'static str; 6] = ["no", "NO", "No", "nO", "n", "N"];
                            let input: String = get_in(" ");
                            let input = input.trim();
                            if false_vals.contains(&&*input) {
                                continue;
                            }
                        }
                    }
                    let dep_name = get_in("Enter the Name of the Department: ");
                    let dep_name = dep_name.trim();
                    if dep_name.len() == 0 {
                        println!("Department Name can't be Empty");
                    } else {
                        self.add(&employee, &dep_name);
                    }
                }
                2 => {
                    let mut employee = get_in("\nEnter the Name of the Employee: ");
                    employee.pop();
                    match self.employees.get(&employee) {
                        None => {
                            println!("Employee '{}' doesn't exist!", employee);
                            break;
                        }
                        _ => {
                            self.remove_employee(&employee);
                        }
                    }
                }
                3 => {
                    let mut lastdep: String = String::new();
                    for (i, c) in self.list_employees_by_dep() {
                        if i != lastdep {
                            println!("\n'{}' Department: ", i);
                            lastdep = i.clone();
                        }
                        println!("\tEmployee: '{}'", c);
                    }
                }
                4 => {
                    println!("\nAll Employees (A-Z):\n");
                    for i in self.list_employees() {
                        println!("'{}'", i);
                    }
                }
                _ => (),
            }
        }
    }
}
