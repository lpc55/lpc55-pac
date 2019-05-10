#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHBCLKCTRL0 {
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
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
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
#[doc = "Possible values of the field `SRAM_CTRL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL1R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl SRAM_CTRL1R {
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
            SRAM_CTRL1R::DISABLE => false,
            SRAM_CTRL1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRAM_CTRL1R {
        match value {
            false => SRAM_CTRL1R::DISABLE,
            true => SRAM_CTRL1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SRAM_CTRL1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SRAM_CTRL1R::ENABLE
    }
}
#[doc = "Possible values of the field `SRAM_CTRL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL2R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl SRAM_CTRL2R {
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
            SRAM_CTRL2R::DISABLE => false,
            SRAM_CTRL2R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRAM_CTRL2R {
        match value {
            false => SRAM_CTRL2R::DISABLE,
            true => SRAM_CTRL2R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SRAM_CTRL2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SRAM_CTRL2R::ENABLE
    }
}
#[doc = "Possible values of the field `SRAM_CTRL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL3R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl SRAM_CTRL3R {
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
            SRAM_CTRL3R::DISABLE => false,
            SRAM_CTRL3R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRAM_CTRL3R {
        match value {
            false => SRAM_CTRL3R::DISABLE,
            true => SRAM_CTRL3R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SRAM_CTRL3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SRAM_CTRL3R::ENABLE
    }
}
#[doc = "Possible values of the field `SRAM_CTRL4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL4R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl SRAM_CTRL4R {
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
            SRAM_CTRL4R::DISABLE => false,
            SRAM_CTRL4R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRAM_CTRL4R {
        match value {
            false => SRAM_CTRL4R::DISABLE,
            true => SRAM_CTRL4R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SRAM_CTRL4R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SRAM_CTRL4R::ENABLE
    }
}
#[doc = "Possible values of the field `FLASH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
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
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
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
#[doc = "Possible values of the field `MUX0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX0R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl MUX0R {
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
            MUX0R::DISABLE => false,
            MUX0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MUX0R {
        match value {
            false => MUX0R::DISABLE,
            true => MUX0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == MUX0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == MUX0R::ENABLE
    }
}
#[doc = "Possible values of the field `IOCON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCONR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl IOCONR {
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
            IOCONR::DISABLE => false,
            IOCONR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOCONR {
        match value {
            false => IOCONR::DISABLE,
            true => IOCONR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IOCONR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == IOCONR::ENABLE
    }
}
#[doc = "Possible values of the field `GPIO0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl GPIO0R {
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
            GPIO0R::DISABLE => false,
            GPIO0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO0R {
        match value {
            false => GPIO0R::DISABLE,
            true => GPIO0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == GPIO0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == GPIO0R::ENABLE
    }
}
#[doc = "Possible values of the field `GPIO1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO1R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl GPIO1R {
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
            GPIO1R::DISABLE => false,
            GPIO1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO1R {
        match value {
            false => GPIO1R::DISABLE,
            true => GPIO1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == GPIO1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == GPIO1R::ENABLE
    }
}
#[doc = "Possible values of the field `GPIO2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO2R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl GPIO2R {
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
            GPIO2R::DISABLE => false,
            GPIO2R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO2R {
        match value {
            false => GPIO2R::DISABLE,
            true => GPIO2R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == GPIO2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == GPIO2R::ENABLE
    }
}
#[doc = "Possible values of the field `GPIO3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO3R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl GPIO3R {
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
            GPIO3R::DISABLE => false,
            GPIO3R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO3R {
        match value {
            false => GPIO3R::DISABLE,
            true => GPIO3R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == GPIO3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == GPIO3R::ENABLE
    }
}
#[doc = "Possible values of the field `PINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINTR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl PINTR {
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
            PINTR::DISABLE => false,
            PINTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PINTR {
        match value {
            false => PINTR::DISABLE,
            true => PINTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PINTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PINTR::ENABLE
    }
}
#[doc = "Possible values of the field `GINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GINTR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl GINTR {
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
            GINTR::DISABLE => false,
            GINTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GINTR {
        match value {
            false => GINTR::DISABLE,
            true => GINTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == GINTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == GINTR::ENABLE
    }
}
#[doc = "Possible values of the field `DMA0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA0R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl DMA0R {
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
            DMA0R::DISABLE => false,
            DMA0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA0R {
        match value {
            false => DMA0R::DISABLE,
            true => DMA0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DMA0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == DMA0R::ENABLE
    }
}
#[doc = "Possible values of the field `CRCGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCGENR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
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
#[doc = "Possible values of the field `WWDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDTR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl WWDTR {
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
            WWDTR::DISABLE => false,
            WWDTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WWDTR {
        match value {
            false => WWDTR::DISABLE,
            true => WWDTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WWDTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WWDTR::ENABLE
    }
}
#[doc = "Possible values of the field `RTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl RTCR {
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
            RTCR::DISABLE => false,
            RTCR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTCR {
        match value {
            false => RTCR::DISABLE,
            true => RTCR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RTCR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RTCR::ENABLE
    }
}
#[doc = "Possible values of the field `MAILBOX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAILBOXR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl MAILBOXR {
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
            MAILBOXR::DISABLE => false,
            MAILBOXR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAILBOXR {
        match value {
            false => MAILBOXR::DISABLE,
            true => MAILBOXR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == MAILBOXR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == MAILBOXR::ENABLE
    }
}
#[doc = "Possible values of the field `ADC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl ADCR {
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
            ADCR::DISABLE => false,
            ADCR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCR {
        match value {
            false => ADCR::DISABLE,
            true => ADCR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ADCR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ADCR::ENABLE
    }
}
#[doc = "Values that can be written to the field `ROM`"]
pub enum ROMW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
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
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ROMW::DISABLE)
    }
    #[doc = "Enable Clock."]
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRAM_CTRL1`"]
pub enum SRAM_CTRL1W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl SRAM_CTRL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRAM_CTRL1W::DISABLE => false,
            SRAM_CTRL1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRAM_CTRL1W<'a> {
    w: &'a mut W,
}
impl<'a> _SRAM_CTRL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAM_CTRL1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_CTRL1W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_CTRL1W::ENABLE)
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
#[doc = "Values that can be written to the field `SRAM_CTRL2`"]
pub enum SRAM_CTRL2W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl SRAM_CTRL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRAM_CTRL2W::DISABLE => false,
            SRAM_CTRL2W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRAM_CTRL2W<'a> {
    w: &'a mut W,
}
impl<'a> _SRAM_CTRL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAM_CTRL2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_CTRL2W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_CTRL2W::ENABLE)
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
#[doc = "Values that can be written to the field `SRAM_CTRL3`"]
pub enum SRAM_CTRL3W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl SRAM_CTRL3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRAM_CTRL3W::DISABLE => false,
            SRAM_CTRL3W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRAM_CTRL3W<'a> {
    w: &'a mut W,
}
impl<'a> _SRAM_CTRL3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAM_CTRL3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_CTRL3W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_CTRL3W::ENABLE)
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
#[doc = "Values that can be written to the field `SRAM_CTRL4`"]
pub enum SRAM_CTRL4W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl SRAM_CTRL4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRAM_CTRL4W::DISABLE => false,
            SRAM_CTRL4W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRAM_CTRL4W<'a> {
    w: &'a mut W,
}
impl<'a> _SRAM_CTRL4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAM_CTRL4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_CTRL4W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_CTRL4W::ENABLE)
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
#[doc = "Values that can be written to the field `FLASH`"]
pub enum FLASHW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
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
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLASHW::DISABLE)
    }
    #[doc = "Enable Clock."]
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FMC`"]
pub enum FMCW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
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
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FMCW::DISABLE)
    }
    #[doc = "Enable Clock."]
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MUX0`"]
pub enum MUX0W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl MUX0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MUX0W::DISABLE => false,
            MUX0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUX0W<'a> {
    w: &'a mut W,
}
impl<'a> _MUX0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUX0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(MUX0W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(MUX0W::ENABLE)
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
#[doc = "Values that can be written to the field `IOCON`"]
pub enum IOCONW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl IOCONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOCONW::DISABLE => false,
            IOCONW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOCONW<'a> {
    w: &'a mut W,
}
impl<'a> _IOCONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOCONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IOCONW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(IOCONW::ENABLE)
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
#[doc = "Values that can be written to the field `GPIO0`"]
pub enum GPIO0W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl GPIO0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO0W::DISABLE => false,
            GPIO0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO0W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO0W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO0W::ENABLE)
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
#[doc = "Values that can be written to the field `GPIO1`"]
pub enum GPIO1W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl GPIO1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO1W::DISABLE => false,
            GPIO1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO1W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO1W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO1W::ENABLE)
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
#[doc = "Values that can be written to the field `GPIO2`"]
pub enum GPIO2W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl GPIO2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO2W::DISABLE => false,
            GPIO2W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO2W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO2W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO2W::ENABLE)
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
#[doc = "Values that can be written to the field `GPIO3`"]
pub enum GPIO3W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl GPIO3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO3W::DISABLE => false,
            GPIO3W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO3W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO3W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO3W::ENABLE)
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
#[doc = "Values that can be written to the field `PINT`"]
pub enum PINTW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl PINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PINTW::DISABLE => false,
            PINTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PINTW<'a> {
    w: &'a mut W,
}
impl<'a> _PINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PINTW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PINTW::ENABLE)
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
#[doc = "Values that can be written to the field `GINT`"]
pub enum GINTW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl GINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GINTW::DISABLE => false,
            GINTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GINTW<'a> {
    w: &'a mut W,
}
impl<'a> _GINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(GINTW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(GINTW::ENABLE)
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
#[doc = "Values that can be written to the field `DMA0`"]
pub enum DMA0W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl DMA0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA0W::DISABLE => false,
            DMA0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA0W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA0W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA0W::ENABLE)
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
#[doc = "Values that can be written to the field `CRCGEN`"]
pub enum CRCGENW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
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
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CRCGENW::DISABLE)
    }
    #[doc = "Enable Clock."]
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WWDT`"]
pub enum WWDTW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl WWDTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WWDTW::DISABLE => false,
            WWDTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WWDTW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WWDTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WWDTW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WWDTW::ENABLE)
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
#[doc = "Values that can be written to the field `RTC`"]
pub enum RTCW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl RTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCW::DISABLE => false,
            RTCW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTCW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTCW::ENABLE)
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
#[doc = "Values that can be written to the field `MAILBOX`"]
pub enum MAILBOXW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl MAILBOXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAILBOXW::DISABLE => false,
            MAILBOXW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAILBOXW<'a> {
    w: &'a mut W,
}
impl<'a> _MAILBOXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAILBOXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(MAILBOXW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(MAILBOXW::ENABLE)
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
#[doc = "Values that can be written to the field `ADC`"]
pub enum ADCW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl ADCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADCW::DISABLE => false,
            ADCW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADCW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADCW::ENABLE)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Enables the clock for the ROM."]
    #[inline]
    pub fn rom(&self) -> ROMR {
        ROMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enables the clock for the SRAM Controller 1."]
    #[inline]
    pub fn sram_ctrl1(&self) -> SRAM_CTRL1R {
        SRAM_CTRL1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enables the clock for the SRAM Controller 2."]
    #[inline]
    pub fn sram_ctrl2(&self) -> SRAM_CTRL2R {
        SRAM_CTRL2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enables the clock for the SRAM Controller 3."]
    #[inline]
    pub fn sram_ctrl3(&self) -> SRAM_CTRL3R {
        SRAM_CTRL3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enables the clock for the SRAM Controller 4."]
    #[inline]
    pub fn sram_ctrl4(&self) -> SRAM_CTRL4R {
        SRAM_CTRL4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enables the clock for the Flash controller."]
    #[inline]
    pub fn flash(&self) -> FLASHR {
        FLASHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enables the clock for the FMC controller."]
    #[inline]
    pub fn fmc(&self) -> FMCR {
        FMCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Enables the clock for the Input Mux 0."]
    #[inline]
    pub fn mux0(&self) -> MUX0R {
        MUX0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Enables the clock for the I/O controller."]
    #[inline]
    pub fn iocon(&self) -> IOCONR {
        IOCONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Enables the clock for the GPIO0."]
    #[inline]
    pub fn gpio0(&self) -> GPIO0R {
        GPIO0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Enables the clock for the GPIO1."]
    #[inline]
    pub fn gpio1(&self) -> GPIO1R {
        GPIO1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Enables the clock for the GPIO2."]
    #[inline]
    pub fn gpio2(&self) -> GPIO2R {
        GPIO2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Enables the clock for the GPIO3."]
    #[inline]
    pub fn gpio3(&self) -> GPIO3R {
        GPIO3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Enables the clock for the Pin interrupt (PINT)."]
    #[inline]
    pub fn pint(&self) -> PINTR {
        PINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Enables the clock for the Group interrupt (GINT)."]
    #[inline]
    pub fn gint(&self) -> GINTR {
        GINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Enables the clock for the DMA0."]
    #[inline]
    pub fn dma0(&self) -> DMA0R {
        DMA0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Enables the clock for the CRCGEN."]
    #[inline]
    pub fn crcgen(&self) -> CRCGENR {
        CRCGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Enables the clock for the Watchdog Timer."]
    #[inline]
    pub fn wwdt(&self) -> WWDTR {
        WWDTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Enables the clock for the Real Time Clock (RTC)."]
    #[inline]
    pub fn rtc(&self) -> RTCR {
        RTCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Enables the clock for the Inter CPU communication Mailbox."]
    #[inline]
    pub fn mailbox(&self) -> MAILBOXR {
        MAILBOXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Enables the clock for the ADC."]
    #[inline]
    pub fn adc(&self) -> ADCR {
        ADCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 384 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Enables the clock for the ROM."]
    #[inline]
    pub fn rom(&mut self) -> _ROMW {
        _ROMW { w: self }
    }
    #[doc = "Bit 3 - Enables the clock for the SRAM Controller 1."]
    #[inline]
    pub fn sram_ctrl1(&mut self) -> _SRAM_CTRL1W {
        _SRAM_CTRL1W { w: self }
    }
    #[doc = "Bit 4 - Enables the clock for the SRAM Controller 2."]
    #[inline]
    pub fn sram_ctrl2(&mut self) -> _SRAM_CTRL2W {
        _SRAM_CTRL2W { w: self }
    }
    #[doc = "Bit 5 - Enables the clock for the SRAM Controller 3."]
    #[inline]
    pub fn sram_ctrl3(&mut self) -> _SRAM_CTRL3W {
        _SRAM_CTRL3W { w: self }
    }
    #[doc = "Bit 6 - Enables the clock for the SRAM Controller 4."]
    #[inline]
    pub fn sram_ctrl4(&mut self) -> _SRAM_CTRL4W {
        _SRAM_CTRL4W { w: self }
    }
    #[doc = "Bit 7 - Enables the clock for the Flash controller."]
    #[inline]
    pub fn flash(&mut self) -> _FLASHW {
        _FLASHW { w: self }
    }
    #[doc = "Bit 8 - Enables the clock for the FMC controller."]
    #[inline]
    pub fn fmc(&mut self) -> _FMCW {
        _FMCW { w: self }
    }
    #[doc = "Bit 11 - Enables the clock for the Input Mux 0."]
    #[inline]
    pub fn mux0(&mut self) -> _MUX0W {
        _MUX0W { w: self }
    }
    #[doc = "Bit 13 - Enables the clock for the I/O controller."]
    #[inline]
    pub fn iocon(&mut self) -> _IOCONW {
        _IOCONW { w: self }
    }
    #[doc = "Bit 14 - Enables the clock for the GPIO0."]
    #[inline]
    pub fn gpio0(&mut self) -> _GPIO0W {
        _GPIO0W { w: self }
    }
    #[doc = "Bit 15 - Enables the clock for the GPIO1."]
    #[inline]
    pub fn gpio1(&mut self) -> _GPIO1W {
        _GPIO1W { w: self }
    }
    #[doc = "Bit 16 - Enables the clock for the GPIO2."]
    #[inline]
    pub fn gpio2(&mut self) -> _GPIO2W {
        _GPIO2W { w: self }
    }
    #[doc = "Bit 17 - Enables the clock for the GPIO3."]
    #[inline]
    pub fn gpio3(&mut self) -> _GPIO3W {
        _GPIO3W { w: self }
    }
    #[doc = "Bit 18 - Enables the clock for the Pin interrupt (PINT)."]
    #[inline]
    pub fn pint(&mut self) -> _PINTW {
        _PINTW { w: self }
    }
    #[doc = "Bit 19 - Enables the clock for the Group interrupt (GINT)."]
    #[inline]
    pub fn gint(&mut self) -> _GINTW {
        _GINTW { w: self }
    }
    #[doc = "Bit 20 - Enables the clock for the DMA0."]
    #[inline]
    pub fn dma0(&mut self) -> _DMA0W {
        _DMA0W { w: self }
    }
    #[doc = "Bit 21 - Enables the clock for the CRCGEN."]
    #[inline]
    pub fn crcgen(&mut self) -> _CRCGENW {
        _CRCGENW { w: self }
    }
    #[doc = "Bit 22 - Enables the clock for the Watchdog Timer."]
    #[inline]
    pub fn wwdt(&mut self) -> _WWDTW {
        _WWDTW { w: self }
    }
    #[doc = "Bit 23 - Enables the clock for the Real Time Clock (RTC)."]
    #[inline]
    pub fn rtc(&mut self) -> _RTCW {
        _RTCW { w: self }
    }
    #[doc = "Bit 26 - Enables the clock for the Inter CPU communication Mailbox."]
    #[inline]
    pub fn mailbox(&mut self) -> _MAILBOXW {
        _MAILBOXW { w: self }
    }
    #[doc = "Bit 27 - Enables the clock for the ADC."]
    #[inline]
    pub fn adc(&mut self) -> _ADCW {
        _ADCW { w: self }
    }
}
