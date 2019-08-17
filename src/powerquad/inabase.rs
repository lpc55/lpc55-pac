#[doc = "Reader of register INABASE"]
pub type R = crate::R<u32, super::INABASE>;
#[doc = "Writer for register INABASE"]
pub type W = crate::W<u32, super::INABASE>;
#[doc = "Register INABASE `reset()`'s with value 0"]
impl crate::ResetValue for super::INABASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `inabase`"]
pub type INABASE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `inabase`"]
pub struct INABASE_W<'a> {
    w: &'a mut W,
}
impl<'a> INABASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Base address register for the input A region"]
    #[inline(always)]
    pub fn inabase(&self) -> INABASE_R {
        INABASE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address register for the input A region"]
    #[inline(always)]
    pub fn inabase(&mut self) -> INABASE_W {
        INABASE_W { w: self }
    }
}
