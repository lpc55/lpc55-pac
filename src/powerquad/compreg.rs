#[doc = "Register `compreg[%s]` reader"]
pub struct R(crate::R<COMPREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMPREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMPREG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMPREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `compreg[%s]` writer"]
pub struct W(crate::W<COMPREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMPREG_SPEC>;
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
impl From<crate::W<COMPREG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMPREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `compreg` reader - Compute register bank"]
pub struct COMPREG_R(crate::FieldReader<u32, u32>);
impl COMPREG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        COMPREG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPREG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `compreg` writer - Compute register bank"]
pub struct COMPREG_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPREG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Compute register bank"]
    #[inline(always)]
    pub fn compreg(&self) -> COMPREG_R {
        COMPREG_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compute register bank"]
    #[inline(always)]
    pub fn compreg(&mut self) -> COMPREG_W {
        COMPREG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compute register bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compreg](index.html) module"]
pub struct COMPREG_SPEC;
impl crate::RegisterSpec for COMPREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [compreg::R](R) reader structure"]
impl crate::Readable for COMPREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [compreg::W](W) writer structure"]
impl crate::Writable for COMPREG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets compreg[%s]
to value 0"]
impl crate::Resettable for COMPREG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
