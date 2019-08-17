#[doc = "Reader of register ANALOG_CTRL_STATUS"]
pub type R = crate::R<u32, super::ANALOG_CTRL_STATUS>;
#[doc = "Reader of field `PMU_ID`"]
pub type PMU_ID_R = crate::R<u8, u8>;
#[doc = "Reader of field `OSC_ID`"]
pub type OSC_ID_R = crate::R<u8, u8>;
#[doc = "Possible values of the field `FLASH_PWRDWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_PWRDWN_A {
    #[doc = "Flash is not in power down mode."]
    PWRUP,
    #[doc = "Flash is in power down mode."]
    PWRDWN,
}
impl From<FLASH_PWRDWN_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_PWRDWN_A) -> Self {
        match variant {
            FLASH_PWRDWN_A::PWRUP => false,
            FLASH_PWRDWN_A::PWRDWN => true,
        }
    }
}
#[doc = "Reader of field `FLASH_PWRDWN`"]
pub type FLASH_PWRDWN_R = crate::R<bool, FLASH_PWRDWN_A>;
impl FLASH_PWRDWN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_PWRDWN_A {
        match self.bits {
            false => FLASH_PWRDWN_A::PWRUP,
            true => FLASH_PWRDWN_A::PWRDWN,
        }
    }
    #[doc = "Checks if the value of the field is `PWRUP`"]
    #[inline(always)]
    pub fn is_pwrup(&self) -> bool {
        *self == FLASH_PWRDWN_A::PWRUP
    }
    #[doc = "Checks if the value of the field is `PWRDWN`"]
    #[inline(always)]
    pub fn is_pwrdwn(&self) -> bool {
        *self == FLASH_PWRDWN_A::PWRDWN
    }
}
#[doc = "Possible values of the field `FLASH_INIT_ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_INIT_ERROR_A {
    #[doc = "No error."]
    NOERROR,
    #[doc = "At least one error occured during flash initialization.."]
    ERROR,
}
impl From<FLASH_INIT_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_INIT_ERROR_A) -> Self {
        match variant {
            FLASH_INIT_ERROR_A::NOERROR => false,
            FLASH_INIT_ERROR_A::ERROR => true,
        }
    }
}
#[doc = "Reader of field `FLASH_INIT_ERROR`"]
pub type FLASH_INIT_ERROR_R = crate::R<bool, FLASH_INIT_ERROR_A>;
impl FLASH_INIT_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_INIT_ERROR_A {
        match self.bits {
            false => FLASH_INIT_ERROR_A::NOERROR,
            true => FLASH_INIT_ERROR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == FLASH_INIT_ERROR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FLASH_INIT_ERROR_A::ERROR
    }
}
#[doc = "Reader of field `FINAL_TEST_DONE_VECT`"]
pub type FINAL_TEST_DONE_VECT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Power Management Unit (PMU) Analog macro-bloc identification number : ."]
    #[inline(always)]
    pub fn pmu_id(&self) -> PMU_ID_R {
        PMU_ID_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Oscillators Analog macro-bloc identification number : ."]
    #[inline(always)]
    pub fn osc_id(&self) -> OSC_ID_R {
        OSC_ID_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Flash Power Down status."]
    #[inline(always)]
    pub fn flash_pwrdwn(&self) -> FLASH_PWRDWN_R {
        FLASH_PWRDWN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Flash initialization error status."]
    #[inline(always)]
    pub fn flash_init_error(&self) -> FLASH_INIT_ERROR_R {
        FLASH_INIT_ERROR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - Indicates current status of Final Test."]
    #[inline(always)]
    pub fn final_test_done_vect(&self) -> FINAL_TEST_DONE_VECT_R {
        FINAL_TEST_DONE_VECT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
