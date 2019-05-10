#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEBUG1_SET {
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
#[doc = "Possible values of the field `ENTAILADJVD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENTAILADJVDR {
    #[doc = "Delay is nominal"]
    VALUE0,
    #[doc = "Delay is +20%"]
    VALUE1,
    #[doc = "Delay is -20%"]
    VALUE2,
    #[doc = "Delay is -40%"]
    VALUE3,
}
impl ENTAILADJVDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENTAILADJVDR::VALUE0 => 0,
            ENTAILADJVDR::VALUE1 => 1,
            ENTAILADJVDR::VALUE2 => 2,
            ENTAILADJVDR::VALUE3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENTAILADJVDR {
        match value {
            0 => ENTAILADJVDR::VALUE0,
            1 => ENTAILADJVDR::VALUE1,
            2 => ENTAILADJVDR::VALUE2,
            3 => ENTAILADJVDR::VALUE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == ENTAILADJVDR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENTAILADJVDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENTAILADJVDR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ENTAILADJVDR::VALUE3
    }
}
#[doc = r" Value of the field"]
pub struct USB2_REFBIAS_VBGADJR {
    bits: u8,
}
impl USB2_REFBIAS_VBGADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USB2_REFBIAS_TSTR {
    bits: u8,
}
impl USB2_REFBIAS_TSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `ENTAILADJVD`"]
pub enum ENTAILADJVDW {
    #[doc = "Delay is nominal"]
    VALUE0,
    #[doc = "Delay is +20%"]
    VALUE1,
    #[doc = "Delay is -20%"]
    VALUE2,
    #[doc = "Delay is -40%"]
    VALUE3,
}
impl ENTAILADJVDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENTAILADJVDW::VALUE0 => 0,
            ENTAILADJVDW::VALUE1 => 1,
            ENTAILADJVDW::VALUE2 => 2,
            ENTAILADJVDW::VALUE3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENTAILADJVDW<'a> {
    w: &'a mut W,
}
impl<'a> _ENTAILADJVDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENTAILADJVDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Delay is nominal"]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(ENTAILADJVDW::VALUE0)
    }
    #[doc = "Delay is +20%"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENTAILADJVDW::VALUE1)
    }
    #[doc = "Delay is -20%"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENTAILADJVDW::VALUE2)
    }
    #[doc = "Delay is -40%"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ENTAILADJVDW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USB2_REFBIAS_VBGADJW<'a> {
    w: &'a mut W,
}
impl<'a> _USB2_REFBIAS_VBGADJW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USB2_REFBIAS_TSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB2_REFBIAS_TSTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
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
    #[doc = "Bits 13:14 - Delay increment of the rise of squelch:"]
    #[inline]
    pub fn entailadjvd(&self) -> ENTAILADJVDR {
        ENTAILADJVDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:20 - Adjustment bits on bandgap"]
    #[inline]
    pub fn usb2_refbias_vbgadj(&self) -> USB2_REFBIAS_VBGADJR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USB2_REFBIAS_VBGADJR { bits }
    }
    #[doc = "Bits 21:22 - Bias current control for usb2_phy"]
    #[inline]
    pub fn usb2_refbias_tst(&self) -> USB2_REFBIAS_TSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USB2_REFBIAS_TSTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4096 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 13:14 - Delay increment of the rise of squelch:"]
    #[inline]
    pub fn entailadjvd(&mut self) -> _ENTAILADJVDW {
        _ENTAILADJVDW { w: self }
    }
    #[doc = "Bits 18:20 - Adjustment bits on bandgap"]
    #[inline]
    pub fn usb2_refbias_vbgadj(&mut self) -> _USB2_REFBIAS_VBGADJW {
        _USB2_REFBIAS_VBGADJW { w: self }
    }
    #[doc = "Bits 21:22 - Bias current control for usb2_phy"]
    #[inline]
    pub fn usb2_refbias_tst(&mut self) -> _USB2_REFBIAS_TSTW {
        _USB2_REFBIAS_TSTW { w: self }
    }
}
