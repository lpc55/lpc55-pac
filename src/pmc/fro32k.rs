#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FRO32K {
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
pub struct NTATR {
    bits: u8,
}
impl NTATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PTATR {
    bits: u8,
}
impl PTATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CAPCALR {
    bits: u16,
}
impl CAPCALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ATBCTRLR {
    bits: u8,
}
impl ATBCTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _NTATW<'a> {
    w: &'a mut W,
}
impl<'a> _NTATW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PTATW<'a> {
    w: &'a mut W,
}
impl<'a> _PTATW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CAPCALW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPCALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ATBCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _ATBCTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 1:3 - Temperature coefficient trimming bits."]
    #[inline]
    pub fn ntat(&self) -> NTATR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NTATR { bits }
    }
    #[doc = "Bits 4:6 - Bias trimming bits (course frequency trimming)."]
    #[inline]
    pub fn ptat(&self) -> PTATR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PTATR { bits }
    }
    #[doc = "Bits 7:15 - Capacitive dac calibration bits (fine frequency trimming)."]
    #[inline]
    pub fn capcal(&self) -> CAPCALR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CAPCALR { bits }
    }
    #[doc = "Bits 16:17 - Debug control bits to set the analog/digital test modes."]
    #[inline]
    pub fn atbctrl(&self) -> ATBCTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ATBCTRLR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 37046 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 1:3 - Temperature coefficient trimming bits."]
    #[inline]
    pub fn ntat(&mut self) -> _NTATW {
        _NTATW { w: self }
    }
    #[doc = "Bits 4:6 - Bias trimming bits (course frequency trimming)."]
    #[inline]
    pub fn ptat(&mut self) -> _PTATW {
        _PTATW { w: self }
    }
    #[doc = "Bits 7:15 - Capacitive dac calibration bits (fine frequency trimming)."]
    #[inline]
    pub fn capcal(&mut self) -> _CAPCALW {
        _CAPCALW { w: self }
    }
    #[doc = "Bits 16:17 - Debug control bits to set the analog/digital test modes."]
    #[inline]
    pub fn atbctrl(&mut self) -> _ATBCTRLW {
        _ATBCTRLW { w: self }
    }
}
