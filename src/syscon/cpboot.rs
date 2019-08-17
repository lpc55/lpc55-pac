#[doc = "Reader of register CPBOOT"]
pub type R = crate::R<u32, super::CPBOOT>;
#[doc = "Writer for register CPBOOT"]
pub type W = crate::W<u32, super::CPBOOT>;
#[doc = "Register CPBOOT `reset()`'s with value 0"]
impl crate::ResetValue for super::CPBOOT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CPBOOT`"]
pub type CPBOOT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CPBOOT`"]
pub struct CPBOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> CPBOOT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Coprocessor Boot Address for CPU1."]
    #[inline(always)]
    pub fn cpboot(&self) -> CPBOOT_R {
        CPBOOT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Coprocessor Boot Address for CPU1."]
    #[inline(always)]
    pub fn cpboot(&mut self) -> CPBOOT_W {
        CPBOOT_W { w: self }
    }
}
