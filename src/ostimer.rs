#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EVTIMER Low Register"]
    pub evtimerl: EVTIMERL,
    #[doc = "0x04 - EVTIMER High Register"]
    pub evtimerh: EVTIMERH,
    #[doc = "0x08 - Local Capture Low Register for CPUn"]
    pub capturen_l: CAPTUREN_L,
    #[doc = "0x0c - Local Capture High Register for CPUn"]
    pub capturen_h: CAPTUREN_H,
    #[doc = "0x10 - Local Match Low Register for CPUn"]
    pub matchn_l: MATCHN_L,
    #[doc = "0x14 - Match High Register for CPUn"]
    pub matchn_h: MATCHN_H,
    _reserved0: [u8; 4usize],
    #[doc = "0x1c - OS_EVENT TIMER Control Register for CPUn"]
    pub osevent_ctrl: OSEVENT_CTRL,
}
#[doc = "EVTIMER Low Register"]
pub struct EVTIMERL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EVTIMER Low Register"]
pub mod evtimerl;
#[doc = "EVTIMER High Register"]
pub struct EVTIMERH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EVTIMER High Register"]
pub mod evtimerh;
#[doc = "Local Capture Low Register for CPUn"]
pub struct CAPTUREN_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Local Capture Low Register for CPUn"]
pub mod capturen_l;
#[doc = "Local Capture High Register for CPUn"]
pub struct CAPTUREN_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Local Capture High Register for CPUn"]
pub mod capturen_h;
#[doc = "Local Match Low Register for CPUn"]
pub struct MATCHN_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Local Match Low Register for CPUn"]
pub mod matchn_l;
#[doc = "Match High Register for CPUn"]
pub struct MATCHN_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Match High Register for CPUn"]
pub mod matchn_h;
#[doc = "OS_EVENT TIMER Control Register for CPUn"]
pub struct OSEVENT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OS_EVENT TIMER Control Register for CPUn"]
pub mod osevent_ctrl;
