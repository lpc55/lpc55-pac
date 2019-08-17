#[doc = "Reader of register SR_ENABLE2"]
pub type R = crate::R<u32, super::SR_ENABLE2>;
#[doc = "Writer for register SR_ENABLE2"]
pub type W = crate::W<u32, super::SR_ENABLE2>;
#[doc = "Register SR_ENABLE2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SR_ENABLE2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 2."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 2."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
