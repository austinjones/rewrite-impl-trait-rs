#[rewrite_impl_trait::into_generic]
pub trait AppendString {
    fn append_string(&mut self, param: impl ToString);
}

#[rewrite_impl_trait::into_generic]
impl AppendString for String {
    fn append_string(&mut self, param: impl ToString) {
        *self += &param.to_string();
    }
}

fn main() {
    let mut string = "Hello".to_string();
    string.append_string(" World");
    println!("{}", string);
}
