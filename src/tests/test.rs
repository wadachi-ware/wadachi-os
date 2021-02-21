use custom_test::custom_test;
use inventory;
use paste;

#[allow(unused)]
pub enum TestCondition {
    ModeMachine,
    ModeSupervisor,
}

pub trait Testable {
    fn run(&self);
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

fn test_entry() {
    condition_test!(TestCondition::ModeSupervisor);
}
