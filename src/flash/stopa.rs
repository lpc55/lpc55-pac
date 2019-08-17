#[doc = "Reader of register STOPA"]
pub type R = crate::R<u32, super::STOPA>;
#[doc = "Writer for register STOPA"]
pub type W = crate::W<u32, super::STOPA>;
#[doc = "Register STOPA `reset()`'s with value 0"]
impl crate::ResetValue for super::STOPA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STOPA`"]
pub type STOPA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STOPA`"]
pub struct STOPA_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Stop address for commands that take an address range as a parameter (the word specified by STOPA is included in the address range)."]
    #[inline(always)]
    pub fn stopa(&self) -> STOPA_R {
        STOPA_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Stop address for commands that take an address range as a parameter (the word specified by STOPA is included in the address range)."]
    #[inline(always)]
    pub fn stopa(&mut self) -> STOPA_W {
        STOPA_W { w: self }
    }
}
