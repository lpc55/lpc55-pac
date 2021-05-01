#[doc = "Register `KEYSIZE` reader"]
pub struct R(crate::R<KEYSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<KEYSIZE_SPEC>> for R {
    fn from(reader: crate::R<KEYSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYSIZE` writer"]
pub struct W(crate::W<KEYSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYSIZE_SPEC>;
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
impl core::convert::From<crate::W<KEYSIZE_SPEC>> for W {
    fn from(writer: crate::W<KEYSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYSIZE` reader - Key size for Set Key operations"]
pub struct KEYSIZE_R(crate::FieldReader<u8, u8>);
impl KEYSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEYSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEYSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEYSIZE` writer - Key size for Set Key operations"]
pub struct KEYSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Key size for Set Key operations"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Key size for Set Key operations"]
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W {
        KEYSIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF Key Size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keysize](index.html) module"]
pub struct KEYSIZE_SPEC;
impl crate::RegisterSpec for KEYSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keysize::R](R) reader structure"]
impl crate::Readable for KEYSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keysize::W](W) writer structure"]
impl crate::Writable for KEYSIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYSIZE to value 0"]
impl crate::Resettable for KEYSIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
