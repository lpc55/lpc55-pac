#[doc = "Reader of register SAU_RNR"]
pub type R = crate::R<u32, super::SAU_RNR>;
#[doc = "Writer for register SAU_RNR"]
pub type W = crate::W<u32, super::SAU_RNR>;
#[doc = "Register SAU_RNR `reset()`'s with value 0"]
impl crate::ResetValue for super::SAU_RNR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REGION`"]
pub type REGION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REGION`"]
pub struct REGION_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Region number."]
    #[inline(always)]
    pub fn region(&self) -> REGION_R {
        REGION_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Region number."]
    #[inline(always)]
    pub fn region(&mut self) -> REGION_W {
        REGION_W { w: self }
    }
}
