#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BOD_DCDC_INT_CTRL {
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
#[doc = "Possible values of the field `BODVBAT_INT_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODVBAT_INT_ENABLER {
    #[doc = "BOD VBAT interrupt is disabled."]
    DISABLE,
    #[doc = "BOD VBAT interrupt is enabled."]
    ENABLE,
}
impl BODVBAT_INT_ENABLER {
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
            BODVBAT_INT_ENABLER::DISABLE => false,
            BODVBAT_INT_ENABLER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODVBAT_INT_ENABLER {
        match value {
            false => BODVBAT_INT_ENABLER::DISABLE,
            true => BODVBAT_INT_ENABLER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == BODVBAT_INT_ENABLER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == BODVBAT_INT_ENABLER::ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct BODVBAT_INT_CLEARR {
    bits: bool,
}
impl BODVBAT_INT_CLEARR {
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
#[doc = "Possible values of the field `BODCORE_INT_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODCORE_INT_ENABLER {
    #[doc = "BOD CORE interrupt is disabled."]
    DISABLE,
    #[doc = "BOD CORE interrupt is enabled."]
    ENABLE,
}
impl BODCORE_INT_ENABLER {
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
            BODCORE_INT_ENABLER::DISABLE => false,
            BODCORE_INT_ENABLER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODCORE_INT_ENABLER {
        match value {
            false => BODCORE_INT_ENABLER::DISABLE,
            true => BODCORE_INT_ENABLER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == BODCORE_INT_ENABLER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == BODCORE_INT_ENABLER::ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct BODCORE_INT_CLEARR {
    bits: bool,
}
impl BODCORE_INT_CLEARR {
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
#[doc = "Possible values of the field `DCDC_INT_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_INT_ENABLER {
    #[doc = "DCDC interrupt is disabled."]
    DISABLE,
    #[doc = "DCDC interrupt is enabled."]
    ENABLE,
}
impl DCDC_INT_ENABLER {
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
            DCDC_INT_ENABLER::DISABLE => false,
            DCDC_INT_ENABLER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCDC_INT_ENABLER {
        match value {
            false => DCDC_INT_ENABLER::DISABLE,
            true => DCDC_INT_ENABLER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DCDC_INT_ENABLER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == DCDC_INT_ENABLER::ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct DCDC_INT_CLEARR {
    bits: bool,
}
impl DCDC_INT_CLEARR {
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
#[doc = "Values that can be written to the field `BODVBAT_INT_ENABLE`"]
pub enum BODVBAT_INT_ENABLEW {
    #[doc = "BOD VBAT interrupt is disabled."]
    DISABLE,
    #[doc = "BOD VBAT interrupt is enabled."]
    ENABLE,
}
impl BODVBAT_INT_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BODVBAT_INT_ENABLEW::DISABLE => false,
            BODVBAT_INT_ENABLEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BODVBAT_INT_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _BODVBAT_INT_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BODVBAT_INT_ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "BOD VBAT interrupt is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODVBAT_INT_ENABLEW::DISABLE)
    }
    #[doc = "BOD VBAT interrupt is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODVBAT_INT_ENABLEW::ENABLE)
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
#[doc = r" Proxy"]
pub struct _BODVBAT_INT_CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _BODVBAT_INT_CLEARW<'a> {
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
#[doc = "Values that can be written to the field `BODCORE_INT_ENABLE`"]
pub enum BODCORE_INT_ENABLEW {
    #[doc = "BOD CORE interrupt is disabled."]
    DISABLE,
    #[doc = "BOD CORE interrupt is enabled."]
    ENABLE,
}
impl BODCORE_INT_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BODCORE_INT_ENABLEW::DISABLE => false,
            BODCORE_INT_ENABLEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BODCORE_INT_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _BODCORE_INT_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BODCORE_INT_ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "BOD CORE interrupt is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODCORE_INT_ENABLEW::DISABLE)
    }
    #[doc = "BOD CORE interrupt is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODCORE_INT_ENABLEW::ENABLE)
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
#[doc = r" Proxy"]
pub struct _BODCORE_INT_CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _BODCORE_INT_CLEARW<'a> {
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
#[doc = "Values that can be written to the field `DCDC_INT_ENABLE`"]
pub enum DCDC_INT_ENABLEW {
    #[doc = "DCDC interrupt is disabled."]
    DISABLE,
    #[doc = "DCDC interrupt is enabled."]
    ENABLE,
}
impl DCDC_INT_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCDC_INT_ENABLEW::DISABLE => false,
            DCDC_INT_ENABLEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCDC_INT_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_INT_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCDC_INT_ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DCDC interrupt is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DCDC_INT_ENABLEW::DISABLE)
    }
    #[doc = "DCDC interrupt is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(DCDC_INT_ENABLEW::ENABLE)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCDC_INT_CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_INT_CLEARW<'a> {
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
        const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - BOD VBAT interrupt control."]
    #[inline]
    pub fn bodvbat_int_enable(&self) -> BODVBAT_INT_ENABLER {
        BODVBAT_INT_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - BOD VBAT interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline]
    pub fn bodvbat_int_clear(&self) -> BODVBAT_INT_CLEARR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BODVBAT_INT_CLEARR { bits }
    }
    #[doc = "Bit 2 - BOD CORE interrupt control."]
    #[inline]
    pub fn bodcore_int_enable(&self) -> BODCORE_INT_ENABLER {
        BODCORE_INT_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - BOD CORE interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline]
    pub fn bodcore_int_clear(&self) -> BODCORE_INT_CLEARR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BODCORE_INT_CLEARR { bits }
    }
    #[doc = "Bit 4 - DCDC interrupt control."]
    #[inline]
    pub fn dcdc_int_enable(&self) -> DCDC_INT_ENABLER {
        DCDC_INT_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - DCDC interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline]
    pub fn dcdc_int_clear(&self) -> DCDC_INT_CLEARR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_INT_CLEARR { bits }
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
    #[doc = "Bit 0 - BOD VBAT interrupt control."]
    #[inline]
    pub fn bodvbat_int_enable(&mut self) -> _BODVBAT_INT_ENABLEW {
        _BODVBAT_INT_ENABLEW { w: self }
    }
    #[doc = "Bit 1 - BOD VBAT interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline]
    pub fn bodvbat_int_clear(&mut self) -> _BODVBAT_INT_CLEARW {
        _BODVBAT_INT_CLEARW { w: self }
    }
    #[doc = "Bit 2 - BOD CORE interrupt control."]
    #[inline]
    pub fn bodcore_int_enable(&mut self) -> _BODCORE_INT_ENABLEW {
        _BODCORE_INT_ENABLEW { w: self }
    }
    #[doc = "Bit 3 - BOD CORE interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline]
    pub fn bodcore_int_clear(&mut self) -> _BODCORE_INT_CLEARW {
        _BODCORE_INT_CLEARW { w: self }
    }
    #[doc = "Bit 4 - DCDC interrupt control."]
    #[inline]
    pub fn dcdc_int_enable(&mut self) -> _DCDC_INT_ENABLEW {
        _DCDC_INT_ENABLEW { w: self }
    }
    #[doc = "Bit 5 - DCDC interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline]
    pub fn dcdc_int_clear(&mut self) -> _DCDC_INT_CLEARW {
        _DCDC_INT_CLEARW { w: self }
    }
}
