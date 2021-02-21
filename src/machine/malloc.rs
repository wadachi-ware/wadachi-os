#![allow(unused)]
use alloc::alloc::{GlobalAlloc, Layout};
use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap() {
    unsafe {
        ALLOCATOR.lock().init(
            crate::KERNEL_HEAP_START_ADDR,
            crate::KERNEL_HEAP_END_ADDR - crate::KERNEL_HEAP_START_ADDR,
        );
    }
}

#[alloc_error_handler]
fn handle(x: Layout) -> ! {
    panic!("malloc fail. size: {}", x.size());
}

#[test_case]
fn raw_malloc_test() {
    let layout = Layout::from_size_align(4, 4).unwrap();
    let addr = unsafe { ALLOCATOR.alloc(layout) };

    assert!(unsafe { crate::KERNEL_HEAP_START_ADDR } <= addr as usize);
    assert!((addr as usize) < unsafe { crate::KERNEL_STACK_END_ADDR });
}

#[test_case]
fn box_test() {
    use alloc::boxed::Box;

    let addr = Box::into_raw(Box::new(0));

    assert!(unsafe { crate::KERNEL_HEAP_START_ADDR } <= addr as usize);
    assert!((addr as usize) < unsafe { crate::KERNEL_STACK_END_ADDR });
}

#[test_case]
fn heap_write_test() {
    use alloc::boxed::Box;

    let mut b = Box::new(10);
    assert_eq!(*b, 10);

    *b = 20;
    assert_eq!(*b, 20);
}
