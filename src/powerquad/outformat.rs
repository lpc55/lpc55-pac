#[doc = "Reader of register OUTFORMAT"]
pub type R = crate::R<u32, super::OUTFORMAT>;
#[doc = "Writer for register OUTFORMAT"]
pub type W = crate::W<u32, super::OUTFORMAT>;
#[doc = "Register OUTFORMAT `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTFORMAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `out_formatint`"]
pub type OUT_FORMATINT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `out_formatint`"]
pub struct OUT_FORMATINT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_FORMATINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `out_formatext`"]
pub type OUT_FORMATEXT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `out_formatext`"]
pub struct OUT_FORMATEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_FORMATEXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `out_scaler`"]
pub type OUT_SCALER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `out_scaler`"]
pub struct OUT_SCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_SCALER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
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
}
