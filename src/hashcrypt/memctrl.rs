#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MEMCTRL {
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
#[doc = "Possible values of the field `MASTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASTERR {
    #[doc = "Mastering is not used and the normal DMA or Interrupt based model is used with INDATA."]
    NOT_USED,
    #[doc = "Mastering is enabled and DMA and INDATA should not be used."]
    ENABLED,
}
impl MASTERR {
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
            MASTERR::NOT_USED => false,
            MASTERR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASTERR {
        match value {
            false => MASTERR::NOT_USED,
            true => MASTERR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_USED`"]
    #[inline]
    pub fn is_not_used(&self) -> bool {
        *self == MASTERR::NOT_USED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == MASTERR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct COUNTR {
    bits: u16,
}
impl COUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `MASTER`"]
pub enum MASTERW {
    #[doc = "Mastering is not used and the normal DMA or Interrupt based model is used with INDATA."]
    NOT_USED,
    #[doc = "Mastering is enabled and DMA and INDATA should not be used."]
    ENABLED,
}
impl MASTERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASTERW::NOT_USED => false,
            MASTERW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASTERW<'a> {
    w: &'a mut W,
}
impl<'a> _MASTERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASTERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Mastering is not used and the normal DMA or Interrupt based model is used with INDATA."]
    #[inline]
    pub fn not_used(self) -> &'a mut W {
        self.variant(MASTERW::NOT_USED)
    }
    #[doc = "Mastering is enabled and DMA and INDATA should not be used."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MASTERW::ENABLED)
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
pub struct _COUNTW<'a> {
    w: &'a mut W,
}
impl<'a> _COUNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - no description available"]
    #[inline]
    pub fn master(&self) -> MASTERR {
        MASTERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:26 - Number of 512-bit (128-bit if AES, except 1st block which may include key and IV) blocks to copy starting at MEMADDR. This register will decrement after each block is copied, ending in 0. For Hash, the DIGEST interrupt will occur when it reaches 0. Fro AES, the DIGEST/OUTDATA interrupt will occur on ever block. If a bus error occurs, it will stop with this field set to the block that failed. 0:Done - nothing to process. 1 to 2K: Number of 512-bit (or 128bit) blocks to hash."]
    #[inline]
    pub fn count(&self) -> COUNTR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        COUNTR { bits }
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
    #[doc = "Bit 0 - no description available"]
    #[inline]
    pub fn master(&mut self) -> _MASTERW {
        _MASTERW { w: self }
    }
    #[doc = "Bits 16:26 - Number of 512-bit (128-bit if AES, except 1st block which may include key and IV) blocks to copy starting at MEMADDR. This register will decrement after each block is copied, ending in 0. For Hash, the DIGEST interrupt will occur when it reaches 0. Fro AES, the DIGEST/OUTDATA interrupt will occur on ever block. If a bus error occurs, it will stop with this field set to the block that failed. 0:Done - nothing to process. 1 to 2K: Number of 512-bit (or 128bit) blocks to hash."]
    #[inline]
    pub fn count(&mut self) -> _COUNTW {
        _COUNTW { w: self }
    }
}
