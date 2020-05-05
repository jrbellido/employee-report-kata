#[derive(Debug)]
struct Employee {
    name: String,
    age: u8
}

fn get_employees() -> Vec<Employee>{
    let employee_list = vec![
        Employee{name: String::from("Max"), age: 17},
        Employee{name: String::from("Sepp"), age: 18},
        Employee{name: String::from("Nina"), age: 15},
        Employee{name: String::from("Mike"), age: 51}
    ];

    employee_list
}

fn employee_schedule(day_of_week: u8) -> Vec<Employee> {
    let mut result = Vec::new();

    for employee in get_employees() {
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

#[test]
fn employees_on_sunday_must_be_2() {
    let result = employee_schedule( 6);
    assert!(result.len() == 2);
}

#[test]
fn first_employee_on_sunday_must_be_sepp() {
    let result = employee_schedule( 6);
    assert!(result[0].name == "Sepp")
}

#[test]
fn first_employee_on_monday_must_be_max() {
    let result = employee_schedule( 0);
    assert!(result[0].name == "Max")
}

