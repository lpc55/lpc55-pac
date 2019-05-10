#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USART Configuration register. Basic USART configuration settings that typically are not changed during operation."]
    pub cfg: CFG,
    #[doc = "0x04 - USART Control register. USART control settings that are more likely to change during operation."]
    pub ctl: CTL,
    #[doc = "0x08 - USART Status register. The complete status value can be read here. Writing ones clears some bits in the register. Some bits can be cleared by writing a 1 to them."]
    pub stat: STAT,
    #[doc = "0x0c - Interrupt Enable read and Set register for USART (not FIFO) status. Contains individual interrupt enable bits for each potential USART interrupt. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
    pub intenset: INTENSET,
    #[doc = "0x10 - Interrupt Enable Clear register. Allows clearing any combination of bits in the INTENSET register. Writing a 1 to any implemented bit position causes the corresponding bit to be cleared."]
    pub intenclr: INTENCLR,
    _reserved0: [u8; 12usize],
    #[doc = "0x20 - Baud Rate Generator register. 16-bit integer baud rate divisor value."]
    pub brg: BRG,
    #[doc = "0x24 - Interrupt status register. Reflects interrupts that are currently enabled."]
    pub intstat: INTSTAT,
    #[doc = "0x28 - Oversample selection register for asynchronous communication."]
    pub osr: OSR,
    #[doc = "0x2c - Address register for automatic address matching."]
    pub addr: ADDR,
    _reserved1: [u8; 3536usize],
    #[doc = "0xe00 - FIFO configuration and enable register."]
    pub fifocfg: FIFOCFG,
    #[doc = "0xe04 - FIFO status register."]
    pub fifostat: FIFOSTAT,
    #[doc = "0xe08 - FIFO trigger settings for interrupt and DMA request."]
    pub fifotrig: FIFOTRIG,
    _reserved2: [u8; 4usize],
    #[doc = "0xe10 - FIFO interrupt enable set (enable) and read register."]
    pub fifointenset: FIFOINTENSET,
    #[doc = "0xe14 - FIFO interrupt enable clear (disable) and read register."]
    pub fifointenclr: FIFOINTENCLR,
    #[doc = "0xe18 - FIFO interrupt status register."]
    pub fifointstat: FIFOINTSTAT,
    _reserved3: [u8; 4usize],
    #[doc = "0xe20 - FIFO write data."]
    pub fifowr: FIFOWR,
    _reserved4: [u8; 12usize],
    #[doc = "0xe30 - FIFO read data."]
    pub fiford: FIFORD,
    _reserved5: [u8; 12usize],
    #[doc = "0xe40 - FIFO data read with no FIFO pop."]
    pub fifordnopop: FIFORDNOPOP,
    _reserved6: [u8; 440usize],
    #[doc = "0xffc - Peripheral identification register."]
    pub id: ID,
}
#[doc = "USART Configuration register. Basic USART configuration settings that typically are not changed during operation."]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART Configuration register. Basic USART configuration settings that typically are not changed during operation."]
pub mod cfg;
#[doc = "USART Control register. USART control settings that are more likely to change during operation."]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART Control register. USART control settings that are more likely to change during operation."]
pub mod ctl;
#[doc = "USART Status register. The complete status value can be read here. Writing ones clears some bits in the register. Some bits can be cleared by writing a 1 to them."]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART Status register. The complete status value can be read here. Writing ones clears some bits in the register. Some bits can be cleared by writing a 1 to them."]
pub mod stat;
#[doc = "Interrupt Enable read and Set register for USART (not FIFO) status. Contains individual interrupt enable bits for each potential USART interrupt. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable read and Set register for USART (not FIFO) status. Contains individual interrupt enable bits for each potential USART interrupt. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
pub mod intenset;
#[doc = "Interrupt Enable Clear register. Allows clearing any combination of bits in the INTENSET register. Writing a 1 to any implemented bit position causes the corresponding bit to be cleared."]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Clear register. Allows clearing any combination of bits in the INTENSET register. Writing a 1 to any implemented bit position causes the corresponding bit to be cleared."]
pub mod intenclr;
#[doc = "Baud Rate Generator register. 16-bit integer baud rate divisor value."]
pub struct BRG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Baud Rate Generator register. 16-bit integer baud rate divisor value."]
pub mod brg;
#[doc = "Interrupt status register. Reflects interrupts that are currently enabled."]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt status register. Reflects interrupts that are currently enabled."]
pub mod intstat;
#[doc = "Oversample selection register for asynchronous communication."]
pub struct OSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Oversample selection register for asynchronous communication."]
pub mod osr;
#[doc = "Address register for automatic address matching."]
pub struct ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address register for automatic address matching."]
pub mod addr;
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
