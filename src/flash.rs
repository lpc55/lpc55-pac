#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - command register"]
    pub cmd: CMD,
    #[doc = "0x04 - event register"]
    pub event: EVENT,
    #[doc = "0x08 - read burst register"]
    pub burst: BURST,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - start (or only) address for next flash command"]
    pub starta: STARTA,
    #[doc = "0x14 - end address for next flash command, if command operates on address ranges"]
    pub stopa: STOPA,
    _reserved1: [u8; 104usize],
    #[doc = "0x80 - data register, word 0-7; Memory data, or command parameter, or command result."]
    pub dataw: [DATAW; 8],
    _reserved2: [u8; 3896usize],
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
    _reserved3: [u8; 12usize],
    #[doc = "0xffc - Controller+Memory module identification"]
    pub module_id: MODULE_ID,
}
#[doc = "command register"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "command register"]
pub mod cmd;
#[doc = "event register"]
pub struct EVENT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "event register"]
pub mod event;
#[doc = "read burst register"]
pub struct BURST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "read burst register"]
pub mod burst;
#[doc = "start (or only) address for next flash command"]
pub struct STARTA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "start (or only) address for next flash command"]
pub mod starta;
#[doc = "end address for next flash command, if command operates on address ranges"]
pub struct STOPA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "end address for next flash command, if command operates on address ranges"]
pub mod stopa;
#[doc = "data register, word 0-7; Memory data, or command parameter, or command result."]
pub struct DATAW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data register, word 0-7; Memory data, or command parameter, or command result."]
pub mod dataw;
#[doc = "Clear interrupt enable bits"]
pub struct INT_CLR_ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear interrupt enable bits"]
pub mod int_clr_enable;
#[doc = "Set interrupt enable bits"]
pub struct INT_SET_ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set interrupt enable bits"]
pub mod int_set_enable;
#[doc = "Interrupt status bits"]
pub struct INT_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt status bits"]
pub mod int_status;
#[doc = "Interrupt enable bits"]
pub struct INT_ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable bits"]
pub mod int_enable;
#[doc = "Clear interrupt status bits"]
pub struct INT_CLR_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear interrupt status bits"]
pub mod int_clr_status;
#[doc = "Set interrupt status bits"]
pub struct INT_SET_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set interrupt status bits"]
pub mod int_set_status;
#[doc = "Controller+Memory module identification"]
pub struct MODULE_ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controller+Memory module identification"]
pub mod module_id;
