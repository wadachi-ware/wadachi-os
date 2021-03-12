use custom_test::custom_test;

#[allow(unused)]
#[derive(PartialEq)]
pub enum TestCondition {
    FirstTest,
    ModeMachine,
    ModeSupervisor,
    IntegrationMachineToSupervisor,
    IntegrationVirtualMemory,
}

pub trait Testable {
    fn run(&self) -> bool;
}
impl<T> Testable for (TestCondition, T, &'static str)
where
    T: Fn(),
{
    fn run(&self) -> bool {
        match self {
            (c, f, n) => match c {
                TestCondition::IntegrationMachineToSupervisor |
                TestCondition::IntegrationVirtualMemory => {
                    println!("[+] Integration test. {}", n);
                    f();
                    println!("[b] ok");
                }
                _ => {
                    print!("[+] {0: <100} ", n);
                    f();
                    println!("[b] ok");
                }
            },
        }
        true
    }
}

pub trait TestProvider {
    fn test_if_match(&self, _: TestCondition) -> bool;
}

impl<T, F> TestProvider for T
where
    F: Fn(),
    T: Fn() -> (TestCondition, F, &'static str),
{
    fn test_if_match(&self, c: TestCondition) -> bool {
        match self() {
            (p, f, n) if p == c => (c, f, n).run(),
            (_, _, _) => false,
        }
    }
}

#[allow(dead_code)]
pub fn runner_interface(test_case: &[&dyn TestProvider]) {
    println!("[/] First Testing...");
    for x in test_case {
        x.test_if_match(TestCondition::FirstTest);
    }

    println!("[/] Testing in Machine mode");
    for x in test_case {
        x.test_if_match(TestCondition::ModeMachine);
    }

    println!("[/] Mode switch");
    for x in test_case {
        x.test_if_match(TestCondition::IntegrationMachineToSupervisor);
    }

    for x in test_case {
        x.test_if_match(TestCondition::IntegrationVirtualMemory);
    }

    println!("[/] Testing in Supervisor mode");
    for x in test_case {
        x.test_if_match(TestCondition::ModeSupervisor);
    }

    println!();
    println!("test ok. see ya!");
    crate::shutdown(0);
}

#[custom_test(FirstTest)]
fn test_of_test() {
    assert_eq!(1 + 2, 3);
}
