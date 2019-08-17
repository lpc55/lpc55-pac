#[doc = "Reader of register DICE_REG5"]
pub type R = crate::R<u32, super::DICE_REG5>;
#[doc = "Writer for register DICE_REG5"]
pub type W = crate::W<u32, super::DICE_REG5>;
#[doc = "Register DICE_REG5 `reset()`'s with value 0"]
impl crate::ResetValue for super::DICE_REG5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DICE_REG5`"]
pub type DICE_REG5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DICE_REG5`"]
pub struct DICE_REG5_W<'a> {
    w: &'a mut W,
}
impl<'a> DICE_REG5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn dice_reg5(&self) -> DICE_REG5_R {
        DICE_REG5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn dice_reg5(&mut self) -> DICE_REG5_W {
        DICE_REG5_W { w: self }
    }
}
