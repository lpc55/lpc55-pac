#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `HOSTDISCONDETECT_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOSTDISCONDETECT_STATUS_A {
    #[doc = "USB cable disconnect has not been detected at the local host"]
    VALUE0,
    #[doc = "USB cable disconnect has been detected at the local host"]
    VALUE1,
}
impl From<HOSTDISCONDETECT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: HOSTDISCONDETECT_STATUS_A) -> Self {
        match variant {
            HOSTDISCONDETECT_STATUS_A::VALUE0 => false,
            HOSTDISCONDETECT_STATUS_A::VALUE1 => true,
        }
    }
}
#[doc = "Reader of field `HOSTDISCONDETECT_STATUS`"]
pub type HOSTDISCONDETECT_STATUS_R = crate::R<bool, HOSTDISCONDETECT_STATUS_A>;
impl HOSTDISCONDETECT_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOSTDISCONDETECT_STATUS_A {
        match self.bits {
            false => HOSTDISCONDETECT_STATUS_A::VALUE0,
            true => HOSTDISCONDETECT_STATUS_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == HOSTDISCONDETECT_STATUS_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HOSTDISCONDETECT_STATUS_A::VALUE1
    }
}
#[doc = "Write proxy for field `HOSTDISCONDETECT_STATUS`"]
pub struct HOSTDISCONDETECT_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> HOSTDISCONDETECT_STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HOSTDISCONDETECT_STATUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USB cable disconnect has not been detected at the local host"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(HOSTDISCONDETECT_STATUS_A::VALUE0)
    }
    #[doc = "USB cable disconnect has been detected at the local host"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HOSTDISCONDETECT_STATUS_A::VALUE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Possible values of the field `DEVPLUGIN_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVPLUGIN_STATUS_A {
    #[doc = "No attachment to a USB host is detected"]
    VALUE0,
    #[doc = "Cable attachment to a USB host is detected"]
    VALUE1,
}
impl From<DEVPLUGIN_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: DEVPLUGIN_STATUS_A) -> Self {
        match variant {
            DEVPLUGIN_STATUS_A::VALUE0 => false,
            DEVPLUGIN_STATUS_A::VALUE1 => true,
        }
    }
}
#[doc = "Reader of field `DEVPLUGIN_STATUS`"]
pub type DEVPLUGIN_STATUS_R = crate::R<bool, DEVPLUGIN_STATUS_A>;
impl DEVPLUGIN_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEVPLUGIN_STATUS_A {
        match self.bits {
            false => DEVPLUGIN_STATUS_A::VALUE0,
            true => DEVPLUGIN_STATUS_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == DEVPLUGIN_STATUS_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DEVPLUGIN_STATUS_A::VALUE1
    }
}
#[doc = "Write proxy for field `DEVPLUGIN_STATUS`"]
pub struct DEVPLUGIN_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVPLUGIN_STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEVPLUGIN_STATUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No attachment to a USB host is detected"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(DEVPLUGIN_STATUS_A::VALUE0)
    }
    #[doc = "Cable attachment to a USB host is detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DEVPLUGIN_STATUS_A::VALUE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `OTGID_STATUS`"]
pub type OTGID_STATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTGID_STATUS`"]
pub struct OTGID_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGID_STATUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESUME_STATUS`"]
pub type RESUME_STATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESUME_STATUS`"]
pub struct RESUME_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_STATUS_W<'a> {
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
    #[doc = "Bit 3 - Indicates at the local host (downstream) port that the remote device has disconnected while in High-Speed mode"]
    #[inline(always)]
    pub fn hostdiscondetect_status(&self) -> HOSTDISCONDETECT_STATUS_R {
        HOSTDISCONDETECT_STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Status indicator for non-standard resistive plugged-in detection Indicates that the device has been connected on the USB_DP and USB_DM lines using the nonstandard resistive plugged-in detection method controlled by CTRL\\[4\\]"]
    #[inline(always)]
    pub fn devplugin_status(&self) -> DEVPLUGIN_STATUS_R {
        DEVPLUGIN_STATUS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Indicates the results of USB_ID pin on the USB cable plugged into the local Micro- or Mini-AB receptacle"]
    #[inline(always)]
    pub fn otgid_status(&self) -> OTGID_STATUS_R {
        OTGID_STATUS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Indicates that the host is sending a wake-up after Suspend and has triggered an interrupt."]
    #[inline(always)]
    pub fn resume_status(&self) -> RESUME_STATUS_R {
        RESUME_STATUS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Indicates at the local host (downstream) port that the remote device has disconnected while in High-Speed mode"]
    #[inline(always)]
    pub fn hostdiscondetect_status(&mut self) -> HOSTDISCONDETECT_STATUS_W {
        HOSTDISCONDETECT_STATUS_W { w: self }
    }
    #[doc = "Bit 6 - Status indicator for non-standard resistive plugged-in detection Indicates that the device has been connected on the USB_DP and USB_DM lines using the nonstandard resistive plugged-in detection method controlled by CTRL\\[4\\]"]
    #[inline(always)]
    pub fn devplugin_status(&mut self) -> DEVPLUGIN_STATUS_W {
        DEVPLUGIN_STATUS_W { w: self }
    }
    #[doc = "Bit 8 - Indicates the results of USB_ID pin on the USB cable plugged into the local Micro- or Mini-AB receptacle"]
    #[inline(always)]
    pub fn otgid_status(&mut self) -> OTGID_STATUS_W {
        OTGID_STATUS_W { w: self }
    }
    #[doc = "Bit 10 - Indicates that the host is sending a wake-up after Suspend and has triggered an interrupt."]
    #[inline(always)]
    pub fn resume_status(&mut self) -> RESUME_STATUS_W {
        RESUME_STATUS_W { w: self }
    }
}
