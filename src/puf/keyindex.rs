#[doc = "Reader of register KEYINDEX"]
pub type R = crate::R<u32, super::KEYINDEX>;
#[doc = "Writer for register KEYINDEX"]
pub type W = crate::W<u32, super::KEYINDEX>;
#[doc = "Register KEYINDEX `reset()`'s with value 0"]
impl crate::ResetValue for super::KEYINDEX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `KEYIDX`"]
pub type KEYIDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KEYIDX`"]
pub struct KEYIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYIDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Key index for Set Key operations"]
    #[inline(always)]
    pub fn keyidx(&self) -> KEYIDX_R {
        KEYIDX_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Key index for Set Key operations"]
    #[inline(always)]
    pub fn keyidx(&mut self) -> KEYIDX_W {
        KEYIDX_W { w: self }
    }
}
