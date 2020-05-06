#[derive(Debug)]
struct Employee {
    name: String,
    age: u8
}

fn employee_schedule(employees: Vec<Employee> ,day_of_week: u8) -> Vec<Employee> {
    let mut result = Vec::new();

    for employee in employees {
        if day_of_week == 6 {
            if employee.age >= 18 {
                result.push(employee)
            }
        } else {
            result.push(employee)
        }
    }

    return result;
}

fn employees_older_than_18(employees: Vec<Employee>) -> Vec<Employee> {
    let mut result = Vec::new();

    for employee in employees {
        if employee.age >= 18 {
            result.push(employee)
        }
    }

    return result;
}

#[test]
fn employees_on_sunday_must_be_older_than_18() {
    let employees = vec![
        Employee{name: String::from("Max"), age: 17},
        Employee{name: String::from("Sam"), age: 18}
    ];
    let result = employee_schedule(employees, 6);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "Sam");
}

#[test]
fn employees_on_monday_can_be_anyone() {
    let employees = vec![
        Employee{name: String::from("Max"), age: 17},
        Employee{name: String::from("Sam"), age: 18}
    ];
    let result = employee_schedule(employees, 0);
    assert_eq!(result.len(), 2);
}

#[test]
fn get_only_18_employees() {
    let employees = vec![
        Employee{name: String::from("Max"), age: 17},
        Employee{name: String::from("Sam"), age: 18}
    ];
    let result = employees_older_than_18(employees);
    assert_eq!(result.len(), 1)
}

