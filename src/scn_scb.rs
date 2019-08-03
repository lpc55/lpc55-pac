#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 12usize],
    #[doc = "0x0c - Coprocessor Power Control Register"]
    pub cppwr: CPPWR,
}
#[doc = "Coprocessor Power Control Register"]
pub struct CPPWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Coprocessor Power Control Register"]
pub mod cppwr;
