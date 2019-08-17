#[doc = "Reader of register INBFORMAT"]
pub type R = crate::R<u32, super::INBFORMAT>;
#[doc = "Writer for register INBFORMAT"]
pub type W = crate::W<u32, super::INBFORMAT>;
#[doc = "Register INBFORMAT `reset()`'s with value 0"]
impl crate::ResetValue for super::INBFORMAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `inb_formatint`"]
pub type INB_FORMATINT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `inb_formatint`"]
pub struct INB_FORMATINT_W<'a> {
    w: &'a mut W,
}
impl<'a> INB_FORMATINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `inb_formatext`"]
pub type INB_FORMATEXT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `inb_formatext`"]
pub struct INB_FORMATEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> INB_FORMATEXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `inb_scaler`"]
pub type INB_SCALER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `inb_scaler`"]
pub struct INB_SCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> INB_SCALER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
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
}
