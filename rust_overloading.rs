struct MyStruct {
    x: i32,
    y: i32,
}

impl MyStruct {
    fn new(x: i32) -> Self {
        Self { x, y: 0 }
    }

    fn new_with_y(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}