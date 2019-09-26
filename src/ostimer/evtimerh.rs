#[doc = "Reader of register EVTIMERH"]
pub type R = crate::R<u32, super::EVTIMERH>;
#[doc = "Writer for register EVTIMERH"]
pub type W = crate::W<u32, super::EVTIMERH>;
#[doc = "Register EVTIMERH `reset()`'s with value 0"]
impl crate::ResetValue for super::EVTIMERH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVTIMER_COUNT_VALUE`"]
pub type EVTIMER_COUNT_VALUE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - A read reflects the current value of the upper 32 bits of the EVTIMER. Note there is physically only one EVTimer, readable from all domains."]
    #[inline(always)]
    pub fn evtimer_count_value(&self) -> EVTIMER_COUNT_VALUE_R {
        EVTIMER_COUNT_VALUE_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {}
