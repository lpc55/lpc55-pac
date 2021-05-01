#[doc = "Register `INBFORMAT` reader"]
pub struct R(crate::R<INBFORMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INBFORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INBFORMAT_SPEC>> for R {
    fn from(reader: crate::R<INBFORMAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INBFORMAT` writer"]
pub struct W(crate::W<INBFORMAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INBFORMAT_SPEC>;
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
impl core::convert::From<crate::W<INBFORMAT_SPEC>> for W {
    fn from(writer: crate::W<INBFORMAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `inb_formatint` reader - Input B Internal format (00: q15; 01:q31; 10:float)"]
pub struct INB_FORMATINT_R(crate::FieldReader<u8, u8>);
impl INB_FORMATINT_R {
    pub(crate) fn new(bits: u8) -> Self {
        INB_FORMATINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INB_FORMATINT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inb_formatint` writer - Input B Internal format (00: q15; 01:q31; 10:float)"]
pub struct INB_FORMATINT_W<'a> {
    w: &'a mut W,
}
impl<'a> INB_FORMATINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `inb_formatext` reader - Input B External format (00: q15; 01:q31; 10:float)"]
pub struct INB_FORMATEXT_R(crate::FieldReader<u8, u8>);
impl INB_FORMATEXT_R {
    pub(crate) fn new(bits: u8) -> Self {
        INB_FORMATEXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INB_FORMATEXT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inb_formatext` writer - Input B External format (00: q15; 01:q31; 10:float)"]
pub struct INB_FORMATEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> INB_FORMATEXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `inb_scaler` reader - Input B Scaler value (for scaled 'q31' formats)"]
pub struct INB_SCALER_R(crate::FieldReader<u8, u8>);
impl INB_SCALER_R {
    pub(crate) fn new(bits: u8) -> Self {
        INB_SCALER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INB_SCALER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inb_scaler` writer - Input B Scaler value (for scaled 'q31' formats)"]
pub struct INB_SCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> INB_SCALER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Input B Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn inb_formatint(&self) -> INB_FORMATINT_R {
        INB_FORMATINT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Input B External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn inb_formatext(&self) -> INB_FORMATEXT_R {
        INB_FORMATEXT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - Input B Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub fn inb_scaler(&self) -> INB_SCALER_R {
        INB_SCALER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input B Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn inb_formatint(&mut self) -> INB_FORMATINT_W {
        INB_FORMATINT_W { w: self }
    }
    #[doc = "Bits 4:5 - Input B External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn inb_formatext(&mut self) -> INB_FORMATEXT_W {
        INB_FORMATEXT_W { w: self }
    }
    #[doc = "Bits 8:15 - Input B Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub fn inb_scaler(&mut self) -> INB_SCALER_W {
        INB_SCALER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input B format\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inbformat](index.html) module"]
pub struct INBFORMAT_SPEC;
impl crate::RegisterSpec for INBFORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inbformat::R](R) reader structure"]
impl crate::Readable for INBFORMAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inbformat::W](W) writer structure"]
impl crate::Writable for INBFORMAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INBFORMAT to value 0"]
impl crate::Resettable for INBFORMAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
