#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub resetctrl: RESETCTRL,
    #[doc = "0x0c - Reset Cause register \\[Reset by: PoR\\]"]
    pub resetcause: RESETCAUSE,
    _reserved2: [u8; 32usize],
    #[doc = "0x30 - VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]"]
    pub bodvbat: BODVBAT,
    _reserved3: [u8; 4usize],
    #[doc = "0x38 - Digital Core logic Brown Out Dectector control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub bodcore: BODCORE,
    _reserved4: [u8; 8usize],
    #[doc = "0x44 - 1 MHz Free Running Oscillator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub fro1m: FRO1M,
    #[doc = "0x48 - 32 KHz Free Running Oscillator (FRO) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub fro32k: FRO32K,
    #[doc = "0x4c - 32 KHz Crystal oscillator (XTAL) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub xtal32k: XTAL32K,
    #[doc = "0x50 - Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub comp: COMP,
    _reserved8: [u8; 20usize],
    #[doc = "0x68 - Allows to identify the Wake-up I/O source from Deep Power Down mode"]
    pub wakeiocause: WAKEIOCAUSE,
    _reserved9: [u8; 8usize],
    #[doc = "0x74 - FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub statusclk: STATUSCLK,
    _reserved10: [u8; 12usize],
    #[doc = "0x84 - General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub aoreg1: AOREG1,
    _reserved11: [u8; 16usize],
    #[doc = "0x98 - RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub rtcosc32k: RTCOSC32K,
    #[doc = "0x9c - OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub ostimer: OSTIMER,
    _reserved13: [u8; 16usize],
    #[doc = "0xb0 - Controls the power to various modules during Low Power modes - DEEP SLEEP, POWER DOWN and DEEP POWER DOWN \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Software Reset\\]"]
    pub pdsleepcfg0: PDSLEEPCFG0,
    _reserved14: [u8; 4usize],
    #[doc = "0xb8 - Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub pdruncfg0: PDRUNCFG0,
    _reserved15: [u8; 4usize],
    #[doc = "0xc0 - Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub pdruncfgset0: PDRUNCFGSET0,
    _reserved16: [u8; 4usize],
    #[doc = "0xc8 - Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub pdruncfgclr0: PDRUNCFGCLR0,
}
#[doc = "Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [resetctrl](resetctrl) module"]
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
#[doc = "Reset Cause register \\[Reset by: PoR\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [resetcause](resetcause) module"]
pub type RESETCAUSE = crate::Reg<u32, _RESETCAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESETCAUSE;
#[doc = "`read()` method returns [resetcause::R](resetcause::R) reader structure"]
impl crate::Readable for RESETCAUSE {}
#[doc = "`write(|w| ..)` method takes [resetcause::W](resetcause::W) writer structure"]
impl crate::Writable for RESETCAUSE {}
#[doc = "Reset Cause register \\[Reset by: PoR\\]"]
pub mod resetcause;
#[doc = "VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bodvbat](bodvbat) module"]
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
#[doc = "Digital Core logic Brown Out Dectector control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bodcore](bodcore) module"]
pub type BODCORE = crate::Reg<u32, _BODCORE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BODCORE;
#[doc = "`read()` method returns [bodcore::R](bodcore::R) reader structure"]
impl crate::Readable for BODCORE {}
#[doc = "`write(|w| ..)` method takes [bodcore::W](bodcore::W) writer structure"]
impl crate::Writable for BODCORE {}
#[doc = "Digital Core logic Brown Out Dectector control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod bodcore;
#[doc = "1 MHz Free Running Oscillator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fro1m](fro1m) module"]
pub type FRO1M = crate::Reg<u32, _FRO1M>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRO1M;
#[doc = "`read()` method returns [fro1m::R](fro1m::R) reader structure"]
impl crate::Readable for FRO1M {}
#[doc = "`write(|w| ..)` method takes [fro1m::W](fro1m::W) writer structure"]
impl crate::Writable for FRO1M {}
#[doc = "1 MHz Free Running Oscillator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod fro1m;
#[doc = "32 KHz Free Running Oscillator (FRO) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fro32k](fro32k) module"]
pub type FRO32K = crate::Reg<u32, _FRO32K>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRO32K;
#[doc = "`read()` method returns [fro32k::R](fro32k::R) reader structure"]
impl crate::Readable for FRO32K {}
#[doc = "`write(|w| ..)` method takes [fro32k::W](fro32k::W) writer structure"]
impl crate::Writable for FRO32K {}
#[doc = "32 KHz Free Running Oscillator (FRO) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod fro32k;
#[doc = "32 KHz Crystal oscillator (XTAL) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xtal32k](xtal32k) module"]
pub type XTAL32K = crate::Reg<u32, _XTAL32K>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XTAL32K;
#[doc = "`read()` method returns [xtal32k::R](xtal32k::R) reader structure"]
impl crate::Readable for XTAL32K {}
#[doc = "`write(|w| ..)` method takes [xtal32k::W](xtal32k::W) writer structure"]
impl crate::Writable for XTAL32K {}
#[doc = "32 KHz Crystal oscillator (XTAL) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod xtal32k;
#[doc = "Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comp](comp) module"]
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
#[doc = "Allows to identify the Wake-up I/O source from Deep Power Down mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wakeiocause](wakeiocause) module"]
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
#[doc = "FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [statusclk](statusclk) module"]
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
#[doc = "General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aoreg1](aoreg1) module"]
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
#[doc = "RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtcosc32k](rtcosc32k) module"]
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
#[doc = "OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ostimer](ostimer) module"]
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
#[doc = "Controls the power to various modules during Low Power modes - DEEP SLEEP, POWER DOWN and DEEP POWER DOWN \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdsleepcfg0](pdsleepcfg0) module"]
pub type PDSLEEPCFG0 = crate::Reg<u32, _PDSLEEPCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSLEEPCFG0;
#[doc = "`read()` method returns [pdsleepcfg0::R](pdsleepcfg0::R) reader structure"]
impl crate::Readable for PDSLEEPCFG0 {}
#[doc = "`write(|w| ..)` method takes [pdsleepcfg0::W](pdsleepcfg0::W) writer structure"]
impl crate::Writable for PDSLEEPCFG0 {}
#[doc = "Controls the power to various modules during Low Power modes - DEEP SLEEP, POWER DOWN and DEEP POWER DOWN \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Software Reset\\]"]
pub mod pdsleepcfg0;
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdruncfg0](pdruncfg0) module"]
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
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdruncfgset0](pdruncfgset0) module"]
pub type PDRUNCFGSET0 = crate::Reg<u32, _PDRUNCFGSET0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDRUNCFGSET0;
#[doc = "`write(|w| ..)` method takes [pdruncfgset0::W](pdruncfgset0::W) writer structure"]
impl crate::Writable for PDRUNCFGSET0 {}
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod pdruncfgset0;
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdruncfgclr0](pdruncfgclr0) module"]
pub type PDRUNCFGCLR0 = crate::Reg<u32, _PDRUNCFGCLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDRUNCFGCLR0;
#[doc = "`write(|w| ..)` method takes [pdruncfgclr0::W](pdruncfgclr0::W) writer structure"]
impl crate::Writable for PDRUNCFGCLR0 {}
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod pdruncfgclr0;
