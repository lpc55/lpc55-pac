#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLOCKGENUPDATELOCKOUT {
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
#[doc = "Possible values of the field `CLOCKGENUPDATELOCKOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLOCKGENUPDATELOCKOUTR {
    #[doc = "all hardware clock configruration are freeze."]
    FREEZE,
    #[doc = "update all clock configuration."]
    ENABLE,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl CLOCKGENUPDATELOCKOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            CLOCKGENUPDATELOCKOUTR::FREEZE => 0,
            CLOCKGENUPDATELOCKOUTR::ENABLE => 1,
            CLOCKGENUPDATELOCKOUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> CLOCKGENUPDATELOCKOUTR {
        match value {
            0 => CLOCKGENUPDATELOCKOUTR::FREEZE,
            1 => CLOCKGENUPDATELOCKOUTR::ENABLE,
            i => CLOCKGENUPDATELOCKOUTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FREEZE`"]
    #[inline]
    pub fn is_freeze(&self) -> bool {
        *self == CLOCKGENUPDATELOCKOUTR::FREEZE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CLOCKGENUPDATELOCKOUTR::ENABLE
    }
}
#[doc = "Values that can be written to the field `CLOCKGENUPDATELOCKOUT`"]
pub enum CLOCKGENUPDATELOCKOUTW {
    #[doc = "all hardware clock configruration are freeze."]
    FREEZE,
    #[doc = "update all clock configuration."]
    ENABLE,
}
impl CLOCKGENUPDATELOCKOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            CLOCKGENUPDATELOCKOUTW::FREEZE => 0,
            CLOCKGENUPDATELOCKOUTW::ENABLE => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLOCKGENUPDATELOCKOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _CLOCKGENUPDATELOCKOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLOCKGENUPDATELOCKOUTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "all hardware clock configruration are freeze."]
    #[inline]
    pub fn freeze(self) -> &'a mut W {
        self.variant(CLOCKGENUPDATELOCKOUTW::FREEZE)
    }
    #[doc = "update all clock configuration."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CLOCKGENUPDATELOCKOUTW::ENABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
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
    #[doc = "Bits 0:31 - Control clock configuration registers access (like xxxDIV, xxxSEL)."]
    #[inline]
    pub fn clockgenupdatelockout(&self) -> CLOCKGENUPDATELOCKOUTR {
        CLOCKGENUPDATELOCKOUTR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
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
    #[doc = "Bits 0:31 - Control clock configuration registers access (like xxxDIV, xxxSEL)."]
    #[inline]
    pub fn clockgenupdatelockout(&mut self) -> _CLOCKGENUPDATELOCKOUTW {
        _CLOCKGENUPDATELOCKOUTW { w: self }
    }
}
