#[doc = "Writer for register MASK_MSB"]
pub type W = crate::W<u32, super::MASK_MSB>;
#[doc = "Register MASK_MSB `reset()`'s with value 0"]
impl crate::ResetValue for super::MASK_MSB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `MASKVAL`"]
pub struct MASKVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of the 32 Most Significant Bits of the 64-bit data mask."]
    #[inline(always)]
    pub fn maskval(&mut self) -> MASKVAL_W {
        MASKVAL_W { w: self }
    }
}
