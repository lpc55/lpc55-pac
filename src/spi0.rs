#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1024usize],
    #[doc = "0x400 - SPI Configuration register"]
    pub cfg: CFG,
    #[doc = "0x404 - SPI Delay register"]
    pub dly: DLY,
    #[doc = "0x408 - SPI Status. Some status flags can be cleared by writing a 1 to that bit position."]
    pub stat: STAT,
    #[doc = "0x40c - SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
    pub intenset: INTENSET,
    #[doc = "0x410 - SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared."]
    pub intenclr: INTENCLR,
    _reserved1: [u8; 16usize],
    #[doc = "0x424 - SPI clock Divider"]
    pub div: DIV,
    #[doc = "0x428 - SPI Interrupt Status"]
    pub intstat: INTSTAT,
    _reserved2: [u8; 2516usize],
    #[doc = "0xe00 - FIFO configuration and enable register."]
    pub fifocfg: FIFOCFG,
    #[doc = "0xe04 - FIFO status register."]
    pub fifostat: FIFOSTAT,
    #[doc = "0xe08 - FIFO trigger settings for interrupt and DMA request."]
    pub fifotrig: FIFOTRIG,
    _reserved3: [u8; 4usize],
    #[doc = "0xe10 - FIFO interrupt enable set (enable) and read register."]
    pub fifointenset: FIFOINTENSET,
    #[doc = "0xe14 - FIFO interrupt enable clear (disable) and read register."]
    pub fifointenclr: FIFOINTENCLR,
    #[doc = "0xe18 - FIFO interrupt status register."]
    pub fifointstat: FIFOINTSTAT,
    _reserved4: [u8; 4usize],
    #[doc = "0xe20 - FIFO write data."]
    pub fifowr: FIFOWR,
    _reserved5: [u8; 12usize],
    #[doc = "0xe30 - FIFO read data."]
    pub fiford: FIFORD,
    _reserved6: [u8; 12usize],
    #[doc = "0xe40 - FIFO data read with no FIFO pop."]
    pub fifordnopop: FIFORDNOPOP,
    _reserved7: [u8; 440usize],
    #[doc = "0xffc - Peripheral identification register."]
    pub id: ID,
}
#[doc = "SPI Configuration register"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Configuration register"]
pub mod cfg;
#[doc = "SPI Delay register"]
pub struct DLY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Delay register"]
pub mod dly;
#[doc = "SPI Status. Some status flags can be cleared by writing a 1 to that bit position."]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Status. Some status flags can be cleared by writing a 1 to that bit position."]
pub mod stat;
#[doc = "SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
pub mod intenset;
#[doc = "SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared."]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared."]
pub mod intenclr;
#[doc = "SPI clock Divider"]
pub struct DIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI clock Divider"]
pub mod div;
#[doc = "SPI Interrupt Status"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Interrupt Status"]
pub mod intstat;
#[doc = "FIFO configuration and enable register."]
pub struct FIFOCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO configuration and enable register."]
pub mod fifocfg;
#[doc = "FIFO status register."]
pub struct FIFOSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO status register."]
pub mod fifostat;
#[doc = "FIFO trigger settings for interrupt and DMA request."]
pub struct FIFOTRIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO trigger settings for interrupt and DMA request."]
pub mod fifotrig;
#[doc = "FIFO interrupt enable set (enable) and read register."]
pub struct FIFOINTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO interrupt enable set (enable) and read register."]
pub mod fifointenset;
#[doc = "FIFO interrupt enable clear (disable) and read register."]
pub struct FIFOINTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO interrupt enable clear (disable) and read register."]
pub mod fifointenclr;
#[doc = "FIFO interrupt status register."]
pub struct FIFOINTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO interrupt status register."]
pub mod fifointstat;
#[doc = "FIFO write data."]
pub struct FIFOWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO write data."]
pub mod fifowr;
#[doc = "FIFO read data."]
pub struct FIFORD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO read data."]
pub mod fiford;
#[doc = "FIFO data read with no FIFO pop."]
pub struct FIFORDNOPOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO data read with no FIFO pop."]
pub mod fifordnopop;
#[doc = "Peripheral identification register."]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral identification register."]
pub mod id;
