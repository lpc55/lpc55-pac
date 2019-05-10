#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Various Analog blocks configuration (like FRO 192MHz trimmings source ...)"]
    pub analog_ctrl_cfg: ANALOG_CTRL_CFG,
    #[doc = "0x04 - Analog Macroblock Identity registers, Flash Status registers"]
    pub analog_ctrl_status: ANALOG_CTRL_STATUS,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - Frequency Measure function control register"]
    pub freq_me_ctrl: FREQ_ME_CTRL,
    #[doc = "0x10 - 192MHz Free Running OScillator (FRO) Control register"]
    pub fro192m_ctrl: FRO192M_CTRL,
    #[doc = "0x14 - 192MHz Free Running OScillator (FRO) Status register"]
    pub fro192m_status: FRO192M_STATUS,
    #[doc = "0x18 - General Purpose ADC VBAT Divider branch control"]
    pub adc_ctrl: ADC_CTRL,
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - 32 MHz Crystal Oscillator Control register"]
    pub xo32m_ctrl: XO32M_CTRL,
    #[doc = "0x24 - 32 MHz Crystal Oscillator Status register"]
    pub xo32m_status: XO32M_STATUS,
    _reserved2: [u8; 8usize],
    #[doc = "0x30 - Brown Out Detectors (BoDs) & DCDC interrupts generation control register"]
    pub bod_dcdc_int_ctrl: BOD_DCDC_INT_CTRL,
    #[doc = "0x34 - BoDs & DCDC interrupts status register"]
    pub bod_dcdc_int_status: BOD_DCDC_INT_STATUS,
    _reserved3: [u8; 8usize],
    #[doc = "0x40 - First Ring Oscillator module control register."]
    pub ringo0_ctrl: RINGO0_CTRL,
    #[doc = "0x44 - Second Ring Oscillator module control register."]
    pub ringo1_ctrl: RINGO1_CTRL,
    #[doc = "0x48 - Third Ring Oscillator module control register."]
    pub ringo2_ctrl: RINGO2_CTRL,
    _reserved4: [u8; 100usize],
    #[doc = "0xb0 - High Speed Crystal Oscillator (12 MHz - 32 MHz) Voltage Source Supply Control register"]
    pub ldo_xo32m: LDO_XO32M,
    _reserved5: [u8; 12usize],
    #[doc = "0xc0 - All Crystal Oscillators (both the 32 KHz and the High speed) Capacitive Banks Calibration Configuration register"]
    pub xo_cal_cfg: XO_CAL_CFG,
    #[doc = "0xc4 - All Crystal Oscillators (both the 32 KHz and the High Speed) Capacitive Banks Calibration Command register."]
    pub xo_cal_cmd: XO_CAL_CMD,
    #[doc = "0xc8 - All Crystal Oscillators (both the 32 KHz and the High speed) Capacitive Banks Calibration Status register."]
    pub xo_cal_status: XO_CAL_STATUS,
    _reserved6: [u8; 52usize],
    #[doc = "0x100 - USB High Speed Phy Control"]
    pub usbhs_phy_ctrl: USBHS_PHY_CTRL,
    #[doc = "0x104 - USB High Speed Phy Trim values"]
    pub usbhs_phy_trim: USBHS_PHY_TRIM,
    #[doc = "0x108 - USB High Speed Phy Status"]
    pub usbhs_phy_status: USBHS_PHY_STATUS,
}
#[doc = "Various Analog blocks configuration (like FRO 192MHz trimmings source ...)"]
pub struct ANALOG_CTRL_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Various Analog blocks configuration (like FRO 192MHz trimmings source ...)"]
pub mod analog_ctrl_cfg;
#[doc = "Analog Macroblock Identity registers, Flash Status registers"]
pub struct ANALOG_CTRL_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Macroblock Identity registers, Flash Status registers"]
pub mod analog_ctrl_status;
#[doc = "Frequency Measure function control register"]
pub struct FREQ_ME_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frequency Measure function control register"]
pub mod freq_me_ctrl;
#[doc = "192MHz Free Running OScillator (FRO) Control register"]
pub struct FRO192M_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "192MHz Free Running OScillator (FRO) Control register"]
pub mod fro192m_ctrl;
#[doc = "192MHz Free Running OScillator (FRO) Status register"]
pub struct FRO192M_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "192MHz Free Running OScillator (FRO) Status register"]
pub mod fro192m_status;
#[doc = "General Purpose ADC VBAT Divider branch control"]
pub struct ADC_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose ADC VBAT Divider branch control"]
pub mod adc_ctrl;
#[doc = "32 MHz Crystal Oscillator Control register"]
pub struct XO32M_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "32 MHz Crystal Oscillator Control register"]
pub mod xo32m_ctrl;
#[doc = "32 MHz Crystal Oscillator Status register"]
pub struct XO32M_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "32 MHz Crystal Oscillator Status register"]
pub mod xo32m_status;
#[doc = "Brown Out Detectors (BoDs) & DCDC interrupts generation control register"]
pub struct BOD_DCDC_INT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Brown Out Detectors (BoDs) & DCDC interrupts generation control register"]
pub mod bod_dcdc_int_ctrl;
#[doc = "BoDs & DCDC interrupts status register"]
pub struct BOD_DCDC_INT_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BoDs & DCDC interrupts status register"]
pub mod bod_dcdc_int_status;
#[doc = "First Ring Oscillator module control register."]
pub struct RINGO0_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "First Ring Oscillator module control register."]
pub mod ringo0_ctrl;
#[doc = "Second Ring Oscillator module control register."]
pub struct RINGO1_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Second Ring Oscillator module control register."]
pub mod ringo1_ctrl;
#[doc = "Third Ring Oscillator module control register."]
pub struct RINGO2_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Third Ring Oscillator module control register."]
pub mod ringo2_ctrl;
#[doc = "High Speed Crystal Oscillator (12 MHz - 32 MHz) Voltage Source Supply Control register"]
pub struct LDO_XO32M {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "High Speed Crystal Oscillator (12 MHz - 32 MHz) Voltage Source Supply Control register"]
pub mod ldo_xo32m;
#[doc = "All Crystal Oscillators (both the 32 KHz and the High speed) Capacitive Banks Calibration Configuration register"]
pub struct XO_CAL_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "All Crystal Oscillators (both the 32 KHz and the High speed) Capacitive Banks Calibration Configuration register"]
pub mod xo_cal_cfg;
#[doc = "All Crystal Oscillators (both the 32 KHz and the High Speed) Capacitive Banks Calibration Command register."]
pub struct XO_CAL_CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "All Crystal Oscillators (both the 32 KHz and the High Speed) Capacitive Banks Calibration Command register."]
pub mod xo_cal_cmd;
#[doc = "All Crystal Oscillators (both the 32 KHz and the High speed) Capacitive Banks Calibration Status register."]
pub struct XO_CAL_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "All Crystal Oscillators (both the 32 KHz and the High speed) Capacitive Banks Calibration Status register."]
pub mod xo_cal_status;
#[doc = "USB High Speed Phy Control"]
pub struct USBHS_PHY_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB High Speed Phy Control"]
pub mod usbhs_phy_ctrl;
#[doc = "USB High Speed Phy Trim values"]
pub struct USBHS_PHY_TRIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB High Speed Phy Trim values"]
pub mod usbhs_phy_trim;
#[doc = "USB High Speed Phy Status"]
pub struct USBHS_PHY_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB High Speed Phy Status"]
pub mod usbhs_phy_status;
