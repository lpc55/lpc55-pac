#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x0c - no description available"]
    pub mboxirq0: MBOXIRQ,
    _reserved1: [u8; 0x04],
    #[doc = "0x10..0x1c - no description available"]
    pub mboxirq1: MBOXIRQ,
    _reserved2: [u8; 0xdc],
    #[doc = "0xf8 - Mutual exclusion register\\[1\\]"]
    pub mutex: crate::Reg<mutex::MUTEX_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MBOXIRQ {
    #[doc = "0x00 - Interrupt request register for the Cortex-M0+ CPU."]
    pub irq: crate::Reg<self::mboxirq::irq::IRQ_SPEC>,
    #[doc = "0x04 - Set bits in IRQ0"]
    pub irqset: crate::Reg<self::mboxirq::irqset::IRQSET_SPEC>,
    #[doc = "0x08 - Clear bits in IRQ0"]
    pub irqclr: crate::Reg<self::mboxirq::irqclr::IRQCLR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod mboxirq;
#[doc = "MUTEX register accessor: an alias for `Reg<MUTEX_SPEC>`"]
pub type MUTEX = crate::Reg<mutex::MUTEX_SPEC>;
#[doc = "Mutual exclusion register\\[1\\]"]
pub mod mutex;
