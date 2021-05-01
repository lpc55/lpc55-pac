#[doc = "Register `OUTFORMAT` reader"]
pub struct R(crate::R<OUTFORMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTFORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OUTFORMAT_SPEC>> for R {
    fn from(reader: crate::R<OUTFORMAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTFORMAT` writer"]
pub struct W(crate::W<OUTFORMAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTFORMAT_SPEC>;
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
impl core::convert::From<crate::W<OUTFORMAT_SPEC>> for W {
    fn from(writer: crate::W<OUTFORMAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `out_formatint` reader - Output Internal format (00: q15; 01:q31; 10:float)"]
pub struct OUT_FORMATINT_R(crate::FieldReader<u8, u8>);
impl OUT_FORMATINT_R {
    pub(crate) fn new(bits: u8) -> Self {
        OUT_FORMATINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_FORMATINT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `out_formatint` writer - Output Internal format (00: q15; 01:q31; 10:float)"]
pub struct OUT_FORMATINT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_FORMATINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `out_formatext` reader - Output External format (00: q15; 01:q31; 10:float)"]
pub struct OUT_FORMATEXT_R(crate::FieldReader<u8, u8>);
impl OUT_FORMATEXT_R {
    pub(crate) fn new(bits: u8) -> Self {
        OUT_FORMATEXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_FORMATEXT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `out_formatext` writer - Output External format (00: q15; 01:q31; 10:float)"]
pub struct OUT_FORMATEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_FORMATEXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `out_scaler` reader - Output Scaler value (for scaled 'q31' formats)"]
pub struct OUT_SCALER_R(crate::FieldReader<u8, u8>);
impl OUT_SCALER_R {
    pub(crate) fn new(bits: u8) -> Self {
        OUT_SCALER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_SCALER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `out_scaler` writer - Output Scaler value (for scaled 'q31' formats)"]
pub struct OUT_SCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_SCALER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Output Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn out_formatint(&self) -> OUT_FORMATINT_R {
        OUT_FORMATINT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Output External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn out_formatext(&self) -> OUT_FORMATEXT_R {
        OUT_FORMATEXT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - Output Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub fn out_scaler(&self) -> OUT_SCALER_R {
        OUT_SCALER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Output Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn out_formatint(&mut self) -> OUT_FORMATINT_W {
        OUT_FORMATINT_W { w: self }
    }
    #[doc = "Bits 4:5 - Output External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn out_formatext(&mut self) -> OUT_FORMATEXT_W {
        OUT_FORMATEXT_W { w: self }
    }
    #[doc = "Bits 8:15 - Output Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub fn out_scaler(&mut self) -> OUT_SCALER_W {
        OUT_SCALER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output format\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outformat](index.html) module"]
pub struct OUTFORMAT_SPEC;
impl crate::RegisterSpec for OUTFORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outformat::R](R) reader structure"]
impl crate::Readable for OUTFORMAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outformat::W](W) writer structure"]
impl crate::Writable for OUTFORMAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTFORMAT to value 0"]
impl crate::Resettable for OUTFORMAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
