#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u32, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `RDY0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDY0_A {
    #[doc = "Result FIFO 0 data level not above watermark level."]
    RDY0_0,
    #[doc = "Result FIFO 0 holding data above watermark level."]
    RDY0_1,
}
impl From<RDY0_A> for bool {
    #[inline(always)]
    fn from(variant: RDY0_A) -> Self {
        match variant {
            RDY0_A::RDY0_0 => false,
            RDY0_A::RDY0_1 => true,
        }
    }
}
#[doc = "Reader of field `RDY0`"]
pub type RDY0_R = crate::R<bool, RDY0_A>;
impl RDY0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDY0_A {
        match self.bits {
            false => RDY0_A::RDY0_0,
            true => RDY0_A::RDY0_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDY0_0`"]
    #[inline(always)]
    pub fn is_rdy0_0(&self) -> bool {
        *self == RDY0_A::RDY0_0
    }
    #[doc = "Checks if the value of the field is `RDY0_1`"]
    #[inline(always)]
    pub fn is_rdy0_1(&self) -> bool {
        *self == RDY0_A::RDY0_1
    }
}
#[doc = "Possible values of the field `FOF0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOF0_A {
    #[doc = "No result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    FOF0_0,
    #[doc = "At least one result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    FOF0_1,
}
impl From<FOF0_A> for bool {
    #[inline(always)]
    fn from(variant: FOF0_A) -> Self {
        match variant {
            FOF0_A::FOF0_0 => false,
            FOF0_A::FOF0_1 => true,
        }
    }
}
#[doc = "Reader of field `FOF0`"]
pub type FOF0_R = crate::R<bool, FOF0_A>;
impl FOF0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOF0_A {
        match self.bits {
            false => FOF0_A::FOF0_0,
            true => FOF0_A::FOF0_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOF0_0`"]
    #[inline(always)]
    pub fn is_fof0_0(&self) -> bool {
        *self == FOF0_A::FOF0_0
    }
    #[doc = "Checks if the value of the field is `FOF0_1`"]
    #[inline(always)]
    pub fn is_fof0_1(&self) -> bool {
        *self == FOF0_A::FOF0_1
    }
}
#[doc = "Write proxy for field `FOF0`"]
pub struct FOF0_W<'a> {
    w: &'a mut W,
}
impl<'a> FOF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOF0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn fof0_0(self) -> &'a mut W {
        self.variant(FOF0_A::FOF0_0)
    }
    #[doc = "At least one result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn fof0_1(self) -> &'a mut W {
        self.variant(FOF0_A::FOF0_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `RDY1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDY1_A {
    #[doc = "Result FIFO1 data level not above watermark level."]
    RDY1_0,
    #[doc = "Result FIFO1 holding data above watermark level."]
    RDY1_1,
}
impl From<RDY1_A> for bool {
    #[inline(always)]
    fn from(variant: RDY1_A) -> Self {
        match variant {
            RDY1_A::RDY1_0 => false,
            RDY1_A::RDY1_1 => true,
        }
    }
}
#[doc = "Reader of field `RDY1`"]
pub type RDY1_R = crate::R<bool, RDY1_A>;
impl RDY1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDY1_A {
        match self.bits {
            false => RDY1_A::RDY1_0,
            true => RDY1_A::RDY1_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDY1_0`"]
    #[inline(always)]
    pub fn is_rdy1_0(&self) -> bool {
        *self == RDY1_A::RDY1_0
    }
    #[doc = "Checks if the value of the field is `RDY1_1`"]
    #[inline(always)]
    pub fn is_rdy1_1(&self) -> bool {
        *self == RDY1_A::RDY1_1
    }
}
#[doc = "Possible values of the field `FOF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOF1_A {
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOF1_0,
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOF1_1,
}
impl From<FOF1_A> for bool {
    #[inline(always)]
    fn from(variant: FOF1_A) -> Self {
        match variant {
            FOF1_A::FOF1_0 => false,
            FOF1_A::FOF1_1 => true,
        }
    }
}
#[doc = "Reader of field `FOF1`"]
pub type FOF1_R = crate::R<bool, FOF1_A>;
impl FOF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOF1_A {
        match self.bits {
            false => FOF1_A::FOF1_0,
            true => FOF1_A::FOF1_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOF1_0`"]
    #[inline(always)]
    pub fn is_fof1_0(&self) -> bool {
        *self == FOF1_A::FOF1_0
    }
    #[doc = "Checks if the value of the field is `FOF1_1`"]
    #[inline(always)]
    pub fn is_fof1_1(&self) -> bool {
        *self == FOF1_A::FOF1_1
    }
}
#[doc = "Write proxy for field `FOF1`"]
pub struct FOF1_W<'a> {
    w: &'a mut W,
}
impl<'a> FOF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOF1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn fof1_0(self) -> &'a mut W {
        self.variant(FOF1_A::FOF1_0)
    }
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn fof1_1(self) -> &'a mut W {
        self.variant(FOF1_A::FOF1_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Possible values of the field `TEXC_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEXC_INT_A {
    #[doc = "No trigger exceptions have occurred."]
    TEXC_INT_0,
    #[doc = "A trigger exception has occurred and is pending acknowledgement."]
    TEXC_INT_1,
}
impl From<TEXC_INT_A> for bool {
    #[inline(always)]
    fn from(variant: TEXC_INT_A) -> Self {
        match variant {
            TEXC_INT_A::TEXC_INT_0 => false,
            TEXC_INT_A::TEXC_INT_1 => true,
        }
    }
}
#[doc = "Reader of field `TEXC_INT`"]
pub type TEXC_INT_R = crate::R<bool, TEXC_INT_A>;
impl TEXC_INT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEXC_INT_A {
        match self.bits {
            false => TEXC_INT_A::TEXC_INT_0,
            true => TEXC_INT_A::TEXC_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `TEXC_INT_0`"]
    #[inline(always)]
    pub fn is_texc_int_0(&self) -> bool {
        *self == TEXC_INT_A::TEXC_INT_0
    }
    #[doc = "Checks if the value of the field is `TEXC_INT_1`"]
    #[inline(always)]
    pub fn is_texc_int_1(&self) -> bool {
        *self == TEXC_INT_A::TEXC_INT_1
    }
}
#[doc = "Write proxy for field `TEXC_INT`"]
pub struct TEXC_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> TEXC_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEXC_INT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No trigger exceptions have occurred."]
    #[inline(always)]
    pub fn texc_int_0(self) -> &'a mut W {
        self.variant(TEXC_INT_A::TEXC_INT_0)
    }
    #[doc = "A trigger exception has occurred and is pending acknowledgement."]
    #[inline(always)]
    pub fn texc_int_1(self) -> &'a mut W {
        self.variant(TEXC_INT_A::TEXC_INT_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `TCOMP_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCOMP_INT_A {
    #[doc = "Either IE\\[TCOMP_IE\\] is set to 0, or no trigger sequences have run to completion."]
    TCOMP_INT_0,
    #[doc = "Trigger sequence has been completed and all data is stored in the associated FIFO."]
    TCOMP_INT_1,
}
impl From<TCOMP_INT_A> for bool {
    #[inline(always)]
    fn from(variant: TCOMP_INT_A) -> Self {
        match variant {
            TCOMP_INT_A::TCOMP_INT_0 => false,
            TCOMP_INT_A::TCOMP_INT_1 => true,
        }
    }
}
#[doc = "Reader of field `TCOMP_INT`"]
pub type TCOMP_INT_R = crate::R<bool, TCOMP_INT_A>;
impl TCOMP_INT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCOMP_INT_A {
        match self.bits {
            false => TCOMP_INT_A::TCOMP_INT_0,
            true => TCOMP_INT_A::TCOMP_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCOMP_INT_0`"]
    #[inline(always)]
    pub fn is_tcomp_int_0(&self) -> bool {
        *self == TCOMP_INT_A::TCOMP_INT_0
    }
    #[doc = "Checks if the value of the field is `TCOMP_INT_1`"]
    #[inline(always)]
    pub fn is_tcomp_int_1(&self) -> bool {
        *self == TCOMP_INT_A::TCOMP_INT_1
    }
}
#[doc = "Write proxy for field `TCOMP_INT`"]
pub struct TCOMP_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> TCOMP_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCOMP_INT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Either IE\\[TCOMP_IE\\] is set to 0, or no trigger sequences have run to completion."]
    #[inline(always)]
    pub fn tcomp_int_0(self) -> &'a mut W {
        self.variant(TCOMP_INT_A::TCOMP_INT_0)
    }
    #[doc = "Trigger sequence has been completed and all data is stored in the associated FIFO."]
    #[inline(always)]
    pub fn tcomp_int_1(self) -> &'a mut W {
        self.variant(TCOMP_INT_A::TCOMP_INT_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Possible values of the field `CAL_RDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAL_RDY_A {
    #[doc = "Calibration is incomplete or hasn't been ran."]
    CAL_RDY_0,
    #[doc = "The ADC is calibrated."]
    CAL_RDY_1,
}
impl From<CAL_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: CAL_RDY_A) -> Self {
        match variant {
            CAL_RDY_A::CAL_RDY_0 => false,
            CAL_RDY_A::CAL_RDY_1 => true,
        }
    }
}
#[doc = "Reader of field `CAL_RDY`"]
pub type CAL_RDY_R = crate::R<bool, CAL_RDY_A>;
impl CAL_RDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAL_RDY_A {
        match self.bits {
            false => CAL_RDY_A::CAL_RDY_0,
            true => CAL_RDY_A::CAL_RDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAL_RDY_0`"]
    #[inline(always)]
    pub fn is_cal_rdy_0(&self) -> bool {
        *self == CAL_RDY_A::CAL_RDY_0
    }
    #[doc = "Checks if the value of the field is `CAL_RDY_1`"]
    #[inline(always)]
    pub fn is_cal_rdy_1(&self) -> bool {
        *self == CAL_RDY_A::CAL_RDY_1
    }
}
#[doc = "Possible values of the field `ADC_ACTIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_ACTIVE_A {
    #[doc = "The ADC is IDLE. There are no pending triggers to service and no active commands are being processed."]
    ADC_ACTIVE_0,
    #[doc = "The ADC is processing a conversion, running through the power up delay, or servicing a trigger."]
    ADC_ACTIVE_1,
}
impl From<ADC_ACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_ACTIVE_A) -> Self {
        match variant {
            ADC_ACTIVE_A::ADC_ACTIVE_0 => false,
            ADC_ACTIVE_A::ADC_ACTIVE_1 => true,
        }
    }
}
#[doc = "Reader of field `ADC_ACTIVE`"]
pub type ADC_ACTIVE_R = crate::R<bool, ADC_ACTIVE_A>;
impl ADC_ACTIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_ACTIVE_A {
        match self.bits {
            false => ADC_ACTIVE_A::ADC_ACTIVE_0,
            true => ADC_ACTIVE_A::ADC_ACTIVE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_ACTIVE_0`"]
    #[inline(always)]
    pub fn is_adc_active_0(&self) -> bool {
        *self == ADC_ACTIVE_A::ADC_ACTIVE_0
    }
    #[doc = "Checks if the value of the field is `ADC_ACTIVE_1`"]
    #[inline(always)]
    pub fn is_adc_active_1(&self) -> bool {
        *self == ADC_ACTIVE_A::ADC_ACTIVE_1
    }
}
#[doc = "Possible values of the field `TRGACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGACT_A {
    #[doc = "Command (sequence) associated with Trigger 0 currently being executed."]
    TRGACT_0,
    #[doc = "Command (sequence) associated with Trigger 1 currently being executed."]
    TRGACT_1,
    #[doc = "Command (sequence) associated with Trigger 2 currently being executed."]
    TRGACT_2,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_3,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_4,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_5,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_6,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_7,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_8,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_9,
}
impl From<TRGACT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGACT_A) -> Self {
        match variant {
            TRGACT_A::TRGACT_0 => 0,
            TRGACT_A::TRGACT_1 => 1,
            TRGACT_A::TRGACT_2 => 2,
            TRGACT_A::TRGACT_3 => 3,
            TRGACT_A::TRGACT_4 => 4,
            TRGACT_A::TRGACT_5 => 5,
            TRGACT_A::TRGACT_6 => 6,
            TRGACT_A::TRGACT_7 => 7,
            TRGACT_A::TRGACT_8 => 8,
            TRGACT_A::TRGACT_9 => 9,
        }
    }
}
#[doc = "Reader of field `TRGACT`"]
pub type TRGACT_R = crate::R<u8, TRGACT_A>;
impl TRGACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRGACT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRGACT_A::TRGACT_0),
            1 => Val(TRGACT_A::TRGACT_1),
            2 => Val(TRGACT_A::TRGACT_2),
            3 => Val(TRGACT_A::TRGACT_3),
            4 => Val(TRGACT_A::TRGACT_4),
            5 => Val(TRGACT_A::TRGACT_5),
            6 => Val(TRGACT_A::TRGACT_6),
            7 => Val(TRGACT_A::TRGACT_7),
            8 => Val(TRGACT_A::TRGACT_8),
            9 => Val(TRGACT_A::TRGACT_9),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TRGACT_0`"]
    #[inline(always)]
    pub fn is_trgact_0(&self) -> bool {
        *self == TRGACT_A::TRGACT_0
    }
    #[doc = "Checks if the value of the field is `TRGACT_1`"]
    #[inline(always)]
    pub fn is_trgact_1(&self) -> bool {
        *self == TRGACT_A::TRGACT_1
    }
    #[doc = "Checks if the value of the field is `TRGACT_2`"]
    #[inline(always)]
    pub fn is_trgact_2(&self) -> bool {
        *self == TRGACT_A::TRGACT_2
    }
    #[doc = "Checks if the value of the field is `TRGACT_3`"]
    #[inline(always)]
    pub fn is_trgact_3(&self) -> bool {
        *self == TRGACT_A::TRGACT_3
    }
    #[doc = "Checks if the value of the field is `TRGACT_4`"]
    #[inline(always)]
    pub fn is_trgact_4(&self) -> bool {
        *self == TRGACT_A::TRGACT_4
    }
    #[doc = "Checks if the value of the field is `TRGACT_5`"]
    #[inline(always)]
    pub fn is_trgact_5(&self) -> bool {
        *self == TRGACT_A::TRGACT_5
    }
    #[doc = "Checks if the value of the field is `TRGACT_6`"]
    #[inline(always)]
    pub fn is_trgact_6(&self) -> bool {
        *self == TRGACT_A::TRGACT_6
    }
    #[doc = "Checks if the value of the field is `TRGACT_7`"]
    #[inline(always)]
    pub fn is_trgact_7(&self) -> bool {
        *self == TRGACT_A::TRGACT_7
    }
    #[doc = "Checks if the value of the field is `TRGACT_8`"]
    #[inline(always)]
    pub fn is_trgact_8(&self) -> bool {
        *self == TRGACT_A::TRGACT_8
    }
    #[doc = "Checks if the value of the field is `TRGACT_9`"]
    #[inline(always)]
    pub fn is_trgact_9(&self) -> bool {
        *self == TRGACT_A::TRGACT_9
    }
}
#[doc = "Possible values of the field `CMDACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDACT_A {
    #[doc = "No command is currently in progress."]
    CMDACT_0,
    #[doc = "Command 1 currently being executed."]
    CMDACT_1,
    #[doc = "Command 2 currently being executed."]
    CMDACT_2,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_3,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_4,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_5,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_6,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_7,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_8,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_9,
}
impl From<CMDACT_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDACT_A) -> Self {
        match variant {
            CMDACT_A::CMDACT_0 => 0,
            CMDACT_A::CMDACT_1 => 1,
            CMDACT_A::CMDACT_2 => 2,
            CMDACT_A::CMDACT_3 => 3,
            CMDACT_A::CMDACT_4 => 4,
            CMDACT_A::CMDACT_5 => 5,
            CMDACT_A::CMDACT_6 => 6,
            CMDACT_A::CMDACT_7 => 7,
            CMDACT_A::CMDACT_8 => 8,
            CMDACT_A::CMDACT_9 => 9,
        }
    }
}
#[doc = "Reader of field `CMDACT`"]
pub type CMDACT_R = crate::R<u8, CMDACT_A>;
impl CMDACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CMDACT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CMDACT_A::CMDACT_0),
            1 => Val(CMDACT_A::CMDACT_1),
            2 => Val(CMDACT_A::CMDACT_2),
            3 => Val(CMDACT_A::CMDACT_3),
            4 => Val(CMDACT_A::CMDACT_4),
            5 => Val(CMDACT_A::CMDACT_5),
            6 => Val(CMDACT_A::CMDACT_6),
            7 => Val(CMDACT_A::CMDACT_7),
            8 => Val(CMDACT_A::CMDACT_8),
            9 => Val(CMDACT_A::CMDACT_9),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CMDACT_0`"]
    #[inline(always)]
    pub fn is_cmdact_0(&self) -> bool {
        *self == CMDACT_A::CMDACT_0
    }
    #[doc = "Checks if the value of the field is `CMDACT_1`"]
    #[inline(always)]
    pub fn is_cmdact_1(&self) -> bool {
        *self == CMDACT_A::CMDACT_1
    }
    #[doc = "Checks if the value of the field is `CMDACT_2`"]
    #[inline(always)]
    pub fn is_cmdact_2(&self) -> bool {
        *self == CMDACT_A::CMDACT_2
    }
    #[doc = "Checks if the value of the field is `CMDACT_3`"]
    #[inline(always)]
    pub fn is_cmdact_3(&self) -> bool {
        *self == CMDACT_A::CMDACT_3
    }
    #[doc = "Checks if the value of the field is `CMDACT_4`"]
    #[inline(always)]
    pub fn is_cmdact_4(&self) -> bool {
        *self == CMDACT_A::CMDACT_4
    }
    #[doc = "Checks if the value of the field is `CMDACT_5`"]
    #[inline(always)]
    pub fn is_cmdact_5(&self) -> bool {
        *self == CMDACT_A::CMDACT_5
    }
    #[doc = "Checks if the value of the field is `CMDACT_6`"]
    #[inline(always)]
    pub fn is_cmdact_6(&self) -> bool {
        *self == CMDACT_A::CMDACT_6
    }
    #[doc = "Checks if the value of the field is `CMDACT_7`"]
    #[inline(always)]
    pub fn is_cmdact_7(&self) -> bool {
        *self == CMDACT_A::CMDACT_7
    }
    #[doc = "Checks if the value of the field is `CMDACT_8`"]
    #[inline(always)]
    pub fn is_cmdact_8(&self) -> bool {
        *self == CMDACT_A::CMDACT_8
    }
    #[doc = "Checks if the value of the field is `CMDACT_9`"]
    #[inline(always)]
    pub fn is_cmdact_9(&self) -> bool {
        *self == CMDACT_A::CMDACT_9
    }
}
impl R {
    #[doc = "Bit 0 - Result FIFO 0 Ready Flag"]
    #[inline(always)]
    pub fn rdy0(&self) -> RDY0_R {
        RDY0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Result FIFO 0 Overflow Flag"]
    #[inline(always)]
    pub fn fof0(&self) -> FOF0_R {
        FOF0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Result FIFO1 Ready Flag"]
    #[inline(always)]
    pub fn rdy1(&self) -> RDY1_R {
        RDY1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Result FIFO1 Overflow Flag"]
    #[inline(always)]
    pub fn fof1(&self) -> FOF1_R {
        FOF1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt Flag For High Priority Trigger Exception"]
    #[inline(always)]
    pub fn texc_int(&self) -> TEXC_INT_R {
        TEXC_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt Flag For Trigger Completion"]
    #[inline(always)]
    pub fn tcomp_int(&self) -> TCOMP_INT_R {
        TCOMP_INT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Calibration Ready"]
    #[inline(always)]
    pub fn cal_rdy(&self) -> CAL_RDY_R {
        CAL_RDY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADC Active"]
    #[inline(always)]
    pub fn adc_active(&self) -> ADC_ACTIVE_R {
        ADC_ACTIVE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Trigger Active"]
    #[inline(always)]
    pub fn trgact(&self) -> TRGACT_R {
        TRGACT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Command Active"]
    #[inline(always)]
    pub fn cmdact(&self) -> CMDACT_R {
        CMDACT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Result FIFO 0 Overflow Flag"]
    #[inline(always)]
    pub fn fof0(&mut self) -> FOF0_W {
        FOF0_W { w: self }
    }
    #[doc = "Bit 3 - Result FIFO1 Overflow Flag"]
    #[inline(always)]
    pub fn fof1(&mut self) -> FOF1_W {
        FOF1_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt Flag For High Priority Trigger Exception"]
    #[inline(always)]
    pub fn texc_int(&mut self) -> TEXC_INT_W {
        TEXC_INT_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt Flag For Trigger Completion"]
    #[inline(always)]
    pub fn tcomp_int(&mut self) -> TCOMP_INT_W {
        TCOMP_INT_W { w: self }
    }
}
