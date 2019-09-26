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
#[doc = "Reader of field `OK_STATUS_3V`"]
pub type OK_STATUS_3V_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `RESUME_STATUS`"]
pub type RESUME_STATUS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Indicates the USB 3v power rails are in range."]
    #[inline(always)]
    pub fn ok_status_3v(&self) -> OK_STATUS_3V_R {
        OK_STATUS_3V_R::new((self.bits & 0x01) != 0)
    }
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
    #[doc = "Bit 10 - Indicates that the host is sending a wake-up after Suspend and has triggered an interrupt."]
    #[inline(always)]
    pub fn resume_status(&self) -> RESUME_STATUS_R {
        RESUME_STATUS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {}
