#[doc = "Reader of register DMA1_OTRIG_INMUX[%s]"]
pub type R = crate::R<u32, super::DMA1_OTRIG_INMUX>;
#[doc = "Writer for register DMA1_OTRIG_INMUX[%s]"]
pub type W = crate::W<u32, super::DMA1_OTRIG_INMUX>;
#[doc = "Register DMA1_OTRIG_INMUX[%s]
`reset()`'s with value 0x0f"]
impl crate::ResetValue for super::DMA1_OTRIG_INMUX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `INP`"]
pub type INP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INP`"]
pub struct INP_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 9)."]
    #[inline(always)]
    pub fn inp(&self) -> INP_R {
        INP_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 9)."]
    #[inline(always)]
    pub fn inp(&mut self) -> INP_W {
        INP_W { w: self }
    }
}
