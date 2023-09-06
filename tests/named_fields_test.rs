#[test]
fn without_fields() {
    #[derive(impl_new::New)]
    struct Test {}

    let _ = Test::new();
}

#[test]
fn single_field() {
    #[derive(impl_new::New)]
    struct Test {
        name: String,
    }

    let test = Test::new("Awiteb");
    assert_eq!(test.name, "Awiteb".to_owned());
}

#[test]
fn multiple_fields() {
    #[derive(Debug, PartialEq)]
    enum Country {
        Ksa,
    }

    #[derive(impl_new::New)]
    struct Test {
        name: String,
        age: u8,
        country: Country,
    }

    let test = Test::new("Awiteb", 20, Country::Ksa);
    assert_eq!(test.name, "Awiteb".to_owned());
    assert_eq!(test.age, 20);
    assert_eq!(test.country, Country::Ksa);
}

#[test]
fn into_of_into() {
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
    struct Test {
        name: String,
        data: Foo,
    }

    let test = Test::new("Awiteb", "Hello World");
    assert_eq!(test.name, "Awiteb".to_owned());
    assert_eq!(test.data.0, "Hello World".to_owned());
}

#[test]
fn with_name_option() {
    #[derive(impl_new::New)]
    struct Test {
        #[impl_new(name = "user_name")]
        name: String,
        #[impl_new(name = "user_age")]
        age: usize,
    }

    let test = Test::new("Awiteb", 20usize);
    assert_eq!(test.name, "Awiteb".to_owned());
    assert_eq!(test.age, 20);
}

#[test]
fn with_default_option() {
    #[derive(impl_new::New)]
    struct Test {
        #[impl_new(default)]
        name: String,
        #[impl_new(default)]
        age: usize,
    }

    let test = Test::new();
    assert_eq!(test.name, String::default());
    assert_eq!(test.age, usize::default());
}

#[test]
fn with_default_and_value_option() {
    #[derive(impl_new::New)]
    struct Test {
        #[impl_new(default)]
        name: String,
        #[impl_new(value = || 42)]
        age: usize,
    }

    let test = Test::new();
    assert_eq!(test.name, String::default());
    assert_eq!(test.age, 42);
}

#[test]
fn with_value_option() {
    #[derive(impl_new::New)]
    struct Test {
        name: String,
        #[impl_new(value = || 42)]
        age: usize,
        #[impl_new(value = || true)]
        is_somthing: bool,
    }

    let test = Test::new("Awiteb");
    assert_eq!(test.name, "Awiteb".to_owned());
    assert_eq!(test.age, 42);
    assert!(test.is_somthing); // true
}

#[test]
fn with_function_value_option() {
    fn get_age() -> usize {
        42
    }

    #[derive(impl_new::New)]
    struct Test {
        name: String,
        #[impl_new(value = || get_age())]
        age: usize,
    }

    let test = Test::new("Awiteb");
    assert_eq!(test.name, "Awiteb".to_owned());
    assert_eq!(test.age, 42);
}

#[test]
fn with_associated_function_value_option() {
    struct Foo;

    impl Foo {
        fn get_age() -> usize {
            42
        }
    }

    #[derive(impl_new::New)]
    struct Test {
        name: String,
        #[impl_new(value = || Foo::get_age())]
        age: usize,
    }

    let test = Test::new("Awiteb");
    assert_eq!(test.name, "Awiteb".to_owned());
    assert_eq!(test.age, 42);
}

#[test]
fn with_self_associated_function_value_option() {
    #[derive(impl_new::New)]
    struct Test {
        name: String,
        #[impl_new(value = || Self::some_date())]
        data: String,
    }

    impl Test {
        fn some_date() -> String {
            "2023-01-01".to_owned()
        }
    }

    let test = Test::new("Awiteb");
    assert_eq!(test.name, "Awiteb".to_owned());
    assert_eq!(test.data, "2023-01-01".to_owned());
}
