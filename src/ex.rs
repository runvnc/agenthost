pub struct Model {
    name: String,
}

impl Model {
    pub fn new(name: String) -> Model {
        Model {
            name,
        }
    }
}