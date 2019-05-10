#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SWR_RESET {
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
#[doc = "Values that can be written to the field `SWR_RESET`"]
pub enum SWR_RESETW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Generate a software reset."]
    ASSERTED,
}
impl SWR_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            SWR_RESETW::RELEASED => 0,
            SWR_RESETW::ASSERTED => 1509949441,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWR_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _SWR_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWR_RESETW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(SWR_RESETW::RELEASED)
    }
    #[doc = "Generate a software reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SWR_RESETW::ASSERTED)
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
    #[doc = "Bits 0:31 - Write 0x5A00_0001 to generate a software_reset."]
    #[inline]
    pub fn swr_reset(&mut self) -> _SWR_RESETW {
        _SWR_RESETW { w: self }
    }
}
