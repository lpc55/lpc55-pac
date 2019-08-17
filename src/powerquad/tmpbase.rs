#[doc = "Reader of register TMPBASE"]
pub type R = crate::R<u32, super::TMPBASE>;
#[doc = "Writer for register TMPBASE"]
pub type W = crate::W<u32, super::TMPBASE>;
#[doc = "Register TMPBASE `reset()`'s with value 0"]
impl crate::ResetValue for super::TMPBASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tmpbase`"]
pub type TMPBASE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `tmpbase`"]
pub struct TMPBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMPBASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Base address register for the temporary region"]
    #[inline(always)]
    pub fn tmpbase(&self) -> TMPBASE_R {
        TMPBASE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address register for the temporary region"]
    #[inline(always)]
    pub fn tmpbase(&mut self) -> TMPBASE_W {
        TMPBASE_W { w: self }
    }
}
