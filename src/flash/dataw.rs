#[doc = "Register `DATAW[%s]` reader"]
pub struct R(crate::R<DATAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATAW[%s]` writer"]
pub struct W(crate::W<DATAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAW_SPEC>;
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
impl From<crate::W<DATAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAW` reader - no description available"]
pub struct DATAW_R(crate::FieldReader<u32, u32>);
impl DATAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DATAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAW_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAW` writer - no description available"]
pub struct DATAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn dataw(&self) -> DATAW_R {
        DATAW_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn dataw(&mut self) -> DATAW_W {
        DATAW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "data register, word 0-7; Memory data, or command parameter, or command result.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dataw](index.html) module"]
pub struct DATAW_SPEC;
impl crate::RegisterSpec for DATAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dataw::R](R) reader structure"]
impl crate::Readable for DATAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dataw::W](W) writer structure"]
impl crate::Writable for DATAW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATAW[%s]
to value 0"]
impl crate::Resettable for DATAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
