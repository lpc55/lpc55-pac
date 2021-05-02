#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - no description available"]
    pub b: [B; 2],
    _reserved1: [u8; 4032usize],
    #[doc = "0x1000 - no description available"]
    pub w: [W; 2],
    _reserved2: [u8; 3840usize],
    #[doc = "0x2000 - Direction registers for all port GPIO pins"]
    pub dir: [crate::Reg<dir::DIR_SPEC>; 2],
    _reserved3: [u8; 120usize],
    #[doc = "0x2080 - Mask register for all port GPIO pins"]
    pub mask: [crate::Reg<mask::MASK_SPEC>; 2],
    _reserved4: [u8; 120usize],
    #[doc = "0x2100 - Port pin register for all port GPIO pins"]
    pub pin: [crate::Reg<pin::PIN_SPEC>; 2],
    _reserved5: [u8; 120usize],
    #[doc = "0x2180 - Masked port register for all port GPIO pins"]
    pub mpin: [crate::Reg<mpin::MPIN_SPEC>; 2],
    _reserved6: [u8; 120usize],
    #[doc = "0x2200 - Write: Set register for port. Read: output bits for port"]
    pub set: [crate::Reg<set::SET_SPEC>; 2],
    _reserved7: [u8; 120usize],
    #[doc = "0x2280 - Clear port for all port GPIO pins"]
    pub clr: [crate::Reg<clr::CLR_SPEC>; 2],
    _reserved8: [u8; 120usize],
    #[doc = "0x2300 - Toggle port for all port GPIO pins"]
    pub not: [crate::Reg<not::NOT_SPEC>; 2],
    _reserved9: [u8; 120usize],
    #[doc = "0x2380 - Set pin direction bits for port"]
    pub dirset: [crate::Reg<dirset::DIRSET_SPEC>; 2],
    _reserved10: [u8; 120usize],
    #[doc = "0x2400 - Clear pin direction bits for port"]
    pub dirclr: [crate::Reg<dirclr::DIRCLR_SPEC>; 2],
    _reserved11: [u8; 120usize],
    #[doc = "0x2480 - Toggle pin direction bits for port"]
    pub dirnot: [crate::Reg<dirnot::DIRNOT_SPEC>; 2],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct B {
    #[doc = "0x00 - Byte pin registers for all port GPIO pins"]
    pub b_: [crate::Reg<self::b::b_::B__SPEC>; 32],
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod b;
#[doc = r"Register block"]
#[repr(C)]
pub struct W {
    #[doc = "0x00 - Word pin registers for all port GPIO pins"]
    pub w_: [crate::Reg<self::w::w_::W__SPEC>; 32],
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod w;
#[doc = "DIR register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "Direction registers for all port GPIO pins"]
pub mod dir;
#[doc = "MASK register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "Mask register for all port GPIO pins"]
pub mod mask;
#[doc = "PIN register accessor: an alias for `Reg<PIN_SPEC>`"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "Port pin register for all port GPIO pins"]
pub mod pin;
#[doc = "MPIN register accessor: an alias for `Reg<MPIN_SPEC>`"]
pub type MPIN = crate::Reg<mpin::MPIN_SPEC>;
#[doc = "Masked port register for all port GPIO pins"]
pub mod mpin;
#[doc = "SET register accessor: an alias for `Reg<SET_SPEC>`"]
pub type SET = crate::Reg<set::SET_SPEC>;
#[doc = "Write: Set register for port. Read: output bits for port"]
pub mod set;
#[doc = "CLR register accessor: an alias for `Reg<CLR_SPEC>`"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "Clear port for all port GPIO pins"]
pub mod clr;
#[doc = "NOT register accessor: an alias for `Reg<NOT_SPEC>`"]
pub type NOT = crate::Reg<not::NOT_SPEC>;
#[doc = "Toggle port for all port GPIO pins"]
pub mod not;
#[doc = "DIRSET register accessor: an alias for `Reg<DIRSET_SPEC>`"]
pub type DIRSET = crate::Reg<dirset::DIRSET_SPEC>;
#[doc = "Set pin direction bits for port"]
pub mod dirset;
#[doc = "DIRCLR register accessor: an alias for `Reg<DIRCLR_SPEC>`"]
pub type DIRCLR = crate::Reg<dirclr::DIRCLR_SPEC>;
#[doc = "Clear pin direction bits for port"]
pub mod dirclr;
#[doc = "DIRNOT register accessor: an alias for `Reg<DIRNOT_SPEC>`"]
pub type DIRNOT = crate::Reg<dirnot::DIRNOT_SPEC>;
#[doc = "Toggle pin direction bits for port"]
pub mod dirnot;
