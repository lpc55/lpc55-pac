#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status register."]
    pub stat: STAT,
    #[doc = "0x08 - Capture configuration register."]
    pub cfg: CFG,
    #[doc = "0x0c - Capture clear register."]
    pub capclr: CAPCLR,
    #[doc = "0x10 - Capture register ."]
    pub cap: [CAP; 4],
}
#[doc = "Control register."]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register."]
pub mod ctrl;
#[doc = "Status register."]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register."]
pub mod stat;
#[doc = "Capture configuration register."]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture configuration register."]
pub mod cfg;
#[doc = "Capture clear register."]
pub struct CAPCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture clear register."]
pub mod capclr;
#[doc = "Capture register ."]
pub struct CAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture register ."]
pub mod cap;
