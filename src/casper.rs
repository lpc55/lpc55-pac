#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Contains the offsets of AB and CD in the RAM."]
    pub ctrl0: CTRL0,
    #[doc = "0x04 - Contains the opcode mode, iteration count, and result offset (in RAM) and also launches the accelerator. Note: with CP version: CTRL0 and CRTL1 can be written in one go with MCRR."]
    pub ctrl1: CTRL1,
    #[doc = "0x08 - Contains an optional loader to load into CTRL0/1 in steps to perform a set of operations."]
    pub loader: LOADER,
    #[doc = "0x0c - Indicates operational status and would contain the carry bit if used."]
    pub status: STATUS,
    #[doc = "0x10 - Sets interrupts"]
    pub intenset: INTENSET,
    #[doc = "0x14 - Clears interrupts"]
    pub intenclr: INTENCLR,
    #[doc = "0x18 - Interrupt status bits (mask of INTENSET and STATUS)"]
    pub intstat: INTSTAT,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - A register"]
    pub areg: AREG,
    #[doc = "0x24 - B register"]
    pub breg: BREG,
    #[doc = "0x28 - C register"]
    pub creg: CREG,
    #[doc = "0x2c - D register"]
    pub dreg: DREG,
    #[doc = "0x30 - Result register 0"]
    pub res0: RES0,
    #[doc = "0x34 - Result register 1"]
    pub res1: RES1,
    #[doc = "0x38 - Result register 2"]
    pub res2: RES2,
    #[doc = "0x3c - Result register 3"]
    pub res3: RES3,
    _reserved1: [u8; 32usize],
    #[doc = "0x60 - Optional mask register"]
    pub mask: MASK,
    #[doc = "0x64 - Optional re-mask register"]
    pub remask: REMASK,
    _reserved2: [u8; 24usize],
    #[doc = "0x80 - Security lock register"]
    pub lock: LOCK,
}
#[doc = "Contains the offsets of AB and CD in the RAM."]
pub struct CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Contains the offsets of AB and CD in the RAM."]
pub mod ctrl0;
#[doc = "Contains the opcode mode, iteration count, and result offset (in RAM) and also launches the accelerator. Note: with CP version: CTRL0 and CRTL1 can be written in one go with MCRR."]
pub struct CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Contains the opcode mode, iteration count, and result offset (in RAM) and also launches the accelerator. Note: with CP version: CTRL0 and CRTL1 can be written in one go with MCRR."]
pub mod ctrl1;
#[doc = "Contains an optional loader to load into CTRL0/1 in steps to perform a set of operations."]
pub struct LOADER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Contains an optional loader to load into CTRL0/1 in steps to perform a set of operations."]
pub mod loader;
#[doc = "Indicates operational status and would contain the carry bit if used."]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Indicates operational status and would contain the carry bit if used."]
pub mod status;
#[doc = "Sets interrupts"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sets interrupts"]
pub mod intenset;
#[doc = "Clears interrupts"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clears interrupts"]
pub mod intenclr;
#[doc = "Interrupt status bits (mask of INTENSET and STATUS)"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt status bits (mask of INTENSET and STATUS)"]
pub mod intstat;
#[doc = "A register"]
pub struct AREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A register"]
pub mod areg;
#[doc = "B register"]
pub struct BREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "B register"]
pub mod breg;
#[doc = "C register"]
pub struct CREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "C register"]
pub mod creg;
#[doc = "D register"]
pub struct DREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "D register"]
pub mod dreg;
#[doc = "Result register 0"]
pub struct RES0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Result register 0"]
pub mod res0;
#[doc = "Result register 1"]
pub struct RES1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Result register 1"]
pub mod res1;
#[doc = "Result register 2"]
pub struct RES2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Result register 2"]
pub mod res2;
#[doc = "Result register 3"]
pub struct RES3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Result register 3"]
pub mod res3;
#[doc = "Optional mask register"]
pub struct MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Optional mask register"]
pub mod mask;
#[doc = "Optional re-mask register"]
pub struct REMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Optional re-mask register"]
pub mod remask;
#[doc = "Security lock register"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security lock register"]
pub mod lock;
