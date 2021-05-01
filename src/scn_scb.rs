#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 12usize],
    #[doc = "0x0c - Coprocessor Power Control Register"]
    pub cppwr: crate::Reg<cppwr::CPPWR_SPEC>,
}
#[doc = "CPPWR register accessor: an alias for `Reg<CPPWR_SPEC>`"]
pub type CPPWR = crate::Reg<cppwr::CPPWR_SPEC>;
#[doc = "Coprocessor Power Control Register"]
pub mod cppwr;
