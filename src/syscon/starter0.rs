#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STARTER0 {
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
#[doc = "Possible values of the field `SYS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSR {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl SYSR {
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
            SYSR::DISABLE => false,
            SYSR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYSR {
        match value {
            false => SYSR::DISABLE,
            true => SYSR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SYSR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SYSR::ENABLE
    }
}
#[doc = "Possible values of the field `SDMA0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMA0R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl SDMA0R {
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
            SDMA0R::DISABLE => false,
            SDMA0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDMA0R {
        match value {
            false => SDMA0R::DISABLE,
            true => SDMA0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SDMA0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SDMA0R::ENABLE
    }
}
#[doc = "Possible values of the field `GINT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GINT0R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl GINT0R {
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
            GINT0R::DISABLE => false,
            GINT0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GINT0R {
        match value {
            false => GINT0R::DISABLE,
            true => GINT0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == GINT0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == GINT0R::ENABLE
    }
}
#[doc = "Possible values of the field `GINT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GINT1R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl GINT1R {
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
            GINT1R::DISABLE => false,
            GINT1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GINT1R {
        match value {
            false => GINT1R::DISABLE,
            true => GINT1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == GINT1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == GINT1R::ENABLE
    }
}
#[doc = "Possible values of the field `PIO_INT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO_INT0R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl PIO_INT0R {
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
            PIO_INT0R::DISABLE => false,
            PIO_INT0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO_INT0R {
        match value {
            false => PIO_INT0R::DISABLE,
            true => PIO_INT0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PIO_INT0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PIO_INT0R::ENABLE
    }
}
#[doc = "Possible values of the field `PIO_INT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO_INT1R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl PIO_INT1R {
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
            PIO_INT1R::DISABLE => false,
            PIO_INT1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO_INT1R {
        match value {
            false => PIO_INT1R::DISABLE,
            true => PIO_INT1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PIO_INT1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PIO_INT1R::ENABLE
    }
}
#[doc = "Possible values of the field `PIO_INT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO_INT2R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl PIO_INT2R {
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
            PIO_INT2R::DISABLE => false,
            PIO_INT2R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO_INT2R {
        match value {
            false => PIO_INT2R::DISABLE,
            true => PIO_INT2R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PIO_INT2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PIO_INT2R::ENABLE
    }
}
#[doc = "Possible values of the field `PIO_INT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO_INT3R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl PIO_INT3R {
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
            PIO_INT3R::DISABLE => false,
            PIO_INT3R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIO_INT3R {
        match value {
            false => PIO_INT3R::DISABLE,
            true => PIO_INT3R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PIO_INT3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PIO_INT3R::ENABLE
    }
}
#[doc = "Possible values of the field `UTICK0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UTICK0R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl UTICK0R {
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
            UTICK0R::DISABLE => false,
            UTICK0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UTICK0R {
        match value {
            false => UTICK0R::DISABLE,
            true => UTICK0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == UTICK0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == UTICK0R::ENABLE
    }
}
#[doc = "Possible values of the field `MRT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRT0R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl MRT0R {
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
            MRT0R::DISABLE => false,
            MRT0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MRT0R {
        match value {
            false => MRT0R::DISABLE,
            true => MRT0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == MRT0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == MRT0R::ENABLE
    }
}
#[doc = "Possible values of the field `CTIMER0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER0R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl CTIMER0R {
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
            CTIMER0R::DISABLE => false,
            CTIMER0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTIMER0R {
        match value {
            false => CTIMER0R::DISABLE,
            true => CTIMER0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CTIMER0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CTIMER0R::ENABLE
    }
}
#[doc = "Possible values of the field `CTIMER1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER1R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl CTIMER1R {
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
            CTIMER1R::DISABLE => false,
            CTIMER1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTIMER1R {
        match value {
            false => CTIMER1R::DISABLE,
            true => CTIMER1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CTIMER1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CTIMER1R::ENABLE
    }
}
#[doc = "Possible values of the field `SCT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT0R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl SCT0R {
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
            SCT0R::DISABLE => false,
            SCT0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCT0R {
        match value {
            false => SCT0R::DISABLE,
            true => SCT0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SCT0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SCT0R::ENABLE
    }
}
#[doc = "Possible values of the field `CTIMER3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER3R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl CTIMER3R {
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
            CTIMER3R::DISABLE => false,
            CTIMER3R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTIMER3R {
        match value {
            false => CTIMER3R::DISABLE,
            true => CTIMER3R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CTIMER3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CTIMER3R::ENABLE
    }
}
#[doc = "Possible values of the field `FLEXINT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXINT0R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl FLEXINT0R {
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
            FLEXINT0R::DISABLE => false,
            FLEXINT0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXINT0R {
        match value {
            false => FLEXINT0R::DISABLE,
            true => FLEXINT0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FLEXINT0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FLEXINT0R::ENABLE
    }
}
#[doc = "Possible values of the field `FLEXINT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXINT1R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl FLEXINT1R {
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
            FLEXINT1R::DISABLE => false,
            FLEXINT1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXINT1R {
        match value {
            false => FLEXINT1R::DISABLE,
            true => FLEXINT1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FLEXINT1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FLEXINT1R::ENABLE
    }
}
#[doc = "Possible values of the field `FLEXINT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXINT2R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl FLEXINT2R {
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
            FLEXINT2R::DISABLE => false,
            FLEXINT2R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXINT2R {
        match value {
            false => FLEXINT2R::DISABLE,
            true => FLEXINT2R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FLEXINT2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FLEXINT2R::ENABLE
    }
}
#[doc = "Possible values of the field `FLEXINT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXINT3R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl FLEXINT3R {
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
            FLEXINT3R::DISABLE => false,
            FLEXINT3R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXINT3R {
        match value {
            false => FLEXINT3R::DISABLE,
            true => FLEXINT3R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FLEXINT3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FLEXINT3R::ENABLE
    }
}
#[doc = "Possible values of the field `FLEXINT4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXINT4R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl FLEXINT4R {
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
            FLEXINT4R::DISABLE => false,
            FLEXINT4R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXINT4R {
        match value {
            false => FLEXINT4R::DISABLE,
            true => FLEXINT4R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FLEXINT4R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FLEXINT4R::ENABLE
    }
}
#[doc = "Possible values of the field `FLEXINT5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXINT5R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl FLEXINT5R {
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
            FLEXINT5R::DISABLE => false,
            FLEXINT5R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXINT5R {
        match value {
            false => FLEXINT5R::DISABLE,
            true => FLEXINT5R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FLEXINT5R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FLEXINT5R::ENABLE
    }
}
#[doc = "Possible values of the field `FLEXINT6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXINT6R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl FLEXINT6R {
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
            FLEXINT6R::DISABLE => false,
            FLEXINT6R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXINT6R {
        match value {
            false => FLEXINT6R::DISABLE,
            true => FLEXINT6R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FLEXINT6R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FLEXINT6R::ENABLE
    }
}
#[doc = "Possible values of the field `FLEXINT7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXINT7R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl FLEXINT7R {
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
            FLEXINT7R::DISABLE => false,
            FLEXINT7R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXINT7R {
        match value {
            false => FLEXINT7R::DISABLE,
            true => FLEXINT7R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FLEXINT7R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FLEXINT7R::ENABLE
    }
}
#[doc = "Possible values of the field `ADC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl ADC0R {
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
            ADC0R::DISABLE => false,
            ADC0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC0R {
        match value {
            false => ADC0R::DISABLE,
            true => ADC0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ADC0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ADC0R::ENABLE
    }
}
#[doc = "Possible values of the field `ADC0_THCMP_OVR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0_THCMP_OVRR {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl ADC0_THCMP_OVRR {
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
            ADC0_THCMP_OVRR::DISABLE => false,
            ADC0_THCMP_OVRR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC0_THCMP_OVRR {
        match value {
            false => ADC0_THCMP_OVRR::DISABLE,
            true => ADC0_THCMP_OVRR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ADC0_THCMP_OVRR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ADC0_THCMP_OVRR::ENABLE
    }
}
#[doc = "Possible values of the field `USB0_NEEDCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_NEEDCLKR {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl USB0_NEEDCLKR {
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
            USB0_NEEDCLKR::DISABLE => false,
            USB0_NEEDCLKR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB0_NEEDCLKR {
        match value {
            false => USB0_NEEDCLKR::DISABLE,
            true => USB0_NEEDCLKR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == USB0_NEEDCLKR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == USB0_NEEDCLKR::ENABLE
    }
}
#[doc = "Possible values of the field `USB0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl USB0R {
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
            USB0R::DISABLE => false,
            USB0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB0R {
        match value {
            false => USB0R::DISABLE,
            true => USB0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == USB0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == USB0R::ENABLE
    }
}
#[doc = "Possible values of the field `RTC_LITE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_LITE0R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl RTC_LITE0R {
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
            RTC_LITE0R::DISABLE => false,
            RTC_LITE0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_LITE0R {
        match value {
            false => RTC_LITE0R::DISABLE,
            true => RTC_LITE0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RTC_LITE0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RTC_LITE0R::ENABLE
    }
}
#[doc = "Possible values of the field `EZH_ARCH_B0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EZH_ARCH_B0R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl EZH_ARCH_B0R {
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
            EZH_ARCH_B0R::DISABLE => false,
            EZH_ARCH_B0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EZH_ARCH_B0R {
        match value {
            false => EZH_ARCH_B0R::DISABLE,
            true => EZH_ARCH_B0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == EZH_ARCH_B0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == EZH_ARCH_B0R::ENABLE
    }
}
#[doc = "Possible values of the field `WAKEUP_MAILBOX0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_MAILBOX0R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl WAKEUP_MAILBOX0R {
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
            WAKEUP_MAILBOX0R::DISABLE => false,
            WAKEUP_MAILBOX0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUP_MAILBOX0R {
        match value {
            false => WAKEUP_MAILBOX0R::DISABLE,
            true => WAKEUP_MAILBOX0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WAKEUP_MAILBOX0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WAKEUP_MAILBOX0R::ENABLE
    }
}
#[doc = "Values that can be written to the field `SYS`"]
pub enum SYSW {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl SYSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSW::DISABLE => false,
            SYSW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYSW::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYSW::ENABLE)
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
#[doc = "Values that can be written to the field `SDMA0`"]
pub enum SDMA0W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl SDMA0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDMA0W::DISABLE => false,
            SDMA0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDMA0W<'a> {
    w: &'a mut W,
}
impl<'a> _SDMA0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDMA0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDMA0W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDMA0W::ENABLE)
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
#[doc = "Values that can be written to the field `GINT0`"]
pub enum GINT0W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl GINT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GINT0W::DISABLE => false,
            GINT0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GINT0W<'a> {
    w: &'a mut W,
}
impl<'a> _GINT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GINT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(GINT0W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(GINT0W::ENABLE)
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
#[doc = "Values that can be written to the field `GINT1`"]
pub enum GINT1W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl GINT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GINT1W::DISABLE => false,
            GINT1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GINT1W<'a> {
    w: &'a mut W,
}
impl<'a> _GINT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GINT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(GINT1W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(GINT1W::ENABLE)
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
#[doc = "Values that can be written to the field `PIO_INT0`"]
pub enum PIO_INT0W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl PIO_INT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO_INT0W::DISABLE => false,
            PIO_INT0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO_INT0W<'a> {
    w: &'a mut W,
}
impl<'a> _PIO_INT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO_INT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PIO_INT0W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PIO_INT0W::ENABLE)
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
#[doc = "Values that can be written to the field `PIO_INT1`"]
pub enum PIO_INT1W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl PIO_INT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO_INT1W::DISABLE => false,
            PIO_INT1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO_INT1W<'a> {
    w: &'a mut W,
}
impl<'a> _PIO_INT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO_INT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PIO_INT1W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PIO_INT1W::ENABLE)
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
#[doc = "Values that can be written to the field `PIO_INT2`"]
pub enum PIO_INT2W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl PIO_INT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO_INT2W::DISABLE => false,
            PIO_INT2W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO_INT2W<'a> {
    w: &'a mut W,
}
impl<'a> _PIO_INT2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO_INT2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PIO_INT2W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PIO_INT2W::ENABLE)
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
#[doc = "Values that can be written to the field `PIO_INT3`"]
pub enum PIO_INT3W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl PIO_INT3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIO_INT3W::DISABLE => false,
            PIO_INT3W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIO_INT3W<'a> {
    w: &'a mut W,
}
impl<'a> _PIO_INT3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIO_INT3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PIO_INT3W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PIO_INT3W::ENABLE)
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
#[doc = "Values that can be written to the field `UTICK0`"]
pub enum UTICK0W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl UTICK0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UTICK0W::DISABLE => false,
            UTICK0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UTICK0W<'a> {
    w: &'a mut W,
}
impl<'a> _UTICK0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UTICK0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(UTICK0W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(UTICK0W::ENABLE)
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
#[doc = "Values that can be written to the field `MRT0`"]
pub enum MRT0W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl MRT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MRT0W::DISABLE => false,
            MRT0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MRT0W<'a> {
    w: &'a mut W,
}
impl<'a> _MRT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MRT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(MRT0W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(MRT0W::ENABLE)
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
#[doc = "Values that can be written to the field `CTIMER0`"]
pub enum CTIMER0W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl CTIMER0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTIMER0W::DISABLE => false,
            CTIMER0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTIMER0W<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTIMER0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTIMER0W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTIMER0W::ENABLE)
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
#[doc = "Values that can be written to the field `CTIMER1`"]
pub enum CTIMER1W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl CTIMER1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTIMER1W::DISABLE => false,
            CTIMER1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTIMER1W<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTIMER1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTIMER1W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTIMER1W::ENABLE)
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
#[doc = "Values that can be written to the field `SCT0`"]
pub enum SCT0W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl SCT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCT0W::DISABLE => false,
            SCT0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCT0W<'a> {
    w: &'a mut W,
}
impl<'a> _SCT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCT0W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCT0W::ENABLE)
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
#[doc = "Values that can be written to the field `CTIMER3`"]
pub enum CTIMER3W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl CTIMER3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTIMER3W::DISABLE => false,
            CTIMER3W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTIMER3W<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTIMER3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTIMER3W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTIMER3W::ENABLE)
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
#[doc = "Values that can be written to the field `FLEXINT0`"]
pub enum FLEXINT0W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl FLEXINT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXINT0W::DISABLE => false,
            FLEXINT0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXINT0W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXINT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXINT0W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXINT0W::ENABLE)
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
#[doc = "Values that can be written to the field `FLEXINT1`"]
pub enum FLEXINT1W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl FLEXINT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXINT1W::DISABLE => false,
            FLEXINT1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXINT1W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXINT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXINT1W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXINT1W::ENABLE)
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
#[doc = "Values that can be written to the field `FLEXINT2`"]
pub enum FLEXINT2W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl FLEXINT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXINT2W::DISABLE => false,
            FLEXINT2W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXINT2W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXINT2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXINT2W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXINT2W::ENABLE)
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
#[doc = "Values that can be written to the field `FLEXINT3`"]
pub enum FLEXINT3W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl FLEXINT3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXINT3W::DISABLE => false,
            FLEXINT3W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXINT3W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXINT3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXINT3W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXINT3W::ENABLE)
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
#[doc = "Values that can be written to the field `FLEXINT4`"]
pub enum FLEXINT4W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl FLEXINT4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXINT4W::DISABLE => false,
            FLEXINT4W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXINT4W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXINT4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXINT4W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXINT4W::ENABLE)
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
#[doc = "Values that can be written to the field `FLEXINT5`"]
pub enum FLEXINT5W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl FLEXINT5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXINT5W::DISABLE => false,
            FLEXINT5W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXINT5W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXINT5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXINT5W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXINT5W::ENABLE)
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
#[doc = "Values that can be written to the field `FLEXINT6`"]
pub enum FLEXINT6W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl FLEXINT6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXINT6W::DISABLE => false,
            FLEXINT6W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXINT6W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXINT6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXINT6W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXINT6W::ENABLE)
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
#[doc = "Values that can be written to the field `FLEXINT7`"]
pub enum FLEXINT7W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl FLEXINT7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXINT7W::DISABLE => false,
            FLEXINT7W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXINT7W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXINT7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXINT7W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXINT7W::ENABLE)
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
#[doc = "Values that can be written to the field `ADC0`"]
pub enum ADC0W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl ADC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC0W::DISABLE => false,
            ADC0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC0W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC0W::ENABLE)
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
#[doc = "Values that can be written to the field `ADC0_THCMP_OVR`"]
pub enum ADC0_THCMP_OVRW {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl ADC0_THCMP_OVRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC0_THCMP_OVRW::DISABLE => false,
            ADC0_THCMP_OVRW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC0_THCMP_OVRW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0_THCMP_OVRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC0_THCMP_OVRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC0_THCMP_OVRW::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC0_THCMP_OVRW::ENABLE)
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
#[doc = "Values that can be written to the field `USB0_NEEDCLK`"]
pub enum USB0_NEEDCLKW {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl USB0_NEEDCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB0_NEEDCLKW::DISABLE => false,
            USB0_NEEDCLKW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB0_NEEDCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_NEEDCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB0_NEEDCLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB0_NEEDCLKW::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB0_NEEDCLKW::ENABLE)
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
#[doc = "Values that can be written to the field `USB0`"]
pub enum USB0W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl USB0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB0W::DISABLE => false,
            USB0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB0W<'a> {
    w: &'a mut W,
}
impl<'a> _USB0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB0W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB0W::ENABLE)
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
#[doc = "Values that can be written to the field `RTC_LITE0`"]
pub enum RTC_LITE0W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl RTC_LITE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_LITE0W::DISABLE => false,
            RTC_LITE0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_LITE0W<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_LITE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_LITE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTC_LITE0W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTC_LITE0W::ENABLE)
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
#[doc = "Values that can be written to the field `EZH_ARCH_B0`"]
pub enum EZH_ARCH_B0W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl EZH_ARCH_B0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EZH_ARCH_B0W::DISABLE => false,
            EZH_ARCH_B0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EZH_ARCH_B0W<'a> {
    w: &'a mut W,
}
impl<'a> _EZH_ARCH_B0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EZH_ARCH_B0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(EZH_ARCH_B0W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(EZH_ARCH_B0W::ENABLE)
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
#[doc = "Values that can be written to the field `WAKEUP_MAILBOX0`"]
pub enum WAKEUP_MAILBOX0W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl WAKEUP_MAILBOX0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUP_MAILBOX0W::DISABLE => false,
            WAKEUP_MAILBOX0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUP_MAILBOX0W<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP_MAILBOX0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUP_MAILBOX0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WAKEUP_MAILBOX0W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WAKEUP_MAILBOX0W::ENABLE)
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
    #[doc = "Bit 0 - SYS interrupt wake-up."]
    #[inline]
    pub fn sys(&self) -> SYSR {
        SYSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - SDMA0 interrupt wake-up."]
    #[inline]
    pub fn sdma0(&self) -> SDMA0R {
        SDMA0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - GINT0 interrupt wake-up."]
    #[inline]
    pub fn gint0(&self) -> GINT0R {
        GINT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - GINT1 interrupt wake-up."]
    #[inline]
    pub fn gint1(&self) -> GINT1R {
        GINT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - PIO_INT0 interrupt wake-up."]
    #[inline]
    pub fn pio_int0(&self) -> PIO_INT0R {
        PIO_INT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - PIO_INT1 interrupt wake-up."]
    #[inline]
    pub fn pio_int1(&self) -> PIO_INT1R {
        PIO_INT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - PIO_INT2 interrupt wake-up."]
    #[inline]
    pub fn pio_int2(&self) -> PIO_INT2R {
        PIO_INT2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - PIO_INT3 interrupt wake-up."]
    #[inline]
    pub fn pio_int3(&self) -> PIO_INT3R {
        PIO_INT3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - UTICK0 interrupt wake-up."]
    #[inline]
    pub fn utick0(&self) -> UTICK0R {
        UTICK0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - MRT0 interrupt wake-up."]
    #[inline]
    pub fn mrt0(&self) -> MRT0R {
        MRT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - CTIMER0 interrupt wake-up."]
    #[inline]
    pub fn ctimer0(&self) -> CTIMER0R {
        CTIMER0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - CTIMER1 interrupt wake-up."]
    #[inline]
    pub fn ctimer1(&self) -> CTIMER1R {
        CTIMER1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - SCT0 interrupt wake-up."]
    #[inline]
    pub fn sct0(&self) -> SCT0R {
        SCT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - CTIMER3 interrupt wake-up."]
    #[inline]
    pub fn ctimer3(&self) -> CTIMER3R {
        CTIMER3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - FLEXINT0 interrupt wake-up."]
    #[inline]
    pub fn flexint0(&self) -> FLEXINT0R {
        FLEXINT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - FLEXINT1 interrupt wake-up."]
    #[inline]
    pub fn flexint1(&self) -> FLEXINT1R {
        FLEXINT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - FLEXINT2 interrupt wake-up."]
    #[inline]
    pub fn flexint2(&self) -> FLEXINT2R {
        FLEXINT2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - FLEXINT3 interrupt wake-up."]
    #[inline]
    pub fn flexint3(&self) -> FLEXINT3R {
        FLEXINT3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - FLEXINT4 interrupt wake-up."]
    #[inline]
    pub fn flexint4(&self) -> FLEXINT4R {
        FLEXINT4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - FLEXINT5 interrupt wake-up."]
    #[inline]
    pub fn flexint5(&self) -> FLEXINT5R {
        FLEXINT5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - FLEXINT6 interrupt wake-up."]
    #[inline]
    pub fn flexint6(&self) -> FLEXINT6R {
        FLEXINT6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - FLEXINT7 interrupt wake-up."]
    #[inline]
    pub fn flexint7(&self) -> FLEXINT7R {
        FLEXINT7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - ADC0 interrupt wake-up."]
    #[inline]
    pub fn adc0(&self) -> ADC0R {
        ADC0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - ADC0_THCMP_OVR interrupt wake-up."]
    #[inline]
    pub fn adc0_thcmp_ovr(&self) -> ADC0_THCMP_OVRR {
        ADC0_THCMP_OVRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - USB0_NEEDCLK interrupt wake-up."]
    #[inline]
    pub fn usb0_needclk(&self) -> USB0_NEEDCLKR {
        USB0_NEEDCLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - USB0 interrupt wake-up."]
    #[inline]
    pub fn usb0(&self) -> USB0R {
        USB0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - RTC_LITE0 interrupt wake-up."]
    #[inline]
    pub fn rtc_lite0(&self) -> RTC_LITE0R {
        RTC_LITE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - EZH_ARCH_B0 interrupt wake-up."]
    #[inline]
    pub fn ezh_arch_b0(&self) -> EZH_ARCH_B0R {
        EZH_ARCH_B0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - WAKEUP_MAILBOX0 interrupt wake-up."]
    #[inline]
    pub fn wakeup_mailbox0(&self) -> WAKEUP_MAILBOX0R {
        WAKEUP_MAILBOX0R::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SYS interrupt wake-up."]
    #[inline]
    pub fn sys(&mut self) -> _SYSW {
        _SYSW { w: self }
    }
    #[doc = "Bit 1 - SDMA0 interrupt wake-up."]
    #[inline]
    pub fn sdma0(&mut self) -> _SDMA0W {
        _SDMA0W { w: self }
    }
    #[doc = "Bit 2 - GINT0 interrupt wake-up."]
    #[inline]
    pub fn gint0(&mut self) -> _GINT0W {
        _GINT0W { w: self }
    }
    #[doc = "Bit 3 - GINT1 interrupt wake-up."]
    #[inline]
    pub fn gint1(&mut self) -> _GINT1W {
        _GINT1W { w: self }
    }
    #[doc = "Bit 4 - PIO_INT0 interrupt wake-up."]
    #[inline]
    pub fn pio_int0(&mut self) -> _PIO_INT0W {
        _PIO_INT0W { w: self }
    }
    #[doc = "Bit 5 - PIO_INT1 interrupt wake-up."]
    #[inline]
    pub fn pio_int1(&mut self) -> _PIO_INT1W {
        _PIO_INT1W { w: self }
    }
    #[doc = "Bit 6 - PIO_INT2 interrupt wake-up."]
    #[inline]
    pub fn pio_int2(&mut self) -> _PIO_INT2W {
        _PIO_INT2W { w: self }
    }
    #[doc = "Bit 7 - PIO_INT3 interrupt wake-up."]
    #[inline]
    pub fn pio_int3(&mut self) -> _PIO_INT3W {
        _PIO_INT3W { w: self }
    }
    #[doc = "Bit 8 - UTICK0 interrupt wake-up."]
    #[inline]
    pub fn utick0(&mut self) -> _UTICK0W {
        _UTICK0W { w: self }
    }
    #[doc = "Bit 9 - MRT0 interrupt wake-up."]
    #[inline]
    pub fn mrt0(&mut self) -> _MRT0W {
        _MRT0W { w: self }
    }
    #[doc = "Bit 10 - CTIMER0 interrupt wake-up."]
    #[inline]
    pub fn ctimer0(&mut self) -> _CTIMER0W {
        _CTIMER0W { w: self }
    }
    #[doc = "Bit 11 - CTIMER1 interrupt wake-up."]
    #[inline]
    pub fn ctimer1(&mut self) -> _CTIMER1W {
        _CTIMER1W { w: self }
    }
    #[doc = "Bit 12 - SCT0 interrupt wake-up."]
    #[inline]
    pub fn sct0(&mut self) -> _SCT0W {
        _SCT0W { w: self }
    }
    #[doc = "Bit 13 - CTIMER3 interrupt wake-up."]
    #[inline]
    pub fn ctimer3(&mut self) -> _CTIMER3W {
        _CTIMER3W { w: self }
    }
    #[doc = "Bit 14 - FLEXINT0 interrupt wake-up."]
    #[inline]
    pub fn flexint0(&mut self) -> _FLEXINT0W {
        _FLEXINT0W { w: self }
    }
    #[doc = "Bit 15 - FLEXINT1 interrupt wake-up."]
    #[inline]
    pub fn flexint1(&mut self) -> _FLEXINT1W {
        _FLEXINT1W { w: self }
    }
    #[doc = "Bit 16 - FLEXINT2 interrupt wake-up."]
    #[inline]
    pub fn flexint2(&mut self) -> _FLEXINT2W {
        _FLEXINT2W { w: self }
    }
    #[doc = "Bit 17 - FLEXINT3 interrupt wake-up."]
    #[inline]
    pub fn flexint3(&mut self) -> _FLEXINT3W {
        _FLEXINT3W { w: self }
    }
    #[doc = "Bit 18 - FLEXINT4 interrupt wake-up."]
    #[inline]
    pub fn flexint4(&mut self) -> _FLEXINT4W {
        _FLEXINT4W { w: self }
    }
    #[doc = "Bit 19 - FLEXINT5 interrupt wake-up."]
    #[inline]
    pub fn flexint5(&mut self) -> _FLEXINT5W {
        _FLEXINT5W { w: self }
    }
    #[doc = "Bit 20 - FLEXINT6 interrupt wake-up."]
    #[inline]
    pub fn flexint6(&mut self) -> _FLEXINT6W {
        _FLEXINT6W { w: self }
    }
    #[doc = "Bit 21 - FLEXINT7 interrupt wake-up."]
    #[inline]
    pub fn flexint7(&mut self) -> _FLEXINT7W {
        _FLEXINT7W { w: self }
    }
    #[doc = "Bit 22 - ADC0 interrupt wake-up."]
    #[inline]
    pub fn adc0(&mut self) -> _ADC0W {
        _ADC0W { w: self }
    }
    #[doc = "Bit 24 - ADC0_THCMP_OVR interrupt wake-up."]
    #[inline]
    pub fn adc0_thcmp_ovr(&mut self) -> _ADC0_THCMP_OVRW {
        _ADC0_THCMP_OVRW { w: self }
    }
    #[doc = "Bit 27 - USB0_NEEDCLK interrupt wake-up."]
    #[inline]
    pub fn usb0_needclk(&mut self) -> _USB0_NEEDCLKW {
        _USB0_NEEDCLKW { w: self }
    }
    #[doc = "Bit 28 - USB0 interrupt wake-up."]
    #[inline]
    pub fn usb0(&mut self) -> _USB0W {
        _USB0W { w: self }
    }
    #[doc = "Bit 29 - RTC_LITE0 interrupt wake-up."]
    #[inline]
    pub fn rtc_lite0(&mut self) -> _RTC_LITE0W {
        _RTC_LITE0W { w: self }
    }
    #[doc = "Bit 30 - EZH_ARCH_B0 interrupt wake-up."]
    #[inline]
    pub fn ezh_arch_b0(&mut self) -> _EZH_ARCH_B0W {
        _EZH_ARCH_B0W { w: self }
    }
    #[doc = "Bit 31 - WAKEUP_MAILBOX0 interrupt wake-up."]
    #[inline]
    pub fn wakeup_mailbox0(&mut self) -> _WAKEUP_MAILBOX0W {
        _WAKEUP_MAILBOX0W { w: self }
    }
}
