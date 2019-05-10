#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHBCLKCTRL1 {
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
#[doc = "Possible values of the field `MRT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRTR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl MRTR {
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
            MRTR::DISABLE => false,
            MRTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MRTR {
        match value {
            false => MRTR::DISABLE,
            true => MRTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == MRTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == MRTR::ENABLE
    }
}
#[doc = "Possible values of the field `OSTIMER0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSTIMER0R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl OSTIMER0R {
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
            OSTIMER0R::DISABLE => false,
            OSTIMER0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSTIMER0R {
        match value {
            false => OSTIMER0R::DISABLE,
            true => OSTIMER0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == OSTIMER0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == OSTIMER0R::ENABLE
    }
}
#[doc = "Possible values of the field `SCT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT0R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
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
#[doc = "Possible values of the field `SCTIPU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCTIPUR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl SCTIPUR {
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
            SCTIPUR::DISABLE => false,
            SCTIPUR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCTIPUR {
        match value {
            false => SCTIPUR::DISABLE,
            true => SCTIPUR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SCTIPUR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SCTIPUR::ENABLE
    }
}
#[doc = "Possible values of the field `UTICK0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UTICK0R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
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
#[doc = "Possible values of the field `FC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC0R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl FC0R {
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
            FC0R::DISABLE => false,
            FC0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC0R {
        match value {
            false => FC0R::DISABLE,
            true => FC0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FC0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FC0R::ENABLE
    }
}
#[doc = "Possible values of the field `FC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC1R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl FC1R {
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
            FC1R::DISABLE => false,
            FC1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC1R {
        match value {
            false => FC1R::DISABLE,
            true => FC1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FC1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FC1R::ENABLE
    }
}
#[doc = "Possible values of the field `FC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC2R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl FC2R {
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
            FC2R::DISABLE => false,
            FC2R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC2R {
        match value {
            false => FC2R::DISABLE,
            true => FC2R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FC2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FC2R::ENABLE
    }
}
#[doc = "Possible values of the field `FC3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC3R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl FC3R {
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
            FC3R::DISABLE => false,
            FC3R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC3R {
        match value {
            false => FC3R::DISABLE,
            true => FC3R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FC3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FC3R::ENABLE
    }
}
#[doc = "Possible values of the field `FC4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC4R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl FC4R {
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
            FC4R::DISABLE => false,
            FC4R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC4R {
        match value {
            false => FC4R::DISABLE,
            true => FC4R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FC4R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FC4R::ENABLE
    }
}
#[doc = "Possible values of the field `FC5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC5R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl FC5R {
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
            FC5R::DISABLE => false,
            FC5R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC5R {
        match value {
            false => FC5R::DISABLE,
            true => FC5R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FC5R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FC5R::ENABLE
    }
}
#[doc = "Possible values of the field `FC6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC6R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl FC6R {
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
            FC6R::DISABLE => false,
            FC6R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC6R {
        match value {
            false => FC6R::DISABLE,
            true => FC6R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FC6R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FC6R::ENABLE
    }
}
#[doc = "Possible values of the field `FC7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC7R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl FC7R {
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
            FC7R::DISABLE => false,
            FC7R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC7R {
        match value {
            false => FC7R::DISABLE,
            true => FC7R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FC7R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FC7R::ENABLE
    }
}
#[doc = "Possible values of the field `TIMER2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER2R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl TIMER2R {
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
            TIMER2R::DISABLE => false,
            TIMER2R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMER2R {
        match value {
            false => TIMER2R::DISABLE,
            true => TIMER2R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TIMER2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TIMER2R::ENABLE
    }
}
#[doc = "Possible values of the field `USB0_DEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_DEVR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl USB0_DEVR {
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
            USB0_DEVR::DISABLE => false,
            USB0_DEVR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB0_DEVR {
        match value {
            false => USB0_DEVR::DISABLE,
            true => USB0_DEVR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == USB0_DEVR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == USB0_DEVR::ENABLE
    }
}
#[doc = "Possible values of the field `TIMER0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER0R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl TIMER0R {
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
            TIMER0R::DISABLE => false,
            TIMER0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMER0R {
        match value {
            false => TIMER0R::DISABLE,
            true => TIMER0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TIMER0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TIMER0R::ENABLE
    }
}
#[doc = "Possible values of the field `TIMER1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER1R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl TIMER1R {
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
            TIMER1R::DISABLE => false,
            TIMER1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMER1R {
        match value {
            false => TIMER1R::DISABLE,
            true => TIMER1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TIMER1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TIMER1R::ENABLE
    }
}
#[doc = "Possible values of the field `PVT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PVTR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl PVTR {
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
            PVTR::DISABLE => false,
            PVTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PVTR {
        match value {
            false => PVTR::DISABLE,
            true => PVTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PVTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PVTR::ENABLE
    }
}
#[doc = "Possible values of the field `EZHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EZHAR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl EZHAR {
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
            EZHAR::DISABLE => false,
            EZHAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EZHAR {
        match value {
            false => EZHAR::DISABLE,
            true => EZHAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == EZHAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == EZHAR::ENABLE
    }
}
#[doc = "Possible values of the field `EZHB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EZHBR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl EZHBR {
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
            EZHBR::DISABLE => false,
            EZHBR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EZHBR {
        match value {
            false => EZHBR::DISABLE,
            true => EZHBR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == EZHBR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == EZHBR::ENABLE
    }
}
#[doc = "Values that can be written to the field `MRT`"]
pub enum MRTW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl MRTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MRTW::DISABLE => false,
            MRTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MRTW<'a> {
    w: &'a mut W,
}
impl<'a> _MRTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MRTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(MRTW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(MRTW::ENABLE)
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
#[doc = "Values that can be written to the field `OSTIMER0`"]
pub enum OSTIMER0W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl OSTIMER0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OSTIMER0W::DISABLE => false,
            OSTIMER0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSTIMER0W<'a> {
    w: &'a mut W,
}
impl<'a> _OSTIMER0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSTIMER0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(OSTIMER0W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(OSTIMER0W::ENABLE)
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
#[doc = "Values that can be written to the field `SCT0`"]
pub enum SCT0W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
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
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCT0W::DISABLE)
    }
    #[doc = "Enable Clock."]
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCTIPU`"]
pub enum SCTIPUW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl SCTIPUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCTIPUW::DISABLE => false,
            SCTIPUW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCTIPUW<'a> {
    w: &'a mut W,
}
impl<'a> _SCTIPUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCTIPUW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCTIPUW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCTIPUW::ENABLE)
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
#[doc = "Values that can be written to the field `UTICK0`"]
pub enum UTICK0W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
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
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(UTICK0W::DISABLE)
    }
    #[doc = "Enable Clock."]
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FC0`"]
pub enum FC0W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl FC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC0W::DISABLE => false,
            FC0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC0W<'a> {
    w: &'a mut W,
}
impl<'a> _FC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC0W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC0W::ENABLE)
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
#[doc = "Values that can be written to the field `FC1`"]
pub enum FC1W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl FC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC1W::DISABLE => false,
            FC1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC1W<'a> {
    w: &'a mut W,
}
impl<'a> _FC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC1W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC1W::ENABLE)
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
#[doc = "Values that can be written to the field `FC2`"]
pub enum FC2W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl FC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC2W::DISABLE => false,
            FC2W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC2W<'a> {
    w: &'a mut W,
}
impl<'a> _FC2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC2W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC2W::ENABLE)
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
#[doc = "Values that can be written to the field `FC3`"]
pub enum FC3W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl FC3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC3W::DISABLE => false,
            FC3W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC3W<'a> {
    w: &'a mut W,
}
impl<'a> _FC3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC3W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC3W::ENABLE)
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
#[doc = "Values that can be written to the field `FC4`"]
pub enum FC4W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl FC4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC4W::DISABLE => false,
            FC4W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC4W<'a> {
    w: &'a mut W,
}
impl<'a> _FC4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC4W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC4W::ENABLE)
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
#[doc = "Values that can be written to the field `FC5`"]
pub enum FC5W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl FC5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC5W::DISABLE => false,
            FC5W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC5W<'a> {
    w: &'a mut W,
}
impl<'a> _FC5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC5W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC5W::ENABLE)
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
#[doc = "Values that can be written to the field `FC6`"]
pub enum FC6W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl FC6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC6W::DISABLE => false,
            FC6W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC6W<'a> {
    w: &'a mut W,
}
impl<'a> _FC6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC6W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC6W::ENABLE)
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
#[doc = "Values that can be written to the field `FC7`"]
pub enum FC7W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl FC7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC7W::DISABLE => false,
            FC7W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC7W<'a> {
    w: &'a mut W,
}
impl<'a> _FC7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC7W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC7W::ENABLE)
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
#[doc = "Values that can be written to the field `TIMER2`"]
pub enum TIMER2W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl TIMER2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMER2W::DISABLE => false,
            TIMER2W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMER2W<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMER2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER2W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER2W::ENABLE)
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
#[doc = "Values that can be written to the field `USB0_DEV`"]
pub enum USB0_DEVW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl USB0_DEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB0_DEVW::DISABLE => false,
            USB0_DEVW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB0_DEVW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_DEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB0_DEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB0_DEVW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB0_DEVW::ENABLE)
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
#[doc = "Values that can be written to the field `TIMER0`"]
pub enum TIMER0W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl TIMER0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMER0W::DISABLE => false,
            TIMER0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMER0W<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMER0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER0W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER0W::ENABLE)
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
#[doc = "Values that can be written to the field `TIMER1`"]
pub enum TIMER1W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl TIMER1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMER1W::DISABLE => false,
            TIMER1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMER1W<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMER1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER1W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER1W::ENABLE)
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
#[doc = "Values that can be written to the field `PVT`"]
pub enum PVTW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl PVTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PVTW::DISABLE => false,
            PVTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PVTW<'a> {
    w: &'a mut W,
}
impl<'a> _PVTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PVTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PVTW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PVTW::ENABLE)
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
#[doc = "Values that can be written to the field `EZHA`"]
pub enum EZHAW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl EZHAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EZHAW::DISABLE => false,
            EZHAW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EZHAW<'a> {
    w: &'a mut W,
}
impl<'a> _EZHAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EZHAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(EZHAW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(EZHAW::ENABLE)
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
#[doc = "Values that can be written to the field `EZHB`"]
pub enum EZHBW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl EZHBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EZHBW::DISABLE => false,
            EZHBW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EZHBW<'a> {
    w: &'a mut W,
}
impl<'a> _EZHBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EZHBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(EZHBW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(EZHBW::ENABLE)
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
    #[doc = "Bit 0 - Enables the clock for the MRT."]
    #[inline]
    pub fn mrt(&self) -> MRTR {
        MRTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enables the clock for the OS Timer 0."]
    #[inline]
    pub fn ostimer0(&self) -> OSTIMER0R {
        OSTIMER0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enables the clock for the SCT0."]
    #[inline]
    pub fn sct0(&self) -> SCT0R {
        SCT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enables the clock for the SCTIPU."]
    #[inline]
    pub fn sctipu(&self) -> SCTIPUR {
        SCTIPUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Enables the clock for the UTICK0."]
    #[inline]
    pub fn utick0(&self) -> UTICK0R {
        UTICK0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Enables the clock for the FC0."]
    #[inline]
    pub fn fc0(&self) -> FC0R {
        FC0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Enables the clock for the FC1."]
    #[inline]
    pub fn fc1(&self) -> FC1R {
        FC1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Enables the clock for the FC2."]
    #[inline]
    pub fn fc2(&self) -> FC2R {
        FC2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Enables the clock for the FC3."]
    #[inline]
    pub fn fc3(&self) -> FC3R {
        FC3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Enables the clock for the FC4."]
    #[inline]
    pub fn fc4(&self) -> FC4R {
        FC4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Enables the clock for the FC5."]
    #[inline]
    pub fn fc5(&self) -> FC5R {
        FC5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Enables the clock for the FC6."]
    #[inline]
    pub fn fc6(&self) -> FC6R {
        FC6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Enables the clock for the FC7."]
    #[inline]
    pub fn fc7(&self) -> FC7R {
        FC7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Enables the clock for the Timer 2."]
    #[inline]
    pub fn timer2(&self) -> TIMER2R {
        TIMER2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Enables the clock for the USB0 DEV."]
    #[inline]
    pub fn usb0_dev(&self) -> USB0_DEVR {
        USB0_DEVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Enables the clock for the Timer 0."]
    #[inline]
    pub fn timer0(&self) -> TIMER0R {
        TIMER0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Enables the clock for the Timer 1."]
    #[inline]
    pub fn timer1(&self) -> TIMER1R {
        TIMER1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Enables the clock for the PVT."]
    #[inline]
    pub fn pvt(&self) -> PVTR {
        PVTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Enables the clock for the EZH a."]
    #[inline]
    pub fn ezha(&self) -> EZHAR {
        EZHAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Enables the clock for the EZH b."]
    #[inline]
    pub fn ezhb(&self) -> EZHBR {
        EZHBR::_from({
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
    #[doc = "Bit 0 - Enables the clock for the MRT."]
    #[inline]
    pub fn mrt(&mut self) -> _MRTW {
        _MRTW { w: self }
    }
    #[doc = "Bit 1 - Enables the clock for the OS Timer 0."]
    #[inline]
    pub fn ostimer0(&mut self) -> _OSTIMER0W {
        _OSTIMER0W { w: self }
    }
    #[doc = "Bit 2 - Enables the clock for the SCT0."]
    #[inline]
    pub fn sct0(&mut self) -> _SCT0W {
        _SCT0W { w: self }
    }
    #[doc = "Bit 6 - Enables the clock for the SCTIPU."]
    #[inline]
    pub fn sctipu(&mut self) -> _SCTIPUW {
        _SCTIPUW { w: self }
    }
    #[doc = "Bit 10 - Enables the clock for the UTICK0."]
    #[inline]
    pub fn utick0(&mut self) -> _UTICK0W {
        _UTICK0W { w: self }
    }
    #[doc = "Bit 11 - Enables the clock for the FC0."]
    #[inline]
    pub fn fc0(&mut self) -> _FC0W {
        _FC0W { w: self }
    }
    #[doc = "Bit 12 - Enables the clock for the FC1."]
    #[inline]
    pub fn fc1(&mut self) -> _FC1W {
        _FC1W { w: self }
    }
    #[doc = "Bit 13 - Enables the clock for the FC2."]
    #[inline]
    pub fn fc2(&mut self) -> _FC2W {
        _FC2W { w: self }
    }
    #[doc = "Bit 14 - Enables the clock for the FC3."]
    #[inline]
    pub fn fc3(&mut self) -> _FC3W {
        _FC3W { w: self }
    }
    #[doc = "Bit 15 - Enables the clock for the FC4."]
    #[inline]
    pub fn fc4(&mut self) -> _FC4W {
        _FC4W { w: self }
    }
    #[doc = "Bit 16 - Enables the clock for the FC5."]
    #[inline]
    pub fn fc5(&mut self) -> _FC5W {
        _FC5W { w: self }
    }
    #[doc = "Bit 17 - Enables the clock for the FC6."]
    #[inline]
    pub fn fc6(&mut self) -> _FC6W {
        _FC6W { w: self }
    }
    #[doc = "Bit 18 - Enables the clock for the FC7."]
    #[inline]
    pub fn fc7(&mut self) -> _FC7W {
        _FC7W { w: self }
    }
    #[doc = "Bit 22 - Enables the clock for the Timer 2."]
    #[inline]
    pub fn timer2(&mut self) -> _TIMER2W {
        _TIMER2W { w: self }
    }
    #[doc = "Bit 25 - Enables the clock for the USB0 DEV."]
    #[inline]
    pub fn usb0_dev(&mut self) -> _USB0_DEVW {
        _USB0_DEVW { w: self }
    }
    #[doc = "Bit 26 - Enables the clock for the Timer 0."]
    #[inline]
    pub fn timer0(&mut self) -> _TIMER0W {
        _TIMER0W { w: self }
    }
    #[doc = "Bit 27 - Enables the clock for the Timer 1."]
    #[inline]
    pub fn timer1(&mut self) -> _TIMER1W {
        _TIMER1W { w: self }
    }
    #[doc = "Bit 28 - Enables the clock for the PVT."]
    #[inline]
    pub fn pvt(&mut self) -> _PVTW {
        _PVTW { w: self }
    }
    #[doc = "Bit 30 - Enables the clock for the EZH a."]
    #[inline]
    pub fn ezha(&mut self) -> _EZHAW {
        _EZHAW { w: self }
    }
    #[doc = "Bit 31 - Enables the clock for the EZH b."]
    #[inline]
    pub fn ezhb(&mut self) -> _EZHBW {
        _EZHBW { w: self }
    }
}
