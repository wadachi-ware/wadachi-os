use super::ppn::PPN;
use bit_field::BitField;

#[allow(unused)]
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct PTE {
    // Page Table Entry
    // see p59 at https://riscv.org/wp-content/uploads/2017/05/riscv-privileged-v1.10.pdf
    value: usize,
}

// bit-field position
const V: usize = 0; // valid
const R: usize = 1; // readable
const W: usize = 2; // writable
const X: usize = 3; // executable
const U: usize = 4; // accessibile in user mode
const G: usize = 5; // global mapping
const A: usize = 6; // accessed
const D: usize = 7; // dirty

impl PTE {

    #[allow(unused)]
    #[inline]
    pub fn new(ppn: PPN) -> Self {
        PTE {value: ppn.to_page() << 10}
    }

    #[allow(unused)]
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.value.get_bit(V)
    }

    #[allow(unused)]
    #[inline]
    pub fn set_valid(&mut self, value: bool) -> &mut Self {
        self.value.set_bit(V, value);
        self
    }

    #[allow(unused)]
    #[inline]
    pub fn can_read(&self) -> bool {
        self.value.get_bit(R)
    }

    #[allow(unused)]
    #[inline]
    pub fn set_read(&mut self, value: bool) -> &mut Self {
        self.value.set_bit(R, value);
        self
    }

    #[allow(unused)]
    #[inline]
    pub fn can_write(&self) -> bool {
        self.value.get_bit(W)
    }

    #[allow(unused)]
    #[inline]
    pub fn set_write(&mut self, value: bool) -> &mut Self {
        self.value.set_bit(W, value);
        self
    }

    #[allow(unused)]
    #[inline]
    pub fn can_execute(&self) -> bool {
        self.value.get_bit(X)
    }

    #[allow(unused)]
    #[inline]
    pub fn set_execute(&mut self, value: bool) -> &mut Self {
        self.value.set_bit(X, value);
        self
    }

    #[allow(unused)]
    #[inline]
    pub fn is_user(&self) -> bool {
        self.value.get_bit(U)
    }

    #[allow(unused)]
    #[inline]
    pub fn set_user(&mut self, value: bool) -> &mut Self {
        self.value.set_bit(U, value);
        self
    }

    #[allow(unused)]
    #[inline]
    pub fn is_global(&self) -> bool {
        self.value.get_bit(G)
    }

    #[allow(unused)]
    #[inline]
    pub fn set_global(&mut self, value: bool) -> &mut Self {
        self.value.set_bit(G, value);
        self
    }

    #[allow(unused)]
    #[inline]
    pub fn check_accessed(&self) -> bool {
        self.value.get_bit(A)
    }

    #[allow(unused)]
    #[inline]
    pub fn clear_accessed(&mut self) -> &mut Self {
        self.value.set_bit(A, false);
        self
    }

    #[allow(unused)]
    #[inline]
    pub fn check_dirty(&self) -> bool {
        self.value.get_bit(D)
    }

    #[allow(unused)]
    #[inline]
    pub fn clear_dirty(&mut self) -> &mut Self {
        self.value.set_bit(D, false);
        self
    }
}
