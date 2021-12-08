#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB PHY Power-Down Register"]
    pub pwd: crate::Reg<pwd::PWD_SPEC>,
    #[doc = "0x04 - USB PHY Power-Down Register"]
    pub pwd_set: crate::Reg<pwd_set::PWD_SET_SPEC>,
    #[doc = "0x08 - USB PHY Power-Down Register"]
    pub pwd_clr: crate::Reg<pwd_clr::PWD_CLR_SPEC>,
    #[doc = "0x0c - USB PHY Power-Down Register"]
    pub pwd_tog: crate::Reg<pwd_tog::PWD_TOG_SPEC>,
    #[doc = "0x10 - USB PHY Transmitter Control Register"]
    pub tx: crate::Reg<tx::TX_SPEC>,
    #[doc = "0x14 - USB PHY Transmitter Control Register"]
    pub tx_set: crate::Reg<tx_set::TX_SET_SPEC>,
    #[doc = "0x18 - USB PHY Transmitter Control Register"]
    pub tx_clr: crate::Reg<tx_clr::TX_CLR_SPEC>,
    #[doc = "0x1c - USB PHY Transmitter Control Register"]
    pub tx_tog: crate::Reg<tx_tog::TX_TOG_SPEC>,
    #[doc = "0x20 - USB PHY Receiver Control Register"]
    pub rx: crate::Reg<rx::RX_SPEC>,
    #[doc = "0x24 - USB PHY Receiver Control Register"]
    pub rx_set: crate::Reg<rx_set::RX_SET_SPEC>,
    #[doc = "0x28 - USB PHY Receiver Control Register"]
    pub rx_clr: crate::Reg<rx_clr::RX_CLR_SPEC>,
    #[doc = "0x2c - USB PHY Receiver Control Register"]
    pub rx_tog: crate::Reg<rx_tog::RX_TOG_SPEC>,
    #[doc = "0x30 - USB PHY General Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x34 - USB PHY General Control Register"]
    pub ctrl_set: crate::Reg<ctrl_set::CTRL_SET_SPEC>,
    #[doc = "0x38 - USB PHY General Control Register"]
    pub ctrl_clr: crate::Reg<ctrl_clr::CTRL_CLR_SPEC>,
    #[doc = "0x3c - USB PHY General Control Register"]
    pub ctrl_tog: crate::Reg<ctrl_tog::CTRL_TOG_SPEC>,
    #[doc = "0x40 - USB PHY Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved17: [u8; 0x5c],
    #[doc = "0xa0 - USB PHY PLL Control/Status Register"]
    pub pll_sic: crate::Reg<pll_sic::PLL_SIC_SPEC>,
    #[doc = "0xa4 - USB PHY PLL Control/Status Register"]
    pub pll_sic_set: crate::Reg<pll_sic_set::PLL_SIC_SET_SPEC>,
    #[doc = "0xa8 - USB PHY PLL Control/Status Register"]
    pub pll_sic_clr: crate::Reg<pll_sic_clr::PLL_SIC_CLR_SPEC>,
    #[doc = "0xac - USB PHY PLL Control/Status Register"]
    pub pll_sic_tog: crate::Reg<pll_sic_tog::PLL_SIC_TOG_SPEC>,
    _reserved21: [u8; 0x10],
    #[doc = "0xc0 - USB PHY VBUS Detect Control Register"]
    pub usb1_vbus_detect: crate::Reg<usb1_vbus_detect::USB1_VBUS_DETECT_SPEC>,
    #[doc = "0xc4 - USB PHY VBUS Detect Control Register"]
    pub usb1_vbus_detect_set: crate::Reg<usb1_vbus_detect_set::USB1_VBUS_DETECT_SET_SPEC>,
    #[doc = "0xc8 - USB PHY VBUS Detect Control Register"]
    pub usb1_vbus_detect_clr: crate::Reg<usb1_vbus_detect_clr::USB1_VBUS_DETECT_CLR_SPEC>,
    #[doc = "0xcc - USB PHY VBUS Detect Control Register"]
    pub usb1_vbus_detect_tog: crate::Reg<usb1_vbus_detect_tog::USB1_VBUS_DETECT_TOG_SPEC>,
    _reserved25: [u8; 0x30],
    #[doc = "0x100 - USB PHY Analog Control Register"]
    pub anactrl: crate::Reg<anactrl::ANACTRL_SPEC>,
    #[doc = "0x104 - USB PHY Analog Control Register"]
    pub anactrl_set: crate::Reg<anactrl_set::ANACTRL_SET_SPEC>,
    #[doc = "0x108 - USB PHY Analog Control Register"]
    pub anactrl_clr: crate::Reg<anactrl_clr::ANACTRL_CLR_SPEC>,
    #[doc = "0x10c - USB PHY Analog Control Register"]
    pub anactrl_tog: crate::Reg<anactrl_tog::ANACTRL_TOG_SPEC>,
}
#[doc = "PWD register accessor: an alias for `Reg<PWD_SPEC>`"]
pub type PWD = crate::Reg<pwd::PWD_SPEC>;
#[doc = "USB PHY Power-Down Register"]
pub mod pwd;
#[doc = "PWD_SET register accessor: an alias for `Reg<PWD_SET_SPEC>`"]
pub type PWD_SET = crate::Reg<pwd_set::PWD_SET_SPEC>;
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_set;
#[doc = "PWD_CLR register accessor: an alias for `Reg<PWD_CLR_SPEC>`"]
pub type PWD_CLR = crate::Reg<pwd_clr::PWD_CLR_SPEC>;
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_clr;
#[doc = "PWD_TOG register accessor: an alias for `Reg<PWD_TOG_SPEC>`"]
pub type PWD_TOG = crate::Reg<pwd_tog::PWD_TOG_SPEC>;
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_tog;
#[doc = "TX register accessor: an alias for `Reg<TX_SPEC>`"]
pub type TX = crate::Reg<tx::TX_SPEC>;
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx;
#[doc = "TX_SET register accessor: an alias for `Reg<TX_SET_SPEC>`"]
pub type TX_SET = crate::Reg<tx_set::TX_SET_SPEC>;
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_set;
#[doc = "TX_CLR register accessor: an alias for `Reg<TX_CLR_SPEC>`"]
pub type TX_CLR = crate::Reg<tx_clr::TX_CLR_SPEC>;
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_clr;
#[doc = "TX_TOG register accessor: an alias for `Reg<TX_TOG_SPEC>`"]
pub type TX_TOG = crate::Reg<tx_tog::TX_TOG_SPEC>;
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_tog;
#[doc = "RX register accessor: an alias for `Reg<RX_SPEC>`"]
pub type RX = crate::Reg<rx::RX_SPEC>;
#[doc = "USB PHY Receiver Control Register"]
pub mod rx;
#[doc = "RX_SET register accessor: an alias for `Reg<RX_SET_SPEC>`"]
pub type RX_SET = crate::Reg<rx_set::RX_SET_SPEC>;
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_set;
#[doc = "RX_CLR register accessor: an alias for `Reg<RX_CLR_SPEC>`"]
pub type RX_CLR = crate::Reg<rx_clr::RX_CLR_SPEC>;
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_clr;
#[doc = "RX_TOG register accessor: an alias for `Reg<RX_TOG_SPEC>`"]
pub type RX_TOG = crate::Reg<rx_tog::RX_TOG_SPEC>;
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_tog;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "USB PHY General Control Register"]
pub mod ctrl;
#[doc = "CTRL_SET register accessor: an alias for `Reg<CTRL_SET_SPEC>`"]
pub type CTRL_SET = crate::Reg<ctrl_set::CTRL_SET_SPEC>;
#[doc = "USB PHY General Control Register"]
pub mod ctrl_set;
#[doc = "CTRL_CLR register accessor: an alias for `Reg<CTRL_CLR_SPEC>`"]
pub type CTRL_CLR = crate::Reg<ctrl_clr::CTRL_CLR_SPEC>;
#[doc = "USB PHY General Control Register"]
pub mod ctrl_clr;
#[doc = "CTRL_TOG register accessor: an alias for `Reg<CTRL_TOG_SPEC>`"]
pub type CTRL_TOG = crate::Reg<ctrl_tog::CTRL_TOG_SPEC>;
#[doc = "USB PHY General Control Register"]
pub mod ctrl_tog;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "USB PHY Status Register"]
pub mod status;
#[doc = "PLL_SIC register accessor: an alias for `Reg<PLL_SIC_SPEC>`"]
pub type PLL_SIC = crate::Reg<pll_sic::PLL_SIC_SPEC>;
#[doc = "USB PHY PLL Control/Status Register"]
pub mod pll_sic;
#[doc = "PLL_SIC_SET register accessor: an alias for `Reg<PLL_SIC_SET_SPEC>`"]
pub type PLL_SIC_SET = crate::Reg<pll_sic_set::PLL_SIC_SET_SPEC>;
#[doc = "USB PHY PLL Control/Status Register"]
pub mod pll_sic_set;
#[doc = "PLL_SIC_CLR register accessor: an alias for `Reg<PLL_SIC_CLR_SPEC>`"]
pub type PLL_SIC_CLR = crate::Reg<pll_sic_clr::PLL_SIC_CLR_SPEC>;
#[doc = "USB PHY PLL Control/Status Register"]
pub mod pll_sic_clr;
#[doc = "PLL_SIC_TOG register accessor: an alias for `Reg<PLL_SIC_TOG_SPEC>`"]
pub type PLL_SIC_TOG = crate::Reg<pll_sic_tog::PLL_SIC_TOG_SPEC>;
#[doc = "USB PHY PLL Control/Status Register"]
pub mod pll_sic_tog;
#[doc = "USB1_VBUS_DETECT register accessor: an alias for `Reg<USB1_VBUS_DETECT_SPEC>`"]
pub type USB1_VBUS_DETECT = crate::Reg<usb1_vbus_detect::USB1_VBUS_DETECT_SPEC>;
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod usb1_vbus_detect;
#[doc = "USB1_VBUS_DETECT_SET register accessor: an alias for `Reg<USB1_VBUS_DETECT_SET_SPEC>`"]
pub type USB1_VBUS_DETECT_SET = crate::Reg<usb1_vbus_detect_set::USB1_VBUS_DETECT_SET_SPEC>;
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod usb1_vbus_detect_set;
#[doc = "USB1_VBUS_DETECT_CLR register accessor: an alias for `Reg<USB1_VBUS_DETECT_CLR_SPEC>`"]
pub type USB1_VBUS_DETECT_CLR = crate::Reg<usb1_vbus_detect_clr::USB1_VBUS_DETECT_CLR_SPEC>;
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod usb1_vbus_detect_clr;
#[doc = "USB1_VBUS_DETECT_TOG register accessor: an alias for `Reg<USB1_VBUS_DETECT_TOG_SPEC>`"]
pub type USB1_VBUS_DETECT_TOG = crate::Reg<usb1_vbus_detect_tog::USB1_VBUS_DETECT_TOG_SPEC>;
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod usb1_vbus_detect_tog;
#[doc = "ANACTRL register accessor: an alias for `Reg<ANACTRL_SPEC>`"]
pub type ANACTRL = crate::Reg<anactrl::ANACTRL_SPEC>;
#[doc = "USB PHY Analog Control Register"]
pub mod anactrl;
#[doc = "ANACTRL_SET register accessor: an alias for `Reg<ANACTRL_SET_SPEC>`"]
pub type ANACTRL_SET = crate::Reg<anactrl_set::ANACTRL_SET_SPEC>;
#[doc = "USB PHY Analog Control Register"]
pub mod anactrl_set;
#[doc = "ANACTRL_CLR register accessor: an alias for `Reg<ANACTRL_CLR_SPEC>`"]
pub type ANACTRL_CLR = crate::Reg<anactrl_clr::ANACTRL_CLR_SPEC>;
#[doc = "USB PHY Analog Control Register"]
pub mod anactrl_clr;
#[doc = "ANACTRL_TOG register accessor: an alias for `Reg<ANACTRL_TOG_SPEC>`"]
pub type ANACTRL_TOG = crate::Reg<anactrl_tog::ANACTRL_TOG_SPEC>;
#[doc = "USB PHY Analog Control Register"]
pub mod anactrl_tog;
