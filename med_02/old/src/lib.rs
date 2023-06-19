pub struct Foo {
    pub first: i64,
    pub second: bool,
}

impl Foo {
    pub fn new(first: i64, second: bool) -> Self {
        Self {
            first,
            second,
        }
    }
}
