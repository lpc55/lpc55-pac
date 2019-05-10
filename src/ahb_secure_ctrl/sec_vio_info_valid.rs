#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SEC_VIO_INFO_VALID {
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
pub struct VIO_INFO_VALID0R {
    bits: bool,
}
impl VIO_INFO_VALID0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VIO_INFO_VALID1R {
    bits: bool,
}
impl VIO_INFO_VALID1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VIO_INFO_VALID2R {
    bits: bool,
}
impl VIO_INFO_VALID2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VIO_INFO_VALID3R {
    bits: bool,
}
impl VIO_INFO_VALID3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VIO_INFO_VALID4R {
    bits: bool,
}
impl VIO_INFO_VALID4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VIO_INFO_VALID5R {
    bits: bool,
}
impl VIO_INFO_VALID5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VIO_INFO_VALID6R {
    bits: bool,
}
impl VIO_INFO_VALID6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VIO_INFO_VALID7R {
    bits: bool,
}
impl VIO_INFO_VALID7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VIO_INFO_VALID8R {
    bits: bool,
}
impl VIO_INFO_VALID8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VIO_INFO_VALID9R {
    bits: bool,
}
impl VIO_INFO_VALID9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VIO_INFO_VALID10R {
    bits: bool,
}
impl VIO_INFO_VALID10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VIO_INFO_VALID11R {
    bits: bool,
}
impl VIO_INFO_VALID11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VIO_INFO_VALID12R {
    bits: bool,
}
impl VIO_INFO_VALID12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VIO_INFO_VALID13R {
    bits: bool,
}
impl VIO_INFO_VALID13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VIO_INFO_VALID14R {
    bits: bool,
}
impl VIO_INFO_VALID14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VIO_INFO_VALID15R {
    bits: bool,
}
impl VIO_INFO_VALID15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VIO_INFO_VALID16R {
    bits: bool,
}
impl VIO_INFO_VALID16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VIO_INFO_VALID17R {
    bits: bool,
}
impl VIO_INFO_VALID17R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID0W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID0W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID1W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID1W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID2W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID2W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID3W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID3W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID4W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID4W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID5W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID5W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID6W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID6W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID7W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID7W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID8W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID8W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID9W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID9W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID10W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID10W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID11W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID11W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID12W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID12W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID13W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID13W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID14W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID14W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID15W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID15W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID16W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID16W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID17W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID17W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
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
    #[doc = "Bit 0 - violation information valid flag for AHB layer 0. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid0(&self) -> VIO_INFO_VALID0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VIO_INFO_VALID0R { bits }
    }
    #[doc = "Bit 1 - violation information valid flag for AHB layer 1. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid1(&self) -> VIO_INFO_VALID1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VIO_INFO_VALID1R { bits }
    }
    #[doc = "Bit 2 - violation information valid flag for AHB layer 2. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid2(&self) -> VIO_INFO_VALID2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VIO_INFO_VALID2R { bits }
    }
    #[doc = "Bit 3 - violation information valid flag for AHB layer 3. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid3(&self) -> VIO_INFO_VALID3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VIO_INFO_VALID3R { bits }
    }
    #[doc = "Bit 4 - violation information valid flag for AHB layer 4. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid4(&self) -> VIO_INFO_VALID4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VIO_INFO_VALID4R { bits }
    }
    #[doc = "Bit 5 - violation information valid flag for AHB layer 5. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid5(&self) -> VIO_INFO_VALID5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VIO_INFO_VALID5R { bits }
    }
    #[doc = "Bit 6 - violation information valid flag for AHB layer 6. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid6(&self) -> VIO_INFO_VALID6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VIO_INFO_VALID6R { bits }
    }
    #[doc = "Bit 7 - violation information valid flag for AHB layer 7. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid7(&self) -> VIO_INFO_VALID7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VIO_INFO_VALID7R { bits }
    }
    #[doc = "Bit 8 - violation information valid flag for AHB layer 8. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid8(&self) -> VIO_INFO_VALID8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VIO_INFO_VALID8R { bits }
    }
    #[doc = "Bit 9 - violation information valid flag for AHB layer 9. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid9(&self) -> VIO_INFO_VALID9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VIO_INFO_VALID9R { bits }
    }
    #[doc = "Bit 10 - violation information valid flag for AHB layer 10. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid10(&self) -> VIO_INFO_VALID10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VIO_INFO_VALID10R { bits }
    }
    #[doc = "Bit 11 - violation information valid flag for AHB layer 11. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid11(&self) -> VIO_INFO_VALID11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VIO_INFO_VALID11R { bits }
    }
    #[doc = "Bit 12 - violation information valid flag for AHB layer 12. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid12(&self) -> VIO_INFO_VALID12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VIO_INFO_VALID12R { bits }
    }
    #[doc = "Bit 13 - violation information valid flag for AHB layer 13. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid13(&self) -> VIO_INFO_VALID13R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VIO_INFO_VALID13R { bits }
    }
    #[doc = "Bit 14 - violation information valid flag for AHB layer 14. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid14(&self) -> VIO_INFO_VALID14R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VIO_INFO_VALID14R { bits }
    }
    #[doc = "Bit 15 - violation information valid flag for AHB layer 15. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid15(&self) -> VIO_INFO_VALID15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VIO_INFO_VALID15R { bits }
    }
    #[doc = "Bit 16 - violation information valid flag for AHB layer 16. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid16(&self) -> VIO_INFO_VALID16R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VIO_INFO_VALID16R { bits }
    }
    #[doc = "Bit 17 - violation information valid flag for AHB layer 17. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid17(&self) -> VIO_INFO_VALID17R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VIO_INFO_VALID17R { bits }
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
    #[doc = "Bit 0 - violation information valid flag for AHB layer 0. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid0(&mut self) -> _VIO_INFO_VALID0W {
        _VIO_INFO_VALID0W { w: self }
    }
    #[doc = "Bit 1 - violation information valid flag for AHB layer 1. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid1(&mut self) -> _VIO_INFO_VALID1W {
        _VIO_INFO_VALID1W { w: self }
    }
    #[doc = "Bit 2 - violation information valid flag for AHB layer 2. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid2(&mut self) -> _VIO_INFO_VALID2W {
        _VIO_INFO_VALID2W { w: self }
    }
    #[doc = "Bit 3 - violation information valid flag for AHB layer 3. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid3(&mut self) -> _VIO_INFO_VALID3W {
        _VIO_INFO_VALID3W { w: self }
    }
    #[doc = "Bit 4 - violation information valid flag for AHB layer 4. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid4(&mut self) -> _VIO_INFO_VALID4W {
        _VIO_INFO_VALID4W { w: self }
    }
    #[doc = "Bit 5 - violation information valid flag for AHB layer 5. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid5(&mut self) -> _VIO_INFO_VALID5W {
        _VIO_INFO_VALID5W { w: self }
    }
    #[doc = "Bit 6 - violation information valid flag for AHB layer 6. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid6(&mut self) -> _VIO_INFO_VALID6W {
        _VIO_INFO_VALID6W { w: self }
    }
    #[doc = "Bit 7 - violation information valid flag for AHB layer 7. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid7(&mut self) -> _VIO_INFO_VALID7W {
        _VIO_INFO_VALID7W { w: self }
    }
    #[doc = "Bit 8 - violation information valid flag for AHB layer 8. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid8(&mut self) -> _VIO_INFO_VALID8W {
        _VIO_INFO_VALID8W { w: self }
    }
    #[doc = "Bit 9 - violation information valid flag for AHB layer 9. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid9(&mut self) -> _VIO_INFO_VALID9W {
        _VIO_INFO_VALID9W { w: self }
    }
    #[doc = "Bit 10 - violation information valid flag for AHB layer 10. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid10(&mut self) -> _VIO_INFO_VALID10W {
        _VIO_INFO_VALID10W { w: self }
    }
    #[doc = "Bit 11 - violation information valid flag for AHB layer 11. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid11(&mut self) -> _VIO_INFO_VALID11W {
        _VIO_INFO_VALID11W { w: self }
    }
    #[doc = "Bit 12 - violation information valid flag for AHB layer 12. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid12(&mut self) -> _VIO_INFO_VALID12W {
        _VIO_INFO_VALID12W { w: self }
    }
    #[doc = "Bit 13 - violation information valid flag for AHB layer 13. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid13(&mut self) -> _VIO_INFO_VALID13W {
        _VIO_INFO_VALID13W { w: self }
    }
    #[doc = "Bit 14 - violation information valid flag for AHB layer 14. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid14(&mut self) -> _VIO_INFO_VALID14W {
        _VIO_INFO_VALID14W { w: self }
    }
    #[doc = "Bit 15 - violation information valid flag for AHB layer 15. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid15(&mut self) -> _VIO_INFO_VALID15W {
        _VIO_INFO_VALID15W { w: self }
    }
    #[doc = "Bit 16 - violation information valid flag for AHB layer 16. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid16(&mut self) -> _VIO_INFO_VALID16W {
        _VIO_INFO_VALID16W { w: self }
    }
    #[doc = "Bit 17 - violation information valid flag for AHB layer 17. 0: not valid. 1: valid (violation occurred). Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid17(&mut self) -> _VIO_INFO_VALID17W {
        _VIO_INFO_VALID17W { w: self }
    }
}
