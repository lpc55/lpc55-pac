#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USB1_CHRG_DETECT {
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
pub struct PULLUP_DPR {
    bits: bool,
}
impl PULLUP_DPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `BGR_IBIAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGR_IBIASR {
    #[doc = "Bias current is derived from the USB PHY internal current generator."]
    VALUE0,
    #[doc = "Bias current is derived from the reference generator of the bandgap."]
    VALUE1,
}
impl BGR_IBIASR {
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
            BGR_IBIASR::VALUE0 => false,
            BGR_IBIASR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BGR_IBIASR {
        match value {
            false => BGR_IBIASR::VALUE0,
            true => BGR_IBIASR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == BGR_IBIASR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BGR_IBIASR::VALUE1
    }
}
#[doc = r" Proxy"]
pub struct _PULLUP_DPW<'a> {
    w: &'a mut W,
}
impl<'a> _PULLUP_DPW<'a> {
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
#[doc = "Values that can be written to the field `BGR_IBIAS`"]
pub enum BGR_IBIASW {
    #[doc = "Bias current is derived from the USB PHY internal current generator."]
    VALUE0,
    #[doc = "Bias current is derived from the reference generator of the bandgap."]
    VALUE1,
}
impl BGR_IBIASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BGR_IBIASW::VALUE0 => false,
            BGR_IBIASW::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BGR_IBIASW<'a> {
    w: &'a mut W,
}
impl<'a> _BGR_IBIASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BGR_IBIASW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bias current is derived from the USB PHY internal current generator."]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(BGR_IBIASW::VALUE0)
    }
    #[doc = "Bias current is derived from the reference generator of the bandgap."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BGR_IBIASW::VALUE1)
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
        const OFFSET: u8 = 23;
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
    #[doc = "Bit 2 - This bit is used to pull up DP, for digital charge detect."]
    #[inline]
    pub fn pullup_dp(&self) -> PULLUP_DPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PULLUP_DPR { bits }
    }
    #[doc = "Bit 23 - USB charge detector bias current reference This bit determines the reference for the bias current of the USB charge detector"]
    #[inline]
    pub fn bgr_ibias(&self) -> BGR_IBIASR {
        BGR_IBIASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2149056512 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - This bit is used to pull up DP, for digital charge detect."]
    #[inline]
    pub fn pullup_dp(&mut self) -> _PULLUP_DPW {
        _PULLUP_DPW { w: self }
    }
    #[doc = "Bit 23 - USB charge detector bias current reference This bit determines the reference for the bias current of the USB charge detector"]
    #[inline]
    pub fn bgr_ibias(&mut self) -> _BGR_IBIASW {
        _BGR_IBIASW { w: self }
    }
}
