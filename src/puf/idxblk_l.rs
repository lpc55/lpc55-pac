#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IDXBLK_L {
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
pub struct IDX0R {
    bits: u8,
}
impl IDX0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IDX1R {
    bits: u8,
}
impl IDX1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IDX2R {
    bits: u8,
}
impl IDX2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IDX3R {
    bits: u8,
}
impl IDX3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IDX4R {
    bits: u8,
}
impl IDX4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IDX5R {
    bits: u8,
}
impl IDX5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IDX6R {
    bits: u8,
}
impl IDX6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IDX7R {
    bits: u8,
}
impl IDX7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _IDX0W<'a> {
    w: &'a mut W,
}
impl<'a> _IDX0W<'a> {
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
pub struct _IDX1W<'a> {
    w: &'a mut W,
}
impl<'a> _IDX1W<'a> {
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
pub struct _IDX2W<'a> {
    w: &'a mut W,
}
impl<'a> _IDX2W<'a> {
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
pub struct _IDX3W<'a> {
    w: &'a mut W,
}
impl<'a> _IDX3W<'a> {
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
pub struct _IDX4W<'a> {
    w: &'a mut W,
}
impl<'a> _IDX4W<'a> {
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
pub struct _IDX5W<'a> {
    w: &'a mut W,
}
impl<'a> _IDX5W<'a> {
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
pub struct _IDX6W<'a> {
    w: &'a mut W,
}
impl<'a> _IDX6W<'a> {
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
pub struct _IDX7W<'a> {
    w: &'a mut W,
}
impl<'a> _IDX7W<'a> {
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
    #[doc = "Bits 0:1 - Use to block PUF index 0"]
    #[inline]
    pub fn idx0(&self) -> IDX0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDX0R { bits }
    }
    #[doc = "Bits 2:3 - Use to block PUF index 1"]
    #[inline]
    pub fn idx1(&self) -> IDX1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDX1R { bits }
    }
    #[doc = "Bits 4:5 - Use to block PUF index 2"]
    #[inline]
    pub fn idx2(&self) -> IDX2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDX2R { bits }
    }
    #[doc = "Bits 6:7 - Use to block PUF index 3"]
    #[inline]
    pub fn idx3(&self) -> IDX3R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDX3R { bits }
    }
    #[doc = "Bits 8:9 - Use to block PUF index 4"]
    #[inline]
    pub fn idx4(&self) -> IDX4R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDX4R { bits }
    }
    #[doc = "Bits 10:11 - Use to block PUF index 5"]
    #[inline]
    pub fn idx5(&self) -> IDX5R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDX5R { bits }
    }
    #[doc = "Bits 12:13 - Use to block PUF index 6"]
    #[inline]
    pub fn idx6(&self) -> IDX6R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDX6R { bits }
    }
    #[doc = "Bits 14:15 - Use to block PUF index 7"]
    #[inline]
    pub fn idx7(&self) -> IDX7R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDX7R { bits }
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
    #[doc = "Bits 0:1 - Use to block PUF index 0"]
    #[inline]
    pub fn idx0(&mut self) -> _IDX0W {
        _IDX0W { w: self }
    }
    #[doc = "Bits 2:3 - Use to block PUF index 1"]
    #[inline]
    pub fn idx1(&mut self) -> _IDX1W {
        _IDX1W { w: self }
    }
    #[doc = "Bits 4:5 - Use to block PUF index 2"]
    #[inline]
    pub fn idx2(&mut self) -> _IDX2W {
        _IDX2W { w: self }
    }
    #[doc = "Bits 6:7 - Use to block PUF index 3"]
    #[inline]
    pub fn idx3(&mut self) -> _IDX3W {
        _IDX3W { w: self }
    }
    #[doc = "Bits 8:9 - Use to block PUF index 4"]
    #[inline]
    pub fn idx4(&mut self) -> _IDX4W {
        _IDX4W { w: self }
    }
    #[doc = "Bits 10:11 - Use to block PUF index 5"]
    #[inline]
    pub fn idx5(&mut self) -> _IDX5W {
        _IDX5W { w: self }
    }
    #[doc = "Bits 12:13 - Use to block PUF index 6"]
    #[inline]
    pub fn idx6(&mut self) -> _IDX6W {
        _IDX6W { w: self }
    }
    #[doc = "Bits 14:15 - Use to block PUF index 7"]
    #[inline]
    pub fn idx7(&mut self) -> _IDX7W {
        _IDX7W { w: self }
    }
    #[doc = "Bits 30:31 - Lock 0 to 7 PUF key indexes"]
    #[inline]
    pub fn lock_idx(&mut self) -> _LOCK_IDXW {
        _LOCK_IDXW { w: self }
    }
}
