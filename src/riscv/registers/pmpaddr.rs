use super::CSRegister;

use custom_test::custom_test;

macro_rules! make_pmpaddr {
    ($addr_num: expr) => {
        paste::item! {
            #[derive(Debug, Clone, PartialEq, Default)] pub struct [<PMPAddr $addr_num>] { value: usize,
            }

            impl [<PMPAddr $addr_num>] {
                #[allow(unused)]
                #[inline]
                pub fn get_addr(&self) -> usize {
                    self.value
                }
                #[allow(unused)]
                #[inline]
                pub fn set_addr(mut self, value: usize) -> Self {
                    self.value = value;
                    self
                }
            }

            impl CSRegister for [<PMPAddr $addr_num>] {
                unsafe fn write(s: Self) {
                    write_csr!(concat!("pmpaddr", stringify!($addr_num)));
                    internal_write(s.value);
                }
                fn read() -> Self {
                    read_csr!(concat!("pmpaddr", stringify!($addr_num)));
                    Self {
                        value: internal_read(),
                    }
                }
            }
        }
    };
}

use seq_macro::seq;

seq!(N in 0..16 {
    make_pmpaddr!(N);
});

#[custom_test(ModeMachine)]
fn write_method_test() {
    let mut pmpaddr = PMPAddr0::default();
    pmpaddr = pmpaddr.set_addr(0xdeadbeef);

    assert_eq!(pmpaddr.value, 0xdeadbeef);
}

macro_rules! make_pmpaddr_test {
    ($addr_num: expr) => {
        paste::item! {
            #[custom_test(ModeMachine)]
            fn [<write_pmpaddr_test_ $addr_num>]() {
                unsafe {
                    [<PMPAddr $addr_num>]::initialize();
                }
                [<PMPAddr $addr_num>]::operate(| old | {
                    old.set_addr($addr_num * 1000)
                });
                assert_eq!([<PMPAddr $addr_num>]::read().value, $addr_num * 1000);
            }
        }
    };
}

seq!(N in 0..16 {
    make_pmpaddr_test!(N);
});
