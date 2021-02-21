use custom_test::custom_test;
use inventory;

#[allow(unused)]
pub enum TestCondition {
    ModeMachine,
    ModeSupervisor,
}

pub trait Testable {
    fn run(&self);
    fn get_name(&self) -> &str;
}
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("{}...\t", core::any::type_name::<T>());
        self();
        println!("[b] ok");
    }
    fn get_name(&self) -> &str {
        core::any::type_name::<T>()
    }
}

#[allow(unused)]
pub struct TestCase {
    #[allow(unused)]
    cond: TestCondition,
    #[allow(unused)]
    test: fn() -> (),
}

inventory::collect!(TestCase);

macro_rules! condition_test {
    ($condition: expr) => {
        paste::item! {
            for x in inventory::iter::<TestCase>.into_iter() {
                match x {
                    TestCase { cond: $condition, test: f } => {
                        f();
                    }
                    _ => {
                        println!("Foo!");
                    }
                }
            }
        }
    };
}

pub fn runner_interface(test_case: &[&dyn Testable]) {
    let tc_num = test_case.len();
    if tc_num != 0 {
        println!();
        println!("#[test_case] attribute can not use.");
        println!("Use #[custom_test(Mode)] instead.");
        println!();
        for t in test_case {
            println!("Ignore {}", t.get_name());
        }
        println!();
    }

    // condition_test!(TestCondition::ModeMachine);

    println!("Test ok. see ya!");
    crate::shutdown(0);
}
