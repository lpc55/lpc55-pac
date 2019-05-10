#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::COMP_INT_CTRL {
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
#[doc = "Possible values of the field `INT_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_ENABLER {
    #[doc = "interrupt disable."]
    INT_DISABLE,
    #[doc = "interrupt enable."]
    INT_ENABLE,
}
impl INT_ENABLER {
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
            INT_ENABLER::INT_DISABLE => false,
            INT_ENABLER::INT_ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT_ENABLER {
        match value {
            false => INT_ENABLER::INT_DISABLE,
            true => INT_ENABLER::INT_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `INT_DISABLE`"]
    #[inline]
    pub fn is_int_disable(&self) -> bool {
        *self == INT_ENABLER::INT_DISABLE
    }
    #[doc = "Checks if the value of the field is `INT_ENABLE`"]
    #[inline]
    pub fn is_int_enable(&self) -> bool {
        *self == INT_ENABLER::INT_ENABLE
    }
}
#[doc = "Possible values of the field `INT_CLEAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_CLEARR {
    #[doc = "No effect."]
    NONE,
    #[doc = "Clear the interrupt. Self-cleared bit."]
    CLEAR,
}
impl INT_CLEARR {
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
            INT_CLEARR::NONE => false,
            INT_CLEARR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT_CLEARR {
        match value {
            false => INT_CLEARR::NONE,
            true => INT_CLEARR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == INT_CLEARR::NONE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == INT_CLEARR::CLEAR
    }
}
#[doc = "Possible values of the field `INT_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_CTRLR {
    #[doc = "The analog comparator interrupt edge sensitive is disabled."]
    EDGE_DISABLE,
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    LVL_DISABLE,
    #[doc = "analog comparator interrupt is rising edge sensitive."]
    EDGE_RISING,
    #[doc = "Analog Comparator interrupt is high level sensitive."]
    LVL_HIGH,
    #[doc = "analog comparator interrupt is falling edge sensitive."]
    EDGE_FALLING,
    #[doc = "Analog Comparator interrupt is low level sensitive."]
    LVL_LOW,
    #[doc = "analog comparator interrupt is rising and falling edge sensitive."]
    EDGE_BOTH,
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    LVL_DIS2,
}
impl INT_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INT_CTRLR::EDGE_DISABLE => 0,
            INT_CTRLR::LVL_DISABLE => 1,
            INT_CTRLR::EDGE_RISING => 2,
            INT_CTRLR::LVL_HIGH => 3,
            INT_CTRLR::EDGE_FALLING => 4,
            INT_CTRLR::LVL_LOW => 5,
            INT_CTRLR::EDGE_BOTH => 6,
            INT_CTRLR::LVL_DIS2 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INT_CTRLR {
        match value {
            0 => INT_CTRLR::EDGE_DISABLE,
            1 => INT_CTRLR::LVL_DISABLE,
            2 => INT_CTRLR::EDGE_RISING,
            3 => INT_CTRLR::LVL_HIGH,
            4 => INT_CTRLR::EDGE_FALLING,
            5 => INT_CTRLR::LVL_LOW,
            6 => INT_CTRLR::EDGE_BOTH,
            7 => INT_CTRLR::LVL_DIS2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGE_DISABLE`"]
    #[inline]
    pub fn is_edge_disable(&self) -> bool {
        *self == INT_CTRLR::EDGE_DISABLE
    }
    #[doc = "Checks if the value of the field is `LVL_DISABLE`"]
    #[inline]
    pub fn is_lvl_disable(&self) -> bool {
        *self == INT_CTRLR::LVL_DISABLE
    }
    #[doc = "Checks if the value of the field is `EDGE_RISING`"]
    #[inline]
    pub fn is_edge_rising(&self) -> bool {
        *self == INT_CTRLR::EDGE_RISING
    }
    #[doc = "Checks if the value of the field is `LVL_HIGH`"]
    #[inline]
    pub fn is_lvl_high(&self) -> bool {
        *self == INT_CTRLR::LVL_HIGH
    }
    #[doc = "Checks if the value of the field is `EDGE_FALLING`"]
    #[inline]
    pub fn is_edge_falling(&self) -> bool {
        *self == INT_CTRLR::EDGE_FALLING
    }
    #[doc = "Checks if the value of the field is `LVL_LOW`"]
    #[inline]
    pub fn is_lvl_low(&self) -> bool {
        *self == INT_CTRLR::LVL_LOW
    }
    #[doc = "Checks if the value of the field is `EDGE_BOTH`"]
    #[inline]
    pub fn is_edge_both(&self) -> bool {
        *self == INT_CTRLR::EDGE_BOTH
    }
    #[doc = "Checks if the value of the field is `LVL_DIS2`"]
    #[inline]
    pub fn is_lvl_dis2(&self) -> bool {
        *self == INT_CTRLR::LVL_DIS2
    }
}
#[doc = "Possible values of the field `INT_SOURCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_SOURCER {
    #[doc = "Select Analog Comparator filtered output as input for interrupt detection."]
    FILTER_INT,
    #[doc = "Select Analog Comparator raw output (unfiltered) as input for interrupt detection. Must be used when Analog comparator is used as wake up source in Power down mode."]
    RAW_INT,
}
impl INT_SOURCER {
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
            INT_SOURCER::FILTER_INT => false,
            INT_SOURCER::RAW_INT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT_SOURCER {
        match value {
            false => INT_SOURCER::FILTER_INT,
            true => INT_SOURCER::RAW_INT,
        }
    }
    #[doc = "Checks if the value of the field is `FILTER_INT`"]
    #[inline]
    pub fn is_filter_int(&self) -> bool {
        *self == INT_SOURCER::FILTER_INT
    }
    #[doc = "Checks if the value of the field is `RAW_INT`"]
    #[inline]
    pub fn is_raw_int(&self) -> bool {
        *self == INT_SOURCER::RAW_INT
    }
}
#[doc = "Values that can be written to the field `INT_ENABLE`"]
pub enum INT_ENABLEW {
    #[doc = "interrupt disable."]
    INT_DISABLE,
    #[doc = "interrupt enable."]
    INT_ENABLE,
}
impl INT_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT_ENABLEW::INT_DISABLE => false,
            INT_ENABLEW::INT_ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _INT_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT_ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "interrupt disable."]
    #[inline]
    pub fn int_disable(self) -> &'a mut W {
        self.variant(INT_ENABLEW::INT_DISABLE)
    }
    #[doc = "interrupt enable."]
    #[inline]
    pub fn int_enable(self) -> &'a mut W {
        self.variant(INT_ENABLEW::INT_ENABLE)
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
#[doc = "Values that can be written to the field `INT_CLEAR`"]
pub enum INT_CLEARW {
    #[doc = "No effect."]
    NONE,
    #[doc = "Clear the interrupt. Self-cleared bit."]
    CLEAR,
}
impl INT_CLEARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT_CLEARW::NONE => false,
            INT_CLEARW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT_CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _INT_CLEARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT_CLEARW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(INT_CLEARW::NONE)
    }
    #[doc = "Clear the interrupt. Self-cleared bit."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(INT_CLEARW::CLEAR)
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
#[doc = "Values that can be written to the field `INT_CTRL`"]
pub enum INT_CTRLW {
    #[doc = "The analog comparator interrupt edge sensitive is disabled."]
    EDGE_DISABLE,
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    LVL_DISABLE,
    #[doc = "analog comparator interrupt is rising edge sensitive."]
    EDGE_RISING,
    #[doc = "Analog Comparator interrupt is high level sensitive."]
    LVL_HIGH,
    #[doc = "analog comparator interrupt is falling edge sensitive."]
    EDGE_FALLING,
    #[doc = "Analog Comparator interrupt is low level sensitive."]
    LVL_LOW,
    #[doc = "analog comparator interrupt is rising and falling edge sensitive."]
    EDGE_BOTH,
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    LVL_DIS2,
}
impl INT_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INT_CTRLW::EDGE_DISABLE => 0,
            INT_CTRLW::LVL_DISABLE => 1,
            INT_CTRLW::EDGE_RISING => 2,
            INT_CTRLW::LVL_HIGH => 3,
            INT_CTRLW::EDGE_FALLING => 4,
            INT_CTRLW::LVL_LOW => 5,
            INT_CTRLW::EDGE_BOTH => 6,
            INT_CTRLW::LVL_DIS2 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _INT_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT_CTRLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The analog comparator interrupt edge sensitive is disabled."]
    #[inline]
    pub fn edge_disable(self) -> &'a mut W {
        self.variant(INT_CTRLW::EDGE_DISABLE)
    }
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    #[inline]
    pub fn lvl_disable(self) -> &'a mut W {
        self.variant(INT_CTRLW::LVL_DISABLE)
    }
    #[doc = "analog comparator interrupt is rising edge sensitive."]
    #[inline]
    pub fn edge_rising(self) -> &'a mut W {
        self.variant(INT_CTRLW::EDGE_RISING)
    }
    #[doc = "Analog Comparator interrupt is high level sensitive."]
    #[inline]
    pub fn lvl_high(self) -> &'a mut W {
        self.variant(INT_CTRLW::LVL_HIGH)
    }
    #[doc = "analog comparator interrupt is falling edge sensitive."]
    #[inline]
    pub fn edge_falling(self) -> &'a mut W {
        self.variant(INT_CTRLW::EDGE_FALLING)
    }
    #[doc = "Analog Comparator interrupt is low level sensitive."]
    #[inline]
    pub fn lvl_low(self) -> &'a mut W {
        self.variant(INT_CTRLW::LVL_LOW)
    }
    #[doc = "analog comparator interrupt is rising and falling edge sensitive."]
    #[inline]
    pub fn edge_both(self) -> &'a mut W {
        self.variant(INT_CTRLW::EDGE_BOTH)
    }
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    #[inline]
    pub fn lvl_dis2(self) -> &'a mut W {
        self.variant(INT_CTRLW::LVL_DIS2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INT_SOURCE`"]
pub enum INT_SOURCEW {
    #[doc = "Select Analog Comparator filtered output as input for interrupt detection."]
    FILTER_INT,
    #[doc = "Select Analog Comparator raw output (unfiltered) as input for interrupt detection. Must be used when Analog comparator is used as wake up source in Power down mode."]
    RAW_INT,
}
impl INT_SOURCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT_SOURCEW::FILTER_INT => false,
            INT_SOURCEW::RAW_INT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT_SOURCEW<'a> {
    w: &'a mut W,
}
impl<'a> _INT_SOURCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT_SOURCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Select Analog Comparator filtered output as input for interrupt detection."]
    #[inline]
    pub fn filter_int(self) -> &'a mut W {
        self.variant(INT_SOURCEW::FILTER_INT)
    }
    #[doc = "Select Analog Comparator raw output (unfiltered) as input for interrupt detection. Must be used when Analog comparator is used as wake up source in Power down mode."]
    #[inline]
    pub fn raw_int(self) -> &'a mut W {
        self.variant(INT_SOURCEW::RAW_INT)
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
    #[doc = "Bit 0 - Analog Comparator interrupt enable control:."]
    #[inline]
    pub fn int_enable(&self) -> INT_ENABLER {
        INT_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Analog Comparator interrupt clear."]
    #[inline]
    pub fn int_clear(&self) -> INT_CLEARR {
        INT_CLEARR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:4 - Comparator interrupt type selector:."]
    #[inline]
    pub fn int_ctrl(&self) -> INT_CTRLR {
        INT_CTRLR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Select which Analog comparator output (filtered our un-filtered) is used for interrupt detection."]
    #[inline]
    pub fn int_source(&self) -> INT_SOURCER {
        INT_SOURCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - Analog Comparator interrupt enable control:."]
    #[inline]
    pub fn int_enable(&mut self) -> _INT_ENABLEW {
        _INT_ENABLEW { w: self }
    }
    #[doc = "Bit 1 - Analog Comparator interrupt clear."]
    #[inline]
    pub fn int_clear(&mut self) -> _INT_CLEARW {
        _INT_CLEARW { w: self }
    }
    #[doc = "Bits 2:4 - Comparator interrupt type selector:."]
    #[inline]
    pub fn int_ctrl(&mut self) -> _INT_CTRLW {
        _INT_CTRLW { w: self }
    }
    #[doc = "Bit 5 - Select which Analog comparator output (filtered our un-filtered) is used for interrupt detection."]
    #[inline]
    pub fn int_source(&mut self) -> _INT_SOURCEW {
        _INT_SOURCEW { w: self }
    }
}
