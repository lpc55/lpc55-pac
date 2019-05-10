#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEBUG_LOCK_EN {
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
#[doc = "Possible values of the field `LOCK_ALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_ALLR {
    #[doc = "Any other value than b1010: disable write access to all 6 registers."]
    DISABLE,
    #[doc = "1010: Enable write access to all 6 registers."]
    ENABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LOCK_ALLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LOCK_ALLR::DISABLE => 0,
            LOCK_ALLR::ENABLE => 10,
            LOCK_ALLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LOCK_ALLR {
        match value {
            0 => LOCK_ALLR::DISABLE,
            10 => LOCK_ALLR::ENABLE,
            i => LOCK_ALLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == LOCK_ALLR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == LOCK_ALLR::ENABLE
    }
}
#[doc = "Values that can be written to the field `LOCK_ALL`"]
pub enum LOCK_ALLW {
    #[doc = "Any other value than b1010: disable write access to all 6 registers."]
    DISABLE,
    #[doc = "1010: Enable write access to all 6 registers."]
    ENABLE,
}
impl LOCK_ALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LOCK_ALLW::DISABLE => 0,
            LOCK_ALLW::ENABLE => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_ALLW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_ALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_ALLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Any other value than b1010: disable write access to all 6 registers."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOCK_ALLW::DISABLE)
    }
    #[doc = "1010: Enable write access to all 6 registers."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(LOCK_ALLW::ENABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Control write access to CODESECURITYPROTTEST, CODESECURITYPROTCPU0, CODESECURITYPROTCPU1, CM33_DEBUG_FEATURES, MCM33_DEBUG_FEATURES and DBG_AUTH_SCRATCH registers."]
    #[inline]
    pub fn lock_all(&self) -> LOCK_ALLR {
        LOCK_ALLR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 5 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Control write access to CODESECURITYPROTTEST, CODESECURITYPROTCPU0, CODESECURITYPROTCPU1, CM33_DEBUG_FEATURES, MCM33_DEBUG_FEATURES and DBG_AUTH_SCRATCH registers."]
    #[inline]
    pub fn lock_all(&mut self) -> _LOCK_ALLW {
        _LOCK_ALLW { w: self }
    }
}
