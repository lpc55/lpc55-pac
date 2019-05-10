#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CODESECURITYPROTTEST {
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
}
#[doc = "Values that can be written to the field `SEC_CODE`"]
pub enum SEC_CODEW {
    #[doc = "test access is not allowed."]
    DISABLE,
    #[doc = "Security code to allow test access."]
    ENABLE,
}
impl SEC_CODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            SEC_CODEW::DISABLE => 0,
            SEC_CODEW::ENABLE => 305419896,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEC_CODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_CODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEC_CODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "test access is not allowed."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SEC_CODEW::DISABLE)
    }
    #[doc = "Security code to allow test access."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SEC_CODEW::ENABLE)
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
    #[doc = "Bits 0:31 - Security code to allow test access : 0x12345678."]
    #[inline]
    pub fn sec_code(&mut self) -> _SEC_CODEW {
        _SEC_CODEW { w: self }
    }
}
