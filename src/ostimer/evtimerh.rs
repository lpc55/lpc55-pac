#[doc = "Reader of register EVTIMERH"]
pub type R = crate::R<u32, super::EVTIMERH>;
#[doc = "Reader of field `EVTIMER_COUNT_VALUE`"]
pub type EVTIMER_COUNT_VALUE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - A read reflects the current value of the upper 32 bits of the EVTIMER. Note there is physically only one EVTimer, readable from all domains."]
    #[inline(always)]
    pub fn evtimer_count_value(&self) -> EVTIMER_COUNT_VALUE_R {
        EVTIMER_COUNT_VALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
