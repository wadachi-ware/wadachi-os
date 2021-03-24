use bit_field::BitField;
use super::ppn::PPN;
use super::pte::PTE;
use super::pte::Attributes;

/// Page Table Page stores mapping from virtual address to physical address.
/// One page has 1 Ki entries and divides one virtual address into one of them.
/// More detail, see p60 at https://riscv.org/wp-content/uploads/2017/05/riscv-privileged-v1.10.pdf
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct PTP4M {
    entries: [PTE; 1024],
}
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct PTP4K {
    entries: [PTE; 1024],
}

/// Route page allocates entries in units of virtual address 4 MB
impl PTP4M {

    /// Makes a new root PTP on specified address
    ///
    /// # Arguments
    /// * `addr` - the start address of the page. It must be alignmeted by 4 KiB boundary.
    pub fn make(addr: usize) -> &'static mut Self {
        check_alignment_4k_addr(addr);
        unsafe {
            let ptp = addr as *mut PTP4M;
            (*ptp).init()
        }
    }

    /// Iitialize the PTP
    /// Fills the PIP by PTE indicates invalid
    pub fn init(&mut self) -> &mut Self {
        self.entries = [Default::default(); 1024];
        self
    }

    /// Returns the PTE stored the specified index
    #[allow(unused)]
    pub fn get(&self, addr: usize) -> PTE {
        self.entries[vpn1(addr)]
    }

    /// Sets the specified address megapage which starts by the specified PPN
    pub fn set_megapage(&mut self, addr: usize, page: &PPN, attrs: &Attributes) -> &mut Self {
        check_alignment_4m_addr(addr);
        check_alignment_4m_page(page);

        self.entries[vpn1(addr)] = PTE::new(page, attrs);
        self
    }

    /// Sets to distribute the specified address on the subpage
    /// The address that matches the upper 10 bits of the specified address is sorted by the specified PTP using the next 10 bits.
    pub fn set_subpage(&mut self, addr: usize, sub: &PTP4K) -> &mut Self {
        check_alignment_4m_addr(addr);

        let ptr = sub as *const _;
        let page = PPN::from_addr(ptr as usize);
        self.entries[vpn1(addr)] = PTE::new(&page, &Attributes::V);
        self
    }
}

impl PTP4K {
    /// Makes a new leaf PTP on specified address
    ///
    /// # Arguments
    /// * `addr` - the start address of the page. It must be alignmeted by 4 KiB boundary.
    pub fn make(addr: usize) -> &'static mut Self {
        check_alignment_4k_addr(addr);
        unsafe {
            let ptp = addr as *mut PTP4K;
            (*ptp).init()
        }
    }

    /// Iitialize the PTP
    /// Fills the PIP by PTE indicates invalid
    pub fn init(&mut self) -> &mut Self {
        self.entries = [Default::default(); 1024];
        self
    }

    /// Returns the PTE stored the specified index
    #[allow(unused)]
    pub fn get(&self, addr: usize) -> PTE {
        self.entries[vpn0(addr)]
    }

    /// Sets to distribute the specified address on the subpage
    /// The address that matches the 11~20th bit of the specified address is assocciated the specified PPN.
    pub fn set_page(&mut self, addr: usize, page: &PPN, attrs: &Attributes) -> &mut Self {
        check_alignment_4k_addr(addr);

        self.entries[vpn0(addr)] = PTE::new(page, attrs);
        self
    }
}

#[inline]
fn vpn0 (vaddr: usize) -> usize {
    vaddr.get_bits(12..=21)
}

#[inline]
fn vpn1 (vaddr: usize) -> usize {
    vaddr.get_bits(22..=31)
}

#[inline]
fn check_alignment_4k_addr (vaddr: usize) -> () {
    if vaddr.get_bits(0..12) != 0 {
        panic!("address alignment 4 KB error: {:08X}", vaddr);
    }
}

#[inline]
fn check_alignment_4m_addr (vaddr: usize) -> () {
    if vaddr.get_bits(0..22) != 0  {
        panic!("address alignment 4 MB error: {:08X}", vaddr);
    }
}

#[inline]
fn check_alignment_4m_page (page: &PPN) -> () {
    if page.get_0() != 0 {
        panic!("page alignment 4 MB error: {:?}", page);
    }
}
