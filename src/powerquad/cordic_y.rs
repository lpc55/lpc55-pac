#[doc = "Reader of register CORDIC_Y"]
pub type R = crate::R<u32, super::CORDIC_Y>;
#[doc = "Writer for register CORDIC_Y"]
pub type W = crate::W<u32, super::CORDIC_Y>;
#[doc = "Register CORDIC_Y `reset()`'s with value 0"]
impl crate::ResetValue for super::CORDIC_Y {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cordic_y`"]
pub type CORDIC_Y_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `cordic_y`"]
pub struct CORDIC_Y_W<'a> {
    w: &'a mut W,
}
impl<'a> CORDIC_Y_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Cordic input y"]
    #[inline(always)]
    pub fn cordic_y(&self) -> CORDIC_Y_R {
        CORDIC_Y_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cordic input y"]
    #[inline(always)]
    pub fn cordic_y(&mut self) -> CORDIC_Y_W {
        CORDIC_Y_W { w: self }
    }
}
