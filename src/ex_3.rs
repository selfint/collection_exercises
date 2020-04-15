/*
Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
*/
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

// main interface that interacts with the user and calls other functions, runs in an infinite loop
pub fn run_interface() {
    println!("Running company management interface...");

    // generate company employee hash map
    let mut company_employees: HashMap<String, String> = HashMap::new();
    let mut company_departments: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        // get user action
        let action = get_user_action();

        match action {
            UserAction::AddEmployee => {
                add_employee(&mut company_employees, &mut company_departments)
            }
            UserAction::GetEmployeeInDepartment => {
                retrieve_employees_in_department(&company_departments)
            }
            UserAction::GetAllEmployees => retrieve_all_employees(&company_employees),
            UserAction::NoAction => (),
            UserAction::Exit => {
                return;
            }
        }
    }
}

enum UserAction {
    AddEmployee,
    GetEmployeeInDepartment,
    GetAllEmployees,
    Exit,
    NoAction,
}

fn get_user_action() -> UserAction {
    // returns the index of the user action
    println!(
        "Choose an action:
    1) add employee to deparment
    2) Retrieve list of employees in a deparment
    3) Retrieve list of all employees by department
    4) Exit"
    );

    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("failed to read answer");

    // return answer as a number
    let answer = answer
        .trim()
        .parse()
        .expect("failed to convert answer to number");

    match answer {
        1 => UserAction::AddEmployee,
        2 => UserAction::GetEmployeeInDepartment,
        3 => UserAction::GetAllEmployees,
        4 => UserAction::Exit,
        _ => UserAction::NoAction,
    }
}

fn add_employee(
    company_employees: &mut HashMap<String, String>,
    company_departments: &mut HashMap<String, Vec<String>>,
) {
    // get department
    println!("Enter department name:");
    let mut department = String::new();
    io::stdin()
        .read_line(&mut department)
        .expect("failed to read line");
    let department = process_input(&department);

    // get employee
    println!("Enter employee name:");
    let mut employee = String::new();
    io::stdin()
        .read_line(&mut employee)
        .expect("failed to read line");
    let employee = process_input(&employee);

    // add new employee to department (create the department if necassary)
    company_employees.insert(employee.clone(), department.clone());
    company_departments
        .entry(department)
        .or_insert_with(Vec::new)
        .push(employee);
}

fn retrieve_employees_in_department(company_departments: &HashMap<String, Vec<String>>) {
    // ask the user which department to display
    let mut department = String::new();
    println!("Enter department name:");
    io::stdin()
        .read_line(&mut department)
        .expect("failed to read line");
    let department = process_input(&department);
    let employees = match company_departments.get(&process_input(&department)) {
        Some(dep) => dep,
        None => {
            println!("ERROR: no such department {}", department);
            return;
        }
    };

    // sort the names of all employees in the department using insertion sort
    let mut sorted_employees: Vec<String> = vec![];
    for employee in employees {
        insert_to_sorted(employee, &mut sorted_employees);
    }

    for employee in sorted_employees {
        println!("{:#?}", employee);
    }
}

fn retrieve_all_employees(company_employees: &HashMap<String, String>) {
    // sort the names of all employees in the company using insertion sort
    let mut sorted_employees: Vec<String> = vec![];
    for employee in company_employees.keys() {
        insert_to_sorted(employee, &mut sorted_employees);
    }

    // print sorted employees and their departments
    println!("Employee\tDepartment");
    for employee in &sorted_employees {
        println!(
            "{:#?}\t{:#?}",
            employee,
            company_employees
                .get(employee)
                .expect("employee in sorted employees missing from company employees hash map")
        );
    }
}

fn insert_to_sorted(element: &str, vector: &mut Vec<String>) {
    // assumes the vector is already sorted
    for (index, sorted_element) in vector.iter().enumerate() {
        let comparison = element.cmp(sorted_element);
        match comparison {
            Ordering::Less => {
                vector.insert(index, element.to_string());
                return;
            }
            Ordering::Equal => {
                vector.insert(index, element.to_string());
                return;
            }
            Ordering::Greater => (),
        }
    }

    vector.push(element.to_string());
}

fn process_input(input: &str) -> String {
    input.trim_end().to_string()
}
