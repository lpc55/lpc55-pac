#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ANACTRL_TOG {
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
#[doc = "Possible values of the field `DEV_PULLDOWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV_PULLDOWNR {
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    VALUE0,
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    VALUE1,
}
impl DEV_PULLDOWNR {
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
            DEV_PULLDOWNR::VALUE0 => false,
            DEV_PULLDOWNR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEV_PULLDOWNR {
        match value {
            false => DEV_PULLDOWNR::VALUE0,
            true => DEV_PULLDOWNR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == DEV_PULLDOWNR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DEV_PULLDOWNR::VALUE1
    }
}
#[doc = "Values that can be written to the field `DEV_PULLDOWN`"]
pub enum DEV_PULLDOWNW {
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    VALUE0,
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    VALUE1,
}
impl DEV_PULLDOWNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEV_PULLDOWNW::VALUE0 => false,
            DEV_PULLDOWNW::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEV_PULLDOWNW<'a> {
    w: &'a mut W,
}
impl<'a> _DEV_PULLDOWNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEV_PULLDOWNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(DEV_PULLDOWNW::VALUE0)
    }
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DEV_PULLDOWNW::VALUE1)
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
    #[doc = "Bit 10 - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline]
    pub fn dev_pulldown(&self) -> DEV_PULLDOWNR {
        DEV_PULLDOWNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 167773186 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 10 - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline]
    pub fn dev_pulldown(&mut self) -> _DEV_PULLDOWNW {
        _DEV_PULLDOWNW { w: self }
    }
}
