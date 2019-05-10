#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENSET {
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
#[doc = "Possible values of the field `SSAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSAENR {
    #[doc = "Disabled. No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    DISABLED,
    #[doc = "Enabled. An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    ENABLED,
}
impl SSAENR {
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
            SSAENR::DISABLED => false,
            SSAENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSAENR {
        match value {
            false => SSAENR::DISABLED,
            true => SSAENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SSAENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SSAENR::ENABLED
    }
}
#[doc = "Possible values of the field `SSDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSDENR {
    #[doc = "Disabled. No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    DISABLED,
    #[doc = "Enabled. An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    ENABLED,
}
impl SSDENR {
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
            SSDENR::DISABLED => false,
            SSDENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSDENR {
        match value {
            false => SSDENR::DISABLED,
            true => SSDENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SSDENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SSDENR::ENABLED
    }
}
#[doc = "Possible values of the field `MSTIDLEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTIDLEENR {
    #[doc = "No interrupt will be generated when the SPI master function is idle."]
    DISABLED,
    #[doc = "An interrupt will be generated when the SPI master function is fully idle."]
    ENABLED,
}
impl MSTIDLEENR {
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
            MSTIDLEENR::DISABLED => false,
            MSTIDLEENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTIDLEENR {
        match value {
            false => MSTIDLEENR::DISABLED,
            true => MSTIDLEENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MSTIDLEENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == MSTIDLEENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SSAEN`"]
pub enum SSAENW {
    #[doc = "Disabled. No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    DISABLED,
    #[doc = "Enabled. An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    ENABLED,
}
impl SSAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSAENW::DISABLED => false,
            SSAENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSAENW<'a> {
    w: &'a mut W,
}
impl<'a> _SSAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSAENW::DISABLED)
    }
    #[doc = "Enabled. An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSAENW::ENABLED)
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
#[doc = "Values that can be written to the field `SSDEN`"]
pub enum SSDENW {
    #[doc = "Disabled. No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    DISABLED,
    #[doc = "Enabled. An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    ENABLED,
}
impl SSDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSDENW::DISABLED => false,
            SSDENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSDENW<'a> {
    w: &'a mut W,
}
impl<'a> _SSDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSDENW::DISABLED)
    }
    #[doc = "Enabled. An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSDENW::ENABLED)
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
#[doc = "Values that can be written to the field `MSTIDLEEN`"]
pub enum MSTIDLEENW {
    #[doc = "No interrupt will be generated when the SPI master function is idle."]
    DISABLED,
    #[doc = "An interrupt will be generated when the SPI master function is fully idle."]
    ENABLED,
}
impl MSTIDLEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTIDLEENW::DISABLED => false,
            MSTIDLEENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTIDLEENW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTIDLEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTIDLEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated when the SPI master function is idle."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTIDLEENW::DISABLED)
    }
    #[doc = "An interrupt will be generated when the SPI master function is fully idle."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTIDLEENW::ENABLED)
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
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 4 - Slave select assert interrupt enable. Determines whether an interrupt occurs when the Slave Select is asserted."]
    #[inline]
    pub fn ssaen(&self) -> SSAENR {
        SSAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Slave select deassert interrupt enable. Determines whether an interrupt occurs when the Slave Select is deasserted."]
    #[inline]
    pub fn ssden(&self) -> SSDENR {
        SSDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Master idle interrupt enable."]
    #[inline]
    pub fn mstidleen(&self) -> MSTIDLEENR {
        MSTIDLEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
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
    #[doc = "Bit 4 - Slave select assert interrupt enable. Determines whether an interrupt occurs when the Slave Select is asserted."]
    #[inline]
    pub fn ssaen(&mut self) -> _SSAENW {
        _SSAENW { w: self }
    }
    #[doc = "Bit 5 - Slave select deassert interrupt enable. Determines whether an interrupt occurs when the Slave Select is deasserted."]
    #[inline]
    pub fn ssden(&mut self) -> _SSDENW {
        _SSDENW { w: self }
    }
    #[doc = "Bit 8 - Master idle interrupt enable."]
    #[inline]
    pub fn mstidleen(&mut self) -> _MSTIDLEENW {
        _MSTIDLEENW { w: self }
    }
}
