use std::collections::HashMap;
use std::io;

fn main() {
    println!("Welcome!");
    println!("Please feel free to:");
    println!("1. Add an employee - For example \"Add Amit to Sales\"");
    println!("2. List all employees in a department - For example \"Show Sales\"");
    println!("3. List all employees in the company - Using the command \"Show Company\"");
    println!("Invalid queries will be ignored.");

    let mut employees = HashMap::new();
    loop {
        println!("\nPlease enter a query\n");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let query = guess
            .split(&[' ', '\n'][..])
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        match query.len() {
            3 => {
                if query[0] == "Show" && is_valid(&query[1]) {
                    if query[1] == "Company" {
                        show_company(&employees);
                    } else {
                        show_department(&employees, &query[1]);
                    }
                } else {
                    println!("\nInvalid query!");
                    continue;
                }
            }
            5 => {
                if query[0] == "Add"
                    && is_valid(&query[1])
                    && query[2] == "to"
                    && is_valid(&query[3])
                {
                    let department_employees =
                        employees.entry(query[3].clone()).or_insert(Vec::new());
                    department_employees.push(query[1].clone());
                    department_employees.sort();

                    println!(
                        "\n{} has been added to the {} department",
                        query[1], query[3]
                    )
                } else {
                    println!("\nInvalid query!!!!");
                    continue;
                }
            }
            _ => {
                println!("\nInvalid query!!!!!!!!!!!");
                continue;
            }
        }
    }
}

fn is_valid(s: &str) -> bool {
    ('A'..='Z').contains(&s.chars().nth(0).unwrap())
        && s.chars().skip(1).all(|c| ('a'..='z').contains(&c))
}

fn show_company(employees: &HashMap<String, Vec<String>>) {
    for (department, department_employees) in employees {
        println!("\n{} Department:", department);
        for employee in department_employees {
            println!("{}", employee);
        }
    }
}

fn show_department(employees: &HashMap<String, Vec<String>>, department: &str) {
    if let Some(department_employees) = employees.get(department) {
        println!("\n{} Department:", department);
        for employee in department_employees {
            println!("{}", employee);
        }
    } else {
        println!("Your company does not have a {} department!", department);
    }
}
