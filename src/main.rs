struct Counter {
    count: u32,
}

impl Counter {
    // Returns the current count
    pub fn get_count(&self) -> u32 {
        self.count
    }

    // Increments the count (remember to create a mutable counter)
    pub fn increment(&mut self) {
        self.count += 1;
    }

    // Creates a function that returns the current count
    pub fn create_get_count_fn(&mut self) -> impl Fn() -> u32 + '_ {
        move || self.count
    }
}

fn main() {
    let mut counter1 = Counter { count: 0 };
    println!("Counter: {}", counter1.get_count());
    counter1.increment();
    println!("Counter: {}", counter1.get_count());
    let new_get_count = counter1.create_get_count_fn();
    // counter1.increment(); // This won't work, cause there are two mutable borrows
    println!("Counter: {}", new_get_count());
}
