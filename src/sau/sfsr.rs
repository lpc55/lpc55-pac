#[doc = "Register `SFSR` reader"]
pub struct R(crate::R<SFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFSR` writer"]
pub struct W(crate::W<SFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFSR_SPEC>;
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
impl From<crate::W<SFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Invalid entry point.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVEP_A {
    #[doc = "0: Error has not occurred."]
    NO_ERROR = 0,
    #[doc = "1: Error has occurred."]
    ERROR = 1,
}
impl From<INVEP_A> for bool {
    #[inline(always)]
    fn from(variant: INVEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVEP` reader - Invalid entry point."]
pub struct INVEP_R(crate::FieldReader<bool, INVEP_A>);
impl INVEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVEP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVEP_A {
        match self.bits {
            false => INVEP_A::NO_ERROR,
            true => INVEP_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == INVEP_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == INVEP_A::ERROR
    }
}
impl core::ops::Deref for INVEP_R {
    type Target = crate::FieldReader<bool, INVEP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVEP` writer - Invalid entry point."]
pub struct INVEP_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVEP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Error has not occurred."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(INVEP_A::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(INVEP_A::ERROR)
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
#[doc = "Invalid integrity signature flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVIS_A {
    #[doc = "0: Error has not occurred."]
    NO_ERROR = 0,
    #[doc = "1: Error has occurred."]
    ERROR = 1,
}
impl From<INVIS_A> for bool {
    #[inline(always)]
    fn from(variant: INVIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVIS` reader - Invalid integrity signature flag."]
pub struct INVIS_R(crate::FieldReader<bool, INVIS_A>);
impl INVIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVIS_A {
        match self.bits {
            false => INVIS_A::NO_ERROR,
            true => INVIS_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == INVIS_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == INVIS_A::ERROR
    }
}
impl core::ops::Deref for INVIS_R {
    type Target = crate::FieldReader<bool, INVIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVIS` writer - Invalid integrity signature flag."]
pub struct INVIS_W<'a> {
    w: &'a mut W,
}
impl<'a> INVIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Error has not occurred."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(INVIS_A::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(INVIS_A::ERROR)
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
#[doc = "Invalid exception return flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVER_A {
    #[doc = "0: Error has not occurred."]
    NO_ERROR = 0,
    #[doc = "1: Error has occurred."]
    ERROR = 1,
}
impl From<INVER_A> for bool {
    #[inline(always)]
    fn from(variant: INVER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVER` reader - Invalid exception return flag."]
pub struct INVER_R(crate::FieldReader<bool, INVER_A>);
impl INVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVER_A {
        match self.bits {
            false => INVER_A::NO_ERROR,
            true => INVER_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == INVER_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == INVER_A::ERROR
    }
}
impl core::ops::Deref for INVER_R {
    type Target = crate::FieldReader<bool, INVER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVER` writer - Invalid exception return flag."]
pub struct INVER_W<'a> {
    w: &'a mut W,
}
impl<'a> INVER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Error has not occurred."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(INVER_A::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(INVER_A::ERROR)
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
#[doc = "Attribution unit violation flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUVIOL_A {
    #[doc = "0: Error has not occurred."]
    NO_ERROR = 0,
    #[doc = "1: Error has occurred."]
    ERROR = 1,
}
impl From<AUVIOL_A> for bool {
    #[inline(always)]
    fn from(variant: AUVIOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUVIOL` reader - Attribution unit violation flag."]
pub struct AUVIOL_R(crate::FieldReader<bool, AUVIOL_A>);
impl AUVIOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUVIOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUVIOL_A {
        match self.bits {
            false => AUVIOL_A::NO_ERROR,
            true => AUVIOL_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == AUVIOL_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == AUVIOL_A::ERROR
    }
}
impl core::ops::Deref for AUVIOL_R {
    type Target = crate::FieldReader<bool, AUVIOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUVIOL` writer - Attribution unit violation flag."]
pub struct AUVIOL_W<'a> {
    w: &'a mut W,
}
impl<'a> AUVIOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUVIOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Error has not occurred."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(AUVIOL_A::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(AUVIOL_A::ERROR)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Invalid transition flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVTRAN_A {
    #[doc = "0: Error has not occurred."]
    NO_ERROR = 0,
    #[doc = "1: Error has occurred."]
    ERROR = 1,
}
impl From<INVTRAN_A> for bool {
    #[inline(always)]
    fn from(variant: INVTRAN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVTRAN` reader - Invalid transition flag."]
pub struct INVTRAN_R(crate::FieldReader<bool, INVTRAN_A>);
impl INVTRAN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVTRAN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVTRAN_A {
        match self.bits {
            false => INVTRAN_A::NO_ERROR,
            true => INVTRAN_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == INVTRAN_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == INVTRAN_A::ERROR
    }
}
impl core::ops::Deref for INVTRAN_R {
    type Target = crate::FieldReader<bool, INVTRAN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVTRAN` writer - Invalid transition flag."]
pub struct INVTRAN_W<'a> {
    w: &'a mut W,
}
impl<'a> INVTRAN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVTRAN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Error has not occurred."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(INVTRAN_A::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(INVTRAN_A::ERROR)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Lazy state preservation error flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPERR_A {
    #[doc = "0: Error has not occurred."]
    NO_ERROR = 0,
    #[doc = "1: Error has occurred."]
    ERROR = 1,
}
impl From<LSPERR_A> for bool {
    #[inline(always)]
    fn from(variant: LSPERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSPERR` reader - Lazy state preservation error flag."]
pub struct LSPERR_R(crate::FieldReader<bool, LSPERR_A>);
impl LSPERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSPERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSPERR_A {
        match self.bits {
            false => LSPERR_A::NO_ERROR,
            true => LSPERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == LSPERR_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == LSPERR_A::ERROR
    }
}
impl core::ops::Deref for LSPERR_R {
    type Target = crate::FieldReader<bool, LSPERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSPERR` writer - Lazy state preservation error flag."]
pub struct LSPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> LSPERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSPERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Error has not occurred."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(LSPERR_A::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(LSPERR_A::ERROR)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Secure fault address valid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFARVALID_A {
    #[doc = "0: SFAR content not valid."]
    NOT_VALID = 0,
    #[doc = "1: SFAR content valid."]
    VALID = 1,
}
impl From<SFARVALID_A> for bool {
    #[inline(always)]
    fn from(variant: SFARVALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFARVALID` reader - Secure fault address valid."]
pub struct SFARVALID_R(crate::FieldReader<bool, SFARVALID_A>);
impl SFARVALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SFARVALID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFARVALID_A {
        match self.bits {
            false => SFARVALID_A::NOT_VALID,
            true => SFARVALID_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        **self == SFARVALID_A::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        **self == SFARVALID_A::VALID
    }
}
impl core::ops::Deref for SFARVALID_R {
    type Target = crate::FieldReader<bool, SFARVALID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFARVALID` writer - Secure fault address valid."]
pub struct SFARVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> SFARVALID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SFARVALID_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SFAR content not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(SFARVALID_A::NOT_VALID)
    }
    #[doc = "SFAR content valid."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(SFARVALID_A::VALID)
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
#[doc = "Lazy state error flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSERR_A {
    #[doc = "0: Error has not occurred"]
    NO_ERROR = 0,
    #[doc = "1: Error has occurred."]
    ERROR = 1,
}
impl From<LSERR_A> for bool {
    #[inline(always)]
    fn from(variant: LSERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSERR` reader - Lazy state error flag."]
pub struct LSERR_R(crate::FieldReader<bool, LSERR_A>);
impl LSERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSERR_A {
        match self.bits {
            false => LSERR_A::NO_ERROR,
            true => LSERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == LSERR_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == LSERR_A::ERROR
    }
}
impl core::ops::Deref for LSERR_R {
    type Target = crate::FieldReader<bool, LSERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSERR` writer - Lazy state error flag."]
pub struct LSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> LSERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Error has not occurred"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(LSERR_A::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(LSERR_A::ERROR)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Invalid entry point."]
    #[inline(always)]
    pub fn invep(&self) -> INVEP_R {
        INVEP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Invalid integrity signature flag."]
    #[inline(always)]
    pub fn invis(&self) -> INVIS_R {
        INVIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Invalid exception return flag."]
    #[inline(always)]
    pub fn inver(&self) -> INVER_R {
        INVER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Attribution unit violation flag."]
    #[inline(always)]
    pub fn auviol(&self) -> AUVIOL_R {
        AUVIOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Invalid transition flag."]
    #[inline(always)]
    pub fn invtran(&self) -> INVTRAN_R {
        INVTRAN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Lazy state preservation error flag."]
    #[inline(always)]
    pub fn lsperr(&self) -> LSPERR_R {
        LSPERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Secure fault address valid."]
    #[inline(always)]
    pub fn sfarvalid(&self) -> SFARVALID_R {
        SFARVALID_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Lazy state error flag."]
    #[inline(always)]
    pub fn lserr(&self) -> LSERR_R {
        LSERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invalid entry point."]
    #[inline(always)]
    pub fn invep(&mut self) -> INVEP_W {
        INVEP_W { w: self }
    }
    #[doc = "Bit 1 - Invalid integrity signature flag."]
    #[inline(always)]
    pub fn invis(&mut self) -> INVIS_W {
        INVIS_W { w: self }
    }
    #[doc = "Bit 2 - Invalid exception return flag."]
    #[inline(always)]
    pub fn inver(&mut self) -> INVER_W {
        INVER_W { w: self }
    }
    #[doc = "Bit 3 - Attribution unit violation flag."]
    #[inline(always)]
    pub fn auviol(&mut self) -> AUVIOL_W {
        AUVIOL_W { w: self }
    }
    #[doc = "Bit 4 - Invalid transition flag."]
    #[inline(always)]
    pub fn invtran(&mut self) -> INVTRAN_W {
        INVTRAN_W { w: self }
    }
    #[doc = "Bit 5 - Lazy state preservation error flag."]
    #[inline(always)]
    pub fn lsperr(&mut self) -> LSPERR_W {
        LSPERR_W { w: self }
    }
    #[doc = "Bit 6 - Secure fault address valid."]
    #[inline(always)]
    pub fn sfarvalid(&mut self) -> SFARVALID_W {
        SFARVALID_W { w: self }
    }
    #[doc = "Bit 7 - Lazy state error flag."]
    #[inline(always)]
    pub fn lserr(&mut self) -> LSERR_W {
        LSERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfsr](index.html) module"]
pub struct SFSR_SPEC;
impl crate::RegisterSpec for SFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfsr::R](R) reader structure"]
impl crate::Readable for SFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfsr::W](W) writer structure"]
impl crate::Writable for SFSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SFSR to value 0"]
impl crate::Resettable for SFSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
