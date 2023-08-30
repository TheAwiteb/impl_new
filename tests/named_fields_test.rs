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
fn with_attributes() {
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
