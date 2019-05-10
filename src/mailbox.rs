#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - no description available"]
    pub mboxirq0: MBOXIRQ,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - no description available"]
    pub mboxirq1: MBOXIRQ,
    _reserved1: [u8; 220usize],
    #[doc = "0xf8 - Mutual exclusion register\\[1\\]"]
    pub mutex: MUTEX,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct MBOXIRQ {
    #[doc = "0x00 - Interrupt request register for the Cortex-M0+ CPU."]
    pub irq: self::mboxirq::IRQ,
    #[doc = "0x04 - Set bits in IRQ0"]
    pub irqset: self::mboxirq::IRQSET,
    #[doc = "0x08 - Clear bits in IRQ0"]
    pub irqclr: self::mboxirq::IRQCLR,
}
#[doc = r" Register block"]
#[doc = "no description available"]
pub mod mboxirq;
#[doc = "Mutual exclusion register\\[1\\]"]
pub struct MUTEX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mutual exclusion register\\[1\\]"]
pub mod mutex;
