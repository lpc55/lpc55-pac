#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SAU_RLAR {
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
#[doc = "Possible values of the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLER {
    #[doc = "SAU region is enabled."]
    ENABLED,
    #[doc = "SAU region is disabled."]
    DISABLED,
}
impl ENABLER {
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
            ENABLER::ENABLED => false,
            ENABLER::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENABLER {
        match value {
            false => ENABLER::ENABLED,
            true => ENABLER::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLER::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLER::DISABLED
    }
}
#[doc = "Possible values of the field `NSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSCR {
    #[doc = "Region is not Non-secure callable."]
    NOT_NON_SECURE_CALLABLE,
    #[doc = "Region is Non-secure callable."]
    NON_SECURE_CALLABLE,
}
impl NSCR {
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
            NSCR::NOT_NON_SECURE_CALLABLE => false,
            NSCR::NON_SECURE_CALLABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NSCR {
        match value {
            false => NSCR::NOT_NON_SECURE_CALLABLE,
            true => NSCR::NON_SECURE_CALLABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_NON_SECURE_CALLABLE`"]
    #[inline]
    pub fn is_not_non_secure_callable(&self) -> bool {
        *self == NSCR::NOT_NON_SECURE_CALLABLE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE_CALLABLE`"]
    #[inline]
    pub fn is_non_secure_callable(&self) -> bool {
        *self == NSCR::NON_SECURE_CALLABLE
    }
}
#[doc = r" Value of the field"]
pub struct LADDRR {
    bits: u32,
}
impl LADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `ENABLE`"]
pub enum ENABLEW {
    #[doc = "SAU region is enabled."]
    ENABLED,
    #[doc = "SAU region is disabled."]
    DISABLED,
}
impl ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLEW::ENABLED => false,
            ENABLEW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SAU region is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLEW::ENABLED)
    }
    #[doc = "SAU region is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLEW::DISABLED)
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
#[doc = "Values that can be written to the field `NSC`"]
pub enum NSCW {
    #[doc = "Region is not Non-secure callable."]
    NOT_NON_SECURE_CALLABLE,
    #[doc = "Region is Non-secure callable."]
    NON_SECURE_CALLABLE,
}
impl NSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NSCW::NOT_NON_SECURE_CALLABLE => false,
            NSCW::NON_SECURE_CALLABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSCW<'a> {
    w: &'a mut W,
}
impl<'a> _NSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Region is not Non-secure callable."]
    #[inline]
    pub fn not_non_secure_callable(self) -> &'a mut W {
        self.variant(NSCW::NOT_NON_SECURE_CALLABLE)
    }
    #[doc = "Region is Non-secure callable."]
    #[inline]
    pub fn non_secure_callable(self) -> &'a mut W {
        self.variant(NSCW::NON_SECURE_CALLABLE)
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
#[doc = r" Proxy"]
pub struct _LADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _LADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 134217727;
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
    #[doc = "Bit 0 - Enable. SAU region enable."]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region."]
    #[inline]
    pub fn nsc(&self) -> NSCR {
        NSCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:31 - Limit address. Holds bits\\[31:5\\] of the limit address for the selected SAU region. Bits\\[4:0\\] of the limit address are defined as 0x1F."]
    #[inline]
    pub fn laddr(&self) -> LADDRR {
        let bits = {
            const MASK: u32 = 134217727;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        LADDRR { bits }
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
    #[doc = "Bit 0 - Enable. SAU region enable."]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 1 - Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region."]
    #[inline]
    pub fn nsc(&mut self) -> _NSCW {
        _NSCW { w: self }
    }
    #[doc = "Bits 5:31 - Limit address. Holds bits\\[31:5\\] of the limit address for the selected SAU region. Bits\\[4:0\\] of the limit address are defined as 0x1F."]
    #[inline]
    pub fn laddr(&mut self) -> _LADDRW {
        _LADDRW { w: self }
    }
}
