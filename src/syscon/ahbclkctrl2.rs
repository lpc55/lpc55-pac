#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHBCLKCTRL2 {
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
#[doc = "Possible values of the field `DMA1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl DMA1R {
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
            DMA1R::DISABLE => false,
            DMA1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA1R {
        match value {
            false => DMA1R::DISABLE,
            true => DMA1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DMA1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == DMA1R::ENABLE
    }
}
#[doc = "Possible values of the field `COMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl COMPR {
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
            COMPR::DISABLE => false,
            COMPR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPR {
        match value {
            false => COMPR::DISABLE,
            true => COMPR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == COMPR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == COMPR::ENABLE
    }
}
#[doc = "Possible values of the field `SDIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIOR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
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
#[doc = "Possible values of the field `USB1_HOST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_HOSTR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl USB1_HOSTR {
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
            USB1_HOSTR::DISABLE => false,
            USB1_HOSTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB1_HOSTR {
        match value {
            false => USB1_HOSTR::DISABLE,
            true => USB1_HOSTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == USB1_HOSTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == USB1_HOSTR::ENABLE
    }
}
#[doc = "Possible values of the field `USB1_DEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_DEVR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl USB1_DEVR {
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
            USB1_DEVR::DISABLE => false,
            USB1_DEVR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB1_DEVR {
        match value {
            false => USB1_DEVR::DISABLE,
            true => USB1_DEVR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == USB1_DEVR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == USB1_DEVR::ENABLE
    }
}
#[doc = "Possible values of the field `USB1_RAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_RAMR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl USB1_RAMR {
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
            USB1_RAMR::DISABLE => false,
            USB1_RAMR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB1_RAMR {
        match value {
            false => USB1_RAMR::DISABLE,
            true => USB1_RAMR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == USB1_RAMR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == USB1_RAMR::ENABLE
    }
}
#[doc = "Possible values of the field `USB1_PHY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_PHYR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl USB1_PHYR {
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
            USB1_PHYR::DISABLE => false,
            USB1_PHYR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB1_PHYR {
        match value {
            false => USB1_PHYR::DISABLE,
            true => USB1_PHYR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == USB1_PHYR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == USB1_PHYR::ENABLE
    }
}
#[doc = "Possible values of the field `FREQME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQMER {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl FREQMER {
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
            FREQMER::DISABLE => false,
            FREQMER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FREQMER {
        match value {
            false => FREQMER::DISABLE,
            true => FREQMER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FREQMER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FREQMER::ENABLE
    }
}
#[doc = "Possible values of the field `GPIO4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO4R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl GPIO4R {
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
            GPIO4R::DISABLE => false,
            GPIO4R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO4R {
        match value {
            false => GPIO4R::DISABLE,
            true => GPIO4R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == GPIO4R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == GPIO4R::ENABLE
    }
}
#[doc = "Possible values of the field `GPIO5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO5R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl GPIO5R {
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
            GPIO5R::DISABLE => false,
            GPIO5R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO5R {
        match value {
            false => GPIO5R::DISABLE,
            true => GPIO5R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == GPIO5R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == GPIO5R::ENABLE
    }
}
#[doc = "Possible values of the field `OTP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTPR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl OTPR {
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
            OTPR::DISABLE => false,
            OTPR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OTPR {
        match value {
            false => OTPR::DISABLE,
            true => OTPR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == OTPR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == OTPR::ENABLE
    }
}
#[doc = "Possible values of the field `RNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNGR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl RNGR {
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
            RNGR::DISABLE => false,
            RNGR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RNGR {
        match value {
            false => RNGR::DISABLE,
            true => RNGR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RNGR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RNGR::ENABLE
    }
}
#[doc = "Possible values of the field `MUX1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX1R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl MUX1R {
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
            MUX1R::DISABLE => false,
            MUX1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MUX1R {
        match value {
            false => MUX1R::DISABLE,
            true => MUX1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == MUX1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == MUX1R::ENABLE
    }
}
#[doc = "Possible values of the field `USB0_HOSTM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_HOSTMR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl USB0_HOSTMR {
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
            USB0_HOSTMR::DISABLE => false,
            USB0_HOSTMR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB0_HOSTMR {
        match value {
            false => USB0_HOSTMR::DISABLE,
            true => USB0_HOSTMR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == USB0_HOSTMR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == USB0_HOSTMR::ENABLE
    }
}
#[doc = "Possible values of the field `USB0_HOSTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_HOSTSR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl USB0_HOSTSR {
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
            USB0_HOSTSR::DISABLE => false,
            USB0_HOSTSR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB0_HOSTSR {
        match value {
            false => USB0_HOSTSR::DISABLE,
            true => USB0_HOSTSR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == USB0_HOSTSR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == USB0_HOSTSR::ENABLE
    }
}
#[doc = "Possible values of the field `HASH0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASH0R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl HASH0R {
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
            HASH0R::DISABLE => false,
            HASH0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HASH0R {
        match value {
            false => HASH0R::DISABLE,
            true => HASH0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == HASH0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == HASH0R::ENABLE
    }
}
#[doc = "Possible values of the field `PQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PQR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
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
#[doc = "Possible values of the field `PLULUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLULUTR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl PLULUTR {
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
            PLULUTR::DISABLE => false,
            PLULUTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLULUTR {
        match value {
            false => PLULUTR::DISABLE,
            true => PLULUTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PLULUTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PLULUTR::ENABLE
    }
}
#[doc = "Possible values of the field `TIMER3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER3R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl TIMER3R {
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
            TIMER3R::DISABLE => false,
            TIMER3R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMER3R {
        match value {
            false => TIMER3R::DISABLE,
            true => TIMER3R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TIMER3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TIMER3R::ENABLE
    }
}
#[doc = "Possible values of the field `TIMER4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER4R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl TIMER4R {
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
            TIMER4R::DISABLE => false,
            TIMER4R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMER4R {
        match value {
            false => TIMER4R::DISABLE,
            true => TIMER4R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TIMER4R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TIMER4R::ENABLE
    }
}
#[doc = "Possible values of the field `PUF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUFR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl PUFR {
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
            PUFR::DISABLE => false,
            PUFR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PUFR {
        match value {
            false => PUFR::DISABLE,
            true => PUFR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PUFR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PUFR::ENABLE
    }
}
#[doc = "Possible values of the field `CASPER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CASPERR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl CASPERR {
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
            CASPERR::DISABLE => false,
            CASPERR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CASPERR {
        match value {
            false => CASPERR::DISABLE,
            true => CASPERR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CASPERR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CASPERR::ENABLE
    }
}
#[doc = "Possible values of the field `CAPT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT0R {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl CAPT0R {
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
            CAPT0R::DISABLE => false,
            CAPT0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPT0R {
        match value {
            false => CAPT0R::DISABLE,
            true => CAPT0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CAPT0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CAPT0R::ENABLE
    }
}
#[doc = "Possible values of the field `ANALOG_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANALOG_CTRLR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl ANALOG_CTRLR {
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
            ANALOG_CTRLR::DISABLE => false,
            ANALOG_CTRLR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ANALOG_CTRLR {
        match value {
            false => ANALOG_CTRLR::DISABLE,
            true => ANALOG_CTRLR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ANALOG_CTRLR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ANALOG_CTRLR::ENABLE
    }
}
#[doc = "Possible values of the field `HS_LSPI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_LSPIR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl HS_LSPIR {
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
            HS_LSPIR::DISABLE => false,
            HS_LSPIR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HS_LSPIR {
        match value {
            false => HS_LSPIR::DISABLE,
            true => HS_LSPIR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == HS_LSPIR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == HS_LSPIR::ENABLE
    }
}
#[doc = "Possible values of the field `GPIO_SEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_SECR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl GPIO_SECR {
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
            GPIO_SECR::DISABLE => false,
            GPIO_SECR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO_SECR {
        match value {
            false => GPIO_SECR::DISABLE,
            true => GPIO_SECR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_SECR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_SECR::ENABLE
    }
}
#[doc = "Possible values of the field `GPIO_SEC_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_SEC_INTR {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl GPIO_SEC_INTR {
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
            GPIO_SEC_INTR::DISABLE => false,
            GPIO_SEC_INTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO_SEC_INTR {
        match value {
            false => GPIO_SEC_INTR::DISABLE,
            true => GPIO_SEC_INTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_SEC_INTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_SEC_INTR::ENABLE
    }
}
#[doc = "Values that can be written to the field `DMA1`"]
pub enum DMA1W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl DMA1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA1W::DISABLE => false,
            DMA1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA1W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA1W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA1W::ENABLE)
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
#[doc = "Values that can be written to the field `COMP`"]
pub enum COMPW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl COMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPW::DISABLE => false,
            COMPW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPW::ENABLE)
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
#[doc = "Values that can be written to the field `SDIO`"]
pub enum SDIOW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
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
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDIOW::DISABLE)
    }
    #[doc = "Enable Clock."]
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USB1_HOST`"]
pub enum USB1_HOSTW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl USB1_HOSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB1_HOSTW::DISABLE => false,
            USB1_HOSTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB1_HOSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_HOSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB1_HOSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB1_HOSTW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB1_HOSTW::ENABLE)
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
#[doc = "Values that can be written to the field `USB1_DEV`"]
pub enum USB1_DEVW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl USB1_DEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB1_DEVW::DISABLE => false,
            USB1_DEVW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB1_DEVW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_DEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB1_DEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB1_DEVW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB1_DEVW::ENABLE)
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
#[doc = "Values that can be written to the field `USB1_RAM`"]
pub enum USB1_RAMW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl USB1_RAMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB1_RAMW::DISABLE => false,
            USB1_RAMW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB1_RAMW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_RAMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB1_RAMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB1_RAMW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB1_RAMW::ENABLE)
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
#[doc = "Values that can be written to the field `USB1_PHY`"]
pub enum USB1_PHYW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl USB1_PHYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB1_PHYW::DISABLE => false,
            USB1_PHYW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB1_PHYW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_PHYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB1_PHYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB1_PHYW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB1_PHYW::ENABLE)
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
#[doc = "Values that can be written to the field `FREQME`"]
pub enum FREQMEW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl FREQMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FREQMEW::DISABLE => false,
            FREQMEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FREQMEW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FREQMEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FREQMEW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FREQMEW::ENABLE)
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
#[doc = "Values that can be written to the field `GPIO4`"]
pub enum GPIO4W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl GPIO4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO4W::DISABLE => false,
            GPIO4W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO4W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO4W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO4W::ENABLE)
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
#[doc = "Values that can be written to the field `GPIO5`"]
pub enum GPIO5W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl GPIO5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO5W::DISABLE => false,
            GPIO5W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO5W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO5W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO5W::ENABLE)
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
#[doc = "Values that can be written to the field `OTP`"]
pub enum OTPW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl OTPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OTPW::DISABLE => false,
            OTPW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OTPW<'a> {
    w: &'a mut W,
}
impl<'a> _OTPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OTPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(OTPW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(OTPW::ENABLE)
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
#[doc = "Values that can be written to the field `RNG`"]
pub enum RNGW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl RNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RNGW::DISABLE => false,
            RNGW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RNGW<'a> {
    w: &'a mut W,
}
impl<'a> _RNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RNGW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RNGW::ENABLE)
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
#[doc = "Values that can be written to the field `MUX1`"]
pub enum MUX1W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl MUX1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MUX1W::DISABLE => false,
            MUX1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUX1W<'a> {
    w: &'a mut W,
}
impl<'a> _MUX1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUX1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(MUX1W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(MUX1W::ENABLE)
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
#[doc = "Values that can be written to the field `USB0_HOSTM`"]
pub enum USB0_HOSTMW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl USB0_HOSTMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB0_HOSTMW::DISABLE => false,
            USB0_HOSTMW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB0_HOSTMW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_HOSTMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB0_HOSTMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB0_HOSTMW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB0_HOSTMW::ENABLE)
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
#[doc = "Values that can be written to the field `USB0_HOSTS`"]
pub enum USB0_HOSTSW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl USB0_HOSTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB0_HOSTSW::DISABLE => false,
            USB0_HOSTSW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB0_HOSTSW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_HOSTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB0_HOSTSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB0_HOSTSW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB0_HOSTSW::ENABLE)
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
#[doc = "Values that can be written to the field `HASH0`"]
pub enum HASH0W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl HASH0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HASH0W::DISABLE => false,
            HASH0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HASH0W<'a> {
    w: &'a mut W,
}
impl<'a> _HASH0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HASH0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(HASH0W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(HASH0W::ENABLE)
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
#[doc = "Values that can be written to the field `PQ`"]
pub enum PQW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
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
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PQW::DISABLE)
    }
    #[doc = "Enable Clock."]
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLULUT`"]
pub enum PLULUTW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl PLULUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLULUTW::DISABLE => false,
            PLULUTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLULUTW<'a> {
    w: &'a mut W,
}
impl<'a> _PLULUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLULUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLULUTW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLULUTW::ENABLE)
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
#[doc = "Values that can be written to the field `TIMER3`"]
pub enum TIMER3W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl TIMER3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMER3W::DISABLE => false,
            TIMER3W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMER3W<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMER3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER3W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER3W::ENABLE)
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
#[doc = "Values that can be written to the field `TIMER4`"]
pub enum TIMER4W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl TIMER4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMER4W::DISABLE => false,
            TIMER4W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMER4W<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMER4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER4W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER4W::ENABLE)
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
#[doc = "Values that can be written to the field `PUF`"]
pub enum PUFW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl PUFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PUFW::DISABLE => false,
            PUFW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PUFW<'a> {
    w: &'a mut W,
}
impl<'a> _PUFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PUFW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PUFW::ENABLE)
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
#[doc = "Values that can be written to the field `CASPER`"]
pub enum CASPERW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl CASPERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CASPERW::DISABLE => false,
            CASPERW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CASPERW<'a> {
    w: &'a mut W,
}
impl<'a> _CASPERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CASPERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CASPERW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CASPERW::ENABLE)
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
#[doc = "Values that can be written to the field `CAPT0`"]
pub enum CAPT0W {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl CAPT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPT0W::DISABLE => false,
            CAPT0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPT0W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPT0W::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAPT0W::ENABLE)
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
#[doc = "Values that can be written to the field `ANALOG_CTRL`"]
pub enum ANALOG_CTRLW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl ANALOG_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ANALOG_CTRLW::DISABLE => false,
            ANALOG_CTRLW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ANALOG_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _ANALOG_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ANALOG_CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ANALOG_CTRLW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ANALOG_CTRLW::ENABLE)
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
#[doc = "Values that can be written to the field `HS_LSPI`"]
pub enum HS_LSPIW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl HS_LSPIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HS_LSPIW::DISABLE => false,
            HS_LSPIW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HS_LSPIW<'a> {
    w: &'a mut W,
}
impl<'a> _HS_LSPIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HS_LSPIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(HS_LSPIW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(HS_LSPIW::ENABLE)
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
#[doc = "Values that can be written to the field `GPIO_SEC`"]
pub enum GPIO_SECW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl GPIO_SECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO_SECW::DISABLE => false,
            GPIO_SECW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_SECW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_SECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_SECW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_SECW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_SECW::ENABLE)
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
#[doc = "Values that can be written to the field `GPIO_SEC_INT`"]
pub enum GPIO_SEC_INTW {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl GPIO_SEC_INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO_SEC_INTW::DISABLE => false,
            GPIO_SEC_INTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_SEC_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_SEC_INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_SEC_INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_SEC_INTW::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_SEC_INTW::ENABLE)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Enables the clock for the DMA1."]
    #[inline]
    pub fn dma1(&self) -> DMA1R {
        DMA1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enables the clock for the Comparator."]
    #[inline]
    pub fn comp(&self) -> COMPR {
        COMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enables the clock for the SDIO."]
    #[inline]
    pub fn sdio(&self) -> SDIOR {
        SDIOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enables the clock for the USB1 Host."]
    #[inline]
    pub fn usb1_host(&self) -> USB1_HOSTR {
        USB1_HOSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enables the clock for the USB1 dev."]
    #[inline]
    pub fn usb1_dev(&self) -> USB1_DEVR {
        USB1_DEVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enables the clock for the USB1 RAM."]
    #[inline]
    pub fn usb1_ram(&self) -> USB1_RAMR {
        USB1_RAMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enables the clock for the USB1 PHY."]
    #[inline]
    pub fn usb1_phy(&self) -> USB1_PHYR {
        USB1_PHYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enables the clock for the Frequency meter."]
    #[inline]
    pub fn freqme(&self) -> FREQMER {
        FREQMER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enables the clock for the GPIO4."]
    #[inline]
    pub fn gpio4(&self) -> GPIO4R {
        GPIO4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Enables the clock for the GPIO5."]
    #[inline]
    pub fn gpio5(&self) -> GPIO5R {
        GPIO5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Enables the clock for the OTP."]
    #[inline]
    pub fn otp(&self) -> OTPR {
        OTPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Enables the clock for the RNG."]
    #[inline]
    pub fn rng(&self) -> RNGR {
        RNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Enables the clock for the Peripheral Input Mux 1."]
    #[inline]
    pub fn mux1(&self) -> MUX1R {
        MUX1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Enables the clock for the USB0 Host Master."]
    #[inline]
    pub fn usb0_hostm(&self) -> USB0_HOSTMR {
        USB0_HOSTMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Enables the clock for the USB0 Host Slave."]
    #[inline]
    pub fn usb0_hosts(&self) -> USB0_HOSTSR {
        USB0_HOSTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Enables the clock for the HASH0."]
    #[inline]
    pub fn hash0(&self) -> HASH0R {
        HASH0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Enables the clock for the Power Quad."]
    #[inline]
    pub fn pq(&self) -> PQR {
        PQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Enables the clock for the PLU LUT."]
    #[inline]
    pub fn plulut(&self) -> PLULUTR {
        PLULUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Enables the clock for the Timer 3."]
    #[inline]
    pub fn timer3(&self) -> TIMER3R {
        TIMER3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Enables the clock for the Timer 4."]
    #[inline]
    pub fn timer4(&self) -> TIMER4R {
        TIMER4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Enables the clock for the PUF reset control."]
    #[inline]
    pub fn puf(&self) -> PUFR {
        PUFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Enables the clock for the Casper."]
    #[inline]
    pub fn casper(&self) -> CASPERR {
        CASPERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Enables the clock for the CAPT0."]
    #[inline]
    pub fn capt0(&self) -> CAPT0R {
        CAPT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Enables the clock for the analog control."]
    #[inline]
    pub fn analog_ctrl(&self) -> ANALOG_CTRLR {
        ANALOG_CTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Enables the clock for the HS LSPI."]
    #[inline]
    pub fn hs_lspi(&self) -> HS_LSPIR {
        HS_LSPIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Enables the clock for the GPIO secure."]
    #[inline]
    pub fn gpio_sec(&self) -> GPIO_SECR {
        GPIO_SECR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Enables the clock for the GPIO secure int."]
    #[inline]
    pub fn gpio_sec_int(&self) -> GPIO_SEC_INTR {
        GPIO_SEC_INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
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
    #[doc = "Bit 1 - Enables the clock for the DMA1."]
    #[inline]
    pub fn dma1(&mut self) -> _DMA1W {
        _DMA1W { w: self }
    }
    #[doc = "Bit 2 - Enables the clock for the Comparator."]
    #[inline]
    pub fn comp(&mut self) -> _COMPW {
        _COMPW { w: self }
    }
    #[doc = "Bit 3 - Enables the clock for the SDIO."]
    #[inline]
    pub fn sdio(&mut self) -> _SDIOW {
        _SDIOW { w: self }
    }
    #[doc = "Bit 4 - Enables the clock for the USB1 Host."]
    #[inline]
    pub fn usb1_host(&mut self) -> _USB1_HOSTW {
        _USB1_HOSTW { w: self }
    }
    #[doc = "Bit 5 - Enables the clock for the USB1 dev."]
    #[inline]
    pub fn usb1_dev(&mut self) -> _USB1_DEVW {
        _USB1_DEVW { w: self }
    }
    #[doc = "Bit 6 - Enables the clock for the USB1 RAM."]
    #[inline]
    pub fn usb1_ram(&mut self) -> _USB1_RAMW {
        _USB1_RAMW { w: self }
    }
    #[doc = "Bit 7 - Enables the clock for the USB1 PHY."]
    #[inline]
    pub fn usb1_phy(&mut self) -> _USB1_PHYW {
        _USB1_PHYW { w: self }
    }
    #[doc = "Bit 8 - Enables the clock for the Frequency meter."]
    #[inline]
    pub fn freqme(&mut self) -> _FREQMEW {
        _FREQMEW { w: self }
    }
    #[doc = "Bit 9 - Enables the clock for the GPIO4."]
    #[inline]
    pub fn gpio4(&mut self) -> _GPIO4W {
        _GPIO4W { w: self }
    }
    #[doc = "Bit 10 - Enables the clock for the GPIO5."]
    #[inline]
    pub fn gpio5(&mut self) -> _GPIO5W {
        _GPIO5W { w: self }
    }
    #[doc = "Bit 12 - Enables the clock for the OTP."]
    #[inline]
    pub fn otp(&mut self) -> _OTPW {
        _OTPW { w: self }
    }
    #[doc = "Bit 13 - Enables the clock for the RNG."]
    #[inline]
    pub fn rng(&mut self) -> _RNGW {
        _RNGW { w: self }
    }
    #[doc = "Bit 14 - Enables the clock for the Peripheral Input Mux 1."]
    #[inline]
    pub fn mux1(&mut self) -> _MUX1W {
        _MUX1W { w: self }
    }
    #[doc = "Bit 16 - Enables the clock for the USB0 Host Master."]
    #[inline]
    pub fn usb0_hostm(&mut self) -> _USB0_HOSTMW {
        _USB0_HOSTMW { w: self }
    }
    #[doc = "Bit 17 - Enables the clock for the USB0 Host Slave."]
    #[inline]
    pub fn usb0_hosts(&mut self) -> _USB0_HOSTSW {
        _USB0_HOSTSW { w: self }
    }
    #[doc = "Bit 18 - Enables the clock for the HASH0."]
    #[inline]
    pub fn hash0(&mut self) -> _HASH0W {
        _HASH0W { w: self }
    }
    #[doc = "Bit 19 - Enables the clock for the Power Quad."]
    #[inline]
    pub fn pq(&mut self) -> _PQW {
        _PQW { w: self }
    }
    #[doc = "Bit 20 - Enables the clock for the PLU LUT."]
    #[inline]
    pub fn plulut(&mut self) -> _PLULUTW {
        _PLULUTW { w: self }
    }
    #[doc = "Bit 21 - Enables the clock for the Timer 3."]
    #[inline]
    pub fn timer3(&mut self) -> _TIMER3W {
        _TIMER3W { w: self }
    }
    #[doc = "Bit 22 - Enables the clock for the Timer 4."]
    #[inline]
    pub fn timer4(&mut self) -> _TIMER4W {
        _TIMER4W { w: self }
    }
    #[doc = "Bit 23 - Enables the clock for the PUF reset control."]
    #[inline]
    pub fn puf(&mut self) -> _PUFW {
        _PUFW { w: self }
    }
    #[doc = "Bit 24 - Enables the clock for the Casper."]
    #[inline]
    pub fn casper(&mut self) -> _CASPERW {
        _CASPERW { w: self }
    }
    #[doc = "Bit 25 - Enables the clock for the CAPT0."]
    #[inline]
    pub fn capt0(&mut self) -> _CAPT0W {
        _CAPT0W { w: self }
    }
    #[doc = "Bit 27 - Enables the clock for the analog control."]
    #[inline]
    pub fn analog_ctrl(&mut self) -> _ANALOG_CTRLW {
        _ANALOG_CTRLW { w: self }
    }
    #[doc = "Bit 28 - Enables the clock for the HS LSPI."]
    #[inline]
    pub fn hs_lspi(&mut self) -> _HS_LSPIW {
        _HS_LSPIW { w: self }
    }
    #[doc = "Bit 29 - Enables the clock for the GPIO secure."]
    #[inline]
    pub fn gpio_sec(&mut self) -> _GPIO_SECW {
        _GPIO_SECW { w: self }
    }
    #[doc = "Bit 30 - Enables the clock for the GPIO secure int."]
    #[inline]
    pub fn gpio_sec_int(&mut self) -> _GPIO_SEC_INTW {
        _GPIO_SEC_INTW { w: self }
    }
}
