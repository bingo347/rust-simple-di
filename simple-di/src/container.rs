use dashmap::DashMap;
use lazy_static::lazy_static;
use std::{any::TypeId, mem, ptr, sync::Arc};

lazy_static! {
    static ref SHARED_CONTAINER: DashMap<TypeId, ItemPtr> = DashMap::new();
}

pub(crate) fn get_sized_item<T: 'static>() -> Option<Arc<T>> {
    let item = SHARED_CONTAINER.get(&TypeId::of::<T>())?;
    Some(item.to_arc_sized(true))
}

pub(crate) fn get_unsized_item<T: ?Sized + 'static>() -> Option<Arc<T>> {
    let item = SHARED_CONTAINER.get(&TypeId::of::<T>())?;
    Some(item.to_arc_unsized(true))
}

pub(crate) fn set_item(id: TypeId, item: impl Into<ItemPtr>) -> Option<ItemPtr> {
    let item = item.into();
    SHARED_CONTAINER.insert(id, item)
}

#[derive(Clone, Copy)]
#[repr(C)]
pub(crate) struct ItemPtr {
    data: *const (),
    v_table: *const (),
}

unsafe impl Send for ItemPtr {}
unsafe impl Sync for ItemPtr {}

const PTR_SIZE: usize = mem::size_of::<usize>();
const FAT_PTR_SIZE: usize = PTR_SIZE * 2;

union ItemPtrWrapper<T: ?Sized> {
    raw: *const T,
    item: ItemPtr,
}

impl<T: ?Sized> From<Arc<T>> for ItemPtr {
    fn from(value: Arc<T>) -> Self {
        let raw = Arc::into_raw(value);
        match mem::size_of::<*const T>() {
            PTR_SIZE => ItemPtr {
                data: raw as _,
                v_table: ptr::null(),
            },
            FAT_PTR_SIZE => {
                let wrapper = ItemPtrWrapper { raw };
                unsafe { wrapper.item }
            }
            _ => unreachable!(),
        }
    }
}

impl ItemPtr {
    pub(crate) fn to_arc_unsized<T: ?Sized>(self, need_increment: bool) -> Arc<T> {
        let t_ptr_size = mem::size_of::<*const T>();
        assert_eq!(t_ptr_size, FAT_PTR_SIZE);
        let wrapper = ItemPtrWrapper { item: self };
        unsafe {
            if need_increment {
                Arc::<T>::increment_strong_count(wrapper.raw);
            }
            Arc::<T>::from_raw(wrapper.raw)
        }
    }

    pub(crate) fn to_arc_sized<T>(self, need_increment: bool) -> Arc<T> {
        assert!(self.v_table.is_null());
        unsafe {
            if need_increment {
                Arc::<T>::increment_strong_count(self.data as _);
            }
            Arc::<T>::from_raw(self.data as _)
        }
    }
}
