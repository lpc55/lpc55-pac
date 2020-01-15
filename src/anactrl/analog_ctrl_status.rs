#[doc = "Reader of register ANALOG_CTRL_STATUS"]
pub type R = crate::R<u32, super::ANALOG_CTRL_STATUS>;
#[doc = "Flash Power Down status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_PWRDWN_A {
    #[doc = "0: Flash is not in power down mode."]
    PWRUP = 0,
    #[doc = "1: Flash is in power down mode."]
    PWRDWN = 1,
}
impl From<FLASH_PWRDWN_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_PWRDWN_A) -> Self {
        variant as u8 != 0
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
#[doc = "Flash initialization error status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_INIT_ERROR_A {
    #[doc = "0: No error."]
    NOERROR = 0,
    #[doc = "1: At least one error occured during flash initialization.."]
    ERROR = 1,
}
impl From<FLASH_INIT_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_INIT_ERROR_A) -> Self {
        variant as u8 != 0
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
impl R {
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
}
