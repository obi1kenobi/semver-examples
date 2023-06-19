use std::rc::Rc;

#[derive(Clone)]
struct Foo {
    value: Rc<str>,  // support non-static strings;
                     // ref-count for cheap cloning
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
            value: Rc::from("some string"),
        }
    }
}
