#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUTFORMAT {
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
pub struct OUT_FORMATINTR {
    bits: u8,
}
impl OUT_FORMATINTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OUT_FORMATEXTR {
    bits: u8,
}
impl OUT_FORMATEXTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OUT_SCALERR {
    bits: u8,
}
impl OUT_SCALERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _OUT_FORMATINTW<'a> {
    w: &'a mut W,
}
impl<'a> _OUT_FORMATINTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OUT_FORMATEXTW<'a> {
    w: &'a mut W,
}
impl<'a> _OUT_FORMATEXTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OUT_SCALERW<'a> {
    w: &'a mut W,
}
impl<'a> _OUT_SCALERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:1 - Output Internal format (00: q15; 01:q31; 10:float)"]
    #[inline]
    pub fn out_formatint(&self) -> OUT_FORMATINTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OUT_FORMATINTR { bits }
    }
    #[doc = "Bits 4:5 - Output External format (00: q15; 01:q31; 10:float)"]
    #[inline]
    pub fn out_formatext(&self) -> OUT_FORMATEXTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OUT_FORMATEXTR { bits }
    }
    #[doc = "Bits 8:15 - Output Scaler value (for scaled 'q31' formats)"]
    #[inline]
    pub fn out_scaler(&self) -> OUT_SCALERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OUT_SCALERR { bits }
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
    #[doc = "Bits 0:1 - Output Internal format (00: q15; 01:q31; 10:float)"]
    #[inline]
    pub fn out_formatint(&mut self) -> _OUT_FORMATINTW {
        _OUT_FORMATINTW { w: self }
    }
    #[doc = "Bits 4:5 - Output External format (00: q15; 01:q31; 10:float)"]
    #[inline]
    pub fn out_formatext(&mut self) -> _OUT_FORMATEXTW {
        _OUT_FORMATEXTW { w: self }
    }
    #[doc = "Bits 8:15 - Output Scaler value (for scaled 'q31' formats)"]
    #[inline]
    pub fn out_scaler(&mut self) -> _OUT_SCALERW {
        _OUT_SCALERW { w: self }
    }
}
