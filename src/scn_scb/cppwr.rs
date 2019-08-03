#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CPPWR {
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
#[doc = "Possible values of the field `SU0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SU0R {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED,
}
impl SU0R {
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
            SU0R::UNKNOWN_NOT_PERMITTED => false,
            SU0R::UNKNOWN_PERMITTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SU0R {
        match value {
            false => SU0R::UNKNOWN_NOT_PERMITTED,
            true => SU0R::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline]
    pub fn is_unknown_not_permitted(&self) -> bool {
        *self == SU0R::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU0R::UNKNOWN_PERMITTED
    }
}
#[doc = "Possible values of the field `SUS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUS0R {
    #[doc = "The SU0 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE,
    #[doc = "The SU0 field is only accessible from the Secure state."]
    SECURE_ONLY,
}
impl SUS0R {
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
            SUS0R::SECURE_AND_NON_SECURE => false,
            SUS0R::SECURE_ONLY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUS0R {
        match value {
            false => SUS0R::SECURE_AND_NON_SECURE,
            true => SUS0R::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline]
    pub fn is_secure_and_non_secure(&self) -> bool {
        *self == SUS0R::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS0R::SECURE_ONLY
    }
}
#[doc = "Possible values of the field `SU1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SU1R {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED,
}
impl SU1R {
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
            SU1R::UNKNOWN_NOT_PERMITTED => false,
            SU1R::UNKNOWN_PERMITTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SU1R {
        match value {
            false => SU1R::UNKNOWN_NOT_PERMITTED,
            true => SU1R::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline]
    pub fn is_unknown_not_permitted(&self) -> bool {
        *self == SU1R::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU1R::UNKNOWN_PERMITTED
    }
}
#[doc = "Possible values of the field `SUS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUS1R {
    #[doc = "The SU7 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE,
    #[doc = "The SU7 field is only accessible from the Secure state."]
    SECURE_ONLY,
}
impl SUS1R {
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
            SUS1R::SECURE_AND_NON_SECURE => false,
            SUS1R::SECURE_ONLY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUS1R {
        match value {
            false => SUS1R::SECURE_AND_NON_SECURE,
            true => SUS1R::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline]
    pub fn is_secure_and_non_secure(&self) -> bool {
        *self == SUS1R::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS1R::SECURE_ONLY
    }
}
#[doc = "Possible values of the field `SU2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SU2R {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED,
}
impl SU2R {
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
            SU2R::UNKNOWN_NOT_PERMITTED => false,
            SU2R::UNKNOWN_PERMITTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SU2R {
        match value {
            false => SU2R::UNKNOWN_NOT_PERMITTED,
            true => SU2R::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline]
    pub fn is_unknown_not_permitted(&self) -> bool {
        *self == SU2R::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU2R::UNKNOWN_PERMITTED
    }
}
#[doc = "Possible values of the field `SUS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUS2R {
    #[doc = "The SU2 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE,
    #[doc = "The SU2 field is only accessible from the Secure state."]
    SECURE_ONLY,
}
impl SUS2R {
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
            SUS2R::SECURE_AND_NON_SECURE => false,
            SUS2R::SECURE_ONLY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUS2R {
        match value {
            false => SUS2R::SECURE_AND_NON_SECURE,
            true => SUS2R::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline]
    pub fn is_secure_and_non_secure(&self) -> bool {
        *self == SUS2R::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS2R::SECURE_ONLY
    }
}
#[doc = "Possible values of the field `SU3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SU3R {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED,
}
impl SU3R {
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
            SU3R::UNKNOWN_NOT_PERMITTED => false,
            SU3R::UNKNOWN_PERMITTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SU3R {
        match value {
            false => SU3R::UNKNOWN_NOT_PERMITTED,
            true => SU3R::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline]
    pub fn is_unknown_not_permitted(&self) -> bool {
        *self == SU3R::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU3R::UNKNOWN_PERMITTED
    }
}
#[doc = "Possible values of the field `SUS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUS3R {
    #[doc = "The SU3 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE,
    #[doc = "The SU3 field is only accessible from the Secure state."]
    SECURE_ONLY,
}
impl SUS3R {
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
            SUS3R::SECURE_AND_NON_SECURE => false,
            SUS3R::SECURE_ONLY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUS3R {
        match value {
            false => SUS3R::SECURE_AND_NON_SECURE,
            true => SUS3R::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline]
    pub fn is_secure_and_non_secure(&self) -> bool {
        *self == SUS3R::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS3R::SECURE_ONLY
    }
}
#[doc = "Possible values of the field `SU4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SU4R {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED,
}
impl SU4R {
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
            SU4R::UNKNOWN_NOT_PERMITTED => false,
            SU4R::UNKNOWN_PERMITTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SU4R {
        match value {
            false => SU4R::UNKNOWN_NOT_PERMITTED,
            true => SU4R::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline]
    pub fn is_unknown_not_permitted(&self) -> bool {
        *self == SU4R::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU4R::UNKNOWN_PERMITTED
    }
}
#[doc = "Possible values of the field `SUS4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUS4R {
    #[doc = "The SU4 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE,
    #[doc = "The SU4 field is only accessible from the Secure state."]
    SECURE_ONLY,
}
impl SUS4R {
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
            SUS4R::SECURE_AND_NON_SECURE => false,
            SUS4R::SECURE_ONLY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUS4R {
        match value {
            false => SUS4R::SECURE_AND_NON_SECURE,
            true => SUS4R::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline]
    pub fn is_secure_and_non_secure(&self) -> bool {
        *self == SUS4R::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS4R::SECURE_ONLY
    }
}
#[doc = "Possible values of the field `SU5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SU5R {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED,
}
impl SU5R {
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
            SU5R::UNKNOWN_NOT_PERMITTED => false,
            SU5R::UNKNOWN_PERMITTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SU5R {
        match value {
            false => SU5R::UNKNOWN_NOT_PERMITTED,
            true => SU5R::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline]
    pub fn is_unknown_not_permitted(&self) -> bool {
        *self == SU5R::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU5R::UNKNOWN_PERMITTED
    }
}
#[doc = "Possible values of the field `SUS5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUS5R {
    #[doc = "The SU5 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE,
    #[doc = "The SU5 field is only accessible from the Secure state."]
    SECURE_ONLY,
}
impl SUS5R {
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
            SUS5R::SECURE_AND_NON_SECURE => false,
            SUS5R::SECURE_ONLY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUS5R {
        match value {
            false => SUS5R::SECURE_AND_NON_SECURE,
            true => SUS5R::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline]
    pub fn is_secure_and_non_secure(&self) -> bool {
        *self == SUS5R::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS5R::SECURE_ONLY
    }
}
#[doc = "Possible values of the field `SU6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SU6R {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED,
}
impl SU6R {
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
            SU6R::UNKNOWN_NOT_PERMITTED => false,
            SU6R::UNKNOWN_PERMITTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SU6R {
        match value {
            false => SU6R::UNKNOWN_NOT_PERMITTED,
            true => SU6R::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline]
    pub fn is_unknown_not_permitted(&self) -> bool {
        *self == SU6R::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU6R::UNKNOWN_PERMITTED
    }
}
#[doc = "Possible values of the field `SUS6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUS6R {
    #[doc = "The SU6 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE,
    #[doc = "The SU6 field is only accessible from the Secure state."]
    SECURE_ONLY,
}
impl SUS6R {
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
            SUS6R::SECURE_AND_NON_SECURE => false,
            SUS6R::SECURE_ONLY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUS6R {
        match value {
            false => SUS6R::SECURE_AND_NON_SECURE,
            true => SUS6R::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline]
    pub fn is_secure_and_non_secure(&self) -> bool {
        *self == SUS6R::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS6R::SECURE_ONLY
    }
}
#[doc = "Possible values of the field `SU7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SU7R {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED,
}
impl SU7R {
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
            SU7R::UNKNOWN_NOT_PERMITTED => false,
            SU7R::UNKNOWN_PERMITTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SU7R {
        match value {
            false => SU7R::UNKNOWN_NOT_PERMITTED,
            true => SU7R::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline]
    pub fn is_unknown_not_permitted(&self) -> bool {
        *self == SU7R::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU7R::UNKNOWN_PERMITTED
    }
}
#[doc = "Possible values of the field `SUS7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUS7R {
    #[doc = "The SU7 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE,
    #[doc = "The SU7 field is only accessible from the Secure state."]
    SECURE_ONLY,
}
impl SUS7R {
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
            SUS7R::SECURE_AND_NON_SECURE => false,
            SUS7R::SECURE_ONLY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUS7R {
        match value {
            false => SUS7R::SECURE_AND_NON_SECURE,
            true => SUS7R::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline]
    pub fn is_secure_and_non_secure(&self) -> bool {
        *self == SUS7R::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS7R::SECURE_ONLY
    }
}
#[doc = "Possible values of the field `SU10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SU10R {
    #[doc = "The floating-point state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED,
    #[doc = "The floating-point state is permitted to become UNKNOWN"]
    UNKNOWN_PERMITTED,
}
impl SU10R {
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
            SU10R::UNKNOWN_NOT_PERMITTED => false,
            SU10R::UNKNOWN_PERMITTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SU10R {
        match value {
            false => SU10R::UNKNOWN_NOT_PERMITTED,
            true => SU10R::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline]
    pub fn is_unknown_not_permitted(&self) -> bool {
        *self == SU10R::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU10R::UNKNOWN_PERMITTED
    }
}
#[doc = "Possible values of the field `SUS10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUS10R {
    #[doc = "The SU10 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE,
    #[doc = "The SU10 field is only accessible from the Secure state."]
    SECURE_ONLY,
}
impl SUS10R {
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
            SUS10R::SECURE_AND_NON_SECURE => false,
            SUS10R::SECURE_ONLY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUS10R {
        match value {
            false => SUS10R::SECURE_AND_NON_SECURE,
            true => SUS10R::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline]
    pub fn is_secure_and_non_secure(&self) -> bool {
        *self == SUS10R::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS10R::SECURE_ONLY
    }
}
#[doc = r" Value of the field"]
pub struct SU11R {
    bits: bool,
}
impl SU11R {
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
pub struct SUS11R {
    bits: bool,
}
impl SUS11R {
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
#[doc = "Values that can be written to the field `SU0`"]
pub enum SU0W {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED,
}
impl SU0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SU0W::UNKNOWN_NOT_PERMITTED => false,
            SU0W::UNKNOWN_PERMITTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SU0W<'a> {
    w: &'a mut W,
}
impl<'a> _SU0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SU0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU0W::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU0W::UNKNOWN_PERMITTED)
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
#[doc = "Values that can be written to the field `SUS0`"]
pub enum SUS0W {
    #[doc = "The SU0 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE,
    #[doc = "The SU0 field is only accessible from the Secure state."]
    SECURE_ONLY,
}
impl SUS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUS0W::SECURE_AND_NON_SECURE => false,
            SUS0W::SECURE_ONLY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUS0W<'a> {
    w: &'a mut W,
}
impl<'a> _SUS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SU0 field is accessible from both Security states."]
    #[inline]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS0W::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU0 field is only accessible from the Secure state."]
    #[inline]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS0W::SECURE_ONLY)
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
#[doc = "Values that can be written to the field `SU1`"]
pub enum SU1W {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED,
}
impl SU1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SU1W::UNKNOWN_NOT_PERMITTED => false,
            SU1W::UNKNOWN_PERMITTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SU1W<'a> {
    w: &'a mut W,
}
impl<'a> _SU1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SU1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU1W::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU1W::UNKNOWN_PERMITTED)
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
#[doc = "Values that can be written to the field `SUS1`"]
pub enum SUS1W {
    #[doc = "The SU7 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE,
    #[doc = "The SU7 field is only accessible from the Secure state."]
    SECURE_ONLY,
}
impl SUS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUS1W::SECURE_AND_NON_SECURE => false,
            SUS1W::SECURE_ONLY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUS1W<'a> {
    w: &'a mut W,
}
impl<'a> _SUS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SU7 field is accessible from both Security states."]
    #[inline]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS1W::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU7 field is only accessible from the Secure state."]
    #[inline]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS1W::SECURE_ONLY)
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
#[doc = "Values that can be written to the field `SU2`"]
pub enum SU2W {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED,
}
impl SU2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SU2W::UNKNOWN_NOT_PERMITTED => false,
            SU2W::UNKNOWN_PERMITTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SU2W<'a> {
    w: &'a mut W,
}
impl<'a> _SU2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SU2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU2W::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU2W::UNKNOWN_PERMITTED)
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
#[doc = "Values that can be written to the field `SUS2`"]
pub enum SUS2W {
    #[doc = "The SU2 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE,
    #[doc = "The SU2 field is only accessible from the Secure state."]
    SECURE_ONLY,
}
impl SUS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUS2W::SECURE_AND_NON_SECURE => false,
            SUS2W::SECURE_ONLY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUS2W<'a> {
    w: &'a mut W,
}
impl<'a> _SUS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUS2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SU2 field is accessible from both Security states."]
    #[inline]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS2W::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU2 field is only accessible from the Secure state."]
    #[inline]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS2W::SECURE_ONLY)
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
#[doc = "Values that can be written to the field `SU3`"]
pub enum SU3W {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED,
}
impl SU3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SU3W::UNKNOWN_NOT_PERMITTED => false,
            SU3W::UNKNOWN_PERMITTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SU3W<'a> {
    w: &'a mut W,
}
impl<'a> _SU3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SU3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU3W::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU3W::UNKNOWN_PERMITTED)
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
#[doc = "Values that can be written to the field `SUS3`"]
pub enum SUS3W {
    #[doc = "The SU3 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE,
    #[doc = "The SU3 field is only accessible from the Secure state."]
    SECURE_ONLY,
}
impl SUS3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUS3W::SECURE_AND_NON_SECURE => false,
            SUS3W::SECURE_ONLY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUS3W<'a> {
    w: &'a mut W,
}
impl<'a> _SUS3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUS3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SU3 field is accessible from both Security states."]
    #[inline]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS3W::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU3 field is only accessible from the Secure state."]
    #[inline]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS3W::SECURE_ONLY)
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
#[doc = "Values that can be written to the field `SU4`"]
pub enum SU4W {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED,
}
impl SU4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SU4W::UNKNOWN_NOT_PERMITTED => false,
            SU4W::UNKNOWN_PERMITTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SU4W<'a> {
    w: &'a mut W,
}
impl<'a> _SU4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SU4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU4W::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU4W::UNKNOWN_PERMITTED)
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
#[doc = "Values that can be written to the field `SUS4`"]
pub enum SUS4W {
    #[doc = "The SU4 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE,
    #[doc = "The SU4 field is only accessible from the Secure state."]
    SECURE_ONLY,
}
impl SUS4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUS4W::SECURE_AND_NON_SECURE => false,
            SUS4W::SECURE_ONLY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUS4W<'a> {
    w: &'a mut W,
}
impl<'a> _SUS4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUS4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SU4 field is accessible from both Security states."]
    #[inline]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS4W::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU4 field is only accessible from the Secure state."]
    #[inline]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS4W::SECURE_ONLY)
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
#[doc = "Values that can be written to the field `SU5`"]
pub enum SU5W {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED,
}
impl SU5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SU5W::UNKNOWN_NOT_PERMITTED => false,
            SU5W::UNKNOWN_PERMITTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SU5W<'a> {
    w: &'a mut W,
}
impl<'a> _SU5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SU5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU5W::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU5W::UNKNOWN_PERMITTED)
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
#[doc = "Values that can be written to the field `SUS5`"]
pub enum SUS5W {
    #[doc = "The SU5 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE,
    #[doc = "The SU5 field is only accessible from the Secure state."]
    SECURE_ONLY,
}
impl SUS5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUS5W::SECURE_AND_NON_SECURE => false,
            SUS5W::SECURE_ONLY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUS5W<'a> {
    w: &'a mut W,
}
impl<'a> _SUS5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUS5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SU5 field is accessible from both Security states."]
    #[inline]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS5W::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU5 field is only accessible from the Secure state."]
    #[inline]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS5W::SECURE_ONLY)
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
#[doc = "Values that can be written to the field `SU6`"]
pub enum SU6W {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED,
}
impl SU6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SU6W::UNKNOWN_NOT_PERMITTED => false,
            SU6W::UNKNOWN_PERMITTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SU6W<'a> {
    w: &'a mut W,
}
impl<'a> _SU6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SU6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU6W::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU6W::UNKNOWN_PERMITTED)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SUS6`"]
pub enum SUS6W {
    #[doc = "The SU6 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE,
    #[doc = "The SU6 field is only accessible from the Secure state."]
    SECURE_ONLY,
}
impl SUS6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUS6W::SECURE_AND_NON_SECURE => false,
            SUS6W::SECURE_ONLY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUS6W<'a> {
    w: &'a mut W,
}
impl<'a> _SUS6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUS6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SU6 field is accessible from both Security states."]
    #[inline]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS6W::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU6 field is only accessible from the Secure state."]
    #[inline]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS6W::SECURE_ONLY)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SU7`"]
pub enum SU7W {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED,
}
impl SU7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SU7W::UNKNOWN_NOT_PERMITTED => false,
            SU7W::UNKNOWN_PERMITTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SU7W<'a> {
    w: &'a mut W,
}
impl<'a> _SU7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SU7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU7W::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU7W::UNKNOWN_PERMITTED)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SUS7`"]
pub enum SUS7W {
    #[doc = "The SU7 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE,
    #[doc = "The SU7 field is only accessible from the Secure state."]
    SECURE_ONLY,
}
impl SUS7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUS7W::SECURE_AND_NON_SECURE => false,
            SUS7W::SECURE_ONLY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUS7W<'a> {
    w: &'a mut W,
}
impl<'a> _SUS7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUS7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SU7 field is accessible from both Security states."]
    #[inline]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS7W::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU7 field is only accessible from the Secure state."]
    #[inline]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS7W::SECURE_ONLY)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SU10`"]
pub enum SU10W {
    #[doc = "The floating-point state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED,
    #[doc = "The floating-point state is permitted to become UNKNOWN"]
    UNKNOWN_PERMITTED,
}
impl SU10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SU10W::UNKNOWN_NOT_PERMITTED => false,
            SU10W::UNKNOWN_PERMITTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SU10W<'a> {
    w: &'a mut W,
}
impl<'a> _SU10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SU10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The floating-point state is not permitted to become UNKNOWN."]
    #[inline]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU10W::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The floating-point state is permitted to become UNKNOWN"]
    #[inline]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU10W::UNKNOWN_PERMITTED)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SUS10`"]
pub enum SUS10W {
    #[doc = "The SU10 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE,
    #[doc = "The SU10 field is only accessible from the Secure state."]
    SECURE_ONLY,
}
impl SUS10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUS10W::SECURE_AND_NON_SECURE => false,
            SUS10W::SECURE_ONLY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUS10W<'a> {
    w: &'a mut W,
}
impl<'a> _SUS10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUS10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SU10 field is accessible from both Security states."]
    #[inline]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS10W::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU10 field is only accessible from the Secure state."]
    #[inline]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS10W::SECURE_ONLY)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SU11W<'a> {
    w: &'a mut W,
}
impl<'a> _SU11W<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SUS11W<'a> {
    w: &'a mut W,
}
impl<'a> _SUS11W<'a> {
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
        const OFFSET: u8 = 23;
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
    #[doc = "Bit 0 - State UNKNOWN 0."]
    #[inline]
    pub fn su0(&self) -> SU0R {
        SU0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - State UNKNOWN Secure only 0."]
    #[inline]
    pub fn sus0(&self) -> SUS0R {
        SUS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - State UNKNOWN 1."]
    #[inline]
    pub fn su1(&self) -> SU1R {
        SU1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - State UNKNOWN Secure only 1."]
    #[inline]
    pub fn sus1(&self) -> SUS1R {
        SUS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - State UNKNOWN 2."]
    #[inline]
    pub fn su2(&self) -> SU2R {
        SU2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - State UNKNOWN Secure only 2."]
    #[inline]
    pub fn sus2(&self) -> SUS2R {
        SUS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - State UNKNOWN 3."]
    #[inline]
    pub fn su3(&self) -> SU3R {
        SU3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - State UNKNOWN Secure only 3."]
    #[inline]
    pub fn sus3(&self) -> SUS3R {
        SUS3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - State UNKNOWN 4."]
    #[inline]
    pub fn su4(&self) -> SU4R {
        SU4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - State UNKNOWN Secure only 4."]
    #[inline]
    pub fn sus4(&self) -> SUS4R {
        SUS4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - State UNKNOWN 5."]
    #[inline]
    pub fn su5(&self) -> SU5R {
        SU5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - State UNKNOWN Secure only 5."]
    #[inline]
    pub fn sus5(&self) -> SUS5R {
        SUS5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - State UNKNOWN 6."]
    #[inline]
    pub fn su6(&self) -> SU6R {
        SU6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - State UNKNOWN Secure only 6."]
    #[inline]
    pub fn sus6(&self) -> SUS6R {
        SUS6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - State UNKNOWN 7."]
    #[inline]
    pub fn su7(&self) -> SU7R {
        SU7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - State UNKNOWN Secure only 7."]
    #[inline]
    pub fn sus7(&self) -> SUS7R {
        SUS7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - State UNKNOWN 10."]
    #[inline]
    pub fn su10(&self) -> SU10R {
        SU10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - State UNKNOWN Secure only 10."]
    #[inline]
    pub fn sus10(&self) -> SUS10R {
        SUS10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - State UNKNOWN 11."]
    #[inline]
    pub fn su11(&self) -> SU11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SU11R { bits }
    }
    #[doc = "Bit 23 - State UNKNOWN Secure only 11."]
    #[inline]
    pub fn sus11(&self) -> SUS11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SUS11R { bits }
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
    #[doc = "Bit 0 - State UNKNOWN 0."]
    #[inline]
    pub fn su0(&mut self) -> _SU0W {
        _SU0W { w: self }
    }
    #[doc = "Bit 1 - State UNKNOWN Secure only 0."]
    #[inline]
    pub fn sus0(&mut self) -> _SUS0W {
        _SUS0W { w: self }
    }
    #[doc = "Bit 2 - State UNKNOWN 1."]
    #[inline]
    pub fn su1(&mut self) -> _SU1W {
        _SU1W { w: self }
    }
    #[doc = "Bit 3 - State UNKNOWN Secure only 1."]
    #[inline]
    pub fn sus1(&mut self) -> _SUS1W {
        _SUS1W { w: self }
    }
    #[doc = "Bit 4 - State UNKNOWN 2."]
    #[inline]
    pub fn su2(&mut self) -> _SU2W {
        _SU2W { w: self }
    }
    #[doc = "Bit 5 - State UNKNOWN Secure only 2."]
    #[inline]
    pub fn sus2(&mut self) -> _SUS2W {
        _SUS2W { w: self }
    }
    #[doc = "Bit 6 - State UNKNOWN 3."]
    #[inline]
    pub fn su3(&mut self) -> _SU3W {
        _SU3W { w: self }
    }
    #[doc = "Bit 7 - State UNKNOWN Secure only 3."]
    #[inline]
    pub fn sus3(&mut self) -> _SUS3W {
        _SUS3W { w: self }
    }
    #[doc = "Bit 8 - State UNKNOWN 4."]
    #[inline]
    pub fn su4(&mut self) -> _SU4W {
        _SU4W { w: self }
    }
    #[doc = "Bit 9 - State UNKNOWN Secure only 4."]
    #[inline]
    pub fn sus4(&mut self) -> _SUS4W {
        _SUS4W { w: self }
    }
    #[doc = "Bit 10 - State UNKNOWN 5."]
    #[inline]
    pub fn su5(&mut self) -> _SU5W {
        _SU5W { w: self }
    }
    #[doc = "Bit 11 - State UNKNOWN Secure only 5."]
    #[inline]
    pub fn sus5(&mut self) -> _SUS5W {
        _SUS5W { w: self }
    }
    #[doc = "Bit 12 - State UNKNOWN 6."]
    #[inline]
    pub fn su6(&mut self) -> _SU6W {
        _SU6W { w: self }
    }
    #[doc = "Bit 13 - State UNKNOWN Secure only 6."]
    #[inline]
    pub fn sus6(&mut self) -> _SUS6W {
        _SUS6W { w: self }
    }
    #[doc = "Bit 14 - State UNKNOWN 7."]
    #[inline]
    pub fn su7(&mut self) -> _SU7W {
        _SU7W { w: self }
    }
    #[doc = "Bit 15 - State UNKNOWN Secure only 7."]
    #[inline]
    pub fn sus7(&mut self) -> _SUS7W {
        _SUS7W { w: self }
    }
    #[doc = "Bit 20 - State UNKNOWN 10."]
    #[inline]
    pub fn su10(&mut self) -> _SU10W {
        _SU10W { w: self }
    }
    #[doc = "Bit 21 - State UNKNOWN Secure only 10."]
    #[inline]
    pub fn sus10(&mut self) -> _SUS10W {
        _SUS10W { w: self }
    }
    #[doc = "Bit 22 - State UNKNOWN 11."]
    #[inline]
    pub fn su11(&mut self) -> _SU11W {
        _SU11W { w: self }
    }
    #[doc = "Bit 23 - State UNKNOWN Secure only 11."]
    #[inline]
    pub fn sus11(&mut self) -> _SUS11W {
        _SUS11W { w: self }
    }
}
