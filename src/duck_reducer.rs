#[derive(Debug)]
pub struct Store {
  state: DuckPond,
}
impl Store {
  pub fn new() -> Store {
    Store { state: DuckPond::new() }
  }
  pub fn get_state(&self) -> &DuckPond {
    &self.state
  }
  pub fn dispatch(&mut self, duck: Duck) {
    self.state.add_duck(duck);
  }
}

#[derive(Debug, Clone)]
pub struct DuckPond {
  pub ducks: Vec<Duck>,
}
impl DuckPond {
  pub fn new() -> DuckPond {
    DuckPond { ducks: Vec::new() }
  }
  pub fn add_duck(&self, duck: Duck) -> DuckPond {
    let mut copy_duck_pond = self.clone();
    copy_duck_pond.ducks.push(duck);
    copy_duck_pond
  }
}

#[derive(Debug, Clone)]
pub struct Location {
  lat: f64,
  long: f64,
}
impl Location {
  pub fn new(lat: f64, long: f64) -> Location {
    Location { lat, long }
  }
}

#[derive(Debug, Clone)]
pub struct Duck {
  id: u32,
  name: String,
  age: u32,
  location: Location,
}

impl Duck {
  pub fn new(id: u32, name: String, age: u32, lat: f64, long: f64) -> Duck {
    Duck {
      id,
      name,
      age,
      location: Location::new(lat, long),
    }
  }
}
