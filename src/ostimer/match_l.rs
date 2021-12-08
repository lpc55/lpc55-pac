#[doc = "Register `MATCH_L` reader"]
pub struct R(crate::R<MATCH_L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MATCH_L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MATCH_L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MATCH_L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATCH_L` writer"]
pub struct W(crate::W<MATCH_L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MATCH_L_SPEC>;
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
impl From<crate::W<MATCH_L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MATCH_L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MATCH_VALUE` reader - The value written to the MATCH (L/H) register pair is compared against the central EVTIMER. When a match occurs, an interrupt request is generated if enabled."]
pub struct MATCH_VALUE_R(crate::FieldReader<u32, u32>);
impl MATCH_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MATCH_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MATCH_VALUE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MATCH_VALUE` writer - The value written to the MATCH (L/H) register pair is compared against the central EVTIMER. When a match occurs, an interrupt request is generated if enabled."]
pub struct MATCH_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCH_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The value written to the MATCH (L/H) register pair is compared against the central EVTIMER. When a match occurs, an interrupt request is generated if enabled."]
    #[inline(always)]
    pub fn match_value(&self) -> MATCH_VALUE_R {
        MATCH_VALUE_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The value written to the MATCH (L/H) register pair is compared against the central EVTIMER. When a match occurs, an interrupt request is generated if enabled."]
    #[inline(always)]
    pub fn match_value(&mut self) -> MATCH_VALUE_W {
        MATCH_VALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Match Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match_l](index.html) module"]
pub struct MATCH_L_SPEC;
impl crate::RegisterSpec for MATCH_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [match_l::R](R) reader structure"]
impl crate::Readable for MATCH_L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [match_l::W](W) writer structure"]
impl crate::Writable for MATCH_L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MATCH_L to value 0"]
impl crate::Resettable for MATCH_L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
