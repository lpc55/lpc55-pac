#[doc = "Reader of register PWREN"]
pub type R = crate::R<u32, super::PWREN>;
#[doc = "Writer for register PWREN"]
pub type W = crate::W<u32, super::PWREN>;
#[doc = "Register PWREN `reset()`'s with value 0"]
impl crate::ResetValue for super::PWREN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `POWER_ENABLE0`"]
pub type POWER_ENABLE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POWER_ENABLE0`"]
pub struct POWER_ENABLE0_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_ENABLE0_W<'a> {
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
#[doc = "Reader of field `POWER_ENABLE1`"]
pub type POWER_ENABLE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POWER_ENABLE1`"]
pub struct POWER_ENABLE1_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_ENABLE1_W<'a> {
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
    #[doc = "Bit 0 - Power on/off switch for card 0; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card 0."]
    #[inline(always)]
    pub fn power_enable0(&self) -> POWER_ENABLE0_R {
        POWER_ENABLE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Power on/off switch for card 1; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card 1."]
    #[inline(always)]
    pub fn power_enable1(&self) -> POWER_ENABLE1_R {
        POWER_ENABLE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power on/off switch for card 0; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card 0."]
    #[inline(always)]
    pub fn power_enable0(&mut self) -> POWER_ENABLE0_W {
        POWER_ENABLE0_W { w: self }
    }
    #[doc = "Bit 1 - Power on/off switch for card 1; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card 1."]
    #[inline(always)]
    pub fn power_enable1(&mut self) -> POWER_ENABLE1_W {
        POWER_ENABLE1_W { w: self }
    }
}
