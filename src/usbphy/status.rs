#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STATUS {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `HOSTDISCONDETECT_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOSTDISCONDETECT_STATUSR {
    #[doc = "USB cable disconnect has not been detected at the local host"]
    VALUE0,
    #[doc = "USB cable disconnect has been detected at the local host"]
    VALUE1,
}
impl HOSTDISCONDETECT_STATUSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HOSTDISCONDETECT_STATUSR::VALUE0 => false,
            HOSTDISCONDETECT_STATUSR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HOSTDISCONDETECT_STATUSR {
        match value {
            false => HOSTDISCONDETECT_STATUSR::VALUE0,
            true => HOSTDISCONDETECT_STATUSR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == HOSTDISCONDETECT_STATUSR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HOSTDISCONDETECT_STATUSR::VALUE1
    }
}
#[doc = "Possible values of the field `DEVPLUGIN_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVPLUGIN_STATUSR {
    #[doc = "No attachment to a USB host is detected"]
    VALUE0,
    #[doc = "Cable attachment to a USB host is detected"]
    VALUE1,
}
impl DEVPLUGIN_STATUSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DEVPLUGIN_STATUSR::VALUE0 => false,
            DEVPLUGIN_STATUSR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEVPLUGIN_STATUSR {
        match value {
            false => DEVPLUGIN_STATUSR::VALUE0,
            true => DEVPLUGIN_STATUSR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == DEVPLUGIN_STATUSR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DEVPLUGIN_STATUSR::VALUE1
    }
}
#[doc = r" Value of the field"]
pub struct OTGID_STATUSR {
    bits: bool,
}
impl OTGID_STATUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RESUME_STATUSR {
    bits: bool,
}
impl RESUME_STATUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `HOSTDISCONDETECT_STATUS`"]
pub enum HOSTDISCONDETECT_STATUSW {
    #[doc = "USB cable disconnect has not been detected at the local host"]
    VALUE0,
    #[doc = "USB cable disconnect has been detected at the local host"]
    VALUE1,
}
impl HOSTDISCONDETECT_STATUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HOSTDISCONDETECT_STATUSW::VALUE0 => false,
            HOSTDISCONDETECT_STATUSW::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HOSTDISCONDETECT_STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _HOSTDISCONDETECT_STATUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HOSTDISCONDETECT_STATUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "USB cable disconnect has not been detected at the local host"]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(HOSTDISCONDETECT_STATUSW::VALUE0)
    }
    #[doc = "USB cable disconnect has been detected at the local host"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HOSTDISCONDETECT_STATUSW::VALUE1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DEVPLUGIN_STATUS`"]
pub enum DEVPLUGIN_STATUSW {
    #[doc = "No attachment to a USB host is detected"]
    VALUE0,
    #[doc = "Cable attachment to a USB host is detected"]
    VALUE1,
}
impl DEVPLUGIN_STATUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEVPLUGIN_STATUSW::VALUE0 => false,
            DEVPLUGIN_STATUSW::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEVPLUGIN_STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _DEVPLUGIN_STATUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEVPLUGIN_STATUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No attachment to a USB host is detected"]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(DEVPLUGIN_STATUSW::VALUE0)
    }
    #[doc = "Cable attachment to a USB host is detected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DEVPLUGIN_STATUSW::VALUE1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OTGID_STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _OTGID_STATUSW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESUME_STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _RESUME_STATUSW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 3 - Indicates at the local host (downstream) port that the remote device has disconnected while in High-Speed mode"]
    #[inline]
    pub fn hostdiscondetect_status(&self) -> HOSTDISCONDETECT_STATUSR {
        HOSTDISCONDETECT_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Status indicator for non-standard resistive plugged-in detection Indicates that the device has been connected on the USB_DP and USB_DM lines using the nonstandard resistive plugged-in detection method controlled by CTRL\\[4\\]"]
    #[inline]
    pub fn devplugin_status(&self) -> DEVPLUGIN_STATUSR {
        DEVPLUGIN_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Indicates the results of USB_ID pin on the USB cable plugged into the local Micro- or Mini-AB receptacle"]
    #[inline]
    pub fn otgid_status(&self) -> OTGID_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTGID_STATUSR { bits }
    }
    #[doc = "Bit 10 - Indicates that the host is sending a wake-up after Suspend and has triggered an interrupt."]
    #[inline]
    pub fn resume_status(&self) -> RESUME_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESUME_STATUSR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 3 - Indicates at the local host (downstream) port that the remote device has disconnected while in High-Speed mode"]
    #[inline]
    pub fn hostdiscondetect_status(&mut self) -> _HOSTDISCONDETECT_STATUSW {
        _HOSTDISCONDETECT_STATUSW { w: self }
    }
    #[doc = "Bit 6 - Status indicator for non-standard resistive plugged-in detection Indicates that the device has been connected on the USB_DP and USB_DM lines using the nonstandard resistive plugged-in detection method controlled by CTRL\\[4\\]"]
    #[inline]
    pub fn devplugin_status(&mut self) -> _DEVPLUGIN_STATUSW {
        _DEVPLUGIN_STATUSW { w: self }
    }
    #[doc = "Bit 8 - Indicates the results of USB_ID pin on the USB cable plugged into the local Micro- or Mini-AB receptacle"]
    #[inline]
    pub fn otgid_status(&mut self) -> _OTGID_STATUSW {
        _OTGID_STATUSW { w: self }
    }
    #[doc = "Bit 10 - Indicates that the host is sending a wake-up after Suspend and has triggered an interrupt."]
    #[inline]
    pub fn resume_status(&mut self) -> _RESUME_STATUSW {
        _RESUME_STATUSW { w: self }
    }
}
