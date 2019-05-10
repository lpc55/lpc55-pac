#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDRUNCFG0 {
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
#[doc = "Possible values of the field `PDEN_DCDC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_DCDCR {
    #[doc = "DCDC is powered."]
    POWEREDON,
    #[doc = "DCDC is powered down."]
    POWEREDOFF,
}
impl PDEN_DCDCR {
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
            PDEN_DCDCR::POWEREDON => false,
            PDEN_DCDCR::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_DCDCR {
        match value {
            false => PDEN_DCDCR::POWEREDON,
            true => PDEN_DCDCR::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_DCDCR::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_DCDCR::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_BIAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_BIASR {
    #[doc = "Analog Bias is powered."]
    POWEREDON,
    #[doc = "Analog Bias is powered down."]
    POWEREDOFF,
}
impl PDEN_BIASR {
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
            PDEN_BIASR::POWEREDON => false,
            PDEN_BIASR::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_BIASR {
        match value {
            false => PDEN_BIASR::POWEREDON,
            true => PDEN_BIASR::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_BIASR::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_BIASR::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_BODCORE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_BODCORER {
    #[doc = "BOD CORE is powered."]
    POWEREDON,
    #[doc = "BOD CORE is powered down."]
    POWEREDOFF,
}
impl PDEN_BODCORER {
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
            PDEN_BODCORER::POWEREDON => false,
            PDEN_BODCORER::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_BODCORER {
        match value {
            false => PDEN_BODCORER::POWEREDON,
            true => PDEN_BODCORER::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_BODCORER::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_BODCORER::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_BODVBAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_BODVBATR {
    #[doc = "BOD VBAT is powered."]
    POWEREDON,
    #[doc = "BOD VBAT is powered down."]
    POWEREDOFF,
}
impl PDEN_BODVBATR {
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
            PDEN_BODVBATR::POWEREDON => false,
            PDEN_BODVBATR::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_BODVBATR {
        match value {
            false => PDEN_BODVBATR::POWEREDON,
            true => PDEN_BODVBATR::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_BODVBATR::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_BODVBATR::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_FRO192M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_FRO192MR {
    #[doc = "FRO 192MHz is powered."]
    POWEREDON,
    #[doc = "FRO 192MHz is powered down."]
    POWEREDOFF,
}
impl PDEN_FRO192MR {
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
            PDEN_FRO192MR::POWEREDON => false,
            PDEN_FRO192MR::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_FRO192MR {
        match value {
            false => PDEN_FRO192MR::POWEREDON,
            true => PDEN_FRO192MR::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_FRO192MR::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_FRO192MR::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_FRO32K`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_FRO32KR {
    #[doc = "FRO32KHz is powered."]
    POWEREDON,
    #[doc = "FRO32KHz is powered down."]
    POWEREDOFF,
}
impl PDEN_FRO32KR {
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
            PDEN_FRO32KR::POWEREDON => false,
            PDEN_FRO32KR::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_FRO32KR {
        match value {
            false => PDEN_FRO32KR::POWEREDON,
            true => PDEN_FRO32KR::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_FRO32KR::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_FRO32KR::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_XTAL32K`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_XTAL32KR {
    #[doc = "Crystal 32KHz is powered."]
    POWEREDON,
    #[doc = "Crystal 32KHz is powered down."]
    POWEREDOFF,
}
impl PDEN_XTAL32KR {
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
            PDEN_XTAL32KR::POWEREDON => false,
            PDEN_XTAL32KR::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_XTAL32KR {
        match value {
            false => PDEN_XTAL32KR::POWEREDON,
            true => PDEN_XTAL32KR::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_XTAL32KR::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_XTAL32KR::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_XTAL32M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_XTAL32MR {
    #[doc = "Crystal 32MHz is powered."]
    POWEREDON,
    #[doc = "Crystal 32MHz is powered down."]
    POWEREDOFF,
}
impl PDEN_XTAL32MR {
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
            PDEN_XTAL32MR::POWEREDON => false,
            PDEN_XTAL32MR::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_XTAL32MR {
        match value {
            false => PDEN_XTAL32MR::POWEREDON,
            true => PDEN_XTAL32MR::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_XTAL32MR::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_XTAL32MR::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_PLL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_PLL0R {
    #[doc = "PLL0 is powered."]
    POWEREDON,
    #[doc = "PLL0 is powered down."]
    POWEREDOFF,
}
impl PDEN_PLL0R {
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
            PDEN_PLL0R::POWEREDON => false,
            PDEN_PLL0R::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_PLL0R {
        match value {
            false => PDEN_PLL0R::POWEREDON,
            true => PDEN_PLL0R::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_PLL0R::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_PLL0R::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_PLL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_PLL1R {
    #[doc = "PLL1 is powered."]
    POWEREDON,
    #[doc = "PLL1 is powered down."]
    POWEREDOFF,
}
impl PDEN_PLL1R {
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
            PDEN_PLL1R::POWEREDON => false,
            PDEN_PLL1R::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_PLL1R {
        match value {
            false => PDEN_PLL1R::POWEREDON,
            true => PDEN_PLL1R::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_PLL1R::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_PLL1R::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_USBFSPHY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_USBFSPHYR {
    #[doc = "USB Full Speed phy is powered."]
    POWEREDON,
    #[doc = "USB Full Speed phy is powered down."]
    POWEREDOFF,
}
impl PDEN_USBFSPHYR {
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
            PDEN_USBFSPHYR::POWEREDON => false,
            PDEN_USBFSPHYR::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_USBFSPHYR {
        match value {
            false => PDEN_USBFSPHYR::POWEREDON,
            true => PDEN_USBFSPHYR::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_USBFSPHYR::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_USBFSPHYR::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_USBHSPHY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_USBHSPHYR {
    #[doc = "USB HS phy is powered."]
    POWEREDON,
    #[doc = "USB HS phy is powered down."]
    POWEREDOFF,
}
impl PDEN_USBHSPHYR {
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
            PDEN_USBHSPHYR::POWEREDON => false,
            PDEN_USBHSPHYR::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_USBHSPHYR {
        match value {
            false => PDEN_USBHSPHYR::POWEREDON,
            true => PDEN_USBHSPHYR::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_USBHSPHYR::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_USBHSPHYR::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_COMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_COMPR {
    #[doc = "Analog Comparator is powered."]
    POWEREDON,
    #[doc = "Analog Comparator is powered down."]
    POWEREDOFF,
}
impl PDEN_COMPR {
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
            PDEN_COMPR::POWEREDON => false,
            PDEN_COMPR::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_COMPR {
        match value {
            false => PDEN_COMPR::POWEREDON,
            true => PDEN_COMPR::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_COMPR::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_COMPR::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_TEMPSENS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_TEMPSENSR {
    #[doc = "Temperature Sensor is powered."]
    POWEREDON,
    #[doc = "Temperature Sensor is powered down."]
    POWEREDOFF,
}
impl PDEN_TEMPSENSR {
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
            PDEN_TEMPSENSR::POWEREDON => false,
            PDEN_TEMPSENSR::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_TEMPSENSR {
        match value {
            false => PDEN_TEMPSENSR::POWEREDON,
            true => PDEN_TEMPSENSR::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_TEMPSENSR::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_TEMPSENSR::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_GPADC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_GPADCR {
    #[doc = "GPADC is powered."]
    POWEREDON,
    #[doc = "GPADC is powered down."]
    POWEREDOFF,
}
impl PDEN_GPADCR {
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
            PDEN_GPADCR::POWEREDON => false,
            PDEN_GPADCR::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_GPADCR {
        match value {
            false => PDEN_GPADCR::POWEREDON,
            true => PDEN_GPADCR::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_GPADCR::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_GPADCR::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_LDOMEM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_LDOMEMR {
    #[doc = "Memories LDO is powered."]
    POWEREDON,
    #[doc = "Memories LDO is powered down."]
    POWEREDOFF,
}
impl PDEN_LDOMEMR {
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
            PDEN_LDOMEMR::POWEREDON => false,
            PDEN_LDOMEMR::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_LDOMEMR {
        match value {
            false => PDEN_LDOMEMR::POWEREDON,
            true => PDEN_LDOMEMR::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_LDOMEMR::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_LDOMEMR::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_LDODEEPSLEEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_LDODEEPSLEEPR {
    #[doc = "Deep Sleep LDO is powered."]
    POWEREDON,
    #[doc = "Deep Sleep LDO is powered down."]
    POWEREDOFF,
}
impl PDEN_LDODEEPSLEEPR {
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
            PDEN_LDODEEPSLEEPR::POWEREDON => false,
            PDEN_LDODEEPSLEEPR::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_LDODEEPSLEEPR {
        match value {
            false => PDEN_LDODEEPSLEEPR::POWEREDON,
            true => PDEN_LDODEEPSLEEPR::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_LDODEEPSLEEPR::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_LDODEEPSLEEPR::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_LDOUSBHS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_LDOUSBHSR {
    #[doc = "USB high speed LDO is powered."]
    POWEREDON,
    #[doc = "USB high speed LDO is powered down."]
    POWEREDOFF,
}
impl PDEN_LDOUSBHSR {
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
            PDEN_LDOUSBHSR::POWEREDON => false,
            PDEN_LDOUSBHSR::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_LDOUSBHSR {
        match value {
            false => PDEN_LDOUSBHSR::POWEREDON,
            true => PDEN_LDOUSBHSR::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_LDOUSBHSR::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_LDOUSBHSR::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_AUXBIAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_AUXBIASR {
    #[doc = "auxiliary biasing is powered."]
    POWEREDON,
    #[doc = "auxiliary biasing is powered down."]
    POWEREDOFF,
}
impl PDEN_AUXBIASR {
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
            PDEN_AUXBIASR::POWEREDON => false,
            PDEN_AUXBIASR::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_AUXBIASR {
        match value {
            false => PDEN_AUXBIASR::POWEREDON,
            true => PDEN_AUXBIASR::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_AUXBIASR::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_AUXBIASR::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_LDOXO32M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_LDOXO32MR {
    #[doc = "crystal 32 MHz LDO is powered."]
    POWEREDON,
    #[doc = "crystal 32 MHz LDO is powered down."]
    POWEREDOFF,
}
impl PDEN_LDOXO32MR {
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
            PDEN_LDOXO32MR::POWEREDON => false,
            PDEN_LDOXO32MR::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_LDOXO32MR {
        match value {
            false => PDEN_LDOXO32MR::POWEREDON,
            true => PDEN_LDOXO32MR::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_LDOXO32MR::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_LDOXO32MR::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_LDOFLASHNV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_LDOFLASHNVR {
    #[doc = "Flash NV LDO is powered."]
    POWEREDON,
    #[doc = "Flash NV LDO is powered down."]
    POWEREDOFF,
}
impl PDEN_LDOFLASHNVR {
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
            PDEN_LDOFLASHNVR::POWEREDON => false,
            PDEN_LDOFLASHNVR::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_LDOFLASHNVR {
        match value {
            false => PDEN_LDOFLASHNVR::POWEREDON,
            true => PDEN_LDOFLASHNVR::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_LDOFLASHNVR::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_LDOFLASHNVR::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_RNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_RNGR {
    #[doc = "TRNG clocks are powered."]
    POWEREDON,
    #[doc = "TRNG clocks are powered down."]
    POWEREDOFF,
}
impl PDEN_RNGR {
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
            PDEN_RNGR::POWEREDON => false,
            PDEN_RNGR::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_RNGR {
        match value {
            false => PDEN_RNGR::POWEREDON,
            true => PDEN_RNGR::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_RNGR::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_RNGR::POWEREDOFF
    }
}
#[doc = "Possible values of the field `PDEN_PLL0_SSCG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_PLL0_SSCGR {
    #[doc = "PLL0 Sread spectrum module is powered."]
    POWEREDON,
    #[doc = "PLL0 Sread spectrum module is powered down."]
    POWEREDOFF,
}
impl PDEN_PLL0_SSCGR {
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
            PDEN_PLL0_SSCGR::POWEREDON => false,
            PDEN_PLL0_SSCGR::POWEREDOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDEN_PLL0_SSCGR {
        match value {
            false => PDEN_PLL0_SSCGR::POWEREDON,
            true => PDEN_PLL0_SSCGR::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_PLL0_SSCGR::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_PLL0_SSCGR::POWEREDOFF
    }
}
#[doc = "Values that can be written to the field `PDEN_DCDC`"]
pub enum PDEN_DCDCW {
    #[doc = "DCDC is powered."]
    POWEREDON,
    #[doc = "DCDC is powered down."]
    POWEREDOFF,
}
impl PDEN_DCDCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_DCDCW::POWEREDON => false,
            PDEN_DCDCW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_DCDCW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_DCDCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_DCDCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DCDC is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_DCDCW::POWEREDON)
    }
    #[doc = "DCDC is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_DCDCW::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_BIAS`"]
pub enum PDEN_BIASW {
    #[doc = "Analog Bias is powered."]
    POWEREDON,
    #[doc = "Analog Bias is powered down."]
    POWEREDOFF,
}
impl PDEN_BIASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_BIASW::POWEREDON => false,
            PDEN_BIASW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_BIASW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_BIASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_BIASW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Analog Bias is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_BIASW::POWEREDON)
    }
    #[doc = "Analog Bias is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_BIASW::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_BODCORE`"]
pub enum PDEN_BODCOREW {
    #[doc = "BOD CORE is powered."]
    POWEREDON,
    #[doc = "BOD CORE is powered down."]
    POWEREDOFF,
}
impl PDEN_BODCOREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_BODCOREW::POWEREDON => false,
            PDEN_BODCOREW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_BODCOREW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_BODCOREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_BODCOREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "BOD CORE is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_BODCOREW::POWEREDON)
    }
    #[doc = "BOD CORE is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_BODCOREW::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_BODVBAT`"]
pub enum PDEN_BODVBATW {
    #[doc = "BOD VBAT is powered."]
    POWEREDON,
    #[doc = "BOD VBAT is powered down."]
    POWEREDOFF,
}
impl PDEN_BODVBATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_BODVBATW::POWEREDON => false,
            PDEN_BODVBATW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_BODVBATW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_BODVBATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_BODVBATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "BOD VBAT is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_BODVBATW::POWEREDON)
    }
    #[doc = "BOD VBAT is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_BODVBATW::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_FRO192M`"]
pub enum PDEN_FRO192MW {
    #[doc = "FRO 192MHz is powered."]
    POWEREDON,
    #[doc = "FRO 192MHz is powered down."]
    POWEREDOFF,
}
impl PDEN_FRO192MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_FRO192MW::POWEREDON => false,
            PDEN_FRO192MW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_FRO192MW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_FRO192MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_FRO192MW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FRO 192MHz is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_FRO192MW::POWEREDON)
    }
    #[doc = "FRO 192MHz is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_FRO192MW::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_FRO32K`"]
pub enum PDEN_FRO32KW {
    #[doc = "FRO32KHz is powered."]
    POWEREDON,
    #[doc = "FRO32KHz is powered down."]
    POWEREDOFF,
}
impl PDEN_FRO32KW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_FRO32KW::POWEREDON => false,
            PDEN_FRO32KW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_FRO32KW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_FRO32KW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_FRO32KW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FRO32KHz is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_FRO32KW::POWEREDON)
    }
    #[doc = "FRO32KHz is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_FRO32KW::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_XTAL32K`"]
pub enum PDEN_XTAL32KW {
    #[doc = "Crystal 32KHz is powered."]
    POWEREDON,
    #[doc = "Crystal 32KHz is powered down."]
    POWEREDOFF,
}
impl PDEN_XTAL32KW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_XTAL32KW::POWEREDON => false,
            PDEN_XTAL32KW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_XTAL32KW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_XTAL32KW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_XTAL32KW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Crystal 32KHz is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_XTAL32KW::POWEREDON)
    }
    #[doc = "Crystal 32KHz is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_XTAL32KW::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_XTAL32M`"]
pub enum PDEN_XTAL32MW {
    #[doc = "Crystal 32MHz is powered."]
    POWEREDON,
    #[doc = "Crystal 32MHz is powered down."]
    POWEREDOFF,
}
impl PDEN_XTAL32MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_XTAL32MW::POWEREDON => false,
            PDEN_XTAL32MW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_XTAL32MW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_XTAL32MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_XTAL32MW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Crystal 32MHz is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_XTAL32MW::POWEREDON)
    }
    #[doc = "Crystal 32MHz is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_XTAL32MW::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_PLL0`"]
pub enum PDEN_PLL0W {
    #[doc = "PLL0 is powered."]
    POWEREDON,
    #[doc = "PLL0 is powered down."]
    POWEREDOFF,
}
impl PDEN_PLL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_PLL0W::POWEREDON => false,
            PDEN_PLL0W::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_PLL0W<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_PLL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_PLL0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PLL0 is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_PLL0W::POWEREDON)
    }
    #[doc = "PLL0 is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_PLL0W::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_PLL1`"]
pub enum PDEN_PLL1W {
    #[doc = "PLL1 is powered."]
    POWEREDON,
    #[doc = "PLL1 is powered down."]
    POWEREDOFF,
}
impl PDEN_PLL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_PLL1W::POWEREDON => false,
            PDEN_PLL1W::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_PLL1W<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_PLL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_PLL1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PLL1 is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_PLL1W::POWEREDON)
    }
    #[doc = "PLL1 is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_PLL1W::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_USBFSPHY`"]
pub enum PDEN_USBFSPHYW {
    #[doc = "USB Full Speed phy is powered."]
    POWEREDON,
    #[doc = "USB Full Speed phy is powered down."]
    POWEREDOFF,
}
impl PDEN_USBFSPHYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_USBFSPHYW::POWEREDON => false,
            PDEN_USBFSPHYW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_USBFSPHYW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_USBFSPHYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_USBFSPHYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "USB Full Speed phy is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_USBFSPHYW::POWEREDON)
    }
    #[doc = "USB Full Speed phy is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_USBFSPHYW::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_USBHSPHY`"]
pub enum PDEN_USBHSPHYW {
    #[doc = "USB HS phy is powered."]
    POWEREDON,
    #[doc = "USB HS phy is powered down."]
    POWEREDOFF,
}
impl PDEN_USBHSPHYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_USBHSPHYW::POWEREDON => false,
            PDEN_USBHSPHYW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_USBHSPHYW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_USBHSPHYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_USBHSPHYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "USB HS phy is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_USBHSPHYW::POWEREDON)
    }
    #[doc = "USB HS phy is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_USBHSPHYW::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_COMP`"]
pub enum PDEN_COMPW {
    #[doc = "Analog Comparator is powered."]
    POWEREDON,
    #[doc = "Analog Comparator is powered down."]
    POWEREDOFF,
}
impl PDEN_COMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_COMPW::POWEREDON => false,
            PDEN_COMPW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_COMPW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_COMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_COMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Analog Comparator is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_COMPW::POWEREDON)
    }
    #[doc = "Analog Comparator is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_COMPW::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_TEMPSENS`"]
pub enum PDEN_TEMPSENSW {
    #[doc = "Temperature Sensor is powered."]
    POWEREDON,
    #[doc = "Temperature Sensor is powered down."]
    POWEREDOFF,
}
impl PDEN_TEMPSENSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_TEMPSENSW::POWEREDON => false,
            PDEN_TEMPSENSW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_TEMPSENSW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_TEMPSENSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_TEMPSENSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Temperature Sensor is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_TEMPSENSW::POWEREDON)
    }
    #[doc = "Temperature Sensor is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_TEMPSENSW::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_GPADC`"]
pub enum PDEN_GPADCW {
    #[doc = "GPADC is powered."]
    POWEREDON,
    #[doc = "GPADC is powered down."]
    POWEREDOFF,
}
impl PDEN_GPADCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_GPADCW::POWEREDON => false,
            PDEN_GPADCW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_GPADCW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_GPADCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_GPADCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "GPADC is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_GPADCW::POWEREDON)
    }
    #[doc = "GPADC is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_GPADCW::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_LDOMEM`"]
pub enum PDEN_LDOMEMW {
    #[doc = "Memories LDO is powered."]
    POWEREDON,
    #[doc = "Memories LDO is powered down."]
    POWEREDOFF,
}
impl PDEN_LDOMEMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_LDOMEMW::POWEREDON => false,
            PDEN_LDOMEMW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_LDOMEMW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_LDOMEMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_LDOMEMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Memories LDO is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_LDOMEMW::POWEREDON)
    }
    #[doc = "Memories LDO is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_LDOMEMW::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_LDODEEPSLEEP`"]
pub enum PDEN_LDODEEPSLEEPW {
    #[doc = "Deep Sleep LDO is powered."]
    POWEREDON,
    #[doc = "Deep Sleep LDO is powered down."]
    POWEREDOFF,
}
impl PDEN_LDODEEPSLEEPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_LDODEEPSLEEPW::POWEREDON => false,
            PDEN_LDODEEPSLEEPW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_LDODEEPSLEEPW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_LDODEEPSLEEPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_LDODEEPSLEEPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Deep Sleep LDO is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_LDODEEPSLEEPW::POWEREDON)
    }
    #[doc = "Deep Sleep LDO is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_LDODEEPSLEEPW::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_LDOUSBHS`"]
pub enum PDEN_LDOUSBHSW {
    #[doc = "USB high speed LDO is powered."]
    POWEREDON,
    #[doc = "USB high speed LDO is powered down."]
    POWEREDOFF,
}
impl PDEN_LDOUSBHSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_LDOUSBHSW::POWEREDON => false,
            PDEN_LDOUSBHSW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_LDOUSBHSW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_LDOUSBHSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_LDOUSBHSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "USB high speed LDO is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_LDOUSBHSW::POWEREDON)
    }
    #[doc = "USB high speed LDO is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_LDOUSBHSW::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_AUXBIAS`"]
pub enum PDEN_AUXBIASW {
    #[doc = "auxiliary biasing is powered."]
    POWEREDON,
    #[doc = "auxiliary biasing is powered down."]
    POWEREDOFF,
}
impl PDEN_AUXBIASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_AUXBIASW::POWEREDON => false,
            PDEN_AUXBIASW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_AUXBIASW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_AUXBIASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_AUXBIASW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "auxiliary biasing is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_AUXBIASW::POWEREDON)
    }
    #[doc = "auxiliary biasing is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_AUXBIASW::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_LDOXO32M`"]
pub enum PDEN_LDOXO32MW {
    #[doc = "crystal 32 MHz LDO is powered."]
    POWEREDON,
    #[doc = "crystal 32 MHz LDO is powered down."]
    POWEREDOFF,
}
impl PDEN_LDOXO32MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_LDOXO32MW::POWEREDON => false,
            PDEN_LDOXO32MW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_LDOXO32MW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_LDOXO32MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_LDOXO32MW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "crystal 32 MHz LDO is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_LDOXO32MW::POWEREDON)
    }
    #[doc = "crystal 32 MHz LDO is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_LDOXO32MW::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_LDOFLASHNV`"]
pub enum PDEN_LDOFLASHNVW {
    #[doc = "Flash NV LDO is powered."]
    POWEREDON,
    #[doc = "Flash NV LDO is powered down."]
    POWEREDOFF,
}
impl PDEN_LDOFLASHNVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_LDOFLASHNVW::POWEREDON => false,
            PDEN_LDOFLASHNVW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_LDOFLASHNVW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_LDOFLASHNVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_LDOFLASHNVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flash NV LDO is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_LDOFLASHNVW::POWEREDON)
    }
    #[doc = "Flash NV LDO is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_LDOFLASHNVW::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_RNG`"]
pub enum PDEN_RNGW {
    #[doc = "TRNG clocks are powered."]
    POWEREDON,
    #[doc = "TRNG clocks are powered down."]
    POWEREDOFF,
}
impl PDEN_RNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_RNGW::POWEREDON => false,
            PDEN_RNGW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_RNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_RNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_RNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TRNG clocks are powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_RNGW::POWEREDON)
    }
    #[doc = "TRNG clocks are powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_RNGW::POWEREDOFF)
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
#[doc = "Values that can be written to the field `PDEN_PLL0_SSCG`"]
pub enum PDEN_PLL0_SSCGW {
    #[doc = "PLL0 Sread spectrum module is powered."]
    POWEREDON,
    #[doc = "PLL0 Sread spectrum module is powered down."]
    POWEREDOFF,
}
impl PDEN_PLL0_SSCGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEN_PLL0_SSCGW::POWEREDON => false,
            PDEN_PLL0_SSCGW::POWEREDOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEN_PLL0_SSCGW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_PLL0_SSCGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEN_PLL0_SSCGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PLL0 Sread spectrum module is powered."]
    #[inline]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_PLL0_SSCGW::POWEREDON)
    }
    #[doc = "PLL0 Sread spectrum module is powered down."]
    #[inline]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_PLL0_SSCGW::POWEREDOFF)
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
    #[doc = "Bit 0 - Controls power to Bulk DCDC Converter."]
    #[inline]
    pub fn pden_dcdc(&self) -> PDEN_DCDCR {
        PDEN_DCDCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Controls power to ."]
    #[inline]
    pub fn pden_bias(&self) -> PDEN_BIASR {
        PDEN_BIASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Controls power to Core Brown Out Detector (BOD)."]
    #[inline]
    pub fn pden_bodcore(&self) -> PDEN_BODCORER {
        PDEN_BODCORER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Controls power to VBAT Brown Out Detector (BOD)."]
    #[inline]
    pub fn pden_bodvbat(&self) -> PDEN_BODVBATR {
        PDEN_BODVBATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Controls power to the Free Running Oscillator (FRO) 192 MHz; The 12MHz, 48 MHz and 96 MHz clocks are derived from this FRO."]
    #[inline]
    pub fn pden_fro192m(&self) -> PDEN_FRO192MR {
        PDEN_FRO192MR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Controls power to the Free Running Oscillator (FRO) 32 KHz."]
    #[inline]
    pub fn pden_fro32k(&self) -> PDEN_FRO32KR {
        PDEN_FRO32KR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Controls power to crystal 32 KHz."]
    #[inline]
    pub fn pden_xtal32k(&self) -> PDEN_XTAL32KR {
        PDEN_XTAL32KR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Controls power to crystal 32 MHz."]
    #[inline]
    pub fn pden_xtal32m(&self) -> PDEN_XTAL32MR {
        PDEN_XTAL32MR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Controls power to System PLL (also refered as PLL0)."]
    #[inline]
    pub fn pden_pll0(&self) -> PDEN_PLL0R {
        PDEN_PLL0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Controls power to USB PLL (also refered as PLL1)."]
    #[inline]
    pub fn pden_pll1(&self) -> PDEN_PLL1R {
        PDEN_PLL1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Controls power to USB Full Speed phy."]
    #[inline]
    pub fn pden_usbfsphy(&self) -> PDEN_USBFSPHYR {
        PDEN_USBFSPHYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Controls power to USB High Speed Phy."]
    #[inline]
    pub fn pden_usbhsphy(&self) -> PDEN_USBHSPHYR {
        PDEN_USBHSPHYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Controls power to Analog Comparator."]
    #[inline]
    pub fn pden_comp(&self) -> PDEN_COMPR {
        PDEN_COMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Controls power to Temperature Sensor."]
    #[inline]
    pub fn pden_tempsens(&self) -> PDEN_TEMPSENSR {
        PDEN_TEMPSENSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Controls power to General Purpose ADC (GPADC)."]
    #[inline]
    pub fn pden_gpadc(&self) -> PDEN_GPADCR {
        PDEN_GPADCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Controls power to Memories LDO."]
    #[inline]
    pub fn pden_ldomem(&self) -> PDEN_LDOMEMR {
        PDEN_LDOMEMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Controls power to Deep Sleep LDO."]
    #[inline]
    pub fn pden_ldodeepsleep(&self) -> PDEN_LDODEEPSLEEPR {
        PDEN_LDODEEPSLEEPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Controls power to USB high speed LDO."]
    #[inline]
    pub fn pden_ldousbhs(&self) -> PDEN_LDOUSBHSR {
        PDEN_LDOUSBHSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Controls power to auxiliary biasing (AUXBIAS)"]
    #[inline]
    pub fn pden_auxbias(&self) -> PDEN_AUXBIASR {
        PDEN_AUXBIASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Controls power to crystal 32 MHz LDO."]
    #[inline]
    pub fn pden_ldoxo32m(&self) -> PDEN_LDOXO32MR {
        PDEN_LDOXO32MR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Controls power to Flasn NV (high voltage) LDO."]
    #[inline]
    pub fn pden_ldoflashnv(&self) -> PDEN_LDOFLASHNVR {
        PDEN_LDOFLASHNVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Controls power to all True Random Number Genetaor (TRNG) clock sources."]
    #[inline]
    pub fn pden_rng(&self) -> PDEN_RNGR {
        PDEN_RNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Controls power to System PLL (PLL0) Spread Spectrum module."]
    #[inline]
    pub fn pden_pll0_sscg(&self) -> PDEN_PLL0_SSCGR {
        PDEN_PLL0_SSCGR::_from({
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
        W { bits: 14614468 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Controls power to Bulk DCDC Converter."]
    #[inline]
    pub fn pden_dcdc(&mut self) -> _PDEN_DCDCW {
        _PDEN_DCDCW { w: self }
    }
    #[doc = "Bit 1 - Controls power to ."]
    #[inline]
    pub fn pden_bias(&mut self) -> _PDEN_BIASW {
        _PDEN_BIASW { w: self }
    }
    #[doc = "Bit 2 - Controls power to Core Brown Out Detector (BOD)."]
    #[inline]
    pub fn pden_bodcore(&mut self) -> _PDEN_BODCOREW {
        _PDEN_BODCOREW { w: self }
    }
    #[doc = "Bit 3 - Controls power to VBAT Brown Out Detector (BOD)."]
    #[inline]
    pub fn pden_bodvbat(&mut self) -> _PDEN_BODVBATW {
        _PDEN_BODVBATW { w: self }
    }
    #[doc = "Bit 5 - Controls power to the Free Running Oscillator (FRO) 192 MHz; The 12MHz, 48 MHz and 96 MHz clocks are derived from this FRO."]
    #[inline]
    pub fn pden_fro192m(&mut self) -> _PDEN_FRO192MW {
        _PDEN_FRO192MW { w: self }
    }
    #[doc = "Bit 6 - Controls power to the Free Running Oscillator (FRO) 32 KHz."]
    #[inline]
    pub fn pden_fro32k(&mut self) -> _PDEN_FRO32KW {
        _PDEN_FRO32KW { w: self }
    }
    #[doc = "Bit 7 - Controls power to crystal 32 KHz."]
    #[inline]
    pub fn pden_xtal32k(&mut self) -> _PDEN_XTAL32KW {
        _PDEN_XTAL32KW { w: self }
    }
    #[doc = "Bit 8 - Controls power to crystal 32 MHz."]
    #[inline]
    pub fn pden_xtal32m(&mut self) -> _PDEN_XTAL32MW {
        _PDEN_XTAL32MW { w: self }
    }
    #[doc = "Bit 9 - Controls power to System PLL (also refered as PLL0)."]
    #[inline]
    pub fn pden_pll0(&mut self) -> _PDEN_PLL0W {
        _PDEN_PLL0W { w: self }
    }
    #[doc = "Bit 10 - Controls power to USB PLL (also refered as PLL1)."]
    #[inline]
    pub fn pden_pll1(&mut self) -> _PDEN_PLL1W {
        _PDEN_PLL1W { w: self }
    }
    #[doc = "Bit 11 - Controls power to USB Full Speed phy."]
    #[inline]
    pub fn pden_usbfsphy(&mut self) -> _PDEN_USBFSPHYW {
        _PDEN_USBFSPHYW { w: self }
    }
    #[doc = "Bit 12 - Controls power to USB High Speed Phy."]
    #[inline]
    pub fn pden_usbhsphy(&mut self) -> _PDEN_USBHSPHYW {
        _PDEN_USBHSPHYW { w: self }
    }
    #[doc = "Bit 13 - Controls power to Analog Comparator."]
    #[inline]
    pub fn pden_comp(&mut self) -> _PDEN_COMPW {
        _PDEN_COMPW { w: self }
    }
    #[doc = "Bit 14 - Controls power to Temperature Sensor."]
    #[inline]
    pub fn pden_tempsens(&mut self) -> _PDEN_TEMPSENSW {
        _PDEN_TEMPSENSW { w: self }
    }
    #[doc = "Bit 15 - Controls power to General Purpose ADC (GPADC)."]
    #[inline]
    pub fn pden_gpadc(&mut self) -> _PDEN_GPADCW {
        _PDEN_GPADCW { w: self }
    }
    #[doc = "Bit 16 - Controls power to Memories LDO."]
    #[inline]
    pub fn pden_ldomem(&mut self) -> _PDEN_LDOMEMW {
        _PDEN_LDOMEMW { w: self }
    }
    #[doc = "Bit 17 - Controls power to Deep Sleep LDO."]
    #[inline]
    pub fn pden_ldodeepsleep(&mut self) -> _PDEN_LDODEEPSLEEPW {
        _PDEN_LDODEEPSLEEPW { w: self }
    }
    #[doc = "Bit 18 - Controls power to USB high speed LDO."]
    #[inline]
    pub fn pden_ldousbhs(&mut self) -> _PDEN_LDOUSBHSW {
        _PDEN_LDOUSBHSW { w: self }
    }
    #[doc = "Bit 19 - Controls power to auxiliary biasing (AUXBIAS)"]
    #[inline]
    pub fn pden_auxbias(&mut self) -> _PDEN_AUXBIASW {
        _PDEN_AUXBIASW { w: self }
    }
    #[doc = "Bit 20 - Controls power to crystal 32 MHz LDO."]
    #[inline]
    pub fn pden_ldoxo32m(&mut self) -> _PDEN_LDOXO32MW {
        _PDEN_LDOXO32MW { w: self }
    }
    #[doc = "Bit 21 - Controls power to Flasn NV (high voltage) LDO."]
    #[inline]
    pub fn pden_ldoflashnv(&mut self) -> _PDEN_LDOFLASHNVW {
        _PDEN_LDOFLASHNVW { w: self }
    }
    #[doc = "Bit 22 - Controls power to all True Random Number Genetaor (TRNG) clock sources."]
    #[inline]
    pub fn pden_rng(&mut self) -> _PDEN_RNGW {
        _PDEN_RNGW { w: self }
    }
    #[doc = "Bit 23 - Controls power to System PLL (PLL0) Spread Spectrum module."]
    #[inline]
    pub fn pden_pll0_sscg(&mut self) -> _PDEN_PLL0_SSCGW {
        _PDEN_PLL0_SSCGW { w: self }
    }
}
