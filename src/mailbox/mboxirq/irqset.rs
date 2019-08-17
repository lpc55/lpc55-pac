#[doc = "Writer for register IRQSET"]
pub type W = crate::W<u32, super::IRQSET>;
#[doc = "Register IRQSET `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `INTREQSET`"]
pub struct INTREQSET_W<'a> {
    w: &'a mut W,
}
impl<'a> INTREQSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing 1 sets the corresponding bit in the IRQ0 register."]
    #[inline(always)]
    pub fn intreqset(&mut self) -> INTREQSET_W {
        INTREQSET_W { w: self }
    }
}
