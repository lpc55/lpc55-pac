#[doc = "Register `PRINCE_REGION1_IV_HEADER1` reader"]
pub struct R(crate::R<PRINCE_REGION1_IV_HEADER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRINCE_REGION1_IV_HEADER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PRINCE_REGION1_IV_HEADER1_SPEC>> for R {
    fn from(reader: crate::R<PRINCE_REGION1_IV_HEADER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRINCE_REGION1_IV_HEADER1` writer"]
pub struct W(crate::W<PRINCE_REGION1_IV_HEADER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRINCE_REGION1_IV_HEADER1_SPEC>;
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
impl core::convert::From<crate::W<PRINCE_REGION1_IV_HEADER1_SPEC>> for W {
    fn from(writer: crate::W<PRINCE_REGION1_IV_HEADER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TYPE` reader - ."]
pub struct TYPE_R(crate::FieldReader<u8, u8>);
impl TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPE` writer - ."]
pub struct TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `INDEX` reader - ."]
pub struct INDEX_R(crate::FieldReader<u8, u8>);
impl INDEX_R {
    pub(crate) fn new(bits: u8) -> Self {
        INDEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INDEX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INDEX` writer - ."]
pub struct INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> INDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `SIZE` reader - ."]
pub struct SIZE_R(crate::FieldReader<u8, u8>);
impl SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIZE` writer - ."]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ."]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - ."]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - ."]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ."]
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W {
        TYPE_W { w: self }
    }
    #[doc = "Bits 8:11 - ."]
    #[inline(always)]
    pub fn index(&mut self) -> INDEX_W {
        INDEX_W { w: self }
    }
    #[doc = "Bits 24:29 - ."]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_header1](index.html) module"]
pub struct PRINCE_REGION1_IV_HEADER1_SPEC;
impl crate::RegisterSpec for PRINCE_REGION1_IV_HEADER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prince_region1_iv_header1::R](R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_HEADER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_header1::W](W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_HEADER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRINCE_REGION1_IV_HEADER1 to value 0"]
impl crate::Resettable for PRINCE_REGION1_IV_HEADER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
