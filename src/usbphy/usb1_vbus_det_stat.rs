#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USB1_VBUS_DET_STAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `SESSEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SESSENDR {
    #[doc = "The VBUS voltage is above the Session Valid threshold"]
    VALUE0,
    #[doc = "The VBUS voltage is below the Session Valid threshold"]
    VALUE1,
}
impl SESSENDR {
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
            SESSENDR::VALUE0 => false,
            SESSENDR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SESSENDR {
        match value {
            false => SESSENDR::VALUE0,
            true => SESSENDR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == SESSENDR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SESSENDR::VALUE1
    }
}
#[doc = "Possible values of the field `BVALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BVALIDR {
    #[doc = "The VBUS voltage is below the Session Valid threshold"]
    VALUE0,
    #[doc = "The VBUS voltage is above the Session Valid threshold"]
    VALUE1,
}
impl BVALIDR {
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
            BVALIDR::VALUE0 => false,
            BVALIDR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BVALIDR {
        match value {
            false => BVALIDR::VALUE0,
            true => BVALIDR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == BVALIDR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BVALIDR::VALUE1
    }
}
#[doc = "Possible values of the field `AVALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVALIDR {
    #[doc = "The VBUS voltage is below the Session Valid threshold"]
    VALUE0,
    #[doc = "The VBUS voltage is above the Session Valid threshold"]
    VALUE1,
}
impl AVALIDR {
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
            AVALIDR::VALUE0 => false,
            AVALIDR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVALIDR {
        match value {
            false => AVALIDR::VALUE0,
            true => AVALIDR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == AVALIDR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVALIDR::VALUE1
    }
}
#[doc = "Possible values of the field `VBUS_VALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUS_VALIDR {
    #[doc = "VBUS is below the comparator threshold"]
    VALUE0,
    #[doc = "VBUS is above the comparator threshold"]
    VALUE1,
}
impl VBUS_VALIDR {
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
            VBUS_VALIDR::VALUE0 => false,
            VBUS_VALIDR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VBUS_VALIDR {
        match value {
            false => VBUS_VALIDR::VALUE0,
            true => VBUS_VALIDR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == VBUS_VALIDR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VBUS_VALIDR::VALUE1
    }
}
#[doc = "Possible values of the field `VBUS_VALID_3V`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUS_VALID_3VR {
    #[doc = "VBUS voltage is below VBUS_VALID_3V threshold"]
    VALUE0,
    #[doc = "VBUS voltage is above VBUS_VALID_3V threshold"]
    VALUE1,
}
impl VBUS_VALID_3VR {
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
            VBUS_VALID_3VR::VALUE0 => false,
            VBUS_VALID_3VR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VBUS_VALID_3VR {
        match value {
            false => VBUS_VALID_3VR::VALUE0,
            true => VBUS_VALID_3VR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == VBUS_VALID_3VR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VBUS_VALID_3VR::VALUE1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Session End indicator Session End status, value inverted from Session Valid comparator"]
    #[inline]
    pub fn sessend(&self) -> SESSENDR {
        SESSENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - B-Device Session Valid status B-Device Session Valid status, determined by the Session Valid comparator"]
    #[inline]
    pub fn bvalid(&self) -> BVALIDR {
        BVALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - A-Device Session Valid status A-Device Session Valid status, determined by the Session Valid comparator"]
    #[inline]
    pub fn avalid(&self) -> AVALIDR {
        AVALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - VBUS voltage status This bit field shows the result of VBUS_VALID detection for the USB1_VBUS pin"]
    #[inline]
    pub fn vbus_valid(&self) -> VBUS_VALIDR {
        VBUS_VALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - VBUS_VALID_3V detector status The VBUS_VALID_3V detector has a lower threshold for the voltage on the USB1_VBUS pin than either the Session Valid or VBUS_VALID comparators"]
    #[inline]
    pub fn vbus_valid_3v(&self) -> VBUS_VALID_3VR {
        VBUS_VALID_3VR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
