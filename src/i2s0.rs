#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - no description available"]
    pub secchannel0: SECCHANNEL,
    _reserved0: [u8; 468usize],
    #[doc = "0xe00 - FIFO configuration and enable register."]
    pub fifocfg: FIFOCFG,
    #[doc = "0xe04 - FIFO status register."]
    pub fifostat: FIFOSTAT,
    #[doc = "0xe08 - FIFO trigger settings for interrupt and DMA request."]
    pub fifotrig: FIFOTRIG,
    _reserved1: [u8; 4usize],
    #[doc = "0xe10 - FIFO interrupt enable set (enable) and read register."]
    pub fifointenset: FIFOINTENSET,
    #[doc = "0xe14 - FIFO interrupt enable clear (disable) and read register."]
    pub fifointenclr: FIFOINTENCLR,
    #[doc = "0xe18 - FIFO interrupt status register."]
    pub fifointstat: FIFOINTSTAT,
    _reserved2: [u8; 4usize],
    #[doc = "0xe20 - FIFO write data."]
    pub fifowr: FIFOWR,
    #[doc = "0xe24 - FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    pub fifowr48h: FIFOWR48H,
    _reserved3: [u8; 8usize],
    #[doc = "0xe30 - FIFO read data."]
    pub fiford: FIFORD,
    #[doc = "0xe34 - FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    pub fiford48h: FIFORD48H,
    _reserved4: [u8; 8usize],
    #[doc = "0xe40 - FIFO data read with no FIFO pop."]
    pub fifordnopop: FIFORDNOPOP,
    #[doc = "0xe44 - FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    pub fiford48hnopop: FIFORD48HNOPOP,
    _reserved5: [u8; 436usize],
    #[doc = "0xffc - I2S Module identification"]
    pub id: ID,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct SECCHANNEL {
    _reserved0: [u8; 3104usize],
    #[doc = "0xc20 - Configuration register 1 for channel pair"]
    pub pcfg1: self::secchannel::PCFG1,
    #[doc = "0xc24 - Configuration register 2 for channel pair"]
    pub pcfg2: self::secchannel::PCFG2,
    #[doc = "0xc28 - Status register for channel pair"]
    pub pstat: self::secchannel::PSTAT,
}
#[doc = r" Register block"]
#[doc = "no description available"]
pub mod secchannel;
#[doc = "Configuration register 1 for the primary channel pair."]
pub struct CFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register 1 for the primary channel pair."]
pub mod cfg1;
#[doc = "Configuration register 2 for the primary channel pair."]
pub struct CFG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register 2 for the primary channel pair."]
pub mod cfg2;
#[doc = "Status register for the primary channel pair."]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register for the primary channel pair."]
pub mod stat;
#[doc = "Clock divider, used by all channel pairs."]
pub struct DIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock divider, used by all channel pairs."]
pub mod div;
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
#[doc = "FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub struct FIFOWR48H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub mod fifowr48h;
#[doc = "FIFO read data."]
pub struct FIFORD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO read data."]
pub mod fiford;
#[doc = "FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub struct FIFORD48H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub mod fiford48h;
#[doc = "FIFO data read with no FIFO pop."]
pub struct FIFORDNOPOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO data read with no FIFO pop."]
pub mod fifordnopop;
#[doc = "FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub struct FIFORD48HNOPOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub mod fiford48hnopop;
#[doc = "I2S Module identification"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S Module identification"]
pub mod id;
