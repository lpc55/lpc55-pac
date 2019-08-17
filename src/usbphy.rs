#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB PHY Power-Down Register"]
    pub pwd: PWD,
    #[doc = "0x04 - USB PHY Power-Down Register"]
    pub pwd_set: PWD_SET,
    #[doc = "0x08 - USB PHY Power-Down Register"]
    pub pwd_clr: PWD_CLR,
    #[doc = "0x0c - USB PHY Power-Down Register"]
    pub pwd_tog: PWD_TOG,
    #[doc = "0x10 - USB PHY Transmitter Control Register"]
    pub tx: TX,
    #[doc = "0x14 - USB PHY Transmitter Control Register"]
    pub tx_set: TX_SET,
    #[doc = "0x18 - USB PHY Transmitter Control Register"]
    pub tx_clr: TX_CLR,
    #[doc = "0x1c - USB PHY Transmitter Control Register"]
    pub tx_tog: TX_TOG,
    #[doc = "0x20 - USB PHY Receiver Control Register"]
    pub rx: RX,
    #[doc = "0x24 - USB PHY Receiver Control Register"]
    pub rx_set: RX_SET,
    #[doc = "0x28 - USB PHY Receiver Control Register"]
    pub rx_clr: RX_CLR,
    #[doc = "0x2c - USB PHY Receiver Control Register"]
    pub rx_tog: RX_TOG,
    #[doc = "0x30 - USB PHY General Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x34 - USB PHY General Control Register"]
    pub ctrl_set: CTRL_SET,
    #[doc = "0x38 - USB PHY General Control Register"]
    pub ctrl_clr: CTRL_CLR,
    #[doc = "0x3c - USB PHY General Control Register"]
    pub ctrl_tog: CTRL_TOG,
    #[doc = "0x40 - USB PHY Status Register"]
    pub status: STATUS,
    _reserved17: [u8; 12usize],
    #[doc = "0x50 - USB PHY Debug Register 0"]
    pub debug0: DEBUG0,
    #[doc = "0x54 - USB PHY Debug Register 0"]
    pub debug0_set: DEBUG0_SET,
    #[doc = "0x58 - USB PHY Debug Register 0"]
    pub debug0_clr: DEBUG0_CLR,
    #[doc = "0x5c - USB PHY Debug Register 0"]
    pub debug0_tog: DEBUG0_TOG,
    _reserved21: [u8; 16usize],
    #[doc = "0x70 - UTMI Debug Status Register 1"]
    pub debug1: DEBUG1,
    #[doc = "0x74 - UTMI Debug Status Register 1"]
    pub debug1_set: DEBUG1_SET,
    #[doc = "0x78 - UTMI Debug Status Register 1"]
    pub debug1_clr: DEBUG1_CLR,
    #[doc = "0x7c - UTMI Debug Status Register 1"]
    pub debug1_tog: DEBUG1_TOG,
    #[doc = "0x80 - UTMI RTL Version"]
    pub version: VERSION,
    _reserved26: [u8; 28usize],
    #[doc = "0xa0 - USB PHY PLL Control/Status Register"]
    pub pll_sic: PLL_SIC,
    #[doc = "0xa4 - USB PHY PLL Control/Status Register"]
    pub pll_sic_set: PLL_SIC_SET,
    #[doc = "0xa8 - USB PHY PLL Control/Status Register"]
    pub pll_sic_clr: PLL_SIC_CLR,
    #[doc = "0xac - USB PHY PLL Control/Status Register"]
    pub pll_sic_tog: PLL_SIC_TOG,
    _reserved30: [u8; 16usize],
    #[doc = "0xc0 - USB PHY VBUS Detect Control Register"]
    pub usb1_vbus_detect: USB1_VBUS_DETECT,
    #[doc = "0xc4 - USB PHY VBUS Detect Control Register"]
    pub usb1_vbus_detect_set: USB1_VBUS_DETECT_SET,
    #[doc = "0xc8 - USB PHY VBUS Detect Control Register"]
    pub usb1_vbus_detect_clr: USB1_VBUS_DETECT_CLR,
    #[doc = "0xcc - USB PHY VBUS Detect Control Register"]
    pub usb1_vbus_detect_tog: USB1_VBUS_DETECT_TOG,
    #[doc = "0xd0 - USB PHY VBUS Detector Status Register"]
    pub usb1_vbus_det_stat: USB1_VBUS_DET_STAT,
    _reserved35: [u8; 12usize],
    #[doc = "0xe0 - USB PHY Charger Detect Control Register"]
    pub usb1_chrg_detect: USB1_CHRG_DETECT,
    #[doc = "0xe4 - USB PHY Charger Detect Control Register"]
    pub usb1_chrg_detect_set: USB1_CHRG_DETECT_SET,
    #[doc = "0xe8 - USB PHY Charger Detect Control Register"]
    pub usb1_chrg_detect_clr: USB1_CHRG_DETECT_CLR,
    #[doc = "0xec - USB PHY Charger Detect Control Register"]
    pub usb1_chrg_detect_tog: USB1_CHRG_DETECT_TOG,
    #[doc = "0xf0 - USB PHY Charger Detect Status Register"]
    pub usb1_chrg_det_stat: USB1_CHRG_DET_STAT,
    _reserved40: [u8; 12usize],
    #[doc = "0x100 - USB PHY Analog Control Register"]
    pub anactrl: ANACTRL,
    #[doc = "0x104 - USB PHY Analog Control Register"]
    pub anactrl_set: ANACTRL_SET,
    #[doc = "0x108 - USB PHY Analog Control Register"]
    pub anactrl_clr: ANACTRL_CLR,
    #[doc = "0x10c - USB PHY Analog Control Register"]
    pub anactrl_tog: ANACTRL_TOG,
}
#[doc = "USB PHY Power-Down Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwd](pwd) module"]
pub type PWD = crate::Reg<u32, _PWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWD;
#[doc = "`read()` method returns [pwd::R](pwd::R) reader structure"]
impl crate::Readable for PWD {}
#[doc = "`write(|w| ..)` method takes [pwd::W](pwd::W) writer structure"]
impl crate::Writable for PWD {}
#[doc = "USB PHY Power-Down Register"]
pub mod pwd;
#[doc = "USB PHY Power-Down Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwd_set](pwd_set) module"]
pub type PWD_SET = crate::Reg<u32, _PWD_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWD_SET;
#[doc = "`read()` method returns [pwd_set::R](pwd_set::R) reader structure"]
impl crate::Readable for PWD_SET {}
#[doc = "`write(|w| ..)` method takes [pwd_set::W](pwd_set::W) writer structure"]
impl crate::Writable for PWD_SET {}
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_set;
#[doc = "USB PHY Power-Down Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwd_clr](pwd_clr) module"]
pub type PWD_CLR = crate::Reg<u32, _PWD_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWD_CLR;
#[doc = "`read()` method returns [pwd_clr::R](pwd_clr::R) reader structure"]
impl crate::Readable for PWD_CLR {}
#[doc = "`write(|w| ..)` method takes [pwd_clr::W](pwd_clr::W) writer structure"]
impl crate::Writable for PWD_CLR {}
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_clr;
#[doc = "USB PHY Power-Down Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwd_tog](pwd_tog) module"]
pub type PWD_TOG = crate::Reg<u32, _PWD_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWD_TOG;
#[doc = "`read()` method returns [pwd_tog::R](pwd_tog::R) reader structure"]
impl crate::Readable for PWD_TOG {}
#[doc = "`write(|w| ..)` method takes [pwd_tog::W](pwd_tog::W) writer structure"]
impl crate::Writable for PWD_TOG {}
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_tog;
#[doc = "USB PHY Transmitter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx](tx) module"]
pub type TX = crate::Reg<u32, _TX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX;
#[doc = "`read()` method returns [tx::R](tx::R) reader structure"]
impl crate::Readable for TX {}
#[doc = "`write(|w| ..)` method takes [tx::W](tx::W) writer structure"]
impl crate::Writable for TX {}
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx;
#[doc = "USB PHY Transmitter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_set](tx_set) module"]
pub type TX_SET = crate::Reg<u32, _TX_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_SET;
#[doc = "`read()` method returns [tx_set::R](tx_set::R) reader structure"]
impl crate::Readable for TX_SET {}
#[doc = "`write(|w| ..)` method takes [tx_set::W](tx_set::W) writer structure"]
impl crate::Writable for TX_SET {}
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_set;
#[doc = "USB PHY Transmitter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_clr](tx_clr) module"]
pub type TX_CLR = crate::Reg<u32, _TX_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_CLR;
#[doc = "`read()` method returns [tx_clr::R](tx_clr::R) reader structure"]
impl crate::Readable for TX_CLR {}
#[doc = "`write(|w| ..)` method takes [tx_clr::W](tx_clr::W) writer structure"]
impl crate::Writable for TX_CLR {}
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_clr;
#[doc = "USB PHY Transmitter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_tog](tx_tog) module"]
pub type TX_TOG = crate::Reg<u32, _TX_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_TOG;
#[doc = "`read()` method returns [tx_tog::R](tx_tog::R) reader structure"]
impl crate::Readable for TX_TOG {}
#[doc = "`write(|w| ..)` method takes [tx_tog::W](tx_tog::W) writer structure"]
impl crate::Writable for TX_TOG {}
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_tog;
#[doc = "USB PHY Receiver Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx](rx) module"]
pub type RX = crate::Reg<u32, _RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX;
#[doc = "`read()` method returns [rx::R](rx::R) reader structure"]
impl crate::Readable for RX {}
#[doc = "`write(|w| ..)` method takes [rx::W](rx::W) writer structure"]
impl crate::Writable for RX {}
#[doc = "USB PHY Receiver Control Register"]
pub mod rx;
#[doc = "USB PHY Receiver Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_set](rx_set) module"]
pub type RX_SET = crate::Reg<u32, _RX_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_SET;
#[doc = "`read()` method returns [rx_set::R](rx_set::R) reader structure"]
impl crate::Readable for RX_SET {}
#[doc = "`write(|w| ..)` method takes [rx_set::W](rx_set::W) writer structure"]
impl crate::Writable for RX_SET {}
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_set;
#[doc = "USB PHY Receiver Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_clr](rx_clr) module"]
pub type RX_CLR = crate::Reg<u32, _RX_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_CLR;
#[doc = "`read()` method returns [rx_clr::R](rx_clr::R) reader structure"]
impl crate::Readable for RX_CLR {}
#[doc = "`write(|w| ..)` method takes [rx_clr::W](rx_clr::W) writer structure"]
impl crate::Writable for RX_CLR {}
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_clr;
#[doc = "USB PHY Receiver Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_tog](rx_tog) module"]
pub type RX_TOG = crate::Reg<u32, _RX_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_TOG;
#[doc = "`read()` method returns [rx_tog::R](rx_tog::R) reader structure"]
impl crate::Readable for RX_TOG {}
#[doc = "`write(|w| ..)` method takes [rx_tog::W](rx_tog::W) writer structure"]
impl crate::Writable for RX_TOG {}
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_tog;
#[doc = "USB PHY General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "USB PHY General Control Register"]
pub mod ctrl;
#[doc = "USB PHY General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl_set](ctrl_set) module"]
pub type CTRL_SET = crate::Reg<u32, _CTRL_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_SET;
#[doc = "`read()` method returns [ctrl_set::R](ctrl_set::R) reader structure"]
impl crate::Readable for CTRL_SET {}
#[doc = "`write(|w| ..)` method takes [ctrl_set::W](ctrl_set::W) writer structure"]
impl crate::Writable for CTRL_SET {}
#[doc = "USB PHY General Control Register"]
pub mod ctrl_set;
#[doc = "USB PHY General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl_clr](ctrl_clr) module"]
pub type CTRL_CLR = crate::Reg<u32, _CTRL_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_CLR;
#[doc = "`read()` method returns [ctrl_clr::R](ctrl_clr::R) reader structure"]
impl crate::Readable for CTRL_CLR {}
#[doc = "`write(|w| ..)` method takes [ctrl_clr::W](ctrl_clr::W) writer structure"]
impl crate::Writable for CTRL_CLR {}
#[doc = "USB PHY General Control Register"]
pub mod ctrl_clr;
#[doc = "USB PHY General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl_tog](ctrl_tog) module"]
pub type CTRL_TOG = crate::Reg<u32, _CTRL_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_TOG;
#[doc = "`read()` method returns [ctrl_tog::R](ctrl_tog::R) reader structure"]
impl crate::Readable for CTRL_TOG {}
#[doc = "`write(|w| ..)` method takes [ctrl_tog::W](ctrl_tog::W) writer structure"]
impl crate::Writable for CTRL_TOG {}
#[doc = "USB PHY General Control Register"]
pub mod ctrl_tog;
#[doc = "USB PHY Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "USB PHY Status Register"]
pub mod status;
#[doc = "USB PHY Debug Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [debug0](debug0) module"]
pub type DEBUG0 = crate::Reg<u32, _DEBUG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG0;
#[doc = "`read()` method returns [debug0::R](debug0::R) reader structure"]
impl crate::Readable for DEBUG0 {}
#[doc = "`write(|w| ..)` method takes [debug0::W](debug0::W) writer structure"]
impl crate::Writable for DEBUG0 {}
#[doc = "USB PHY Debug Register 0"]
pub mod debug0;
#[doc = "USB PHY Debug Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [debug0_set](debug0_set) module"]
pub type DEBUG0_SET = crate::Reg<u32, _DEBUG0_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG0_SET;
#[doc = "`read()` method returns [debug0_set::R](debug0_set::R) reader structure"]
impl crate::Readable for DEBUG0_SET {}
#[doc = "`write(|w| ..)` method takes [debug0_set::W](debug0_set::W) writer structure"]
impl crate::Writable for DEBUG0_SET {}
#[doc = "USB PHY Debug Register 0"]
pub mod debug0_set;
#[doc = "USB PHY Debug Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [debug0_clr](debug0_clr) module"]
pub type DEBUG0_CLR = crate::Reg<u32, _DEBUG0_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG0_CLR;
#[doc = "`read()` method returns [debug0_clr::R](debug0_clr::R) reader structure"]
impl crate::Readable for DEBUG0_CLR {}
#[doc = "`write(|w| ..)` method takes [debug0_clr::W](debug0_clr::W) writer structure"]
impl crate::Writable for DEBUG0_CLR {}
#[doc = "USB PHY Debug Register 0"]
pub mod debug0_clr;
#[doc = "USB PHY Debug Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [debug0_tog](debug0_tog) module"]
pub type DEBUG0_TOG = crate::Reg<u32, _DEBUG0_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG0_TOG;
#[doc = "`read()` method returns [debug0_tog::R](debug0_tog::R) reader structure"]
impl crate::Readable for DEBUG0_TOG {}
#[doc = "`write(|w| ..)` method takes [debug0_tog::W](debug0_tog::W) writer structure"]
impl crate::Writable for DEBUG0_TOG {}
#[doc = "USB PHY Debug Register 0"]
pub mod debug0_tog;
#[doc = "UTMI Debug Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [debug1](debug1) module"]
pub type DEBUG1 = crate::Reg<u32, _DEBUG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG1;
#[doc = "`read()` method returns [debug1::R](debug1::R) reader structure"]
impl crate::Readable for DEBUG1 {}
#[doc = "`write(|w| ..)` method takes [debug1::W](debug1::W) writer structure"]
impl crate::Writable for DEBUG1 {}
#[doc = "UTMI Debug Status Register 1"]
pub mod debug1;
#[doc = "UTMI Debug Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [debug1_set](debug1_set) module"]
pub type DEBUG1_SET = crate::Reg<u32, _DEBUG1_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG1_SET;
#[doc = "`read()` method returns [debug1_set::R](debug1_set::R) reader structure"]
impl crate::Readable for DEBUG1_SET {}
#[doc = "`write(|w| ..)` method takes [debug1_set::W](debug1_set::W) writer structure"]
impl crate::Writable for DEBUG1_SET {}
#[doc = "UTMI Debug Status Register 1"]
pub mod debug1_set;
#[doc = "UTMI Debug Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [debug1_clr](debug1_clr) module"]
pub type DEBUG1_CLR = crate::Reg<u32, _DEBUG1_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG1_CLR;
#[doc = "`read()` method returns [debug1_clr::R](debug1_clr::R) reader structure"]
impl crate::Readable for DEBUG1_CLR {}
#[doc = "`write(|w| ..)` method takes [debug1_clr::W](debug1_clr::W) writer structure"]
impl crate::Writable for DEBUG1_CLR {}
#[doc = "UTMI Debug Status Register 1"]
pub mod debug1_clr;
#[doc = "UTMI Debug Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [debug1_tog](debug1_tog) module"]
pub type DEBUG1_TOG = crate::Reg<u32, _DEBUG1_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG1_TOG;
#[doc = "`read()` method returns [debug1_tog::R](debug1_tog::R) reader structure"]
impl crate::Readable for DEBUG1_TOG {}
#[doc = "`write(|w| ..)` method takes [debug1_tog::W](debug1_tog::W) writer structure"]
impl crate::Writable for DEBUG1_TOG {}
#[doc = "UTMI Debug Status Register 1"]
pub mod debug1_tog;
#[doc = "UTMI RTL Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "UTMI RTL Version"]
pub mod version;
#[doc = "USB PHY PLL Control/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_sic](pll_sic) module"]
pub type PLL_SIC = crate::Reg<u32, _PLL_SIC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_SIC;
#[doc = "`read()` method returns [pll_sic::R](pll_sic::R) reader structure"]
impl crate::Readable for PLL_SIC {}
#[doc = "`write(|w| ..)` method takes [pll_sic::W](pll_sic::W) writer structure"]
impl crate::Writable for PLL_SIC {}
#[doc = "USB PHY PLL Control/Status Register"]
pub mod pll_sic;
#[doc = "USB PHY PLL Control/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_sic_set](pll_sic_set) module"]
pub type PLL_SIC_SET = crate::Reg<u32, _PLL_SIC_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_SIC_SET;
#[doc = "`read()` method returns [pll_sic_set::R](pll_sic_set::R) reader structure"]
impl crate::Readable for PLL_SIC_SET {}
#[doc = "`write(|w| ..)` method takes [pll_sic_set::W](pll_sic_set::W) writer structure"]
impl crate::Writable for PLL_SIC_SET {}
#[doc = "USB PHY PLL Control/Status Register"]
pub mod pll_sic_set;
#[doc = "USB PHY PLL Control/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_sic_clr](pll_sic_clr) module"]
pub type PLL_SIC_CLR = crate::Reg<u32, _PLL_SIC_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_SIC_CLR;
#[doc = "`read()` method returns [pll_sic_clr::R](pll_sic_clr::R) reader structure"]
impl crate::Readable for PLL_SIC_CLR {}
#[doc = "`write(|w| ..)` method takes [pll_sic_clr::W](pll_sic_clr::W) writer structure"]
impl crate::Writable for PLL_SIC_CLR {}
#[doc = "USB PHY PLL Control/Status Register"]
pub mod pll_sic_clr;
#[doc = "USB PHY PLL Control/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_sic_tog](pll_sic_tog) module"]
pub type PLL_SIC_TOG = crate::Reg<u32, _PLL_SIC_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_SIC_TOG;
#[doc = "`read()` method returns [pll_sic_tog::R](pll_sic_tog::R) reader structure"]
impl crate::Readable for PLL_SIC_TOG {}
#[doc = "`write(|w| ..)` method takes [pll_sic_tog::W](pll_sic_tog::W) writer structure"]
impl crate::Writable for PLL_SIC_TOG {}
#[doc = "USB PHY PLL Control/Status Register"]
pub mod pll_sic_tog;
#[doc = "USB PHY VBUS Detect Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb1_vbus_detect](usb1_vbus_detect) module"]
pub type USB1_VBUS_DETECT = crate::Reg<u32, _USB1_VBUS_DETECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_VBUS_DETECT;
#[doc = "`read()` method returns [usb1_vbus_detect::R](usb1_vbus_detect::R) reader structure"]
impl crate::Readable for USB1_VBUS_DETECT {}
#[doc = "`write(|w| ..)` method takes [usb1_vbus_detect::W](usb1_vbus_detect::W) writer structure"]
impl crate::Writable for USB1_VBUS_DETECT {}
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod usb1_vbus_detect;
#[doc = "USB PHY VBUS Detect Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb1_vbus_detect_set](usb1_vbus_detect_set) module"]
pub type USB1_VBUS_DETECT_SET = crate::Reg<u32, _USB1_VBUS_DETECT_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_VBUS_DETECT_SET;
#[doc = "`read()` method returns [usb1_vbus_detect_set::R](usb1_vbus_detect_set::R) reader structure"]
impl crate::Readable for USB1_VBUS_DETECT_SET {}
#[doc = "`write(|w| ..)` method takes [usb1_vbus_detect_set::W](usb1_vbus_detect_set::W) writer structure"]
impl crate::Writable for USB1_VBUS_DETECT_SET {}
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod usb1_vbus_detect_set;
#[doc = "USB PHY VBUS Detect Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb1_vbus_detect_clr](usb1_vbus_detect_clr) module"]
pub type USB1_VBUS_DETECT_CLR = crate::Reg<u32, _USB1_VBUS_DETECT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_VBUS_DETECT_CLR;
#[doc = "`read()` method returns [usb1_vbus_detect_clr::R](usb1_vbus_detect_clr::R) reader structure"]
impl crate::Readable for USB1_VBUS_DETECT_CLR {}
#[doc = "`write(|w| ..)` method takes [usb1_vbus_detect_clr::W](usb1_vbus_detect_clr::W) writer structure"]
impl crate::Writable for USB1_VBUS_DETECT_CLR {}
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod usb1_vbus_detect_clr;
#[doc = "USB PHY VBUS Detect Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb1_vbus_detect_tog](usb1_vbus_detect_tog) module"]
pub type USB1_VBUS_DETECT_TOG = crate::Reg<u32, _USB1_VBUS_DETECT_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_VBUS_DETECT_TOG;
#[doc = "`read()` method returns [usb1_vbus_detect_tog::R](usb1_vbus_detect_tog::R) reader structure"]
impl crate::Readable for USB1_VBUS_DETECT_TOG {}
#[doc = "`write(|w| ..)` method takes [usb1_vbus_detect_tog::W](usb1_vbus_detect_tog::W) writer structure"]
impl crate::Writable for USB1_VBUS_DETECT_TOG {}
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod usb1_vbus_detect_tog;
#[doc = "USB PHY VBUS Detector Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb1_vbus_det_stat](usb1_vbus_det_stat) module"]
pub type USB1_VBUS_DET_STAT = crate::Reg<u32, _USB1_VBUS_DET_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_VBUS_DET_STAT;
#[doc = "`read()` method returns [usb1_vbus_det_stat::R](usb1_vbus_det_stat::R) reader structure"]
impl crate::Readable for USB1_VBUS_DET_STAT {}
#[doc = "USB PHY VBUS Detector Status Register"]
pub mod usb1_vbus_det_stat;
#[doc = "USB PHY Charger Detect Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb1_chrg_detect](usb1_chrg_detect) module"]
pub type USB1_CHRG_DETECT = crate::Reg<u32, _USB1_CHRG_DETECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_CHRG_DETECT;
#[doc = "`read()` method returns [usb1_chrg_detect::R](usb1_chrg_detect::R) reader structure"]
impl crate::Readable for USB1_CHRG_DETECT {}
#[doc = "`write(|w| ..)` method takes [usb1_chrg_detect::W](usb1_chrg_detect::W) writer structure"]
impl crate::Writable for USB1_CHRG_DETECT {}
#[doc = "USB PHY Charger Detect Control Register"]
pub mod usb1_chrg_detect;
#[doc = "USB PHY Charger Detect Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb1_chrg_detect_set](usb1_chrg_detect_set) module"]
pub type USB1_CHRG_DETECT_SET = crate::Reg<u32, _USB1_CHRG_DETECT_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_CHRG_DETECT_SET;
#[doc = "`read()` method returns [usb1_chrg_detect_set::R](usb1_chrg_detect_set::R) reader structure"]
impl crate::Readable for USB1_CHRG_DETECT_SET {}
#[doc = "`write(|w| ..)` method takes [usb1_chrg_detect_set::W](usb1_chrg_detect_set::W) writer structure"]
impl crate::Writable for USB1_CHRG_DETECT_SET {}
#[doc = "USB PHY Charger Detect Control Register"]
pub mod usb1_chrg_detect_set;
#[doc = "USB PHY Charger Detect Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb1_chrg_detect_clr](usb1_chrg_detect_clr) module"]
pub type USB1_CHRG_DETECT_CLR = crate::Reg<u32, _USB1_CHRG_DETECT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_CHRG_DETECT_CLR;
#[doc = "`read()` method returns [usb1_chrg_detect_clr::R](usb1_chrg_detect_clr::R) reader structure"]
impl crate::Readable for USB1_CHRG_DETECT_CLR {}
#[doc = "`write(|w| ..)` method takes [usb1_chrg_detect_clr::W](usb1_chrg_detect_clr::W) writer structure"]
impl crate::Writable for USB1_CHRG_DETECT_CLR {}
#[doc = "USB PHY Charger Detect Control Register"]
pub mod usb1_chrg_detect_clr;
#[doc = "USB PHY Charger Detect Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb1_chrg_detect_tog](usb1_chrg_detect_tog) module"]
pub type USB1_CHRG_DETECT_TOG = crate::Reg<u32, _USB1_CHRG_DETECT_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_CHRG_DETECT_TOG;
#[doc = "`read()` method returns [usb1_chrg_detect_tog::R](usb1_chrg_detect_tog::R) reader structure"]
impl crate::Readable for USB1_CHRG_DETECT_TOG {}
#[doc = "`write(|w| ..)` method takes [usb1_chrg_detect_tog::W](usb1_chrg_detect_tog::W) writer structure"]
impl crate::Writable for USB1_CHRG_DETECT_TOG {}
#[doc = "USB PHY Charger Detect Control Register"]
pub mod usb1_chrg_detect_tog;
#[doc = "USB PHY Charger Detect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb1_chrg_det_stat](usb1_chrg_det_stat) module"]
pub type USB1_CHRG_DET_STAT = crate::Reg<u32, _USB1_CHRG_DET_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_CHRG_DET_STAT;
#[doc = "`read()` method returns [usb1_chrg_det_stat::R](usb1_chrg_det_stat::R) reader structure"]
impl crate::Readable for USB1_CHRG_DET_STAT {}
#[doc = "USB PHY Charger Detect Status Register"]
pub mod usb1_chrg_det_stat;
#[doc = "USB PHY Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [anactrl](anactrl) module"]
pub type ANACTRL = crate::Reg<u32, _ANACTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANACTRL;
#[doc = "`read()` method returns [anactrl::R](anactrl::R) reader structure"]
impl crate::Readable for ANACTRL {}
#[doc = "`write(|w| ..)` method takes [anactrl::W](anactrl::W) writer structure"]
impl crate::Writable for ANACTRL {}
#[doc = "USB PHY Analog Control Register"]
pub mod anactrl;
#[doc = "USB PHY Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [anactrl_set](anactrl_set) module"]
pub type ANACTRL_SET = crate::Reg<u32, _ANACTRL_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANACTRL_SET;
#[doc = "`read()` method returns [anactrl_set::R](anactrl_set::R) reader structure"]
impl crate::Readable for ANACTRL_SET {}
#[doc = "`write(|w| ..)` method takes [anactrl_set::W](anactrl_set::W) writer structure"]
impl crate::Writable for ANACTRL_SET {}
#[doc = "USB PHY Analog Control Register"]
pub mod anactrl_set;
#[doc = "USB PHY Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [anactrl_clr](anactrl_clr) module"]
pub type ANACTRL_CLR = crate::Reg<u32, _ANACTRL_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANACTRL_CLR;
#[doc = "`read()` method returns [anactrl_clr::R](anactrl_clr::R) reader structure"]
impl crate::Readable for ANACTRL_CLR {}
#[doc = "`write(|w| ..)` method takes [anactrl_clr::W](anactrl_clr::W) writer structure"]
impl crate::Writable for ANACTRL_CLR {}
#[doc = "USB PHY Analog Control Register"]
pub mod anactrl_clr;
#[doc = "USB PHY Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [anactrl_tog](anactrl_tog) module"]
pub type ANACTRL_TOG = crate::Reg<u32, _ANACTRL_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANACTRL_TOG;
#[doc = "`read()` method returns [anactrl_tog::R](anactrl_tog::R) reader structure"]
impl crate::Readable for ANACTRL_TOG {}
#[doc = "`write(|w| ..)` method takes [anactrl_tog::W](anactrl_tog::W) writer structure"]
impl crate::Writable for ANACTRL_TOG {}
#[doc = "USB PHY Analog Control Register"]
pub mod anactrl_tog;
