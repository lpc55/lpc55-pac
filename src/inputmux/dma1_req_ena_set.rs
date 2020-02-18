#[doc = "Writer for register DMA1_REQ_ENA_SET"]
pub type W = crate::W<u32, super::DMA1_REQ_ENA_SET>;
#[doc = "Register DMA1_REQ_ENA_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA1_REQ_ENA_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SET`"]
pub struct SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:9 - Write : If bit #i = 1, bit #i in DMA1_REQ_ENA register is set to 1; if bit #i = 0 , no change in DMA1_REQ_ENA register"]
    #[inline(always)]
    pub fn set(&mut self) -> SET_W {
        SET_W { w: self }
    }
}
