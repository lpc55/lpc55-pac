#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::BOD_DCDC_INT_STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `BODVBAT_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODVBAT_STATUSR {
    #[doc = "No interrupt pending.."]
    NOT_PENDING,
    #[doc = "Interrupt pending.."]
    PENDING,
}
impl BODVBAT_STATUSR {
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
            BODVBAT_STATUSR::NOT_PENDING => false,
            BODVBAT_STATUSR::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODVBAT_STATUSR {
        match value {
            false => BODVBAT_STATUSR::NOT_PENDING,
            true => BODVBAT_STATUSR::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == BODVBAT_STATUSR::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == BODVBAT_STATUSR::PENDING
    }
}
#[doc = "Possible values of the field `BODVBAT_INT_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODVBAT_INT_STATUSR {
    #[doc = "No interrupt pending.."]
    NOT_PENDING,
    #[doc = "Interrupt pending.."]
    PENDING,
}
impl BODVBAT_INT_STATUSR {
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
            BODVBAT_INT_STATUSR::NOT_PENDING => false,
            BODVBAT_INT_STATUSR::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODVBAT_INT_STATUSR {
        match value {
            false => BODVBAT_INT_STATUSR::NOT_PENDING,
            true => BODVBAT_INT_STATUSR::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == BODVBAT_INT_STATUSR::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == BODVBAT_INT_STATUSR::PENDING
    }
}
#[doc = "Possible values of the field `BODVBAT_VAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODVBAT_VALR {
    #[doc = "VBAT voltage level is below the threshold."]
    NOT_OK,
    #[doc = "VBAT voltage level is above the threshold."]
    OK,
}
impl BODVBAT_VALR {
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
            BODVBAT_VALR::NOT_OK => false,
            BODVBAT_VALR::OK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODVBAT_VALR {
        match value {
            false => BODVBAT_VALR::NOT_OK,
            true => BODVBAT_VALR::OK,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_OK`"]
    #[inline]
    pub fn is_not_ok(&self) -> bool {
        *self == BODVBAT_VALR::NOT_OK
    }
    #[doc = "Checks if the value of the field is `OK`"]
    #[inline]
    pub fn is_ok(&self) -> bool {
        *self == BODVBAT_VALR::OK
    }
}
#[doc = "Possible values of the field `BODCORE_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODCORE_STATUSR {
    #[doc = "No interrupt pending.."]
    NOT_PENDING,
    #[doc = "Interrupt pending.."]
    PENDING,
}
impl BODCORE_STATUSR {
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
            BODCORE_STATUSR::NOT_PENDING => false,
            BODCORE_STATUSR::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODCORE_STATUSR {
        match value {
            false => BODCORE_STATUSR::NOT_PENDING,
            true => BODCORE_STATUSR::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == BODCORE_STATUSR::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == BODCORE_STATUSR::PENDING
    }
}
#[doc = "Possible values of the field `BODCORE_INT_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODCORE_INT_STATUSR {
    #[doc = "No interrupt pending.."]
    NOT_PENDING,
    #[doc = "Interrupt pending.."]
    PENDING,
}
impl BODCORE_INT_STATUSR {
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
            BODCORE_INT_STATUSR::NOT_PENDING => false,
            BODCORE_INT_STATUSR::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODCORE_INT_STATUSR {
        match value {
            false => BODCORE_INT_STATUSR::NOT_PENDING,
            true => BODCORE_INT_STATUSR::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == BODCORE_INT_STATUSR::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == BODCORE_INT_STATUSR::PENDING
    }
}
#[doc = "Possible values of the field `BODCORE_VAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODCORE_VALR {
    #[doc = "CORE voltage level is below the threshold."]
    NOT_OK,
    #[doc = "CORE voltage level is above the threshold."]
    OK,
}
impl BODCORE_VALR {
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
            BODCORE_VALR::NOT_OK => false,
            BODCORE_VALR::OK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODCORE_VALR {
        match value {
            false => BODCORE_VALR::NOT_OK,
            true => BODCORE_VALR::OK,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_OK`"]
    #[inline]
    pub fn is_not_ok(&self) -> bool {
        *self == BODCORE_VALR::NOT_OK
    }
    #[doc = "Checks if the value of the field is `OK`"]
    #[inline]
    pub fn is_ok(&self) -> bool {
        *self == BODCORE_VALR::OK
    }
}
#[doc = "Possible values of the field `DCDC_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_STATUSR {
    #[doc = "No interrupt pending.."]
    NOT_PENDING,
    #[doc = "Interrupt pending.."]
    PENDING,
}
impl DCDC_STATUSR {
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
            DCDC_STATUSR::NOT_PENDING => false,
            DCDC_STATUSR::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCDC_STATUSR {
        match value {
            false => DCDC_STATUSR::NOT_PENDING,
            true => DCDC_STATUSR::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == DCDC_STATUSR::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == DCDC_STATUSR::PENDING
    }
}
#[doc = "Possible values of the field `DCDC_INT_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_INT_STATUSR {
    #[doc = "No interrupt pending.."]
    NOT_PENDING,
    #[doc = "Interrupt pending.."]
    PENDING,
}
impl DCDC_INT_STATUSR {
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
            DCDC_INT_STATUSR::NOT_PENDING => false,
            DCDC_INT_STATUSR::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCDC_INT_STATUSR {
        match value {
            false => DCDC_INT_STATUSR::NOT_PENDING,
            true => DCDC_INT_STATUSR::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == DCDC_INT_STATUSR::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == DCDC_INT_STATUSR::PENDING
    }
}
#[doc = "Possible values of the field `DCDC_VAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_VALR {
    #[doc = "DCDC output Voltage is below the targeted regulation level."]
    NOT_OK,
    #[doc = "DCDC output Voltage is above the targeted regulation level."]
    OK,
}
impl DCDC_VALR {
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
            DCDC_VALR::NOT_OK => false,
            DCDC_VALR::OK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCDC_VALR {
        match value {
            false => DCDC_VALR::NOT_OK,
            true => DCDC_VALR::OK,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_OK`"]
    #[inline]
    pub fn is_not_ok(&self) -> bool {
        *self == DCDC_VALR::NOT_OK
    }
    #[doc = "Checks if the value of the field is `OK`"]
    #[inline]
    pub fn is_ok(&self) -> bool {
        *self == DCDC_VALR::OK
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - BOD VBAT Interrupt status before Interrupt Enable."]
    #[inline]
    pub fn bodvbat_status(&self) -> BODVBAT_STATUSR {
        BODVBAT_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - BOD VBAT Interrupt status after Interrupt Enable."]
    #[inline]
    pub fn bodvbat_int_status(&self) -> BODVBAT_INT_STATUSR {
        BODVBAT_INT_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Current value of BOD VBAT power status output."]
    #[inline]
    pub fn bodvbat_val(&self) -> BODVBAT_VALR {
        BODVBAT_VALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - BOD CORE Interrupt status before Interrupt Enable."]
    #[inline]
    pub fn bodcore_status(&self) -> BODCORE_STATUSR {
        BODCORE_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - BOD CORE Interrupt status after Interrupt Enable."]
    #[inline]
    pub fn bodcore_int_status(&self) -> BODCORE_INT_STATUSR {
        BODCORE_INT_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Current value of BOD CORE power status output."]
    #[inline]
    pub fn bodcore_val(&self) -> BODCORE_VALR {
        BODCORE_VALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - DCDC Interrupt status before Interrupt Enable."]
    #[inline]
    pub fn dcdc_status(&self) -> DCDC_STATUSR {
        DCDC_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - DCDC Interrupt status after Interrupt Enable."]
    #[inline]
    pub fn dcdc_int_status(&self) -> DCDC_INT_STATUSR {
        DCDC_INT_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Current value of DCDC power status output."]
    #[inline]
    pub fn dcdc_val(&self) -> DCDC_VALR {
        DCDC_VALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
