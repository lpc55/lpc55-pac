#[doc = "Reader of register CDETECT"]
pub type R = crate::R<u32, super::CDETECT>;
#[doc = "Writer for register CDETECT"]
pub type W = crate::W<u32, super::CDETECT>;
#[doc = "Register CDETECT `reset()`'s with value 0"]
impl crate::ResetValue for super::CDETECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CARD0_DETECT`"]
pub type CARD0_DETECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD0_DETECT`"]
pub struct CARD0_DETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD0_DETECT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `CARD1_DETECT`"]
pub type CARD1_DETECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD1_DETECT`"]
pub struct CARD1_DETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD1_DETECT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Card 0 detect"]
    #[inline(always)]
    pub fn card0_detect(&self) -> CARD0_DETECT_R {
        CARD0_DETECT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Card 1 detect"]
    #[inline(always)]
    pub fn card1_detect(&self) -> CARD1_DETECT_R {
        CARD1_DETECT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Card 0 detect"]
    #[inline(always)]
    pub fn card0_detect(&mut self) -> CARD0_DETECT_W {
        CARD0_DETECT_W { w: self }
    }
    #[doc = "Bit 1 - Card 1 detect"]
    #[inline(always)]
    pub fn card1_detect(&mut self) -> CARD1_DETECT_W {
        CARD1_DETECT_W { w: self }
    }
}
