use std::io;

#[derive(Debug)]
struct Employee {
    name: String,
    id: i32,
    email: String,
    age: i32,
    phone_number: String,
}

impl Employee {
    fn new(name: String, id: i32, email: String, age: i32, phone_number: String) -> Self {
        Employee {
            name,
            id,
            email,
            age,
            phone_number,
        }
    }
}

fn get_employee_by_id(employees: &[Employee], id: i32) -> Option<&Employee> {
    employees.iter().find(|employee| employee.id == id)
}

fn get_employees_by_age(employees: &[Employee], age: i32) -> Vec<&Employee> {
    employees.iter().filter(|employee| employee.age == age).collect()
}

fn main() {
    let mut employees = Vec::new();

    loop {
        println!("Enter employee name: ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");

        println!("Enter employee id: ");
        let mut id_input = String::new();
        io::stdin().read_line(&mut id_input).expect("Failed to read line");
        let id: i32 = id_input.trim().parse().expect("Invalid input");

        println!("Enter employee email: ");
        let mut email = String::new();
        io::stdin().read_line(&mut email).expect("Failed to read line");

        println!("Enter employee age: ");
        let mut age_input = String::new();
        io::stdin().read_line(&mut age_input).expect("Failed to read line");
        let age: i32 = age_input.trim().parse().expect("Invalid input");

        println!("Enter employee phone number: ");
        let mut phone_number = String::new();
        io::stdin().read_line(&mut phone_number).expect("Failed to read line");

        let employee = Employee::new(name, id, email, age, phone_number);
        employees.push(employee);

        println!("Do you want to enter another employee? (y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() != "y" {
            break;
        }
    }

    println!("Enter an employee ID to retrieve details:");
    let mut id_input = String::new();
    io::stdin().read_line(&mut id_input).expect("Failed to read line");
    let id: i32 = id_input.trim().parse().expect("Invalid input");

    if let Some(employee) = get_employee_by_id(&employees, id) {
        println!("Employee details:");
        println!("Name: {}", employee.name);
        println!("ID: {}", employee.id);
        println!("Email: {}", employee.email);
        println!("Age: {}", employee.age);
        println!("Phone number: {}", employee.phone_number);
    } else {
        println!("No employee found with ID {}", id);
    }

    println!("Enter an age to retrieve employees with that age:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read line");
    let age: i32 = age_input.trim().parse().expect("Invalid input");

    let employees_by_age = get_employees_by_age(&employees, age);
    if !employees_by_age.is_empty() {
        println!("Employees with age {}:", age);
        for employee in employees_by_age {
            println!("Name: {}", employee.name);
            println!("ID: {}", employee.id);
            println!("Email: {}", employee.email);
            println!("Age: {}", employee.age);
            println!("Phone number: {}", employee.phone_number);
            println!("---");
        }
    } else {
        println!("No employees found with age {}", age);
    }
}
