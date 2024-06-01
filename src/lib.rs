use std::sync::Arc;

mod container;
mod macros;

pub fn inject<T: 'static>() -> Arc<T> {
    inject_optional().expect("Must be provided")
}

pub fn inject_optional<T: 'static>() -> Option<Arc<T>> {
    container::get_sized_item()
}

#[doc(hidden)]
pub mod __private {
    use std::{any::TypeId, sync::Arc};

    pub fn inject_unsized<T: ?Sized + 'static>() -> Arc<T> {
        inject_unsized_optional().expect("Must be provided")
    }

    pub fn inject_unsized_optional<T: ?Sized + 'static>() -> Option<Arc<T>> {
        crate::container::get_unsized_item()
    }

    pub fn provide_sized<T: Send + Sync + 'static>(item: Arc<T>) {
        let previous = crate::container::set_item(TypeId::of::<T>(), item);
        if let Some(previous) = previous {
            drop(previous.to_arc_sized::<T>());
        }
    }

    pub fn provide_unsized<T: Send + Sync + ?Sized + 'static>(item: Arc<T>) {
        let previous = crate::container::set_item(TypeId::of::<T>(), item);
        if let Some(previous) = previous {
            drop(previous.to_arc_unsized::<T>());
        }
    }
}
