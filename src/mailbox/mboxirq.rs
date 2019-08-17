#[doc = "Interrupt request register for the Cortex-M0+ CPU.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [irq](irq) module"]
pub type IRQ = crate::Reg<u32, _IRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ;
#[doc = "`read()` method returns [irq::R](irq::R) reader structure"]
impl crate::Readable for IRQ {}
#[doc = "`write(|w| ..)` method takes [irq::W](irq::W) writer structure"]
impl crate::Writable for IRQ {}
#[doc = "Interrupt request register for the Cortex-M0+ CPU."]
pub mod irq;
#[doc = "Set bits in IRQ0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [irqset](irqset) module"]
pub type IRQSET = crate::Reg<u32, _IRQSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQSET;
#[doc = "`write(|w| ..)` method takes [irqset::W](irqset::W) writer structure"]
impl crate::Writable for IRQSET {}
#[doc = "Set bits in IRQ0"]
pub mod irqset;
#[doc = "Clear bits in IRQ0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [irqclr](irqclr) module"]
pub type IRQCLR = crate::Reg<u32, _IRQCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQCLR;
#[doc = "`write(|w| ..)` method takes [irqclr::W](irqclr::W) writer structure"]
impl crate::Writable for IRQCLR {}
#[doc = "Clear bits in IRQ0"]
pub mod irqclr;
