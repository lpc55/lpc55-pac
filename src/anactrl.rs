#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Analog Macroblock Identity registers, Flash Status registers"]
    pub analog_ctrl_status: crate::Reg<analog_ctrl_status::ANALOG_CTRL_STATUS_SPEC>,
    _reserved1: [u8; 4usize],
    #[doc = "0x0c - Frequency Measure function control register"]
    pub freq_me_ctrl: crate::Reg<freq_me_ctrl::FREQ_ME_CTRL_SPEC>,
    #[doc = "0x10 - 192MHz Free Running OScillator (FRO) Control register"]
    pub fro192m_ctrl: crate::Reg<fro192m_ctrl::FRO192M_CTRL_SPEC>,
    #[doc = "0x14 - 192MHz Free Running OScillator (FRO) Status register"]
    pub fro192m_status: crate::Reg<fro192m_status::FRO192M_STATUS_SPEC>,
    _reserved4: [u8; 8usize],
    #[doc = "0x20 - High speed Crystal Oscillator Control register"]
    pub xo32m_ctrl: crate::Reg<xo32m_ctrl::XO32M_CTRL_SPEC>,
    #[doc = "0x24 - High speed Crystal Oscillator Status register"]
    pub xo32m_status: crate::Reg<xo32m_status::XO32M_STATUS_SPEC>,
    _reserved6: [u8; 8usize],
    #[doc = "0x30 - Brown Out Detectors (BoDs) & DCDC interrupts generation control register"]
    pub bod_dcdc_int_ctrl: crate::Reg<bod_dcdc_int_ctrl::BOD_DCDC_INT_CTRL_SPEC>,
    #[doc = "0x34 - BoDs & DCDC interrupts status register"]
    pub bod_dcdc_int_status: crate::Reg<bod_dcdc_int_status::BOD_DCDC_INT_STATUS_SPEC>,
    _reserved8: [u8; 8usize],
    #[doc = "0x40 - First Ring Oscillator module control register."]
    pub ringo0_ctrl: crate::Reg<ringo0_ctrl::RINGO0_CTRL_SPEC>,
    #[doc = "0x44 - Second Ring Oscillator module control register."]
    pub ringo1_ctrl: crate::Reg<ringo1_ctrl::RINGO1_CTRL_SPEC>,
    #[doc = "0x48 - Third Ring Oscillator module control register."]
    pub ringo2_ctrl: crate::Reg<ringo2_ctrl::RINGO2_CTRL_SPEC>,
    _reserved11: [u8; 180usize],
    #[doc = "0x100 - USB High Speed Phy Control"]
    pub usbhs_phy_ctrl: crate::Reg<usbhs_phy_ctrl::USBHS_PHY_CTRL_SPEC>,
}
#[doc = "ANALOG_CTRL_STATUS register accessor: an alias for `Reg<ANALOG_CTRL_STATUS_SPEC>`"]
pub type ANALOG_CTRL_STATUS = crate::Reg<analog_ctrl_status::ANALOG_CTRL_STATUS_SPEC>;
#[doc = "Analog Macroblock Identity registers, Flash Status registers"]
pub mod analog_ctrl_status;
#[doc = "FREQ_ME_CTRL register accessor: an alias for `Reg<FREQ_ME_CTRL_SPEC>`"]
pub type FREQ_ME_CTRL = crate::Reg<freq_me_ctrl::FREQ_ME_CTRL_SPEC>;
#[doc = "Frequency Measure function control register"]
pub mod freq_me_ctrl;
#[doc = "FRO192M_CTRL register accessor: an alias for `Reg<FRO192M_CTRL_SPEC>`"]
pub type FRO192M_CTRL = crate::Reg<fro192m_ctrl::FRO192M_CTRL_SPEC>;
#[doc = "192MHz Free Running OScillator (FRO) Control register"]
pub mod fro192m_ctrl;
#[doc = "FRO192M_STATUS register accessor: an alias for `Reg<FRO192M_STATUS_SPEC>`"]
pub type FRO192M_STATUS = crate::Reg<fro192m_status::FRO192M_STATUS_SPEC>;
#[doc = "192MHz Free Running OScillator (FRO) Status register"]
pub mod fro192m_status;
#[doc = "XO32M_CTRL register accessor: an alias for `Reg<XO32M_CTRL_SPEC>`"]
pub type XO32M_CTRL = crate::Reg<xo32m_ctrl::XO32M_CTRL_SPEC>;
#[doc = "High speed Crystal Oscillator Control register"]
pub mod xo32m_ctrl;
#[doc = "XO32M_STATUS register accessor: an alias for `Reg<XO32M_STATUS_SPEC>`"]
pub type XO32M_STATUS = crate::Reg<xo32m_status::XO32M_STATUS_SPEC>;
#[doc = "High speed Crystal Oscillator Status register"]
pub mod xo32m_status;
#[doc = "BOD_DCDC_INT_CTRL register accessor: an alias for `Reg<BOD_DCDC_INT_CTRL_SPEC>`"]
pub type BOD_DCDC_INT_CTRL = crate::Reg<bod_dcdc_int_ctrl::BOD_DCDC_INT_CTRL_SPEC>;
#[doc = "Brown Out Detectors (BoDs) & DCDC interrupts generation control register"]
pub mod bod_dcdc_int_ctrl;
#[doc = "BOD_DCDC_INT_STATUS register accessor: an alias for `Reg<BOD_DCDC_INT_STATUS_SPEC>`"]
pub type BOD_DCDC_INT_STATUS = crate::Reg<bod_dcdc_int_status::BOD_DCDC_INT_STATUS_SPEC>;
#[doc = "BoDs & DCDC interrupts status register"]
pub mod bod_dcdc_int_status;
#[doc = "RINGO0_CTRL register accessor: an alias for `Reg<RINGO0_CTRL_SPEC>`"]
pub type RINGO0_CTRL = crate::Reg<ringo0_ctrl::RINGO0_CTRL_SPEC>;
#[doc = "First Ring Oscillator module control register."]
pub mod ringo0_ctrl;
#[doc = "RINGO1_CTRL register accessor: an alias for `Reg<RINGO1_CTRL_SPEC>`"]
pub type RINGO1_CTRL = crate::Reg<ringo1_ctrl::RINGO1_CTRL_SPEC>;
#[doc = "Second Ring Oscillator module control register."]
pub mod ringo1_ctrl;
#[doc = "RINGO2_CTRL register accessor: an alias for `Reg<RINGO2_CTRL_SPEC>`"]
pub type RINGO2_CTRL = crate::Reg<ringo2_ctrl::RINGO2_CTRL_SPEC>;
#[doc = "Third Ring Oscillator module control register."]
pub mod ringo2_ctrl;
#[doc = "USBHS_PHY_CTRL register accessor: an alias for `Reg<USBHS_PHY_CTRL_SPEC>`"]
pub type USBHS_PHY_CTRL = crate::Reg<usbhs_phy_ctrl::USBHS_PHY_CTRL_SPEC>;
#[doc = "USB High Speed Phy Control"]
pub mod usbhs_phy_ctrl;
