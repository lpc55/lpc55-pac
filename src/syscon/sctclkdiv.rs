#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCTCLKDIV {
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
#[doc = r" Value of the field"]
pub struct DIVR {
    bits: u8,
}
impl DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `HALT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALTR {
    #[doc = "Divider clock is running."]
    RUN,
    #[doc = "Divider clock is stoped."]
    HALT,
}
impl HALTR {
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
            HALTR::RUN => false,
            HALTR::HALT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HALTR {
        match value {
            false => HALTR::RUN,
            true => HALTR::HALT,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == HALTR::RUN
    }
    #[doc = "Checks if the value of the field is `HALT`"]
    #[inline]
    pub fn is_halt(&self) -> bool {
        *self == HALTR::HALT
    }
}
#[doc = "Possible values of the field `REQFLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQFLAGR {
    #[doc = "Divider clock is stable."]
    STABLE,
    #[doc = "Clock frequency is not stable."]
    ONGOING,
}
impl REQFLAGR {
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
            REQFLAGR::STABLE => false,
            REQFLAGR::ONGOING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REQFLAGR {
        match value {
            false => REQFLAGR::STABLE,
            true => REQFLAGR::ONGOING,
        }
    }
    #[doc = "Checks if the value of the field is `STABLE`"]
    #[inline]
    pub fn is_stable(&self) -> bool {
        *self == REQFLAGR::STABLE
    }
    #[doc = "Checks if the value of the field is `ONGOING`"]
    #[inline]
    pub fn is_ongoing(&self) -> bool {
        *self == REQFLAGR::ONGOING
    }
}
#[doc = r" Proxy"]
pub struct _DIVW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RESET`"]
pub enum RESETW {
    #[doc = "Divider is not reset."]
    RELEASED,
    #[doc = "Divider is reset."]
    ASSERTED,
}
impl RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESETW::RELEASED => false,
            RESETW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Divider is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(RESETW::RELEASED)
    }
    #[doc = "Divider is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(RESETW::ASSERTED)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HALT`"]
pub enum HALTW {
    #[doc = "Divider clock is running."]
    RUN,
    #[doc = "Divider clock is stoped."]
    HALT,
}
impl HALTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HALTW::RUN => false,
            HALTW::HALT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HALTW<'a> {
    w: &'a mut W,
}
impl<'a> _HALTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HALTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Divider clock is running."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(HALTW::RUN)
    }
    #[doc = "Divider clock is stoped."]
    #[inline]
    pub fn halt(self) -> &'a mut W {
        self.variant(HALTW::HALT)
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
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:7 - Clock divider value."]
    #[inline]
    pub fn div(&self) -> DIVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIVR { bits }
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline]
    pub fn halt(&self) -> HALTR {
        HALTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Divider status flag."]
    #[inline]
    pub fn reqflag(&self) -> REQFLAGR {
        REQFLAGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1073741824 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Clock divider value."]
    #[inline]
    pub fn div(&mut self) -> _DIVW {
        _DIVW { w: self }
    }
    #[doc = "Bit 29 - Resets the divider counter."]
    #[inline]
    pub fn reset(&mut self) -> _RESETW {
        _RESETW { w: self }
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline]
    pub fn halt(&mut self) -> _HALTW {
        _HALTW { w: self }
    }
}
