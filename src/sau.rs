#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 208usize],
    #[doc = "0xd0 - Security Attribution Unit Control Register"]
    pub ctrl: CTRL,
    #[doc = "0xd4 - Security Attribution Unit Type Register"]
    pub type_: TYPE,
    #[doc = "0xd8 - Security Attribution Unit Region Number Register"]
    pub rnr: RNR,
    #[doc = "0xdc - Security Attribution Unit Region Base Address Register"]
    pub rbar: RBAR,
    #[doc = "0xe0 - Security Attribution Unit Region Limit Address Register"]
    pub rlar: RLAR,
    #[doc = "0xe4 - Secure Fault Status Register"]
    pub sfsr: SFSR,
    #[doc = "0xe8 - Secure Fault Address Register"]
    pub sfar: SFAR,
}
#[doc = "Security Attribution Unit Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Security Attribution Unit Control Register"]
pub mod ctrl;
#[doc = "Security Attribution Unit Type Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [type_](type_) module"]
pub type TYPE = crate::Reg<u32, _TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TYPE;
#[doc = "`read()` method returns [type_::R](type_::R) reader structure"]
impl crate::Readable for TYPE {}
#[doc = "`write(|w| ..)` method takes [type_::W](type_::W) writer structure"]
impl crate::Writable for TYPE {}
#[doc = "Security Attribution Unit Type Register"]
pub mod type_;
#[doc = "Security Attribution Unit Region Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rnr](rnr) module"]
pub type RNR = crate::Reg<u32, _RNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNR;
#[doc = "`read()` method returns [rnr::R](rnr::R) reader structure"]
impl crate::Readable for RNR {}
#[doc = "`write(|w| ..)` method takes [rnr::W](rnr::W) writer structure"]
impl crate::Writable for RNR {}
#[doc = "Security Attribution Unit Region Number Register"]
pub mod rnr;
#[doc = "Security Attribution Unit Region Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbar](rbar) module"]
pub type RBAR = crate::Reg<u32, _RBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBAR;
#[doc = "`read()` method returns [rbar::R](rbar::R) reader structure"]
impl crate::Readable for RBAR {}
#[doc = "`write(|w| ..)` method takes [rbar::W](rbar::W) writer structure"]
impl crate::Writable for RBAR {}
#[doc = "Security Attribution Unit Region Base Address Register"]
pub mod rbar;
#[doc = "Security Attribution Unit Region Limit Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlar](rlar) module"]
pub type RLAR = crate::Reg<u32, _RLAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLAR;
#[doc = "`read()` method returns [rlar::R](rlar::R) reader structure"]
impl crate::Readable for RLAR {}
#[doc = "`write(|w| ..)` method takes [rlar::W](rlar::W) writer structure"]
impl crate::Writable for RLAR {}
#[doc = "Security Attribution Unit Region Limit Address Register"]
pub mod rlar;
#[doc = "Secure Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfsr](sfsr) module"]
pub type SFSR = crate::Reg<u32, _SFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFSR;
#[doc = "`read()` method returns [sfsr::R](sfsr::R) reader structure"]
impl crate::Readable for SFSR {}
#[doc = "`write(|w| ..)` method takes [sfsr::W](sfsr::W) writer structure"]
impl crate::Writable for SFSR {}
#[doc = "Secure Fault Status Register"]
pub mod sfsr;
#[doc = "Secure Fault Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfar](sfar) module"]
pub type SFAR = crate::Reg<u32, _SFAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFAR;
#[doc = "`read()` method returns [sfar::R](sfar::R) reader structure"]
impl crate::Readable for SFAR {}
#[doc = "`write(|w| ..)` method takes [sfar::W](sfar::W) writer structure"]
impl crate::Writable for SFAR {}
#[doc = "Secure Fault Address Register"]
pub mod sfar;
