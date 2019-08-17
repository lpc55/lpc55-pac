#[doc = "Reader of register CORDIC_Z"]
pub type R = crate::R<u32, super::CORDIC_Z>;
#[doc = "Writer for register CORDIC_Z"]
pub type W = crate::W<u32, super::CORDIC_Z>;
#[doc = "Register CORDIC_Z `reset()`'s with value 0"]
impl crate::ResetValue for super::CORDIC_Z {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cordic_z`"]
pub type CORDIC_Z_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `cordic_z`"]
pub struct CORDIC_Z_W<'a> {
    w: &'a mut W,
}
impl<'a> CORDIC_Z_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Cordic input z"]
    #[inline(always)]
    pub fn cordic_z(&self) -> CORDIC_Z_R {
        CORDIC_Z_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cordic input z"]
    #[inline(always)]
    pub fn cordic_z(&mut self) -> CORDIC_Z_W {
        CORDIC_Z_W { w: self }
    }
}
