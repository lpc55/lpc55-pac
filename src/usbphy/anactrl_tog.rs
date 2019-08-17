#[doc = "Reader of register ANACTRL_TOG"]
pub type R = crate::R<u32, super::ANACTRL_TOG>;
#[doc = "Writer for register ANACTRL_TOG"]
pub type W = crate::W<u32, super::ANACTRL_TOG>;
#[doc = "Register ANACTRL_TOG `reset()`'s with value 0x0a00_0402"]
impl crate::ResetValue for super::ANACTRL_TOG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a00_0402
    }
}
#[doc = "Possible values of the field `DEV_PULLDOWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV_PULLDOWN_A {
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    VALUE0,
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    VALUE1,
}
impl From<DEV_PULLDOWN_A> for bool {
    #[inline(always)]
    fn from(variant: DEV_PULLDOWN_A) -> Self {
        match variant {
            DEV_PULLDOWN_A::VALUE0 => false,
            DEV_PULLDOWN_A::VALUE1 => true,
        }
    }
}
#[doc = "Reader of field `DEV_PULLDOWN`"]
pub type DEV_PULLDOWN_R = crate::R<bool, DEV_PULLDOWN_A>;
impl DEV_PULLDOWN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEV_PULLDOWN_A {
        match self.bits {
            false => DEV_PULLDOWN_A::VALUE0,
            true => DEV_PULLDOWN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == DEV_PULLDOWN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DEV_PULLDOWN_A::VALUE1
    }
}
#[doc = "Write proxy for field `DEV_PULLDOWN`"]
pub struct DEV_PULLDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_PULLDOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEV_PULLDOWN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(DEV_PULLDOWN_A::VALUE0)
    }
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DEV_PULLDOWN_A::VALUE1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 10 - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline(always)]
    pub fn dev_pulldown(&self) -> DEV_PULLDOWN_R {
        DEV_PULLDOWN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline(always)]
    pub fn dev_pulldown(&mut self) -> DEV_PULLDOWN_W {
        DEV_PULLDOWN_W { w: self }
    }
}
