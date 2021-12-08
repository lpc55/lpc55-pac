#[doc = "Register `INAFORMAT` reader"]
pub struct R(crate::R<INAFORMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INAFORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INAFORMAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INAFORMAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INAFORMAT` writer"]
pub struct W(crate::W<INAFORMAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INAFORMAT_SPEC>;
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
impl From<crate::W<INAFORMAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INAFORMAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ina_formatint` reader - Input A Internal format (00: q15; 01:q31; 10:float)"]
pub struct INA_FORMATINT_R(crate::FieldReader<u8, u8>);
impl INA_FORMATINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INA_FORMATINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INA_FORMATINT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ina_formatint` writer - Input A Internal format (00: q15; 01:q31; 10:float)"]
pub struct INA_FORMATINT_W<'a> {
    w: &'a mut W,
}
impl<'a> INA_FORMATINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `ina_formatext` reader - Input A External format (00: q15; 01:q31; 10:float)"]
pub struct INA_FORMATEXT_R(crate::FieldReader<u8, u8>);
impl INA_FORMATEXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INA_FORMATEXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INA_FORMATEXT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ina_formatext` writer - Input A External format (00: q15; 01:q31; 10:float)"]
pub struct INA_FORMATEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> INA_FORMATEXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `ina_scaler` reader - Input A Scaler value (for scaled 'q31' formats)"]
pub struct INA_SCALER_R(crate::FieldReader<u8, u8>);
impl INA_SCALER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INA_SCALER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INA_SCALER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ina_scaler` writer - Input A Scaler value (for scaled 'q31' formats)"]
pub struct INA_SCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> INA_SCALER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Input A Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn ina_formatint(&self) -> INA_FORMATINT_R {
        INA_FORMATINT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Input A External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn ina_formatext(&self) -> INA_FORMATEXT_R {
        INA_FORMATEXT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - Input A Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub fn ina_scaler(&self) -> INA_SCALER_R {
        INA_SCALER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input A Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn ina_formatint(&mut self) -> INA_FORMATINT_W {
        INA_FORMATINT_W { w: self }
    }
    #[doc = "Bits 4:5 - Input A External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn ina_formatext(&mut self) -> INA_FORMATEXT_W {
        INA_FORMATEXT_W { w: self }
    }
    #[doc = "Bits 8:15 - Input A Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub fn ina_scaler(&mut self) -> INA_SCALER_W {
        INA_SCALER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input A format\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inaformat](index.html) module"]
pub struct INAFORMAT_SPEC;
impl crate::RegisterSpec for INAFORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inaformat::R](R) reader structure"]
impl crate::Readable for INAFORMAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inaformat::W](W) writer structure"]
impl crate::Writable for INAFORMAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INAFORMAT to value 0"]
impl crate::Resettable for INAFORMAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
