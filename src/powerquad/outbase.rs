#[doc = "Reader of register OUTBASE"]
pub type R = crate::R<u32, super::OUTBASE>;
#[doc = "Writer for register OUTBASE"]
pub type W = crate::W<u32, super::OUTBASE>;
#[doc = "Register OUTBASE `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTBASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `outbase`"]
pub type OUTBASE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `outbase`"]
pub struct OUTBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTBASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Base address register for the output region"]
    #[inline(always)]
    pub fn outbase(&self) -> OUTBASE_R {
        OUTBASE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address register for the output region"]
    #[inline(always)]
    pub fn outbase(&mut self) -> OUTBASE_W {
        OUTBASE_W { w: self }
    }
}
