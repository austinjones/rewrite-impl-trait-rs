#[rewrite_impl_trait::into_generic]
fn to_string(arg: impl ToString) -> String {
    arg.to_string()
}

fn main() {
    println!("{}", to_string("Hello"));
}
