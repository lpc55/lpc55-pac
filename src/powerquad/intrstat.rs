#[doc = "Reader of register INTRSTAT"]
pub type R = crate::R<u32, super::INTRSTAT>;
#[doc = "Writer for register INTRSTAT"]
pub type W = crate::W<u32, super::INTRSTAT>;
#[doc = "Register INTRSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::INTRSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `intr_stat`"]
pub type INTR_STAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `intr_stat`"]
pub struct INTR_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_STAT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Intr status ( 1 bit to indicate interrupt captured, 0 means no new interrupt), write any value will clear this bit"]
    #[inline(always)]
    pub fn intr_stat(&self) -> INTR_STAT_R {
        INTR_STAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Intr status ( 1 bit to indicate interrupt captured, 0 means no new interrupt), write any value will clear this bit"]
    #[inline(always)]
    pub fn intr_stat(&mut self) -> INTR_STAT_W {
        INTR_STAT_W { w: self }
    }
}
