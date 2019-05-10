#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRESETCTRL0 {
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
#[doc = "Possible values of the field `ROM_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROM_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl ROM_RSTR {
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
            ROM_RSTR::RELEASED => false,
            ROM_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ROM_RSTR {
        match value {
            false => ROM_RSTR::RELEASED,
            true => ROM_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == ROM_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == ROM_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `SRAM_CTRL1_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL1_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl SRAM_CTRL1_RSTR {
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
            SRAM_CTRL1_RSTR::RELEASED => false,
            SRAM_CTRL1_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRAM_CTRL1_RSTR {
        match value {
            false => SRAM_CTRL1_RSTR::RELEASED,
            true => SRAM_CTRL1_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == SRAM_CTRL1_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == SRAM_CTRL1_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `SRAM_CTRL2_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL2_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl SRAM_CTRL2_RSTR {
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
            SRAM_CTRL2_RSTR::RELEASED => false,
            SRAM_CTRL2_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRAM_CTRL2_RSTR {
        match value {
            false => SRAM_CTRL2_RSTR::RELEASED,
            true => SRAM_CTRL2_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == SRAM_CTRL2_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == SRAM_CTRL2_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `SRAM_CTRL3_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL3_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl SRAM_CTRL3_RSTR {
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
            SRAM_CTRL3_RSTR::RELEASED => false,
            SRAM_CTRL3_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRAM_CTRL3_RSTR {
        match value {
            false => SRAM_CTRL3_RSTR::RELEASED,
            true => SRAM_CTRL3_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == SRAM_CTRL3_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == SRAM_CTRL3_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `SRAM_CTRL4_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL4_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl SRAM_CTRL4_RSTR {
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
            SRAM_CTRL4_RSTR::RELEASED => false,
            SRAM_CTRL4_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRAM_CTRL4_RSTR {
        match value {
            false => SRAM_CTRL4_RSTR::RELEASED,
            true => SRAM_CTRL4_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == SRAM_CTRL4_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == SRAM_CTRL4_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `FLASH_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FLASH_RSTR {
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
            FLASH_RSTR::RELEASED => false,
            FLASH_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASH_RSTR {
        match value {
            false => FLASH_RSTR::RELEASED,
            true => FLASH_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == FLASH_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == FLASH_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `FMC_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMC_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FMC_RSTR {
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
            FMC_RSTR::RELEASED => false,
            FMC_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FMC_RSTR {
        match value {
            false => FMC_RSTR::RELEASED,
            true => FMC_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == FMC_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == FMC_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `MUX0_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX0_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl MUX0_RSTR {
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
            MUX0_RSTR::RELEASED => false,
            MUX0_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MUX0_RSTR {
        match value {
            false => MUX0_RSTR::RELEASED,
            true => MUX0_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == MUX0_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == MUX0_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `IOCON_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCON_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl IOCON_RSTR {
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
            IOCON_RSTR::RELEASED => false,
            IOCON_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOCON_RSTR {
        match value {
            false => IOCON_RSTR::RELEASED,
            true => IOCON_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == IOCON_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == IOCON_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `GPIO0_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl GPIO0_RSTR {
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
            GPIO0_RSTR::RELEASED => false,
            GPIO0_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO0_RSTR {
        match value {
            false => GPIO0_RSTR::RELEASED,
            true => GPIO0_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == GPIO0_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == GPIO0_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `GPIO1_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO1_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl GPIO1_RSTR {
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
            GPIO1_RSTR::RELEASED => false,
            GPIO1_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO1_RSTR {
        match value {
            false => GPIO1_RSTR::RELEASED,
            true => GPIO1_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == GPIO1_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == GPIO1_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `GPIO2_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO2_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl GPIO2_RSTR {
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
            GPIO2_RSTR::RELEASED => false,
            GPIO2_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO2_RSTR {
        match value {
            false => GPIO2_RSTR::RELEASED,
            true => GPIO2_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == GPIO2_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == GPIO2_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `GPIO3_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO3_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl GPIO3_RSTR {
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
            GPIO3_RSTR::RELEASED => false,
            GPIO3_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO3_RSTR {
        match value {
            false => GPIO3_RSTR::RELEASED,
            true => GPIO3_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == GPIO3_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == GPIO3_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `PINT_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl PINT_RSTR {
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
            PINT_RSTR::RELEASED => false,
            PINT_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PINT_RSTR {
        match value {
            false => PINT_RSTR::RELEASED,
            true => PINT_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == PINT_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == PINT_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `GINT_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GINT_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl GINT_RSTR {
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
            GINT_RSTR::RELEASED => false,
            GINT_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GINT_RSTR {
        match value {
            false => GINT_RSTR::RELEASED,
            true => GINT_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == GINT_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == GINT_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `DMA0_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA0_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl DMA0_RSTR {
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
            DMA0_RSTR::RELEASED => false,
            DMA0_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA0_RSTR {
        match value {
            false => DMA0_RSTR::RELEASED,
            true => DMA0_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == DMA0_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == DMA0_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `CRCGEN_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCGEN_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl CRCGEN_RSTR {
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
            CRCGEN_RSTR::RELEASED => false,
            CRCGEN_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRCGEN_RSTR {
        match value {
            false => CRCGEN_RSTR::RELEASED,
            true => CRCGEN_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == CRCGEN_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == CRCGEN_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `WWDT_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDT_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl WWDT_RSTR {
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
            WWDT_RSTR::RELEASED => false,
            WWDT_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WWDT_RSTR {
        match value {
            false => WWDT_RSTR::RELEASED,
            true => WWDT_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == WWDT_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == WWDT_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `RTC_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl RTC_RSTR {
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
            RTC_RSTR::RELEASED => false,
            RTC_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_RSTR {
        match value {
            false => RTC_RSTR::RELEASED,
            true => RTC_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == RTC_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == RTC_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `MAILBOX_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAILBOX_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl MAILBOX_RSTR {
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
            MAILBOX_RSTR::RELEASED => false,
            MAILBOX_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAILBOX_RSTR {
        match value {
            false => MAILBOX_RSTR::RELEASED,
            true => MAILBOX_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == MAILBOX_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == MAILBOX_RSTR::ASSERTED
    }
}
#[doc = "Possible values of the field `ADC_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_RSTR {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl ADC_RSTR {
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
            ADC_RSTR::RELEASED => false,
            ADC_RSTR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_RSTR {
        match value {
            false => ADC_RSTR::RELEASED,
            true => ADC_RSTR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline]
    pub fn is_released(&self) -> bool {
        *self == ADC_RSTR::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == ADC_RSTR::ASSERTED
    }
}
#[doc = "Values that can be written to the field `ROM_RST`"]
pub enum ROM_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl ROM_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ROM_RSTW::RELEASED => false,
            ROM_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROM_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ROM_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROM_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(ROM_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(ROM_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `SRAM_CTRL1_RST`"]
pub enum SRAM_CTRL1_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl SRAM_CTRL1_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRAM_CTRL1_RSTW::RELEASED => false,
            SRAM_CTRL1_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRAM_CTRL1_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAM_CTRL1_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAM_CTRL1_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(SRAM_CTRL1_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SRAM_CTRL1_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `SRAM_CTRL2_RST`"]
pub enum SRAM_CTRL2_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl SRAM_CTRL2_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRAM_CTRL2_RSTW::RELEASED => false,
            SRAM_CTRL2_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRAM_CTRL2_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAM_CTRL2_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAM_CTRL2_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(SRAM_CTRL2_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SRAM_CTRL2_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `SRAM_CTRL3_RST`"]
pub enum SRAM_CTRL3_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl SRAM_CTRL3_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRAM_CTRL3_RSTW::RELEASED => false,
            SRAM_CTRL3_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRAM_CTRL3_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAM_CTRL3_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAM_CTRL3_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(SRAM_CTRL3_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SRAM_CTRL3_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `SRAM_CTRL4_RST`"]
pub enum SRAM_CTRL4_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl SRAM_CTRL4_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRAM_CTRL4_RSTW::RELEASED => false,
            SRAM_CTRL4_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRAM_CTRL4_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAM_CTRL4_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAM_CTRL4_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(SRAM_CTRL4_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SRAM_CTRL4_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `FLASH_RST`"]
pub enum FLASH_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FLASH_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASH_RSTW::RELEASED => false,
            FLASH_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASH_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASH_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(FLASH_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FLASH_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `FMC_RST`"]
pub enum FMC_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl FMC_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FMC_RSTW::RELEASED => false,
            FMC_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FMC_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FMC_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FMC_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(FMC_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FMC_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `MUX0_RST`"]
pub enum MUX0_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl MUX0_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MUX0_RSTW::RELEASED => false,
            MUX0_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUX0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MUX0_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUX0_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(MUX0_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(MUX0_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `IOCON_RST`"]
pub enum IOCON_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl IOCON_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOCON_RSTW::RELEASED => false,
            IOCON_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOCON_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _IOCON_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOCON_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(IOCON_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(IOCON_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `GPIO0_RST`"]
pub enum GPIO0_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl GPIO0_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO0_RSTW::RELEASED => false,
            GPIO0_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO0_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO0_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(GPIO0_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GPIO0_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `GPIO1_RST`"]
pub enum GPIO1_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl GPIO1_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO1_RSTW::RELEASED => false,
            GPIO1_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO1_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO1_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO1_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(GPIO1_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GPIO1_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `GPIO2_RST`"]
pub enum GPIO2_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl GPIO2_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO2_RSTW::RELEASED => false,
            GPIO2_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO2_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO2_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO2_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(GPIO2_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GPIO2_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `GPIO3_RST`"]
pub enum GPIO3_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl GPIO3_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO3_RSTW::RELEASED => false,
            GPIO3_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO3_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO3_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO3_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(GPIO3_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GPIO3_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `PINT_RST`"]
pub enum PINT_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl PINT_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PINT_RSTW::RELEASED => false,
            PINT_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PINT_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PINT_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINT_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(PINT_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(PINT_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `GINT_RST`"]
pub enum GINT_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl GINT_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GINT_RSTW::RELEASED => false,
            GINT_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GINT_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GINT_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GINT_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(GINT_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GINT_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `DMA0_RST`"]
pub enum DMA0_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl DMA0_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA0_RSTW::RELEASED => false,
            DMA0_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA0_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA0_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(DMA0_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(DMA0_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `CRCGEN_RST`"]
pub enum CRCGEN_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl CRCGEN_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRCGEN_RSTW::RELEASED => false,
            CRCGEN_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRCGEN_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCGEN_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCGEN_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(CRCGEN_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(CRCGEN_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `WWDT_RST`"]
pub enum WWDT_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl WWDT_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WWDT_RSTW::RELEASED => false,
            WWDT_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WWDT_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDT_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WWDT_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(WWDT_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(WWDT_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `RTC_RST`"]
pub enum RTC_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl RTC_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_RSTW::RELEASED => false,
            RTC_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(RTC_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(RTC_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `MAILBOX_RST`"]
pub enum MAILBOX_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl MAILBOX_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAILBOX_RSTW::RELEASED => false,
            MAILBOX_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAILBOX_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MAILBOX_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAILBOX_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(MAILBOX_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(MAILBOX_RSTW::ASSERTED)
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
#[doc = "Values that can be written to the field `ADC_RST`"]
pub enum ADC_RSTW {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl ADC_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_RSTW::RELEASED => false,
            ADC_RSTW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(ADC_RSTW::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(ADC_RSTW::ASSERTED)
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
    #[doc = "Bit 1 - ROM reset control."]
    #[inline]
    pub fn rom_rst(&self) -> ROM_RSTR {
        ROM_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - SRAM Controller 1 reset control."]
    #[inline]
    pub fn sram_ctrl1_rst(&self) -> SRAM_CTRL1_RSTR {
        SRAM_CTRL1_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - SRAM Controller 2 reset control."]
    #[inline]
    pub fn sram_ctrl2_rst(&self) -> SRAM_CTRL2_RSTR {
        SRAM_CTRL2_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - SRAM Controller 3 reset control."]
    #[inline]
    pub fn sram_ctrl3_rst(&self) -> SRAM_CTRL3_RSTR {
        SRAM_CTRL3_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - SRAM Controller 4 reset control."]
    #[inline]
    pub fn sram_ctrl4_rst(&self) -> SRAM_CTRL4_RSTR {
        SRAM_CTRL4_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Flash controller reset control."]
    #[inline]
    pub fn flash_rst(&self) -> FLASH_RSTR {
        FLASH_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - FMC controller reset control."]
    #[inline]
    pub fn fmc_rst(&self) -> FMC_RSTR {
        FMC_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Input Mux 0 reset control."]
    #[inline]
    pub fn mux0_rst(&self) -> MUX0_RSTR {
        MUX0_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - I/O controller reset control."]
    #[inline]
    pub fn iocon_rst(&self) -> IOCON_RSTR {
        IOCON_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - GPIO0 reset control."]
    #[inline]
    pub fn gpio0_rst(&self) -> GPIO0_RSTR {
        GPIO0_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - GPIO1 reset control."]
    #[inline]
    pub fn gpio1_rst(&self) -> GPIO1_RSTR {
        GPIO1_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - GPIO2 reset control."]
    #[inline]
    pub fn gpio2_rst(&self) -> GPIO2_RSTR {
        GPIO2_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - GPIO3 reset control."]
    #[inline]
    pub fn gpio3_rst(&self) -> GPIO3_RSTR {
        GPIO3_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Pin interrupt (PINT) reset control."]
    #[inline]
    pub fn pint_rst(&self) -> PINT_RSTR {
        PINT_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Group interrupt (GINT) reset control."]
    #[inline]
    pub fn gint_rst(&self) -> GINT_RSTR {
        GINT_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - DMA0 reset control."]
    #[inline]
    pub fn dma0_rst(&self) -> DMA0_RSTR {
        DMA0_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - CRCGEN reset control."]
    #[inline]
    pub fn crcgen_rst(&self) -> CRCGEN_RSTR {
        CRCGEN_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Watchdog Timer reset control."]
    #[inline]
    pub fn wwdt_rst(&self) -> WWDT_RSTR {
        WWDT_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Real Time Clock (RTC) reset control."]
    #[inline]
    pub fn rtc_rst(&self) -> RTC_RSTR {
        RTC_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Inter CPU communication Mailbox reset control."]
    #[inline]
    pub fn mailbox_rst(&self) -> MAILBOX_RSTR {
        MAILBOX_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - ADC reset control."]
    #[inline]
    pub fn adc_rst(&self) -> ADC_RSTR {
        ADC_RSTR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - ROM reset control."]
    #[inline]
    pub fn rom_rst(&mut self) -> _ROM_RSTW {
        _ROM_RSTW { w: self }
    }
    #[doc = "Bit 3 - SRAM Controller 1 reset control."]
    #[inline]
    pub fn sram_ctrl1_rst(&mut self) -> _SRAM_CTRL1_RSTW {
        _SRAM_CTRL1_RSTW { w: self }
    }
    #[doc = "Bit 4 - SRAM Controller 2 reset control."]
    #[inline]
    pub fn sram_ctrl2_rst(&mut self) -> _SRAM_CTRL2_RSTW {
        _SRAM_CTRL2_RSTW { w: self }
    }
    #[doc = "Bit 5 - SRAM Controller 3 reset control."]
    #[inline]
    pub fn sram_ctrl3_rst(&mut self) -> _SRAM_CTRL3_RSTW {
        _SRAM_CTRL3_RSTW { w: self }
    }
    #[doc = "Bit 6 - SRAM Controller 4 reset control."]
    #[inline]
    pub fn sram_ctrl4_rst(&mut self) -> _SRAM_CTRL4_RSTW {
        _SRAM_CTRL4_RSTW { w: self }
    }
    #[doc = "Bit 7 - Flash controller reset control."]
    #[inline]
    pub fn flash_rst(&mut self) -> _FLASH_RSTW {
        _FLASH_RSTW { w: self }
    }
    #[doc = "Bit 8 - FMC controller reset control."]
    #[inline]
    pub fn fmc_rst(&mut self) -> _FMC_RSTW {
        _FMC_RSTW { w: self }
    }
    #[doc = "Bit 11 - Input Mux 0 reset control."]
    #[inline]
    pub fn mux0_rst(&mut self) -> _MUX0_RSTW {
        _MUX0_RSTW { w: self }
    }
    #[doc = "Bit 13 - I/O controller reset control."]
    #[inline]
    pub fn iocon_rst(&mut self) -> _IOCON_RSTW {
        _IOCON_RSTW { w: self }
    }
    #[doc = "Bit 14 - GPIO0 reset control."]
    #[inline]
    pub fn gpio0_rst(&mut self) -> _GPIO0_RSTW {
        _GPIO0_RSTW { w: self }
    }
    #[doc = "Bit 15 - GPIO1 reset control."]
    #[inline]
    pub fn gpio1_rst(&mut self) -> _GPIO1_RSTW {
        _GPIO1_RSTW { w: self }
    }
    #[doc = "Bit 16 - GPIO2 reset control."]
    #[inline]
    pub fn gpio2_rst(&mut self) -> _GPIO2_RSTW {
        _GPIO2_RSTW { w: self }
    }
    #[doc = "Bit 17 - GPIO3 reset control."]
    #[inline]
    pub fn gpio3_rst(&mut self) -> _GPIO3_RSTW {
        _GPIO3_RSTW { w: self }
    }
    #[doc = "Bit 18 - Pin interrupt (PINT) reset control."]
    #[inline]
    pub fn pint_rst(&mut self) -> _PINT_RSTW {
        _PINT_RSTW { w: self }
    }
    #[doc = "Bit 19 - Group interrupt (GINT) reset control."]
    #[inline]
    pub fn gint_rst(&mut self) -> _GINT_RSTW {
        _GINT_RSTW { w: self }
    }
    #[doc = "Bit 20 - DMA0 reset control."]
    #[inline]
    pub fn dma0_rst(&mut self) -> _DMA0_RSTW {
        _DMA0_RSTW { w: self }
    }
    #[doc = "Bit 21 - CRCGEN reset control."]
    #[inline]
    pub fn crcgen_rst(&mut self) -> _CRCGEN_RSTW {
        _CRCGEN_RSTW { w: self }
    }
    #[doc = "Bit 22 - Watchdog Timer reset control."]
    #[inline]
    pub fn wwdt_rst(&mut self) -> _WWDT_RSTW {
        _WWDT_RSTW { w: self }
    }
    #[doc = "Bit 23 - Real Time Clock (RTC) reset control."]
    #[inline]
    pub fn rtc_rst(&mut self) -> _RTC_RSTW {
        _RTC_RSTW { w: self }
    }
    #[doc = "Bit 26 - Inter CPU communication Mailbox reset control."]
    #[inline]
    pub fn mailbox_rst(&mut self) -> _MAILBOX_RSTW {
        _MAILBOX_RSTW { w: self }
    }
    #[doc = "Bit 27 - ADC reset control."]
    #[inline]
    pub fn adc_rst(&mut self) -> _ADC_RSTW {
        _ADC_RSTW { w: self }
    }
}
