#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRESETCTRL2 {
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
#[doc = "Possible values of the field `DMA1_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl DMA1_RSTR {
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
            DMA1_RSTR::RELEASED => false,
            DMA1_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA1_RSTR {
        match value {
            false => DMA1_RSTR::RELEASED,
            true => DMA1_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == DMA1_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == DMA1_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `COMP_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl COMP_RSTR {
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
            COMP_RSTR::RELEASED => false,
            COMP_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMP_RSTR {
        match value {
            false => COMP_RSTR::RELEASED,
            true => COMP_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == COMP_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == COMP_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `SDIO_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIO_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl SDIO_RSTR {
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
            SDIO_RSTR::RELEASED => false,
            SDIO_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDIO_RSTR {
        match value {
            false => SDIO_RSTR::RELEASED,
            true => SDIO_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == SDIO_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == SDIO_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `USB1_HOST_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_HOST_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl USB1_HOST_RSTR {
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
            USB1_HOST_RSTR::RELEASED => false,
            USB1_HOST_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB1_HOST_RSTR {
        match value {
            false => USB1_HOST_RSTR::RELEASED,
            true => USB1_HOST_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == USB1_HOST_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == USB1_HOST_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `USB1_DEV_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_DEV_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl USB1_DEV_RSTR {
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
            USB1_DEV_RSTR::RELEASED => false,
            USB1_DEV_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB1_DEV_RSTR {
        match value {
            false => USB1_DEV_RSTR::RELEASED,
            true => USB1_DEV_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == USB1_DEV_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == USB1_DEV_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `USB1_RAM_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_RAM_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl USB1_RAM_RSTR {
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
            USB1_RAM_RSTR::RELEASED => false,
            USB1_RAM_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB1_RAM_RSTR {
        match value {
            false => USB1_RAM_RSTR::RELEASED,
            true => USB1_RAM_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == USB1_RAM_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == USB1_RAM_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `USB1_PHY_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_PHY_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl USB1_PHY_RSTR {
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
            USB1_PHY_RSTR::RELEASED => false,
            USB1_PHY_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB1_PHY_RSTR {
        match value {
            false => USB1_PHY_RSTR::RELEASED,
            true => USB1_PHY_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == USB1_PHY_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == USB1_PHY_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `FREQME_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQME_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FREQME_RSTR {
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
            FREQME_RSTR::RELEASED => false,
            FREQME_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FREQME_RSTR {
        match value {
            false => FREQME_RSTR::RELEASED,
            true => FREQME_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == FREQME_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == FREQME_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `GPIO4_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO4_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl GPIO4_RSTR {
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
            GPIO4_RSTR::RELEASED => false,
            GPIO4_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO4_RSTR {
        match value {
            false => GPIO4_RSTR::RELEASED,
            true => GPIO4_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == GPIO4_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == GPIO4_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `GPIO5_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO5_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl GPIO5_RSTR {
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
            GPIO5_RSTR::RELEASED => false,
            GPIO5_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO5_RSTR {
        match value {
            false => GPIO5_RSTR::RELEASED,
            true => GPIO5_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == GPIO5_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == GPIO5_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `OTP_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTP_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl OTP_RSTR {
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
            OTP_RSTR::RELEASED => false,
            OTP_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OTP_RSTR {
        match value {
            false => OTP_RSTR::RELEASED,
            true => OTP_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == OTP_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == OTP_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `RNG_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNG_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl RNG_RSTR {
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
            RNG_RSTR::RELEASED => false,
            RNG_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RNG_RSTR {
        match value {
            false => RNG_RSTR::RELEASED,
            true => RNG_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == RNG_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == RNG_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `MUX1_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX1_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl MUX1_RSTR {
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
            MUX1_RSTR::RELEASED => false,
            MUX1_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MUX1_RSTR {
        match value {
            false => MUX1_RSTR::RELEASED,
            true => MUX1_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == MUX1_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == MUX1_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `USB0_HOSTM_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_HOSTM_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl USB0_HOSTM_RSTR {
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
            USB0_HOSTM_RSTR::RELEASED => false,
            USB0_HOSTM_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB0_HOSTM_RSTR {
        match value {
            false => USB0_HOSTM_RSTR::RELEASED,
            true => USB0_HOSTM_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == USB0_HOSTM_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == USB0_HOSTM_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `USB0_HOSTS_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_HOSTS_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl USB0_HOSTS_RSTR {
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
            USB0_HOSTS_RSTR::RELEASED => false,
            USB0_HOSTS_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB0_HOSTS_RSTR {
        match value {
            false => USB0_HOSTS_RSTR::RELEASED,
            true => USB0_HOSTS_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == USB0_HOSTS_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == USB0_HOSTS_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `HASH0_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASH0_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl HASH0_RSTR {
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
            HASH0_RSTR::RELEASED => false,
            HASH0_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HASH0_RSTR {
        match value {
            false => HASH0_RSTR::RELEASED,
            true => HASH0_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == HASH0_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == HASH0_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `PQ_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PQ_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl PQ_RSTR {
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
            PQ_RSTR::RELEASED => false,
            PQ_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PQ_RSTR {
        match value {
            false => PQ_RSTR::RELEASED,
            true => PQ_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == PQ_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == PQ_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `PLULUT_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLULUT_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl PLULUT_RSTR {
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
            PLULUT_RSTR::RELEASED => false,
            PLULUT_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLULUT_RSTR {
        match value {
            false => PLULUT_RSTR::RELEASED,
            true => PLULUT_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == PLULUT_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == PLULUT_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `TIMER3_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER3_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl TIMER3_RSTR {
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
            TIMER3_RSTR::RELEASED => false,
            TIMER3_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMER3_RSTR {
        match value {
            false => TIMER3_RSTR::RELEASED,
            true => TIMER3_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == TIMER3_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == TIMER3_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `TIMER4_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER4_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl TIMER4_RSTR {
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
            TIMER4_RSTR::RELEASED => false,
            TIMER4_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMER4_RSTR {
        match value {
            false => TIMER4_RSTR::RELEASED,
            true => TIMER4_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == TIMER4_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == TIMER4_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `PUF_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUF_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl PUF_RSTR {
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
            PUF_RSTR::RELEASED => false,
            PUF_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PUF_RSTR {
        match value {
            false => PUF_RSTR::RELEASED,
            true => PUF_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == PUF_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == PUF_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `CASPER_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CASPER_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl CASPER_RSTR {
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
            CASPER_RSTR::RELEASED => false,
            CASPER_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CASPER_RSTR {
        match value {
            false => CASPER_RSTR::RELEASED,
            true => CASPER_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == CASPER_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == CASPER_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `CAPT0_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT0_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl CAPT0_RSTR {
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
            CAPT0_RSTR::RELEASED => false,
            CAPT0_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPT0_RSTR {
        match value {
            false => CAPT0_RSTR::RELEASED,
            true => CAPT0_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == CAPT0_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == CAPT0_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `ANALOG_CTRL_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANALOG_CTRL_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl ANALOG_CTRL_RSTR {
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
            ANALOG_CTRL_RSTR::RELEASED => false,
            ANALOG_CTRL_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ANALOG_CTRL_RSTR {
        match value {
            false => ANALOG_CTRL_RSTR::RELEASED,
            true => ANALOG_CTRL_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == ANALOG_CTRL_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == ANALOG_CTRL_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `HS_LSPI_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_LSPI_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl HS_LSPI_RSTR {
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
            HS_LSPI_RSTR::RELEASED => false,
            HS_LSPI_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HS_LSPI_RSTR {
        match value {
            false => HS_LSPI_RSTR::RELEASED,
            true => HS_LSPI_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == HS_LSPI_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == HS_LSPI_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `GPIO_SEC_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_SEC_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl GPIO_SEC_RSTR {
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
            GPIO_SEC_RSTR::RELEASED => false,
            GPIO_SEC_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO_SEC_RSTR {
        match value {
            false => GPIO_SEC_RSTR::RELEASED,
            true => GPIO_SEC_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == GPIO_SEC_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == GPIO_SEC_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `GPIO_SEC_INT_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_SEC_INT_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl GPIO_SEC_INT_RSTR {
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
            GPIO_SEC_INT_RSTR::RELEASED => false,
            GPIO_SEC_INT_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO_SEC_INT_RSTR {
        match value {
            false => GPIO_SEC_INT_RSTR::RELEASED,
            true => GPIO_SEC_INT_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == GPIO_SEC_INT_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == GPIO_SEC_INT_RSTR::ASSERTED
    }
}
#[doc = "Values that can be written to the field `DMA1_RST`"]
pub enum DMA1_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl DMA1_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA1_RSTW::RELEASED => false,
            DMA1_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA1_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA1_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA1_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(DMA1_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(DMA1_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `COMP_RST`"]
pub enum COMP_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl COMP_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMP_RSTW::RELEASED => false,
            COMP_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMP_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _COMP_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMP_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(COMP_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(COMP_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `SDIO_RST`"]
pub enum SDIO_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl SDIO_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDIO_RSTW::RELEASED => false,
            SDIO_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDIO_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SDIO_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDIO_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(SDIO_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SDIO_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `USB1_HOST_RST`"]
pub enum USB1_HOST_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl USB1_HOST_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB1_HOST_RSTW::RELEASED => false,
            USB1_HOST_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB1_HOST_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_HOST_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB1_HOST_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(USB1_HOST_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB1_HOST_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `USB1_DEV_RST`"]
pub enum USB1_DEV_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl USB1_DEV_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB1_DEV_RSTW::RELEASED => false,
            USB1_DEV_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB1_DEV_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_DEV_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB1_DEV_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(USB1_DEV_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB1_DEV_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `USB1_RAM_RST`"]
pub enum USB1_RAM_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl USB1_RAM_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB1_RAM_RSTW::RELEASED => false,
            USB1_RAM_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB1_RAM_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_RAM_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB1_RAM_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(USB1_RAM_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB1_RAM_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `USB1_PHY_RST`"]
pub enum USB1_PHY_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl USB1_PHY_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB1_PHY_RSTW::RELEASED => false,
            USB1_PHY_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB1_PHY_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_PHY_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB1_PHY_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(USB1_PHY_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB1_PHY_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `FREQME_RST`"]
pub enum FREQME_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FREQME_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FREQME_RSTW::RELEASED => false,
            FREQME_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FREQME_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQME_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FREQME_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(FREQME_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FREQME_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `GPIO4_RST`"]
pub enum GPIO4_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl GPIO4_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO4_RSTW::RELEASED => false,
            GPIO4_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO4_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO4_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO4_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(GPIO4_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GPIO4_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `GPIO5_RST`"]
pub enum GPIO5_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl GPIO5_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO5_RSTW::RELEASED => false,
            GPIO5_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO5_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO5_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO5_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(GPIO5_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GPIO5_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `OTP_RST`"]
pub enum OTP_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl OTP_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OTP_RSTW::RELEASED => false,
            OTP_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OTP_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _OTP_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OTP_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(OTP_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(OTP_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `RNG_RST`"]
pub enum RNG_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl RNG_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RNG_RSTW::RELEASED => false,
            RNG_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RNG_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RNG_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RNG_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(RNG_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(RNG_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `MUX1_RST`"]
pub enum MUX1_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl MUX1_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MUX1_RSTW::RELEASED => false,
            MUX1_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUX1_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MUX1_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUX1_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(MUX1_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(MUX1_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `USB0_HOSTM_RST`"]
pub enum USB0_HOSTM_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl USB0_HOSTM_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB0_HOSTM_RSTW::RELEASED => false,
            USB0_HOSTM_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB0_HOSTM_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_HOSTM_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB0_HOSTM_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(USB0_HOSTM_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB0_HOSTM_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `USB0_HOSTS_RST`"]
pub enum USB0_HOSTS_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl USB0_HOSTS_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB0_HOSTS_RSTW::RELEASED => false,
            USB0_HOSTS_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB0_HOSTS_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_HOSTS_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB0_HOSTS_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(USB0_HOSTS_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB0_HOSTS_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `HASH0_RST`"]
pub enum HASH0_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl HASH0_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HASH0_RSTW::RELEASED => false,
            HASH0_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HASH0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HASH0_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HASH0_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(HASH0_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(HASH0_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `PQ_RST`"]
pub enum PQ_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl PQ_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PQ_RSTW::RELEASED => false,
            PQ_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PQ_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PQ_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PQ_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(PQ_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(PQ_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `PLULUT_RST`"]
pub enum PLULUT_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl PLULUT_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLULUT_RSTW::RELEASED => false,
            PLULUT_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLULUT_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PLULUT_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLULUT_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(PLULUT_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(PLULUT_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `TIMER3_RST`"]
pub enum TIMER3_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl TIMER3_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMER3_RSTW::RELEASED => false,
            TIMER3_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMER3_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER3_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMER3_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(TIMER3_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TIMER3_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `TIMER4_RST`"]
pub enum TIMER4_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl TIMER4_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMER4_RSTW::RELEASED => false,
            TIMER4_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMER4_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER4_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMER4_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(TIMER4_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TIMER4_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `PUF_RST`"]
pub enum PUF_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl PUF_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PUF_RSTW::RELEASED => false,
            PUF_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PUF_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PUF_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUF_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(PUF_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(PUF_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `CASPER_RST`"]
pub enum CASPER_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl CASPER_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CASPER_RSTW::RELEASED => false,
            CASPER_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CASPER_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CASPER_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CASPER_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(CASPER_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(CASPER_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `CAPT0_RST`"]
pub enum CAPT0_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl CAPT0_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPT0_RSTW::RELEASED => false,
            CAPT0_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPT0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPT0_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPT0_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(CAPT0_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(CAPT0_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `ANALOG_CTRL_RST`"]
pub enum ANALOG_CTRL_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl ANALOG_CTRL_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ANALOG_CTRL_RSTW::RELEASED => false,
            ANALOG_CTRL_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ANALOG_CTRL_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ANALOG_CTRL_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ANALOG_CTRL_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(ANALOG_CTRL_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(ANALOG_CTRL_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `HS_LSPI_RST`"]
pub enum HS_LSPI_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl HS_LSPI_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HS_LSPI_RSTW::RELEASED => false,
            HS_LSPI_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HS_LSPI_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HS_LSPI_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HS_LSPI_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(HS_LSPI_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(HS_LSPI_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `GPIO_SEC_RST`"]
pub enum GPIO_SEC_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl GPIO_SEC_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO_SEC_RSTW::RELEASED => false,
            GPIO_SEC_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_SEC_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_SEC_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_SEC_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(GPIO_SEC_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GPIO_SEC_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `GPIO_SEC_INT_RST`"]
pub enum GPIO_SEC_INT_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl GPIO_SEC_INT_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO_SEC_INT_RSTW::RELEASED => false,
            GPIO_SEC_INT_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_SEC_INT_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_SEC_INT_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_SEC_INT_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(GPIO_SEC_INT_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GPIO_SEC_INT_RSTW::ASSERTED)
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
    #[doc = "Bit 1 - DMA1 reset control."]
    #[inline]
    pub fn dma1_rst(&self) -> DMA1_RSTR {
        DMA1_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Comparator reset control."]
    #[inline]
    pub fn comp_rst(&self) -> COMP_RSTR {
        COMP_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - SDIO reset control."]
    #[inline]
    pub fn sdio_rst(&self) -> SDIO_RSTR {
        SDIO_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - USB1 Host reset control."]
    #[inline]
    pub fn usb1_host_rst(&self) -> USB1_HOST_RSTR {
        USB1_HOST_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - USB1 dev reset control."]
    #[inline]
    pub fn usb1_dev_rst(&self) -> USB1_DEV_RSTR {
        USB1_DEV_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - USB1 RAM reset control."]
    #[inline]
    pub fn usb1_ram_rst(&self) -> USB1_RAM_RSTR {
        USB1_RAM_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - USB1 PHY reset control."]
    #[inline]
    pub fn usb1_phy_rst(&self) -> USB1_PHY_RSTR {
        USB1_PHY_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Frequency meter reset control."]
    #[inline]
    pub fn freqme_rst(&self) -> FREQME_RSTR {
        FREQME_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - GPIO4 reset control."]
    #[inline]
    pub fn gpio4_rst(&self) -> GPIO4_RSTR {
        GPIO4_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - GPIO5 reset control."]
    #[inline]
    pub fn gpio5_rst(&self) -> GPIO5_RSTR {
        GPIO5_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - OTP reset control."]
    #[inline]
    pub fn otp_rst(&self) -> OTP_RSTR {
        OTP_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - RNG reset control."]
    #[inline]
    pub fn rng_rst(&self) -> RNG_RSTR {
        RNG_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Peripheral Input Mux 1 reset control."]
    #[inline]
    pub fn mux1_rst(&self) -> MUX1_RSTR {
        MUX1_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - USB0 Host Master reset control."]
    #[inline]
    pub fn usb0_hostm_rst(&self) -> USB0_HOSTM_RSTR {
        USB0_HOSTM_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - USB0 Host Slave reset control."]
    #[inline]
    pub fn usb0_hosts_rst(&self) -> USB0_HOSTS_RSTR {
        USB0_HOSTS_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - HASH0 reset control."]
    #[inline]
    pub fn hash0_rst(&self) -> HASH0_RSTR {
        HASH0_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Power Quad reset control."]
    #[inline]
    pub fn pq_rst(&self) -> PQ_RSTR {
        PQ_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - PLU LUT reset control."]
    #[inline]
    pub fn plulut_rst(&self) -> PLULUT_RSTR {
        PLULUT_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Timer 3 reset control."]
    #[inline]
    pub fn timer3_rst(&self) -> TIMER3_RSTR {
        TIMER3_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Timer 4 reset control."]
    #[inline]
    pub fn timer4_rst(&self) -> TIMER4_RSTR {
        TIMER4_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - PUF reset control reset control."]
    #[inline]
    pub fn puf_rst(&self) -> PUF_RSTR {
        PUF_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Casper reset control."]
    #[inline]
    pub fn casper_rst(&self) -> CASPER_RSTR {
        CASPER_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - CAPT0 reset control."]
    #[inline]
    pub fn capt0_rst(&self) -> CAPT0_RSTR {
        CAPT0_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - analog control reset control."]
    #[inline]
    pub fn analog_ctrl_rst(&self) -> ANALOG_CTRL_RSTR {
        ANALOG_CTRL_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - HS LSPI reset control."]
    #[inline]
    pub fn hs_lspi_rst(&self) -> HS_LSPI_RSTR {
        HS_LSPI_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - GPIO secure reset control."]
    #[inline]
    pub fn gpio_sec_rst(&self) -> GPIO_SEC_RSTR {
        GPIO_SEC_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - GPIO secure int reset control."]
    #[inline]
    pub fn gpio_sec_int_rst(&self) -> GPIO_SEC_INT_RSTR {
        GPIO_SEC_INT_RSTR::_from({
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
    #[doc = "Bit 1 - DMA1 reset control."]
    #[inline]
    pub fn dma1_rst(&mut self) -> _DMA1_RSTW {
        _DMA1_RSTW { w: self }
    }
    #[doc = "Bit 2 - Comparator reset control."]
    #[inline]
    pub fn comp_rst(&mut self) -> _COMP_RSTW {
        _COMP_RSTW { w: self }
    }
    #[doc = "Bit 3 - SDIO reset control."]
    #[inline]
    pub fn sdio_rst(&mut self) -> _SDIO_RSTW {
        _SDIO_RSTW { w: self }
    }
    #[doc = "Bit 4 - USB1 Host reset control."]
    #[inline]
    pub fn usb1_host_rst(&mut self) -> _USB1_HOST_RSTW {
        _USB1_HOST_RSTW { w: self }
    }
    #[doc = "Bit 5 - USB1 dev reset control."]
    #[inline]
    pub fn usb1_dev_rst(&mut self) -> _USB1_DEV_RSTW {
        _USB1_DEV_RSTW { w: self }
    }
    #[doc = "Bit 6 - USB1 RAM reset control."]
    #[inline]
    pub fn usb1_ram_rst(&mut self) -> _USB1_RAM_RSTW {
        _USB1_RAM_RSTW { w: self }
    }
    #[doc = "Bit 7 - USB1 PHY reset control."]
    #[inline]
    pub fn usb1_phy_rst(&mut self) -> _USB1_PHY_RSTW {
        _USB1_PHY_RSTW { w: self }
    }
    #[doc = "Bit 8 - Frequency meter reset control."]
    #[inline]
    pub fn freqme_rst(&mut self) -> _FREQME_RSTW {
        _FREQME_RSTW { w: self }
    }
    #[doc = "Bit 9 - GPIO4 reset control."]
    #[inline]
    pub fn gpio4_rst(&mut self) -> _GPIO4_RSTW {
        _GPIO4_RSTW { w: self }
    }
    #[doc = "Bit 10 - GPIO5 reset control."]
    #[inline]
    pub fn gpio5_rst(&mut self) -> _GPIO5_RSTW {
        _GPIO5_RSTW { w: self }
    }
    #[doc = "Bit 12 - OTP reset control."]
    #[inline]
    pub fn otp_rst(&mut self) -> _OTP_RSTW {
        _OTP_RSTW { w: self }
    }
    #[doc = "Bit 13 - RNG reset control."]
    #[inline]
    pub fn rng_rst(&mut self) -> _RNG_RSTW {
        _RNG_RSTW { w: self }
    }
    #[doc = "Bit 14 - Peripheral Input Mux 1 reset control."]
    #[inline]
    pub fn mux1_rst(&mut self) -> _MUX1_RSTW {
        _MUX1_RSTW { w: self }
    }
    #[doc = "Bit 16 - USB0 Host Master reset control."]
    #[inline]
    pub fn usb0_hostm_rst(&mut self) -> _USB0_HOSTM_RSTW {
        _USB0_HOSTM_RSTW { w: self }
    }
    #[doc = "Bit 17 - USB0 Host Slave reset control."]
    #[inline]
    pub fn usb0_hosts_rst(&mut self) -> _USB0_HOSTS_RSTW {
        _USB0_HOSTS_RSTW { w: self }
    }
    #[doc = "Bit 18 - HASH0 reset control."]
    #[inline]
    pub fn hash0_rst(&mut self) -> _HASH0_RSTW {
        _HASH0_RSTW { w: self }
    }
    #[doc = "Bit 19 - Power Quad reset control."]
    #[inline]
    pub fn pq_rst(&mut self) -> _PQ_RSTW {
        _PQ_RSTW { w: self }
    }
    #[doc = "Bit 20 - PLU LUT reset control."]
    #[inline]
    pub fn plulut_rst(&mut self) -> _PLULUT_RSTW {
        _PLULUT_RSTW { w: self }
    }
    #[doc = "Bit 21 - Timer 3 reset control."]
    #[inline]
    pub fn timer3_rst(&mut self) -> _TIMER3_RSTW {
        _TIMER3_RSTW { w: self }
    }
    #[doc = "Bit 22 - Timer 4 reset control."]
    #[inline]
    pub fn timer4_rst(&mut self) -> _TIMER4_RSTW {
        _TIMER4_RSTW { w: self }
    }
    #[doc = "Bit 23 - PUF reset control reset control."]
    #[inline]
    pub fn puf_rst(&mut self) -> _PUF_RSTW {
        _PUF_RSTW { w: self }
    }
    #[doc = "Bit 24 - Casper reset control."]
    #[inline]
    pub fn casper_rst(&mut self) -> _CASPER_RSTW {
        _CASPER_RSTW { w: self }
    }
    #[doc = "Bit 25 - CAPT0 reset control."]
    #[inline]
    pub fn capt0_rst(&mut self) -> _CAPT0_RSTW {
        _CAPT0_RSTW { w: self }
    }
    #[doc = "Bit 27 - analog control reset control."]
    #[inline]
    pub fn analog_ctrl_rst(&mut self) -> _ANALOG_CTRL_RSTW {
        _ANALOG_CTRL_RSTW { w: self }
    }
    #[doc = "Bit 28 - HS LSPI reset control."]
    #[inline]
    pub fn hs_lspi_rst(&mut self) -> _HS_LSPI_RSTW {
        _HS_LSPI_RSTW { w: self }
    }
    #[doc = "Bit 29 - GPIO secure reset control."]
    #[inline]
    pub fn gpio_sec_rst(&mut self) -> _GPIO_SEC_RSTW {
        _GPIO_SEC_RSTW { w: self }
    }
    #[doc = "Bit 30 - GPIO secure int reset control."]
    #[inline]
    pub fn gpio_sec_int_rst(&mut self) -> _GPIO_SEC_INT_RSTW {
        _GPIO_SEC_INT_RSTW { w: self }
    }
}
