#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RX_SET {
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
#[doc = "Possible values of the field `ENVADJ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENVADJR {
    #[doc = "Trip-Level Voltage is 0.1000 V"]
    VALUE0,
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    VALUE1,
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    VALUE2,
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ENVADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENVADJR::VALUE0 => 0,
            ENVADJR::VALUE1 => 1,
            ENVADJR::VALUE2 => 2,
            ENVADJR::VALUE3 => 3,
            ENVADJR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENVADJR {
        match value {
            0 => ENVADJR::VALUE0,
            1 => ENVADJR::VALUE1,
            2 => ENVADJR::VALUE2,
            3 => ENVADJR::VALUE3,
            i => ENVADJR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == ENVADJR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENVADJR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENVADJR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ENVADJR::VALUE3
    }
}
#[doc = "Possible values of the field `DISCONADJ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCONADJR {
    #[doc = "Trip-Level Voltage is 0.56875 V"]
    VALUE0,
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    VALUE1,
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    VALUE2,
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DISCONADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DISCONADJR::VALUE0 => 0,
            DISCONADJR::VALUE1 => 1,
            DISCONADJR::VALUE2 => 2,
            DISCONADJR::VALUE3 => 3,
            DISCONADJR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DISCONADJR {
        match value {
            0 => DISCONADJR::VALUE0,
            1 => DISCONADJR::VALUE1,
            2 => DISCONADJR::VALUE2,
            3 => DISCONADJR::VALUE3,
            i => DISCONADJR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == DISCONADJR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DISCONADJR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DISCONADJR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DISCONADJR::VALUE3
    }
}
#[doc = "Possible values of the field `RXDBYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDBYPASSR {
    #[doc = "Normal operation."]
    VALUE0,
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    VALUE1,
}
impl RXDBYPASSR {
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
            RXDBYPASSR::VALUE0 => false,
            RXDBYPASSR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDBYPASSR {
        match value {
            false => RXDBYPASSR::VALUE0,
            true => RXDBYPASSR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == RXDBYPASSR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXDBYPASSR::VALUE1
    }
}
#[doc = "Values that can be written to the field `ENVADJ`"]
pub enum ENVADJW {
    #[doc = "Trip-Level Voltage is 0.1000 V"]
    VALUE0,
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    VALUE1,
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    VALUE2,
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    VALUE3,
}
impl ENVADJW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENVADJW::VALUE0 => 0,
            ENVADJW::VALUE1 => 1,
            ENVADJW::VALUE2 => 2,
            ENVADJW::VALUE3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENVADJW<'a> {
    w: &'a mut W,
}
impl<'a> _ENVADJW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENVADJW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Trip-Level Voltage is 0.1000 V"]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(ENVADJW::VALUE0)
    }
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENVADJW::VALUE1)
    }
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENVADJW::VALUE2)
    }
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ENVADJW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DISCONADJ`"]
pub enum DISCONADJW {
    #[doc = "Trip-Level Voltage is 0.56875 V"]
    VALUE0,
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    VALUE1,
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    VALUE2,
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    VALUE3,
}
impl DISCONADJW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DISCONADJW::VALUE0 => 0,
            DISCONADJW::VALUE1 => 1,
            DISCONADJW::VALUE2 => 2,
            DISCONADJW::VALUE3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISCONADJW<'a> {
    w: &'a mut W,
}
impl<'a> _DISCONADJW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISCONADJW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Trip-Level Voltage is 0.56875 V"]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(DISCONADJW::VALUE0)
    }
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DISCONADJW::VALUE1)
    }
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DISCONADJW::VALUE2)
    }
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DISCONADJW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXDBYPASS`"]
pub enum RXDBYPASSW {
    #[doc = "Normal operation."]
    VALUE0,
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    VALUE1,
}
impl RXDBYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDBYPASSW::VALUE0 => false,
            RXDBYPASSW::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXDBYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDBYPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXDBYPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(RXDBYPASSW::VALUE0)
    }
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXDBYPASSW::VALUE1)
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
        const OFFSET: u8 = 22;
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
    #[doc = "Bits 0:2 - The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline]
    pub fn envadj(&self) -> ENVADJR {
        ENVADJR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline]
    pub fn disconadj(&self) -> DISCONADJR {
        DISCONADJR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[inline]
    pub fn rxdbypass(&self) -> RXDBYPASSR {
        RXDBYPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bits 0:2 - The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline]
    pub fn envadj(&mut self) -> _ENVADJW {
        _ENVADJW { w: self }
    }
    #[doc = "Bits 4:6 - The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline]
    pub fn disconadj(&mut self) -> _DISCONADJW {
        _DISCONADJW { w: self }
    }
    #[doc = "Bit 22 - This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[inline]
    pub fn rxdbypass(&mut self) -> _RXDBYPASSW {
        _RXDBYPASSW { w: self }
    }
}
