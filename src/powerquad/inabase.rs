#[doc = "Register `INABASE` reader"]
pub struct R(crate::R<INABASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INABASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INABASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INABASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INABASE` writer"]
pub struct W(crate::W<INABASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INABASE_SPEC>;
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
impl From<crate::W<INABASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INABASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `inabase` reader - Base address register for the input A region"]
pub struct INABASE_R(crate::FieldReader<u32, u32>);
impl INABASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INABASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INABASE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inabase` writer - Base address register for the input A region"]
pub struct INABASE_W<'a> {
    w: &'a mut W,
}
impl<'a> INABASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Base address register for the input A region"]
    #[inline(always)]
    pub fn inabase(&self) -> INABASE_R {
        INABASE_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address register for the input A region"]
    #[inline(always)]
    pub fn inabase(&mut self) -> INABASE_W {
        INABASE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Base address register for input A region\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inabase](index.html) module"]
pub struct INABASE_SPEC;
impl crate::RegisterSpec for INABASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inabase::R](R) reader structure"]
impl crate::Readable for INABASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inabase::W](W) writer structure"]
impl crate::Writable for INABASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INABASE to value 0"]
impl crate::Resettable for INABASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
