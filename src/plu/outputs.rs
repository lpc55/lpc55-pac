#[doc = "Register `OUTPUTS` reader"]
pub struct R(crate::R<OUTPUTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTPUTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTPUTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTPUTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTPUTS` writer"]
pub struct W(crate::W<OUTPUTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTPUTS_SPEC>;
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
impl From<crate::W<OUTPUTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTPUTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTPUT_STATE` reader - Provides the current state of the 8 designated PLU Outputs.."]
pub struct OUTPUT_STATE_R(crate::FieldReader<u8, u8>);
impl OUTPUT_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OUTPUT_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTPUT_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Provides the current state of the 8 designated PLU Outputs.."]
    #[inline(always)]
    pub fn output_state(&self) -> OUTPUT_STATE_R {
        OUTPUT_STATE_R::new((self.bits & 0xff) as u8)
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
#[doc = "Provides the current state of the 8 designated PLU Outputs.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outputs](index.html) module"]
pub struct OUTPUTS_SPEC;
impl crate::RegisterSpec for OUTPUTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outputs::R](R) reader structure"]
impl crate::Readable for OUTPUTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outputs::W](W) writer structure"]
impl crate::Writable for OUTPUTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTPUTS to value 0"]
impl crate::Resettable for OUTPUTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
