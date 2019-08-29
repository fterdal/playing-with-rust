// use std::io;

fn main() {
    let fred_the_duck: Duck = Duck {
        id: 1,
        name: "Fred".to_string(),
        age: 12,
        location: Location {
            lat: 234.456456,
            long: 34.765511
        }
    };
    let dorothy_the_duck: Duck = Duck {
        id: 2,
        name: "Dorothy".to_string(),
        age: 8,
        location: Location {
            lat: 36.234566,
            long: 12.765785
        }
    };
    let initial_duck_pond = DuckPond { ducks: Vec::new() };
    let after_one = get_new_duck_pond(&initial_duck_pond, fred_the_duck);
    let after_two = get_new_duck_pond(&after_one, dorothy_the_duck);
    println!("Duck Pond - Initial: {:#?}", initial_duck_pond);
    println!("Duck Pond - Fred: {:#?}", after_one);
    println!("Duck Pond - Dorothy: {:#?}", after_two);
}

fn get_new_duck_pond(duck_pond: &DuckPond, duck: Duck) -> DuckPond {
    let mut new_duck_pond = duck_pond.clone();
    new_duck_pond.ducks.push(duck);
    new_duck_pond
}

#[derive(Debug)]
#[derive(Clone)]
struct DuckPond {
    ducks: Vec<Duck>
}

#[derive(Debug)]
#[derive(Clone)]
struct Location {
    lat: f64,
    long: f64,
}

#[derive(Debug)]
#[derive(Clone)]
struct Duck {
    id: u32,
    name: String,
    age: u32,
    location: Location
}
