#[doc = "Register `PRESETCTRL1` reader"]
pub struct R(crate::R<PRESETCTRL_PRESETCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESETCTRL_PRESETCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PRESETCTRL_PRESETCTRL1_SPEC>> for R {
    fn from(reader: crate::R<PRESETCTRL_PRESETCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESETCTRL1` writer"]
pub struct W(crate::W<PRESETCTRL_PRESETCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESETCTRL_PRESETCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<PRESETCTRL_PRESETCTRL1_SPEC>> for W {
    fn from(writer: crate::W<PRESETCTRL_PRESETCTRL1_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `MRT_RST` reader - MRT reset control."]
pub struct MRT_RST_R(crate::FieldReader<bool, MRT_RST_A>);
impl MRT_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MRT_RST_R(crate::FieldReader::new(bits))
    }
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
        **self == MRT_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == MRT_RST_A::ASSERTED
    }
}
impl core::ops::Deref for MRT_RST_R {
    type Target = crate::FieldReader<bool, MRT_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRT_RST` writer - MRT reset control."]
pub struct MRT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> MRT_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MRT_RST_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
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
#[doc = "Field `OSTIMER_RST` reader - OS Event Timer reset control."]
pub struct OSTIMER_RST_R(crate::FieldReader<bool, OSTIMER_RST_A>);
impl OSTIMER_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSTIMER_RST_R(crate::FieldReader::new(bits))
    }
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
        **self == OSTIMER_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == OSTIMER_RST_A::ASSERTED
    }
}
impl core::ops::Deref for OSTIMER_RST_R {
    type Target = crate::FieldReader<bool, OSTIMER_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSTIMER_RST` writer - OS Event Timer reset control."]
pub struct OSTIMER_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> OSTIMER_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSTIMER_RST_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
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
#[doc = "Field `SCT_RST` reader - SCT reset control."]
pub struct SCT_RST_R(crate::FieldReader<bool, SCT_RST_A>);
impl SCT_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCT_RST_R(crate::FieldReader::new(bits))
    }
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
        **self == SCT_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == SCT_RST_A::ASSERTED
    }
}
impl core::ops::Deref for SCT_RST_R {
    type Target = crate::FieldReader<bool, SCT_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCT_RST` writer - SCT reset control."]
pub struct SCT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCT_RST_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
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
#[doc = "Field `SCTIPU_RST` reader - SCTIPU reset control."]
pub struct SCTIPU_RST_R(crate::FieldReader<bool, SCTIPU_RST_A>);
impl SCTIPU_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCTIPU_RST_R(crate::FieldReader::new(bits))
    }
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
        **self == SCTIPU_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == SCTIPU_RST_A::ASSERTED
    }
}
impl core::ops::Deref for SCTIPU_RST_R {
    type Target = crate::FieldReader<bool, SCTIPU_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCTIPU_RST` writer - SCTIPU reset control."]
pub struct SCTIPU_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SCTIPU_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCTIPU_RST_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
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
#[doc = "Field `UTICK_RST` reader - UTICK reset control."]
pub struct UTICK_RST_R(crate::FieldReader<bool, UTICK_RST_A>);
impl UTICK_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UTICK_RST_R(crate::FieldReader::new(bits))
    }
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
        **self == UTICK_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == UTICK_RST_A::ASSERTED
    }
}
impl core::ops::Deref for UTICK_RST_R {
    type Target = crate::FieldReader<bool, UTICK_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTICK_RST` writer - UTICK reset control."]
pub struct UTICK_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UTICK_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UTICK_RST_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
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
#[doc = "Field `FC0_RST` reader - FC0 reset control."]
pub struct FC0_RST_R(crate::FieldReader<bool, FC0_RST_A>);
impl FC0_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC0_RST_R(crate::FieldReader::new(bits))
    }
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
        **self == FC0_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == FC0_RST_A::ASSERTED
    }
}
impl core::ops::Deref for FC0_RST_R {
    type Target = crate::FieldReader<bool, FC0_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC0_RST` writer - FC0 reset control."]
pub struct FC0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC0_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC0_RST_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
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
#[doc = "Field `FC1_RST` reader - FC1 reset control."]
pub struct FC1_RST_R(crate::FieldReader<bool, FC1_RST_A>);
impl FC1_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC1_RST_R(crate::FieldReader::new(bits))
    }
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
        **self == FC1_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == FC1_RST_A::ASSERTED
    }
}
impl core::ops::Deref for FC1_RST_R {
    type Target = crate::FieldReader<bool, FC1_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC1_RST` writer - FC1 reset control."]
pub struct FC1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC1_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC1_RST_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
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
#[doc = "Field `FC2_RST` reader - FC2 reset control."]
pub struct FC2_RST_R(crate::FieldReader<bool, FC2_RST_A>);
impl FC2_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC2_RST_R(crate::FieldReader::new(bits))
    }
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
        **self == FC2_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == FC2_RST_A::ASSERTED
    }
}
impl core::ops::Deref for FC2_RST_R {
    type Target = crate::FieldReader<bool, FC2_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC2_RST` writer - FC2 reset control."]
pub struct FC2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC2_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC2_RST_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
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
#[doc = "Field `FC3_RST` reader - FC3 reset control."]
pub struct FC3_RST_R(crate::FieldReader<bool, FC3_RST_A>);
impl FC3_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC3_RST_R(crate::FieldReader::new(bits))
    }
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
        **self == FC3_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == FC3_RST_A::ASSERTED
    }
}
impl core::ops::Deref for FC3_RST_R {
    type Target = crate::FieldReader<bool, FC3_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC3_RST` writer - FC3 reset control."]
pub struct FC3_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC3_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC3_RST_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
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
#[doc = "Field `FC4_RST` reader - FC4 reset control."]
pub struct FC4_RST_R(crate::FieldReader<bool, FC4_RST_A>);
impl FC4_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC4_RST_R(crate::FieldReader::new(bits))
    }
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
        **self == FC4_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == FC4_RST_A::ASSERTED
    }
}
impl core::ops::Deref for FC4_RST_R {
    type Target = crate::FieldReader<bool, FC4_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC4_RST` writer - FC4 reset control."]
pub struct FC4_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC4_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC4_RST_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
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
#[doc = "Field `FC5_RST` reader - FC5 reset control."]
pub struct FC5_RST_R(crate::FieldReader<bool, FC5_RST_A>);
impl FC5_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC5_RST_R(crate::FieldReader::new(bits))
    }
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
        **self == FC5_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == FC5_RST_A::ASSERTED
    }
}
impl core::ops::Deref for FC5_RST_R {
    type Target = crate::FieldReader<bool, FC5_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC5_RST` writer - FC5 reset control."]
pub struct FC5_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC5_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC5_RST_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
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
#[doc = "Field `FC6_RST` reader - FC6 reset control."]
pub struct FC6_RST_R(crate::FieldReader<bool, FC6_RST_A>);
impl FC6_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC6_RST_R(crate::FieldReader::new(bits))
    }
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
        **self == FC6_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == FC6_RST_A::ASSERTED
    }
}
impl core::ops::Deref for FC6_RST_R {
    type Target = crate::FieldReader<bool, FC6_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC6_RST` writer - FC6 reset control."]
pub struct FC6_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC6_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC6_RST_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
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
#[doc = "Field `FC7_RST` reader - FC7 reset control."]
pub struct FC7_RST_R(crate::FieldReader<bool, FC7_RST_A>);
impl FC7_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC7_RST_R(crate::FieldReader::new(bits))
    }
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
        **self == FC7_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == FC7_RST_A::ASSERTED
    }
}
impl core::ops::Deref for FC7_RST_R {
    type Target = crate::FieldReader<bool, FC7_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC7_RST` writer - FC7 reset control."]
pub struct FC7_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC7_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC7_RST_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
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
#[doc = "Field `TIMER2_RST` reader - Timer 2 reset control."]
pub struct TIMER2_RST_R(crate::FieldReader<bool, TIMER2_RST_A>);
impl TIMER2_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER2_RST_R(crate::FieldReader::new(bits))
    }
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
        **self == TIMER2_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == TIMER2_RST_A::ASSERTED
    }
}
impl core::ops::Deref for TIMER2_RST_R {
    type Target = crate::FieldReader<bool, TIMER2_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER2_RST` writer - Timer 2 reset control."]
pub struct TIMER2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER2_RST_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
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
#[doc = "Field `USB0_DEV_RST` reader - USB0 DEV reset control."]
pub struct USB0_DEV_RST_R(crate::FieldReader<bool, USB0_DEV_RST_A>);
impl USB0_DEV_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB0_DEV_RST_R(crate::FieldReader::new(bits))
    }
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
        **self == USB0_DEV_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == USB0_DEV_RST_A::ASSERTED
    }
}
impl core::ops::Deref for USB0_DEV_RST_R {
    type Target = crate::FieldReader<bool, USB0_DEV_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB0_DEV_RST` writer - USB0 DEV reset control."]
pub struct USB0_DEV_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_DEV_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB0_DEV_RST_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
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
#[doc = "Field `TIMER0_RST` reader - Timer 0 reset control."]
pub struct TIMER0_RST_R(crate::FieldReader<bool, TIMER0_RST_A>);
impl TIMER0_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER0_RST_R(crate::FieldReader::new(bits))
    }
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
        **self == TIMER0_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == TIMER0_RST_A::ASSERTED
    }
}
impl core::ops::Deref for TIMER0_RST_R {
    type Target = crate::FieldReader<bool, TIMER0_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER0_RST` writer - Timer 0 reset control."]
pub struct TIMER0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER0_RST_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
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
#[doc = "Field `TIMER1_RST` reader - Timer 1 reset control."]
pub struct TIMER1_RST_R(crate::FieldReader<bool, TIMER1_RST_A>);
impl TIMER1_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_RST_R(crate::FieldReader::new(bits))
    }
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
        **self == TIMER1_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == TIMER1_RST_A::ASSERTED
    }
}
impl core::ops::Deref for TIMER1_RST_R {
    type Target = crate::FieldReader<bool, TIMER1_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_RST` writer - Timer 1 reset control."]
pub struct TIMER1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER1_RST_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
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
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral reset control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl_presetctrl1](index.html) module"]
pub struct PRESETCTRL_PRESETCTRL1_SPEC;
impl crate::RegisterSpec for PRESETCTRL_PRESETCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [presetctrl_presetctrl1::R](R) reader structure"]
impl crate::Readable for PRESETCTRL_PRESETCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presetctrl_presetctrl1::W](W) writer structure"]
impl crate::Writable for PRESETCTRL_PRESETCTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESETCTRL1 to value 0"]
impl crate::Resettable for PRESETCTRL_PRESETCTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
