#[doc = "Writer for register KEYMASK[%s]"]
pub type W = crate::W<u32, super::KEYMASK>;
#[doc = "Register KEYMASK[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::KEYMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `KEYMASK`"]
pub struct KEYMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn keymask(&mut self) -> KEYMASK_W {
        KEYMASK_W { w: self }
    }
}
