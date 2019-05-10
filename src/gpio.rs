#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - no description available"]
    pub b: [B; 4],
    _reserved0: [u8; 3968usize],
    #[doc = "0x1000 - no description available"]
    pub w: [W; 4],
    _reserved1: [u8; 3584usize],
    #[doc = "0x2000 - Direction registers for all port GPIO pins"]
    pub dir: [DIR; 4],
    _reserved2: [u8; 112usize],
    #[doc = "0x2080 - Mask register for all port GPIO pins"]
    pub mask: [MASK; 4],
    _reserved3: [u8; 112usize],
    #[doc = "0x2100 - Port pin register for all port GPIO pins"]
    pub pin: [PIN; 4],
    _reserved4: [u8; 112usize],
    #[doc = "0x2180 - Masked port register for all port GPIO pins"]
    pub mpin: [MPIN; 4],
    _reserved5: [u8; 112usize],
    #[doc = "0x2200 - Write: Set register for port. Read: output bits for port"]
    pub set: [SET; 4],
    _reserved6: [u8; 112usize],
    #[doc = "0x2280 - Clear port for all port GPIO pins"]
    pub clr: [CLR; 4],
    _reserved7: [u8; 112usize],
    #[doc = "0x2300 - Toggle port for all port GPIO pins"]
    pub not: [NOT; 4],
    _reserved8: [u8; 112usize],
    #[doc = "0x2380 - Set pin direction bits for port"]
    pub dirset: [DIRSET; 4],
    _reserved9: [u8; 112usize],
    #[doc = "0x2400 - Clear pin direction bits for port"]
    pub dirclr: [DIRCLR; 4],
    _reserved10: [u8; 112usize],
    #[doc = "0x2480 - Toggle pin direction bits for port"]
    pub dirnot: [DIRNOT; 4],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct B {
    #[doc = "0x00 - Byte pin registers for all port GPIO pins"]
    pub b_: [self::b::B_; 32],
}
#[doc = r" Register block"]
#[doc = "no description available"]
pub mod b;
#[doc = r" Register block"]
#[repr(C)]
pub struct W {
    #[doc = "0x00 - Word pin registers for all port GPIO pins"]
    pub w_: [self::w::W_; 32],
}
#[doc = r" Register block"]
#[doc = "no description available"]
pub mod w;
#[doc = "Direction registers for all port GPIO pins"]
pub struct DIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direction registers for all port GPIO pins"]
pub mod dir;
#[doc = "Mask register for all port GPIO pins"]
pub struct MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask register for all port GPIO pins"]
pub mod mask;
#[doc = "Port pin register for all port GPIO pins"]
pub struct PIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port pin register for all port GPIO pins"]
pub mod pin;
#[doc = "Masked port register for all port GPIO pins"]
pub struct MPIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Masked port register for all port GPIO pins"]
pub mod mpin;
#[doc = "Write: Set register for port. Read: output bits for port"]
pub struct SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write: Set register for port. Read: output bits for port"]
pub mod set;
#[doc = "Clear port for all port GPIO pins"]
pub struct CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear port for all port GPIO pins"]
pub mod clr;
#[doc = "Toggle port for all port GPIO pins"]
pub struct NOT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Toggle port for all port GPIO pins"]
pub mod not;
#[doc = "Set pin direction bits for port"]
pub struct DIRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set pin direction bits for port"]
pub mod dirset;
#[doc = "Clear pin direction bits for port"]
pub struct DIRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear pin direction bits for port"]
pub mod dirclr;
#[doc = "Toggle pin direction bits for port"]
pub struct DIRNOT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Toggle pin direction bits for port"]
pub mod dirnot;
