#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Analog Macroblock Identity registers, Flash Status registers"]
    pub analog_ctrl_status: ANALOG_CTRL_STATUS,
    _reserved1: [u8; 4usize],
    #[doc = "0x0c - Frequency Measure function control register"]
    pub freq_me_ctrl: FREQ_ME_CTRL,
    #[doc = "0x10 - 192MHz Free Running OScillator (FRO) Control register"]
    pub fro192m_ctrl: FRO192M_CTRL,
    #[doc = "0x14 - 192MHz Free Running OScillator (FRO) Status register"]
    pub fro192m_status: FRO192M_STATUS,
    _reserved4: [u8; 8usize],
    #[doc = "0x20 - High speed Crystal Oscillator Control register"]
    pub xo32m_ctrl: XO32M_CTRL,
    #[doc = "0x24 - High speed Crystal Oscillator Status register"]
    pub xo32m_status: XO32M_STATUS,
    _reserved6: [u8; 8usize],
    #[doc = "0x30 - Brown Out Detectors (BoDs) & DCDC interrupts generation control register"]
    pub bod_dcdc_int_ctrl: BOD_DCDC_INT_CTRL,
    #[doc = "0x34 - BoDs & DCDC interrupts status register"]
    pub bod_dcdc_int_status: BOD_DCDC_INT_STATUS,
    _reserved8: [u8; 8usize],
    #[doc = "0x40 - First Ring Oscillator module control register."]
    pub ringo0_ctrl: RINGO0_CTRL,
    #[doc = "0x44 - Second Ring Oscillator module control register."]
    pub ringo1_ctrl: RINGO1_CTRL,
    #[doc = "0x48 - Third Ring Oscillator module control register."]
    pub ringo2_ctrl: RINGO2_CTRL,
    _reserved11: [u8; 180usize],
    #[doc = "0x100 - USB High Speed Phy Control"]
    pub usbhs_phy_ctrl: USBHS_PHY_CTRL,
}
#[doc = "Analog Macroblock Identity registers, Flash Status registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [analog_ctrl_status](analog_ctrl_status) module"]
pub type ANALOG_CTRL_STATUS = crate::Reg<u32, _ANALOG_CTRL_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANALOG_CTRL_STATUS;
#[doc = "`read()` method returns [analog_ctrl_status::R](analog_ctrl_status::R) reader structure"]
impl crate::Readable for ANALOG_CTRL_STATUS {}
#[doc = "Analog Macroblock Identity registers, Flash Status registers"]
pub mod analog_ctrl_status;
#[doc = "Frequency Measure function control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [freq_me_ctrl](freq_me_ctrl) module"]
pub type FREQ_ME_CTRL = crate::Reg<u32, _FREQ_ME_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQ_ME_CTRL;
#[doc = "`read()` method returns [freq_me_ctrl::R](freq_me_ctrl::R) reader structure"]
impl crate::Readable for FREQ_ME_CTRL {}
#[doc = "`write(|w| ..)` method takes [freq_me_ctrl::W](freq_me_ctrl::W) writer structure"]
impl crate::Writable for FREQ_ME_CTRL {}
#[doc = "Frequency Measure function control register"]
pub mod freq_me_ctrl;
#[doc = "192MHz Free Running OScillator (FRO) Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fro192m_ctrl](fro192m_ctrl) module"]
pub type FRO192M_CTRL = crate::Reg<u32, _FRO192M_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRO192M_CTRL;
#[doc = "`read()` method returns [fro192m_ctrl::R](fro192m_ctrl::R) reader structure"]
impl crate::Readable for FRO192M_CTRL {}
#[doc = "`write(|w| ..)` method takes [fro192m_ctrl::W](fro192m_ctrl::W) writer structure"]
impl crate::Writable for FRO192M_CTRL {}
#[doc = "192MHz Free Running OScillator (FRO) Control register"]
pub mod fro192m_ctrl;
#[doc = "192MHz Free Running OScillator (FRO) Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fro192m_status](fro192m_status) module"]
pub type FRO192M_STATUS = crate::Reg<u32, _FRO192M_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRO192M_STATUS;
#[doc = "`read()` method returns [fro192m_status::R](fro192m_status::R) reader structure"]
impl crate::Readable for FRO192M_STATUS {}
#[doc = "`write(|w| ..)` method takes [fro192m_status::W](fro192m_status::W) writer structure"]
impl crate::Writable for FRO192M_STATUS {}
#[doc = "192MHz Free Running OScillator (FRO) Status register"]
pub mod fro192m_status;
#[doc = "High speed Crystal Oscillator Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xo32m_ctrl](xo32m_ctrl) module"]
pub type XO32M_CTRL = crate::Reg<u32, _XO32M_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XO32M_CTRL;
#[doc = "`read()` method returns [xo32m_ctrl::R](xo32m_ctrl::R) reader structure"]
impl crate::Readable for XO32M_CTRL {}
#[doc = "`write(|w| ..)` method takes [xo32m_ctrl::W](xo32m_ctrl::W) writer structure"]
impl crate::Writable for XO32M_CTRL {}
#[doc = "High speed Crystal Oscillator Control register"]
pub mod xo32m_ctrl;
#[doc = "High speed Crystal Oscillator Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xo32m_status](xo32m_status) module"]
pub type XO32M_STATUS = crate::Reg<u32, _XO32M_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XO32M_STATUS;
#[doc = "`read()` method returns [xo32m_status::R](xo32m_status::R) reader structure"]
impl crate::Readable for XO32M_STATUS {}
#[doc = "High speed Crystal Oscillator Status register"]
pub mod xo32m_status;
#[doc = "Brown Out Detectors (BoDs) & DCDC interrupts generation control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bod_dcdc_int_ctrl](bod_dcdc_int_ctrl) module"]
pub type BOD_DCDC_INT_CTRL = crate::Reg<u32, _BOD_DCDC_INT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOD_DCDC_INT_CTRL;
#[doc = "`read()` method returns [bod_dcdc_int_ctrl::R](bod_dcdc_int_ctrl::R) reader structure"]
impl crate::Readable for BOD_DCDC_INT_CTRL {}
#[doc = "`write(|w| ..)` method takes [bod_dcdc_int_ctrl::W](bod_dcdc_int_ctrl::W) writer structure"]
impl crate::Writable for BOD_DCDC_INT_CTRL {}
#[doc = "Brown Out Detectors (BoDs) & DCDC interrupts generation control register"]
pub mod bod_dcdc_int_ctrl;
#[doc = "BoDs & DCDC interrupts status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bod_dcdc_int_status](bod_dcdc_int_status) module"]
pub type BOD_DCDC_INT_STATUS = crate::Reg<u32, _BOD_DCDC_INT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOD_DCDC_INT_STATUS;
#[doc = "`read()` method returns [bod_dcdc_int_status::R](bod_dcdc_int_status::R) reader structure"]
impl crate::Readable for BOD_DCDC_INT_STATUS {}
#[doc = "BoDs & DCDC interrupts status register"]
pub mod bod_dcdc_int_status;
#[doc = "First Ring Oscillator module control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ringo0_ctrl](ringo0_ctrl) module"]
pub type RINGO0_CTRL = crate::Reg<u32, _RINGO0_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RINGO0_CTRL;
#[doc = "`read()` method returns [ringo0_ctrl::R](ringo0_ctrl::R) reader structure"]
impl crate::Readable for RINGO0_CTRL {}
#[doc = "`write(|w| ..)` method takes [ringo0_ctrl::W](ringo0_ctrl::W) writer structure"]
impl crate::Writable for RINGO0_CTRL {}
#[doc = "First Ring Oscillator module control register."]
pub mod ringo0_ctrl;
#[doc = "Second Ring Oscillator module control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ringo1_ctrl](ringo1_ctrl) module"]
pub type RINGO1_CTRL = crate::Reg<u32, _RINGO1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RINGO1_CTRL;
#[doc = "`read()` method returns [ringo1_ctrl::R](ringo1_ctrl::R) reader structure"]
impl crate::Readable for RINGO1_CTRL {}
#[doc = "`write(|w| ..)` method takes [ringo1_ctrl::W](ringo1_ctrl::W) writer structure"]
impl crate::Writable for RINGO1_CTRL {}
#[doc = "Second Ring Oscillator module control register."]
pub mod ringo1_ctrl;
#[doc = "Third Ring Oscillator module control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ringo2_ctrl](ringo2_ctrl) module"]
pub type RINGO2_CTRL = crate::Reg<u32, _RINGO2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RINGO2_CTRL;
#[doc = "`read()` method returns [ringo2_ctrl::R](ringo2_ctrl::R) reader structure"]
impl crate::Readable for RINGO2_CTRL {}
#[doc = "`write(|w| ..)` method takes [ringo2_ctrl::W](ringo2_ctrl::W) writer structure"]
impl crate::Writable for RINGO2_CTRL {}
#[doc = "Third Ring Oscillator module control register."]
pub mod ringo2_ctrl;
#[doc = "USB High Speed Phy Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usbhs_phy_ctrl](usbhs_phy_ctrl) module"]
pub type USBHS_PHY_CTRL = crate::Reg<u32, _USBHS_PHY_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_PHY_CTRL;
#[doc = "`read()` method returns [usbhs_phy_ctrl::R](usbhs_phy_ctrl::R) reader structure"]
impl crate::Readable for USBHS_PHY_CTRL {}
#[doc = "`write(|w| ..)` method takes [usbhs_phy_ctrl::W](usbhs_phy_ctrl::W) writer structure"]
impl crate::Writable for USBHS_PHY_CTRL {}
#[doc = "USB High Speed Phy Control"]
pub mod usbhs_phy_ctrl;
