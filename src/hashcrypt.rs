#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register to enable and operate Hash and Crypto"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Indicates status of Hash peripheral."]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - Write 1 to enable interrupts; reads back with which are set."]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x0c - Write 1 to clear interrupts."]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x10 - Setup Master to access memory (if available)"]
    pub memctrl: crate::Reg<memctrl::MEMCTRL_SPEC>,
    #[doc = "0x14 - Address to start memory access from (if available)."]
    pub memaddr: crate::Reg<memaddr::MEMADDR_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - Input of 16 words at a time to load up buffer."]
    pub indata: crate::Reg<indata::INDATA_SPEC>,
    #[doc = "0x24..0x40 - no description available"]
    pub alias: [crate::Reg<alias::ALIAS_SPEC>; 7],
    #[doc = "0x40..0x60 - no description available"]
    pub digest0: [crate::Reg<digest0::DIGEST0_SPEC>; 8],
    _reserved9: [u8; 0x20],
    #[doc = "0x80 - Crypto settings for AES and Salsa and ChaCha"]
    pub cryptcfg: crate::Reg<cryptcfg::CRYPTCFG_SPEC>,
    #[doc = "0x84 - Returns the configuration of this block in this chip - indicates what services are available."]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x8c - Lock register allows locking to the current security level or unlocking by the lock holding level."]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
    #[doc = "0x90..0xa0 - no description available"]
    pub mask: [crate::Reg<mask::MASK_SPEC>; 4],
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register to enable and operate Hash and Crypto"]
pub mod ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Indicates status of Hash peripheral."]
pub mod status;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Write 1 to enable interrupts; reads back with which are set."]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Write 1 to clear interrupts."]
pub mod intenclr;
#[doc = "MEMCTRL register accessor: an alias for `Reg<MEMCTRL_SPEC>`"]
pub type MEMCTRL = crate::Reg<memctrl::MEMCTRL_SPEC>;
#[doc = "Setup Master to access memory (if available)"]
pub mod memctrl;
#[doc = "MEMADDR register accessor: an alias for `Reg<MEMADDR_SPEC>`"]
pub type MEMADDR = crate::Reg<memaddr::MEMADDR_SPEC>;
#[doc = "Address to start memory access from (if available)."]
pub mod memaddr;
#[doc = "INDATA register accessor: an alias for `Reg<INDATA_SPEC>`"]
pub type INDATA = crate::Reg<indata::INDATA_SPEC>;
#[doc = "Input of 16 words at a time to load up buffer."]
pub mod indata;
#[doc = "ALIAS register accessor: an alias for `Reg<ALIAS_SPEC>`"]
pub type ALIAS = crate::Reg<alias::ALIAS_SPEC>;
#[doc = "no description available"]
pub mod alias;
#[doc = "DIGEST0 register accessor: an alias for `Reg<DIGEST0_SPEC>`"]
pub type DIGEST0 = crate::Reg<digest0::DIGEST0_SPEC>;
#[doc = "no description available"]
pub mod digest0;
#[doc = "CRYPTCFG register accessor: an alias for `Reg<CRYPTCFG_SPEC>`"]
pub type CRYPTCFG = crate::Reg<cryptcfg::CRYPTCFG_SPEC>;
#[doc = "Crypto settings for AES and Salsa and ChaCha"]
pub mod cryptcfg;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Returns the configuration of this block in this chip - indicates what services are available."]
pub mod config;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Lock register allows locking to the current security level or unlocking by the lock holding level."]
pub mod lock;
#[doc = "MASK register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "no description available"]
pub mod mask;
