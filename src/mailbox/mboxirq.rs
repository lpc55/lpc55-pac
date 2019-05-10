#[doc = "Interrupt request register for the Cortex-M0+ CPU."]
pub struct IRQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt request register for the Cortex-M0+ CPU."]
pub mod irq;
#[doc = "Set bits in IRQ0"]
pub struct IRQSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set bits in IRQ0"]
pub mod irqset;
#[doc = "Clear bits in IRQ0"]
pub struct IRQCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear bits in IRQ0"]
pub mod irqclr;
