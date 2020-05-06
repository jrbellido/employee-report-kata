#[derive(Debug)]
struct Employee {
    name: String,
    age: u8
}

const SORT_ASC: u8 =  1;
const SORT_DESC: u8 = 2;

fn employee_schedule(employees: Vec<Employee> ,day_of_week: u8) -> Vec<Employee> {
    if day_of_week == 6 {
        return employees_older_than(employees, 18);
    } else {
        return employees_older_than(employees, 0);
    }
}

fn employees_older_than(employees: Vec<Employee>, age: u8) -> Vec<Employee> {
    return employees_older_than_sorted(employees, age, SORT_ASC);
}

fn employees_older_than_sorted(employees: Vec<Employee>, age: u8, sort: u8) -> Vec<Employee> {
    let mut result = Vec::new();
    for employee in employees {
        if employee.age >= age {
            let emp = capitalize_name(employee);
            result.push(emp)
        }
    }
    result.sort_by(|a, b| a.name.cmp(&b.name));
    if sort == SORT_DESC {
        result.reverse();
    }
    return result;
}

fn capitalize_name(mut employee: Employee) -> Employee {
    let mut capitalized = String::new();
    let mut word_start = true;
    for c in employee.name.chars() {
        match c {
            ' ' => {
                capitalized.push(c);
                word_start = true
            },
            _ => {
                if word_start { 
                    capitalized.push(c.to_ascii_uppercase());
                } else {
                    capitalized.push(c.to_ascii_lowercase());
                };
                word_start = false;
            }
        };
    }
    employee.name = capitalized;
    return employee
}

fn create_name(name: &str) -> String {
    String::from(name)
}

#[test]
fn employees_on_sunday_must_be_older_than_18() {
    let employees = vec![
        Employee{name: create_name("Max"), age: 17},
        Employee{name: create_name("Sam"), age: 18}
    ];
    let result = employee_schedule(employees, 6);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "Sam");
}

#[test]
fn employees_on_monday_can_be_anyone() {
    let employees = vec![
        Employee{name: create_name("Max"), age: 17},
        Employee{name: create_name("Sam"), age: 18}
    ];
    let result = employee_schedule(employees, 0);
    assert_eq!(result.len(), 2);
}

#[test]
fn get_only_18_employees() {
    let employees = vec![
        Employee{name: create_name("Max"), age: 17},
        Employee{name: create_name("Sam"), age: 18}
    ];
    let result = employees_older_than(employees, 18);
    assert_eq!(result.len(), 1)
}

#[test]
fn get_employees_sorted() {
    let employees = vec![
        Employee{name: create_name("Sam"), age: 18},
        Employee{name: create_name("Max"), age: 18}
    ];
    let result = employees_older_than_sorted(employees, 18, SORT_ASC);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].name, "Max");
    assert_eq!(result[1].name, "Sam")
}

#[test]
fn get_employees_capitalized() {
    let employees = vec![
       Employee{name: create_name("john doe AA"), age: 18},
    ];
    let result = employees_older_than(employees, 18);
    assert_eq!(result[0].name, "John Doe Aa");
}

#[test]
fn get_employees_sorted_desc() {
    let employees = vec![
        Employee{name: create_name("Max"), age: 18},
        Employee{name: create_name("Sam"), age: 18}
    ];
    let result = employees_older_than_sorted(employees, 18, SORT_DESC);
    assert_eq!(result[0].name, "Sam");
}

