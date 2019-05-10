#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ANALOG_CTRL_STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PMU_IDR {
    bits: u8,
}
impl PMU_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OSC_IDR {
    bits: u8,
}
impl OSC_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FLASH_PWRDWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_PWRDWNR {
    #[doc = "Flash is not in power down mode."]
    PWRUP,
    #[doc = "Flash is in power down mode."]
    PWRDWN,
}
impl FLASH_PWRDWNR {
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
            FLASH_PWRDWNR::PWRUP => false,
            FLASH_PWRDWNR::PWRDWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASH_PWRDWNR {
        match value {
            false => FLASH_PWRDWNR::PWRUP,
            true => FLASH_PWRDWNR::PWRDWN,
        }
    }
    #[doc = "Checks if the value of the field is `PWRUP`"]
    #[inline]
    pub fn is_pwrup(&self) -> bool {
        *self == FLASH_PWRDWNR::PWRUP
    }
    #[doc = "Checks if the value of the field is `PWRDWN`"]
    #[inline]
    pub fn is_pwrdwn(&self) -> bool {
        *self == FLASH_PWRDWNR::PWRDWN
    }
}
#[doc = "Possible values of the field `FLASH_INIT_ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_INIT_ERRORR {
    #[doc = "No error."]
    NOERROR,
    #[doc = "At least one error occured during flash initialization.."]
    ERROR,
}
impl FLASH_INIT_ERRORR {
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
            FLASH_INIT_ERRORR::NOERROR => false,
            FLASH_INIT_ERRORR::ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASH_INIT_ERRORR {
        match value {
            false => FLASH_INIT_ERRORR::NOERROR,
            true => FLASH_INIT_ERRORR::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline]
    pub fn is_noerror(&self) -> bool {
        *self == FLASH_INIT_ERRORR::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == FLASH_INIT_ERRORR::ERROR
    }
}
#[doc = r" Value of the field"]
pub struct FINAL_TEST_DONE_VECTR {
    bits: u8,
}
impl FINAL_TEST_DONE_VECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Power Management Unit (PMU) Analog macro-bloc identification number : ."]
    #[inline]
    pub fn pmu_id(&self) -> PMU_IDR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PMU_IDR { bits }
    }
    #[doc = "Bits 6:11 - Oscillators Analog macro-bloc identification number : ."]
    #[inline]
    pub fn osc_id(&self) -> OSC_IDR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OSC_IDR { bits }
    }
    #[doc = "Bit 12 - Flash Power Down status."]
    #[inline]
    pub fn flash_pwrdwn(&self) -> FLASH_PWRDWNR {
        FLASH_PWRDWNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Flash initialization error status."]
    #[inline]
    pub fn flash_init_error(&self) -> FLASH_INIT_ERRORR {
        FLASH_INIT_ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 28:31 - Indicates current status of Final Test."]
    #[inline]
    pub fn final_test_done_vect(&self) -> FINAL_TEST_DONE_VECTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FINAL_TEST_DONE_VECTR { bits }
    }
}
