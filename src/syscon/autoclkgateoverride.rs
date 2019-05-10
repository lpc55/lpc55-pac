#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUTOCLKGATEOVERRIDE {
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
#[doc = "Possible values of the field `ROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROMR {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl ROMR {
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
            ROMR::DISABLE => false,
            ROMR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ROMR {
        match value {
            false => ROMR::DISABLE,
            true => ROMR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ROMR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ROMR::ENABLE
    }
}
#[doc = "Possible values of the field `RAMX_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMX_CTRLR {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl RAMX_CTRLR {
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
            RAMX_CTRLR::DISABLE => false,
            RAMX_CTRLR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAMX_CTRLR {
        match value {
            false => RAMX_CTRLR::DISABLE,
            true => RAMX_CTRLR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RAMX_CTRLR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RAMX_CTRLR::ENABLE
    }
}
#[doc = "Possible values of the field `RAM0_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM0_CTRLR {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl RAM0_CTRLR {
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
            RAM0_CTRLR::DISABLE => false,
            RAM0_CTRLR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM0_CTRLR {
        match value {
            false => RAM0_CTRLR::DISABLE,
            true => RAM0_CTRLR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RAM0_CTRLR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RAM0_CTRLR::ENABLE
    }
}
#[doc = "Possible values of the field `RAM1_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM1_CTRLR {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl RAM1_CTRLR {
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
            RAM1_CTRLR::DISABLE => false,
            RAM1_CTRLR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM1_CTRLR {
        match value {
            false => RAM1_CTRLR::DISABLE,
            true => RAM1_CTRLR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RAM1_CTRLR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RAM1_CTRLR::ENABLE
    }
}
#[doc = "Possible values of the field `RAM2_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM2_CTRLR {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl RAM2_CTRLR {
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
            RAM2_CTRLR::DISABLE => false,
            RAM2_CTRLR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM2_CTRLR {
        match value {
            false => RAM2_CTRLR::DISABLE,
            true => RAM2_CTRLR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RAM2_CTRLR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RAM2_CTRLR::ENABLE
    }
}
#[doc = "Possible values of the field `RAM3_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM3_CTRLR {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl RAM3_CTRLR {
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
            RAM3_CTRLR::DISABLE => false,
            RAM3_CTRLR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM3_CTRLR {
        match value {
            false => RAM3_CTRLR::DISABLE,
            true => RAM3_CTRLR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RAM3_CTRLR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RAM3_CTRLR::ENABLE
    }
}
#[doc = "Possible values of the field `RAM4_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM4_CTRLR {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl RAM4_CTRLR {
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
            RAM4_CTRLR::DISABLE => false,
            RAM4_CTRLR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM4_CTRLR {
        match value {
            false => RAM4_CTRLR::DISABLE,
            true => RAM4_CTRLR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RAM4_CTRLR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RAM4_CTRLR::ENABLE
    }
}
#[doc = "Possible values of the field `SYNC0_APB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC0_APBR {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl SYNC0_APBR {
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
            SYNC0_APBR::DISABLE => false,
            SYNC0_APBR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNC0_APBR {
        match value {
            false => SYNC0_APBR::DISABLE,
            true => SYNC0_APBR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SYNC0_APBR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SYNC0_APBR::ENABLE
    }
}
#[doc = "Possible values of the field `SYNC1_APB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC1_APBR {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl SYNC1_APBR {
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
            SYNC1_APBR::DISABLE => false,
            SYNC1_APBR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNC1_APBR {
        match value {
            false => SYNC1_APBR::DISABLE,
            true => SYNC1_APBR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SYNC1_APBR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SYNC1_APBR::ENABLE
    }
}
#[doc = "Possible values of the field `FLASH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHR {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl FLASHR {
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
            FLASHR::DISABLE => false,
            FLASHR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASHR {
        match value {
            false => FLASHR::DISABLE,
            true => FLASHR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FLASHR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FLASHR::ENABLE
    }
}
#[doc = "Possible values of the field `FMC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMCR {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl FMCR {
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
            FMCR::DISABLE => false,
            FMCR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FMCR {
        match value {
            false => FMCR::DISABLE,
            true => FMCR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FMCR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FMCR::ENABLE
    }
}
#[doc = "Possible values of the field `CRCGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCGENR {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl CRCGENR {
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
            CRCGENR::DISABLE => false,
            CRCGENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRCGENR {
        match value {
            false => CRCGENR::DISABLE,
            true => CRCGENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CRCGENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CRCGENR::ENABLE
    }
}
#[doc = "Possible values of the field `SDMA0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMA0R {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
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
#[doc = "Possible values of the field `SDMA1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMA1R {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
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
#[doc = "Possible values of the field `USB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBR {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl USBR {
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
            USBR::DISABLE => false,
            USBR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBR {
        match value {
            false => USBR::DISABLE,
            true => USBR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == USBR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == USBR::ENABLE
    }
}
#[doc = "Possible values of the field `SYSCON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCONR {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl SYSCONR {
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
            SYSCONR::DISABLE => false,
            SYSCONR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYSCONR {
        match value {
            false => SYSCONR::DISABLE,
            true => SYSCONR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SYSCONR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SYSCONR::ENABLE
    }
}
#[doc = "Values that can be written to the field `ROM`"]
pub enum ROMW {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl ROMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ROMW::DISABLE => false,
            ROMW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROMW<'a> {
    w: &'a mut W,
}
impl<'a> _ROMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ROMW::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ROMW::ENABLE)
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
#[doc = "Values that can be written to the field `RAMX_CTRL`"]
pub enum RAMX_CTRLW {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl RAMX_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAMX_CTRLW::DISABLE => false,
            RAMX_CTRLW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAMX_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _RAMX_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAMX_CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAMX_CTRLW::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAMX_CTRLW::ENABLE)
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
#[doc = "Values that can be written to the field `RAM0_CTRL`"]
pub enum RAM0_CTRLW {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl RAM0_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM0_CTRLW::DISABLE => false,
            RAM0_CTRLW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM0_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _RAM0_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM0_CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAM0_CTRLW::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAM0_CTRLW::ENABLE)
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
#[doc = "Values that can be written to the field `RAM1_CTRL`"]
pub enum RAM1_CTRLW {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl RAM1_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM1_CTRLW::DISABLE => false,
            RAM1_CTRLW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM1_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _RAM1_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM1_CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAM1_CTRLW::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAM1_CTRLW::ENABLE)
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
#[doc = "Values that can be written to the field `RAM2_CTRL`"]
pub enum RAM2_CTRLW {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl RAM2_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM2_CTRLW::DISABLE => false,
            RAM2_CTRLW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM2_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _RAM2_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM2_CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAM2_CTRLW::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAM2_CTRLW::ENABLE)
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
#[doc = "Values that can be written to the field `RAM3_CTRL`"]
pub enum RAM3_CTRLW {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl RAM3_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM3_CTRLW::DISABLE => false,
            RAM3_CTRLW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM3_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _RAM3_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM3_CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAM3_CTRLW::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAM3_CTRLW::ENABLE)
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
#[doc = "Values that can be written to the field `RAM4_CTRL`"]
pub enum RAM4_CTRLW {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl RAM4_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM4_CTRLW::DISABLE => false,
            RAM4_CTRLW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM4_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _RAM4_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM4_CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAM4_CTRLW::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAM4_CTRLW::ENABLE)
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
#[doc = "Values that can be written to the field `SYNC0_APB`"]
pub enum SYNC0_APBW {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl SYNC0_APBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNC0_APBW::DISABLE => false,
            SYNC0_APBW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNC0_APBW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNC0_APBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNC0_APBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYNC0_APBW::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYNC0_APBW::ENABLE)
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
#[doc = "Values that can be written to the field `SYNC1_APB`"]
pub enum SYNC1_APBW {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl SYNC1_APBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNC1_APBW::DISABLE => false,
            SYNC1_APBW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNC1_APBW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNC1_APBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNC1_APBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYNC1_APBW::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYNC1_APBW::ENABLE)
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
#[doc = "Values that can be written to the field `FLASH`"]
pub enum FLASHW {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl FLASHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASHW::DISABLE => false,
            FLASHW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASHW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLASHW::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLASHW::ENABLE)
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
#[doc = "Values that can be written to the field `FMC`"]
pub enum FMCW {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl FMCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FMCW::DISABLE => false,
            FMCW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FMCW<'a> {
    w: &'a mut W,
}
impl<'a> _FMCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FMCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FMCW::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FMCW::ENABLE)
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
#[doc = "Values that can be written to the field `CRCGEN`"]
pub enum CRCGENW {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl CRCGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRCGENW::DISABLE => false,
            CRCGENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRCGENW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CRCGENW::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CRCGENW::ENABLE)
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
#[doc = "Values that can be written to the field `SDMA0`"]
pub enum SDMA0W {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
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
    #[doc = "Automatic clock gating is not overridden."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDMA0W::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SDMA1`"]
pub enum SDMA1W {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
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
    #[doc = "Automatic clock gating is not overridden."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDMA1W::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USB`"]
pub enum USBW {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl USBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBW::DISABLE => false,
            USBW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBW<'a> {
    w: &'a mut W,
}
impl<'a> _USBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(USBW::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(USBW::ENABLE)
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
#[doc = "Values that can be written to the field `SYSCON`"]
pub enum SYSCONW {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl SYSCONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSCONW::DISABLE => false,
            SYSCONW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSCONW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSCONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYSCONW::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYSCONW::ENABLE)
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
#[doc = "Values that can be written to the field `ENABLEUPDATE`"]
pub enum ENABLEUPDATEW {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE,
}
impl ENABLEUPDATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            ENABLEUPDATEW::DISABLE => 0,
            ENABLEUPDATEW::ENABLE => 49374,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEUPDATEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEUPDATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLEUPDATEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLEUPDATEW::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLEUPDATEW::ENABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Control automatic clock gating of ROM controller."]
    #[inline]
    pub fn rom(&self) -> ROMR {
        ROMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Control automatic clock gating of RAMX controller."]
    #[inline]
    pub fn ramx_ctrl(&self) -> RAMX_CTRLR {
        RAMX_CTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Control automatic clock gating of RAM0 controller."]
    #[inline]
    pub fn ram0_ctrl(&self) -> RAM0_CTRLR {
        RAM0_CTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Control automatic clock gating of RAM1 controller."]
    #[inline]
    pub fn ram1_ctrl(&self) -> RAM1_CTRLR {
        RAM1_CTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Control automatic clock gating of RAM2 controller."]
    #[inline]
    pub fn ram2_ctrl(&self) -> RAM2_CTRLR {
        RAM2_CTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Control automatic clock gating of RAM3 controller."]
    #[inline]
    pub fn ram3_ctrl(&self) -> RAM3_CTRLR {
        RAM3_CTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Control automatic clock gating of RAM4 controller."]
    #[inline]
    pub fn ram4_ctrl(&self) -> RAM4_CTRLR {
        RAM4_CTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Control automatic clock gating of synchronous bridge controller 0."]
    #[inline]
    pub fn sync0_apb(&self) -> SYNC0_APBR {
        SYNC0_APBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Control automatic clock gating of synchronous bridge controller 1."]
    #[inline]
    pub fn sync1_apb(&self) -> SYNC1_APBR {
        SYNC1_APBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Control automatic clock gating of FLASH controller."]
    #[inline]
    pub fn flash(&self) -> FLASHR {
        FLASHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Control automatic clock gating of FMC controller."]
    #[inline]
    pub fn fmc(&self) -> FMCR {
        FMCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Control automatic clock gating of CRCGEN controller."]
    #[inline]
    pub fn crcgen(&self) -> CRCGENR {
        CRCGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Control automatic clock gating of DMA0 controller."]
    #[inline]
    pub fn sdma0(&self) -> SDMA0R {
        SDMA0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Control automatic clock gating of DMA1 controller."]
    #[inline]
    pub fn sdma1(&self) -> SDMA1R {
        SDMA1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Control automatic clock gating of USB controller."]
    #[inline]
    pub fn usb(&self) -> USBR {
        USBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Control automatic clock gating of synchronous system controller registers bank."]
    #[inline]
    pub fn syscon(&self) -> SYSCONR {
        SYSCONR::_from({
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
        W { bits: 65535 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Control automatic clock gating of ROM controller."]
    #[inline]
    pub fn rom(&mut self) -> _ROMW {
        _ROMW { w: self }
    }
    #[doc = "Bit 1 - Control automatic clock gating of RAMX controller."]
    #[inline]
    pub fn ramx_ctrl(&mut self) -> _RAMX_CTRLW {
        _RAMX_CTRLW { w: self }
    }
    #[doc = "Bit 2 - Control automatic clock gating of RAM0 controller."]
    #[inline]
    pub fn ram0_ctrl(&mut self) -> _RAM0_CTRLW {
        _RAM0_CTRLW { w: self }
    }
    #[doc = "Bit 3 - Control automatic clock gating of RAM1 controller."]
    #[inline]
    pub fn ram1_ctrl(&mut self) -> _RAM1_CTRLW {
        _RAM1_CTRLW { w: self }
    }
    #[doc = "Bit 4 - Control automatic clock gating of RAM2 controller."]
    #[inline]
    pub fn ram2_ctrl(&mut self) -> _RAM2_CTRLW {
        _RAM2_CTRLW { w: self }
    }
    #[doc = "Bit 5 - Control automatic clock gating of RAM3 controller."]
    #[inline]
    pub fn ram3_ctrl(&mut self) -> _RAM3_CTRLW {
        _RAM3_CTRLW { w: self }
    }
    #[doc = "Bit 6 - Control automatic clock gating of RAM4 controller."]
    #[inline]
    pub fn ram4_ctrl(&mut self) -> _RAM4_CTRLW {
        _RAM4_CTRLW { w: self }
    }
    #[doc = "Bit 7 - Control automatic clock gating of synchronous bridge controller 0."]
    #[inline]
    pub fn sync0_apb(&mut self) -> _SYNC0_APBW {
        _SYNC0_APBW { w: self }
    }
    #[doc = "Bit 8 - Control automatic clock gating of synchronous bridge controller 1."]
    #[inline]
    pub fn sync1_apb(&mut self) -> _SYNC1_APBW {
        _SYNC1_APBW { w: self }
    }
    #[doc = "Bit 9 - Control automatic clock gating of FLASH controller."]
    #[inline]
    pub fn flash(&mut self) -> _FLASHW {
        _FLASHW { w: self }
    }
    #[doc = "Bit 10 - Control automatic clock gating of FMC controller."]
    #[inline]
    pub fn fmc(&mut self) -> _FMCW {
        _FMCW { w: self }
    }
    #[doc = "Bit 11 - Control automatic clock gating of CRCGEN controller."]
    #[inline]
    pub fn crcgen(&mut self) -> _CRCGENW {
        _CRCGENW { w: self }
    }
    #[doc = "Bit 12 - Control automatic clock gating of DMA0 controller."]
    #[inline]
    pub fn sdma0(&mut self) -> _SDMA0W {
        _SDMA0W { w: self }
    }
    #[doc = "Bit 13 - Control automatic clock gating of DMA1 controller."]
    #[inline]
    pub fn sdma1(&mut self) -> _SDMA1W {
        _SDMA1W { w: self }
    }
    #[doc = "Bit 14 - Control automatic clock gating of USB controller."]
    #[inline]
    pub fn usb(&mut self) -> _USBW {
        _USBW { w: self }
    }
    #[doc = "Bit 15 - Control automatic clock gating of synchronous system controller registers bank."]
    #[inline]
    pub fn syscon(&mut self) -> _SYSCONW {
        _SYSCONW { w: self }
    }
    #[doc = "Bits 16:31 - The value 0xC0DE must be written for AUTOCLKGATEOVERRIDE registers fields updates to have effect."]
    #[inline]
    pub fn enableupdate(&mut self) -> _ENABLEUPDATEW {
        _ENABLEUPDATEW { w: self }
    }
}
