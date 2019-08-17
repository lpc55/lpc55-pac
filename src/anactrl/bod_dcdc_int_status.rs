#[doc = "Reader of register BOD_DCDC_INT_STATUS"]
pub type R = crate::R<u32, super::BOD_DCDC_INT_STATUS>;
#[doc = "Possible values of the field `BODVBAT_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODVBAT_STATUS_A {
    #[doc = "No interrupt pending.."]
    NOT_PENDING,
    #[doc = "Interrupt pending.."]
    PENDING,
}
impl From<BODVBAT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: BODVBAT_STATUS_A) -> Self {
        match variant {
            BODVBAT_STATUS_A::NOT_PENDING => false,
            BODVBAT_STATUS_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `BODVBAT_STATUS`"]
pub type BODVBAT_STATUS_R = crate::R<bool, BODVBAT_STATUS_A>;
impl BODVBAT_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODVBAT_STATUS_A {
        match self.bits {
            false => BODVBAT_STATUS_A::NOT_PENDING,
            true => BODVBAT_STATUS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == BODVBAT_STATUS_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == BODVBAT_STATUS_A::PENDING
    }
}
#[doc = "Possible values of the field `BODVBAT_INT_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODVBAT_INT_STATUS_A {
    #[doc = "No interrupt pending.."]
    NOT_PENDING,
    #[doc = "Interrupt pending.."]
    PENDING,
}
impl From<BODVBAT_INT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: BODVBAT_INT_STATUS_A) -> Self {
        match variant {
            BODVBAT_INT_STATUS_A::NOT_PENDING => false,
            BODVBAT_INT_STATUS_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `BODVBAT_INT_STATUS`"]
pub type BODVBAT_INT_STATUS_R = crate::R<bool, BODVBAT_INT_STATUS_A>;
impl BODVBAT_INT_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODVBAT_INT_STATUS_A {
        match self.bits {
            false => BODVBAT_INT_STATUS_A::NOT_PENDING,
            true => BODVBAT_INT_STATUS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == BODVBAT_INT_STATUS_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == BODVBAT_INT_STATUS_A::PENDING
    }
}
#[doc = "Possible values of the field `BODVBAT_VAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODVBAT_VAL_A {
    #[doc = "VBAT voltage level is below the threshold."]
    NOT_OK,
    #[doc = "VBAT voltage level is above the threshold."]
    OK,
}
impl From<BODVBAT_VAL_A> for bool {
    #[inline(always)]
    fn from(variant: BODVBAT_VAL_A) -> Self {
        match variant {
            BODVBAT_VAL_A::NOT_OK => false,
            BODVBAT_VAL_A::OK => true,
        }
    }
}
#[doc = "Reader of field `BODVBAT_VAL`"]
pub type BODVBAT_VAL_R = crate::R<bool, BODVBAT_VAL_A>;
impl BODVBAT_VAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODVBAT_VAL_A {
        match self.bits {
            false => BODVBAT_VAL_A::NOT_OK,
            true => BODVBAT_VAL_A::OK,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_OK`"]
    #[inline(always)]
    pub fn is_not_ok(&self) -> bool {
        *self == BODVBAT_VAL_A::NOT_OK
    }
    #[doc = "Checks if the value of the field is `OK`"]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == BODVBAT_VAL_A::OK
    }
}
#[doc = "Possible values of the field `BODCORE_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODCORE_STATUS_A {
    #[doc = "No interrupt pending.."]
    NOT_PENDING,
    #[doc = "Interrupt pending.."]
    PENDING,
}
impl From<BODCORE_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: BODCORE_STATUS_A) -> Self {
        match variant {
            BODCORE_STATUS_A::NOT_PENDING => false,
            BODCORE_STATUS_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `BODCORE_STATUS`"]
pub type BODCORE_STATUS_R = crate::R<bool, BODCORE_STATUS_A>;
impl BODCORE_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODCORE_STATUS_A {
        match self.bits {
            false => BODCORE_STATUS_A::NOT_PENDING,
            true => BODCORE_STATUS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == BODCORE_STATUS_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == BODCORE_STATUS_A::PENDING
    }
}
#[doc = "Possible values of the field `BODCORE_INT_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODCORE_INT_STATUS_A {
    #[doc = "No interrupt pending.."]
    NOT_PENDING,
    #[doc = "Interrupt pending.."]
    PENDING,
}
impl From<BODCORE_INT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: BODCORE_INT_STATUS_A) -> Self {
        match variant {
            BODCORE_INT_STATUS_A::NOT_PENDING => false,
            BODCORE_INT_STATUS_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `BODCORE_INT_STATUS`"]
pub type BODCORE_INT_STATUS_R = crate::R<bool, BODCORE_INT_STATUS_A>;
impl BODCORE_INT_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODCORE_INT_STATUS_A {
        match self.bits {
            false => BODCORE_INT_STATUS_A::NOT_PENDING,
            true => BODCORE_INT_STATUS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == BODCORE_INT_STATUS_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == BODCORE_INT_STATUS_A::PENDING
    }
}
#[doc = "Possible values of the field `BODCORE_VAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODCORE_VAL_A {
    #[doc = "CORE voltage level is below the threshold."]
    NOT_OK,
    #[doc = "CORE voltage level is above the threshold."]
    OK,
}
impl From<BODCORE_VAL_A> for bool {
    #[inline(always)]
    fn from(variant: BODCORE_VAL_A) -> Self {
        match variant {
            BODCORE_VAL_A::NOT_OK => false,
            BODCORE_VAL_A::OK => true,
        }
    }
}
#[doc = "Reader of field `BODCORE_VAL`"]
pub type BODCORE_VAL_R = crate::R<bool, BODCORE_VAL_A>;
impl BODCORE_VAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODCORE_VAL_A {
        match self.bits {
            false => BODCORE_VAL_A::NOT_OK,
            true => BODCORE_VAL_A::OK,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_OK`"]
    #[inline(always)]
    pub fn is_not_ok(&self) -> bool {
        *self == BODCORE_VAL_A::NOT_OK
    }
    #[doc = "Checks if the value of the field is `OK`"]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == BODCORE_VAL_A::OK
    }
}
#[doc = "Possible values of the field `DCDC_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_STATUS_A {
    #[doc = "No interrupt pending.."]
    NOT_PENDING,
    #[doc = "Interrupt pending.."]
    PENDING,
}
impl From<DCDC_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: DCDC_STATUS_A) -> Self {
        match variant {
            DCDC_STATUS_A::NOT_PENDING => false,
            DCDC_STATUS_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `DCDC_STATUS`"]
pub type DCDC_STATUS_R = crate::R<bool, DCDC_STATUS_A>;
impl DCDC_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDC_STATUS_A {
        match self.bits {
            false => DCDC_STATUS_A::NOT_PENDING,
            true => DCDC_STATUS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == DCDC_STATUS_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DCDC_STATUS_A::PENDING
    }
}
#[doc = "Possible values of the field `DCDC_INT_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_INT_STATUS_A {
    #[doc = "No interrupt pending.."]
    NOT_PENDING,
    #[doc = "Interrupt pending.."]
    PENDING,
}
impl From<DCDC_INT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: DCDC_INT_STATUS_A) -> Self {
        match variant {
            DCDC_INT_STATUS_A::NOT_PENDING => false,
            DCDC_INT_STATUS_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `DCDC_INT_STATUS`"]
pub type DCDC_INT_STATUS_R = crate::R<bool, DCDC_INT_STATUS_A>;
impl DCDC_INT_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDC_INT_STATUS_A {
        match self.bits {
            false => DCDC_INT_STATUS_A::NOT_PENDING,
            true => DCDC_INT_STATUS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == DCDC_INT_STATUS_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DCDC_INT_STATUS_A::PENDING
    }
}
#[doc = "Possible values of the field `DCDC_VAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_VAL_A {
    #[doc = "DCDC output Voltage is below the targeted regulation level."]
    NOT_OK,
    #[doc = "DCDC output Voltage is above the targeted regulation level."]
    OK,
}
impl From<DCDC_VAL_A> for bool {
    #[inline(always)]
    fn from(variant: DCDC_VAL_A) -> Self {
        match variant {
            DCDC_VAL_A::NOT_OK => false,
            DCDC_VAL_A::OK => true,
        }
    }
}
#[doc = "Reader of field `DCDC_VAL`"]
pub type DCDC_VAL_R = crate::R<bool, DCDC_VAL_A>;
impl DCDC_VAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDC_VAL_A {
        match self.bits {
            false => DCDC_VAL_A::NOT_OK,
            true => DCDC_VAL_A::OK,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_OK`"]
    #[inline(always)]
    pub fn is_not_ok(&self) -> bool {
        *self == DCDC_VAL_A::NOT_OK
    }
    #[doc = "Checks if the value of the field is `OK`"]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == DCDC_VAL_A::OK
    }
}
impl R {
    #[doc = "Bit 0 - BOD VBAT Interrupt status before Interrupt Enable."]
    #[inline(always)]
    pub fn bodvbat_status(&self) -> BODVBAT_STATUS_R {
        BODVBAT_STATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BOD VBAT Interrupt status after Interrupt Enable."]
    #[inline(always)]
    pub fn bodvbat_int_status(&self) -> BODVBAT_INT_STATUS_R {
        BODVBAT_INT_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Current value of BOD VBAT power status output."]
    #[inline(always)]
    pub fn bodvbat_val(&self) -> BODVBAT_VAL_R {
        BODVBAT_VAL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BOD CORE Interrupt status before Interrupt Enable."]
    #[inline(always)]
    pub fn bodcore_status(&self) -> BODCORE_STATUS_R {
        BODCORE_STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BOD CORE Interrupt status after Interrupt Enable."]
    #[inline(always)]
    pub fn bodcore_int_status(&self) -> BODCORE_INT_STATUS_R {
        BODCORE_INT_STATUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Current value of BOD CORE power status output."]
    #[inline(always)]
    pub fn bodcore_val(&self) -> BODCORE_VAL_R {
        BODCORE_VAL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DCDC Interrupt status before Interrupt Enable."]
    #[inline(always)]
    pub fn dcdc_status(&self) -> DCDC_STATUS_R {
        DCDC_STATUS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DCDC Interrupt status after Interrupt Enable."]
    #[inline(always)]
    pub fn dcdc_int_status(&self) -> DCDC_INT_STATUS_R {
        DCDC_INT_STATUS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Current value of DCDC power status output."]
    #[inline(always)]
    pub fn dcdc_val(&self) -> DCDC_VAL_R {
        DCDC_VAL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
