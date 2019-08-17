#[doc = "Reader of register INBBASE"]
pub type R = crate::R<u32, super::INBBASE>;
#[doc = "Writer for register INBBASE"]
pub type W = crate::W<u32, super::INBBASE>;
#[doc = "Register INBBASE `reset()`'s with value 0"]
impl crate::ResetValue for super::INBBASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `inbbase`"]
pub type INBBASE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `inbbase`"]
pub struct INBBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> INBBASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Base address register for the input B region"]
    #[inline(always)]
    pub fn inbbase(&self) -> INBBASE_R {
        INBBASE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address register for the input B region"]
    #[inline(always)]
    pub fn inbbase(&mut self) -> INBBASE_W {
        INBBASE_W { w: self }
    }
}
