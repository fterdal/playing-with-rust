mod duck_reducer;
use duck_reducer::{Duck, DuckPond, Store};

fn main() {
    // let fred = Duck::new(1, "Fred".to_string(), 12, 12.444, -32.555);
    let dorothy = Duck::new(2, "Dorothy".to_string(), 8, 36.234566, -12.765785);
    // let initial_duck_pond = DuckPond::new();
    // println!("Initial Duck Pond: {:#?}", initial_duck_pond);
    // let pond_with_ducks = initial_duck_pond
    //     .add_duck(fred);
        // .add_duck(dorothy);
    // println!("Add Fred and Dorothy to Duck Pond: {:#?}", pond_with_ducks);
    // println!("Initial Duck Pond (unmutated): {:#?}", initial_duck_pond);


    // After dispatching Dorothy, the store still seems empty. Why is this?
    // ANSWER: Because add_duck returns a new DuckPond, duh!!!
    let mut store = Store::new();
    // println!("{:#?}", store);
    // store.get_state();
    store.dispatch(dorothy);
    // store.get_state();
    println!("store get state: {:#?}", store.get_state());
    println!("store: {:#?}", store);
}
