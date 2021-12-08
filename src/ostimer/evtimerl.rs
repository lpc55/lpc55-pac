#[doc = "Register `EVTIMERL` reader"]
pub struct R(crate::R<EVTIMERL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVTIMERL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVTIMERL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVTIMERL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EVTIMER_COUNT_VALUE` reader - A read reflects the current value of the lower 32 bits of the 42-bits EVTIMER. Note: There is only one EVTIMER, readable from all domains."]
pub struct EVTIMER_COUNT_VALUE_R(crate::FieldReader<u32, u32>);
impl EVTIMER_COUNT_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        EVTIMER_COUNT_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVTIMER_COUNT_VALUE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - A read reflects the current value of the lower 32 bits of the 42-bits EVTIMER. Note: There is only one EVTIMER, readable from all domains."]
    #[inline(always)]
    pub fn evtimer_count_value(&self) -> EVTIMER_COUNT_VALUE_R {
        EVTIMER_COUNT_VALUE_R::new(self.bits as u32)
    }
}
#[doc = "EVTIMER Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtimerl](index.html) module"]
pub struct EVTIMERL_SPEC;
impl crate::RegisterSpec for EVTIMERL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evtimerl::R](R) reader structure"]
impl crate::Readable for EVTIMERL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EVTIMERL to value 0"]
impl crate::Resettable for EVTIMERL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
