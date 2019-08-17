#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Various Analog blocks configuration (like FRO 192MHz trimmings source ...)"]
    pub analog_ctrl_cfg: ANALOG_CTRL_CFG,
    #[doc = "0x04 - Analog Macroblock Identity registers, Flash Status registers"]
    pub analog_ctrl_status: ANALOG_CTRL_STATUS,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - Frequency Measure function control register"]
    pub freq_me_ctrl: FREQ_ME_CTRL,
    #[doc = "0x10 - 192MHz Free Running OScillator (FRO) Control register"]
    pub fro192m_ctrl: FRO192M_CTRL,
    #[doc = "0x14 - 192MHz Free Running OScillator (FRO) Status register"]
    pub fro192m_status: FRO192M_STATUS,
    #[doc = "0x18 - General Purpose ADC VBAT Divider branch control"]
    pub adc_ctrl: ADC_CTRL,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - 32 MHz Crystal Oscillator Control register"]
    pub xo32m_ctrl: XO32M_CTRL,
    #[doc = "0x24 - 32 MHz Crystal Oscillator Status register"]
    pub xo32m_status: XO32M_STATUS,
    _reserved8: [u8; 8usize],
    #[doc = "0x30 - Brown Out Detectors (BoDs) & DCDC interrupts generation control register"]
    pub bod_dcdc_int_ctrl: BOD_DCDC_INT_CTRL,
    #[doc = "0x34 - BoDs & DCDC interrupts status register"]
    pub bod_dcdc_int_status: BOD_DCDC_INT_STATUS,
    _reserved10: [u8; 8usize],
    #[doc = "0x40 - First Ring Oscillator module control register."]
    pub ringo0_ctrl: RINGO0_CTRL,
    #[doc = "0x44 - Second Ring Oscillator module control register."]
    pub ringo1_ctrl: RINGO1_CTRL,
    #[doc = "0x48 - Third Ring Oscillator module control register."]
    pub ringo2_ctrl: RINGO2_CTRL,
    _reserved13: [u8; 100usize],
    #[doc = "0xb0 - High Speed Crystal Oscillator (12 MHz - 32 MHz) Voltage Source Supply Control register"]
    pub ldo_xo32m: LDO_XO32M,
    _reserved14: [u8; 12usize],
    #[doc = "0xc0 - All Crystal Oscillators (both the 32 KHz and the High speed) Capacitive Banks Calibration Configuration register"]
    pub xo_cal_cfg: XO_CAL_CFG,
    #[doc = "0xc4 - All Crystal Oscillators (both the 32 KHz and the High Speed) Capacitive Banks Calibration Command register."]
    pub xo_cal_cmd: XO_CAL_CMD,
    #[doc = "0xc8 - All Crystal Oscillators (both the 32 KHz and the High speed) Capacitive Banks Calibration Status register."]
    pub xo_cal_status: XO_CAL_STATUS,
    _reserved17: [u8; 52usize],
    #[doc = "0x100 - USB High Speed Phy Control"]
    pub usbhs_phy_ctrl: USBHS_PHY_CTRL,
    #[doc = "0x104 - USB High Speed Phy Trim values"]
    pub usbhs_phy_trim: USBHS_PHY_TRIM,
    #[doc = "0x108 - USB High Speed Phy Status"]
    pub usbhs_phy_status: USBHS_PHY_STATUS,
}
#[doc = "Various Analog blocks configuration (like FRO 192MHz trimmings source ...)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [analog_ctrl_cfg](analog_ctrl_cfg) module"]
pub type ANALOG_CTRL_CFG = crate::Reg<u32, _ANALOG_CTRL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANALOG_CTRL_CFG;
#[doc = "`read()` method returns [analog_ctrl_cfg::R](analog_ctrl_cfg::R) reader structure"]
impl crate::Readable for ANALOG_CTRL_CFG {}
#[doc = "`write(|w| ..)` method takes [analog_ctrl_cfg::W](analog_ctrl_cfg::W) writer structure"]
impl crate::Writable for ANALOG_CTRL_CFG {}
#[doc = "Various Analog blocks configuration (like FRO 192MHz trimmings source ...)"]
pub mod analog_ctrl_cfg;
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
#[doc = "General Purpose ADC VBAT Divider branch control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adc_ctrl](adc_ctrl) module"]
pub type ADC_CTRL = crate::Reg<u32, _ADC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_CTRL;
#[doc = "`read()` method returns [adc_ctrl::R](adc_ctrl::R) reader structure"]
impl crate::Readable for ADC_CTRL {}
#[doc = "`write(|w| ..)` method takes [adc_ctrl::W](adc_ctrl::W) writer structure"]
impl crate::Writable for ADC_CTRL {}
#[doc = "General Purpose ADC VBAT Divider branch control"]
pub mod adc_ctrl;
#[doc = "32 MHz Crystal Oscillator Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xo32m_ctrl](xo32m_ctrl) module"]
pub type XO32M_CTRL = crate::Reg<u32, _XO32M_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XO32M_CTRL;
#[doc = "`read()` method returns [xo32m_ctrl::R](xo32m_ctrl::R) reader structure"]
impl crate::Readable for XO32M_CTRL {}
#[doc = "`write(|w| ..)` method takes [xo32m_ctrl::W](xo32m_ctrl::W) writer structure"]
impl crate::Writable for XO32M_CTRL {}
#[doc = "32 MHz Crystal Oscillator Control register"]
pub mod xo32m_ctrl;
#[doc = "32 MHz Crystal Oscillator Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xo32m_status](xo32m_status) module"]
pub type XO32M_STATUS = crate::Reg<u32, _XO32M_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XO32M_STATUS;
#[doc = "`read()` method returns [xo32m_status::R](xo32m_status::R) reader structure"]
impl crate::Readable for XO32M_STATUS {}
#[doc = "32 MHz Crystal Oscillator Status register"]
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
#[doc = "High Speed Crystal Oscillator (12 MHz - 32 MHz) Voltage Source Supply Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ldo_xo32m](ldo_xo32m) module"]
pub type LDO_XO32M = crate::Reg<u32, _LDO_XO32M>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LDO_XO32M;
#[doc = "`read()` method returns [ldo_xo32m::R](ldo_xo32m::R) reader structure"]
impl crate::Readable for LDO_XO32M {}
#[doc = "`write(|w| ..)` method takes [ldo_xo32m::W](ldo_xo32m::W) writer structure"]
impl crate::Writable for LDO_XO32M {}
#[doc = "High Speed Crystal Oscillator (12 MHz - 32 MHz) Voltage Source Supply Control register"]
pub mod ldo_xo32m;
#[doc = "All Crystal Oscillators (both the 32 KHz and the High speed) Capacitive Banks Calibration Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xo_cal_cfg](xo_cal_cfg) module"]
pub type XO_CAL_CFG = crate::Reg<u32, _XO_CAL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XO_CAL_CFG;
#[doc = "`read()` method returns [xo_cal_cfg::R](xo_cal_cfg::R) reader structure"]
impl crate::Readable for XO_CAL_CFG {}
#[doc = "`write(|w| ..)` method takes [xo_cal_cfg::W](xo_cal_cfg::W) writer structure"]
impl crate::Writable for XO_CAL_CFG {}
#[doc = "All Crystal Oscillators (both the 32 KHz and the High speed) Capacitive Banks Calibration Configuration register"]
pub mod xo_cal_cfg;
#[doc = "All Crystal Oscillators (both the 32 KHz and the High Speed) Capacitive Banks Calibration Command register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xo_cal_cmd](xo_cal_cmd) module"]
pub type XO_CAL_CMD = crate::Reg<u32, _XO_CAL_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XO_CAL_CMD;
#[doc = "`read()` method returns [xo_cal_cmd::R](xo_cal_cmd::R) reader structure"]
impl crate::Readable for XO_CAL_CMD {}
#[doc = "`write(|w| ..)` method takes [xo_cal_cmd::W](xo_cal_cmd::W) writer structure"]
impl crate::Writable for XO_CAL_CMD {}
#[doc = "All Crystal Oscillators (both the 32 KHz and the High Speed) Capacitive Banks Calibration Command register."]
pub mod xo_cal_cmd;
#[doc = "All Crystal Oscillators (both the 32 KHz and the High speed) Capacitive Banks Calibration Status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xo_cal_status](xo_cal_status) module"]
pub type XO_CAL_STATUS = crate::Reg<u32, _XO_CAL_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XO_CAL_STATUS;
#[doc = "`read()` method returns [xo_cal_status::R](xo_cal_status::R) reader structure"]
impl crate::Readable for XO_CAL_STATUS {}
#[doc = "All Crystal Oscillators (both the 32 KHz and the High speed) Capacitive Banks Calibration Status register."]
pub mod xo_cal_status;
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
#[doc = "USB High Speed Phy Trim values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usbhs_phy_trim](usbhs_phy_trim) module"]
pub type USBHS_PHY_TRIM = crate::Reg<u32, _USBHS_PHY_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_PHY_TRIM;
#[doc = "`read()` method returns [usbhs_phy_trim::R](usbhs_phy_trim::R) reader structure"]
impl crate::Readable for USBHS_PHY_TRIM {}
#[doc = "`write(|w| ..)` method takes [usbhs_phy_trim::W](usbhs_phy_trim::W) writer structure"]
impl crate::Writable for USBHS_PHY_TRIM {}
#[doc = "USB High Speed Phy Trim values"]
pub mod usbhs_phy_trim;
#[doc = "USB High Speed Phy Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usbhs_phy_status](usbhs_phy_status) module"]
pub type USBHS_PHY_STATUS = crate::Reg<u32, _USBHS_PHY_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_PHY_STATUS;
#[doc = "`read()` method returns [usbhs_phy_status::R](usbhs_phy_status::R) reader structure"]
impl crate::Readable for USBHS_PHY_STATUS {}
#[doc = "USB High Speed Phy Status"]
pub mod usbhs_phy_status;
