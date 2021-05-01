#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<STATUS_SPEC>> for R {
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<STATUS_SPEC>> for W {
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OK_STATUS_3V` reader - Indicates the USB 3v power rails are in range."]
pub struct OK_STATUS_3V_R(crate::FieldReader<bool, bool>);
impl OK_STATUS_3V_R {
    pub(crate) fn new(bits: bool) -> Self {
        OK_STATUS_3V_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OK_STATUS_3V_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Indicates at the local host (downstream) port that the remote device has disconnected while in High-Speed mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOSTDISCONDETECT_STATUS_A {
    #[doc = "0: USB cable disconnect has not been detected at the local host"]
    VALUE0 = 0,
    #[doc = "1: USB cable disconnect has been detected at the local host"]
    VALUE1 = 1,
}
impl From<HOSTDISCONDETECT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: HOSTDISCONDETECT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOSTDISCONDETECT_STATUS` reader - Indicates at the local host (downstream) port that the remote device has disconnected while in High-Speed mode"]
pub struct HOSTDISCONDETECT_STATUS_R(crate::FieldReader<bool, HOSTDISCONDETECT_STATUS_A>);
impl HOSTDISCONDETECT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        HOSTDISCONDETECT_STATUS_R(crate::FieldReader::new(bits))
    }
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
        **self == HOSTDISCONDETECT_STATUS_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == HOSTDISCONDETECT_STATUS_A::VALUE1
    }
}
impl core::ops::Deref for HOSTDISCONDETECT_STATUS_R {
    type Target = crate::FieldReader<bool, HOSTDISCONDETECT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Status indicator for non-standard resistive plugged-in detection Indicates that the device has been connected on the USB_DP and USB_DM lines using the nonstandard resistive plugged-in detection method controlled by CTRL\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVPLUGIN_STATUS_A {
    #[doc = "0: No attachment to a USB host is detected"]
    VALUE0 = 0,
    #[doc = "1: Cable attachment to a USB host is detected"]
    VALUE1 = 1,
}
impl From<DEVPLUGIN_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: DEVPLUGIN_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVPLUGIN_STATUS` reader - Status indicator for non-standard resistive plugged-in detection Indicates that the device has been connected on the USB_DP and USB_DM lines using the nonstandard resistive plugged-in detection method controlled by CTRL\\[4\\]"]
pub struct DEVPLUGIN_STATUS_R(crate::FieldReader<bool, DEVPLUGIN_STATUS_A>);
impl DEVPLUGIN_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEVPLUGIN_STATUS_R(crate::FieldReader::new(bits))
    }
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
        **self == DEVPLUGIN_STATUS_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DEVPLUGIN_STATUS_A::VALUE1
    }
}
impl core::ops::Deref for DEVPLUGIN_STATUS_R {
    type Target = crate::FieldReader<bool, DEVPLUGIN_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUME_STATUS` reader - Indicates that the host is sending a wake-up after Suspend and has triggered an interrupt."]
pub struct RESUME_STATUS_R(crate::FieldReader<bool, bool>);
impl RESUME_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESUME_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESUME_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
