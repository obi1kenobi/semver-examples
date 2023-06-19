pub struct Foo {
    pub first: i64,
    pub second: bool,
    pub third: Option<String>,
}

impl Foo {
    pub fn new(first: i64, second: bool) -> Self {
        Self {
            first,
            second,
            third: None,  // set to a default value
        }
    }
}
