#[doc = "Reader of register FCTRL[%s]"]
pub type R = crate::R<u32, super::FCTRL>;
#[doc = "Writer for register FCTRL[%s]"]
pub type W = crate::W<u32, super::FCTRL>;
#[doc = "Register FCTRL[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::FCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FCOUNT`"]
pub type FCOUNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `FWMARK`"]
pub type FWMARK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FWMARK`"]
pub struct FWMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> FWMARK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Result FIFO counter"]
    #[inline(always)]
    pub fn fcount(&self) -> FCOUNT_R {
        FCOUNT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Watermark level selection"]
    #[inline(always)]
    pub fn fwmark(&self) -> FWMARK_R {
        FWMARK_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - Watermark level selection"]
    #[inline(always)]
    pub fn fwmark(&mut self) -> FWMARK_W {
        FWMARK_W { w: self }
    }
}
