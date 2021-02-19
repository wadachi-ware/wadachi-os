pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        crate::print!("{}...\t", core::any::type_name::<T>());
        self();
        crate::println!("[b] ok");
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    crate::println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    crate::shutdown(0);
}

#[test_case]
fn test_of_test() {
    assert_eq!(1 + 2, 3);
}
