#[doc = "Register `CPPRE` reader"]
pub struct R(crate::R<CPPRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPPRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPPRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPPRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPPRE` writer"]
pub struct W(crate::W<CPPRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPPRE_SPEC>;
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
impl From<crate::W<CPPRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPPRE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cppre_in` reader - co-processor scaling of input"]
pub struct CPPRE_IN_R(crate::FieldReader<u8, u8>);
impl CPPRE_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPPRE_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPPRE_IN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cppre_in` writer - co-processor scaling of input"]
pub struct CPPRE_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPPRE_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `cppre_out` reader - co-processor fixed point output"]
pub struct CPPRE_OUT_R(crate::FieldReader<u8, u8>);
impl CPPRE_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPPRE_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPPRE_OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cppre_out` writer - co-processor fixed point output"]
pub struct CPPRE_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CPPRE_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `cppre_sat` reader - 1 : forces sub-32 bit saturation"]
pub struct CPPRE_SAT_R(crate::FieldReader<bool, bool>);
impl CPPRE_SAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPPRE_SAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPPRE_SAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cppre_sat` writer - 1 : forces sub-32 bit saturation"]
pub struct CPPRE_SAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CPPRE_SAT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `cppre_sat8` reader - 0 = 8bits, 1 = 16bits"]
pub struct CPPRE_SAT8_R(crate::FieldReader<bool, bool>);
impl CPPRE_SAT8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPPRE_SAT8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPPRE_SAT8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cppre_sat8` writer - 0 = 8bits, 1 = 16bits"]
pub struct CPPRE_SAT8_W<'a> {
    w: &'a mut W,
}
impl<'a> CPPRE_SAT8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - co-processor scaling of input"]
    #[inline(always)]
    pub fn cppre_in(&self) -> CPPRE_IN_R {
        CPPRE_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - co-processor fixed point output"]
    #[inline(always)]
    pub fn cppre_out(&self) -> CPPRE_OUT_R {
        CPPRE_OUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 1 : forces sub-32 bit saturation"]
    #[inline(always)]
    pub fn cppre_sat(&self) -> CPPRE_SAT_R {
        CPPRE_SAT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 0 = 8bits, 1 = 16bits"]
    #[inline(always)]
    pub fn cppre_sat8(&self) -> CPPRE_SAT8_R {
        CPPRE_SAT8_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - co-processor scaling of input"]
    #[inline(always)]
    pub fn cppre_in(&mut self) -> CPPRE_IN_W {
        CPPRE_IN_W { w: self }
    }
    #[doc = "Bits 8:15 - co-processor fixed point output"]
    #[inline(always)]
    pub fn cppre_out(&mut self) -> CPPRE_OUT_W {
        CPPRE_OUT_W { w: self }
    }
    #[doc = "Bit 16 - 1 : forces sub-32 bit saturation"]
    #[inline(always)]
    pub fn cppre_sat(&mut self) -> CPPRE_SAT_W {
        CPPRE_SAT_W { w: self }
    }
    #[doc = "Bit 17 - 0 = 8bits, 1 = 16bits"]
    #[inline(always)]
    pub fn cppre_sat8(&mut self) -> CPPRE_SAT8_W {
        CPPRE_SAT8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pre-scale register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cppre](index.html) module"]
pub struct CPPRE_SPEC;
impl crate::RegisterSpec for CPPRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cppre::R](R) reader structure"]
impl crate::Readable for CPPRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cppre::W](W) writer structure"]
impl crate::Writable for CPPRE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPPRE to value 0"]
impl crate::Resettable for CPPRE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
