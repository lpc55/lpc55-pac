#[doc = "IRQ register accessor: an alias for `Reg<IRQ_SPEC>`"]
pub type IRQ = crate::Reg<irq::IRQ_SPEC>;
#[doc = "Interrupt request register for the Cortex-M0+ CPU."]
pub mod irq;
#[doc = "IRQSET register accessor: an alias for `Reg<IRQSET_SPEC>`"]
pub type IRQSET = crate::Reg<irqset::IRQSET_SPEC>;
#[doc = "Set bits in IRQ0"]
pub mod irqset;
#[doc = "IRQCLR register accessor: an alias for `Reg<IRQCLR_SPEC>`"]
pub type IRQCLR = crate::Reg<irqclr::IRQCLR_SPEC>;
#[doc = "Clear bits in IRQ0"]
pub mod irqclr;
