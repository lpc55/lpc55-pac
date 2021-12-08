#[doc = "Register `SUBSEC` reader"]
pub struct R(crate::R<SUBSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUBSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUBSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUBSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUBSEC` writer"]
pub struct W(crate::W<SUBSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUBSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SUBSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUBSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBSEC` reader - A read reflects the current value of the 32KHz sub-second counter. This counter is cleared whenever the SUBSEC_ENA bit in the RTC_CONTROL register is low. Up-counting at a 32KHz rate commences at the start of the next one-second interval after the SUBSEC_ENA bit is set. This counter must be re-enabled after exiting deep power-down mode or after the main RTC module is disabled and re-enabled. On modules not equipped with a sub-second counter, this register will read-back as all zeroes."]
pub struct SUBSEC_R(crate::FieldReader<u16, u16>);
impl SUBSEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SUBSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUBSEC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:14 - A read reflects the current value of the 32KHz sub-second counter. This counter is cleared whenever the SUBSEC_ENA bit in the RTC_CONTROL register is low. Up-counting at a 32KHz rate commences at the start of the next one-second interval after the SUBSEC_ENA bit is set. This counter must be re-enabled after exiting deep power-down mode or after the main RTC module is disabled and re-enabled. On modules not equipped with a sub-second counter, this register will read-back as all zeroes."]
    #[inline(always)]
    pub fn subsec(&self) -> SUBSEC_R {
        SUBSEC_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sub-second counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subsec](index.html) module"]
pub struct SUBSEC_SPEC;
impl crate::RegisterSpec for SUBSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [subsec::R](R) reader structure"]
impl crate::Readable for SUBSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [subsec::W](W) writer structure"]
impl crate::Writable for SUBSEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUBSEC to value 0"]
impl crate::Resettable for SUBSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
