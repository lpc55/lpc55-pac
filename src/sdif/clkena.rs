#[doc = "Reader of register CLKENA"]
pub type R = crate::R<u32, super::CLKENA>;
#[doc = "Writer for register CLKENA"]
pub type W = crate::W<u32, super::CLKENA>;
#[doc = "Register CLKENA `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCLK0_ENABLE`"]
pub type CCLK0_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCLK0_ENABLE`"]
pub struct CCLK0_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK0_ENABLE_W<'a> {
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
#[doc = "Reader of field `CCLK1_ENABLE`"]
pub type CCLK1_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCLK1_ENABLE`"]
pub struct CCLK1_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK1_ENABLE_W<'a> {
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
#[doc = "Reader of field `CCLK0_LOW_POWER`"]
pub type CCLK0_LOW_POWER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCLK0_LOW_POWER`"]
pub struct CCLK0_LOW_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK0_LOW_POWER_W<'a> {
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
#[doc = "Reader of field `CCLK1_LOW_POWER`"]
pub type CCLK1_LOW_POWER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCLK1_LOW_POWER`"]
pub struct CCLK1_LOW_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK1_LOW_POWER_W<'a> {
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
    #[doc = "Bit 0 - Clock-enable control for SD card 0 clock."]
    #[inline(always)]
    pub fn cclk0_enable(&self) -> CCLK0_ENABLE_R {
        CCLK0_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock-enable control for SD card 1 clock."]
    #[inline(always)]
    pub fn cclk1_enable(&self) -> CCLK1_ENABLE_R {
        CCLK1_ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Low-power control for SD card 0 clock."]
    #[inline(always)]
    pub fn cclk0_low_power(&self) -> CCLK0_LOW_POWER_R {
        CCLK0_LOW_POWER_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Low-power control for SD card 1 clock."]
    #[inline(always)]
    pub fn cclk1_low_power(&self) -> CCLK1_LOW_POWER_R {
        CCLK1_LOW_POWER_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock-enable control for SD card 0 clock."]
    #[inline(always)]
    pub fn cclk0_enable(&mut self) -> CCLK0_ENABLE_W {
        CCLK0_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Clock-enable control for SD card 1 clock."]
    #[inline(always)]
    pub fn cclk1_enable(&mut self) -> CCLK1_ENABLE_W {
        CCLK1_ENABLE_W { w: self }
    }
    #[doc = "Bit 16 - Low-power control for SD card 0 clock."]
    #[inline(always)]
    pub fn cclk0_low_power(&mut self) -> CCLK0_LOW_POWER_W {
        CCLK0_LOW_POWER_W { w: self }
    }
    #[doc = "Bit 17 - Low-power control for SD card 1 clock."]
    #[inline(always)]
    pub fn cclk1_low_power(&mut self) -> CCLK1_LOW_POWER_W {
        CCLK1_LOW_POWER_W { w: self }
    }
}
