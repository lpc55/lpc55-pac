#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC mode register"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x04 - CRC seed register"]
    pub seed: crate::Reg<seed::SEED_SPEC>,
    _reserved_2_sum: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x08 - CRC data register"]
    #[inline(always)]
    pub fn wr_data(&self) -> &crate::Reg<wr_data::WR_DATA_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<wr_data::WR_DATA_SPEC>)
        }
    }
    #[doc = "0x08 - CRC checksum register"]
    #[inline(always)]
    pub fn sum(&self) -> &crate::Reg<sum::SUM_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize) as *const crate::Reg<sum::SUM_SPEC>)
        }
    }
}
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "CRC mode register"]
pub mod mode;
#[doc = "SEED register accessor: an alias for `Reg<SEED_SPEC>`"]
pub type SEED = crate::Reg<seed::SEED_SPEC>;
#[doc = "CRC seed register"]
pub mod seed;
#[doc = "SUM register accessor: an alias for `Reg<SUM_SPEC>`"]
pub type SUM = crate::Reg<sum::SUM_SPEC>;
#[doc = "CRC checksum register"]
pub mod sum;
#[doc = "WR_DATA register accessor: an alias for `Reg<WR_DATA_SPEC>`"]
pub type WR_DATA = crate::Reg<wr_data::WR_DATA_SPEC>;
#[doc = "CRC data register"]
pub mod wr_data;
