#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - RTC match register"]
    pub match_: MATCH,
    #[doc = "0x08 - RTC counter register"]
    pub count: COUNT,
    #[doc = "0x0c - High-resolution/wake-up timer control register"]
    pub wake: WAKE,
    #[doc = "0x10 - RTC Sub-second Counter register"]
    pub subsec: SUBSEC,
    _reserved0: [u8; 44usize],
    #[doc = "0x40 - General Purpose register"]
    pub gpreg: [GPREG; 8],
}
#[doc = "RTC control register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC control register"]
pub mod ctrl;
#[doc = "RTC match register"]
pub struct MATCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC match register"]
pub mod match_;
#[doc = "RTC counter register"]
pub struct COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC counter register"]
pub mod count;
#[doc = "High-resolution/wake-up timer control register"]
pub struct WAKE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "High-resolution/wake-up timer control register"]
pub mod wake;
#[doc = "RTC Sub-second Counter register"]
pub struct SUBSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Sub-second Counter register"]
pub mod subsec;
#[doc = "General Purpose register"]
pub struct GPREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose register"]
pub mod gpreg;
