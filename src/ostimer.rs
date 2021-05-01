#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EVTIMER Low Register"]
    pub evtimerl: crate::Reg<evtimerl::EVTIMERL_SPEC>,
    #[doc = "0x04 - EVTIMER High Register"]
    pub evtimerh: crate::Reg<evtimerh::EVTIMERH_SPEC>,
    #[doc = "0x08 - Capture Low Register"]
    pub capture_l: crate::Reg<capture_l::CAPTURE_L_SPEC>,
    #[doc = "0x0c - Capture High Register"]
    pub capture_h: crate::Reg<capture_h::CAPTURE_H_SPEC>,
    #[doc = "0x10 - Match Low Register"]
    pub match_l: crate::Reg<match_l::MATCH_L_SPEC>,
    #[doc = "0x14 - Match High Register"]
    pub match_h: crate::Reg<match_h::MATCH_H_SPEC>,
    _reserved6: [u8; 4usize],
    #[doc = "0x1c - OS_EVENT TIMER Control Register"]
    pub osevent_ctrl: crate::Reg<osevent_ctrl::OSEVENT_CTRL_SPEC>,
}
#[doc = "EVTIMERL register accessor: an alias for `Reg<EVTIMERL_SPEC>`"]
pub type EVTIMERL = crate::Reg<evtimerl::EVTIMERL_SPEC>;
#[doc = "EVTIMER Low Register"]
pub mod evtimerl;
#[doc = "EVTIMERH register accessor: an alias for `Reg<EVTIMERH_SPEC>`"]
pub type EVTIMERH = crate::Reg<evtimerh::EVTIMERH_SPEC>;
#[doc = "EVTIMER High Register"]
pub mod evtimerh;
#[doc = "CAPTURE_L register accessor: an alias for `Reg<CAPTURE_L_SPEC>`"]
pub type CAPTURE_L = crate::Reg<capture_l::CAPTURE_L_SPEC>;
#[doc = "Capture Low Register"]
pub mod capture_l;
#[doc = "CAPTURE_H register accessor: an alias for `Reg<CAPTURE_H_SPEC>`"]
pub type CAPTURE_H = crate::Reg<capture_h::CAPTURE_H_SPEC>;
#[doc = "Capture High Register"]
pub mod capture_h;
#[doc = "MATCH_L register accessor: an alias for `Reg<MATCH_L_SPEC>`"]
pub type MATCH_L = crate::Reg<match_l::MATCH_L_SPEC>;
#[doc = "Match Low Register"]
pub mod match_l;
#[doc = "MATCH_H register accessor: an alias for `Reg<MATCH_H_SPEC>`"]
pub type MATCH_H = crate::Reg<match_h::MATCH_H_SPEC>;
#[doc = "Match High Register"]
pub mod match_h;
#[doc = "OSEVENT_CTRL register accessor: an alias for `Reg<OSEVENT_CTRL_SPEC>`"]
pub type OSEVENT_CTRL = crate::Reg<osevent_ctrl::OSEVENT_CTRL_SPEC>;
#[doc = "OS_EVENT TIMER Control Register"]
pub mod osevent_ctrl;
