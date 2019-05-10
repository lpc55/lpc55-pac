#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4088usize],
    #[doc = "0xff8 - Peripheral Select and Flexcomm ID register."]
    pub pselid: PSELID,
    #[doc = "0xffc - Peripheral identification register."]
    pub pid: PID,
}
#[doc = "Peripheral Select and Flexcomm ID register."]
pub struct PSELID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Select and Flexcomm ID register."]
pub mod pselid;
#[doc = "Peripheral identification register."]
pub struct PID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral identification register."]
pub mod pid;
