#[doc = "Register `ANALOG_CTRL_STATUS` reader"]
pub struct R(crate::R<ANALOG_CTRL_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANALOG_CTRL_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ANALOG_CTRL_STATUS_SPEC>> for R {
    fn from(reader: crate::R<ANALOG_CTRL_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `FLASH_PWRDWN` reader - Flash Power Down status."]
pub struct FLASH_PWRDWN_R(crate::FieldReader<bool, FLASH_PWRDWN_A>);
impl FLASH_PWRDWN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_PWRDWN_R(crate::FieldReader::new(bits))
    }
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
        **self == FLASH_PWRDWN_A::PWRUP
    }
    #[doc = "Checks if the value of the field is `PWRDWN`"]
    #[inline(always)]
    pub fn is_pwrdwn(&self) -> bool {
        **self == FLASH_PWRDWN_A::PWRDWN
    }
}
impl core::ops::Deref for FLASH_PWRDWN_R {
    type Target = crate::FieldReader<bool, FLASH_PWRDWN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `FLASH_INIT_ERROR` reader - Flash initialization error status."]
pub struct FLASH_INIT_ERROR_R(crate::FieldReader<bool, FLASH_INIT_ERROR_A>);
impl FLASH_INIT_ERROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_INIT_ERROR_R(crate::FieldReader::new(bits))
    }
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
        **self == FLASH_INIT_ERROR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == FLASH_INIT_ERROR_A::ERROR
    }
}
impl core::ops::Deref for FLASH_INIT_ERROR_R {
    type Target = crate::FieldReader<bool, FLASH_INIT_ERROR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Analog Macroblock Identity registers, Flash Status registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [analog_ctrl_status](index.html) module"]
pub struct ANALOG_CTRL_STATUS_SPEC;
impl crate::RegisterSpec for ANALOG_CTRL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [analog_ctrl_status::R](R) reader structure"]
impl crate::Readable for ANALOG_CTRL_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ANALOG_CTRL_STATUS to value 0x5000_0000"]
impl crate::Resettable for ANALOG_CTRL_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5000_0000
    }
}
