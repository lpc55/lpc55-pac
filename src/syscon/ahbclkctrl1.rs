#[doc = "Register `AHBCLKCTRL1` reader"]
pub struct R(crate::R<AHBCLKCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBCLKCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBCLKCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBCLKCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBCLKCTRL1` writer"]
pub struct W(crate::W<AHBCLKCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCLKCTRL1_SPEC>;
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
impl From<crate::W<AHBCLKCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCLKCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enables the clock for the MRT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRT_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<MRT_A> for bool {
    #[inline(always)]
    fn from(variant: MRT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRT` reader - Enables the clock for the MRT."]
pub struct MRT_R(crate::FieldReader<bool, MRT_A>);
impl MRT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MRT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRT_A {
        match self.bits {
            false => MRT_A::DISABLE,
            true => MRT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == MRT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == MRT_A::ENABLE
    }
}
impl core::ops::Deref for MRT_R {
    type Target = crate::FieldReader<bool, MRT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRT` writer - Enables the clock for the MRT."]
pub struct MRT_W<'a> {
    w: &'a mut W,
}
impl<'a> MRT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MRT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MRT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MRT_A::ENABLE)
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
#[doc = "Enables the clock for the OS Event Timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSTIMER_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<OSTIMER_A> for bool {
    #[inline(always)]
    fn from(variant: OSTIMER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSTIMER` reader - Enables the clock for the OS Event Timer."]
pub struct OSTIMER_R(crate::FieldReader<bool, OSTIMER_A>);
impl OSTIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSTIMER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSTIMER_A {
        match self.bits {
            false => OSTIMER_A::DISABLE,
            true => OSTIMER_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == OSTIMER_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == OSTIMER_A::ENABLE
    }
}
impl core::ops::Deref for OSTIMER_R {
    type Target = crate::FieldReader<bool, OSTIMER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSTIMER` writer - Enables the clock for the OS Event Timer."]
pub struct OSTIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> OSTIMER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSTIMER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OSTIMER_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OSTIMER_A::ENABLE)
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
#[doc = "Enables the clock for the SCT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<SCT_A> for bool {
    #[inline(always)]
    fn from(variant: SCT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCT` reader - Enables the clock for the SCT."]
pub struct SCT_R(crate::FieldReader<bool, SCT_A>);
impl SCT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT_A {
        match self.bits {
            false => SCT_A::DISABLE,
            true => SCT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SCT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SCT_A::ENABLE
    }
}
impl core::ops::Deref for SCT_R {
    type Target = crate::FieldReader<bool, SCT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCT` writer - Enables the clock for the SCT."]
pub struct SCT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCT_A::ENABLE)
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
#[doc = "Enables the clock for the UTICK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UTICK_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<UTICK_A> for bool {
    #[inline(always)]
    fn from(variant: UTICK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UTICK` reader - Enables the clock for the UTICK."]
pub struct UTICK_R(crate::FieldReader<bool, UTICK_A>);
impl UTICK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UTICK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTICK_A {
        match self.bits {
            false => UTICK_A::DISABLE,
            true => UTICK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == UTICK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == UTICK_A::ENABLE
    }
}
impl core::ops::Deref for UTICK_R {
    type Target = crate::FieldReader<bool, UTICK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTICK` writer - Enables the clock for the UTICK."]
pub struct UTICK_W<'a> {
    w: &'a mut W,
}
impl<'a> UTICK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UTICK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UTICK_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UTICK_A::ENABLE)
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
#[doc = "Enables the clock for the FC0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC0_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FC0_A> for bool {
    #[inline(always)]
    fn from(variant: FC0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC0` reader - Enables the clock for the FC0."]
pub struct FC0_R(crate::FieldReader<bool, FC0_A>);
impl FC0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FC0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC0_A {
        match self.bits {
            false => FC0_A::DISABLE,
            true => FC0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FC0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FC0_A::ENABLE
    }
}
impl core::ops::Deref for FC0_R {
    type Target = crate::FieldReader<bool, FC0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC0` writer - Enables the clock for the FC0."]
pub struct FC0_W<'a> {
    w: &'a mut W,
}
impl<'a> FC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC0_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC0_A::ENABLE)
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
#[doc = "Enables the clock for the FC1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC1_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FC1_A> for bool {
    #[inline(always)]
    fn from(variant: FC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC1` reader - Enables the clock for the FC1."]
pub struct FC1_R(crate::FieldReader<bool, FC1_A>);
impl FC1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FC1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC1_A {
        match self.bits {
            false => FC1_A::DISABLE,
            true => FC1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FC1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FC1_A::ENABLE
    }
}
impl core::ops::Deref for FC1_R {
    type Target = crate::FieldReader<bool, FC1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC1` writer - Enables the clock for the FC1."]
pub struct FC1_W<'a> {
    w: &'a mut W,
}
impl<'a> FC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC1_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC1_A::ENABLE)
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
#[doc = "Enables the clock for the FC2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC2_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FC2_A> for bool {
    #[inline(always)]
    fn from(variant: FC2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC2` reader - Enables the clock for the FC2."]
pub struct FC2_R(crate::FieldReader<bool, FC2_A>);
impl FC2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FC2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC2_A {
        match self.bits {
            false => FC2_A::DISABLE,
            true => FC2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FC2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FC2_A::ENABLE
    }
}
impl core::ops::Deref for FC2_R {
    type Target = crate::FieldReader<bool, FC2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC2` writer - Enables the clock for the FC2."]
pub struct FC2_W<'a> {
    w: &'a mut W,
}
impl<'a> FC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC2_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC2_A::ENABLE)
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
#[doc = "Enables the clock for the FC3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC3_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FC3_A> for bool {
    #[inline(always)]
    fn from(variant: FC3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC3` reader - Enables the clock for the FC3."]
pub struct FC3_R(crate::FieldReader<bool, FC3_A>);
impl FC3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FC3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC3_A {
        match self.bits {
            false => FC3_A::DISABLE,
            true => FC3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FC3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FC3_A::ENABLE
    }
}
impl core::ops::Deref for FC3_R {
    type Target = crate::FieldReader<bool, FC3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC3` writer - Enables the clock for the FC3."]
pub struct FC3_W<'a> {
    w: &'a mut W,
}
impl<'a> FC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC3_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC3_A::ENABLE)
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
#[doc = "Enables the clock for the FC4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC4_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FC4_A> for bool {
    #[inline(always)]
    fn from(variant: FC4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC4` reader - Enables the clock for the FC4."]
pub struct FC4_R(crate::FieldReader<bool, FC4_A>);
impl FC4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FC4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC4_A {
        match self.bits {
            false => FC4_A::DISABLE,
            true => FC4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FC4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FC4_A::ENABLE
    }
}
impl core::ops::Deref for FC4_R {
    type Target = crate::FieldReader<bool, FC4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC4` writer - Enables the clock for the FC4."]
pub struct FC4_W<'a> {
    w: &'a mut W,
}
impl<'a> FC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC4_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC4_A::ENABLE)
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
#[doc = "Enables the clock for the FC5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC5_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FC5_A> for bool {
    #[inline(always)]
    fn from(variant: FC5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC5` reader - Enables the clock for the FC5."]
pub struct FC5_R(crate::FieldReader<bool, FC5_A>);
impl FC5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FC5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC5_A {
        match self.bits {
            false => FC5_A::DISABLE,
            true => FC5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FC5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FC5_A::ENABLE
    }
}
impl core::ops::Deref for FC5_R {
    type Target = crate::FieldReader<bool, FC5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC5` writer - Enables the clock for the FC5."]
pub struct FC5_W<'a> {
    w: &'a mut W,
}
impl<'a> FC5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC5_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC5_A::ENABLE)
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
#[doc = "Enables the clock for the FC6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC6_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FC6_A> for bool {
    #[inline(always)]
    fn from(variant: FC6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC6` reader - Enables the clock for the FC6."]
pub struct FC6_R(crate::FieldReader<bool, FC6_A>);
impl FC6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FC6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC6_A {
        match self.bits {
            false => FC6_A::DISABLE,
            true => FC6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FC6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FC6_A::ENABLE
    }
}
impl core::ops::Deref for FC6_R {
    type Target = crate::FieldReader<bool, FC6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC6` writer - Enables the clock for the FC6."]
pub struct FC6_W<'a> {
    w: &'a mut W,
}
impl<'a> FC6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC6_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC6_A::ENABLE)
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
#[doc = "Enables the clock for the FC7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC7_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FC7_A> for bool {
    #[inline(always)]
    fn from(variant: FC7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC7` reader - Enables the clock for the FC7."]
pub struct FC7_R(crate::FieldReader<bool, FC7_A>);
impl FC7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FC7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC7_A {
        match self.bits {
            false => FC7_A::DISABLE,
            true => FC7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FC7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FC7_A::ENABLE
    }
}
impl core::ops::Deref for FC7_R {
    type Target = crate::FieldReader<bool, FC7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC7` writer - Enables the clock for the FC7."]
pub struct FC7_W<'a> {
    w: &'a mut W,
}
impl<'a> FC7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC7_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC7_A::ENABLE)
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
#[doc = "Enables the clock for the Timer 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER2_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<TIMER2_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER2` reader - Enables the clock for the Timer 2."]
pub struct TIMER2_R(crate::FieldReader<bool, TIMER2_A>);
impl TIMER2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER2_A {
        match self.bits {
            false => TIMER2_A::DISABLE,
            true => TIMER2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TIMER2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TIMER2_A::ENABLE
    }
}
impl core::ops::Deref for TIMER2_R {
    type Target = crate::FieldReader<bool, TIMER2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER2` writer - Enables the clock for the Timer 2."]
pub struct TIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER2_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER2_A::ENABLE)
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
#[doc = "Enables the clock for the USB0 DEV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_DEV_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<USB0_DEV_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_DEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB0_DEV` reader - Enables the clock for the USB0 DEV."]
pub struct USB0_DEV_R(crate::FieldReader<bool, USB0_DEV_A>);
impl USB0_DEV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB0_DEV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_DEV_A {
        match self.bits {
            false => USB0_DEV_A::DISABLE,
            true => USB0_DEV_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == USB0_DEV_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == USB0_DEV_A::ENABLE
    }
}
impl core::ops::Deref for USB0_DEV_R {
    type Target = crate::FieldReader<bool, USB0_DEV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB0_DEV` writer - Enables the clock for the USB0 DEV."]
pub struct USB0_DEV_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_DEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB0_DEV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB0_DEV_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB0_DEV_A::ENABLE)
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
#[doc = "Enables the clock for the Timer 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER0_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<TIMER0_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER0` reader - Enables the clock for the Timer 0."]
pub struct TIMER0_R(crate::FieldReader<bool, TIMER0_A>);
impl TIMER0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER0_A {
        match self.bits {
            false => TIMER0_A::DISABLE,
            true => TIMER0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TIMER0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TIMER0_A::ENABLE
    }
}
impl core::ops::Deref for TIMER0_R {
    type Target = crate::FieldReader<bool, TIMER0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER0` writer - Enables the clock for the Timer 0."]
pub struct TIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER0_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER0_A::ENABLE)
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
#[doc = "Enables the clock for the Timer 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER1_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<TIMER1_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER1` reader - Enables the clock for the Timer 1."]
pub struct TIMER1_R(crate::FieldReader<bool, TIMER1_A>);
impl TIMER1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER1_A {
        match self.bits {
            false => TIMER1_A::DISABLE,
            true => TIMER1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TIMER1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TIMER1_A::ENABLE
    }
}
impl core::ops::Deref for TIMER1_R {
    type Target = crate::FieldReader<bool, TIMER1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1` writer - Enables the clock for the Timer 1."]
pub struct TIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER1_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER1_A::ENABLE)
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
    #[doc = "Bit 0 - Enables the clock for the MRT."]
    #[inline(always)]
    pub fn mrt(&self) -> MRT_R {
        MRT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables the clock for the OS Event Timer."]
    #[inline(always)]
    pub fn ostimer(&self) -> OSTIMER_R {
        OSTIMER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables the clock for the SCT."]
    #[inline(always)]
    pub fn sct(&self) -> SCT_R {
        SCT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enables the clock for the UTICK."]
    #[inline(always)]
    pub fn utick(&self) -> UTICK_R {
        UTICK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for the FC0."]
    #[inline(always)]
    pub fn fc0(&self) -> FC0_R {
        FC0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enables the clock for the FC1."]
    #[inline(always)]
    pub fn fc1(&self) -> FC1_R {
        FC1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for the FC2."]
    #[inline(always)]
    pub fn fc2(&self) -> FC2_R {
        FC2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enables the clock for the FC3."]
    #[inline(always)]
    pub fn fc3(&self) -> FC3_R {
        FC3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enables the clock for the FC4."]
    #[inline(always)]
    pub fn fc4(&self) -> FC4_R {
        FC4_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for the FC5."]
    #[inline(always)]
    pub fn fc5(&self) -> FC5_R {
        FC5_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for the FC6."]
    #[inline(always)]
    pub fn fc6(&self) -> FC6_R {
        FC6_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for the FC7."]
    #[inline(always)]
    pub fn fc7(&self) -> FC7_R {
        FC7_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enables the clock for the Timer 2."]
    #[inline(always)]
    pub fn timer2(&self) -> TIMER2_R {
        TIMER2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enables the clock for the USB0 DEV."]
    #[inline(always)]
    pub fn usb0_dev(&self) -> USB0_DEV_R {
        USB0_DEV_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enables the clock for the Timer 0."]
    #[inline(always)]
    pub fn timer0(&self) -> TIMER0_R {
        TIMER0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enables the clock for the Timer 1."]
    #[inline(always)]
    pub fn timer1(&self) -> TIMER1_R {
        TIMER1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the clock for the MRT."]
    #[inline(always)]
    pub fn mrt(&mut self) -> MRT_W {
        MRT_W { w: self }
    }
    #[doc = "Bit 1 - Enables the clock for the OS Event Timer."]
    #[inline(always)]
    pub fn ostimer(&mut self) -> OSTIMER_W {
        OSTIMER_W { w: self }
    }
    #[doc = "Bit 2 - Enables the clock for the SCT."]
    #[inline(always)]
    pub fn sct(&mut self) -> SCT_W {
        SCT_W { w: self }
    }
    #[doc = "Bit 10 - Enables the clock for the UTICK."]
    #[inline(always)]
    pub fn utick(&mut self) -> UTICK_W {
        UTICK_W { w: self }
    }
    #[doc = "Bit 11 - Enables the clock for the FC0."]
    #[inline(always)]
    pub fn fc0(&mut self) -> FC0_W {
        FC0_W { w: self }
    }
    #[doc = "Bit 12 - Enables the clock for the FC1."]
    #[inline(always)]
    pub fn fc1(&mut self) -> FC1_W {
        FC1_W { w: self }
    }
    #[doc = "Bit 13 - Enables the clock for the FC2."]
    #[inline(always)]
    pub fn fc2(&mut self) -> FC2_W {
        FC2_W { w: self }
    }
    #[doc = "Bit 14 - Enables the clock for the FC3."]
    #[inline(always)]
    pub fn fc3(&mut self) -> FC3_W {
        FC3_W { w: self }
    }
    #[doc = "Bit 15 - Enables the clock for the FC4."]
    #[inline(always)]
    pub fn fc4(&mut self) -> FC4_W {
        FC4_W { w: self }
    }
    #[doc = "Bit 16 - Enables the clock for the FC5."]
    #[inline(always)]
    pub fn fc5(&mut self) -> FC5_W {
        FC5_W { w: self }
    }
    #[doc = "Bit 17 - Enables the clock for the FC6."]
    #[inline(always)]
    pub fn fc6(&mut self) -> FC6_W {
        FC6_W { w: self }
    }
    #[doc = "Bit 18 - Enables the clock for the FC7."]
    #[inline(always)]
    pub fn fc7(&mut self) -> FC7_W {
        FC7_W { w: self }
    }
    #[doc = "Bit 22 - Enables the clock for the Timer 2."]
    #[inline(always)]
    pub fn timer2(&mut self) -> TIMER2_W {
        TIMER2_W { w: self }
    }
    #[doc = "Bit 25 - Enables the clock for the USB0 DEV."]
    #[inline(always)]
    pub fn usb0_dev(&mut self) -> USB0_DEV_W {
        USB0_DEV_W { w: self }
    }
    #[doc = "Bit 26 - Enables the clock for the Timer 0."]
    #[inline(always)]
    pub fn timer0(&mut self) -> TIMER0_W {
        TIMER0_W { w: self }
    }
    #[doc = "Bit 27 - Enables the clock for the Timer 1."]
    #[inline(always)]
    pub fn timer1(&mut self) -> TIMER1_W {
        TIMER1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Clock control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkctrl1](index.html) module"]
pub struct AHBCLKCTRL1_SPEC;
impl crate::RegisterSpec for AHBCLKCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbclkctrl1::R](R) reader structure"]
impl crate::Readable for AHBCLKCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbclkctrl1::W](W) writer structure"]
impl crate::Writable for AHBCLKCTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBCLKCTRL1 to value 0"]
impl crate::Resettable for AHBCLKCTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
