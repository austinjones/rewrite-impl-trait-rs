pub struct Foo;

#[rewrite_impl_trait::into_generic]
impl Foo {
    fn append_string(&mut self, _string: impl ToString) {}
}

fn main() {
    let mut foo = Foo {};
    foo.append_string("Hello World");
}
