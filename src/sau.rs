#[doc = r"Register block"]
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
#[doc = "Security Attribution Unit Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sau_ctrl](sau_ctrl) module"]
pub type SAU_CTRL = crate::Reg<u32, _SAU_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAU_CTRL;
#[doc = "`read()` method returns [sau_ctrl::R](sau_ctrl::R) reader structure"]
impl crate::Readable for SAU_CTRL {}
#[doc = "`write(|w| ..)` method takes [sau_ctrl::W](sau_ctrl::W) writer structure"]
impl crate::Writable for SAU_CTRL {}
#[doc = "Security Attribution Unit Control Register"]
pub mod sau_ctrl;
#[doc = "Security Attribution Unit Type Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sau_type](sau_type) module"]
pub type SAU_TYPE = crate::Reg<u32, _SAU_TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAU_TYPE;
#[doc = "`read()` method returns [sau_type::R](sau_type::R) reader structure"]
impl crate::Readable for SAU_TYPE {}
#[doc = "`write(|w| ..)` method takes [sau_type::W](sau_type::W) writer structure"]
impl crate::Writable for SAU_TYPE {}
#[doc = "Security Attribution Unit Type Register"]
pub mod sau_type;
#[doc = "Security Attribution Unit Region Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sau_rnr](sau_rnr) module"]
pub type SAU_RNR = crate::Reg<u32, _SAU_RNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAU_RNR;
#[doc = "`read()` method returns [sau_rnr::R](sau_rnr::R) reader structure"]
impl crate::Readable for SAU_RNR {}
#[doc = "`write(|w| ..)` method takes [sau_rnr::W](sau_rnr::W) writer structure"]
impl crate::Writable for SAU_RNR {}
#[doc = "Security Attribution Unit Region Number Register"]
pub mod sau_rnr;
#[doc = "Security Attribution Unit Region Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sau_rbar](sau_rbar) module"]
pub type SAU_RBAR = crate::Reg<u32, _SAU_RBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAU_RBAR;
#[doc = "`read()` method returns [sau_rbar::R](sau_rbar::R) reader structure"]
impl crate::Readable for SAU_RBAR {}
#[doc = "`write(|w| ..)` method takes [sau_rbar::W](sau_rbar::W) writer structure"]
impl crate::Writable for SAU_RBAR {}
#[doc = "Security Attribution Unit Region Base Address Register"]
pub mod sau_rbar;
#[doc = "Security Attribution Unit Region Limit Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sau_rlar](sau_rlar) module"]
pub type SAU_RLAR = crate::Reg<u32, _SAU_RLAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAU_RLAR;
#[doc = "`read()` method returns [sau_rlar::R](sau_rlar::R) reader structure"]
impl crate::Readable for SAU_RLAR {}
#[doc = "`write(|w| ..)` method takes [sau_rlar::W](sau_rlar::W) writer structure"]
impl crate::Writable for SAU_RLAR {}
#[doc = "Security Attribution Unit Region Limit Address Register"]
pub mod sau_rlar;
#[doc = "Secure Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sfsr](sfsr) module"]
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
#[doc = "Secure Fault Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sfar](sfar) module"]
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
