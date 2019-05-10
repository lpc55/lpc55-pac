#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LOCK {
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
#[doc = "Possible values of the field `LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKR {
    #[doc = "unlock"]
    UNLOCK,
    #[doc = "Lock to current security level"]
    LOCK,
}
impl LOCKR {
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
            LOCKR::UNLOCK => false,
            LOCKR::LOCK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKR {
        match value {
            false => LOCKR::UNLOCK,
            true => LOCKR::LOCK,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCK`"]
    #[inline]
    pub fn is_unlock(&self) -> bool {
        *self == LOCKR::UNLOCK
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline]
    pub fn is_lock(&self) -> bool {
        *self == LOCKR::LOCK
    }
}
#[doc = "Possible values of the field `KEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYR {
    #[doc = "If set during write, will allow lock or unlock"]
    KWY_VALUE,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl KEYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            KEYR::KWY_VALUE => 1853,
            KEYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> KEYR {
        match value {
            1853 => KEYR::KWY_VALUE,
            i => KEYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `KWY_VALUE`"]
    #[inline]
    pub fn is_kwy_value(&self) -> bool {
        *self == KEYR::KWY_VALUE
    }
}
#[doc = "Values that can be written to the field `LOCK`"]
pub enum LOCKW {
    #[doc = "unlock"]
    UNLOCK,
    #[doc = "Lock to current security level"]
    LOCK,
}
impl LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCKW::UNLOCK => false,
            LOCKW::LOCK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "unlock"]
    #[inline]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LOCKW::UNLOCK)
    }
    #[doc = "Lock to current security level"]
    #[inline]
    pub fn lock(self) -> &'a mut W {
        self.variant(LOCKW::LOCK)
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
#[doc = "Values that can be written to the field `KEY`"]
pub enum KEYW {
    #[doc = "If set during write, will allow lock or unlock"]
    KWY_VALUE,
}
impl KEYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            KEYW::KWY_VALUE => 1853,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KEYW<'a> {
    w: &'a mut W,
}
impl<'a> _KEYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KEYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "If set during write, will allow lock or unlock"]
    #[inline]
    pub fn kwy_value(self) -> &'a mut W {
        self.variant(KEYW::KWY_VALUE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 8191;
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Reads back with security level locked to, or 0. Writes as 0 to unlock, 1 to lock."]
    #[inline]
    pub fn lock(&self) -> LOCKR {
        LOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:16 - Must be written as 0x73D to change the register."]
    #[inline]
    pub fn key(&self) -> KEYR {
        KEYR::_from({
            const MASK: u16 = 8191;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
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
    #[doc = "Bit 0 - Reads back with security level locked to, or 0. Writes as 0 to unlock, 1 to lock."]
    #[inline]
    pub fn lock(&mut self) -> _LOCKW {
        _LOCKW { w: self }
    }
    #[doc = "Bits 4:16 - Must be written as 0x73D to change the register."]
    #[inline]
    pub fn key(&mut self) -> _KEYW {
        _KEYW { w: self }
    }
}
