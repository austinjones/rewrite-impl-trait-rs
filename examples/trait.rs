#[rewrite_impl_trait::into_generic]
pub trait AppendString {
    fn append_string(&mut self, param: impl ToString);
}

fn main() {}
