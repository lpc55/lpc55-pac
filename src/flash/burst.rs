#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BURST {
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
pub struct XOR_MASKR {
    bits: u32,
}
impl XOR_MASKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DESCR1R {
    bits: u8,
}
impl DESCR1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DESCR2R {
    bits: u8,
}
impl DESCR2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DESCR3R {
    bits: u8,
}
impl DESCR3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _XOR_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _XOR_MASKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 1048575;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DESCR1W<'a> {
    w: &'a mut W,
}
impl<'a> _DESCR1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DESCR2W<'a> {
    w: &'a mut W,
}
impl<'a> _DESCR2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DESCR3W<'a> {
    w: &'a mut W,
}
impl<'a> _DESCR3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:19 - burst 2 XOR mask."]
    #[inline]
    pub fn xor_mask(&self) -> XOR_MASKR {
        let bits = {
            const MASK: u32 = 1048575;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        XOR_MASKR { bits }
    }
    #[doc = "Bits 20:23 - Burst 1 descriptor."]
    #[inline]
    pub fn descr1(&self) -> DESCR1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DESCR1R { bits }
    }
    #[doc = "Bits 24:27 - Burst 2 descriptor."]
    #[inline]
    pub fn descr2(&self) -> DESCR2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DESCR2R { bits }
    }
    #[doc = "Bits 28:31 - Burst 3 descriptor."]
    #[inline]
    pub fn descr3(&self) -> DESCR3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DESCR3R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2147483648 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:19 - burst 2 XOR mask."]
    #[inline]
    pub fn xor_mask(&mut self) -> _XOR_MASKW {
        _XOR_MASKW { w: self }
    }
    #[doc = "Bits 20:23 - Burst 1 descriptor."]
    #[inline]
    pub fn descr1(&mut self) -> _DESCR1W {
        _DESCR1W { w: self }
    }
    #[doc = "Bits 24:27 - Burst 2 descriptor."]
    #[inline]
    pub fn descr2(&mut self) -> _DESCR2W {
        _DESCR2W { w: self }
    }
    #[doc = "Bits 28:31 - Burst 3 descriptor."]
    #[inline]
    pub fn descr3(&mut self) -> _DESCR3W {
        _DESCR3W { w: self }
    }
}
