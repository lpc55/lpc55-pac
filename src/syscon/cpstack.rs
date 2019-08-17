#[doc = "Reader of register CPSTACK"]
pub type R = crate::R<u32, super::CPSTACK>;
#[doc = "Writer for register CPSTACK"]
pub type W = crate::W<u32, super::CPSTACK>;
#[doc = "Register CPSTACK `reset()`'s with value 0"]
impl crate::ResetValue for super::CPSTACK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CPSTACK`"]
pub type CPSTACK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CPSTACK`"]
pub struct CPSTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> CPSTACK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Coprocessor Stack Address. -- NOT USED"]
    #[inline(always)]
    pub fn cpstack(&self) -> CPSTACK_R {
        CPSTACK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Coprocessor Stack Address. -- NOT USED"]
    #[inline(always)]
    pub fn cpstack(&mut self) -> CPSTACK_W {
        CPSTACK_W { w: self }
    }
}
