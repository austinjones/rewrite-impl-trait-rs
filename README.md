# rewrite-impl-trait
This crate converts usage of `impl Trait` in function signatures to method generics.  Here are some examples:

## How it works
The macro `into_generic` converts `impl Trait` definitions in 
```rust
#[rewrite_impl_trait::into_generic]
fn to_string(arg: impl ToString) -> String {
    arg.to_string()
}

// expands to:
fn to_string<RewriteImplTrait0: ToString>(arg: RewriteImplTrait0) -> String {
    arg.to_string()
}
```

```rust
pub trait AppendString {
    fn append_string(&mut self, param: impl ToString);
}

// expands to:
pub trait AppendString {
    fn append_string<RewriteImplTrait0: ToString>(&mut self, param: RewriteImplTrait0);
}
```

This can be used to work around language issues with `impl Trait`, such as a lack of support in type aliases.
It also enables [mockall](https://crates.io/crates/mockall), for traits that use `impl Trait` in method arguments.

## Usage
Run `cargo add rewrite-impl-trait`.

Then add `#[rewrite_impl_trait::into_generic]` to your trait, trait impl, or function.