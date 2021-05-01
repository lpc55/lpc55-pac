#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Security Attribution Unit Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Security Attribution Unit Type Register"]
    pub type_: crate::Reg<type_::TYPE_SPEC>,
    #[doc = "0x08 - Security Attribution Unit Region Number Register"]
    pub rnr: crate::Reg<rnr::RNR_SPEC>,
    #[doc = "0x0c - Security Attribution Unit Region Base Address Register"]
    pub rbar: crate::Reg<rbar::RBAR_SPEC>,
    #[doc = "0x10 - Security Attribution Unit Region Limit Address Register"]
    pub rlar: crate::Reg<rlar::RLAR_SPEC>,
    #[doc = "0x14 - Secure Fault Status Register"]
    pub sfsr: crate::Reg<sfsr::SFSR_SPEC>,
    #[doc = "0x18 - Secure Fault Address Register"]
    pub sfar: crate::Reg<sfar::SFAR_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Security Attribution Unit Control Register"]
pub mod ctrl;
#[doc = "TYPE register accessor: an alias for `Reg<TYPE_SPEC>`"]
pub type TYPE = crate::Reg<type_::TYPE_SPEC>;
#[doc = "Security Attribution Unit Type Register"]
pub mod type_;
#[doc = "RNR register accessor: an alias for `Reg<RNR_SPEC>`"]
pub type RNR = crate::Reg<rnr::RNR_SPEC>;
#[doc = "Security Attribution Unit Region Number Register"]
pub mod rnr;
#[doc = "RBAR register accessor: an alias for `Reg<RBAR_SPEC>`"]
pub type RBAR = crate::Reg<rbar::RBAR_SPEC>;
#[doc = "Security Attribution Unit Region Base Address Register"]
pub mod rbar;
#[doc = "RLAR register accessor: an alias for `Reg<RLAR_SPEC>`"]
pub type RLAR = crate::Reg<rlar::RLAR_SPEC>;
#[doc = "Security Attribution Unit Region Limit Address Register"]
pub mod rlar;
#[doc = "SFSR register accessor: an alias for `Reg<SFSR_SPEC>`"]
pub type SFSR = crate::Reg<sfsr::SFSR_SPEC>;
#[doc = "Secure Fault Status Register"]
pub mod sfsr;
#[doc = "SFAR register accessor: an alias for `Reg<SFAR_SPEC>`"]
pub type SFAR = crate::Reg<sfar::SFAR_SPEC>;
#[doc = "Secure Fault Address Register"]
pub mod sfar;
