#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PUF Control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - PUF Key Index register"]
    pub keyindex: KEYINDEX,
    #[doc = "0x08 - PUF Key Size register"]
    pub keysize: KEYSIZE,
    _reserved3: [u8; 20usize],
    #[doc = "0x20 - PUF Status register"]
    pub stat: STAT,
    _reserved4: [u8; 4usize],
    #[doc = "0x28 - PUF Allow register"]
    pub allow: ALLOW,
    _reserved5: [u8; 20usize],
    #[doc = "0x40 - PUF Key Input register"]
    pub keyinput: KEYINPUT,
    #[doc = "0x44 - PUF Code Input register"]
    pub codeinput: CODEINPUT,
    #[doc = "0x48 - PUF Code Output register"]
    pub codeoutput: CODEOUTPUT,
    _reserved8: [u8; 20usize],
    #[doc = "0x60 - PUF Key Output Index register"]
    pub keyoutindex: KEYOUTINDEX,
    #[doc = "0x64 - PUF Key Output register"]
    pub keyoutput: KEYOUTPUT,
    _reserved10: [u8; 116usize],
    #[doc = "0xdc - PUF Interface Status and clear register"]
    pub ifstat: IFSTAT,
    _reserved11: [u8; 28usize],
    #[doc = "0xfc - PUF version register."]
    pub version: VERSION,
    #[doc = "0x100 - PUF Interrupt Enable"]
    pub inten: INTEN,
    #[doc = "0x104 - PUF interrupt status"]
    pub intstat: INTSTAT,
    #[doc = "0x108 - PUF RAM Power Control"]
    pub pwrctrl: PWRCTRL,
    #[doc = "0x10c - PUF config register for block bits"]
    pub cfg: CFG,
    _reserved16: [u8; 240usize],
    #[doc = "0x200 - Only reset in case of full IC reset"]
    pub keylock: KEYLOCK,
    #[doc = "0x204 - no description available"]
    pub keyenable: KEYENABLE,
    #[doc = "0x208 - Reinitialize Keys shift registers counters"]
    pub keyreset: KEYRESET,
    #[doc = "0x20c - no description available"]
    pub idxblk_l: IDXBLK_L,
    #[doc = "0x210 - no description available"]
    pub idxblk_h_dp: IDXBLK_H_DP,
    #[doc = "0x214 - Only reset in case of full IC reset"]
    pub keymask: [KEYMASK; 4],
    _reserved22: [u8; 48usize],
    #[doc = "0x254 - no description available"]
    pub idxblk_h: IDXBLK_H,
    #[doc = "0x258 - no description available"]
    pub idxblk_l_dp: IDXBLK_L_DP,
    #[doc = "0x25c - no description available"]
    pub shift_status: SHIFT_STATUS,
}
#[doc = "PUF Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "PUF Control register"]
pub mod ctrl;
#[doc = "PUF Key Index register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyindex](keyindex) module"]
pub type KEYINDEX = crate::Reg<u32, _KEYINDEX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYINDEX;
#[doc = "`read()` method returns [keyindex::R](keyindex::R) reader structure"]
impl crate::Readable for KEYINDEX {}
#[doc = "`write(|w| ..)` method takes [keyindex::W](keyindex::W) writer structure"]
impl crate::Writable for KEYINDEX {}
#[doc = "PUF Key Index register"]
pub mod keyindex;
#[doc = "PUF Key Size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keysize](keysize) module"]
pub type KEYSIZE = crate::Reg<u32, _KEYSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYSIZE;
#[doc = "`read()` method returns [keysize::R](keysize::R) reader structure"]
impl crate::Readable for KEYSIZE {}
#[doc = "`write(|w| ..)` method takes [keysize::W](keysize::W) writer structure"]
impl crate::Writable for KEYSIZE {}
#[doc = "PUF Key Size register"]
pub mod keysize;
#[doc = "PUF Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "PUF Status register"]
pub mod stat;
#[doc = "PUF Allow register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [allow](allow) module"]
pub type ALLOW = crate::Reg<u32, _ALLOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALLOW;
#[doc = "`read()` method returns [allow::R](allow::R) reader structure"]
impl crate::Readable for ALLOW {}
#[doc = "`write(|w| ..)` method takes [allow::W](allow::W) writer structure"]
impl crate::Writable for ALLOW {}
#[doc = "PUF Allow register"]
pub mod allow;
#[doc = "PUF Key Input register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyinput](keyinput) module"]
pub type KEYINPUT = crate::Reg<u32, _KEYINPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYINPUT;
#[doc = "`write(|w| ..)` method takes [keyinput::W](keyinput::W) writer structure"]
impl crate::Writable for KEYINPUT {}
#[doc = "PUF Key Input register"]
pub mod keyinput;
#[doc = "PUF Code Input register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [codeinput](codeinput) module"]
pub type CODEINPUT = crate::Reg<u32, _CODEINPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CODEINPUT;
#[doc = "`write(|w| ..)` method takes [codeinput::W](codeinput::W) writer structure"]
impl crate::Writable for CODEINPUT {}
#[doc = "PUF Code Input register"]
pub mod codeinput;
#[doc = "PUF Code Output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [codeoutput](codeoutput) module"]
pub type CODEOUTPUT = crate::Reg<u32, _CODEOUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CODEOUTPUT;
#[doc = "`read()` method returns [codeoutput::R](codeoutput::R) reader structure"]
impl crate::Readable for CODEOUTPUT {}
#[doc = "PUF Code Output register"]
pub mod codeoutput;
#[doc = "PUF Key Output Index register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyoutindex](keyoutindex) module"]
pub type KEYOUTINDEX = crate::Reg<u32, _KEYOUTINDEX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYOUTINDEX;
#[doc = "`read()` method returns [keyoutindex::R](keyoutindex::R) reader structure"]
impl crate::Readable for KEYOUTINDEX {}
#[doc = "`write(|w| ..)` method takes [keyoutindex::W](keyoutindex::W) writer structure"]
impl crate::Writable for KEYOUTINDEX {}
#[doc = "PUF Key Output Index register"]
pub mod keyoutindex;
#[doc = "PUF Key Output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyoutput](keyoutput) module"]
pub type KEYOUTPUT = crate::Reg<u32, _KEYOUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYOUTPUT;
#[doc = "`read()` method returns [keyoutput::R](keyoutput::R) reader structure"]
impl crate::Readable for KEYOUTPUT {}
#[doc = "PUF Key Output register"]
pub mod keyoutput;
#[doc = "PUF Interface Status and clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifstat](ifstat) module"]
pub type IFSTAT = crate::Reg<u32, _IFSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFSTAT;
#[doc = "`read()` method returns [ifstat::R](ifstat::R) reader structure"]
impl crate::Readable for IFSTAT {}
#[doc = "`write(|w| ..)` method takes [ifstat::W](ifstat::W) writer structure"]
impl crate::Writable for IFSTAT {}
#[doc = "PUF Interface Status and clear register"]
pub mod ifstat;
#[doc = "PUF version register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "PUF version register."]
pub mod version;
#[doc = "PUF Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "PUF Interrupt Enable"]
pub mod inten;
#[doc = "PUF interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "`write(|w| ..)` method takes [intstat::W](intstat::W) writer structure"]
impl crate::Writable for INTSTAT {}
#[doc = "PUF interrupt status"]
pub mod intstat;
#[doc = "PUF RAM Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrctrl](pwrctrl) module"]
pub type PWRCTRL = crate::Reg<u32, _PWRCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCTRL;
#[doc = "`read()` method returns [pwrctrl::R](pwrctrl::R) reader structure"]
impl crate::Readable for PWRCTRL {}
#[doc = "`write(|w| ..)` method takes [pwrctrl::W](pwrctrl::W) writer structure"]
impl crate::Writable for PWRCTRL {}
#[doc = "PUF RAM Power Control"]
pub mod pwrctrl;
#[doc = "PUF config register for block bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "PUF config register for block bits"]
pub mod cfg;
#[doc = "Only reset in case of full IC reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keylock](keylock) module"]
pub type KEYLOCK = crate::Reg<u32, _KEYLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYLOCK;
#[doc = "`read()` method returns [keylock::R](keylock::R) reader structure"]
impl crate::Readable for KEYLOCK {}
#[doc = "`write(|w| ..)` method takes [keylock::W](keylock::W) writer structure"]
impl crate::Writable for KEYLOCK {}
#[doc = "Only reset in case of full IC reset"]
pub mod keylock;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyenable](keyenable) module"]
pub type KEYENABLE = crate::Reg<u32, _KEYENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYENABLE;
#[doc = "`read()` method returns [keyenable::R](keyenable::R) reader structure"]
impl crate::Readable for KEYENABLE {}
#[doc = "`write(|w| ..)` method takes [keyenable::W](keyenable::W) writer structure"]
impl crate::Writable for KEYENABLE {}
#[doc = "no description available"]
pub mod keyenable;
#[doc = "Reinitialize Keys shift registers counters\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyreset](keyreset) module"]
pub type KEYRESET = crate::Reg<u32, _KEYRESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYRESET;
#[doc = "`write(|w| ..)` method takes [keyreset::W](keyreset::W) writer structure"]
impl crate::Writable for KEYRESET {}
#[doc = "Reinitialize Keys shift registers counters"]
pub mod keyreset;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idxblk_l](idxblk_l) module"]
pub type IDXBLK_L = crate::Reg<u32, _IDXBLK_L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDXBLK_L;
#[doc = "`read()` method returns [idxblk_l::R](idxblk_l::R) reader structure"]
impl crate::Readable for IDXBLK_L {}
#[doc = "`write(|w| ..)` method takes [idxblk_l::W](idxblk_l::W) writer structure"]
impl crate::Writable for IDXBLK_L {}
#[doc = "no description available"]
pub mod idxblk_l;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idxblk_h_dp](idxblk_h_dp) module"]
pub type IDXBLK_H_DP = crate::Reg<u32, _IDXBLK_H_DP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDXBLK_H_DP;
#[doc = "`read()` method returns [idxblk_h_dp::R](idxblk_h_dp::R) reader structure"]
impl crate::Readable for IDXBLK_H_DP {}
#[doc = "`write(|w| ..)` method takes [idxblk_h_dp::W](idxblk_h_dp::W) writer structure"]
impl crate::Writable for IDXBLK_H_DP {}
#[doc = "no description available"]
pub mod idxblk_h_dp;
#[doc = "Only reset in case of full IC reset\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keymask](keymask) module"]
pub type KEYMASK = crate::Reg<u32, _KEYMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYMASK;
#[doc = "`write(|w| ..)` method takes [keymask::W](keymask::W) writer structure"]
impl crate::Writable for KEYMASK {}
#[doc = "Only reset in case of full IC reset"]
pub mod keymask;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idxblk_h](idxblk_h) module"]
pub type IDXBLK_H = crate::Reg<u32, _IDXBLK_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDXBLK_H;
#[doc = "`read()` method returns [idxblk_h::R](idxblk_h::R) reader structure"]
impl crate::Readable for IDXBLK_H {}
#[doc = "`write(|w| ..)` method takes [idxblk_h::W](idxblk_h::W) writer structure"]
impl crate::Writable for IDXBLK_H {}
#[doc = "no description available"]
pub mod idxblk_h;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idxblk_l_dp](idxblk_l_dp) module"]
pub type IDXBLK_L_DP = crate::Reg<u32, _IDXBLK_L_DP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDXBLK_L_DP;
#[doc = "`read()` method returns [idxblk_l_dp::R](idxblk_l_dp::R) reader structure"]
impl crate::Readable for IDXBLK_L_DP {}
#[doc = "`write(|w| ..)` method takes [idxblk_l_dp::W](idxblk_l_dp::W) writer structure"]
impl crate::Writable for IDXBLK_L_DP {}
#[doc = "no description available"]
pub mod idxblk_l_dp;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shift_status](shift_status) module"]
pub type SHIFT_STATUS = crate::Reg<u32, _SHIFT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFT_STATUS;
#[doc = "`read()` method returns [shift_status::R](shift_status::R) reader structure"]
impl crate::Readable for SHIFT_STATUS {}
#[doc = "`write(|w| ..)` method takes [shift_status::W](shift_status::W) writer structure"]
impl crate::Writable for SHIFT_STATUS {}
#[doc = "no description available"]
pub mod shift_status;
