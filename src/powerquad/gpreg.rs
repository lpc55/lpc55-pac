#[doc = "Reader of register gpreg[%s]"]
pub type R = crate::R<u32, super::GPREG>;
#[doc = "Writer for register gpreg[%s]"]
pub type W = crate::W<u32, super::GPREG>;
#[doc = "Register gpreg[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::GPREG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gpreg`"]
pub type GPREG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `gpreg`"]
pub struct GPREG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPREG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - General purpose register bank"]
    #[inline(always)]
    pub fn gpreg(&self) -> GPREG_R {
        GPREG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - General purpose register bank"]
    #[inline(always)]
    pub fn gpreg(&mut self) -> GPREG_W {
        GPREG_W { w: self }
    }
}
