#[derive(Clone)]
struct Foo {
    value: &'static str,
}

fn make_foo() -> Foo {
    Foo {
        value: "some string",
    }
}

#[derive(Clone)]
pub struct Bar {
    int: i64,
    foo: Foo,
}

pub fn barify() -> Bar {
    Bar {
        int: 0,
        foo: make_foo(),
    }
}
