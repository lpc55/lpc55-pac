#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SWTRIG {
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
#[doc = "Possible values of the field `SWT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT0R {
    #[doc = "No trigger 0 event generated."]
    SWT0_0,
    #[doc = "Trigger 0 event generated."]
    SWT0_1,
}
impl SWT0R {
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
            SWT0R::SWT0_0 => false,
            SWT0R::SWT0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWT0R {
        match value {
            false => SWT0R::SWT0_0,
            true => SWT0R::SWT0_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT0_0`"]
    #[inline]
    pub fn is_swt0_0(&self) -> bool {
        *self == SWT0R::SWT0_0
    }
    #[doc = "Checks if the value of the field is `SWT0_1`"]
    #[inline]
    pub fn is_swt0_1(&self) -> bool {
        *self == SWT0R::SWT0_1
    }
}
#[doc = "Possible values of the field `SWT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT1R {
    #[doc = "No trigger 1 event generated."]
    SWT1_0,
    #[doc = "Trigger 1 event generated."]
    SWT1_1,
}
impl SWT1R {
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
            SWT1R::SWT1_0 => false,
            SWT1R::SWT1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWT1R {
        match value {
            false => SWT1R::SWT1_0,
            true => SWT1R::SWT1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT1_0`"]
    #[inline]
    pub fn is_swt1_0(&self) -> bool {
        *self == SWT1R::SWT1_0
    }
    #[doc = "Checks if the value of the field is `SWT1_1`"]
    #[inline]
    pub fn is_swt1_1(&self) -> bool {
        *self == SWT1R::SWT1_1
    }
}
#[doc = "Possible values of the field `SWT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT2R {
    #[doc = "No trigger 2 event generated."]
    SWT2_0,
    #[doc = "Trigger 2 event generated."]
    SWT2_1,
}
impl SWT2R {
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
            SWT2R::SWT2_0 => false,
            SWT2R::SWT2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWT2R {
        match value {
            false => SWT2R::SWT2_0,
            true => SWT2R::SWT2_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT2_0`"]
    #[inline]
    pub fn is_swt2_0(&self) -> bool {
        *self == SWT2R::SWT2_0
    }
    #[doc = "Checks if the value of the field is `SWT2_1`"]
    #[inline]
    pub fn is_swt2_1(&self) -> bool {
        *self == SWT2R::SWT2_1
    }
}
#[doc = "Possible values of the field `SWT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT3R {
    #[doc = "No trigger 3 event generated."]
    SWT3_0,
    #[doc = "Trigger 3 event generated."]
    SWT3_1,
}
impl SWT3R {
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
            SWT3R::SWT3_0 => false,
            SWT3R::SWT3_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWT3R {
        match value {
            false => SWT3R::SWT3_0,
            true => SWT3R::SWT3_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT3_0`"]
    #[inline]
    pub fn is_swt3_0(&self) -> bool {
        *self == SWT3R::SWT3_0
    }
    #[doc = "Checks if the value of the field is `SWT3_1`"]
    #[inline]
    pub fn is_swt3_1(&self) -> bool {
        *self == SWT3R::SWT3_1
    }
}
#[doc = "Possible values of the field `SWT4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT4R {
    #[doc = "No trigger 4 event generated."]
    SWT4_0,
    #[doc = "Trigger 4 event generated."]
    SWT4_1,
}
impl SWT4R {
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
            SWT4R::SWT4_0 => false,
            SWT4R::SWT4_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWT4R {
        match value {
            false => SWT4R::SWT4_0,
            true => SWT4R::SWT4_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT4_0`"]
    #[inline]
    pub fn is_swt4_0(&self) -> bool {
        *self == SWT4R::SWT4_0
    }
    #[doc = "Checks if the value of the field is `SWT4_1`"]
    #[inline]
    pub fn is_swt4_1(&self) -> bool {
        *self == SWT4R::SWT4_1
    }
}
#[doc = "Possible values of the field `SWT5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT5R {
    #[doc = "No trigger 5 event generated."]
    SWT5_0,
    #[doc = "Trigger 5 event generated."]
    SWT5_1,
}
impl SWT5R {
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
            SWT5R::SWT5_0 => false,
            SWT5R::SWT5_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWT5R {
        match value {
            false => SWT5R::SWT5_0,
            true => SWT5R::SWT5_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT5_0`"]
    #[inline]
    pub fn is_swt5_0(&self) -> bool {
        *self == SWT5R::SWT5_0
    }
    #[doc = "Checks if the value of the field is `SWT5_1`"]
    #[inline]
    pub fn is_swt5_1(&self) -> bool {
        *self == SWT5R::SWT5_1
    }
}
#[doc = "Possible values of the field `SWT6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT6R {
    #[doc = "No trigger 6 event generated."]
    SWT6_0,
    #[doc = "Trigger 6 event generated."]
    SWT6_1,
}
impl SWT6R {
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
            SWT6R::SWT6_0 => false,
            SWT6R::SWT6_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWT6R {
        match value {
            false => SWT6R::SWT6_0,
            true => SWT6R::SWT6_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT6_0`"]
    #[inline]
    pub fn is_swt6_0(&self) -> bool {
        *self == SWT6R::SWT6_0
    }
    #[doc = "Checks if the value of the field is `SWT6_1`"]
    #[inline]
    pub fn is_swt6_1(&self) -> bool {
        *self == SWT6R::SWT6_1
    }
}
#[doc = "Possible values of the field `SWT7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT7R {
    #[doc = "No trigger 7 event generated."]
    SWT7_0,
    #[doc = "Trigger 7 event generated."]
    SWT7_1,
}
impl SWT7R {
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
            SWT7R::SWT7_0 => false,
            SWT7R::SWT7_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWT7R {
        match value {
            false => SWT7R::SWT7_0,
            true => SWT7R::SWT7_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT7_0`"]
    #[inline]
    pub fn is_swt7_0(&self) -> bool {
        *self == SWT7R::SWT7_0
    }
    #[doc = "Checks if the value of the field is `SWT7_1`"]
    #[inline]
    pub fn is_swt7_1(&self) -> bool {
        *self == SWT7R::SWT7_1
    }
}
#[doc = "Possible values of the field `SWT8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT8R {
    #[doc = "No trigger 8 event generated."]
    SWT8_0,
    #[doc = "Trigger 8 event generated."]
    SWT8_1,
}
impl SWT8R {
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
            SWT8R::SWT8_0 => false,
            SWT8R::SWT8_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWT8R {
        match value {
            false => SWT8R::SWT8_0,
            true => SWT8R::SWT8_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT8_0`"]
    #[inline]
    pub fn is_swt8_0(&self) -> bool {
        *self == SWT8R::SWT8_0
    }
    #[doc = "Checks if the value of the field is `SWT8_1`"]
    #[inline]
    pub fn is_swt8_1(&self) -> bool {
        *self == SWT8R::SWT8_1
    }
}
#[doc = "Possible values of the field `SWT9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT9R {
    #[doc = "No trigger 9 event generated."]
    SWT9_0,
    #[doc = "Trigger 9 event generated."]
    SWT9_1,
}
impl SWT9R {
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
            SWT9R::SWT9_0 => false,
            SWT9R::SWT9_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWT9R {
        match value {
            false => SWT9R::SWT9_0,
            true => SWT9R::SWT9_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT9_0`"]
    #[inline]
    pub fn is_swt9_0(&self) -> bool {
        *self == SWT9R::SWT9_0
    }
    #[doc = "Checks if the value of the field is `SWT9_1`"]
    #[inline]
    pub fn is_swt9_1(&self) -> bool {
        *self == SWT9R::SWT9_1
    }
}
#[doc = "Possible values of the field `SWT10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT10R {
    #[doc = "No trigger 10 event generated."]
    SWT10_0,
    #[doc = "Trigger 10 event generated."]
    SWT10_1,
}
impl SWT10R {
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
            SWT10R::SWT10_0 => false,
            SWT10R::SWT10_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWT10R {
        match value {
            false => SWT10R::SWT10_0,
            true => SWT10R::SWT10_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT10_0`"]
    #[inline]
    pub fn is_swt10_0(&self) -> bool {
        *self == SWT10R::SWT10_0
    }
    #[doc = "Checks if the value of the field is `SWT10_1`"]
    #[inline]
    pub fn is_swt10_1(&self) -> bool {
        *self == SWT10R::SWT10_1
    }
}
#[doc = "Possible values of the field `SWT11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT11R {
    #[doc = "No trigger 11 event generated."]
    SWT11_0,
    #[doc = "Trigger 11 event generated."]
    SWT11_1,
}
impl SWT11R {
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
            SWT11R::SWT11_0 => false,
            SWT11R::SWT11_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWT11R {
        match value {
            false => SWT11R::SWT11_0,
            true => SWT11R::SWT11_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT11_0`"]
    #[inline]
    pub fn is_swt11_0(&self) -> bool {
        *self == SWT11R::SWT11_0
    }
    #[doc = "Checks if the value of the field is `SWT11_1`"]
    #[inline]
    pub fn is_swt11_1(&self) -> bool {
        *self == SWT11R::SWT11_1
    }
}
#[doc = "Possible values of the field `SWT12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT12R {
    #[doc = "No trigger 12 event generated."]
    SWT12_0,
    #[doc = "Trigger 12 event generated."]
    SWT12_1,
}
impl SWT12R {
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
            SWT12R::SWT12_0 => false,
            SWT12R::SWT12_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWT12R {
        match value {
            false => SWT12R::SWT12_0,
            true => SWT12R::SWT12_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT12_0`"]
    #[inline]
    pub fn is_swt12_0(&self) -> bool {
        *self == SWT12R::SWT12_0
    }
    #[doc = "Checks if the value of the field is `SWT12_1`"]
    #[inline]
    pub fn is_swt12_1(&self) -> bool {
        *self == SWT12R::SWT12_1
    }
}
#[doc = "Possible values of the field `SWT13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT13R {
    #[doc = "No trigger 13 event generated."]
    SWT13_0,
    #[doc = "Trigger 13 event generated."]
    SWT13_1,
}
impl SWT13R {
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
            SWT13R::SWT13_0 => false,
            SWT13R::SWT13_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWT13R {
        match value {
            false => SWT13R::SWT13_0,
            true => SWT13R::SWT13_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT13_0`"]
    #[inline]
    pub fn is_swt13_0(&self) -> bool {
        *self == SWT13R::SWT13_0
    }
    #[doc = "Checks if the value of the field is `SWT13_1`"]
    #[inline]
    pub fn is_swt13_1(&self) -> bool {
        *self == SWT13R::SWT13_1
    }
}
#[doc = "Possible values of the field `SWT14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT14R {
    #[doc = "No trigger 14 event generated."]
    SWT14_0,
    #[doc = "Trigger 14 event generated."]
    SWT14_1,
}
impl SWT14R {
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
            SWT14R::SWT14_0 => false,
            SWT14R::SWT14_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWT14R {
        match value {
            false => SWT14R::SWT14_0,
            true => SWT14R::SWT14_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT14_0`"]
    #[inline]
    pub fn is_swt14_0(&self) -> bool {
        *self == SWT14R::SWT14_0
    }
    #[doc = "Checks if the value of the field is `SWT14_1`"]
    #[inline]
    pub fn is_swt14_1(&self) -> bool {
        *self == SWT14R::SWT14_1
    }
}
#[doc = "Possible values of the field `SWT15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT15R {
    #[doc = "No trigger 15 event generated."]
    SWT15_0,
    #[doc = "Trigger 15 event generated."]
    SWT15_1,
}
impl SWT15R {
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
            SWT15R::SWT15_0 => false,
            SWT15R::SWT15_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWT15R {
        match value {
            false => SWT15R::SWT15_0,
            true => SWT15R::SWT15_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT15_0`"]
    #[inline]
    pub fn is_swt15_0(&self) -> bool {
        *self == SWT15R::SWT15_0
    }
    #[doc = "Checks if the value of the field is `SWT15_1`"]
    #[inline]
    pub fn is_swt15_1(&self) -> bool {
        *self == SWT15R::SWT15_1
    }
}
#[doc = "Values that can be written to the field `SWT0`"]
pub enum SWT0W {
    #[doc = "No trigger 0 event generated."]
    SWT0_0,
    #[doc = "Trigger 0 event generated."]
    SWT0_1,
}
impl SWT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWT0W::SWT0_0 => false,
            SWT0W::SWT0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWT0W<'a> {
    w: &'a mut W,
}
impl<'a> _SWT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No trigger 0 event generated."]
    #[inline]
    pub fn swt0_0(self) -> &'a mut W {
        self.variant(SWT0W::SWT0_0)
    }
    #[doc = "Trigger 0 event generated."]
    #[inline]
    pub fn swt0_1(self) -> &'a mut W {
        self.variant(SWT0W::SWT0_1)
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
#[doc = "Values that can be written to the field `SWT1`"]
pub enum SWT1W {
    #[doc = "No trigger 1 event generated."]
    SWT1_0,
    #[doc = "Trigger 1 event generated."]
    SWT1_1,
}
impl SWT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWT1W::SWT1_0 => false,
            SWT1W::SWT1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWT1W<'a> {
    w: &'a mut W,
}
impl<'a> _SWT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No trigger 1 event generated."]
    #[inline]
    pub fn swt1_0(self) -> &'a mut W {
        self.variant(SWT1W::SWT1_0)
    }
    #[doc = "Trigger 1 event generated."]
    #[inline]
    pub fn swt1_1(self) -> &'a mut W {
        self.variant(SWT1W::SWT1_1)
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
#[doc = "Values that can be written to the field `SWT2`"]
pub enum SWT2W {
    #[doc = "No trigger 2 event generated."]
    SWT2_0,
    #[doc = "Trigger 2 event generated."]
    SWT2_1,
}
impl SWT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWT2W::SWT2_0 => false,
            SWT2W::SWT2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWT2W<'a> {
    w: &'a mut W,
}
impl<'a> _SWT2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWT2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No trigger 2 event generated."]
    #[inline]
    pub fn swt2_0(self) -> &'a mut W {
        self.variant(SWT2W::SWT2_0)
    }
    #[doc = "Trigger 2 event generated."]
    #[inline]
    pub fn swt2_1(self) -> &'a mut W {
        self.variant(SWT2W::SWT2_1)
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
#[doc = "Values that can be written to the field `SWT3`"]
pub enum SWT3W {
    #[doc = "No trigger 3 event generated."]
    SWT3_0,
    #[doc = "Trigger 3 event generated."]
    SWT3_1,
}
impl SWT3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWT3W::SWT3_0 => false,
            SWT3W::SWT3_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWT3W<'a> {
    w: &'a mut W,
}
impl<'a> _SWT3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWT3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No trigger 3 event generated."]
    #[inline]
    pub fn swt3_0(self) -> &'a mut W {
        self.variant(SWT3W::SWT3_0)
    }
    #[doc = "Trigger 3 event generated."]
    #[inline]
    pub fn swt3_1(self) -> &'a mut W {
        self.variant(SWT3W::SWT3_1)
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
#[doc = "Values that can be written to the field `SWT4`"]
pub enum SWT4W {
    #[doc = "No trigger 4 event generated."]
    SWT4_0,
    #[doc = "Trigger 4 event generated."]
    SWT4_1,
}
impl SWT4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWT4W::SWT4_0 => false,
            SWT4W::SWT4_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWT4W<'a> {
    w: &'a mut W,
}
impl<'a> _SWT4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWT4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No trigger 4 event generated."]
    #[inline]
    pub fn swt4_0(self) -> &'a mut W {
        self.variant(SWT4W::SWT4_0)
    }
    #[doc = "Trigger 4 event generated."]
    #[inline]
    pub fn swt4_1(self) -> &'a mut W {
        self.variant(SWT4W::SWT4_1)
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
#[doc = "Values that can be written to the field `SWT5`"]
pub enum SWT5W {
    #[doc = "No trigger 5 event generated."]
    SWT5_0,
    #[doc = "Trigger 5 event generated."]
    SWT5_1,
}
impl SWT5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWT5W::SWT5_0 => false,
            SWT5W::SWT5_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWT5W<'a> {
    w: &'a mut W,
}
impl<'a> _SWT5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWT5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No trigger 5 event generated."]
    #[inline]
    pub fn swt5_0(self) -> &'a mut W {
        self.variant(SWT5W::SWT5_0)
    }
    #[doc = "Trigger 5 event generated."]
    #[inline]
    pub fn swt5_1(self) -> &'a mut W {
        self.variant(SWT5W::SWT5_1)
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
#[doc = "Values that can be written to the field `SWT6`"]
pub enum SWT6W {
    #[doc = "No trigger 6 event generated."]
    SWT6_0,
    #[doc = "Trigger 6 event generated."]
    SWT6_1,
}
impl SWT6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWT6W::SWT6_0 => false,
            SWT6W::SWT6_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWT6W<'a> {
    w: &'a mut W,
}
impl<'a> _SWT6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWT6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No trigger 6 event generated."]
    #[inline]
    pub fn swt6_0(self) -> &'a mut W {
        self.variant(SWT6W::SWT6_0)
    }
    #[doc = "Trigger 6 event generated."]
    #[inline]
    pub fn swt6_1(self) -> &'a mut W {
        self.variant(SWT6W::SWT6_1)
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
#[doc = "Values that can be written to the field `SWT7`"]
pub enum SWT7W {
    #[doc = "No trigger 7 event generated."]
    SWT7_0,
    #[doc = "Trigger 7 event generated."]
    SWT7_1,
}
impl SWT7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWT7W::SWT7_0 => false,
            SWT7W::SWT7_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWT7W<'a> {
    w: &'a mut W,
}
impl<'a> _SWT7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWT7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No trigger 7 event generated."]
    #[inline]
    pub fn swt7_0(self) -> &'a mut W {
        self.variant(SWT7W::SWT7_0)
    }
    #[doc = "Trigger 7 event generated."]
    #[inline]
    pub fn swt7_1(self) -> &'a mut W {
        self.variant(SWT7W::SWT7_1)
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
#[doc = "Values that can be written to the field `SWT8`"]
pub enum SWT8W {
    #[doc = "No trigger 8 event generated."]
    SWT8_0,
    #[doc = "Trigger 8 event generated."]
    SWT8_1,
}
impl SWT8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWT8W::SWT8_0 => false,
            SWT8W::SWT8_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWT8W<'a> {
    w: &'a mut W,
}
impl<'a> _SWT8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWT8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No trigger 8 event generated."]
    #[inline]
    pub fn swt8_0(self) -> &'a mut W {
        self.variant(SWT8W::SWT8_0)
    }
    #[doc = "Trigger 8 event generated."]
    #[inline]
    pub fn swt8_1(self) -> &'a mut W {
        self.variant(SWT8W::SWT8_1)
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
#[doc = "Values that can be written to the field `SWT9`"]
pub enum SWT9W {
    #[doc = "No trigger 9 event generated."]
    SWT9_0,
    #[doc = "Trigger 9 event generated."]
    SWT9_1,
}
impl SWT9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWT9W::SWT9_0 => false,
            SWT9W::SWT9_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWT9W<'a> {
    w: &'a mut W,
}
impl<'a> _SWT9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWT9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No trigger 9 event generated."]
    #[inline]
    pub fn swt9_0(self) -> &'a mut W {
        self.variant(SWT9W::SWT9_0)
    }
    #[doc = "Trigger 9 event generated."]
    #[inline]
    pub fn swt9_1(self) -> &'a mut W {
        self.variant(SWT9W::SWT9_1)
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
#[doc = "Values that can be written to the field `SWT10`"]
pub enum SWT10W {
    #[doc = "No trigger 10 event generated."]
    SWT10_0,
    #[doc = "Trigger 10 event generated."]
    SWT10_1,
}
impl SWT10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWT10W::SWT10_0 => false,
            SWT10W::SWT10_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWT10W<'a> {
    w: &'a mut W,
}
impl<'a> _SWT10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWT10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No trigger 10 event generated."]
    #[inline]
    pub fn swt10_0(self) -> &'a mut W {
        self.variant(SWT10W::SWT10_0)
    }
    #[doc = "Trigger 10 event generated."]
    #[inline]
    pub fn swt10_1(self) -> &'a mut W {
        self.variant(SWT10W::SWT10_1)
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
#[doc = "Values that can be written to the field `SWT11`"]
pub enum SWT11W {
    #[doc = "No trigger 11 event generated."]
    SWT11_0,
    #[doc = "Trigger 11 event generated."]
    SWT11_1,
}
impl SWT11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWT11W::SWT11_0 => false,
            SWT11W::SWT11_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWT11W<'a> {
    w: &'a mut W,
}
impl<'a> _SWT11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWT11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No trigger 11 event generated."]
    #[inline]
    pub fn swt11_0(self) -> &'a mut W {
        self.variant(SWT11W::SWT11_0)
    }
    #[doc = "Trigger 11 event generated."]
    #[inline]
    pub fn swt11_1(self) -> &'a mut W {
        self.variant(SWT11W::SWT11_1)
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
#[doc = "Values that can be written to the field `SWT12`"]
pub enum SWT12W {
    #[doc = "No trigger 12 event generated."]
    SWT12_0,
    #[doc = "Trigger 12 event generated."]
    SWT12_1,
}
impl SWT12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWT12W::SWT12_0 => false,
            SWT12W::SWT12_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWT12W<'a> {
    w: &'a mut W,
}
impl<'a> _SWT12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWT12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No trigger 12 event generated."]
    #[inline]
    pub fn swt12_0(self) -> &'a mut W {
        self.variant(SWT12W::SWT12_0)
    }
    #[doc = "Trigger 12 event generated."]
    #[inline]
    pub fn swt12_1(self) -> &'a mut W {
        self.variant(SWT12W::SWT12_1)
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
#[doc = "Values that can be written to the field `SWT13`"]
pub enum SWT13W {
    #[doc = "No trigger 13 event generated."]
    SWT13_0,
    #[doc = "Trigger 13 event generated."]
    SWT13_1,
}
impl SWT13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWT13W::SWT13_0 => false,
            SWT13W::SWT13_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWT13W<'a> {
    w: &'a mut W,
}
impl<'a> _SWT13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWT13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No trigger 13 event generated."]
    #[inline]
    pub fn swt13_0(self) -> &'a mut W {
        self.variant(SWT13W::SWT13_0)
    }
    #[doc = "Trigger 13 event generated."]
    #[inline]
    pub fn swt13_1(self) -> &'a mut W {
        self.variant(SWT13W::SWT13_1)
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
#[doc = "Values that can be written to the field `SWT14`"]
pub enum SWT14W {
    #[doc = "No trigger 14 event generated."]
    SWT14_0,
    #[doc = "Trigger 14 event generated."]
    SWT14_1,
}
impl SWT14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWT14W::SWT14_0 => false,
            SWT14W::SWT14_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWT14W<'a> {
    w: &'a mut W,
}
impl<'a> _SWT14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWT14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No trigger 14 event generated."]
    #[inline]
    pub fn swt14_0(self) -> &'a mut W {
        self.variant(SWT14W::SWT14_0)
    }
    #[doc = "Trigger 14 event generated."]
    #[inline]
    pub fn swt14_1(self) -> &'a mut W {
        self.variant(SWT14W::SWT14_1)
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
#[doc = "Values that can be written to the field `SWT15`"]
pub enum SWT15W {
    #[doc = "No trigger 15 event generated."]
    SWT15_0,
    #[doc = "Trigger 15 event generated."]
    SWT15_1,
}
impl SWT15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWT15W::SWT15_0 => false,
            SWT15W::SWT15_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWT15W<'a> {
    w: &'a mut W,
}
impl<'a> _SWT15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWT15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No trigger 15 event generated."]
    #[inline]
    pub fn swt15_0(self) -> &'a mut W {
        self.variant(SWT15W::SWT15_0)
    }
    #[doc = "Trigger 15 event generated."]
    #[inline]
    pub fn swt15_1(self) -> &'a mut W {
        self.variant(SWT15W::SWT15_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Software trigger 0 event"]
    #[inline]
    pub fn swt0(&self) -> SWT0R {
        SWT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Software trigger 1 event"]
    #[inline]
    pub fn swt1(&self) -> SWT1R {
        SWT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Software trigger 2 event"]
    #[inline]
    pub fn swt2(&self) -> SWT2R {
        SWT2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Software trigger 3 event"]
    #[inline]
    pub fn swt3(&self) -> SWT3R {
        SWT3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Software trigger 4 event"]
    #[inline]
    pub fn swt4(&self) -> SWT4R {
        SWT4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Software trigger 5 event"]
    #[inline]
    pub fn swt5(&self) -> SWT5R {
        SWT5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Software trigger 6 event"]
    #[inline]
    pub fn swt6(&self) -> SWT6R {
        SWT6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Software trigger 7 event"]
    #[inline]
    pub fn swt7(&self) -> SWT7R {
        SWT7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Software trigger 8 event"]
    #[inline]
    pub fn swt8(&self) -> SWT8R {
        SWT8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Software trigger 9 event"]
    #[inline]
    pub fn swt9(&self) -> SWT9R {
        SWT9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Software trigger 10 event"]
    #[inline]
    pub fn swt10(&self) -> SWT10R {
        SWT10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Software trigger 11 event"]
    #[inline]
    pub fn swt11(&self) -> SWT11R {
        SWT11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Software trigger 12 event"]
    #[inline]
    pub fn swt12(&self) -> SWT12R {
        SWT12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Software trigger 13 event"]
    #[inline]
    pub fn swt13(&self) -> SWT13R {
        SWT13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Software trigger 14 event"]
    #[inline]
    pub fn swt14(&self) -> SWT14R {
        SWT14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Software trigger 15 event"]
    #[inline]
    pub fn swt15(&self) -> SWT15R {
        SWT15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - Software trigger 0 event"]
    #[inline]
    pub fn swt0(&mut self) -> _SWT0W {
        _SWT0W { w: self }
    }
    #[doc = "Bit 1 - Software trigger 1 event"]
    #[inline]
    pub fn swt1(&mut self) -> _SWT1W {
        _SWT1W { w: self }
    }
    #[doc = "Bit 2 - Software trigger 2 event"]
    #[inline]
    pub fn swt2(&mut self) -> _SWT2W {
        _SWT2W { w: self }
    }
    #[doc = "Bit 3 - Software trigger 3 event"]
    #[inline]
    pub fn swt3(&mut self) -> _SWT3W {
        _SWT3W { w: self }
    }
    #[doc = "Bit 4 - Software trigger 4 event"]
    #[inline]
    pub fn swt4(&mut self) -> _SWT4W {
        _SWT4W { w: self }
    }
    #[doc = "Bit 5 - Software trigger 5 event"]
    #[inline]
    pub fn swt5(&mut self) -> _SWT5W {
        _SWT5W { w: self }
    }
    #[doc = "Bit 6 - Software trigger 6 event"]
    #[inline]
    pub fn swt6(&mut self) -> _SWT6W {
        _SWT6W { w: self }
    }
    #[doc = "Bit 7 - Software trigger 7 event"]
    #[inline]
    pub fn swt7(&mut self) -> _SWT7W {
        _SWT7W { w: self }
    }
    #[doc = "Bit 8 - Software trigger 8 event"]
    #[inline]
    pub fn swt8(&mut self) -> _SWT8W {
        _SWT8W { w: self }
    }
    #[doc = "Bit 9 - Software trigger 9 event"]
    #[inline]
    pub fn swt9(&mut self) -> _SWT9W {
        _SWT9W { w: self }
    }
    #[doc = "Bit 10 - Software trigger 10 event"]
    #[inline]
    pub fn swt10(&mut self) -> _SWT10W {
        _SWT10W { w: self }
    }
    #[doc = "Bit 11 - Software trigger 11 event"]
    #[inline]
    pub fn swt11(&mut self) -> _SWT11W {
        _SWT11W { w: self }
    }
    #[doc = "Bit 12 - Software trigger 12 event"]
    #[inline]
    pub fn swt12(&mut self) -> _SWT12W {
        _SWT12W { w: self }
    }
    #[doc = "Bit 13 - Software trigger 13 event"]
    #[inline]
    pub fn swt13(&mut self) -> _SWT13W {
        _SWT13W { w: self }
    }
    #[doc = "Bit 14 - Software trigger 14 event"]
    #[inline]
    pub fn swt14(&mut self) -> _SWT14W {
        _SWT14W { w: self }
    }
    #[doc = "Bit 15 - Software trigger 15 event"]
    #[inline]
    pub fn swt15(&mut self) -> _SWT15W {
        _SWT15W { w: self }
    }
}
