#[doc = "Reader of register OFSTRIM"]
pub type R = crate::R<u32, super::OFSTRIM>;
#[doc = "Writer for register OFSTRIM"]
pub type W = crate::W<u32, super::OFSTRIM>;
#[doc = "Register OFSTRIM `reset()`'s with value 0"]
impl crate::ResetValue for super::OFSTRIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OFSTRIM_A`"]
pub type OFSTRIM_A_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OFSTRIM_A`"]
pub struct OFSTRIM_A_W<'a> {
    w: &'a mut W,
}
impl<'a> OFSTRIM_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `OFSTRIM_B`"]
pub type OFSTRIM_B_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OFSTRIM_B`"]
pub struct OFSTRIM_B_W<'a> {
    w: &'a mut W,
}
impl<'a> OFSTRIM_B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Trim for offset"]
    #[inline(always)]
    pub fn ofstrim_a(&self) -> OFSTRIM_A_R {
        OFSTRIM_A_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Trim for offset"]
    #[inline(always)]
    pub fn ofstrim_b(&self) -> OFSTRIM_B_R {
        OFSTRIM_B_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim for offset"]
    #[inline(always)]
    pub fn ofstrim_a(&mut self) -> OFSTRIM_A_W {
        OFSTRIM_A_W { w: self }
    }
    #[doc = "Bits 16:20 - Trim for offset"]
    #[inline(always)]
    pub fn ofstrim_b(&mut self) -> OFSTRIM_B_W {
        OFSTRIM_B_W { w: self }
    }
}
