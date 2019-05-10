#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRESETCTRL1 {
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
#[doc = "Possible values of the field `MRT_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRT_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl MRT_RSTR {
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
            MRT_RSTR::RELEASED => false,
            MRT_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MRT_RSTR {
        match value {
            false => MRT_RSTR::RELEASED,
            true => MRT_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == MRT_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == MRT_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `OSTIMER0_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSTIMER0_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl OSTIMER0_RSTR {
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
            OSTIMER0_RSTR::RELEASED => false,
            OSTIMER0_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSTIMER0_RSTR {
        match value {
            false => OSTIMER0_RSTR::RELEASED,
            true => OSTIMER0_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == OSTIMER0_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == OSTIMER0_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `SCT0_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT0_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl SCT0_RSTR {
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
            SCT0_RSTR::RELEASED => false,
            SCT0_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCT0_RSTR {
        match value {
            false => SCT0_RSTR::RELEASED,
            true => SCT0_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == SCT0_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == SCT0_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `SCTIPU_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCTIPU_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl SCTIPU_RSTR {
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
            SCTIPU_RSTR::RELEASED => false,
            SCTIPU_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCTIPU_RSTR {
        match value {
            false => SCTIPU_RSTR::RELEASED,
            true => SCTIPU_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == SCTIPU_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == SCTIPU_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `UTICK0_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UTICK0_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl UTICK0_RSTR {
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
            UTICK0_RSTR::RELEASED => false,
            UTICK0_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UTICK0_RSTR {
        match value {
            false => UTICK0_RSTR::RELEASED,
            true => UTICK0_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == UTICK0_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == UTICK0_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `FC0_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC0_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FC0_RSTR {
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
            FC0_RSTR::RELEASED => false,
            FC0_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC0_RSTR {
        match value {
            false => FC0_RSTR::RELEASED,
            true => FC0_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == FC0_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == FC0_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `FC1_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC1_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FC1_RSTR {
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
            FC1_RSTR::RELEASED => false,
            FC1_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC1_RSTR {
        match value {
            false => FC1_RSTR::RELEASED,
            true => FC1_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == FC1_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == FC1_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `FC2_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC2_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FC2_RSTR {
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
            FC2_RSTR::RELEASED => false,
            FC2_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC2_RSTR {
        match value {
            false => FC2_RSTR::RELEASED,
            true => FC2_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == FC2_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == FC2_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `FC3_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC3_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FC3_RSTR {
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
            FC3_RSTR::RELEASED => false,
            FC3_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC3_RSTR {
        match value {
            false => FC3_RSTR::RELEASED,
            true => FC3_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == FC3_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == FC3_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `FC4_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC4_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FC4_RSTR {
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
            FC4_RSTR::RELEASED => false,
            FC4_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC4_RSTR {
        match value {
            false => FC4_RSTR::RELEASED,
            true => FC4_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == FC4_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == FC4_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `FC5_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC5_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FC5_RSTR {
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
            FC5_RSTR::RELEASED => false,
            FC5_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC5_RSTR {
        match value {
            false => FC5_RSTR::RELEASED,
            true => FC5_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == FC5_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == FC5_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `FC6_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC6_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FC6_RSTR {
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
            FC6_RSTR::RELEASED => false,
            FC6_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC6_RSTR {
        match value {
            false => FC6_RSTR::RELEASED,
            true => FC6_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == FC6_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == FC6_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `FC7_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC7_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FC7_RSTR {
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
            FC7_RSTR::RELEASED => false,
            FC7_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC7_RSTR {
        match value {
            false => FC7_RSTR::RELEASED,
            true => FC7_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == FC7_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == FC7_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `TIMER2_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER2_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl TIMER2_RSTR {
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
            TIMER2_RSTR::RELEASED => false,
            TIMER2_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMER2_RSTR {
        match value {
            false => TIMER2_RSTR::RELEASED,
            true => TIMER2_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == TIMER2_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == TIMER2_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `USB0_DEV_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_DEV_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl USB0_DEV_RSTR {
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
            USB0_DEV_RSTR::RELEASED => false,
            USB0_DEV_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB0_DEV_RSTR {
        match value {
            false => USB0_DEV_RSTR::RELEASED,
            true => USB0_DEV_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == USB0_DEV_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == USB0_DEV_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `TIMER0_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER0_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl TIMER0_RSTR {
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
            TIMER0_RSTR::RELEASED => false,
            TIMER0_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMER0_RSTR {
        match value {
            false => TIMER0_RSTR::RELEASED,
            true => TIMER0_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == TIMER0_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == TIMER0_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `TIMER1_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER1_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl TIMER1_RSTR {
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
            TIMER1_RSTR::RELEASED => false,
            TIMER1_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMER1_RSTR {
        match value {
            false => TIMER1_RSTR::RELEASED,
            true => TIMER1_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == TIMER1_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == TIMER1_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `PVT_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PVT_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl PVT_RSTR {
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
            PVT_RSTR::RELEASED => false,
            PVT_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PVT_RSTR {
        match value {
            false => PVT_RSTR::RELEASED,
            true => PVT_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == PVT_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == PVT_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `EZHA_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EZHA_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl EZHA_RSTR {
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
            EZHA_RSTR::RELEASED => false,
            EZHA_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EZHA_RSTR {
        match value {
            false => EZHA_RSTR::RELEASED,
            true => EZHA_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == EZHA_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == EZHA_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `EZHB_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EZHB_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl EZHB_RSTR {
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
            EZHB_RSTR::RELEASED => false,
            EZHB_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EZHB_RSTR {
        match value {
            false => EZHB_RSTR::RELEASED,
            true => EZHB_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == EZHB_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == EZHB_RSTR::ASSERTED
    }
}
#[doc = "Values that can be written to the field `MRT_RST`"]
pub enum MRT_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl MRT_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MRT_RSTW::RELEASED => false,
            MRT_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MRT_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MRT_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MRT_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(MRT_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(MRT_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `OSTIMER0_RST`"]
pub enum OSTIMER0_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl OSTIMER0_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OSTIMER0_RSTW::RELEASED => false,
            OSTIMER0_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSTIMER0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _OSTIMER0_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSTIMER0_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(OSTIMER0_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(OSTIMER0_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `SCT0_RST`"]
pub enum SCT0_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl SCT0_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCT0_RSTW::RELEASED => false,
            SCT0_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCT0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SCT0_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCT0_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(SCT0_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SCT0_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `SCTIPU_RST`"]
pub enum SCTIPU_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl SCTIPU_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCTIPU_RSTW::RELEASED => false,
            SCTIPU_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCTIPU_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SCTIPU_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCTIPU_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(SCTIPU_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SCTIPU_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `UTICK0_RST`"]
pub enum UTICK0_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl UTICK0_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UTICK0_RSTW::RELEASED => false,
            UTICK0_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UTICK0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _UTICK0_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UTICK0_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(UTICK0_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(UTICK0_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `FC0_RST`"]
pub enum FC0_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FC0_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC0_RSTW::RELEASED => false,
            FC0_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FC0_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC0_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(FC0_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC0_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `FC1_RST`"]
pub enum FC1_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FC1_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC1_RSTW::RELEASED => false,
            FC1_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC1_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FC1_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC1_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(FC1_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC1_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `FC2_RST`"]
pub enum FC2_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FC2_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC2_RSTW::RELEASED => false,
            FC2_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC2_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FC2_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC2_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(FC2_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC2_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `FC3_RST`"]
pub enum FC3_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FC3_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC3_RSTW::RELEASED => false,
            FC3_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC3_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FC3_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC3_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(FC3_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC3_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `FC4_RST`"]
pub enum FC4_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FC4_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC4_RSTW::RELEASED => false,
            FC4_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC4_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FC4_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC4_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(FC4_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC4_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `FC5_RST`"]
pub enum FC5_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FC5_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC5_RSTW::RELEASED => false,
            FC5_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC5_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FC5_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC5_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(FC5_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC5_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `FC6_RST`"]
pub enum FC6_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FC6_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC6_RSTW::RELEASED => false,
            FC6_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC6_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FC6_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC6_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(FC6_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC6_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `FC7_RST`"]
pub enum FC7_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FC7_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC7_RSTW::RELEASED => false,
            FC7_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC7_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FC7_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC7_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(FC7_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC7_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `TIMER2_RST`"]
pub enum TIMER2_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl TIMER2_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMER2_RSTW::RELEASED => false,
            TIMER2_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMER2_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER2_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMER2_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(TIMER2_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TIMER2_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `USB0_DEV_RST`"]
pub enum USB0_DEV_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl USB0_DEV_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB0_DEV_RSTW::RELEASED => false,
            USB0_DEV_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB0_DEV_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_DEV_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB0_DEV_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(USB0_DEV_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB0_DEV_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `TIMER0_RST`"]
pub enum TIMER0_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl TIMER0_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMER0_RSTW::RELEASED => false,
            TIMER0_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMER0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER0_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMER0_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(TIMER0_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TIMER0_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `TIMER1_RST`"]
pub enum TIMER1_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl TIMER1_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMER1_RSTW::RELEASED => false,
            TIMER1_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMER1_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER1_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMER1_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(TIMER1_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TIMER1_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `PVT_RST`"]
pub enum PVT_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl PVT_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PVT_RSTW::RELEASED => false,
            PVT_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PVT_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PVT_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PVT_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(PVT_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(PVT_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `EZHA_RST`"]
pub enum EZHA_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl EZHA_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EZHA_RSTW::RELEASED => false,
            EZHA_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EZHA_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _EZHA_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EZHA_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(EZHA_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(EZHA_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `EZHB_RST`"]
pub enum EZHB_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl EZHB_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EZHB_RSTW::RELEASED => false,
            EZHB_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EZHB_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _EZHB_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EZHB_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(EZHB_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(EZHB_RSTW::ASSERTED)
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
    #[doc = "Bit 0 - MRT reset control."]
    #[inline]
    pub fn mrt_rst(&self) -> MRT_RSTR {
        MRT_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - OS Timer 0 reset control."]
    #[inline]
    pub fn ostimer0_rst(&self) -> OSTIMER0_RSTR {
        OSTIMER0_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - SCT0 reset control."]
    #[inline]
    pub fn sct0_rst(&self) -> SCT0_RSTR {
        SCT0_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - SCTIPU reset control."]
    #[inline]
    pub fn sctipu_rst(&self) -> SCTIPU_RSTR {
        SCTIPU_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - UTICK0 reset control."]
    #[inline]
    pub fn utick0_rst(&self) -> UTICK0_RSTR {
        UTICK0_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - FC0 reset control."]
    #[inline]
    pub fn fc0_rst(&self) -> FC0_RSTR {
        FC0_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - FC1 reset control."]
    #[inline]
    pub fn fc1_rst(&self) -> FC1_RSTR {
        FC1_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - FC2 reset control."]
    #[inline]
    pub fn fc2_rst(&self) -> FC2_RSTR {
        FC2_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - FC3 reset control."]
    #[inline]
    pub fn fc3_rst(&self) -> FC3_RSTR {
        FC3_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - FC4 reset control."]
    #[inline]
    pub fn fc4_rst(&self) -> FC4_RSTR {
        FC4_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - FC5 reset control."]
    #[inline]
    pub fn fc5_rst(&self) -> FC5_RSTR {
        FC5_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - FC6 reset control."]
    #[inline]
    pub fn fc6_rst(&self) -> FC6_RSTR {
        FC6_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - FC7 reset control."]
    #[inline]
    pub fn fc7_rst(&self) -> FC7_RSTR {
        FC7_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Timer 2 reset control."]
    #[inline]
    pub fn timer2_rst(&self) -> TIMER2_RSTR {
        TIMER2_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - USB0 DEV reset control."]
    #[inline]
    pub fn usb0_dev_rst(&self) -> USB0_DEV_RSTR {
        USB0_DEV_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Timer 0 reset control."]
    #[inline]
    pub fn timer0_rst(&self) -> TIMER0_RSTR {
        TIMER0_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Timer 1 reset control."]
    #[inline]
    pub fn timer1_rst(&self) -> TIMER1_RSTR {
        TIMER1_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - PVT reset control."]
    #[inline]
    pub fn pvt_rst(&self) -> PVT_RSTR {
        PVT_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - EZH a reset control."]
    #[inline]
    pub fn ezha_rst(&self) -> EZHA_RSTR {
        EZHA_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - EZH b reset control."]
    #[inline]
    pub fn ezhb_rst(&self) -> EZHB_RSTR {
        EZHB_RSTR::_from({
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
    #[doc = "Bit 0 - MRT reset control."]
    #[inline]
    pub fn mrt_rst(&mut self) -> _MRT_RSTW {
        _MRT_RSTW { w: self }
    }
    #[doc = "Bit 1 - OS Timer 0 reset control."]
    #[inline]
    pub fn ostimer0_rst(&mut self) -> _OSTIMER0_RSTW {
        _OSTIMER0_RSTW { w: self }
    }
    #[doc = "Bit 2 - SCT0 reset control."]
    #[inline]
    pub fn sct0_rst(&mut self) -> _SCT0_RSTW {
        _SCT0_RSTW { w: self }
    }
    #[doc = "Bit 6 - SCTIPU reset control."]
    #[inline]
    pub fn sctipu_rst(&mut self) -> _SCTIPU_RSTW {
        _SCTIPU_RSTW { w: self }
    }
    #[doc = "Bit 10 - UTICK0 reset control."]
    #[inline]
    pub fn utick0_rst(&mut self) -> _UTICK0_RSTW {
        _UTICK0_RSTW { w: self }
    }
    #[doc = "Bit 11 - FC0 reset control."]
    #[inline]
    pub fn fc0_rst(&mut self) -> _FC0_RSTW {
        _FC0_RSTW { w: self }
    }
    #[doc = "Bit 12 - FC1 reset control."]
    #[inline]
    pub fn fc1_rst(&mut self) -> _FC1_RSTW {
        _FC1_RSTW { w: self }
    }
    #[doc = "Bit 13 - FC2 reset control."]
    #[inline]
    pub fn fc2_rst(&mut self) -> _FC2_RSTW {
        _FC2_RSTW { w: self }
    }
    #[doc = "Bit 14 - FC3 reset control."]
    #[inline]
    pub fn fc3_rst(&mut self) -> _FC3_RSTW {
        _FC3_RSTW { w: self }
    }
    #[doc = "Bit 15 - FC4 reset control."]
    #[inline]
    pub fn fc4_rst(&mut self) -> _FC4_RSTW {
        _FC4_RSTW { w: self }
    }
    #[doc = "Bit 16 - FC5 reset control."]
    #[inline]
    pub fn fc5_rst(&mut self) -> _FC5_RSTW {
        _FC5_RSTW { w: self }
    }
    #[doc = "Bit 17 - FC6 reset control."]
    #[inline]
    pub fn fc6_rst(&mut self) -> _FC6_RSTW {
        _FC6_RSTW { w: self }
    }
    #[doc = "Bit 18 - FC7 reset control."]
    #[inline]
    pub fn fc7_rst(&mut self) -> _FC7_RSTW {
        _FC7_RSTW { w: self }
    }
    #[doc = "Bit 22 - Timer 2 reset control."]
    #[inline]
    pub fn timer2_rst(&mut self) -> _TIMER2_RSTW {
        _TIMER2_RSTW { w: self }
    }
    #[doc = "Bit 25 - USB0 DEV reset control."]
    #[inline]
    pub fn usb0_dev_rst(&mut self) -> _USB0_DEV_RSTW {
        _USB0_DEV_RSTW { w: self }
    }
    #[doc = "Bit 26 - Timer 0 reset control."]
    #[inline]
    pub fn timer0_rst(&mut self) -> _TIMER0_RSTW {
        _TIMER0_RSTW { w: self }
    }
    #[doc = "Bit 27 - Timer 1 reset control."]
    #[inline]
    pub fn timer1_rst(&mut self) -> _TIMER1_RSTW {
        _TIMER1_RSTW { w: self }
    }
    #[doc = "Bit 28 - PVT reset control."]
    #[inline]
    pub fn pvt_rst(&mut self) -> _PVT_RSTW {
        _PVT_RSTW { w: self }
    }
    #[doc = "Bit 30 - EZH a reset control."]
    #[inline]
    pub fn ezha_rst(&mut self) -> _EZHA_RSTW {
        _EZHA_RSTW { w: self }
    }
    #[doc = "Bit 31 - EZH b reset control."]
    #[inline]
    pub fn ezhb_rst(&mut self) -> _EZHB_RSTW {
        _EZHB_RSTW { w: self }
    }
}
