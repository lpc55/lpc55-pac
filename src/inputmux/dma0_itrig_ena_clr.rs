#[doc = "Writer for register DMA0_ITRIG_ENA_CLR"]
pub type W = crate::W<u32, super::DMA0_ITRIG_ENA_CLR>;
#[doc = "Register DMA0_ITRIG_ENA_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA0_ITRIG_ENA_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CLR`"]
pub struct CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:21 - Write : If bit #i = 1, bit #i in DMA0_ITRIG_ENA register is reset to 0; if bit #i = 0 , no change in DMA0_ITRIG_ENA register"]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W {
        CLR_W { w: self }
    }
}
