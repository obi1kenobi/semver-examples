#[derive(Clone)]
struct Foo {
    value: &'static str,
}

#[derive(Clone)]
pub struct Bar {
    int: i64,
    foo: Foo,
}

pub fn barify() -> Bar {
    Bar {
        int: 0,
        foo: Foo {
            value: "some string",
        }
    }
}
