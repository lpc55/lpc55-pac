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
#[doc = "Possible values of the field `WAITING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITINGR {
    #[doc = "Will not interrupt when waiting."]
    NO_INTERRUPT,
    #[doc = "Will interrupt when waiting"]
    INTERRUPT,
}
impl WAITINGR {
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
            WAITINGR::NO_INTERRUPT => false,
            WAITINGR::INTERRUPT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAITINGR {
        match value {
            false => WAITINGR::NO_INTERRUPT,
            true => WAITINGR::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline]
    pub fn is_no_interrupt(&self) -> bool {
        *self == WAITINGR::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        *self == WAITINGR::INTERRUPT
    }
}
#[doc = "Possible values of the field `DIGEST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIGESTR {
    #[doc = "Will not interrupt when Digest is ready"]
    NO_INTERRUPT,
    #[doc = "Will interrupt when Digest is ready. Interrupt cleared by writing more data, starting a new Hash, or disabling (done)."]
    INTERRUPT,
}
impl DIGESTR {
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
            DIGESTR::NO_INTERRUPT => false,
            DIGESTR::INTERRUPT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIGESTR {
        match value {
            false => DIGESTR::NO_INTERRUPT,
            true => DIGESTR::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline]
    pub fn is_no_interrupt(&self) -> bool {
        *self == DIGESTR::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        *self == DIGESTR::INTERRUPT
    }
}
#[doc = "Possible values of the field `ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORR {
    #[doc = "Will not interrupt on Error."]
    NOT_INTERRUPT,
    #[doc = "Will interrupt on Error (until cleared)."]
    INTERRUPT,
}
impl ERRORR {
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
            ERRORR::NOT_INTERRUPT => false,
            ERRORR::INTERRUPT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRORR {
        match value {
            false => ERRORR::NOT_INTERRUPT,
            true => ERRORR::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INTERRUPT`"]
    #[inline]
    pub fn is_not_interrupt(&self) -> bool {
        *self == ERRORR::NOT_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        *self == ERRORR::INTERRUPT
    }
}
#[doc = "Values that can be written to the field `WAITING`"]
pub enum WAITINGW {
    #[doc = "Will not interrupt when waiting."]
    NO_INTERRUPT,
    #[doc = "Will interrupt when waiting"]
    INTERRUPT,
}
impl WAITINGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAITINGW::NO_INTERRUPT => false,
            WAITINGW::INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAITINGW<'a> {
    w: &'a mut W,
}
impl<'a> _WAITINGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAITINGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Will not interrupt when waiting."]
    #[inline]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(WAITINGW::NO_INTERRUPT)
    }
    #[doc = "Will interrupt when waiting"]
    #[inline]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(WAITINGW::INTERRUPT)
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
#[doc = "Values that can be written to the field `DIGEST`"]
pub enum DIGESTW {
    #[doc = "Will not interrupt when Digest is ready"]
    NO_INTERRUPT,
    #[doc = "Will interrupt when Digest is ready. Interrupt cleared by writing more data, starting a new Hash, or disabling (done)."]
    INTERRUPT,
}
impl DIGESTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIGESTW::NO_INTERRUPT => false,
            DIGESTW::INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIGESTW<'a> {
    w: &'a mut W,
}
impl<'a> _DIGESTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIGESTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Will not interrupt when Digest is ready"]
    #[inline]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(DIGESTW::NO_INTERRUPT)
    }
    #[doc = "Will interrupt when Digest is ready. Interrupt cleared by writing more data, starting a new Hash, or disabling (done)."]
    #[inline]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(DIGESTW::INTERRUPT)
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
#[doc = "Values that can be written to the field `ERROR`"]
pub enum ERRORW {
    #[doc = "Will not interrupt on Error."]
    NOT_INTERRUPT,
    #[doc = "Will interrupt on Error (until cleared)."]
    INTERRUPT,
}
impl ERRORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRORW::NOT_INTERRUPT => false,
            ERRORW::INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERRORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Will not interrupt on Error."]
    #[inline]
    pub fn not_interrupt(self) -> &'a mut W {
        self.variant(ERRORW::NOT_INTERRUPT)
    }
    #[doc = "Will interrupt on Error (until cleared)."]
    #[inline]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(ERRORW::INTERRUPT)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Indicates if should interrupt when waiting for data input."]
    #[inline]
    pub fn waiting(&self) -> WAITINGR {
        WAITINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Indicates if should interrupt when Digest (or Outdata) is ready (completed a hash/crypto or completed a full sequence)."]
    #[inline]
    pub fn digest(&self) -> DIGESTR {
        DIGESTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Indicates if should interrupt on an ERROR (as defined in Status)"]
    #[inline]
    pub fn error(&self) -> ERRORR {
        ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Indicates if should interrupt when waiting for data input."]
    #[inline]
    pub fn waiting(&mut self) -> _WAITINGW {
        _WAITINGW { w: self }
    }
    #[doc = "Bit 1 - Indicates if should interrupt when Digest (or Outdata) is ready (completed a hash/crypto or completed a full sequence)."]
    #[inline]
    pub fn digest(&mut self) -> _DIGESTW {
        _DIGESTW { w: self }
    }
    #[doc = "Bit 2 - Indicates if should interrupt on an ERROR (as defined in Status)"]
    #[inline]
    pub fn error(&mut self) -> _ERRORW {
        _ERRORW { w: self }
    }
}
