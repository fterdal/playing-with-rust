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
    // let pond: DuckPond = DuckPond { ducks: Vec::new() };
    // pond.ducks.push(fred_the_duck);
    // pond.ducks.push(dorothy_the_duck);
    // println!("Duck Pond: {:#?}", pond);
    let initial_duck_pond = DuckPond { ducks: Vec::new() };
    let after_one = get_new_duck_pond(&initial_duck_pond, fred_the_duck);
    // let after_two = get_new_duck_pond(after_one, dorothy_the_duck);
    println!("Initial Duck Pond: {:#?}", initial_duck_pond);
    println!("AFter Duck Pond: {:#?}", after_one);
    // println!("Duck Pond: {:#?}", after_two);
    // println!("Duck Pond: {:#?}", get_new_duck_pond(dorothy_the_duck));
    // println!("Fred the Duck: {:#?}", fred_the_duck);
    // println!("Dorothy the Duck: {:#?}", dorothy_the_duck);
}

fn get_new_duck_pond(duck_pond: &DuckPond, duck: Duck) -> DuckPond {
    let mut new_duck_pond = duck_pond.clone();
    new_duck_pond.ducks.push(duck);
    // println!("New Duck Pond: {:#?}", new_duck_pond);
    new_duck_pond
}

#[derive(Debug)]
#[derive(Clone)]
struct DuckPond {
    ducks: Vec<Duck>
}

// impl DuckPond {
//     fn clone(&self) -> DuckPond {
//         DuckPond { ducks: self.ducks.clone() }
//     }
// }

#[derive(Debug)]
#[derive(Clone)]
struct Location {
    lat: f64,
    long: f64,
}
// impl Location {
//     fn clone(&self) -> Location {
//         Location {
//             lat: self.lat.clone(),
//             long: self.long.clone(),
//         }
//     }
// }

#[derive(Debug)]
#[derive(Clone)]
struct Duck {
    id: u32,
    name: String,
    age: u32,
    location: Location
}
// impl Duck {
//     fn clone(&self) -> Duck {
//         Duck {
//             id: self.id,
//             name: self.name.clone(),
//             age: self.age,
//             location: self.location.clone(),
//         }
//     }
// }
