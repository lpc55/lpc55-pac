#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RESETCTRL {
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
#[doc = "Possible values of the field `DPDWAKEUPRESETENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPDWAKEUPRESETENABLER {
    #[doc = "Reset event from DEEP POWER DOWN mode is disable."]
    DISABLE,
    #[doc = "Reset event from DEEP POWER DOWN mode is enable."]
    ENABLE,
}
impl DPDWAKEUPRESETENABLER {
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
            DPDWAKEUPRESETENABLER::DISABLE => false,
            DPDWAKEUPRESETENABLER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPDWAKEUPRESETENABLER {
        match value {
            false => DPDWAKEUPRESETENABLER::DISABLE,
            true => DPDWAKEUPRESETENABLER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DPDWAKEUPRESETENABLER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == DPDWAKEUPRESETENABLER::ENABLE
    }
}
#[doc = "Possible values of the field `BODVBATRESETENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODVBATRESETENABLER {
    #[doc = "BOD VBAT reset is disable."]
    DISABLE,
    #[doc = "BOD VBAT reset is enable."]
    ENABLE,
}
impl BODVBATRESETENABLER {
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
            BODVBATRESETENABLER::DISABLE => false,
            BODVBATRESETENABLER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODVBATRESETENABLER {
        match value {
            false => BODVBATRESETENABLER::DISABLE,
            true => BODVBATRESETENABLER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == BODVBATRESETENABLER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == BODVBATRESETENABLER::ENABLE
    }
}
#[doc = "Possible values of the field `BODCORERESETENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODCORERESETENABLER {
    #[doc = "BOD CORE reset is disable."]
    DISABLE,
    #[doc = "BOD CORE reset is enable."]
    ENABLE,
}
impl BODCORERESETENABLER {
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
            BODCORERESETENABLER::DISABLE => false,
            BODCORERESETENABLER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODCORERESETENABLER {
        match value {
            false => BODCORERESETENABLER::DISABLE,
            true => BODCORERESETENABLER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == BODCORERESETENABLER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == BODCORERESETENABLER::ENABLE
    }
}
#[doc = "Possible values of the field `SWRRESETENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRRESETENABLER {
    #[doc = "Software reset is disable."]
    DISABLE,
    #[doc = "Software reset is enable."]
    ENABLE,
}
impl SWRRESETENABLER {
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
            SWRRESETENABLER::DISABLE => false,
            SWRRESETENABLER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWRRESETENABLER {
        match value {
            false => SWRRESETENABLER::DISABLE,
            true => SWRRESETENABLER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SWRRESETENABLER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SWRRESETENABLER::ENABLE
    }
}
#[doc = "Values that can be written to the field `DPDWAKEUPRESETENABLE`"]
pub enum DPDWAKEUPRESETENABLEW {
    #[doc = "Reset event from DEEP POWER DOWN mode is disable."]
    DISABLE,
    #[doc = "Reset event from DEEP POWER DOWN mode is enable."]
    ENABLE,
}
impl DPDWAKEUPRESETENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPDWAKEUPRESETENABLEW::DISABLE => false,
            DPDWAKEUPRESETENABLEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPDWAKEUPRESETENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _DPDWAKEUPRESETENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPDWAKEUPRESETENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset event from DEEP POWER DOWN mode is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DPDWAKEUPRESETENABLEW::DISABLE)
    }
    #[doc = "Reset event from DEEP POWER DOWN mode is enable."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(DPDWAKEUPRESETENABLEW::ENABLE)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BODVBATRESETENABLE`"]
pub enum BODVBATRESETENABLEW {
    #[doc = "BOD VBAT reset is disable."]
    DISABLE,
    #[doc = "BOD VBAT reset is enable."]
    ENABLE,
}
impl BODVBATRESETENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BODVBATRESETENABLEW::DISABLE => false,
            BODVBATRESETENABLEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BODVBATRESETENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _BODVBATRESETENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BODVBATRESETENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "BOD VBAT reset is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODVBATRESETENABLEW::DISABLE)
    }
    #[doc = "BOD VBAT reset is enable."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODVBATRESETENABLEW::ENABLE)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BODCORERESETENABLE`"]
pub enum BODCORERESETENABLEW {
    #[doc = "BOD CORE reset is disable."]
    DISABLE,
    #[doc = "BOD CORE reset is enable."]
    ENABLE,
}
impl BODCORERESETENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BODCORERESETENABLEW::DISABLE => false,
            BODCORERESETENABLEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BODCORERESETENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _BODCORERESETENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BODCORERESETENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "BOD CORE reset is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODCORERESETENABLEW::DISABLE)
    }
    #[doc = "BOD CORE reset is enable."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODCORERESETENABLEW::ENABLE)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SWRRESETENABLE`"]
pub enum SWRRESETENABLEW {
    #[doc = "Software reset is disable."]
    DISABLE,
    #[doc = "Software reset is enable."]
    ENABLE,
}
impl SWRRESETENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWRRESETENABLEW::DISABLE => false,
            SWRRESETENABLEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWRRESETENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRRESETENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWRRESETENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Software reset is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SWRRESETENABLEW::DISABLE)
    }
    #[doc = "Software reset is enable."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SWRRESETENABLEW::ENABLE)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
    #[inline]
    pub fn dpdwakeupresetenable(&self) -> DPDWAKEUPRESETENABLER {
        DPDWAKEUPRESETENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - BOD VBAT reset enable."]
    #[inline]
    pub fn bodvbatresetenable(&self) -> BODVBATRESETENABLER {
        BODVBATRESETENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - BOD CORE reset enable."]
    #[inline]
    pub fn bodcoreresetenable(&self) -> BODCORERESETENABLER {
        BODCORERESETENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Software reset enable."]
    #[inline]
    pub fn swrresetenable(&self) -> SWRRESETENABLER {
        SWRRESETENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
    #[inline]
    pub fn dpdwakeupresetenable(&mut self) -> _DPDWAKEUPRESETENABLEW {
        _DPDWAKEUPRESETENABLEW { w: self }
    }
    #[doc = "Bit 1 - BOD VBAT reset enable."]
    #[inline]
    pub fn bodvbatresetenable(&mut self) -> _BODVBATRESETENABLEW {
        _BODVBATRESETENABLEW { w: self }
    }
    #[doc = "Bit 2 - BOD CORE reset enable."]
    #[inline]
    pub fn bodcoreresetenable(&mut self) -> _BODCORERESETENABLEW {
        _BODCORERESETENABLEW { w: self }
    }
    #[doc = "Bit 3 - Software reset enable."]
    #[inline]
    pub fn swrresetenable(&mut self) -> _SWRRESETENABLEW {
        _SWRRESETENABLEW { w: self }
    }
}
