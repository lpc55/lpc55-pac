#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Is control register to enable and operate Hash and Crypto"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Indicates status of Hash peripheral."]
    pub status: STATUS,
    #[doc = "0x08 - Write 1 to enable interrupts; reads back with which are set."]
    pub intenset: INTENSET,
    #[doc = "0x0c - Write 1 to clear interrupts."]
    pub intenclr: INTENCLR,
    #[doc = "0x10 - Setup Master to access memory (if available)"]
    pub memctrl: MEMCTRL,
    #[doc = "0x14 - Address to start memory access from (if available)."]
    pub memaddr: MEMADDR,
    _reserved0: [u8; 8usize],
    #[doc = "0x20 - Input of 16 words at a time to load up buffer."]
    pub indata: INDATA,
    #[doc = "0x24 - no description available"]
    pub alias: [ALIAS; 7],
    #[doc = "0x40 - no description available"]
    pub outdata0: [OUTDATA0; 8],
    #[doc = "0x60 - no description available"]
    pub outdata1: [OUTDATA1; 8],
    #[doc = "0x80 - Crypto settings for AES and Salsa and ChaCha"]
    pub cryptcfg: CRYPTCFG,
    #[doc = "0x84 - Returns the configuration of this block in this chip - indicates what services are available."]
    pub config: CONFIG,
    _reserved1: [u8; 4usize],
    #[doc = "0x8c - Lock register allows locking to the current security level or unlocking by the lock holding level."]
    pub lock: LOCK,
    #[doc = "0x90 - no description available"]
    pub mask: [MASK; 4],
}
#[doc = "Is control register to enable and operate Hash and Crypto"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Is control register to enable and operate Hash and Crypto"]
pub mod ctrl;
#[doc = "Indicates status of Hash peripheral."]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Indicates status of Hash peripheral."]
pub mod status;
#[doc = "Write 1 to enable interrupts; reads back with which are set."]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write 1 to enable interrupts; reads back with which are set."]
pub mod intenset;
#[doc = "Write 1 to clear interrupts."]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write 1 to clear interrupts."]
pub mod intenclr;
#[doc = "Setup Master to access memory (if available)"]
pub struct MEMCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Setup Master to access memory (if available)"]
pub mod memctrl;
#[doc = "Address to start memory access from (if available)."]
pub struct MEMADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address to start memory access from (if available)."]
pub mod memaddr;
#[doc = "Input of 16 words at a time to load up buffer."]
pub struct INDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input of 16 words at a time to load up buffer."]
pub mod indata;
#[doc = "no description available"]
pub struct ALIAS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod alias;
#[doc = "no description available"]
pub struct OUTDATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod outdata0;
#[doc = "no description available"]
pub struct OUTDATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod outdata1;
#[doc = "Crypto settings for AES and Salsa and ChaCha"]
pub struct CRYPTCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Crypto settings for AES and Salsa and ChaCha"]
pub mod cryptcfg;
#[doc = "Returns the configuration of this block in this chip - indicates what services are available."]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Returns the configuration of this block in this chip - indicates what services are available."]
pub mod config;
#[doc = "Lock register allows locking to the current security level or unlocking by the lock holding level."]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lock register allows locking to the current security level or unlocking by the lock holding level."]
pub mod lock;
#[doc = "no description available"]
pub struct MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod mask;
