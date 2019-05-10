#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHAREDCTRLSET {
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
#[doc = "Possible values of the field `SHAREDSCKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHAREDSCKSELR {
    #[doc = "SCK for this shared signal set comes from Flexcomm 0."]
    FLEXCOMM0,
    #[doc = "SCK for this shared signal set comes from Flexcomm 1."]
    FLEXCOMM1,
    #[doc = "SCK for this shared signal set comes from Flexcomm 2."]
    FLEXCOMM2,
    #[doc = "SCK for this shared signal set comes from Flexcomm 3."]
    FLEXCOMM3,
    #[doc = "SCK for this shared signal set comes from Flexcomm 4."]
    FLEXCOMM4,
    #[doc = "SCK for this shared signal set comes from Flexcomm 5."]
    FLEXCOMM5,
    #[doc = "SCK for this shared signal set comes from Flexcomm 6."]
    FLEXCOMM6,
    #[doc = "SCK for this shared signal set comes from Flexcomm 7."]
    FLEXCOMM7,
}
impl SHAREDSCKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SHAREDSCKSELR::FLEXCOMM0 => 0,
            SHAREDSCKSELR::FLEXCOMM1 => 1,
            SHAREDSCKSELR::FLEXCOMM2 => 2,
            SHAREDSCKSELR::FLEXCOMM3 => 3,
            SHAREDSCKSELR::FLEXCOMM4 => 4,
            SHAREDSCKSELR::FLEXCOMM5 => 5,
            SHAREDSCKSELR::FLEXCOMM6 => 6,
            SHAREDSCKSELR::FLEXCOMM7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SHAREDSCKSELR {
        match value {
            0 => SHAREDSCKSELR::FLEXCOMM0,
            1 => SHAREDSCKSELR::FLEXCOMM1,
            2 => SHAREDSCKSELR::FLEXCOMM2,
            3 => SHAREDSCKSELR::FLEXCOMM3,
            4 => SHAREDSCKSELR::FLEXCOMM4,
            5 => SHAREDSCKSELR::FLEXCOMM5,
            6 => SHAREDSCKSELR::FLEXCOMM6,
            7 => SHAREDSCKSELR::FLEXCOMM7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM0`"]
    #[inline]
    pub fn is_flexcomm0(&self) -> bool {
        *self == SHAREDSCKSELR::FLEXCOMM0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM1`"]
    #[inline]
    pub fn is_flexcomm1(&self) -> bool {
        *self == SHAREDSCKSELR::FLEXCOMM1
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM2`"]
    #[inline]
    pub fn is_flexcomm2(&self) -> bool {
        *self == SHAREDSCKSELR::FLEXCOMM2
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM3`"]
    #[inline]
    pub fn is_flexcomm3(&self) -> bool {
        *self == SHAREDSCKSELR::FLEXCOMM3
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM4`"]
    #[inline]
    pub fn is_flexcomm4(&self) -> bool {
        *self == SHAREDSCKSELR::FLEXCOMM4
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM5`"]
    #[inline]
    pub fn is_flexcomm5(&self) -> bool {
        *self == SHAREDSCKSELR::FLEXCOMM5
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM6`"]
    #[inline]
    pub fn is_flexcomm6(&self) -> bool {
        *self == SHAREDSCKSELR::FLEXCOMM6
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM7`"]
    #[inline]
    pub fn is_flexcomm7(&self) -> bool {
        *self == SHAREDSCKSELR::FLEXCOMM7
    }
}
#[doc = "Possible values of the field `SHAREDWSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHAREDWSSELR {
    #[doc = "WS for this shared signal set comes from Flexcomm 0."]
    FLEXCOMM0,
    #[doc = "WS for this shared signal set comes from Flexcomm 1."]
    FLEXCOMM1,
    #[doc = "WS for this shared signal set comes from Flexcomm 2."]
    FLEXCOMM2,
    #[doc = "WS for this shared signal set comes from Flexcomm 3."]
    FLEXCOMM3,
    #[doc = "WS for this shared signal set comes from Flexcomm 4."]
    FLEXCOMM4,
    #[doc = "WS for this shared signal set comes from Flexcomm 5."]
    FLEXCOMM5,
    #[doc = "WS for this shared signal set comes from Flexcomm 6."]
    FLEXCOMM6,
    #[doc = "WS for this shared signal set comes from Flexcomm 7."]
    FLEXCOMM7,
}
impl SHAREDWSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SHAREDWSSELR::FLEXCOMM0 => 0,
            SHAREDWSSELR::FLEXCOMM1 => 1,
            SHAREDWSSELR::FLEXCOMM2 => 2,
            SHAREDWSSELR::FLEXCOMM3 => 3,
            SHAREDWSSELR::FLEXCOMM4 => 4,
            SHAREDWSSELR::FLEXCOMM5 => 5,
            SHAREDWSSELR::FLEXCOMM6 => 6,
            SHAREDWSSELR::FLEXCOMM7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SHAREDWSSELR {
        match value {
            0 => SHAREDWSSELR::FLEXCOMM0,
            1 => SHAREDWSSELR::FLEXCOMM1,
            2 => SHAREDWSSELR::FLEXCOMM2,
            3 => SHAREDWSSELR::FLEXCOMM3,
            4 => SHAREDWSSELR::FLEXCOMM4,
            5 => SHAREDWSSELR::FLEXCOMM5,
            6 => SHAREDWSSELR::FLEXCOMM6,
            7 => SHAREDWSSELR::FLEXCOMM7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM0`"]
    #[inline]
    pub fn is_flexcomm0(&self) -> bool {
        *self == SHAREDWSSELR::FLEXCOMM0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM1`"]
    #[inline]
    pub fn is_flexcomm1(&self) -> bool {
        *self == SHAREDWSSELR::FLEXCOMM1
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM2`"]
    #[inline]
    pub fn is_flexcomm2(&self) -> bool {
        *self == SHAREDWSSELR::FLEXCOMM2
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM3`"]
    #[inline]
    pub fn is_flexcomm3(&self) -> bool {
        *self == SHAREDWSSELR::FLEXCOMM3
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM4`"]
    #[inline]
    pub fn is_flexcomm4(&self) -> bool {
        *self == SHAREDWSSELR::FLEXCOMM4
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM5`"]
    #[inline]
    pub fn is_flexcomm5(&self) -> bool {
        *self == SHAREDWSSELR::FLEXCOMM5
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM6`"]
    #[inline]
    pub fn is_flexcomm6(&self) -> bool {
        *self == SHAREDWSSELR::FLEXCOMM6
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM7`"]
    #[inline]
    pub fn is_flexcomm7(&self) -> bool {
        *self == SHAREDWSSELR::FLEXCOMM7
    }
}
#[doc = "Possible values of the field `SHAREDDATASEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHAREDDATASELR {
    #[doc = "DATA input for this shared signal set comes from Flexcomm 0."]
    FLEXCOMM0,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 1."]
    FLEXCOMM1,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 2."]
    FLEXCOMM2,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 3."]
    FLEXCOMM3,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 4."]
    FLEXCOMM4,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 5."]
    FLEXCOMM5,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 6."]
    FLEXCOMM6,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 7."]
    FLEXCOMM7,
}
impl SHAREDDATASELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SHAREDDATASELR::FLEXCOMM0 => 0,
            SHAREDDATASELR::FLEXCOMM1 => 1,
            SHAREDDATASELR::FLEXCOMM2 => 2,
            SHAREDDATASELR::FLEXCOMM3 => 3,
            SHAREDDATASELR::FLEXCOMM4 => 4,
            SHAREDDATASELR::FLEXCOMM5 => 5,
            SHAREDDATASELR::FLEXCOMM6 => 6,
            SHAREDDATASELR::FLEXCOMM7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SHAREDDATASELR {
        match value {
            0 => SHAREDDATASELR::FLEXCOMM0,
            1 => SHAREDDATASELR::FLEXCOMM1,
            2 => SHAREDDATASELR::FLEXCOMM2,
            3 => SHAREDDATASELR::FLEXCOMM3,
            4 => SHAREDDATASELR::FLEXCOMM4,
            5 => SHAREDDATASELR::FLEXCOMM5,
            6 => SHAREDDATASELR::FLEXCOMM6,
            7 => SHAREDDATASELR::FLEXCOMM7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM0`"]
    #[inline]
    pub fn is_flexcomm0(&self) -> bool {
        *self == SHAREDDATASELR::FLEXCOMM0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM1`"]
    #[inline]
    pub fn is_flexcomm1(&self) -> bool {
        *self == SHAREDDATASELR::FLEXCOMM1
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM2`"]
    #[inline]
    pub fn is_flexcomm2(&self) -> bool {
        *self == SHAREDDATASELR::FLEXCOMM2
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM3`"]
    #[inline]
    pub fn is_flexcomm3(&self) -> bool {
        *self == SHAREDDATASELR::FLEXCOMM3
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM4`"]
    #[inline]
    pub fn is_flexcomm4(&self) -> bool {
        *self == SHAREDDATASELR::FLEXCOMM4
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM5`"]
    #[inline]
    pub fn is_flexcomm5(&self) -> bool {
        *self == SHAREDDATASELR::FLEXCOMM5
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM6`"]
    #[inline]
    pub fn is_flexcomm6(&self) -> bool {
        *self == SHAREDDATASELR::FLEXCOMM6
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM7`"]
    #[inline]
    pub fn is_flexcomm7(&self) -> bool {
        *self == SHAREDDATASELR::FLEXCOMM7
    }
}
#[doc = "Possible values of the field `FC0DATAOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC0DATAOUTENR {
    #[doc = "Data output from FC0 does not contribute to this shared set."]
    INPUT,
    #[doc = "Data output from FC0 does contribute to this shared set."]
    OUTPUT,
}
impl FC0DATAOUTENR {
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
            FC0DATAOUTENR::INPUT => false,
            FC0DATAOUTENR::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC0DATAOUTENR {
        match value {
            false => FC0DATAOUTENR::INPUT,
            true => FC0DATAOUTENR::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == FC0DATAOUTENR::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == FC0DATAOUTENR::OUTPUT
    }
}
#[doc = "Possible values of the field `FC1DATAOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC1DATAOUTENR {
    #[doc = "Data output from FC1 does not contribute to this shared set."]
    INPUT,
    #[doc = "Data output from FC1 does contribute to this shared set."]
    OUTPUT,
}
impl FC1DATAOUTENR {
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
            FC1DATAOUTENR::INPUT => false,
            FC1DATAOUTENR::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC1DATAOUTENR {
        match value {
            false => FC1DATAOUTENR::INPUT,
            true => FC1DATAOUTENR::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == FC1DATAOUTENR::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == FC1DATAOUTENR::OUTPUT
    }
}
#[doc = "Possible values of the field `F20DATAOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum F20DATAOUTENR {
    #[doc = "Data output from FC2 does not contribute to this shared set."]
    INPUT,
    #[doc = "Data output from FC2 does contribute to this shared set."]
    OUTPUT,
}
impl F20DATAOUTENR {
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
            F20DATAOUTENR::INPUT => false,
            F20DATAOUTENR::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> F20DATAOUTENR {
        match value {
            false => F20DATAOUTENR::INPUT,
            true => F20DATAOUTENR::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == F20DATAOUTENR::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == F20DATAOUTENR::OUTPUT
    }
}
#[doc = "Possible values of the field `FC3DATAOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC3DATAOUTENR {
    #[doc = "Data output from FC3 does not contribute to this shared set."]
    INPUT,
    #[doc = "Data output from FC3 does contribute to this shared set."]
    OUTPUT,
}
impl FC3DATAOUTENR {
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
            FC3DATAOUTENR::INPUT => false,
            FC3DATAOUTENR::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC3DATAOUTENR {
        match value {
            false => FC3DATAOUTENR::INPUT,
            true => FC3DATAOUTENR::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == FC3DATAOUTENR::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == FC3DATAOUTENR::OUTPUT
    }
}
#[doc = "Possible values of the field `FC4DATAOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC4DATAOUTENR {
    #[doc = "Data output from FC4 does not contribute to this shared set."]
    INPUT,
    #[doc = "Data output from FC4 does contribute to this shared set."]
    OUTPUT,
}
impl FC4DATAOUTENR {
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
            FC4DATAOUTENR::INPUT => false,
            FC4DATAOUTENR::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC4DATAOUTENR {
        match value {
            false => FC4DATAOUTENR::INPUT,
            true => FC4DATAOUTENR::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == FC4DATAOUTENR::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == FC4DATAOUTENR::OUTPUT
    }
}
#[doc = "Possible values of the field `FC5DATAOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC5DATAOUTENR {
    #[doc = "Data output from FC5 does not contribute to this shared set."]
    INPUT,
    #[doc = "Data output from FC5 does contribute to this shared set."]
    OUTPUT,
}
impl FC5DATAOUTENR {
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
            FC5DATAOUTENR::INPUT => false,
            FC5DATAOUTENR::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC5DATAOUTENR {
        match value {
            false => FC5DATAOUTENR::INPUT,
            true => FC5DATAOUTENR::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == FC5DATAOUTENR::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == FC5DATAOUTENR::OUTPUT
    }
}
#[doc = "Possible values of the field `FC6DATAOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC6DATAOUTENR {
    #[doc = "Data output from FC6 does not contribute to this shared set."]
    INPUT,
    #[doc = "Data output from FC6 does contribute to this shared set."]
    OUTPUT,
}
impl FC6DATAOUTENR {
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
            FC6DATAOUTENR::INPUT => false,
            FC6DATAOUTENR::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC6DATAOUTENR {
        match value {
            false => FC6DATAOUTENR::INPUT,
            true => FC6DATAOUTENR::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == FC6DATAOUTENR::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == FC6DATAOUTENR::OUTPUT
    }
}
#[doc = "Possible values of the field `FC7DATAOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC7DATAOUTENR {
    #[doc = "Data output from FC7 does not contribute to this shared set."]
    INPUT,
    #[doc = "Data output from FC7 does contribute to this shared set."]
    OUTPUT,
}
impl FC7DATAOUTENR {
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
            FC7DATAOUTENR::INPUT => false,
            FC7DATAOUTENR::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FC7DATAOUTENR {
        match value {
            false => FC7DATAOUTENR::INPUT,
            true => FC7DATAOUTENR::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == FC7DATAOUTENR::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == FC7DATAOUTENR::OUTPUT
    }
}
#[doc = "Values that can be written to the field `SHAREDSCKSEL`"]
pub enum SHAREDSCKSELW {
    #[doc = "SCK for this shared signal set comes from Flexcomm 0."]
    FLEXCOMM0,
    #[doc = "SCK for this shared signal set comes from Flexcomm 1."]
    FLEXCOMM1,
    #[doc = "SCK for this shared signal set comes from Flexcomm 2."]
    FLEXCOMM2,
    #[doc = "SCK for this shared signal set comes from Flexcomm 3."]
    FLEXCOMM3,
    #[doc = "SCK for this shared signal set comes from Flexcomm 4."]
    FLEXCOMM4,
    #[doc = "SCK for this shared signal set comes from Flexcomm 5."]
    FLEXCOMM5,
    #[doc = "SCK for this shared signal set comes from Flexcomm 6."]
    FLEXCOMM6,
    #[doc = "SCK for this shared signal set comes from Flexcomm 7."]
    FLEXCOMM7,
}
impl SHAREDSCKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SHAREDSCKSELW::FLEXCOMM0 => 0,
            SHAREDSCKSELW::FLEXCOMM1 => 1,
            SHAREDSCKSELW::FLEXCOMM2 => 2,
            SHAREDSCKSELW::FLEXCOMM3 => 3,
            SHAREDSCKSELW::FLEXCOMM4 => 4,
            SHAREDSCKSELW::FLEXCOMM5 => 5,
            SHAREDSCKSELW::FLEXCOMM6 => 6,
            SHAREDSCKSELW::FLEXCOMM7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SHAREDSCKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAREDSCKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SHAREDSCKSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 0."]
    #[inline]
    pub fn flexcomm0(self) -> &'a mut W {
        self.variant(SHAREDSCKSELW::FLEXCOMM0)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 1."]
    #[inline]
    pub fn flexcomm1(self) -> &'a mut W {
        self.variant(SHAREDSCKSELW::FLEXCOMM1)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 2."]
    #[inline]
    pub fn flexcomm2(self) -> &'a mut W {
        self.variant(SHAREDSCKSELW::FLEXCOMM2)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 3."]
    #[inline]
    pub fn flexcomm3(self) -> &'a mut W {
        self.variant(SHAREDSCKSELW::FLEXCOMM3)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 4."]
    #[inline]
    pub fn flexcomm4(self) -> &'a mut W {
        self.variant(SHAREDSCKSELW::FLEXCOMM4)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 5."]
    #[inline]
    pub fn flexcomm5(self) -> &'a mut W {
        self.variant(SHAREDSCKSELW::FLEXCOMM5)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 6."]
    #[inline]
    pub fn flexcomm6(self) -> &'a mut W {
        self.variant(SHAREDSCKSELW::FLEXCOMM6)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 7."]
    #[inline]
    pub fn flexcomm7(self) -> &'a mut W {
        self.variant(SHAREDSCKSELW::FLEXCOMM7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SHAREDWSSEL`"]
pub enum SHAREDWSSELW {
    #[doc = "WS for this shared signal set comes from Flexcomm 0."]
    FLEXCOMM0,
    #[doc = "WS for this shared signal set comes from Flexcomm 1."]
    FLEXCOMM1,
    #[doc = "WS for this shared signal set comes from Flexcomm 2."]
    FLEXCOMM2,
    #[doc = "WS for this shared signal set comes from Flexcomm 3."]
    FLEXCOMM3,
    #[doc = "WS for this shared signal set comes from Flexcomm 4."]
    FLEXCOMM4,
    #[doc = "WS for this shared signal set comes from Flexcomm 5."]
    FLEXCOMM5,
    #[doc = "WS for this shared signal set comes from Flexcomm 6."]
    FLEXCOMM6,
    #[doc = "WS for this shared signal set comes from Flexcomm 7."]
    FLEXCOMM7,
}
impl SHAREDWSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SHAREDWSSELW::FLEXCOMM0 => 0,
            SHAREDWSSELW::FLEXCOMM1 => 1,
            SHAREDWSSELW::FLEXCOMM2 => 2,
            SHAREDWSSELW::FLEXCOMM3 => 3,
            SHAREDWSSELW::FLEXCOMM4 => 4,
            SHAREDWSSELW::FLEXCOMM5 => 5,
            SHAREDWSSELW::FLEXCOMM6 => 6,
            SHAREDWSSELW::FLEXCOMM7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SHAREDWSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAREDWSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SHAREDWSSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 0."]
    #[inline]
    pub fn flexcomm0(self) -> &'a mut W {
        self.variant(SHAREDWSSELW::FLEXCOMM0)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 1."]
    #[inline]
    pub fn flexcomm1(self) -> &'a mut W {
        self.variant(SHAREDWSSELW::FLEXCOMM1)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 2."]
    #[inline]
    pub fn flexcomm2(self) -> &'a mut W {
        self.variant(SHAREDWSSELW::FLEXCOMM2)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 3."]
    #[inline]
    pub fn flexcomm3(self) -> &'a mut W {
        self.variant(SHAREDWSSELW::FLEXCOMM3)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 4."]
    #[inline]
    pub fn flexcomm4(self) -> &'a mut W {
        self.variant(SHAREDWSSELW::FLEXCOMM4)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 5."]
    #[inline]
    pub fn flexcomm5(self) -> &'a mut W {
        self.variant(SHAREDWSSELW::FLEXCOMM5)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 6."]
    #[inline]
    pub fn flexcomm6(self) -> &'a mut W {
        self.variant(SHAREDWSSELW::FLEXCOMM6)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 7."]
    #[inline]
    pub fn flexcomm7(self) -> &'a mut W {
        self.variant(SHAREDWSSELW::FLEXCOMM7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SHAREDDATASEL`"]
pub enum SHAREDDATASELW {
    #[doc = "DATA input for this shared signal set comes from Flexcomm 0."]
    FLEXCOMM0,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 1."]
    FLEXCOMM1,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 2."]
    FLEXCOMM2,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 3."]
    FLEXCOMM3,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 4."]
    FLEXCOMM4,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 5."]
    FLEXCOMM5,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 6."]
    FLEXCOMM6,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 7."]
    FLEXCOMM7,
}
impl SHAREDDATASELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SHAREDDATASELW::FLEXCOMM0 => 0,
            SHAREDDATASELW::FLEXCOMM1 => 1,
            SHAREDDATASELW::FLEXCOMM2 => 2,
            SHAREDDATASELW::FLEXCOMM3 => 3,
            SHAREDDATASELW::FLEXCOMM4 => 4,
            SHAREDDATASELW::FLEXCOMM5 => 5,
            SHAREDDATASELW::FLEXCOMM6 => 6,
            SHAREDDATASELW::FLEXCOMM7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SHAREDDATASELW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAREDDATASELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SHAREDDATASELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 0."]
    #[inline]
    pub fn flexcomm0(self) -> &'a mut W {
        self.variant(SHAREDDATASELW::FLEXCOMM0)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 1."]
    #[inline]
    pub fn flexcomm1(self) -> &'a mut W {
        self.variant(SHAREDDATASELW::FLEXCOMM1)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 2."]
    #[inline]
    pub fn flexcomm2(self) -> &'a mut W {
        self.variant(SHAREDDATASELW::FLEXCOMM2)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 3."]
    #[inline]
    pub fn flexcomm3(self) -> &'a mut W {
        self.variant(SHAREDDATASELW::FLEXCOMM3)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 4."]
    #[inline]
    pub fn flexcomm4(self) -> &'a mut W {
        self.variant(SHAREDDATASELW::FLEXCOMM4)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 5."]
    #[inline]
    pub fn flexcomm5(self) -> &'a mut W {
        self.variant(SHAREDDATASELW::FLEXCOMM5)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 6."]
    #[inline]
    pub fn flexcomm6(self) -> &'a mut W {
        self.variant(SHAREDDATASELW::FLEXCOMM6)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 7."]
    #[inline]
    pub fn flexcomm7(self) -> &'a mut W {
        self.variant(SHAREDDATASELW::FLEXCOMM7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FC0DATAOUTEN`"]
pub enum FC0DATAOUTENW {
    #[doc = "Data output from FC0 does not contribute to this shared set."]
    INPUT,
    #[doc = "Data output from FC0 does contribute to this shared set."]
    OUTPUT,
}
impl FC0DATAOUTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC0DATAOUTENW::INPUT => false,
            FC0DATAOUTENW::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC0DATAOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _FC0DATAOUTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC0DATAOUTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data output from FC0 does not contribute to this shared set."]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(FC0DATAOUTENW::INPUT)
    }
    #[doc = "Data output from FC0 does contribute to this shared set."]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(FC0DATAOUTENW::OUTPUT)
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
#[doc = "Values that can be written to the field `FC1DATAOUTEN`"]
pub enum FC1DATAOUTENW {
    #[doc = "Data output from FC1 does not contribute to this shared set."]
    INPUT,
    #[doc = "Data output from FC1 does contribute to this shared set."]
    OUTPUT,
}
impl FC1DATAOUTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC1DATAOUTENW::INPUT => false,
            FC1DATAOUTENW::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC1DATAOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _FC1DATAOUTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC1DATAOUTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data output from FC1 does not contribute to this shared set."]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(FC1DATAOUTENW::INPUT)
    }
    #[doc = "Data output from FC1 does contribute to this shared set."]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(FC1DATAOUTENW::OUTPUT)
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
#[doc = "Values that can be written to the field `F20DATAOUTEN`"]
pub enum F20DATAOUTENW {
    #[doc = "Data output from FC2 does not contribute to this shared set."]
    INPUT,
    #[doc = "Data output from FC2 does contribute to this shared set."]
    OUTPUT,
}
impl F20DATAOUTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            F20DATAOUTENW::INPUT => false,
            F20DATAOUTENW::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _F20DATAOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _F20DATAOUTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: F20DATAOUTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data output from FC2 does not contribute to this shared set."]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(F20DATAOUTENW::INPUT)
    }
    #[doc = "Data output from FC2 does contribute to this shared set."]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(F20DATAOUTENW::OUTPUT)
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
#[doc = "Values that can be written to the field `FC3DATAOUTEN`"]
pub enum FC3DATAOUTENW {
    #[doc = "Data output from FC3 does not contribute to this shared set."]
    INPUT,
    #[doc = "Data output from FC3 does contribute to this shared set."]
    OUTPUT,
}
impl FC3DATAOUTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC3DATAOUTENW::INPUT => false,
            FC3DATAOUTENW::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC3DATAOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _FC3DATAOUTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC3DATAOUTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data output from FC3 does not contribute to this shared set."]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(FC3DATAOUTENW::INPUT)
    }
    #[doc = "Data output from FC3 does contribute to this shared set."]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(FC3DATAOUTENW::OUTPUT)
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
#[doc = "Values that can be written to the field `FC4DATAOUTEN`"]
pub enum FC4DATAOUTENW {
    #[doc = "Data output from FC4 does not contribute to this shared set."]
    INPUT,
    #[doc = "Data output from FC4 does contribute to this shared set."]
    OUTPUT,
}
impl FC4DATAOUTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC4DATAOUTENW::INPUT => false,
            FC4DATAOUTENW::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC4DATAOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _FC4DATAOUTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC4DATAOUTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data output from FC4 does not contribute to this shared set."]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(FC4DATAOUTENW::INPUT)
    }
    #[doc = "Data output from FC4 does contribute to this shared set."]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(FC4DATAOUTENW::OUTPUT)
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
#[doc = "Values that can be written to the field `FC5DATAOUTEN`"]
pub enum FC5DATAOUTENW {
    #[doc = "Data output from FC5 does not contribute to this shared set."]
    INPUT,
    #[doc = "Data output from FC5 does contribute to this shared set."]
    OUTPUT,
}
impl FC5DATAOUTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC5DATAOUTENW::INPUT => false,
            FC5DATAOUTENW::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC5DATAOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _FC5DATAOUTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC5DATAOUTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data output from FC5 does not contribute to this shared set."]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(FC5DATAOUTENW::INPUT)
    }
    #[doc = "Data output from FC5 does contribute to this shared set."]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(FC5DATAOUTENW::OUTPUT)
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
#[doc = "Values that can be written to the field `FC6DATAOUTEN`"]
pub enum FC6DATAOUTENW {
    #[doc = "Data output from FC6 does not contribute to this shared set."]
    INPUT,
    #[doc = "Data output from FC6 does contribute to this shared set."]
    OUTPUT,
}
impl FC6DATAOUTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC6DATAOUTENW::INPUT => false,
            FC6DATAOUTENW::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC6DATAOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _FC6DATAOUTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC6DATAOUTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data output from FC6 does not contribute to this shared set."]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(FC6DATAOUTENW::INPUT)
    }
    #[doc = "Data output from FC6 does contribute to this shared set."]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(FC6DATAOUTENW::OUTPUT)
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
#[doc = "Values that can be written to the field `FC7DATAOUTEN`"]
pub enum FC7DATAOUTENW {
    #[doc = "Data output from FC7 does not contribute to this shared set."]
    INPUT,
    #[doc = "Data output from FC7 does contribute to this shared set."]
    OUTPUT,
}
impl FC7DATAOUTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FC7DATAOUTENW::INPUT => false,
            FC7DATAOUTENW::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FC7DATAOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _FC7DATAOUTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC7DATAOUTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data output from FC7 does not contribute to this shared set."]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(FC7DATAOUTENW::INPUT)
    }
    #[doc = "Data output from FC7 does contribute to this shared set."]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(FC7DATAOUTENW::OUTPUT)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Selects the source for SCK of this shared signal set."]
    #[inline]
    pub fn sharedscksel(&self) -> SHAREDSCKSELR {
        SHAREDSCKSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Selects the source for WS of this shared signal set."]
    #[inline]
    pub fn sharedwssel(&self) -> SHAREDWSSELR {
        SHAREDWSSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - Selects the source for DATA input for this shared signal set."]
    #[inline]
    pub fn shareddatasel(&self) -> SHAREDDATASELR {
        SHAREDDATASELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Controls FC0 contribution to SHAREDDATAOUT for this shared set."]
    #[inline]
    pub fn fc0dataouten(&self) -> FC0DATAOUTENR {
        FC0DATAOUTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Controls FC1 contribution to SHAREDDATAOUT for this shared set."]
    #[inline]
    pub fn fc1dataouten(&self) -> FC1DATAOUTENR {
        FC1DATAOUTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Controls FC2 contribution to SHAREDDATAOUT for this shared set."]
    #[inline]
    pub fn f20dataouten(&self) -> F20DATAOUTENR {
        F20DATAOUTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Controls FC3 contribution to SHAREDDATAOUT for this shared set."]
    #[inline]
    pub fn fc3dataouten(&self) -> FC3DATAOUTENR {
        FC3DATAOUTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Controls FC4 contribution to SHAREDDATAOUT for this shared set."]
    #[inline]
    pub fn fc4dataouten(&self) -> FC4DATAOUTENR {
        FC4DATAOUTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Controls FC5 contribution to SHAREDDATAOUT for this shared set."]
    #[inline]
    pub fn fc5dataouten(&self) -> FC5DATAOUTENR {
        FC5DATAOUTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Controls FC6 contribution to SHAREDDATAOUT for this shared set."]
    #[inline]
    pub fn fc6dataouten(&self) -> FC6DATAOUTENR {
        FC6DATAOUTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Controls FC7 contribution to SHAREDDATAOUT for this shared set."]
    #[inline]
    pub fn fc7dataouten(&self) -> FC7DATAOUTENR {
        FC7DATAOUTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
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
    #[doc = "Bits 0:2 - Selects the source for SCK of this shared signal set."]
    #[inline]
    pub fn sharedscksel(&mut self) -> _SHAREDSCKSELW {
        _SHAREDSCKSELW { w: self }
    }
    #[doc = "Bits 4:6 - Selects the source for WS of this shared signal set."]
    #[inline]
    pub fn sharedwssel(&mut self) -> _SHAREDWSSELW {
        _SHAREDWSSELW { w: self }
    }
    #[doc = "Bits 8:10 - Selects the source for DATA input for this shared signal set."]
    #[inline]
    pub fn shareddatasel(&mut self) -> _SHAREDDATASELW {
        _SHAREDDATASELW { w: self }
    }
    #[doc = "Bit 16 - Controls FC0 contribution to SHAREDDATAOUT for this shared set."]
    #[inline]
    pub fn fc0dataouten(&mut self) -> _FC0DATAOUTENW {
        _FC0DATAOUTENW { w: self }
    }
    #[doc = "Bit 17 - Controls FC1 contribution to SHAREDDATAOUT for this shared set."]
    #[inline]
    pub fn fc1dataouten(&mut self) -> _FC1DATAOUTENW {
        _FC1DATAOUTENW { w: self }
    }
    #[doc = "Bit 18 - Controls FC2 contribution to SHAREDDATAOUT for this shared set."]
    #[inline]
    pub fn f20dataouten(&mut self) -> _F20DATAOUTENW {
        _F20DATAOUTENW { w: self }
    }
    #[doc = "Bit 19 - Controls FC3 contribution to SHAREDDATAOUT for this shared set."]
    #[inline]
    pub fn fc3dataouten(&mut self) -> _FC3DATAOUTENW {
        _FC3DATAOUTENW { w: self }
    }
    #[doc = "Bit 20 - Controls FC4 contribution to SHAREDDATAOUT for this shared set."]
    #[inline]
    pub fn fc4dataouten(&mut self) -> _FC4DATAOUTENW {
        _FC4DATAOUTENW { w: self }
    }
    #[doc = "Bit 21 - Controls FC5 contribution to SHAREDDATAOUT for this shared set."]
    #[inline]
    pub fn fc5dataouten(&mut self) -> _FC5DATAOUTENW {
        _FC5DATAOUTENW { w: self }
    }
    #[doc = "Bit 22 - Controls FC6 contribution to SHAREDDATAOUT for this shared set."]
    #[inline]
    pub fn fc6dataouten(&mut self) -> _FC6DATAOUTENW {
        _FC6DATAOUTENW { w: self }
    }
    #[doc = "Bit 23 - Controls FC7 contribution to SHAREDDATAOUT for this shared set."]
    #[inline]
    pub fn fc7dataouten(&mut self) -> _FC7DATAOUTENW {
        _FC7DATAOUTENW { w: self }
    }
}
