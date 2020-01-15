#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - command register"]
    pub cmd: CMD,
    #[doc = "0x04 - event register"]
    pub event: EVENT,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - start (or only) address for next flash command"]
    pub starta: STARTA,
    #[doc = "0x14 - end address for next flash command, if command operates on address ranges"]
    pub stopa: STOPA,
    _reserved4: [u8; 104usize],
    #[doc = "0x80 - data register, word 0-7; Memory data, or command parameter, or command result."]
    pub dataw: [DATAW; 4],
    _reserved5: [u8; 3912usize],
    #[doc = "0xfd8 - Clear interrupt enable bits"]
    pub int_clr_enable: INT_CLR_ENABLE,
    #[doc = "0xfdc - Set interrupt enable bits"]
    pub int_set_enable: INT_SET_ENABLE,
    #[doc = "0xfe0 - Interrupt status bits"]
    pub int_status: INT_STATUS,
    #[doc = "0xfe4 - Interrupt enable bits"]
    pub int_enable: INT_ENABLE,
    #[doc = "0xfe8 - Clear interrupt status bits"]
    pub int_clr_status: INT_CLR_STATUS,
    #[doc = "0xfec - Set interrupt status bits"]
    pub int_set_status: INT_SET_STATUS,
    _reserved11: [u8; 12usize],
    #[doc = "0xffc - Controller+Memory module identification"]
    pub module_id: MODULE_ID,
}
#[doc = "command register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "command register"]
pub mod cmd;
#[doc = "event register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [event](event) module"]
pub type EVENT = crate::Reg<u32, _EVENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENT;
#[doc = "`write(|w| ..)` method takes [event::W](event::W) writer structure"]
impl crate::Writable for EVENT {}
#[doc = "event register"]
pub mod event;
#[doc = "start (or only) address for next flash command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starta](starta) module"]
pub type STARTA = crate::Reg<u32, _STARTA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STARTA;
#[doc = "`read()` method returns [starta::R](starta::R) reader structure"]
impl crate::Readable for STARTA {}
#[doc = "`write(|w| ..)` method takes [starta::W](starta::W) writer structure"]
impl crate::Writable for STARTA {}
#[doc = "start (or only) address for next flash command"]
pub mod starta;
#[doc = "end address for next flash command, if command operates on address ranges\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stopa](stopa) module"]
pub type STOPA = crate::Reg<u32, _STOPA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STOPA;
#[doc = "`read()` method returns [stopa::R](stopa::R) reader structure"]
impl crate::Readable for STOPA {}
#[doc = "`write(|w| ..)` method takes [stopa::W](stopa::W) writer structure"]
impl crate::Writable for STOPA {}
#[doc = "end address for next flash command, if command operates on address ranges"]
pub mod stopa;
#[doc = "data register, word 0-7; Memory data, or command parameter, or command result.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dataw](dataw) module"]
pub type DATAW = crate::Reg<u32, _DATAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW;
#[doc = "`read()` method returns [dataw::R](dataw::R) reader structure"]
impl crate::Readable for DATAW {}
#[doc = "`write(|w| ..)` method takes [dataw::W](dataw::W) writer structure"]
impl crate::Writable for DATAW {}
#[doc = "data register, word 0-7; Memory data, or command parameter, or command result."]
pub mod dataw;
#[doc = "Clear interrupt enable bits\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr_enable](int_clr_enable) module"]
pub type INT_CLR_ENABLE = crate::Reg<u32, _INT_CLR_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR_ENABLE;
#[doc = "`write(|w| ..)` method takes [int_clr_enable::W](int_clr_enable::W) writer structure"]
impl crate::Writable for INT_CLR_ENABLE {}
#[doc = "Clear interrupt enable bits"]
pub mod int_clr_enable;
#[doc = "Set interrupt enable bits\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_set_enable](int_set_enable) module"]
pub type INT_SET_ENABLE = crate::Reg<u32, _INT_SET_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_SET_ENABLE;
#[doc = "`write(|w| ..)` method takes [int_set_enable::W](int_set_enable::W) writer structure"]
impl crate::Writable for INT_SET_ENABLE {}
#[doc = "Set interrupt enable bits"]
pub mod int_set_enable;
#[doc = "Interrupt status bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status](int_status) module"]
pub type INT_STATUS = crate::Reg<u32, _INT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_STATUS;
#[doc = "`read()` method returns [int_status::R](int_status::R) reader structure"]
impl crate::Readable for INT_STATUS {}
#[doc = "`write(|w| ..)` method takes [int_status::W](int_status::W) writer structure"]
impl crate::Writable for INT_STATUS {}
#[doc = "Interrupt status bits"]
pub mod int_status;
#[doc = "Interrupt enable bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_enable](int_enable) module"]
pub type INT_ENABLE = crate::Reg<u32, _INT_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENABLE;
#[doc = "`read()` method returns [int_enable::R](int_enable::R) reader structure"]
impl crate::Readable for INT_ENABLE {}
#[doc = "`write(|w| ..)` method takes [int_enable::W](int_enable::W) writer structure"]
impl crate::Writable for INT_ENABLE {}
#[doc = "Interrupt enable bits"]
pub mod int_enable;
#[doc = "Clear interrupt status bits\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr_status](int_clr_status) module"]
pub type INT_CLR_STATUS = crate::Reg<u32, _INT_CLR_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR_STATUS;
#[doc = "`write(|w| ..)` method takes [int_clr_status::W](int_clr_status::W) writer structure"]
impl crate::Writable for INT_CLR_STATUS {}
#[doc = "Clear interrupt status bits"]
pub mod int_clr_status;
#[doc = "Set interrupt status bits\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_set_status](int_set_status) module"]
pub type INT_SET_STATUS = crate::Reg<u32, _INT_SET_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_SET_STATUS;
#[doc = "`write(|w| ..)` method takes [int_set_status::W](int_set_status::W) writer structure"]
impl crate::Writable for INT_SET_STATUS {}
#[doc = "Set interrupt status bits"]
pub mod int_set_status;
#[doc = "Controller+Memory module identification\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [module_id](module_id) module"]
pub type MODULE_ID = crate::Reg<u32, _MODULE_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODULE_ID;
#[doc = "`read()` method returns [module_id::R](module_id::R) reader structure"]
impl crate::Readable for MODULE_ID {}
#[doc = "Controller+Memory module identification"]
pub mod module_id;
