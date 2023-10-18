struct Example {
    value: i32,
}

impl Example {
    fn new(value: i32) -> Self {
        let mut instance = Self { value };
        instance.initialize();
        instance
    }

    fn initialize(&mut self) {
        // Initialization code here...
        self.value *= 2;
    }
}

fn main() {
    let example = Example::new(5);
    println!("{}", example.value);
}