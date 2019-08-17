#[doc = "Reader of register CURSORY"]
pub type R = crate::R<u32, super::CURSORY>;
#[doc = "Writer for register CURSORY"]
pub type W = crate::W<u32, super::CURSORY>;
#[doc = "Register CURSORY `reset()`'s with value 0"]
impl crate::ResetValue for super::CURSORY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cursory`"]
pub type CURSORY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cursory`"]
pub struct CURSORY_W<'a> {
    w: &'a mut W,
}
impl<'a> CURSORY_W<'a> {
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
    #[doc = "Bit 0 - 1 : Enable cursory mode"]
    #[inline(always)]
    pub fn cursory(&self) -> CURSORY_R {
        CURSORY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1 : Enable cursory mode"]
    #[inline(always)]
    pub fn cursory(&mut self) -> CURSORY_W {
        CURSORY_W { w: self }
    }
}
