<div align="center">

Deprecation Notice: This crate is deprecated in favor of the [`derive_new`](https://crates.io/crates/derive_new) crate. Please use the [`derive_new`](https://crates.io/crates/derive_new) crate instead of this crate.

# Impl New 🦀
A procedural macro to generate a new function implementation for your struct.

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/license/mit/)
[![Crates.io](https://img.shields.io/crates/v/impl_new.svg)](https://crates.io/crates/impl_new)

[![Docs.rs](https://docs.rs/impl_new/badge.svg)](https://docs.rs/impl_new/latest/impl_new/)
[![CI](https://github.com/TheAwiteb/impl_new/actions/workflows/ci.yml/badge.svg)](https://github.com/TheAwiteb/impl_new/actions/workflows/ci.yml)
[![CD](https://github.com/TheAwiteb/impl_new/actions/workflows/cd.yml/badge.svg)](https://github.com/TheAwiteb/impl_new/actions/workflows/cd.yml)

</div>

## 🚀 Add to your project
Add this to your `Cargo.toml`:
```toml
[dependencies]
impl_new = "0.2.0"
```
Or run this command in your workspace:
```bash
cargo add impl_new
```

## ❓ What is the new function?
The `new` function is a function that is used to create a new instance of a struct. It is a common pattern in Rust to use a `new` function to create a new instance of a struct instead of using the struct directly. This is because it is easier to add new fields to the struct without breaking the code that uses it.

## 👨‍💻 Usage
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

## 🛹 Attributes
### `#[impl_new(name = "name")]`
The `name` option specifies the name of the argument in the `new` function.

> Note: This attribute is required for unnamed fields.

#### Example
```rust
#[derive(impl_new::New)]
struct User(#[impl_new(name = "username")] String, #[impl_new(name = "age")] i32);

// The generated code will look like this:
// impl User {
//     pub fn new(username: impl Into<String>, age: Into<i32>) -> Self {
//         Self(username.into(), age.into())
//     }

fn main() {
    let user = User::new("Hello", 42); // Will use `Into::into` to convert the arguments to the fields types.
    assert_eq!(user.0, "Hello".to_string());
    assert_eq!(user.1, 42);
}
```

```rust
#[derive(impl_new::New)]
struct User {
    #[impl_new(name = "username")]
    name: String,
    #[impl_new(name = "user_age")]
    age: i32,
}

// The generated code will look like this:
// impl User {
//     pub fn new(username: impl Into<String>, user_age: Into<i32>) -> Self {
//         Self { name: username.into(), age: user_age.into() }
//     }
// }

fn main() {
    let user = User::new("Hello", 42); // Will use `Into::into` to convert the arguments to the fields types.
    assert_eq!(user.name, "Hello".to_string());
    assert_eq!(user.age, 42);
}
```

### `#[impl_new(default)]`
The `default` option will remove the field from the `new` function arguments and use the default value of the field type instead.

> Note: This option is conflict with the `name` option, because the field will be removed from the `new` function arguments.

#### Example
```rust
#[derive(impl_new::New, Default, Debug, PartialEq)]
struct User {
    name: String,
    #[impl_new(default)]
    is_admin: bool,
}

// The generated code will look like this:
// impl User {
//     pub fn new(name: impl Into<String>) -> Self {
//         Self { name: name.into(), is_admin: bool::default() }
//     }
// }

#[derive(impl_new::New)]
struct Foo(#[impl_new(name = "somthing")] String, #[impl_new(default)] User);

// The generated code will look like this:
// impl Foo {
//     pub fn new(somthing: impl Into<String>) -> Self {
//         Self(somthing.into(), User::default())
//     }
// }

fn main() {
    let user = User::new("Hello"); // Will use `Into::into` to convert the arguments to the fields types.
    let some_foo = Foo::new("Hello"); // Will use `Into::into` to convert the arguments to the fields types.
    assert_eq!(user.name, "Hello".to_string());
    assert_eq!(user.is_admin, false);
    assert_eq!(some_foo.0, "Hello".to_string());
    assert_eq!(some_foo.1, User::default());
}
```

### `#[impl_new(value = || <VALUE>)]`
The `value` option will set the field value to the given value.

> Note: This option is conflict with the `name` and `default` options, because the field will be removed from the `new` function arguments.

> Note: The value must be a closure that returns the field type.

#### Example
```rust
#[derive(impl_new::New)]
struct User {
    name: String,
    #[impl_new(value = || true)]
    is_active: bool,
}

// The generated code will look like this: (Not exactly, but you get the idea)
// impl User {
//     pub fn new(name: impl Into<String>) -> Self {
//         Self { name: name.into(), is_active: true }
//     }
// }

#[derive(impl_new::New)]
struct Foo(#[impl_new(name = "name")] String, #[impl_new(value = || true)] bool);

// The generated code will look like this: (Not exactly, but you get the idea)
// impl Foo {
//     pub fn new(name: impl Into<String>) -> Self {
//         Self(name.into(), true)
//     }
// }

fn main() {
    let user = User::new("Bob"); // Will use `Into::into` to convert the arguments to the fields types.
    let some_foo = Foo::new("Bob"); // Will use `Into::into` to convert the arguments to the fields types.
    assert_eq!(user.name, "Bob".to_string());
    assert_eq!(user.is_active, true);
    assert_eq!(some_foo.0, "Bob".to_string());
    assert_eq!(some_foo.1, true);
}
```

## 🤗 Contributing
Contributions are welcome! You can contribute in many ways, for example:
- Improve the documentation.
- Add more tests.
- Add more examples.
- Report a bug by opening an issue.
- Suggest a new feature by opening an issue.
- Fix a bug or add a new feature. (Please open an issue first if you want to add a new feature or if you want to fix a bug that doesn't have an issue yet.)
- Fix a typo.
- Refactor the code.
- Improve the error messages.

## 🤝 Code of Conduct
We are committed to providing a friendly, safe and welcoming environment for all. Please read and respect the [Code of Conduct].

## 📝 Changelog
See [CHANGELOG.md].

## 🔑 License
This project is licensed under the [MIT license].

[MIT license]: https://opensource.org/license/mit/
[Code of Conduct]: https://www.rust-lang.org/policies/code-of-conduct
[CHANGELOG.md]: CHANGELOG.md
[crates.io]: https://crates.io/crates/impl_new
