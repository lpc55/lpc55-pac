#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STARTER1 {
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
#[doc = "Possible values of the field `GPIO_INT04`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT04R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl GPIO_INT04R {
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
            GPIO_INT04R::DISABLE => false,
            GPIO_INT04R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO_INT04R {
        match value {
            false => GPIO_INT04R::DISABLE,
            true => GPIO_INT04R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INT04R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INT04R::ENABLE
    }
}
#[doc = "Possible values of the field `GPIO_INT05`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT05R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl GPIO_INT05R {
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
            GPIO_INT05R::DISABLE => false,
            GPIO_INT05R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO_INT05R {
        match value {
            false => GPIO_INT05R::DISABLE,
            true => GPIO_INT05R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INT05R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INT05R::ENABLE
    }
}
#[doc = "Possible values of the field `GPIO_INT06`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT06R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl GPIO_INT06R {
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
            GPIO_INT06R::DISABLE => false,
            GPIO_INT06R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO_INT06R {
        match value {
            false => GPIO_INT06R::DISABLE,
            true => GPIO_INT06R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INT06R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INT06R::ENABLE
    }
}
#[doc = "Possible values of the field `GPIO_INT07`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT07R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl GPIO_INT07R {
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
            GPIO_INT07R::DISABLE => false,
            GPIO_INT07R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO_INT07R {
        match value {
            false => GPIO_INT07R::DISABLE,
            true => GPIO_INT07R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INT07R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INT07R::ENABLE
    }
}
#[doc = "Possible values of the field `CTIMER2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER2R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl CTIMER2R {
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
            CTIMER2R::DISABLE => false,
            CTIMER2R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTIMER2R {
        match value {
            false => CTIMER2R::DISABLE,
            true => CTIMER2R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CTIMER2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CTIMER2R::ENABLE
    }
}
#[doc = "Possible values of the field `CTIMER4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER4R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl CTIMER4R {
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
            CTIMER4R::DISABLE => false,
            CTIMER4R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTIMER4R {
        match value {
            false => CTIMER4R::DISABLE,
            true => CTIMER4R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CTIMER4R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CTIMER4R::ENABLE
    }
}
#[doc = "Possible values of the field `OS_EVENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OS_EVENTR {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl OS_EVENTR {
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
            OS_EVENTR::DISABLE => false,
            OS_EVENTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OS_EVENTR {
        match value {
            false => OS_EVENTR::DISABLE,
            true => OS_EVENTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == OS_EVENTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == OS_EVENTR::ENABLE
    }
}
#[doc = "Possible values of the field `SDIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIOR {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl SDIOR {
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
            SDIOR::DISABLE => false,
            SDIOR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDIOR {
        match value {
            false => SDIOR::DISABLE,
            true => SDIOR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SDIOR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SDIOR::ENABLE
    }
}
#[doc = "Possible values of the field `USB1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl USB1R {
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
            USB1R::DISABLE => false,
            USB1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB1R {
        match value {
            false => USB1R::DISABLE,
            true => USB1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == USB1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == USB1R::ENABLE
    }
}
#[doc = "Possible values of the field `USB1_NEEDCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_NEEDCLKR {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl USB1_NEEDCLKR {
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
            USB1_NEEDCLKR::DISABLE => false,
            USB1_NEEDCLKR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB1_NEEDCLKR {
        match value {
            false => USB1_NEEDCLKR::DISABLE,
            true => USB1_NEEDCLKR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == USB1_NEEDCLKR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == USB1_NEEDCLKR::ENABLE
    }
}
#[doc = "Possible values of the field `SEC_HYPERVISOR_CALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_HYPERVISOR_CALLR {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl SEC_HYPERVISOR_CALLR {
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
            SEC_HYPERVISOR_CALLR::DISABLE => false,
            SEC_HYPERVISOR_CALLR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEC_HYPERVISOR_CALLR {
        match value {
            false => SEC_HYPERVISOR_CALLR::DISABLE,
            true => SEC_HYPERVISOR_CALLR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SEC_HYPERVISOR_CALLR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SEC_HYPERVISOR_CALLR::ENABLE
    }
}
#[doc = "Possible values of the field `SEC_GPIO_INT00`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_GPIO_INT00R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl SEC_GPIO_INT00R {
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
            SEC_GPIO_INT00R::DISABLE => false,
            SEC_GPIO_INT00R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEC_GPIO_INT00R {
        match value {
            false => SEC_GPIO_INT00R::DISABLE,
            true => SEC_GPIO_INT00R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SEC_GPIO_INT00R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SEC_GPIO_INT00R::ENABLE
    }
}
#[doc = "Possible values of the field `SEC_GPIO_INT01`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_GPIO_INT01R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl SEC_GPIO_INT01R {
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
            SEC_GPIO_INT01R::DISABLE => false,
            SEC_GPIO_INT01R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEC_GPIO_INT01R {
        match value {
            false => SEC_GPIO_INT01R::DISABLE,
            true => SEC_GPIO_INT01R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SEC_GPIO_INT01R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SEC_GPIO_INT01R::ENABLE
    }
}
#[doc = "Possible values of the field `PLU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLUR {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl PLUR {
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
            PLUR::DISABLE => false,
            PLUR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLUR {
        match value {
            false => PLUR::DISABLE,
            true => PLUR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PLUR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PLUR::ENABLE
    }
}
#[doc = "Possible values of the field `SEC_VIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_VIOR {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl SEC_VIOR {
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
            SEC_VIOR::DISABLE => false,
            SEC_VIOR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEC_VIOR {
        match value {
            false => SEC_VIOR::DISABLE,
            true => SEC_VIOR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SEC_VIOR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SEC_VIOR::ENABLE
    }
}
#[doc = "Possible values of the field `SHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHAR {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl SHAR {
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
            SHAR::DISABLE => false,
            SHAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SHAR {
        match value {
            false => SHAR::DISABLE,
            true => SHAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SHAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SHAR::ENABLE
    }
}
#[doc = "Possible values of the field `CASER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CASERR {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl CASERR {
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
            CASERR::DISABLE => false,
            CASERR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CASERR {
        match value {
            false => CASERR::DISABLE,
            true => CASERR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CASERR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CASERR::ENABLE
    }
}
#[doc = "Possible values of the field `QDDKEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QDDKEYR {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl QDDKEYR {
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
            QDDKEYR::DISABLE => false,
            QDDKEYR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QDDKEYR {
        match value {
            false => QDDKEYR::DISABLE,
            true => QDDKEYR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == QDDKEYR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == QDDKEYR::ENABLE
    }
}
#[doc = "Possible values of the field `PQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PQR {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl PQR {
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
            PQR::DISABLE => false,
            PQR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PQR {
        match value {
            false => PQR::DISABLE,
            true => PQR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PQR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PQR::ENABLE
    }
}
#[doc = "Possible values of the field `SDMA1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMA1R {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl SDMA1R {
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
            SDMA1R::DISABLE => false,
            SDMA1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDMA1R {
        match value {
            false => SDMA1R::DISABLE,
            true => SDMA1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SDMA1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SDMA1R::ENABLE
    }
}
#[doc = "Possible values of the field `LSPI_HS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPI_HSR {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl LSPI_HSR {
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
            LSPI_HSR::DISABLE => false,
            LSPI_HSR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSPI_HSR {
        match value {
            false => LSPI_HSR::DISABLE,
            true => LSPI_HSR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == LSPI_HSR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == LSPI_HSR::ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct WAKEUPPADSR {
    bits: bool,
}
impl WAKEUPPADSR {
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
#[doc = "Values that can be written to the field `GPIO_INT04`"]
pub enum GPIO_INT04W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl GPIO_INT04W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO_INT04W::DISABLE => false,
            GPIO_INT04W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_INT04W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT04W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_INT04W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT04W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT04W::ENABLE)
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
#[doc = "Values that can be written to the field `GPIO_INT05`"]
pub enum GPIO_INT05W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl GPIO_INT05W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO_INT05W::DISABLE => false,
            GPIO_INT05W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_INT05W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT05W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_INT05W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT05W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT05W::ENABLE)
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
#[doc = "Values that can be written to the field `GPIO_INT06`"]
pub enum GPIO_INT06W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl GPIO_INT06W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO_INT06W::DISABLE => false,
            GPIO_INT06W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_INT06W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT06W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_INT06W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT06W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT06W::ENABLE)
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
#[doc = "Values that can be written to the field `GPIO_INT07`"]
pub enum GPIO_INT07W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl GPIO_INT07W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO_INT07W::DISABLE => false,
            GPIO_INT07W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_INT07W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT07W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_INT07W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT07W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT07W::ENABLE)
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
#[doc = "Values that can be written to the field `CTIMER2`"]
pub enum CTIMER2W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl CTIMER2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTIMER2W::DISABLE => false,
            CTIMER2W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTIMER2W<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTIMER2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTIMER2W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTIMER2W::ENABLE)
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
#[doc = "Values that can be written to the field `CTIMER4`"]
pub enum CTIMER4W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl CTIMER4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTIMER4W::DISABLE => false,
            CTIMER4W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTIMER4W<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTIMER4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTIMER4W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTIMER4W::ENABLE)
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
#[doc = "Values that can be written to the field `OS_EVENT`"]
pub enum OS_EVENTW {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl OS_EVENTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OS_EVENTW::DISABLE => false,
            OS_EVENTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OS_EVENTW<'a> {
    w: &'a mut W,
}
impl<'a> _OS_EVENTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OS_EVENTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(OS_EVENTW::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(OS_EVENTW::ENABLE)
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
#[doc = "Values that can be written to the field `SDIO`"]
pub enum SDIOW {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl SDIOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDIOW::DISABLE => false,
            SDIOW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDIOW<'a> {
    w: &'a mut W,
}
impl<'a> _SDIOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDIOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDIOW::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDIOW::ENABLE)
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
#[doc = "Values that can be written to the field `USB1`"]
pub enum USB1W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl USB1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB1W::DISABLE => false,
            USB1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB1W<'a> {
    w: &'a mut W,
}
impl<'a> _USB1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB1W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB1W::ENABLE)
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
#[doc = "Values that can be written to the field `USB1_NEEDCLK`"]
pub enum USB1_NEEDCLKW {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl USB1_NEEDCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB1_NEEDCLKW::DISABLE => false,
            USB1_NEEDCLKW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB1_NEEDCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_NEEDCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB1_NEEDCLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB1_NEEDCLKW::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB1_NEEDCLKW::ENABLE)
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
#[doc = "Values that can be written to the field `SEC_HYPERVISOR_CALL`"]
pub enum SEC_HYPERVISOR_CALLW {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl SEC_HYPERVISOR_CALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEC_HYPERVISOR_CALLW::DISABLE => false,
            SEC_HYPERVISOR_CALLW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEC_HYPERVISOR_CALLW<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_HYPERVISOR_CALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEC_HYPERVISOR_CALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SEC_HYPERVISOR_CALLW::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SEC_HYPERVISOR_CALLW::ENABLE)
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
#[doc = "Values that can be written to the field `SEC_GPIO_INT00`"]
pub enum SEC_GPIO_INT00W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl SEC_GPIO_INT00W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEC_GPIO_INT00W::DISABLE => false,
            SEC_GPIO_INT00W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEC_GPIO_INT00W<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_GPIO_INT00W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEC_GPIO_INT00W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SEC_GPIO_INT00W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SEC_GPIO_INT00W::ENABLE)
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
#[doc = "Values that can be written to the field `SEC_GPIO_INT01`"]
pub enum SEC_GPIO_INT01W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl SEC_GPIO_INT01W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEC_GPIO_INT01W::DISABLE => false,
            SEC_GPIO_INT01W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEC_GPIO_INT01W<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_GPIO_INT01W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEC_GPIO_INT01W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SEC_GPIO_INT01W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SEC_GPIO_INT01W::ENABLE)
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
#[doc = "Values that can be written to the field `PLU`"]
pub enum PLUW {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl PLUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLUW::DISABLE => false,
            PLUW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLUW<'a> {
    w: &'a mut W,
}
impl<'a> _PLUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLUW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLUW::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLUW::ENABLE)
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
#[doc = "Values that can be written to the field `SEC_VIO`"]
pub enum SEC_VIOW {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl SEC_VIOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEC_VIOW::DISABLE => false,
            SEC_VIOW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEC_VIOW<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_VIOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEC_VIOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SEC_VIOW::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SEC_VIOW::ENABLE)
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
#[doc = "Values that can be written to the field `SHA`"]
pub enum SHAW {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl SHAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SHAW::DISABLE => false,
            SHAW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SHAW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SHAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SHAW::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SHAW::ENABLE)
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
#[doc = "Values that can be written to the field `CASER`"]
pub enum CASERW {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl CASERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CASERW::DISABLE => false,
            CASERW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CASERW<'a> {
    w: &'a mut W,
}
impl<'a> _CASERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CASERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CASERW::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CASERW::ENABLE)
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
#[doc = "Values that can be written to the field `QDDKEY`"]
pub enum QDDKEYW {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl QDDKEYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QDDKEYW::DISABLE => false,
            QDDKEYW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QDDKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _QDDKEYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QDDKEYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(QDDKEYW::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(QDDKEYW::ENABLE)
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
#[doc = "Values that can be written to the field `PQ`"]
pub enum PQW {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl PQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PQW::DISABLE => false,
            PQW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PQW<'a> {
    w: &'a mut W,
}
impl<'a> _PQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PQW::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PQW::ENABLE)
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
#[doc = "Values that can be written to the field `SDMA1`"]
pub enum SDMA1W {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl SDMA1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDMA1W::DISABLE => false,
            SDMA1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDMA1W<'a> {
    w: &'a mut W,
}
impl<'a> _SDMA1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDMA1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDMA1W::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDMA1W::ENABLE)
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
#[doc = "Values that can be written to the field `LSPI_HS`"]
pub enum LSPI_HSW {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl LSPI_HSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSPI_HSW::DISABLE => false,
            LSPI_HSW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSPI_HSW<'a> {
    w: &'a mut W,
}
impl<'a> _LSPI_HSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSPI_HSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(LSPI_HSW::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(LSPI_HSW::ENABLE)
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
#[doc = r" Proxy"]
pub struct _WAKEUPPADSW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUPPADSW<'a> {
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
    #[doc = "Bit 0 - GPIO_INT04 interrupt wake-up."]
    #[inline]
    pub fn gpio_int04(&self) -> GPIO_INT04R {
        GPIO_INT04R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - GPIO_INT05 interrupt wake-up."]
    #[inline]
    pub fn gpio_int05(&self) -> GPIO_INT05R {
        GPIO_INT05R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - GPIO_INT06 interrupt wake-up."]
    #[inline]
    pub fn gpio_int06(&self) -> GPIO_INT06R {
        GPIO_INT06R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - GPIO_INT07 interrupt wake-up."]
    #[inline]
    pub fn gpio_int07(&self) -> GPIO_INT07R {
        GPIO_INT07R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - CTIMER2 interrupt wake-up."]
    #[inline]
    pub fn ctimer2(&self) -> CTIMER2R {
        CTIMER2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - CTIMER4 interrupt wake-up."]
    #[inline]
    pub fn ctimer4(&self) -> CTIMER4R {
        CTIMER4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - OS_EVENT interrupt wake-up."]
    #[inline]
    pub fn os_event(&self) -> OS_EVENTR {
        OS_EVENTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - SDIO interrupt wake-up."]
    #[inline]
    pub fn sdio(&self) -> SDIOR {
        SDIOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - USB1 interrupt wake-up."]
    #[inline]
    pub fn usb1(&self) -> USB1R {
        USB1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - USB1_NEEDCLK interrupt wake-up."]
    #[inline]
    pub fn usb1_needclk(&self) -> USB1_NEEDCLKR {
        USB1_NEEDCLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - SEC_HYPERVISOR_CALL interrupt wake-up."]
    #[inline]
    pub fn sec_hypervisor_call(&self) -> SEC_HYPERVISOR_CALLR {
        SEC_HYPERVISOR_CALLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - SEC_GPIO_INT00 interrupt wake-up."]
    #[inline]
    pub fn sec_gpio_int00(&self) -> SEC_GPIO_INT00R {
        SEC_GPIO_INT00R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - SEC_GPIO_INT01 interrupt wake-up."]
    #[inline]
    pub fn sec_gpio_int01(&self) -> SEC_GPIO_INT01R {
        SEC_GPIO_INT01R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - PLU interrupt wake-up."]
    #[inline]
    pub fn plu(&self) -> PLUR {
        PLUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - SEC_VIO interrupt wake-up."]
    #[inline]
    pub fn sec_vio(&self) -> SEC_VIOR {
        SEC_VIOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - SHA interrupt wake-up."]
    #[inline]
    pub fn sha(&self) -> SHAR {
        SHAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - CASER interrupt wake-up."]
    #[inline]
    pub fn caser(&self) -> CASERR {
        CASERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - QDDKEY interrupt wake-up."]
    #[inline]
    pub fn qddkey(&self) -> QDDKEYR {
        QDDKEYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - PQ interrupt wake-up."]
    #[inline]
    pub fn pq(&self) -> PQR {
        PQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - SDMA1 interrupt wake-up."]
    #[inline]
    pub fn sdma1(&self) -> SDMA1R {
        SDMA1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - LSPI_HS interrupt wake-up."]
    #[inline]
    pub fn lspi_hs(&self) -> LSPI_HSR {
        LSPI_HSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - WAKEUPPADS interrupt wake-up."]
    #[inline]
    pub fn wakeuppads(&self) -> WAKEUPPADSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WAKEUPPADSR { bits }
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
    #[doc = "Bit 0 - GPIO_INT04 interrupt wake-up."]
    #[inline]
    pub fn gpio_int04(&mut self) -> _GPIO_INT04W {
        _GPIO_INT04W { w: self }
    }
    #[doc = "Bit 1 - GPIO_INT05 interrupt wake-up."]
    #[inline]
    pub fn gpio_int05(&mut self) -> _GPIO_INT05W {
        _GPIO_INT05W { w: self }
    }
    #[doc = "Bit 2 - GPIO_INT06 interrupt wake-up."]
    #[inline]
    pub fn gpio_int06(&mut self) -> _GPIO_INT06W {
        _GPIO_INT06W { w: self }
    }
    #[doc = "Bit 3 - GPIO_INT07 interrupt wake-up."]
    #[inline]
    pub fn gpio_int07(&mut self) -> _GPIO_INT07W {
        _GPIO_INT07W { w: self }
    }
    #[doc = "Bit 4 - CTIMER2 interrupt wake-up."]
    #[inline]
    pub fn ctimer2(&mut self) -> _CTIMER2W {
        _CTIMER2W { w: self }
    }
    #[doc = "Bit 5 - CTIMER4 interrupt wake-up."]
    #[inline]
    pub fn ctimer4(&mut self) -> _CTIMER4W {
        _CTIMER4W { w: self }
    }
    #[doc = "Bit 6 - OS_EVENT interrupt wake-up."]
    #[inline]
    pub fn os_event(&mut self) -> _OS_EVENTW {
        _OS_EVENTW { w: self }
    }
    #[doc = "Bit 10 - SDIO interrupt wake-up."]
    #[inline]
    pub fn sdio(&mut self) -> _SDIOW {
        _SDIOW { w: self }
    }
    #[doc = "Bit 15 - USB1 interrupt wake-up."]
    #[inline]
    pub fn usb1(&mut self) -> _USB1W {
        _USB1W { w: self }
    }
    #[doc = "Bit 16 - USB1_NEEDCLK interrupt wake-up."]
    #[inline]
    pub fn usb1_needclk(&mut self) -> _USB1_NEEDCLKW {
        _USB1_NEEDCLKW { w: self }
    }
    #[doc = "Bit 17 - SEC_HYPERVISOR_CALL interrupt wake-up."]
    #[inline]
    pub fn sec_hypervisor_call(&mut self) -> _SEC_HYPERVISOR_CALLW {
        _SEC_HYPERVISOR_CALLW { w: self }
    }
    #[doc = "Bit 18 - SEC_GPIO_INT00 interrupt wake-up."]
    #[inline]
    pub fn sec_gpio_int00(&mut self) -> _SEC_GPIO_INT00W {
        _SEC_GPIO_INT00W { w: self }
    }
    #[doc = "Bit 19 - SEC_GPIO_INT01 interrupt wake-up."]
    #[inline]
    pub fn sec_gpio_int01(&mut self) -> _SEC_GPIO_INT01W {
        _SEC_GPIO_INT01W { w: self }
    }
    #[doc = "Bit 20 - PLU interrupt wake-up."]
    #[inline]
    pub fn plu(&mut self) -> _PLUW {
        _PLUW { w: self }
    }
    #[doc = "Bit 21 - SEC_VIO interrupt wake-up."]
    #[inline]
    pub fn sec_vio(&mut self) -> _SEC_VIOW {
        _SEC_VIOW { w: self }
    }
    #[doc = "Bit 22 - SHA interrupt wake-up."]
    #[inline]
    pub fn sha(&mut self) -> _SHAW {
        _SHAW { w: self }
    }
    #[doc = "Bit 23 - CASER interrupt wake-up."]
    #[inline]
    pub fn caser(&mut self) -> _CASERW {
        _CASERW { w: self }
    }
    #[doc = "Bit 24 - QDDKEY interrupt wake-up."]
    #[inline]
    pub fn qddkey(&mut self) -> _QDDKEYW {
        _QDDKEYW { w: self }
    }
    #[doc = "Bit 25 - PQ interrupt wake-up."]
    #[inline]
    pub fn pq(&mut self) -> _PQW {
        _PQW { w: self }
    }
    #[doc = "Bit 26 - SDMA1 interrupt wake-up."]
    #[inline]
    pub fn sdma1(&mut self) -> _SDMA1W {
        _SDMA1W { w: self }
    }
    #[doc = "Bit 27 - LSPI_HS interrupt wake-up."]
    #[inline]
    pub fn lspi_hs(&mut self) -> _LSPI_HSW {
        _LSPI_HSW { w: self }
    }
    #[doc = "Bit 31 - WAKEUPPADS interrupt wake-up."]
    #[inline]
    pub fn wakeuppads(&mut self) -> _WAKEUPPADSW {
        _WAKEUPPADSW { w: self }
    }
}
