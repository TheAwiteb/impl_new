#[test]
fn empty_tuble_struct() {
    #[derive(impl_new::New)]
    struct Test();

    let _ = Test::new();
}

#[test]
fn single_tuble_struct() {
    #[derive(impl_new::New)]
    struct Test(#[impl_new(name = "username")] String);

    let test = Test::new("Awiteb");
    assert_eq!(test.0, "Awiteb".to_owned());
}

#[test]
fn multiple_tuble_struct() {
    #[derive(Debug, PartialEq)]
    enum Country {
        Ksa,
    }

    #[derive(impl_new::New)]
    struct Test(
        #[impl_new(name = "username")] String,
        #[impl_new(name = "age")] u8,
        #[impl_new(name = "country")] Country,
    );

    let test = Test::new("Awiteb", 20, Country::Ksa);
    assert_eq!(test.0, "Awiteb".to_owned());
    assert_eq!(test.1, 20);
    assert_eq!(test.2, Country::Ksa);
}

#[test]
fn into_of_into_tuble_struct() {
    struct Foo(String);

    impl<T> From<T> for Foo
    where
        T: Into<String>,
    {
        fn from(s: T) -> Self {
            Self(s.into())
        }
    }

    #[derive(impl_new::New)]
    struct Test(
        #[impl_new(name = "username")] String,
        #[impl_new(name = "data")] Foo,
    );

    let test = Test::new("Awiteb", "Hello World");
    assert_eq!(test.0, "Awiteb".to_owned());
    assert_eq!(test.1 .0, "Hello World".to_owned());
}

#[test]
fn with_default_option() {
    #[derive(impl_new::New)]
    struct Test(
        #[impl_new(name = "name")] String,
        #[impl_new(default)] usize,
        #[impl_new(default)] bool,
    );

    let test = Test::new("Awiteb");
    assert_eq!(test.0, "Awiteb".to_owned());
    assert_eq!(test.1, 0);
    assert!(!test.2); // false
}

#[test]
fn with_default_and_value_option() {
    #[derive(impl_new::New)]
    struct Test(
        #[impl_new(name = "name")] String,
        #[impl_new(default)] usize,
        #[impl_new(value = || true)] bool,
    );

    let test = Test::new("Awiteb");
    assert_eq!(test.0, "Awiteb".to_owned());
    assert_eq!(test.1, 0);
    assert!(test.2); // true
}

#[test]
fn with_value_option() {
    #[derive(impl_new::New)]
    struct Test(
        #[impl_new(name = "name")] String,
        #[impl_new(value = || 20)] usize,
        #[impl_new(value = || true)] bool,
    );

    let test = Test::new("Awiteb");
    assert_eq!(test.0, "Awiteb".to_owned());
    assert_eq!(test.1, 20);
    assert!(test.2); // true
}

#[test]
fn with_function_value_option() {
    fn get_something() -> String {
        "Some Value".to_owned()
    }

    #[derive(impl_new::New)]
    struct Test(
        #[impl_new(name = "name")] String,
        #[impl_new(value = || get_something())] String,
    );

    let test = Test::new("Awiteb");
    assert_eq!(test.0, "Awiteb".to_owned());
    assert_eq!(test.1, "Some Value".to_owned());
}

#[test]
fn with_associated_function_value_option() {
    #[derive(impl_new::New)]
    struct Test(
        #[impl_new(name = "name")] String,
        #[impl_new(value = || String::from("Some Value"))] String,
    );

    let test = Test::new("Awiteb");
    assert_eq!(test.0, "Awiteb".to_owned());
    assert_eq!(test.1, "Some Value".to_owned());
}

#[test]
fn with_self_associated_function_value_option() {
    #[derive(impl_new::New)]
    struct Test(
        #[impl_new(name = "name")] String,
        #[impl_new(value = || Self::get_something())] String,
    );

    impl Test {
        fn get_something() -> String {
            "Some Value".to_owned()
        }
    }

    let test = Test::new("Awiteb");
    assert_eq!(test.0, "Awiteb".to_owned());
    assert_eq!(test.1, "Some Value".to_owned());
}
