#[doc = "Reader of register STARTA"]
pub type R = crate::R<u32, super::STARTA>;
#[doc = "Writer for register STARTA"]
pub type W = crate::W<u32, super::STARTA>;
#[doc = "Register STARTA `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STARTA`"]
pub type STARTA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STARTA`"]
pub struct STARTA_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Address / Start address for commands that take an address (range) as a parameter."]
    #[inline(always)]
    pub fn starta(&self) -> STARTA_R {
        STARTA_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Address / Start address for commands that take an address (range) as a parameter."]
    #[inline(always)]
    pub fn starta(&mut self) -> STARTA_W {
        STARTA_W { w: self }
    }
}
