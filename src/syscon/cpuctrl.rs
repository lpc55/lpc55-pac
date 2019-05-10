#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CPUCTRL {
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
#[doc = "Possible values of the field `CPU1CLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1CLKENR {
    #[doc = "The CPU1 clock is not enabled."]
    DISABLE,
    #[doc = "The CPU1 clock is enabled."]
    ENABLE,
}
impl CPU1CLKENR {
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
            CPU1CLKENR::DISABLE => false,
            CPU1CLKENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPU1CLKENR {
        match value {
            false => CPU1CLKENR::DISABLE,
            true => CPU1CLKENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CPU1CLKENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CPU1CLKENR::ENABLE
    }
}
#[doc = "Possible values of the field `CPU1RSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1RSTENR {
    #[doc = "The CPU1 is not being reset."]
    RELEASED,
    #[doc = "The CPU1 is being reset."]
    ASSERTED,
}
impl CPU1RSTENR {
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
            CPU1RSTENR::RELEASED => false,
            CPU1RSTENR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPU1RSTENR {
        match value {
            false => CPU1RSTENR::RELEASED,
            true => CPU1RSTENR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == CPU1RSTENR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == CPU1RSTENR::ASSERTED
    }
}
#[doc = "Values that can be written to the field `CPU1CLKEN`"]
pub enum CPU1CLKENW {
    #[doc = "The CPU1 clock is not enabled."]
    DISABLE,
    #[doc = "The CPU1 clock is enabled."]
    ENABLE,
}
impl CPU1CLKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPU1CLKENW::DISABLE => false,
            CPU1CLKENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPU1CLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _CPU1CLKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPU1CLKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The CPU1 clock is not enabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CPU1CLKENW::DISABLE)
    }
    #[doc = "The CPU1 clock is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU1CLKENW::ENABLE)
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
#[doc = "Values that can be written to the field `CPU1RSTEN`"]
pub enum CPU1RSTENW {
    #[doc = "The CPU1 is not being reset."]
    RELEASED,
    #[doc = "The CPU1 is being reset."]
    ASSERTED,
}
impl CPU1RSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPU1RSTENW::RELEASED => false,
            CPU1RSTENW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPU1RSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _CPU1RSTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPU1RSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The CPU1 is not being reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(CPU1RSTENW::RELEASED)
    }
    #[doc = "The CPU1 is being reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(CPU1RSTENW::ASSERTED)
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
    #[doc = "Bit 3 - CPU1 clock enable."]
    #[inline]
    pub fn cpu1clken(&self) -> CPU1CLKENR {
        CPU1CLKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - CPU1 reset."]
    #[inline]
    pub fn cpu1rsten(&self) -> CPU1RSTENR {
        CPU1RSTENR::_from({
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
        W { bits: 44 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 3 - CPU1 clock enable."]
    #[inline]
    pub fn cpu1clken(&mut self) -> _CPU1CLKENW {
        _CPU1CLKENW { w: self }
    }
    #[doc = "Bit 5 - CPU1 reset."]
    #[inline]
    pub fn cpu1rsten(&mut self) -> _CPU1RSTENW {
        _CPU1RSTENW { w: self }
    }
}
