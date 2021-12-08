#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub resetctrl: crate::Reg<resetctrl::RESETCTRL_SPEC>,
    _reserved1: [u8; 0x24],
    #[doc = "0x30 - VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]"]
    pub bodvbat: crate::Reg<bodvbat::BODVBAT_SPEC>,
    _reserved2: [u8; 0x1c],
    #[doc = "0x50 - Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub comp: crate::Reg<comp::COMP_SPEC>,
    _reserved3: [u8; 0x14],
    #[doc = "0x68 - Allows to identify the Wake-up I/O source from Deep Power Down mode"]
    pub wakeiocause: crate::Reg<wakeiocause::WAKEIOCAUSE_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x74 - FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub statusclk: crate::Reg<statusclk::STATUSCLK_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x84 - General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub aoreg1: crate::Reg<aoreg1::AOREG1_SPEC>,
    _reserved6: [u8; 0x10],
    #[doc = "0x98 - RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub rtcosc32k: crate::Reg<rtcosc32k::RTCOSC32K_SPEC>,
    #[doc = "0x9c - OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub ostimer: crate::Reg<ostimer::OSTIMER_SPEC>,
    _reserved8: [u8; 0x18],
    #[doc = "0xb8 - Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub pdruncfg0: crate::Reg<pdruncfg0::PDRUNCFG0_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0xc0 - Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub pdruncfgset0: crate::Reg<pdruncfgset0::PDRUNCFGSET0_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0xc8 - Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub pdruncfgclr0: crate::Reg<pdruncfgclr0::PDRUNCFGCLR0_SPEC>,
}
#[doc = "RESETCTRL register accessor: an alias for `Reg<RESETCTRL_SPEC>`"]
pub type RESETCTRL = crate::Reg<resetctrl::RESETCTRL_SPEC>;
#[doc = "Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod resetctrl;
#[doc = "BODVBAT register accessor: an alias for `Reg<BODVBAT_SPEC>`"]
pub type BODVBAT = crate::Reg<bodvbat::BODVBAT_SPEC>;
#[doc = "VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]"]
pub mod bodvbat;
#[doc = "COMP register accessor: an alias for `Reg<COMP_SPEC>`"]
pub type COMP = crate::Reg<comp::COMP_SPEC>;
#[doc = "Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod comp;
#[doc = "WAKEIOCAUSE register accessor: an alias for `Reg<WAKEIOCAUSE_SPEC>`"]
pub type WAKEIOCAUSE = crate::Reg<wakeiocause::WAKEIOCAUSE_SPEC>;
#[doc = "Allows to identify the Wake-up I/O source from Deep Power Down mode"]
pub mod wakeiocause;
#[doc = "STATUSCLK register accessor: an alias for `Reg<STATUSCLK_SPEC>`"]
pub type STATUSCLK = crate::Reg<statusclk::STATUSCLK_SPEC>;
#[doc = "FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod statusclk;
#[doc = "AOREG1 register accessor: an alias for `Reg<AOREG1_SPEC>`"]
pub type AOREG1 = crate::Reg<aoreg1::AOREG1_SPEC>;
#[doc = "General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod aoreg1;
#[doc = "RTCOSC32K register accessor: an alias for `Reg<RTCOSC32K_SPEC>`"]
pub type RTCOSC32K = crate::Reg<rtcosc32k::RTCOSC32K_SPEC>;
#[doc = "RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod rtcosc32k;
#[doc = "OSTIMER register accessor: an alias for `Reg<OSTIMER_SPEC>`"]
pub type OSTIMER = crate::Reg<ostimer::OSTIMER_SPEC>;
#[doc = "OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod ostimer;
#[doc = "PDRUNCFG0 register accessor: an alias for `Reg<PDRUNCFG0_SPEC>`"]
pub type PDRUNCFG0 = crate::Reg<pdruncfg0::PDRUNCFG0_SPEC>;
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod pdruncfg0;
#[doc = "PDRUNCFGSET0 register accessor: an alias for `Reg<PDRUNCFGSET0_SPEC>`"]
pub type PDRUNCFGSET0 = crate::Reg<pdruncfgset0::PDRUNCFGSET0_SPEC>;
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod pdruncfgset0;
#[doc = "PDRUNCFGCLR0 register accessor: an alias for `Reg<PDRUNCFGCLR0_SPEC>`"]
pub type PDRUNCFGCLR0 = crate::Reg<pdruncfgclr0::PDRUNCFGCLR0_SPEC>;
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod pdruncfgclr0;
