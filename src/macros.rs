#[macro_export]
macro_rules! inject {
    ($trait:path) => {
        $crate::__private::inject_unsized::<dyn $trait + Send + Sync>()
    };
}

#[macro_export]
macro_rules! inject_optional {
    ($trait:path) => {
        $crate::__private::inject_unsized_optional::<dyn $trait + Send + Sync>()
    };
}

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
        $crate::__private::provide_sized(::std::sync::Arc::new($value))
    }};
}
