pub struct Foo {
    pub first: i64,
    pub second: bool,
    value: Option<String>,  // new private field
}

impl Foo {
    pub fn new(first: i64, second: bool) -> Self {
        Self {
            first,
            second,
            value: None,  // set to a default value
        }
    }

    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }
}
