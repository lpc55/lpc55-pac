#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - command register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x04 - event register"]
    pub event: crate::Reg<event::EVENT_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - start (or only) address for next flash command"]
    pub starta: crate::Reg<starta::STARTA_SPEC>,
    #[doc = "0x14 - end address for next flash command, if command operates on address ranges"]
    pub stopa: crate::Reg<stopa::STOPA_SPEC>,
    _reserved4: [u8; 0x68],
    #[doc = "0x80..0x90 - data register, word 0-7; Memory data, or command parameter, or command result."]
    pub dataw: [crate::Reg<dataw::DATAW_SPEC>; 4],
    _reserved5: [u8; 0x0f48],
    #[doc = "0xfd8 - Clear interrupt enable bits"]
    pub int_clr_enable: crate::Reg<int_clr_enable::INT_CLR_ENABLE_SPEC>,
    #[doc = "0xfdc - Set interrupt enable bits"]
    pub int_set_enable: crate::Reg<int_set_enable::INT_SET_ENABLE_SPEC>,
    #[doc = "0xfe0 - Interrupt status bits"]
    pub int_status: crate::Reg<int_status::INT_STATUS_SPEC>,
    #[doc = "0xfe4 - Interrupt enable bits"]
    pub int_enable: crate::Reg<int_enable::INT_ENABLE_SPEC>,
    #[doc = "0xfe8 - Clear interrupt status bits"]
    pub int_clr_status: crate::Reg<int_clr_status::INT_CLR_STATUS_SPEC>,
    #[doc = "0xfec - Set interrupt status bits"]
    pub int_set_status: crate::Reg<int_set_status::INT_SET_STATUS_SPEC>,
    _reserved11: [u8; 0x0c],
    #[doc = "0xffc - Controller+Memory module identification"]
    pub module_id: crate::Reg<module_id::MODULE_ID_SPEC>,
}
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "command register"]
pub mod cmd;
#[doc = "EVENT register accessor: an alias for `Reg<EVENT_SPEC>`"]
pub type EVENT = crate::Reg<event::EVENT_SPEC>;
#[doc = "event register"]
pub mod event;
#[doc = "STARTA register accessor: an alias for `Reg<STARTA_SPEC>`"]
pub type STARTA = crate::Reg<starta::STARTA_SPEC>;
#[doc = "start (or only) address for next flash command"]
pub mod starta;
#[doc = "STOPA register accessor: an alias for `Reg<STOPA_SPEC>`"]
pub type STOPA = crate::Reg<stopa::STOPA_SPEC>;
#[doc = "end address for next flash command, if command operates on address ranges"]
pub mod stopa;
#[doc = "DATAW register accessor: an alias for `Reg<DATAW_SPEC>`"]
pub type DATAW = crate::Reg<dataw::DATAW_SPEC>;
#[doc = "data register, word 0-7; Memory data, or command parameter, or command result."]
pub mod dataw;
#[doc = "INT_CLR_ENABLE register accessor: an alias for `Reg<INT_CLR_ENABLE_SPEC>`"]
pub type INT_CLR_ENABLE = crate::Reg<int_clr_enable::INT_CLR_ENABLE_SPEC>;
#[doc = "Clear interrupt enable bits"]
pub mod int_clr_enable;
#[doc = "INT_SET_ENABLE register accessor: an alias for `Reg<INT_SET_ENABLE_SPEC>`"]
pub type INT_SET_ENABLE = crate::Reg<int_set_enable::INT_SET_ENABLE_SPEC>;
#[doc = "Set interrupt enable bits"]
pub mod int_set_enable;
#[doc = "INT_STATUS register accessor: an alias for `Reg<INT_STATUS_SPEC>`"]
pub type INT_STATUS = crate::Reg<int_status::INT_STATUS_SPEC>;
#[doc = "Interrupt status bits"]
pub mod int_status;
#[doc = "INT_ENABLE register accessor: an alias for `Reg<INT_ENABLE_SPEC>`"]
pub type INT_ENABLE = crate::Reg<int_enable::INT_ENABLE_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_enable;
#[doc = "INT_CLR_STATUS register accessor: an alias for `Reg<INT_CLR_STATUS_SPEC>`"]
pub type INT_CLR_STATUS = crate::Reg<int_clr_status::INT_CLR_STATUS_SPEC>;
#[doc = "Clear interrupt status bits"]
pub mod int_clr_status;
#[doc = "INT_SET_STATUS register accessor: an alias for `Reg<INT_SET_STATUS_SPEC>`"]
pub type INT_SET_STATUS = crate::Reg<int_set_status::INT_SET_STATUS_SPEC>;
#[doc = "Set interrupt status bits"]
pub mod int_set_status;
#[doc = "MODULE_ID register accessor: an alias for `Reg<MODULE_ID_SPEC>`"]
pub type MODULE_ID = crate::Reg<module_id::MODULE_ID_SPEC>;
#[doc = "Controller+Memory module identification"]
pub mod module_id;
