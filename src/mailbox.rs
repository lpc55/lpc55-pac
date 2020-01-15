#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - no description available"]
    pub mboxirq0: MBOXIRQ,
    _reserved1: [u8; 4usize],
    #[doc = "0x10 - no description available"]
    pub mboxirq1: MBOXIRQ,
    _reserved2: [u8; 220usize],
    #[doc = "0xf8 - Mutual exclusion register\\[1\\]"]
    pub mutex: MUTEX,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MBOXIRQ {
    #[doc = "0x00 - Interrupt request register for the Cortex-M0+ CPU."]
    pub irq: self::mboxirq::IRQ,
    #[doc = "0x04 - Set bits in IRQ0"]
    pub irqset: self::mboxirq::IRQSET,
    #[doc = "0x08 - Clear bits in IRQ0"]
    pub irqclr: self::mboxirq::IRQCLR,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod mboxirq;
#[doc = "Mutual exclusion register\\[1\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mutex](mutex) module"]
pub type MUTEX = crate::Reg<u32, _MUTEX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MUTEX;
#[doc = "`read()` method returns [mutex::R](mutex::R) reader structure"]
impl crate::Readable for MUTEX {}
#[doc = "`write(|w| ..)` method takes [mutex::W](mutex::W) writer structure"]
impl crate::Writable for MUTEX {}
#[doc = "Mutual exclusion register\\[1\\]"]
pub mod mutex;
