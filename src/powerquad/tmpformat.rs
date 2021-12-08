#[doc = "Register `TMPFORMAT` reader"]
pub struct R(crate::R<TMPFORMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMPFORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMPFORMAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMPFORMAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMPFORMAT` writer"]
pub struct W(crate::W<TMPFORMAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMPFORMAT_SPEC>;
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
impl From<crate::W<TMPFORMAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMPFORMAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tmp_formatint` reader - Temp Internal format (00: q15; 01:q31; 10:float)"]
pub struct TMP_FORMATINT_R(crate::FieldReader<u8, u8>);
impl TMP_FORMATINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TMP_FORMATINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMP_FORMATINT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmp_formatint` writer - Temp Internal format (00: q15; 01:q31; 10:float)"]
pub struct TMP_FORMATINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMP_FORMATINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `tmp_formatext` reader - Temp External format (00: q15; 01:q31; 10:float)"]
pub struct TMP_FORMATEXT_R(crate::FieldReader<u8, u8>);
impl TMP_FORMATEXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TMP_FORMATEXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMP_FORMATEXT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmp_formatext` writer - Temp External format (00: q15; 01:q31; 10:float)"]
pub struct TMP_FORMATEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMP_FORMATEXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `tmp_scaler` reader - Temp Scaler value (for scaled 'q31' formats)"]
pub struct TMP_SCALER_R(crate::FieldReader<u8, u8>);
impl TMP_SCALER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TMP_SCALER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMP_SCALER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmp_scaler` writer - Temp Scaler value (for scaled 'q31' formats)"]
pub struct TMP_SCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> TMP_SCALER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Temp Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn tmp_formatint(&self) -> TMP_FORMATINT_R {
        TMP_FORMATINT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Temp External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn tmp_formatext(&self) -> TMP_FORMATEXT_R {
        TMP_FORMATEXT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - Temp Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub fn tmp_scaler(&self) -> TMP_SCALER_R {
        TMP_SCALER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Temp Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn tmp_formatint(&mut self) -> TMP_FORMATINT_W {
        TMP_FORMATINT_W { w: self }
    }
    #[doc = "Bits 4:5 - Temp External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn tmp_formatext(&mut self) -> TMP_FORMATEXT_W {
        TMP_FORMATEXT_W { w: self }
    }
    #[doc = "Bits 8:15 - Temp Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub fn tmp_scaler(&mut self) -> TMP_SCALER_W {
        TMP_SCALER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Temp format\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmpformat](index.html) module"]
pub struct TMPFORMAT_SPEC;
impl crate::RegisterSpec for TMPFORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmpformat::R](R) reader structure"]
impl crate::Readable for TMPFORMAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmpformat::W](W) writer structure"]
impl crate::Writable for TMPFORMAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMPFORMAT to value 0"]
impl crate::Resettable for TMPFORMAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
