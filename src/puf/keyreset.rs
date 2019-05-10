#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::KEYRESET {
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
#[doc = r" Proxy"]
pub struct _KEY0W<'a> {
    w: &'a mut W,
}
impl<'a> _KEY0W<'a> {
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
pub struct _KEY1W<'a> {
    w: &'a mut W,
}
impl<'a> _KEY1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _KEY2W<'a> {
    w: &'a mut W,
}
impl<'a> _KEY2W<'a> {
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
pub struct _KEY3W<'a> {
    w: &'a mut W,
}
impl<'a> _KEY3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
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
    #[doc = "Bits 0:1 - 10: Reset KEY0 shift register. Self clearing. Must be done before loading any new key."]
    #[inline]
    pub fn key0(&mut self) -> _KEY0W {
        _KEY0W { w: self }
    }
    #[doc = "Bits 2:3 - 10: Reset KEY1 shift register. Self clearing. Must be done before loading any new key."]
    #[inline]
    pub fn key1(&mut self) -> _KEY1W {
        _KEY1W { w: self }
    }
    #[doc = "Bits 4:5 - 10: Reset KEY2 shift register. Self clearing. Must be done before loading any new key."]
    #[inline]
    pub fn key2(&mut self) -> _KEY2W {
        _KEY2W { w: self }
    }
    #[doc = "Bits 6:7 - 10: Reset KEY3 shift register. Self clearing. Must be done before loading any new key."]
    #[inline]
    pub fn key3(&mut self) -> _KEY3W {
        _KEY3W { w: self }
    }
}
