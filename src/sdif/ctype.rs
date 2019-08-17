#[doc = "Reader of register CTYPE"]
pub type R = crate::R<u32, super::CTYPE>;
#[doc = "Writer for register CTYPE"]
pub type W = crate::W<u32, super::CTYPE>;
#[doc = "Register CTYPE `reset()`'s with value 0"]
impl crate::ResetValue for super::CTYPE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CARD0_WIDTH0`"]
pub type CARD0_WIDTH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD0_WIDTH0`"]
pub struct CARD0_WIDTH0_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD0_WIDTH0_W<'a> {
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
#[doc = "Reader of field `CARD1_WIDTH0`"]
pub type CARD1_WIDTH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD1_WIDTH0`"]
pub struct CARD1_WIDTH0_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD1_WIDTH0_W<'a> {
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
#[doc = "Reader of field `CARD0_WIDTH1`"]
pub type CARD0_WIDTH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD0_WIDTH1`"]
pub struct CARD0_WIDTH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD0_WIDTH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `CARD1_WIDTH1`"]
pub type CARD1_WIDTH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD1_WIDTH1`"]
pub struct CARD1_WIDTH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD1_WIDTH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Indicates if card 0 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD0_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[inline(always)]
    pub fn card0_width0(&self) -> CARD0_WIDTH0_R {
        CARD0_WIDTH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates if card 1 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD1_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[inline(always)]
    pub fn card1_width0(&self) -> CARD1_WIDTH0_R {
        CARD1_WIDTH0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Indicates if card 0 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[inline(always)]
    pub fn card0_width1(&self) -> CARD0_WIDTH1_R {
        CARD0_WIDTH1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Indicates if card 1 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[inline(always)]
    pub fn card1_width1(&self) -> CARD1_WIDTH1_R {
        CARD1_WIDTH1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates if card 0 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD0_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[inline(always)]
    pub fn card0_width0(&mut self) -> CARD0_WIDTH0_W {
        CARD0_WIDTH0_W { w: self }
    }
    #[doc = "Bit 1 - Indicates if card 1 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD1_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[inline(always)]
    pub fn card1_width0(&mut self) -> CARD1_WIDTH0_W {
        CARD1_WIDTH0_W { w: self }
    }
    #[doc = "Bit 16 - Indicates if card 0 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[inline(always)]
    pub fn card0_width1(&mut self) -> CARD0_WIDTH1_W {
        CARD0_WIDTH1_W { w: self }
    }
    #[doc = "Bit 17 - Indicates if card 1 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[inline(always)]
    pub fn card1_width1(&mut self) -> CARD1_WIDTH1_W {
        CARD1_WIDTH1_W { w: self }
    }
}
