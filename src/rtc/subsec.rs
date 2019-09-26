#[doc = "Reader of register SUBSEC"]
pub type R = crate::R<u32, super::SUBSEC>;
#[doc = "Writer for register SUBSEC"]
pub type W = crate::W<u32, super::SUBSEC>;
#[doc = "Register SUBSEC `reset()`'s with value 0"]
impl crate::ResetValue for super::SUBSEC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SUBSEC`"]
pub type SUBSEC_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:14 - A read reflects the current value of the 32KHz sub-second counter. This counter is cleared whenever the SUBSEC_ENA bit in the RTC_CONTROL register is low. Up-counting at a 32KHz rate commences at the start of the next one-second interval after the SUBSEC_ENA bit is set. This counter must be re-enabled after exiting deep power-down mode or after the main RTC module is disabled and re-enabled. On modules not equipped with a sub-second counter, this register will read-back as all zeroes."]
    #[inline(always)]
    pub fn subsec(&self) -> SUBSEC_R {
        SUBSEC_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {}
