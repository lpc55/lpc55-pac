#[doc = "Reader of register TMPFORMAT"]
pub type R = crate::R<u32, super::TMPFORMAT>;
#[doc = "Writer for register TMPFORMAT"]
pub type W = crate::W<u32, super::TMPFORMAT>;
#[doc = "Register TMPFORMAT `reset()`'s with value 0"]
impl crate::ResetValue for super::TMPFORMAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tmp_formatint`"]
pub type TMP_FORMATINT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tmp_formatint`"]
pub struct TMP_FORMATINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMP_FORMATINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `tmp_formatext`"]
pub type TMP_FORMATEXT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tmp_formatext`"]
pub struct TMP_FORMATEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMP_FORMATEXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `tmp_scaler`"]
pub type TMP_SCALER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tmp_scaler`"]
pub struct TMP_SCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> TMP_SCALER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
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
}
