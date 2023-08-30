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
