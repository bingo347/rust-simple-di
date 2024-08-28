/// Inject trait implementation from the DI container.
///
/// # Panics
/// Panics if the trait is not registered in the container.
#[macro_export]
macro_rules! inject {
    ($trait:path) => {
        $crate::__private::inject_unsized::<dyn $trait + Send + Sync>()
    };
}

/// Inject trait implementation from the DI container.
///
/// Returns `None` if the trait is not registered in the container.
#[macro_export]
macro_rules! inject_optional {
    ($trait:path) => {
        $crate::__private::inject_unsized_optional::<dyn $trait + Send + Sync>()
    };
}

/// Provide value to the DI container.
///
/// # Example
/// ```rust
/// use simple_di::provide;
///
/// #[derive(Debug)]
/// struct Foo(i32);
///
/// provide!(Foo(42));
///
/// #[derive(Debug)]
/// struct Bar(i32);
///
/// trait Baz {
///     fn baz(&self) -> i32;
/// }
///
/// impl Baz for Bar {
///     fn baz(&self) -> i32 {
///         self.0
///     }
/// }
///
/// trait Qux {
///     fn qux(&self) -> i32;
/// }
///
/// impl Qux for Bar {
///     fn qux(&self) -> i32 {
///         self.0
///     }
/// }
///
/// provide!(Bar(42) => Baz, Qux);
/// ```
#[macro_export]
macro_rules! provide {
    ($value:expr) => {
        $crate::__private::provide_sized(::std::sync::Arc::new($value))
    };
    ($value:expr => $($trait:path),+ $(,)?) => {{
        let value = ::std::sync::Arc::new($value);
        $(
            $crate::__private::provide_unsized::<dyn $trait + Send + Sync>(value.clone());
        )+
        $crate::__private::provide_sized(value);
    }};
}
