#[doc = "Reader of register PRESETCTRL1"]
pub type R = crate::R<u32, super::PRESETCTRL1>;
#[doc = "Writer for register PRESETCTRL1"]
pub type W = crate::W<u32, super::PRESETCTRL1>;
#[doc = "Register PRESETCTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRESETCTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MRT reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRT_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<MRT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: MRT_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MRT_RST`"]
pub type MRT_RST_R = crate::R<bool, MRT_RST_A>;
impl MRT_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRT_RST_A {
        match self.bits {
            false => MRT_RST_A::RELEASED,
            true => MRT_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == MRT_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == MRT_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `MRT_RST`"]
pub struct MRT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> MRT_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MRT_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(MRT_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(MRT_RST_A::ASSERTED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "OS Event Timer reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSTIMER_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<OSTIMER_RST_A> for bool {
    #[inline(always)]
    fn from(variant: OSTIMER_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OSTIMER_RST`"]
pub type OSTIMER_RST_R = crate::R<bool, OSTIMER_RST_A>;
impl OSTIMER_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSTIMER_RST_A {
        match self.bits {
            false => OSTIMER_RST_A::RELEASED,
            true => OSTIMER_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == OSTIMER_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == OSTIMER_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `OSTIMER_RST`"]
pub struct OSTIMER_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> OSTIMER_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSTIMER_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(OSTIMER_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(OSTIMER_RST_A::ASSERTED)
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
#[doc = "SCT reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<SCT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SCT_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCT_RST`"]
pub type SCT_RST_R = crate::R<bool, SCT_RST_A>;
impl SCT_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT_RST_A {
        match self.bits {
            false => SCT_RST_A::RELEASED,
            true => SCT_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == SCT_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SCT_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `SCT_RST`"]
pub struct SCT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCT_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(SCT_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SCT_RST_A::ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "SCTIPU reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCTIPU_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<SCTIPU_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SCTIPU_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCTIPU_RST`"]
pub type SCTIPU_RST_R = crate::R<bool, SCTIPU_RST_A>;
impl SCTIPU_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCTIPU_RST_A {
        match self.bits {
            false => SCTIPU_RST_A::RELEASED,
            true => SCTIPU_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == SCTIPU_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SCTIPU_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `SCTIPU_RST`"]
pub struct SCTIPU_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SCTIPU_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCTIPU_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(SCTIPU_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SCTIPU_RST_A::ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "UTICK reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UTICK_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<UTICK_RST_A> for bool {
    #[inline(always)]
    fn from(variant: UTICK_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UTICK_RST`"]
pub type UTICK_RST_R = crate::R<bool, UTICK_RST_A>;
impl UTICK_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTICK_RST_A {
        match self.bits {
            false => UTICK_RST_A::RELEASED,
            true => UTICK_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == UTICK_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == UTICK_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `UTICK_RST`"]
pub struct UTICK_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UTICK_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UTICK_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(UTICK_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(UTICK_RST_A::ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "FC0 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC0_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FC0_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FC0_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FC0_RST`"]
pub type FC0_RST_R = crate::R<bool, FC0_RST_A>;
impl FC0_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC0_RST_A {
        match self.bits {
            false => FC0_RST_A::RELEASED,
            true => FC0_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FC0_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FC0_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `FC0_RST`"]
pub struct FC0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC0_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC0_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FC0_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC0_RST_A::ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "FC1 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC1_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FC1_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FC1_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FC1_RST`"]
pub type FC1_RST_R = crate::R<bool, FC1_RST_A>;
impl FC1_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC1_RST_A {
        match self.bits {
            false => FC1_RST_A::RELEASED,
            true => FC1_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FC1_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FC1_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `FC1_RST`"]
pub struct FC1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC1_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC1_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FC1_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC1_RST_A::ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "FC2 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC2_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FC2_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FC2_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FC2_RST`"]
pub type FC2_RST_R = crate::R<bool, FC2_RST_A>;
impl FC2_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC2_RST_A {
        match self.bits {
            false => FC2_RST_A::RELEASED,
            true => FC2_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FC2_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FC2_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `FC2_RST`"]
pub struct FC2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC2_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC2_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FC2_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC2_RST_A::ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "FC3 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC3_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FC3_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FC3_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FC3_RST`"]
pub type FC3_RST_R = crate::R<bool, FC3_RST_A>;
impl FC3_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC3_RST_A {
        match self.bits {
            false => FC3_RST_A::RELEASED,
            true => FC3_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FC3_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FC3_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `FC3_RST`"]
pub struct FC3_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC3_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC3_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FC3_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC3_RST_A::ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "FC4 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC4_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FC4_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FC4_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FC4_RST`"]
pub type FC4_RST_R = crate::R<bool, FC4_RST_A>;
impl FC4_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC4_RST_A {
        match self.bits {
            false => FC4_RST_A::RELEASED,
            true => FC4_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FC4_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FC4_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `FC4_RST`"]
pub struct FC4_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC4_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC4_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FC4_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC4_RST_A::ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "FC5 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC5_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FC5_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FC5_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FC5_RST`"]
pub type FC5_RST_R = crate::R<bool, FC5_RST_A>;
impl FC5_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC5_RST_A {
        match self.bits {
            false => FC5_RST_A::RELEASED,
            true => FC5_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FC5_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FC5_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `FC5_RST`"]
pub struct FC5_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC5_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC5_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FC5_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC5_RST_A::ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "FC6 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC6_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FC6_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FC6_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FC6_RST`"]
pub type FC6_RST_R = crate::R<bool, FC6_RST_A>;
impl FC6_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC6_RST_A {
        match self.bits {
            false => FC6_RST_A::RELEASED,
            true => FC6_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FC6_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FC6_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `FC6_RST`"]
pub struct FC6_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC6_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC6_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FC6_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC6_RST_A::ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "FC7 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC7_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FC7_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FC7_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FC7_RST`"]
pub type FC7_RST_R = crate::R<bool, FC7_RST_A>;
impl FC7_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC7_RST_A {
        match self.bits {
            false => FC7_RST_A::RELEASED,
            true => FC7_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FC7_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FC7_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `FC7_RST`"]
pub struct FC7_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC7_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC7_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FC7_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC7_RST_A::ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Timer 2 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER2_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<TIMER2_RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER2_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMER2_RST`"]
pub type TIMER2_RST_R = crate::R<bool, TIMER2_RST_A>;
impl TIMER2_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER2_RST_A {
        match self.bits {
            false => TIMER2_RST_A::RELEASED,
            true => TIMER2_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == TIMER2_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == TIMER2_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `TIMER2_RST`"]
pub struct TIMER2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER2_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(TIMER2_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TIMER2_RST_A::ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "USB0 DEV reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_DEV_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<USB0_DEV_RST_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_DEV_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USB0_DEV_RST`"]
pub type USB0_DEV_RST_R = crate::R<bool, USB0_DEV_RST_A>;
impl USB0_DEV_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_DEV_RST_A {
        match self.bits {
            false => USB0_DEV_RST_A::RELEASED,
            true => USB0_DEV_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == USB0_DEV_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == USB0_DEV_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `USB0_DEV_RST`"]
pub struct USB0_DEV_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_DEV_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB0_DEV_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(USB0_DEV_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB0_DEV_RST_A::ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Timer 0 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER0_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<TIMER0_RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER0_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMER0_RST`"]
pub type TIMER0_RST_R = crate::R<bool, TIMER0_RST_A>;
impl TIMER0_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER0_RST_A {
        match self.bits {
            false => TIMER0_RST_A::RELEASED,
            true => TIMER0_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == TIMER0_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == TIMER0_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `TIMER0_RST`"]
pub struct TIMER0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER0_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(TIMER0_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TIMER0_RST_A::ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Timer 1 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER1_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<TIMER1_RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER1_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMER1_RST`"]
pub type TIMER1_RST_R = crate::R<bool, TIMER1_RST_A>;
impl TIMER1_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER1_RST_A {
        match self.bits {
            false => TIMER1_RST_A::RELEASED,
            true => TIMER1_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == TIMER1_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == TIMER1_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `TIMER1_RST`"]
pub struct TIMER1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER1_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(TIMER1_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TIMER1_RST_A::ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MRT reset control."]
    #[inline(always)]
    pub fn mrt_rst(&self) -> MRT_RST_R {
        MRT_RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OS Event Timer reset control."]
    #[inline(always)]
    pub fn ostimer_rst(&self) -> OSTIMER_RST_R {
        OSTIMER_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SCT reset control."]
    #[inline(always)]
    pub fn sct_rst(&self) -> SCT_RST_R {
        SCT_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SCTIPU reset control."]
    #[inline(always)]
    pub fn sctipu_rst(&self) -> SCTIPU_RST_R {
        SCTIPU_RST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UTICK reset control."]
    #[inline(always)]
    pub fn utick_rst(&self) -> UTICK_RST_R {
        UTICK_RST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - FC0 reset control."]
    #[inline(always)]
    pub fn fc0_rst(&self) -> FC0_RST_R {
        FC0_RST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FC1 reset control."]
    #[inline(always)]
    pub fn fc1_rst(&self) -> FC1_RST_R {
        FC1_RST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - FC2 reset control."]
    #[inline(always)]
    pub fn fc2_rst(&self) -> FC2_RST_R {
        FC2_RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - FC3 reset control."]
    #[inline(always)]
    pub fn fc3_rst(&self) -> FC3_RST_R {
        FC3_RST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - FC4 reset control."]
    #[inline(always)]
    pub fn fc4_rst(&self) -> FC4_RST_R {
        FC4_RST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FC5 reset control."]
    #[inline(always)]
    pub fn fc5_rst(&self) -> FC5_RST_R {
        FC5_RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FC6 reset control."]
    #[inline(always)]
    pub fn fc6_rst(&self) -> FC6_RST_R {
        FC6_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - FC7 reset control."]
    #[inline(always)]
    pub fn fc7_rst(&self) -> FC7_RST_R {
        FC7_RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Timer 2 reset control."]
    #[inline(always)]
    pub fn timer2_rst(&self) -> TIMER2_RST_R {
        TIMER2_RST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 25 - USB0 DEV reset control."]
    #[inline(always)]
    pub fn usb0_dev_rst(&self) -> USB0_DEV_RST_R {
        USB0_DEV_RST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Timer 0 reset control."]
    #[inline(always)]
    pub fn timer0_rst(&self) -> TIMER0_RST_R {
        TIMER0_RST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Timer 1 reset control."]
    #[inline(always)]
    pub fn timer1_rst(&self) -> TIMER1_RST_R {
        TIMER1_RST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MRT reset control."]
    #[inline(always)]
    pub fn mrt_rst(&mut self) -> MRT_RST_W {
        MRT_RST_W { w: self }
    }
    #[doc = "Bit 1 - OS Event Timer reset control."]
    #[inline(always)]
    pub fn ostimer_rst(&mut self) -> OSTIMER_RST_W {
        OSTIMER_RST_W { w: self }
    }
    #[doc = "Bit 2 - SCT reset control."]
    #[inline(always)]
    pub fn sct_rst(&mut self) -> SCT_RST_W {
        SCT_RST_W { w: self }
    }
    #[doc = "Bit 6 - SCTIPU reset control."]
    #[inline(always)]
    pub fn sctipu_rst(&mut self) -> SCTIPU_RST_W {
        SCTIPU_RST_W { w: self }
    }
    #[doc = "Bit 10 - UTICK reset control."]
    #[inline(always)]
    pub fn utick_rst(&mut self) -> UTICK_RST_W {
        UTICK_RST_W { w: self }
    }
    #[doc = "Bit 11 - FC0 reset control."]
    #[inline(always)]
    pub fn fc0_rst(&mut self) -> FC0_RST_W {
        FC0_RST_W { w: self }
    }
    #[doc = "Bit 12 - FC1 reset control."]
    #[inline(always)]
    pub fn fc1_rst(&mut self) -> FC1_RST_W {
        FC1_RST_W { w: self }
    }
    #[doc = "Bit 13 - FC2 reset control."]
    #[inline(always)]
    pub fn fc2_rst(&mut self) -> FC2_RST_W {
        FC2_RST_W { w: self }
    }
    #[doc = "Bit 14 - FC3 reset control."]
    #[inline(always)]
    pub fn fc3_rst(&mut self) -> FC3_RST_W {
        FC3_RST_W { w: self }
    }
    #[doc = "Bit 15 - FC4 reset control."]
    #[inline(always)]
    pub fn fc4_rst(&mut self) -> FC4_RST_W {
        FC4_RST_W { w: self }
    }
    #[doc = "Bit 16 - FC5 reset control."]
    #[inline(always)]
    pub fn fc5_rst(&mut self) -> FC5_RST_W {
        FC5_RST_W { w: self }
    }
    #[doc = "Bit 17 - FC6 reset control."]
    #[inline(always)]
    pub fn fc6_rst(&mut self) -> FC6_RST_W {
        FC6_RST_W { w: self }
    }
    #[doc = "Bit 18 - FC7 reset control."]
    #[inline(always)]
    pub fn fc7_rst(&mut self) -> FC7_RST_W {
        FC7_RST_W { w: self }
    }
    #[doc = "Bit 22 - Timer 2 reset control."]
    #[inline(always)]
    pub fn timer2_rst(&mut self) -> TIMER2_RST_W {
        TIMER2_RST_W { w: self }
    }
    #[doc = "Bit 25 - USB0 DEV reset control."]
    #[inline(always)]
    pub fn usb0_dev_rst(&mut self) -> USB0_DEV_RST_W {
        USB0_DEV_RST_W { w: self }
    }
    #[doc = "Bit 26 - Timer 0 reset control."]
    #[inline(always)]
    pub fn timer0_rst(&mut self) -> TIMER0_RST_W {
        TIMER0_RST_W { w: self }
    }
    #[doc = "Bit 27 - Timer 1 reset control."]
    #[inline(always)]
    pub fn timer1_rst(&mut self) -> TIMER1_RST_W {
        TIMER1_RST_W { w: self }
    }
}
