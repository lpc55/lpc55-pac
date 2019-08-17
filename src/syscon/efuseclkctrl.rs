#[doc = "Reader of register EFUSECLKCTRL"]
pub type R = crate::R<u32, super::EFUSECLKCTRL>;
#[doc = "Writer for register EFUSECLKCTRL"]
pub type W = crate::W<u32, super::EFUSECLKCTRL>;
#[doc = "Register EFUSECLKCTRL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::EFUSECLKCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `EFUSECLKENA`"]
pub type EFUSECLKENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSECLKENA`"]
pub struct EFUSECLKENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSECLKENA_W<'a> {
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
    #[doc = "Bit 0 - eFUSE controller clock enable."]
    #[inline(always)]
    pub fn efuseclkena(&self) -> EFUSECLKENA_R {
        EFUSECLKENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - eFUSE controller clock enable."]
    #[inline(always)]
    pub fn efuseclkena(&mut self) -> EFUSECLKENA_W {
        EFUSECLKENA_W { w: self }
    }
}
