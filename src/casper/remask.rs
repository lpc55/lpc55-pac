#[doc = "Reader of register REMASK"]
pub type R = crate::R<u32, super::REMASK>;
#[doc = "Writer for register REMASK"]
pub type W = crate::W<u32, super::REMASK>;
#[doc = "Register REMASK `reset()`'s with value 0"]
impl crate::ResetValue for super::REMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MASK`"]
pub type MASK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MASK`"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
}
