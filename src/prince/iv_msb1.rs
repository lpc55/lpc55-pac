#[doc = "Writer for register IV_MSB1"]
pub type W = crate::W<u32, super::IV_MSB1>;
#[doc = "Register IV_MSB1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IV_MSB1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `IVVAL`"]
pub struct IVVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> IVVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Initial Vector value for the 32 Most Significant Bits of the 64-bit Initial Vector."]
    #[inline(always)]
    pub fn ivval(&mut self) -> IVVAL_W {
        IVVAL_W { w: self }
    }
}
