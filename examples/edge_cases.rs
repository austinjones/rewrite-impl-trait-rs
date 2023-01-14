#[rewrite_impl_trait::into_generic]
fn other_generic<T: Into<String>>(smash: impl ToString, with: T) -> String {
    let mut string = smash.to_string();
    string += &with.into();

    string
}

#[rewrite_impl_trait::into_generic]
fn multiple_impls(smash: impl ToString, that: impl AsRef<str>) -> String {
    let mut string = smash.to_string();
    string += that.as_ref();

    string
}

#[rewrite_impl_trait::into_generic]
fn borrowed_lifetime<'a>(string: &'a mut String, smash: impl ToString + 'a) {
    *string += &smash.to_string();
}

// Not rewritten.
#[rewrite_impl_trait::into_generic]
fn return_impl() -> impl ToString {
    "Hello World"
}

fn main() {
    println!("{}", other_generic("Hello", " World"));
    println!("{}", multiple_impls("Hello", " World"));
    println!("{}", return_impl().to_string());
    borrowed_lifetime(&mut "Hello".to_string(), " World")
}
