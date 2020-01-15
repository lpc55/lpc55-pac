#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub resetctrl: RESETCTRL,
    _reserved1: [u8; 36usize],
    #[doc = "0x30 - VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]"]
    pub bodvbat: BODVBAT,
    _reserved2: [u8; 28usize],
    #[doc = "0x50 - Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub comp: COMP,
    _reserved3: [u8; 20usize],
    #[doc = "0x68 - Allows to identify the Wake-up I/O source from Deep Power Down mode"]
    pub wakeiocause: WAKEIOCAUSE,
    _reserved4: [u8; 8usize],
    #[doc = "0x74 - FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub statusclk: STATUSCLK,
    _reserved5: [u8; 12usize],
    #[doc = "0x84 - General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub aoreg1: AOREG1,
    _reserved6: [u8; 16usize],
    #[doc = "0x98 - RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub rtcosc32k: RTCOSC32K,
    #[doc = "0x9c - OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub ostimer: OSTIMER,
    _reserved8: [u8; 24usize],
    #[doc = "0xb8 - Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub pdruncfg0: PDRUNCFG0,
    _reserved9: [u8; 4usize],
    #[doc = "0xc0 - Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub pdruncfgset0: PDRUNCFGSET0,
    _reserved10: [u8; 4usize],
    #[doc = "0xc8 - Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub pdruncfgclr0: PDRUNCFGCLR0,
}
#[doc = "Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetctrl](resetctrl) module"]
pub type RESETCTRL = crate::Reg<u32, _RESETCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESETCTRL;
#[doc = "`read()` method returns [resetctrl::R](resetctrl::R) reader structure"]
impl crate::Readable for RESETCTRL {}
#[doc = "`write(|w| ..)` method takes [resetctrl::W](resetctrl::W) writer structure"]
impl crate::Writable for RESETCTRL {}
#[doc = "Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod resetctrl;
#[doc = "VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodvbat](bodvbat) module"]
pub type BODVBAT = crate::Reg<u32, _BODVBAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BODVBAT;
#[doc = "`read()` method returns [bodvbat::R](bodvbat::R) reader structure"]
impl crate::Readable for BODVBAT {}
#[doc = "`write(|w| ..)` method takes [bodvbat::W](bodvbat::W) writer structure"]
impl crate::Writable for BODVBAT {}
#[doc = "VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]"]
pub mod bodvbat;
#[doc = "Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp](comp) module"]
pub type COMP = crate::Reg<u32, _COMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP;
#[doc = "`read()` method returns [comp::R](comp::R) reader structure"]
impl crate::Readable for COMP {}
#[doc = "`write(|w| ..)` method takes [comp::W](comp::W) writer structure"]
impl crate::Writable for COMP {}
#[doc = "Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod comp;
#[doc = "Allows to identify the Wake-up I/O source from Deep Power Down mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeiocause](wakeiocause) module"]
pub type WAKEIOCAUSE = crate::Reg<u32, _WAKEIOCAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKEIOCAUSE;
#[doc = "`read()` method returns [wakeiocause::R](wakeiocause::R) reader structure"]
impl crate::Readable for WAKEIOCAUSE {}
#[doc = "`write(|w| ..)` method takes [wakeiocause::W](wakeiocause::W) writer structure"]
impl crate::Writable for WAKEIOCAUSE {}
#[doc = "Allows to identify the Wake-up I/O source from Deep Power Down mode"]
pub mod wakeiocause;
#[doc = "FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusclk](statusclk) module"]
pub type STATUSCLK = crate::Reg<u32, _STATUSCLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUSCLK;
#[doc = "`read()` method returns [statusclk::R](statusclk::R) reader structure"]
impl crate::Readable for STATUSCLK {}
#[doc = "`write(|w| ..)` method takes [statusclk::W](statusclk::W) writer structure"]
impl crate::Writable for STATUSCLK {}
#[doc = "FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod statusclk;
#[doc = "General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aoreg1](aoreg1) module"]
pub type AOREG1 = crate::Reg<u32, _AOREG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AOREG1;
#[doc = "`read()` method returns [aoreg1::R](aoreg1::R) reader structure"]
impl crate::Readable for AOREG1 {}
#[doc = "`write(|w| ..)` method takes [aoreg1::W](aoreg1::W) writer structure"]
impl crate::Writable for AOREG1 {}
#[doc = "General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod aoreg1;
#[doc = "RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcosc32k](rtcosc32k) module"]
pub type RTCOSC32K = crate::Reg<u32, _RTCOSC32K>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCOSC32K;
#[doc = "`read()` method returns [rtcosc32k::R](rtcosc32k::R) reader structure"]
impl crate::Readable for RTCOSC32K {}
#[doc = "`write(|w| ..)` method takes [rtcosc32k::W](rtcosc32k::W) writer structure"]
impl crate::Writable for RTCOSC32K {}
#[doc = "RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod rtcosc32k;
#[doc = "OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ostimer](ostimer) module"]
pub type OSTIMER = crate::Reg<u32, _OSTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSTIMER;
#[doc = "`read()` method returns [ostimer::R](ostimer::R) reader structure"]
impl crate::Readable for OSTIMER {}
#[doc = "`write(|w| ..)` method takes [ostimer::W](ostimer::W) writer structure"]
impl crate::Writable for OSTIMER {}
#[doc = "OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod ostimer;
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfg0](pdruncfg0) module"]
pub type PDRUNCFG0 = crate::Reg<u32, _PDRUNCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDRUNCFG0;
#[doc = "`read()` method returns [pdruncfg0::R](pdruncfg0::R) reader structure"]
impl crate::Readable for PDRUNCFG0 {}
#[doc = "`write(|w| ..)` method takes [pdruncfg0::W](pdruncfg0::W) writer structure"]
impl crate::Writable for PDRUNCFG0 {}
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod pdruncfg0;
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfgset0](pdruncfgset0) module"]
pub type PDRUNCFGSET0 = crate::Reg<u32, _PDRUNCFGSET0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDRUNCFGSET0;
#[doc = "`write(|w| ..)` method takes [pdruncfgset0::W](pdruncfgset0::W) writer structure"]
impl crate::Writable for PDRUNCFGSET0 {}
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod pdruncfgset0;
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfgclr0](pdruncfgclr0) module"]
pub type PDRUNCFGCLR0 = crate::Reg<u32, _PDRUNCFGCLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDRUNCFGCLR0;
#[doc = "`write(|w| ..)` method takes [pdruncfgclr0::W](pdruncfgclr0::W) writer structure"]
impl crate::Writable for PDRUNCFGCLR0 {}
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod pdruncfgclr0;
