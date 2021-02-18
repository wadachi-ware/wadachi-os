use super::CSRegister;

macro_rules! make_pmpaddr {
    ($addr_num: expr) => {
        paste::item! {
            #[derive(Debug, PartialEq)]
            pub struct [<PMPAddr $addr_num>] {
                value: usize,
            }

            impl [<PMPAddr $addr_num>] {
                #[allow(unused)]
                #[inline]
                pub fn get_addr(&self) -> usize {
                    self.value
                }
                #[allow(unused)]
                #[inline]
                pub fn set_addr(&mut self, value: usize) {
                    self.value = value;
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
                fn get_unset() -> Self {
                    Self {
                        value: 0
                    }
                }
            }
        }
    };
}

make_pmpaddr!(0);
make_pmpaddr!(1);
make_pmpaddr!(2);
make_pmpaddr!(3);
make_pmpaddr!(4);
make_pmpaddr!(5);
make_pmpaddr!(6);
make_pmpaddr!(7);
make_pmpaddr!(8);
make_pmpaddr!(9);
make_pmpaddr!(10);
make_pmpaddr!(11);
make_pmpaddr!(12);
make_pmpaddr!(13);
make_pmpaddr!(14);
make_pmpaddr!(15);

#[test_case]
fn write_method_test() {
    let mut pmpaddr = PMPAddr0::get_unset();
    pmpaddr.set_addr(0xdeadbeef);

    assert_eq!(pmpaddr.value, 0xdeadbeef);
}

macro_rules! make_pmpaddr_test {
    ($addr_num: expr) => {
        paste::item! {
            #[test_case]
            fn [<write_pmpaddr_test_ $addr_num>]() {
                unsafe {
                    [<PMPAddr $addr_num>]::initialize();
                }
                [<PMPAddr $addr_num>]::operate(|mut old| {
                    old.set_addr($addr_num * 1000);

                    old
                });
                assert_eq!([<PMPAddr $addr_num>]::read().value, $addr_num * 1000);
            }
        }
    };
}

make_pmpaddr_test!(0);
make_pmpaddr_test!(1);
make_pmpaddr_test!(2);
make_pmpaddr_test!(3);
make_pmpaddr_test!(4);
make_pmpaddr_test!(5);
make_pmpaddr_test!(6);
make_pmpaddr_test!(7);
make_pmpaddr_test!(8);
make_pmpaddr_test!(9);
make_pmpaddr_test!(10);
make_pmpaddr_test!(11);
make_pmpaddr_test!(12);
make_pmpaddr_test!(13);
make_pmpaddr_test!(14);
make_pmpaddr_test!(15);
