#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SEC_GPIO_MASK0 {
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
#[doc = "Possible values of the field `PIO0_PIN0_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN0_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN0_SEC_MASKR {
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
            PIO0_PIN0_SEC_MASKR::BLOCKED => false,
            PIO0_PIN0_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN0_SEC_MASKR {
        match value {
            false => PIO0_PIN0_SEC_MASKR::BLOCKED,
            true => PIO0_PIN0_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN0_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN0_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN1_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN1_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN1_SEC_MASKR {
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
            PIO0_PIN1_SEC_MASKR::BLOCKED => false,
            PIO0_PIN1_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN1_SEC_MASKR {
        match value {
            false => PIO0_PIN1_SEC_MASKR::BLOCKED,
            true => PIO0_PIN1_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN1_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN1_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN2_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN2_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN2_SEC_MASKR {
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
            PIO0_PIN2_SEC_MASKR::BLOCKED => false,
            PIO0_PIN2_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN2_SEC_MASKR {
        match value {
            false => PIO0_PIN2_SEC_MASKR::BLOCKED,
            true => PIO0_PIN2_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN2_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN2_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN3_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN3_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN3_SEC_MASKR {
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
            PIO0_PIN3_SEC_MASKR::BLOCKED => false,
            PIO0_PIN3_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN3_SEC_MASKR {
        match value {
            false => PIO0_PIN3_SEC_MASKR::BLOCKED,
            true => PIO0_PIN3_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN3_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN3_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN4_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN4_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN4_SEC_MASKR {
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
            PIO0_PIN4_SEC_MASKR::BLOCKED => false,
            PIO0_PIN4_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN4_SEC_MASKR {
        match value {
            false => PIO0_PIN4_SEC_MASKR::BLOCKED,
            true => PIO0_PIN4_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN4_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN4_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN5_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN5_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN5_SEC_MASKR {
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
            PIO0_PIN5_SEC_MASKR::BLOCKED => false,
            PIO0_PIN5_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN5_SEC_MASKR {
        match value {
            false => PIO0_PIN5_SEC_MASKR::BLOCKED,
            true => PIO0_PIN5_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN5_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN5_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN6_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN6_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN6_SEC_MASKR {
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
            PIO0_PIN6_SEC_MASKR::BLOCKED => false,
            PIO0_PIN6_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN6_SEC_MASKR {
        match value {
            false => PIO0_PIN6_SEC_MASKR::BLOCKED,
            true => PIO0_PIN6_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN6_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN6_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN7_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN7_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN7_SEC_MASKR {
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
            PIO0_PIN7_SEC_MASKR::BLOCKED => false,
            PIO0_PIN7_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN7_SEC_MASKR {
        match value {
            false => PIO0_PIN7_SEC_MASKR::BLOCKED,
            true => PIO0_PIN7_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN7_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN7_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN8_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN8_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN8_SEC_MASKR {
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
            PIO0_PIN8_SEC_MASKR::BLOCKED => false,
            PIO0_PIN8_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN8_SEC_MASKR {
        match value {
            false => PIO0_PIN8_SEC_MASKR::BLOCKED,
            true => PIO0_PIN8_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN8_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN8_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN9_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN9_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN9_SEC_MASKR {
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
            PIO0_PIN9_SEC_MASKR::BLOCKED => false,
            PIO0_PIN9_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN9_SEC_MASKR {
        match value {
            false => PIO0_PIN9_SEC_MASKR::BLOCKED,
            true => PIO0_PIN9_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN9_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN9_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN10_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN10_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN10_SEC_MASKR {
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
            PIO0_PIN10_SEC_MASKR::BLOCKED => false,
            PIO0_PIN10_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN10_SEC_MASKR {
        match value {
            false => PIO0_PIN10_SEC_MASKR::BLOCKED,
            true => PIO0_PIN10_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN10_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN10_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN11_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN11_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN11_SEC_MASKR {
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
            PIO0_PIN11_SEC_MASKR::BLOCKED => false,
            PIO0_PIN11_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN11_SEC_MASKR {
        match value {
            false => PIO0_PIN11_SEC_MASKR::BLOCKED,
            true => PIO0_PIN11_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN11_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN11_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN12_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN12_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN12_SEC_MASKR {
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
            PIO0_PIN12_SEC_MASKR::BLOCKED => false,
            PIO0_PIN12_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN12_SEC_MASKR {
        match value {
            false => PIO0_PIN12_SEC_MASKR::BLOCKED,
            true => PIO0_PIN12_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN12_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN12_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN13_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN13_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN13_SEC_MASKR {
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
            PIO0_PIN13_SEC_MASKR::BLOCKED => false,
            PIO0_PIN13_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN13_SEC_MASKR {
        match value {
            false => PIO0_PIN13_SEC_MASKR::BLOCKED,
            true => PIO0_PIN13_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN13_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN13_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN14_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN14_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN14_SEC_MASKR {
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
            PIO0_PIN14_SEC_MASKR::BLOCKED => false,
            PIO0_PIN14_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN14_SEC_MASKR {
        match value {
            false => PIO0_PIN14_SEC_MASKR::BLOCKED,
            true => PIO0_PIN14_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN14_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN14_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN15_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN15_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN15_SEC_MASKR {
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
            PIO0_PIN15_SEC_MASKR::BLOCKED => false,
            PIO0_PIN15_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN15_SEC_MASKR {
        match value {
            false => PIO0_PIN15_SEC_MASKR::BLOCKED,
            true => PIO0_PIN15_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN15_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN15_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN16_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN16_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN16_SEC_MASKR {
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
            PIO0_PIN16_SEC_MASKR::BLOCKED => false,
            PIO0_PIN16_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN16_SEC_MASKR {
        match value {
            false => PIO0_PIN16_SEC_MASKR::BLOCKED,
            true => PIO0_PIN16_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN16_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN16_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN17_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN17_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN17_SEC_MASKR {
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
            PIO0_PIN17_SEC_MASKR::BLOCKED => false,
            PIO0_PIN17_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN17_SEC_MASKR {
        match value {
            false => PIO0_PIN17_SEC_MASKR::BLOCKED,
            true => PIO0_PIN17_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN17_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN17_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN18_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN18_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN18_SEC_MASKR {
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
            PIO0_PIN18_SEC_MASKR::BLOCKED => false,
            PIO0_PIN18_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN18_SEC_MASKR {
        match value {
            false => PIO0_PIN18_SEC_MASKR::BLOCKED,
            true => PIO0_PIN18_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN18_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN18_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN19_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN19_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN19_SEC_MASKR {
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
            PIO0_PIN19_SEC_MASKR::BLOCKED => false,
            PIO0_PIN19_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN19_SEC_MASKR {
        match value {
            false => PIO0_PIN19_SEC_MASKR::BLOCKED,
            true => PIO0_PIN19_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN19_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN19_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN20_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN20_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN20_SEC_MASKR {
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
            PIO0_PIN20_SEC_MASKR::BLOCKED => false,
            PIO0_PIN20_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN20_SEC_MASKR {
        match value {
            false => PIO0_PIN20_SEC_MASKR::BLOCKED,
            true => PIO0_PIN20_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN20_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN20_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN21_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN21_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN21_SEC_MASKR {
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
            PIO0_PIN21_SEC_MASKR::BLOCKED => false,
            PIO0_PIN21_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN21_SEC_MASKR {
        match value {
            false => PIO0_PIN21_SEC_MASKR::BLOCKED,
            true => PIO0_PIN21_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN21_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN21_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN22_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN22_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN22_SEC_MASKR {
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
            PIO0_PIN22_SEC_MASKR::BLOCKED => false,
            PIO0_PIN22_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN22_SEC_MASKR {
        match value {
            false => PIO0_PIN22_SEC_MASKR::BLOCKED,
            true => PIO0_PIN22_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN22_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN22_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN23_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN23_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN23_SEC_MASKR {
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
            PIO0_PIN23_SEC_MASKR::BLOCKED => false,
            PIO0_PIN23_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN23_SEC_MASKR {
        match value {
            false => PIO0_PIN23_SEC_MASKR::BLOCKED,
            true => PIO0_PIN23_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN23_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN23_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN24_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN24_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN24_SEC_MASKR {
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
            PIO0_PIN24_SEC_MASKR::BLOCKED => false,
            PIO0_PIN24_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN24_SEC_MASKR {
        match value {
            false => PIO0_PIN24_SEC_MASKR::BLOCKED,
            true => PIO0_PIN24_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN24_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN24_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN25_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN25_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN25_SEC_MASKR {
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
            PIO0_PIN25_SEC_MASKR::BLOCKED => false,
            PIO0_PIN25_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN25_SEC_MASKR {
        match value {
            false => PIO0_PIN25_SEC_MASKR::BLOCKED,
            true => PIO0_PIN25_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN25_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN25_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN26_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN26_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN26_SEC_MASKR {
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
            PIO0_PIN26_SEC_MASKR::BLOCKED => false,
            PIO0_PIN26_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN26_SEC_MASKR {
        match value {
            false => PIO0_PIN26_SEC_MASKR::BLOCKED,
            true => PIO0_PIN26_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN26_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN26_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN27_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN27_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN27_SEC_MASKR {
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
            PIO0_PIN27_SEC_MASKR::BLOCKED => false,
            PIO0_PIN27_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN27_SEC_MASKR {
        match value {
            false => PIO0_PIN27_SEC_MASKR::BLOCKED,
            true => PIO0_PIN27_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN27_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN27_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN28_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN28_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN28_SEC_MASKR {
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
            PIO0_PIN28_SEC_MASKR::BLOCKED => false,
            PIO0_PIN28_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN28_SEC_MASKR {
        match value {
            false => PIO0_PIN28_SEC_MASKR::BLOCKED,
            true => PIO0_PIN28_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN28_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN28_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN29_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN29_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN29_SEC_MASKR {
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
            PIO0_PIN29_SEC_MASKR::BLOCKED => false,
            PIO0_PIN29_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN29_SEC_MASKR {
        match value {
            false => PIO0_PIN29_SEC_MASKR::BLOCKED,
            true => PIO0_PIN29_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN29_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN29_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN30_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN30_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN30_SEC_MASKR {
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
            PIO0_PIN30_SEC_MASKR::BLOCKED => false,
            PIO0_PIN30_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN30_SEC_MASKR {
        match value {
            false => PIO0_PIN30_SEC_MASKR::BLOCKED,
            true => PIO0_PIN30_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN30_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN30_SEC_MASKR::READABLE
    }
}
#[doc = "Possible values of the field `PIO0_PIN31_SEC_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN31_SEC_MASKR {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN31_SEC_MASKR {
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
            PIO0_PIN31_SEC_MASKR::BLOCKED => false,
            PIO0_PIN31_SEC_MASKR::READABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO0_PIN31_SEC_MASKR {
        match value {
            false => PIO0_PIN31_SEC_MASKR::BLOCKED,
            true => PIO0_PIN31_SEC_MASKR::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == PIO0_PIN31_SEC_MASKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline]
    pub fn is_readable(&self) -> bool {
        *self == PIO0_PIN31_SEC_MASKR::READABLE
    }
}
#[doc = "Values that can be written to the field `PIO0_PIN0_SEC_MASK`"]
pub enum PIO0_PIN0_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN0_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN0_SEC_MASKW::BLOCKED => false,
            PIO0_PIN0_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN0_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN0_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN0_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN0_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN0_SEC_MASKW::READABLE)
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
#[doc = "Values that can be written to the field `PIO0_PIN1_SEC_MASK`"]
pub enum PIO0_PIN1_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN1_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN1_SEC_MASKW::BLOCKED => false,
            PIO0_PIN1_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN1_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN1_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN1_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN1_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN1_SEC_MASKW::READABLE)
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
#[doc = "Values that can be written to the field `PIO0_PIN2_SEC_MASK`"]
pub enum PIO0_PIN2_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN2_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN2_SEC_MASKW::BLOCKED => false,
            PIO0_PIN2_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN2_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN2_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN2_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN2_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN2_SEC_MASKW::READABLE)
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
#[doc = "Values that can be written to the field `PIO0_PIN3_SEC_MASK`"]
pub enum PIO0_PIN3_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN3_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN3_SEC_MASKW::BLOCKED => false,
            PIO0_PIN3_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN3_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN3_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN3_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN3_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN3_SEC_MASKW::READABLE)
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
#[doc = "Values that can be written to the field `PIO0_PIN4_SEC_MASK`"]
pub enum PIO0_PIN4_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN4_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN4_SEC_MASKW::BLOCKED => false,
            PIO0_PIN4_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN4_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN4_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN4_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN4_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN4_SEC_MASKW::READABLE)
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
#[doc = "Values that can be written to the field `PIO0_PIN5_SEC_MASK`"]
pub enum PIO0_PIN5_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN5_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN5_SEC_MASKW::BLOCKED => false,
            PIO0_PIN5_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN5_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN5_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN5_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN5_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN5_SEC_MASKW::READABLE)
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
#[doc = "Values that can be written to the field `PIO0_PIN6_SEC_MASK`"]
pub enum PIO0_PIN6_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN6_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN6_SEC_MASKW::BLOCKED => false,
            PIO0_PIN6_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN6_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN6_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN6_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN6_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN6_SEC_MASKW::READABLE)
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
#[doc = "Values that can be written to the field `PIO0_PIN7_SEC_MASK`"]
pub enum PIO0_PIN7_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN7_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN7_SEC_MASKW::BLOCKED => false,
            PIO0_PIN7_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN7_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN7_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN7_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN7_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN7_SEC_MASKW::READABLE)
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
#[doc = "Values that can be written to the field `PIO0_PIN8_SEC_MASK`"]
pub enum PIO0_PIN8_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN8_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN8_SEC_MASKW::BLOCKED => false,
            PIO0_PIN8_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN8_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN8_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN8_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN8_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN8_SEC_MASKW::READABLE)
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
#[doc = "Values that can be written to the field `PIO0_PIN9_SEC_MASK`"]
pub enum PIO0_PIN9_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN9_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN9_SEC_MASKW::BLOCKED => false,
            PIO0_PIN9_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN9_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN9_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN9_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN9_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN9_SEC_MASKW::READABLE)
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
#[doc = "Values that can be written to the field `PIO0_PIN10_SEC_MASK`"]
pub enum PIO0_PIN10_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN10_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN10_SEC_MASKW::BLOCKED => false,
            PIO0_PIN10_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN10_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN10_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN10_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN10_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN10_SEC_MASKW::READABLE)
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
#[doc = "Values that can be written to the field `PIO0_PIN11_SEC_MASK`"]
pub enum PIO0_PIN11_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN11_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN11_SEC_MASKW::BLOCKED => false,
            PIO0_PIN11_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN11_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN11_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN11_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN11_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN11_SEC_MASKW::READABLE)
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
#[doc = "Values that can be written to the field `PIO0_PIN12_SEC_MASK`"]
pub enum PIO0_PIN12_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN12_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN12_SEC_MASKW::BLOCKED => false,
            PIO0_PIN12_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN12_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN12_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN12_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN12_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN12_SEC_MASKW::READABLE)
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
#[doc = "Values that can be written to the field `PIO0_PIN13_SEC_MASK`"]
pub enum PIO0_PIN13_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN13_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN13_SEC_MASKW::BLOCKED => false,
            PIO0_PIN13_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN13_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN13_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN13_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN13_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN13_SEC_MASKW::READABLE)
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
#[doc = "Values that can be written to the field `PIO0_PIN14_SEC_MASK`"]
pub enum PIO0_PIN14_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN14_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN14_SEC_MASKW::BLOCKED => false,
            PIO0_PIN14_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN14_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN14_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN14_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN14_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN14_SEC_MASKW::READABLE)
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
#[doc = "Values that can be written to the field `PIO0_PIN15_SEC_MASK`"]
pub enum PIO0_PIN15_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN15_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN15_SEC_MASKW::BLOCKED => false,
            PIO0_PIN15_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN15_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN15_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN15_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN15_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN15_SEC_MASKW::READABLE)
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
#[doc = "Values that can be written to the field `PIO0_PIN16_SEC_MASK`"]
pub enum PIO0_PIN16_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN16_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN16_SEC_MASKW::BLOCKED => false,
            PIO0_PIN16_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN16_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN16_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN16_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN16_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN16_SEC_MASKW::READABLE)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIO0_PIN17_SEC_MASK`"]
pub enum PIO0_PIN17_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN17_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN17_SEC_MASKW::BLOCKED => false,
            PIO0_PIN17_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN17_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN17_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN17_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN17_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN17_SEC_MASKW::READABLE)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIO0_PIN18_SEC_MASK`"]
pub enum PIO0_PIN18_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN18_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN18_SEC_MASKW::BLOCKED => false,
            PIO0_PIN18_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN18_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN18_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN18_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN18_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN18_SEC_MASKW::READABLE)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIO0_PIN19_SEC_MASK`"]
pub enum PIO0_PIN19_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN19_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN19_SEC_MASKW::BLOCKED => false,
            PIO0_PIN19_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN19_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN19_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN19_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN19_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN19_SEC_MASKW::READABLE)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIO0_PIN20_SEC_MASK`"]
pub enum PIO0_PIN20_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN20_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN20_SEC_MASKW::BLOCKED => false,
            PIO0_PIN20_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN20_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN20_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN20_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN20_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN20_SEC_MASKW::READABLE)
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
#[doc = "Values that can be written to the field `PIO0_PIN21_SEC_MASK`"]
pub enum PIO0_PIN21_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN21_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN21_SEC_MASKW::BLOCKED => false,
            PIO0_PIN21_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN21_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN21_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN21_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN21_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN21_SEC_MASKW::READABLE)
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
#[doc = "Values that can be written to the field `PIO0_PIN22_SEC_MASK`"]
pub enum PIO0_PIN22_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN22_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN22_SEC_MASKW::BLOCKED => false,
            PIO0_PIN22_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN22_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN22_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN22_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN22_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN22_SEC_MASKW::READABLE)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIO0_PIN23_SEC_MASK`"]
pub enum PIO0_PIN23_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN23_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN23_SEC_MASKW::BLOCKED => false,
            PIO0_PIN23_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN23_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN23_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN23_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN23_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN23_SEC_MASKW::READABLE)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIO0_PIN24_SEC_MASK`"]
pub enum PIO0_PIN24_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN24_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN24_SEC_MASKW::BLOCKED => false,
            PIO0_PIN24_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN24_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN24_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN24_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN24_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN24_SEC_MASKW::READABLE)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIO0_PIN25_SEC_MASK`"]
pub enum PIO0_PIN25_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN25_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN25_SEC_MASKW::BLOCKED => false,
            PIO0_PIN25_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN25_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN25_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN25_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN25_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN25_SEC_MASKW::READABLE)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIO0_PIN26_SEC_MASK`"]
pub enum PIO0_PIN26_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN26_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN26_SEC_MASKW::BLOCKED => false,
            PIO0_PIN26_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN26_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN26_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN26_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN26_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN26_SEC_MASKW::READABLE)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIO0_PIN27_SEC_MASK`"]
pub enum PIO0_PIN27_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN27_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN27_SEC_MASKW::BLOCKED => false,
            PIO0_PIN27_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN27_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN27_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN27_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN27_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN27_SEC_MASKW::READABLE)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIO0_PIN28_SEC_MASK`"]
pub enum PIO0_PIN28_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN28_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN28_SEC_MASKW::BLOCKED => false,
            PIO0_PIN28_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN28_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN28_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN28_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN28_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN28_SEC_MASKW::READABLE)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIO0_PIN29_SEC_MASK`"]
pub enum PIO0_PIN29_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN29_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN29_SEC_MASKW::BLOCKED => false,
            PIO0_PIN29_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN29_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN29_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN29_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN29_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN29_SEC_MASKW::READABLE)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIO0_PIN30_SEC_MASK`"]
pub enum PIO0_PIN30_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN30_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN30_SEC_MASKW::BLOCKED => false,
            PIO0_PIN30_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN30_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN30_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN30_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN30_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN30_SEC_MASKW::READABLE)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIO0_PIN31_SEC_MASK`"]
pub enum PIO0_PIN31_SEC_MASKW {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE,
}
impl PIO0_PIN31_SEC_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO0_PIN31_SEC_MASKW::BLOCKED => false,
            PIO0_PIN31_SEC_MASKW::READABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO0_PIN31_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN31_SEC_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO0_PIN31_SEC_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN31_SEC_MASKW::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN31_SEC_MASKW::READABLE)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Secure mask for pin P0_0"]
    #[inline]
    pub fn pio0_pin0_sec_mask(&self) -> PIO0_PIN0_SEC_MASKR {
        PIO0_PIN0_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Secure mask for pin P0_1"]
    #[inline]
    pub fn pio0_pin1_sec_mask(&self) -> PIO0_PIN1_SEC_MASKR {
        PIO0_PIN1_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Secure mask for pin P0_2"]
    #[inline]
    pub fn pio0_pin2_sec_mask(&self) -> PIO0_PIN2_SEC_MASKR {
        PIO0_PIN2_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Secure mask for pin P0_3"]
    #[inline]
    pub fn pio0_pin3_sec_mask(&self) -> PIO0_PIN3_SEC_MASKR {
        PIO0_PIN3_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Secure mask for pin P0_4"]
    #[inline]
    pub fn pio0_pin4_sec_mask(&self) -> PIO0_PIN4_SEC_MASKR {
        PIO0_PIN4_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Secure mask for pin P0_5"]
    #[inline]
    pub fn pio0_pin5_sec_mask(&self) -> PIO0_PIN5_SEC_MASKR {
        PIO0_PIN5_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Secure mask for pin P0_6"]
    #[inline]
    pub fn pio0_pin6_sec_mask(&self) -> PIO0_PIN6_SEC_MASKR {
        PIO0_PIN6_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Secure mask for pin P0_7"]
    #[inline]
    pub fn pio0_pin7_sec_mask(&self) -> PIO0_PIN7_SEC_MASKR {
        PIO0_PIN7_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Secure mask for pin P0_8"]
    #[inline]
    pub fn pio0_pin8_sec_mask(&self) -> PIO0_PIN8_SEC_MASKR {
        PIO0_PIN8_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Secure mask for pin P0_9"]
    #[inline]
    pub fn pio0_pin9_sec_mask(&self) -> PIO0_PIN9_SEC_MASKR {
        PIO0_PIN9_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Secure mask for pin P0_10"]
    #[inline]
    pub fn pio0_pin10_sec_mask(&self) -> PIO0_PIN10_SEC_MASKR {
        PIO0_PIN10_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Secure mask for pin P0_11"]
    #[inline]
    pub fn pio0_pin11_sec_mask(&self) -> PIO0_PIN11_SEC_MASKR {
        PIO0_PIN11_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Secure mask for pin P0_12"]
    #[inline]
    pub fn pio0_pin12_sec_mask(&self) -> PIO0_PIN12_SEC_MASKR {
        PIO0_PIN12_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Secure mask for pin P0_13"]
    #[inline]
    pub fn pio0_pin13_sec_mask(&self) -> PIO0_PIN13_SEC_MASKR {
        PIO0_PIN13_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Secure mask for pin P0_14"]
    #[inline]
    pub fn pio0_pin14_sec_mask(&self) -> PIO0_PIN14_SEC_MASKR {
        PIO0_PIN14_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Secure mask for pin P0_15"]
    #[inline]
    pub fn pio0_pin15_sec_mask(&self) -> PIO0_PIN15_SEC_MASKR {
        PIO0_PIN15_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Secure mask for pin P0_16"]
    #[inline]
    pub fn pio0_pin16_sec_mask(&self) -> PIO0_PIN16_SEC_MASKR {
        PIO0_PIN16_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Secure mask for pin P0_17"]
    #[inline]
    pub fn pio0_pin17_sec_mask(&self) -> PIO0_PIN17_SEC_MASKR {
        PIO0_PIN17_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Secure mask for pin P0_18"]
    #[inline]
    pub fn pio0_pin18_sec_mask(&self) -> PIO0_PIN18_SEC_MASKR {
        PIO0_PIN18_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Secure mask for pin P0_19"]
    #[inline]
    pub fn pio0_pin19_sec_mask(&self) -> PIO0_PIN19_SEC_MASKR {
        PIO0_PIN19_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Secure mask for pin P0_20"]
    #[inline]
    pub fn pio0_pin20_sec_mask(&self) -> PIO0_PIN20_SEC_MASKR {
        PIO0_PIN20_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Secure mask for pin P0_21"]
    #[inline]
    pub fn pio0_pin21_sec_mask(&self) -> PIO0_PIN21_SEC_MASKR {
        PIO0_PIN21_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Secure mask for pin P0_22"]
    #[inline]
    pub fn pio0_pin22_sec_mask(&self) -> PIO0_PIN22_SEC_MASKR {
        PIO0_PIN22_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Secure mask for pin P0_23"]
    #[inline]
    pub fn pio0_pin23_sec_mask(&self) -> PIO0_PIN23_SEC_MASKR {
        PIO0_PIN23_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Secure mask for pin P0_24"]
    #[inline]
    pub fn pio0_pin24_sec_mask(&self) -> PIO0_PIN24_SEC_MASKR {
        PIO0_PIN24_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Secure mask for pin P0_25"]
    #[inline]
    pub fn pio0_pin25_sec_mask(&self) -> PIO0_PIN25_SEC_MASKR {
        PIO0_PIN25_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Secure mask for pin P0_26"]
    #[inline]
    pub fn pio0_pin26_sec_mask(&self) -> PIO0_PIN26_SEC_MASKR {
        PIO0_PIN26_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Secure mask for pin P0_27"]
    #[inline]
    pub fn pio0_pin27_sec_mask(&self) -> PIO0_PIN27_SEC_MASKR {
        PIO0_PIN27_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Secure mask for pin P0_28"]
    #[inline]
    pub fn pio0_pin28_sec_mask(&self) -> PIO0_PIN28_SEC_MASKR {
        PIO0_PIN28_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Secure mask for pin P0_29"]
    #[inline]
    pub fn pio0_pin29_sec_mask(&self) -> PIO0_PIN29_SEC_MASKR {
        PIO0_PIN29_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Secure mask for pin P0_30"]
    #[inline]
    pub fn pio0_pin30_sec_mask(&self) -> PIO0_PIN30_SEC_MASKR {
        PIO0_PIN30_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Secure mask for pin P0_31"]
    #[inline]
    pub fn pio0_pin31_sec_mask(&self) -> PIO0_PIN31_SEC_MASKR {
        PIO0_PIN31_SEC_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Secure mask for pin P0_0"]
    #[inline]
    pub fn pio0_pin0_sec_mask(&mut self) -> _PIO0_PIN0_SEC_MASKW {
        _PIO0_PIN0_SEC_MASKW { w: self }
    }
    #[doc = "Bit 1 - Secure mask for pin P0_1"]
    #[inline]
    pub fn pio0_pin1_sec_mask(&mut self) -> _PIO0_PIN1_SEC_MASKW {
        _PIO0_PIN1_SEC_MASKW { w: self }
    }
    #[doc = "Bit 2 - Secure mask for pin P0_2"]
    #[inline]
    pub fn pio0_pin2_sec_mask(&mut self) -> _PIO0_PIN2_SEC_MASKW {
        _PIO0_PIN2_SEC_MASKW { w: self }
    }
    #[doc = "Bit 3 - Secure mask for pin P0_3"]
    #[inline]
    pub fn pio0_pin3_sec_mask(&mut self) -> _PIO0_PIN3_SEC_MASKW {
        _PIO0_PIN3_SEC_MASKW { w: self }
    }
    #[doc = "Bit 4 - Secure mask for pin P0_4"]
    #[inline]
    pub fn pio0_pin4_sec_mask(&mut self) -> _PIO0_PIN4_SEC_MASKW {
        _PIO0_PIN4_SEC_MASKW { w: self }
    }
    #[doc = "Bit 5 - Secure mask for pin P0_5"]
    #[inline]
    pub fn pio0_pin5_sec_mask(&mut self) -> _PIO0_PIN5_SEC_MASKW {
        _PIO0_PIN5_SEC_MASKW { w: self }
    }
    #[doc = "Bit 6 - Secure mask for pin P0_6"]
    #[inline]
    pub fn pio0_pin6_sec_mask(&mut self) -> _PIO0_PIN6_SEC_MASKW {
        _PIO0_PIN6_SEC_MASKW { w: self }
    }
    #[doc = "Bit 7 - Secure mask for pin P0_7"]
    #[inline]
    pub fn pio0_pin7_sec_mask(&mut self) -> _PIO0_PIN7_SEC_MASKW {
        _PIO0_PIN7_SEC_MASKW { w: self }
    }
    #[doc = "Bit 8 - Secure mask for pin P0_8"]
    #[inline]
    pub fn pio0_pin8_sec_mask(&mut self) -> _PIO0_PIN8_SEC_MASKW {
        _PIO0_PIN8_SEC_MASKW { w: self }
    }
    #[doc = "Bit 9 - Secure mask for pin P0_9"]
    #[inline]
    pub fn pio0_pin9_sec_mask(&mut self) -> _PIO0_PIN9_SEC_MASKW {
        _PIO0_PIN9_SEC_MASKW { w: self }
    }
    #[doc = "Bit 10 - Secure mask for pin P0_10"]
    #[inline]
    pub fn pio0_pin10_sec_mask(&mut self) -> _PIO0_PIN10_SEC_MASKW {
        _PIO0_PIN10_SEC_MASKW { w: self }
    }
    #[doc = "Bit 11 - Secure mask for pin P0_11"]
    #[inline]
    pub fn pio0_pin11_sec_mask(&mut self) -> _PIO0_PIN11_SEC_MASKW {
        _PIO0_PIN11_SEC_MASKW { w: self }
    }
    #[doc = "Bit 12 - Secure mask for pin P0_12"]
    #[inline]
    pub fn pio0_pin12_sec_mask(&mut self) -> _PIO0_PIN12_SEC_MASKW {
        _PIO0_PIN12_SEC_MASKW { w: self }
    }
    #[doc = "Bit 13 - Secure mask for pin P0_13"]
    #[inline]
    pub fn pio0_pin13_sec_mask(&mut self) -> _PIO0_PIN13_SEC_MASKW {
        _PIO0_PIN13_SEC_MASKW { w: self }
    }
    #[doc = "Bit 14 - Secure mask for pin P0_14"]
    #[inline]
    pub fn pio0_pin14_sec_mask(&mut self) -> _PIO0_PIN14_SEC_MASKW {
        _PIO0_PIN14_SEC_MASKW { w: self }
    }
    #[doc = "Bit 15 - Secure mask for pin P0_15"]
    #[inline]
    pub fn pio0_pin15_sec_mask(&mut self) -> _PIO0_PIN15_SEC_MASKW {
        _PIO0_PIN15_SEC_MASKW { w: self }
    }
    #[doc = "Bit 16 - Secure mask for pin P0_16"]
    #[inline]
    pub fn pio0_pin16_sec_mask(&mut self) -> _PIO0_PIN16_SEC_MASKW {
        _PIO0_PIN16_SEC_MASKW { w: self }
    }
    #[doc = "Bit 17 - Secure mask for pin P0_17"]
    #[inline]
    pub fn pio0_pin17_sec_mask(&mut self) -> _PIO0_PIN17_SEC_MASKW {
        _PIO0_PIN17_SEC_MASKW { w: self }
    }
    #[doc = "Bit 18 - Secure mask for pin P0_18"]
    #[inline]
    pub fn pio0_pin18_sec_mask(&mut self) -> _PIO0_PIN18_SEC_MASKW {
        _PIO0_PIN18_SEC_MASKW { w: self }
    }
    #[doc = "Bit 19 - Secure mask for pin P0_19"]
    #[inline]
    pub fn pio0_pin19_sec_mask(&mut self) -> _PIO0_PIN19_SEC_MASKW {
        _PIO0_PIN19_SEC_MASKW { w: self }
    }
    #[doc = "Bit 20 - Secure mask for pin P0_20"]
    #[inline]
    pub fn pio0_pin20_sec_mask(&mut self) -> _PIO0_PIN20_SEC_MASKW {
        _PIO0_PIN20_SEC_MASKW { w: self }
    }
    #[doc = "Bit 21 - Secure mask for pin P0_21"]
    #[inline]
    pub fn pio0_pin21_sec_mask(&mut self) -> _PIO0_PIN21_SEC_MASKW {
        _PIO0_PIN21_SEC_MASKW { w: self }
    }
    #[doc = "Bit 22 - Secure mask for pin P0_22"]
    #[inline]
    pub fn pio0_pin22_sec_mask(&mut self) -> _PIO0_PIN22_SEC_MASKW {
        _PIO0_PIN22_SEC_MASKW { w: self }
    }
    #[doc = "Bit 23 - Secure mask for pin P0_23"]
    #[inline]
    pub fn pio0_pin23_sec_mask(&mut self) -> _PIO0_PIN23_SEC_MASKW {
        _PIO0_PIN23_SEC_MASKW { w: self }
    }
    #[doc = "Bit 24 - Secure mask for pin P0_24"]
    #[inline]
    pub fn pio0_pin24_sec_mask(&mut self) -> _PIO0_PIN24_SEC_MASKW {
        _PIO0_PIN24_SEC_MASKW { w: self }
    }
    #[doc = "Bit 25 - Secure mask for pin P0_25"]
    #[inline]
    pub fn pio0_pin25_sec_mask(&mut self) -> _PIO0_PIN25_SEC_MASKW {
        _PIO0_PIN25_SEC_MASKW { w: self }
    }
    #[doc = "Bit 26 - Secure mask for pin P0_26"]
    #[inline]
    pub fn pio0_pin26_sec_mask(&mut self) -> _PIO0_PIN26_SEC_MASKW {
        _PIO0_PIN26_SEC_MASKW { w: self }
    }
    #[doc = "Bit 27 - Secure mask for pin P0_27"]
    #[inline]
    pub fn pio0_pin27_sec_mask(&mut self) -> _PIO0_PIN27_SEC_MASKW {
        _PIO0_PIN27_SEC_MASKW { w: self }
    }
    #[doc = "Bit 28 - Secure mask for pin P0_28"]
    #[inline]
    pub fn pio0_pin28_sec_mask(&mut self) -> _PIO0_PIN28_SEC_MASKW {
        _PIO0_PIN28_SEC_MASKW { w: self }
    }
    #[doc = "Bit 29 - Secure mask for pin P0_29"]
    #[inline]
    pub fn pio0_pin29_sec_mask(&mut self) -> _PIO0_PIN29_SEC_MASKW {
        _PIO0_PIN29_SEC_MASKW { w: self }
    }
    #[doc = "Bit 30 - Secure mask for pin P0_30"]
    #[inline]
    pub fn pio0_pin30_sec_mask(&mut self) -> _PIO0_PIN30_SEC_MASKW {
        _PIO0_PIN30_SEC_MASKW { w: self }
    }
    #[doc = "Bit 31 - Secure mask for pin P0_31"]
    #[inline]
    pub fn pio0_pin31_sec_mask(&mut self) -> _PIO0_PIN31_SEC_MASKW {
        _PIO0_PIN31_SEC_MASKW { w: self }
    }
}
