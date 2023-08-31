# Impl New
A procedural macro to generate a new function for your struct.

## Add to your project
Add this to your `Cargo.toml`:
```toml
[dependencies]
impl_new = "0.1.0"
```
Or run this command in your workspace:
```bash
cargo add impl_new
```

## Usage
Is simple, just derive the `impl_new::New` proc macro on your struct and it will generate a `new` function for you.

### For Named Fields

```rust
#[derive(impl_new::New)]
struct Foo {
    name: String,
    age: usize,
}

// The generated code will look like this:
// impl Foo {
//     pub fn new(name: impl Into<String>, age: Into<usize>) -> Self {
//         Self { name: name.into(), age: age.into() }
//     }
// }

fn main() {
    let foo = Foo::new("Hello", 42usize); // Will use `Into::into` to convert the arguments to the fields types.
    assert_eq!(foo.name, "Hello".to_string());
    assert_eq!(foo.age, 42);
}
```

#### With Attributes

```rust
#[derive(impl_new::New)]
struct Foo {
    #[impl_new(name = "user_name")]
    name: String,
    #[impl_new(name = "user_age")]
    age: usize,
}

// The generated code will look like this:
// impl Foo {
//     pub fn new(user_name: impl Into<String>, user_age: Into<usize>) -> Self {
//         Self { name: user_name.into(), age: user_age.into() }
//     }
// }

fn main() {
    let foo = Foo::new("Hello", 42usize); // Will use `Into::into` to convert the arguments to the fields types.
    assert_eq!(foo.name, "Hello".to_string());
    assert_eq!(foo.age, 42);
}
```


### For Unnamed Fields
> Note: The `#[impl_new(name = "name")]` attribute is required for unnamed fields.

```rust
#[derive(impl_new::New)]
struct Foo(#[impl_new(name = "name")] String, #[impl_new(name = "age")] usize);

// The generated code will look like this:
// impl Foo {
//     pub fn new(name: impl Into<String>, age: Into<usize>) -> Self {
//         Self(name.into(), age.into())
//     }
// }

fn main() {
    let foo = Foo::new("Hello", 42usize); // Will use `Into::into` to convert the arguments to the fields types.
    assert_eq!(foo.0, "Hello".to_string());
    assert_eq!(foo.1, 42);
}
```

## Attributes
- `#[impl_new(name = "name")]`: Use this attribute to change the name of the argument in the generated `new` function.

## License
This project is licensed under the [MIT license](https://opensource.org/license/mit/).


