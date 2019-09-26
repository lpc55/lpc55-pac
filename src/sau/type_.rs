#[doc = "Reader of register TYPE"]
pub type R = crate::R<u32, super::TYPE>;
#[doc = "Writer for register TYPE"]
pub type W = crate::W<u32, super::TYPE>;
#[doc = "Register TYPE `reset()`'s with value 0"]
impl crate::ResetValue for super::TYPE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SREGION`"]
pub type SREGION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SREGION`"]
pub struct SREGION_W<'a> {
    w: &'a mut W,
}
impl<'a> SREGION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SAU regions. The number of implemented SAU regions."]
    #[inline(always)]
    pub fn sregion(&self) -> SREGION_R {
        SREGION_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SAU regions. The number of implemented SAU regions."]
    #[inline(always)]
    pub fn sregion(&mut self) -> SREGION_W {
        SREGION_W { w: self }
    }
}
