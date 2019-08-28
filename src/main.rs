// use std::io;

fn main() {
    let fred_the_duck: Duck = Duck {
        id: 5,
        name: "Fred".to_string(),
        age: 12,
        location: Location { lat: 234.456456, long: 34.7655 }
    };
    println!("Fred the Duck: {:#?}", fred_the_duck);
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

/*
RustDucks Store:
{
    all_ducks: [Duck]
    selected_duck: Duck
}

Duck
{
    id: u32,
    name: String,
    age: u32,
    location: {
        lat: f63,
        long: f64,
    }
}
*/

// struct SetDucksAction {
//     all_ducks:
// }

// enum ActionType {
//     SetDucksAction,
//     ClearDucksAction
// }

// impl SetDucksAction {
//     const action_type: String =  "SET_DUCKS";
// }
