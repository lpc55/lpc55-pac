#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IDXBLK_H {
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
pub struct IDX8R {
    bits: u8,
}
impl IDX8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IDX9R {
    bits: u8,
}
impl IDX9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IDX10R {
    bits: u8,
}
impl IDX10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IDX11R {
    bits: u8,
}
impl IDX11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IDX12R {
    bits: u8,
}
impl IDX12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IDX13R {
    bits: u8,
}
impl IDX13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IDX14R {
    bits: u8,
}
impl IDX14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IDX15R {
    bits: u8,
}
impl IDX15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _IDX8W<'a> {
    w: &'a mut W,
}
impl<'a> _IDX8W<'a> {
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
pub struct _IDX9W<'a> {
    w: &'a mut W,
}
impl<'a> _IDX9W<'a> {
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
pub struct _IDX10W<'a> {
    w: &'a mut W,
}
impl<'a> _IDX10W<'a> {
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
pub struct _IDX11W<'a> {
    w: &'a mut W,
}
impl<'a> _IDX11W<'a> {
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
#[doc = r" Proxy"]
pub struct _IDX12W<'a> {
    w: &'a mut W,
}
impl<'a> _IDX12W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IDX13W<'a> {
    w: &'a mut W,
}
impl<'a> _IDX13W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IDX14W<'a> {
    w: &'a mut W,
}
impl<'a> _IDX14W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IDX15W<'a> {
    w: &'a mut W,
}
impl<'a> _IDX15W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_IDXW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_IDXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - Use to block PUF index 8"]
    #[inline]
    pub fn idx8(&self) -> IDX8R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDX8R { bits }
    }
    #[doc = "Bits 2:3 - Use to block PUF index 9"]
    #[inline]
    pub fn idx9(&self) -> IDX9R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDX9R { bits }
    }
    #[doc = "Bits 4:5 - Use to block PUF index 10"]
    #[inline]
    pub fn idx10(&self) -> IDX10R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDX10R { bits }
    }
    #[doc = "Bits 6:7 - Use to block PUF index 11"]
    #[inline]
    pub fn idx11(&self) -> IDX11R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDX11R { bits }
    }
    #[doc = "Bits 8:9 - Use to block PUF index 12"]
    #[inline]
    pub fn idx12(&self) -> IDX12R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDX12R { bits }
    }
    #[doc = "Bits 10:11 - Use to block PUF index 13"]
    #[inline]
    pub fn idx13(&self) -> IDX13R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDX13R { bits }
    }
    #[doc = "Bits 12:13 - Use to block PUF index 14"]
    #[inline]
    pub fn idx14(&self) -> IDX14R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDX14R { bits }
    }
    #[doc = "Bits 14:15 - Use to block PUF index 15"]
    #[inline]
    pub fn idx15(&self) -> IDX15R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDX15R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2147527338 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Use to block PUF index 8"]
    #[inline]
    pub fn idx8(&mut self) -> _IDX8W {
        _IDX8W { w: self }
    }
    #[doc = "Bits 2:3 - Use to block PUF index 9"]
    #[inline]
    pub fn idx9(&mut self) -> _IDX9W {
        _IDX9W { w: self }
    }
    #[doc = "Bits 4:5 - Use to block PUF index 10"]
    #[inline]
    pub fn idx10(&mut self) -> _IDX10W {
        _IDX10W { w: self }
    }
    #[doc = "Bits 6:7 - Use to block PUF index 11"]
    #[inline]
    pub fn idx11(&mut self) -> _IDX11W {
        _IDX11W { w: self }
    }
    #[doc = "Bits 8:9 - Use to block PUF index 12"]
    #[inline]
    pub fn idx12(&mut self) -> _IDX12W {
        _IDX12W { w: self }
    }
    #[doc = "Bits 10:11 - Use to block PUF index 13"]
    #[inline]
    pub fn idx13(&mut self) -> _IDX13W {
        _IDX13W { w: self }
    }
    #[doc = "Bits 12:13 - Use to block PUF index 14"]
    #[inline]
    pub fn idx14(&mut self) -> _IDX14W {
        _IDX14W { w: self }
    }
    #[doc = "Bits 14:15 - Use to block PUF index 15"]
    #[inline]
    pub fn idx15(&mut self) -> _IDX15W {
        _IDX15W { w: self }
    }
    #[doc = "Bits 30:31 - Lock 8 to 15 PUF key indexes"]
    #[inline]
    pub fn lock_idx(&mut self) -> _LOCK_IDXW {
        _LOCK_IDXW { w: self }
    }
}
