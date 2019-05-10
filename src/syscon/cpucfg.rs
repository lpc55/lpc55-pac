#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CPUCFG {
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
#[doc = "Possible values of the field `CPU1ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1ENABLER {
    #[doc = "CPU1 is disable (Processor in reset)."]
    DISABLE,
    #[doc = "CPU1 is enable."]
    ENABLE,
}
impl CPU1ENABLER {
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
            CPU1ENABLER::DISABLE => false,
            CPU1ENABLER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPU1ENABLER {
        match value {
            false => CPU1ENABLER::DISABLE,
            true => CPU1ENABLER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CPU1ENABLER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CPU1ENABLER::ENABLE
    }
}
#[doc = "Values that can be written to the field `CPU1ENABLE`"]
pub enum CPU1ENABLEW {
    #[doc = "CPU1 is disable (Processor in reset)."]
    DISABLE,
    #[doc = "CPU1 is enable."]
    ENABLE,
}
impl CPU1ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPU1ENABLEW::DISABLE => false,
            CPU1ENABLEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPU1ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _CPU1ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPU1ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CPU1 is disable (Processor in reset)."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CPU1ENABLEW::DISABLE)
    }
    #[doc = "CPU1 is enable."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU1ENABLEW::ENABLE)
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
    #[doc = "Bit 2 - Enable CPU1."]
    #[inline]
    pub fn cpu1enable(&self) -> CPU1ENABLER {
        CPU1ENABLER::_from({
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
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - Enable CPU1."]
    #[inline]
    pub fn cpu1enable(&mut self) -> _CPU1ENABLEW {
        _CPU1ENABLEW { w: self }
    }
}
