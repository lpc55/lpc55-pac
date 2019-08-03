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
#[doc = "Possible values of the field `VIO_INFO_VALID0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID0R {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID0R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VIO_INFO_VALID0R::NOT_VALID => false,
            VIO_INFO_VALID0R::VALID => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VIO_INFO_VALID0R {
        match value {
            false => VIO_INFO_VALID0R::NOT_VALID,
            true => VIO_INFO_VALID0R::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID0R::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID0R::VALID
    }
}
#[doc = "Possible values of the field `VIO_INFO_VALID1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID1R {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID1R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VIO_INFO_VALID1R::NOT_VALID => false,
            VIO_INFO_VALID1R::VALID => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VIO_INFO_VALID1R {
        match value {
            false => VIO_INFO_VALID1R::NOT_VALID,
            true => VIO_INFO_VALID1R::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID1R::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID1R::VALID
    }
}
#[doc = "Possible values of the field `VIO_INFO_VALID2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID2R {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID2R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VIO_INFO_VALID2R::NOT_VALID => false,
            VIO_INFO_VALID2R::VALID => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VIO_INFO_VALID2R {
        match value {
            false => VIO_INFO_VALID2R::NOT_VALID,
            true => VIO_INFO_VALID2R::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID2R::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID2R::VALID
    }
}
#[doc = "Possible values of the field `VIO_INFO_VALID3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID3R {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID3R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VIO_INFO_VALID3R::NOT_VALID => false,
            VIO_INFO_VALID3R::VALID => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VIO_INFO_VALID3R {
        match value {
            false => VIO_INFO_VALID3R::NOT_VALID,
            true => VIO_INFO_VALID3R::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID3R::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID3R::VALID
    }
}
#[doc = "Possible values of the field `VIO_INFO_VALID4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID4R {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID4R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VIO_INFO_VALID4R::NOT_VALID => false,
            VIO_INFO_VALID4R::VALID => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VIO_INFO_VALID4R {
        match value {
            false => VIO_INFO_VALID4R::NOT_VALID,
            true => VIO_INFO_VALID4R::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID4R::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID4R::VALID
    }
}
#[doc = "Possible values of the field `VIO_INFO_VALID5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID5R {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID5R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VIO_INFO_VALID5R::NOT_VALID => false,
            VIO_INFO_VALID5R::VALID => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VIO_INFO_VALID5R {
        match value {
            false => VIO_INFO_VALID5R::NOT_VALID,
            true => VIO_INFO_VALID5R::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID5R::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID5R::VALID
    }
}
#[doc = "Possible values of the field `VIO_INFO_VALID6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID6R {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID6R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VIO_INFO_VALID6R::NOT_VALID => false,
            VIO_INFO_VALID6R::VALID => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VIO_INFO_VALID6R {
        match value {
            false => VIO_INFO_VALID6R::NOT_VALID,
            true => VIO_INFO_VALID6R::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID6R::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID6R::VALID
    }
}
#[doc = "Possible values of the field `VIO_INFO_VALID7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID7R {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID7R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VIO_INFO_VALID7R::NOT_VALID => false,
            VIO_INFO_VALID7R::VALID => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VIO_INFO_VALID7R {
        match value {
            false => VIO_INFO_VALID7R::NOT_VALID,
            true => VIO_INFO_VALID7R::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID7R::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID7R::VALID
    }
}
#[doc = "Possible values of the field `VIO_INFO_VALID8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID8R {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID8R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VIO_INFO_VALID8R::NOT_VALID => false,
            VIO_INFO_VALID8R::VALID => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VIO_INFO_VALID8R {
        match value {
            false => VIO_INFO_VALID8R::NOT_VALID,
            true => VIO_INFO_VALID8R::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID8R::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID8R::VALID
    }
}
#[doc = "Possible values of the field `VIO_INFO_VALID9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID9R {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID9R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VIO_INFO_VALID9R::NOT_VALID => false,
            VIO_INFO_VALID9R::VALID => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VIO_INFO_VALID9R {
        match value {
            false => VIO_INFO_VALID9R::NOT_VALID,
            true => VIO_INFO_VALID9R::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID9R::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID9R::VALID
    }
}
#[doc = "Possible values of the field `VIO_INFO_VALID10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID10R {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID10R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VIO_INFO_VALID10R::NOT_VALID => false,
            VIO_INFO_VALID10R::VALID => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VIO_INFO_VALID10R {
        match value {
            false => VIO_INFO_VALID10R::NOT_VALID,
            true => VIO_INFO_VALID10R::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID10R::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID10R::VALID
    }
}
#[doc = "Possible values of the field `VIO_INFO_VALID11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID11R {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID11R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VIO_INFO_VALID11R::NOT_VALID => false,
            VIO_INFO_VALID11R::VALID => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VIO_INFO_VALID11R {
        match value {
            false => VIO_INFO_VALID11R::NOT_VALID,
            true => VIO_INFO_VALID11R::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID11R::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID11R::VALID
    }
}
#[doc = "Values that can be written to the field `VIO_INFO_VALID0`"]
pub enum VIO_INFO_VALID0W {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VIO_INFO_VALID0W::NOT_VALID => false,
            VIO_INFO_VALID0W::VALID => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID0W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VIO_INFO_VALID0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not valid."]
    #[inline]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID0W::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID0W::VALID)
    }
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
#[doc = "Values that can be written to the field `VIO_INFO_VALID1`"]
pub enum VIO_INFO_VALID1W {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VIO_INFO_VALID1W::NOT_VALID => false,
            VIO_INFO_VALID1W::VALID => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID1W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VIO_INFO_VALID1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not valid."]
    #[inline]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID1W::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID1W::VALID)
    }
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
#[doc = "Values that can be written to the field `VIO_INFO_VALID2`"]
pub enum VIO_INFO_VALID2W {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VIO_INFO_VALID2W::NOT_VALID => false,
            VIO_INFO_VALID2W::VALID => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID2W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VIO_INFO_VALID2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not valid."]
    #[inline]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID2W::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID2W::VALID)
    }
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
#[doc = "Values that can be written to the field `VIO_INFO_VALID3`"]
pub enum VIO_INFO_VALID3W {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VIO_INFO_VALID3W::NOT_VALID => false,
            VIO_INFO_VALID3W::VALID => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID3W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VIO_INFO_VALID3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not valid."]
    #[inline]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID3W::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID3W::VALID)
    }
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
#[doc = "Values that can be written to the field `VIO_INFO_VALID4`"]
pub enum VIO_INFO_VALID4W {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VIO_INFO_VALID4W::NOT_VALID => false,
            VIO_INFO_VALID4W::VALID => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID4W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VIO_INFO_VALID4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not valid."]
    #[inline]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID4W::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID4W::VALID)
    }
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
#[doc = "Values that can be written to the field `VIO_INFO_VALID5`"]
pub enum VIO_INFO_VALID5W {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VIO_INFO_VALID5W::NOT_VALID => false,
            VIO_INFO_VALID5W::VALID => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID5W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VIO_INFO_VALID5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not valid."]
    #[inline]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID5W::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID5W::VALID)
    }
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
#[doc = "Values that can be written to the field `VIO_INFO_VALID6`"]
pub enum VIO_INFO_VALID6W {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VIO_INFO_VALID6W::NOT_VALID => false,
            VIO_INFO_VALID6W::VALID => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID6W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VIO_INFO_VALID6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not valid."]
    #[inline]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID6W::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID6W::VALID)
    }
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
#[doc = "Values that can be written to the field `VIO_INFO_VALID7`"]
pub enum VIO_INFO_VALID7W {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VIO_INFO_VALID7W::NOT_VALID => false,
            VIO_INFO_VALID7W::VALID => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID7W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VIO_INFO_VALID7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not valid."]
    #[inline]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID7W::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID7W::VALID)
    }
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
#[doc = "Values that can be written to the field `VIO_INFO_VALID8`"]
pub enum VIO_INFO_VALID8W {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VIO_INFO_VALID8W::NOT_VALID => false,
            VIO_INFO_VALID8W::VALID => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID8W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VIO_INFO_VALID8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not valid."]
    #[inline]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID8W::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID8W::VALID)
    }
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
#[doc = "Values that can be written to the field `VIO_INFO_VALID9`"]
pub enum VIO_INFO_VALID9W {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VIO_INFO_VALID9W::NOT_VALID => false,
            VIO_INFO_VALID9W::VALID => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID9W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VIO_INFO_VALID9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not valid."]
    #[inline]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID9W::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID9W::VALID)
    }
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
#[doc = "Values that can be written to the field `VIO_INFO_VALID10`"]
pub enum VIO_INFO_VALID10W {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VIO_INFO_VALID10W::NOT_VALID => false,
            VIO_INFO_VALID10W::VALID => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID10W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VIO_INFO_VALID10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not valid."]
    #[inline]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID10W::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID10W::VALID)
    }
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
#[doc = "Values that can be written to the field `VIO_INFO_VALID11`"]
pub enum VIO_INFO_VALID11W {
    #[doc = "Not valid."]
    NOT_VALID,
    #[doc = "Valid (violation occurred)."]
    VALID,
}
impl VIO_INFO_VALID11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VIO_INFO_VALID11W::NOT_VALID => false,
            VIO_INFO_VALID11W::VALID => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VIO_INFO_VALID11W<'a> {
    w: &'a mut W,
}
impl<'a> _VIO_INFO_VALID11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VIO_INFO_VALID11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not valid."]
    #[inline]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID11W::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID11W::VALID)
    }
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - violation information valid flag for AHB layer 0. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid0(&self) -> VIO_INFO_VALID0R {
        VIO_INFO_VALID0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - violation information valid flag for AHB layer 1. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid1(&self) -> VIO_INFO_VALID1R {
        VIO_INFO_VALID1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - violation information valid flag for AHB layer 2. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid2(&self) -> VIO_INFO_VALID2R {
        VIO_INFO_VALID2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - violation information valid flag for AHB layer 3. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid3(&self) -> VIO_INFO_VALID3R {
        VIO_INFO_VALID3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - violation information valid flag for AHB layer 4. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid4(&self) -> VIO_INFO_VALID4R {
        VIO_INFO_VALID4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - violation information valid flag for AHB layer 5. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid5(&self) -> VIO_INFO_VALID5R {
        VIO_INFO_VALID5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - violation information valid flag for AHB layer 6. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid6(&self) -> VIO_INFO_VALID6R {
        VIO_INFO_VALID6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - violation information valid flag for AHB layer 7. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid7(&self) -> VIO_INFO_VALID7R {
        VIO_INFO_VALID7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - violation information valid flag for AHB layer 8. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid8(&self) -> VIO_INFO_VALID8R {
        VIO_INFO_VALID8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - violation information valid flag for AHB layer 9. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid9(&self) -> VIO_INFO_VALID9R {
        VIO_INFO_VALID9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - violation information valid flag for AHB layer 10. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid10(&self) -> VIO_INFO_VALID10R {
        VIO_INFO_VALID10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - violation information valid flag for AHB layer 11. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid11(&self) -> VIO_INFO_VALID11R {
        VIO_INFO_VALID11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 0 - violation information valid flag for AHB layer 0. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid0(&mut self) -> _VIO_INFO_VALID0W {
        _VIO_INFO_VALID0W { w: self }
    }
    #[doc = "Bit 1 - violation information valid flag for AHB layer 1. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid1(&mut self) -> _VIO_INFO_VALID1W {
        _VIO_INFO_VALID1W { w: self }
    }
    #[doc = "Bit 2 - violation information valid flag for AHB layer 2. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid2(&mut self) -> _VIO_INFO_VALID2W {
        _VIO_INFO_VALID2W { w: self }
    }
    #[doc = "Bit 3 - violation information valid flag for AHB layer 3. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid3(&mut self) -> _VIO_INFO_VALID3W {
        _VIO_INFO_VALID3W { w: self }
    }
    #[doc = "Bit 4 - violation information valid flag for AHB layer 4. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid4(&mut self) -> _VIO_INFO_VALID4W {
        _VIO_INFO_VALID4W { w: self }
    }
    #[doc = "Bit 5 - violation information valid flag for AHB layer 5. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid5(&mut self) -> _VIO_INFO_VALID5W {
        _VIO_INFO_VALID5W { w: self }
    }
    #[doc = "Bit 6 - violation information valid flag for AHB layer 6. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid6(&mut self) -> _VIO_INFO_VALID6W {
        _VIO_INFO_VALID6W { w: self }
    }
    #[doc = "Bit 7 - violation information valid flag for AHB layer 7. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid7(&mut self) -> _VIO_INFO_VALID7W {
        _VIO_INFO_VALID7W { w: self }
    }
    #[doc = "Bit 8 - violation information valid flag for AHB layer 8. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid8(&mut self) -> _VIO_INFO_VALID8W {
        _VIO_INFO_VALID8W { w: self }
    }
    #[doc = "Bit 9 - violation information valid flag for AHB layer 9. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid9(&mut self) -> _VIO_INFO_VALID9W {
        _VIO_INFO_VALID9W { w: self }
    }
    #[doc = "Bit 10 - violation information valid flag for AHB layer 10. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid10(&mut self) -> _VIO_INFO_VALID10W {
        _VIO_INFO_VALID10W { w: self }
    }
    #[doc = "Bit 11 - violation information valid flag for AHB layer 11. Write 1 to clear."]
    #[inline]
    pub fn vio_info_valid11(&mut self) -> _VIO_INFO_VALID11W {
        _VIO_INFO_VALID11W { w: self }
    }
}
