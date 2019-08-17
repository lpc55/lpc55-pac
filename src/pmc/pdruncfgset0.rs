#[doc = "Writer for register PDRUNCFGSET0"]
pub type W = crate::W<u32, super::PDRUNCFGSET0>;
#[doc = "Register PDRUNCFGSET0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PDRUNCFGSET0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `PDRUNCFGSET0`"]
pub struct PDRUNCFGSET0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDRUNCFGSET0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing ones to this register sets the corresponding bit or bits in the PDRUNCFG0 register, if they are implemented."]
    #[inline(always)]
    pub fn pdruncfgset0(&mut self) -> PDRUNCFGSET0_W {
        PDRUNCFGSET0_W { w: self }
    }
}
