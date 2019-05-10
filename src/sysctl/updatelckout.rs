#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::UPDATELCKOUT {
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
#[doc = "Possible values of the field `UPDATELCKOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDATELCKOUTR {
    #[doc = "Normal Mode. Can be written to."]
    NORMAL_MODE,
    #[doc = "Protected Mode. Cannot be written to."]
    PROTECTED_MODE,
}
impl UPDATELCKOUTR {
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
            UPDATELCKOUTR::NORMAL_MODE => false,
            UPDATELCKOUTR::PROTECTED_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UPDATELCKOUTR {
        match value {
            false => UPDATELCKOUTR::NORMAL_MODE,
            true => UPDATELCKOUTR::PROTECTED_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_MODE`"]
    #[inline]
    pub fn is_normal_mode(&self) -> bool {
        *self == UPDATELCKOUTR::NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `PROTECTED_MODE`"]
    #[inline]
    pub fn is_protected_mode(&self) -> bool {
        *self == UPDATELCKOUTR::PROTECTED_MODE
    }
}
#[doc = "Values that can be written to the field `UPDATELCKOUT`"]
pub enum UPDATELCKOUTW {
    #[doc = "Normal Mode. Can be written to."]
    NORMAL_MODE,
    #[doc = "Protected Mode. Cannot be written to."]
    PROTECTED_MODE,
}
impl UPDATELCKOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UPDATELCKOUTW::NORMAL_MODE => false,
            UPDATELCKOUTW::PROTECTED_MODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UPDATELCKOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDATELCKOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UPDATELCKOUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal Mode. Can be written to."]
    #[inline]
    pub fn normal_mode(self) -> &'a mut W {
        self.variant(UPDATELCKOUTW::NORMAL_MODE)
    }
    #[doc = "Protected Mode. Cannot be written to."]
    #[inline]
    pub fn protected_mode(self) -> &'a mut W {
        self.variant(UPDATELCKOUTW::PROTECTED_MODE)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - All Registers"]
    #[inline]
    pub fn updatelckout(&self) -> UPDATELCKOUTR {
        UPDATELCKOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 0 - All Registers"]
    #[inline]
    pub fn updatelckout(&mut self) -> _UPDATELCKOUTW {
        _UPDATELCKOUTW { w: self }
    }
}
