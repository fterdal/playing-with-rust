struct Counter {
    count: u32
}

impl Counter {
    pub fn get_count(&self) -> u32 {
        self.count
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }
}

fn main() {
    let mut counter1 = Counter { count: 0 };
    println!("Counter: {}", counter1.get_count());
    counter1.increment();
    println!("Counter: {}", counter1.get_count());
}
