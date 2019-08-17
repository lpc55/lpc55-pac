#[doc = "Reader of register KEYSIZE"]
pub type R = crate::R<u32, super::KEYSIZE>;
#[doc = "Writer for register KEYSIZE"]
pub type W = crate::W<u32, super::KEYSIZE>;
#[doc = "Register KEYSIZE `reset()`'s with value 0"]
impl crate::ResetValue for super::KEYSIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `KEYSIZE`"]
pub type KEYSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KEYSIZE`"]
pub struct KEYSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Key size for Set Key operations"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Key size for Set Key operations"]
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W {
        KEYSIZE_W { w: self }
    }
}
