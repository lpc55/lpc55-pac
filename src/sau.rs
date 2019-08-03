#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 208usize],
    #[doc = "0xd0 - Security Attribution Unit Control Register"]
    pub sau_ctrl: SAU_CTRL,
    #[doc = "0xd4 - Security Attribution Unit Type Register"]
    pub sau_type: SAU_TYPE,
    #[doc = "0xd8 - Security Attribution Unit Region Number Register"]
    pub sau_rnr: SAU_RNR,
    #[doc = "0xdc - Security Attribution Unit Region Base Address Register"]
    pub sau_rbar: SAU_RBAR,
    #[doc = "0xe0 - Security Attribution Unit Region Limit Address Register"]
    pub sau_rlar: SAU_RLAR,
    #[doc = "0xe4 - Secure Fault Status Register"]
    pub sfsr: SFSR,
    #[doc = "0xe8 - Secure Fault Address Register"]
    pub sfar: SFAR,
}
#[doc = "Security Attribution Unit Control Register"]
pub struct SAU_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security Attribution Unit Control Register"]
pub mod sau_ctrl;
#[doc = "Security Attribution Unit Type Register"]
pub struct SAU_TYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security Attribution Unit Type Register"]
pub mod sau_type;
#[doc = "Security Attribution Unit Region Number Register"]
pub struct SAU_RNR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security Attribution Unit Region Number Register"]
pub mod sau_rnr;
#[doc = "Security Attribution Unit Region Base Address Register"]
pub struct SAU_RBAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security Attribution Unit Region Base Address Register"]
pub mod sau_rbar;
#[doc = "Security Attribution Unit Region Limit Address Register"]
pub struct SAU_RLAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security Attribution Unit Region Limit Address Register"]
pub mod sau_rlar;
#[doc = "Secure Fault Status Register"]
pub struct SFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Secure Fault Status Register"]
pub mod sfsr;
#[doc = "Secure Fault Address Register"]
pub struct SFAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Secure Fault Address Register"]
pub mod sfar;
