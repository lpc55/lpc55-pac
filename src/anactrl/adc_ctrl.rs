#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ADC_CTRL {
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
#[doc = "Possible values of the field `VBATDIVENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATDIVENABLER {
    #[doc = "VBAT divider branch is disabled."]
    DISABLE,
    #[doc = "VBAT divider branch is enabled."]
    ENABLE,
}
impl VBATDIVENABLER {
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
            VBATDIVENABLER::DISABLE => false,
            VBATDIVENABLER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VBATDIVENABLER {
        match value {
            false => VBATDIVENABLER::DISABLE,
            true => VBATDIVENABLER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == VBATDIVENABLER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == VBATDIVENABLER::ENABLE
    }
}
#[doc = "Values that can be written to the field `VBATDIVENABLE`"]
pub enum VBATDIVENABLEW {
    #[doc = "VBAT divider branch is disabled."]
    DISABLE,
    #[doc = "VBAT divider branch is enabled."]
    ENABLE,
}
impl VBATDIVENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VBATDIVENABLEW::DISABLE => false,
            VBATDIVENABLEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VBATDIVENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _VBATDIVENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VBATDIVENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "VBAT divider branch is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(VBATDIVENABLEW::DISABLE)
    }
    #[doc = "VBAT divider branch is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(VBATDIVENABLEW::ENABLE)
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
    #[doc = "Bit 0 - Switch On/Off VBAT divider branch."]
    #[inline]
    pub fn vbatdivenable(&self) -> VBATDIVENABLER {
        VBATDIVENABLER::_from({
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
    #[doc = "Bit 0 - Switch On/Off VBAT divider branch."]
    #[inline]
    pub fn vbatdivenable(&mut self) -> _VBATDIVENABLEW {
        _VBATDIVENABLEW { w: self }
    }
}
