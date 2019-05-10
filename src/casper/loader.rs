#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LOADER {
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
pub struct COUNTR {
    bits: u8,
}
impl COUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CTRLBPAIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRLBPAIRR {
    #[doc = "Bank-pair 0 (1st)"]
    PAIR0,
    #[doc = "Bank-pair 1 (2nd)"]
    PAIR1,
}
impl CTRLBPAIRR {
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
            CTRLBPAIRR::PAIR0 => false,
            CTRLBPAIRR::PAIR1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTRLBPAIRR {
        match value {
            false => CTRLBPAIRR::PAIR0,
            true => CTRLBPAIRR::PAIR1,
        }
    }
    #[doc = "Checks if the value of the field is `PAIR0`"]
    #[inline]
    pub fn is_pair0(&self) -> bool {
        *self == CTRLBPAIRR::PAIR0
    }
    #[doc = "Checks if the value of the field is `PAIR1`"]
    #[inline]
    pub fn is_pair1(&self) -> bool {
        *self == CTRLBPAIRR::PAIR1
    }
}
#[doc = r" Value of the field"]
pub struct CTRLOFFR {
    bits: u16,
}
impl CTRLOFFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _COUNTW<'a> {
    w: &'a mut W,
}
impl<'a> _COUNTW<'a> {
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
#[doc = "Values that can be written to the field `CTRLBPAIR`"]
pub enum CTRLBPAIRW {
    #[doc = "Bank-pair 0 (1st)"]
    PAIR0,
    #[doc = "Bank-pair 1 (2nd)"]
    PAIR1,
}
impl CTRLBPAIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTRLBPAIRW::PAIR0 => false,
            CTRLBPAIRW::PAIR1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTRLBPAIRW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRLBPAIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTRLBPAIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bank-pair 0 (1st)"]
    #[inline]
    pub fn pair0(self) -> &'a mut W {
        self.variant(CTRLBPAIRW::PAIR0)
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline]
    pub fn pair1(self) -> &'a mut W {
        self.variant(CTRLBPAIRW::PAIR1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTRLOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRLOFFW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
        const OFFSET: u8 = 18;
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
    #[doc = "Bits 0:7 - Number of control pairs to load 0 relative (so 1 means load 1). write 1 means Does one op - does not iterate, write N means N control pairs to load"]
    #[inline]
    pub fn count(&self) -> COUNTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        COUNTR { bits }
    }
    #[doc = "Bit 16 - Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation."]
    #[inline]
    pub fn ctrlbpair(&self) -> CTRLBPAIRR {
        CTRLBPAIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 18:28 - DWord Offset of CTRL pair to load next."]
    #[inline]
    pub fn ctrloff(&self) -> CTRLOFFR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CTRLOFFR { bits }
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
    #[doc = "Bits 0:7 - Number of control pairs to load 0 relative (so 1 means load 1). write 1 means Does one op - does not iterate, write N means N control pairs to load"]
    #[inline]
    pub fn count(&mut self) -> _COUNTW {
        _COUNTW { w: self }
    }
    #[doc = "Bit 16 - Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation."]
    #[inline]
    pub fn ctrlbpair(&mut self) -> _CTRLBPAIRW {
        _CTRLBPAIRW { w: self }
    }
    #[doc = "Bits 18:28 - DWord Offset of CTRL pair to load next."]
    #[inline]
    pub fn ctrloff(&mut self) -> _CTRLOFFW {
        _CTRLOFFW { w: self }
    }
}
