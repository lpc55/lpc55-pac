#[doc = "Reader of register MEMADDR"]
pub type R = crate::R<u32, super::MEMADDR>;
#[doc = "Writer for register MEMADDR"]
pub type W = crate::W<u32, super::MEMADDR>;
#[doc = "Register MEMADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::MEMADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BASE`"]
pub type BASE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BASE`"]
pub struct BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Address base to start copying from, word aligned (so bits 1:0 must be 0). This field will advance as it processes the words. If it fails with a bus error, the register will contain the failing word. N:Address in Flash or RAM space; RAM only as mapped in this part. May also be able to address SPIFI."]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address base to start copying from, word aligned (so bits 1:0 must be 0). This field will advance as it processes the words. If it fails with a bus error, the register will contain the failing word. N:Address in Flash or RAM space; RAM only as mapped in this part. May also be able to address SPIFI."]
    #[inline(always)]
    pub fn base(&mut self) -> BASE_W {
        BASE_W { w: self }
    }
}
