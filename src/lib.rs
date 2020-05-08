#![allow(dead_code)]

#[derive(Debug)]
struct Employee {
    name: String,
    age: u8,
}

enum Sorting {
    Ascending,
    Descending
}

enum DayOfTheWeek {
    Monday,
    Sunday
}

#[derive(Clone,Copy)]
enum Age {
    OlderThan18 = 18,
    Any = 0
}

fn employee_schedule(employees: Vec<Employee>, day_of_week: DayOfTheWeek) -> Vec<Employee> {
    match day_of_week {
        DayOfTheWeek::Sunday => employees_older_than(employees, Age::OlderThan18),
        DayOfTheWeek::Monday => employees_older_than(employees, Age::Any),
    }
}

fn employees_older_than(employees: Vec<Employee>, age: Age) -> Vec<Employee> {
    return employees_older_than_sorted(employees, age, Sorting::Ascending);
}

fn employees_older_than_sorted(employees: Vec<Employee>, age: Age, sort: Sorting) -> Vec<Employee> {
    let mut result: Vec<Employee> = employees
        .iter()
        .filter(|e| e.age >= age as u8)
        .map(|e| {
            capitalize_name(Employee {
                name: e.name.to_string(),
                age: e.age,
            })
        })
        .collect();
    match sort {
        Sorting::Ascending => result.sort_by(|a, b| a.name.cmp(&b.name)),
        Sorting::Descending => result.sort_by(|a, b| b.name.cmp(&a.name)),
    }
    result
}

fn capitalize_name(mut employee: Employee) -> Employee {
    let mut capitalized = String::new();
    let mut word_start = true;
    for c in employee.name.chars() {
        match c {
            ' ' => {
                capitalized.push(c);
                word_start = true
            }
            _ => {
                if word_start {
                    capitalized.push(c.to_ascii_uppercase());
                } else {
                    capitalized.push(c.to_ascii_lowercase());
                }
                word_start = false;
            }
        };
    }
    employee.name = capitalized;
    return employee;
}

#[test]
fn employees_on_sunday_must_be_older_than_18() {
    let employees = vec![
        Employee {
            name: String::from("Max"),
            age: 17,
        },
        Employee {
            name: String::from("Sam"),
            age: 18,
        },
    ];
    let result = employee_schedule(employees, DayOfTheWeek::Sunday);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "Sam");
}

#[test]
fn employees_on_monday_can_be_anyone() {
    let employees = vec![
        Employee {
            name: String::from("Max"),
            age: 17,
        },
        Employee {
            name: String::from("Sam"),
            age: 18,
        },
    ];
    let result = employee_schedule(employees, DayOfTheWeek::Monday);
    assert_eq!(result.len(), 2);
}

#[test]
fn get_only_18_employees() {
    let employees = vec![
        Employee {
            name: String::from("Max"),
            age: 17,
        },
        Employee {
            name: String::from("Sam"),
            age: 18,
        },
    ];
    let result = employees_older_than(employees, Age::OlderThan18);
    assert_eq!(result.len(), 1)
}

#[test]
fn get_employees_sorted() {
    let employees = vec![
        Employee {
            name: String::from("Sam"),
            age: 18,
        },
        Employee {
            name: String::from("Max"),
            age: 18,
        },
    ];
    let result = employees_older_than_sorted(employees, Age::OlderThan18, Sorting::Ascending);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].name, "Max");
    assert_eq!(result[1].name, "Sam")
}

#[test]
fn get_employees_capitalized() {
    let employees = vec![Employee {
        name: String::from("john doe"),
        age: 18,
    }];
    let result = employees_older_than(employees, Age::OlderThan18);
    assert_eq!(result[0].name, "John Doe");
}

#[test]
fn get_employees_sorted_desc() {
    let employees = vec![
        Employee {
            name: String::from("Max"),
            age: 18,
        },
        Employee {
            name: String::from("Sam"),
            age: 18,
        },
    ];
    let result = employees_older_than_sorted(employees, Age::OlderThan18, Sorting::Descending);
    assert_eq!(result[0].name, "Sam");
}
