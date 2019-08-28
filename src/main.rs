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
    let mut pond: DuckPond = DuckPond { ducks: Vec::new() };
    pond.ducks.push(fred_the_duck);
    pond.ducks.push(dorothy_the_duck);
    println!("Duck Pond: {:#?}", pond);
    // println!("Fred the Duck: {:#?}", fred_the_duck);
    // println!("Dorothy the Duck: {:#?}", dorothy_the_duck);
}

#[derive(Debug)]
struct DuckPond {
    ducks: Vec<Duck>
}

#[derive(Debug)]
struct Location {
    lat: f64,
    long: f64,
}

#[derive(Debug)]
struct Duck {
    id: u32,
    name: String,
    age: u32,
    location: Location
}

