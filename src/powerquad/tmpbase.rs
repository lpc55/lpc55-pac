#[doc = "Register `TMPBASE` reader"]
pub struct R(crate::R<TMPBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMPBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMPBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMPBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMPBASE` writer"]
pub struct W(crate::W<TMPBASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMPBASE_SPEC>;
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
impl From<crate::W<TMPBASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMPBASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tmpbase` reader - Base address register for the temporary region"]
pub struct TMPBASE_R(crate::FieldReader<u32, u32>);
impl TMPBASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TMPBASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMPBASE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmpbase` writer - Base address register for the temporary region"]
pub struct TMPBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMPBASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Base address register for the temporary region"]
    #[inline(always)]
    pub fn tmpbase(&self) -> TMPBASE_R {
        TMPBASE_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address register for the temporary region"]
    #[inline(always)]
    pub fn tmpbase(&mut self) -> TMPBASE_W {
        TMPBASE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Base address register for temp region\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmpbase](index.html) module"]
pub struct TMPBASE_SPEC;
impl crate::RegisterSpec for TMPBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmpbase::R](R) reader structure"]
impl crate::Readable for TMPBASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmpbase::W](W) writer structure"]
impl crate::Writable for TMPBASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMPBASE to value 0"]
impl crate::Resettable for TMPBASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
