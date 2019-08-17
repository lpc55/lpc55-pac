#[doc = "Reader of register INAFORMAT"]
pub type R = crate::R<u32, super::INAFORMAT>;
#[doc = "Writer for register INAFORMAT"]
pub type W = crate::W<u32, super::INAFORMAT>;
#[doc = "Register INAFORMAT `reset()`'s with value 0"]
impl crate::ResetValue for super::INAFORMAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ina_formatint`"]
pub type INA_FORMATINT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ina_formatint`"]
pub struct INA_FORMATINT_W<'a> {
    w: &'a mut W,
}
impl<'a> INA_FORMATINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `ina_formatext`"]
pub type INA_FORMATEXT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ina_formatext`"]
pub struct INA_FORMATEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> INA_FORMATEXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `ina_scaler`"]
pub type INA_SCALER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ina_scaler`"]
pub struct INA_SCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> INA_SCALER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
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
}
