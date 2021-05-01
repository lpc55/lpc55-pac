#[doc = "Register `CPPWR` reader"]
pub struct R(crate::R<CPPWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPPWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CPPWR_SPEC>> for R {
    fn from(reader: crate::R<CPPWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPPWR` writer"]
pub struct W(crate::W<CPPWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPPWR_SPEC>;
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
impl core::convert::From<crate::W<CPPWR_SPEC>> for W {
    fn from(writer: crate::W<CPPWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "State UNKNOWN 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SU0_A {
    #[doc = "0: The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0,
    #[doc = "1: The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 1,
}
impl From<SU0_A> for bool {
    #[inline(always)]
    fn from(variant: SU0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SU0` reader - State UNKNOWN 0."]
pub struct SU0_R(crate::FieldReader<bool, SU0_A>);
impl SU0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SU0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SU0_A {
        match self.bits {
            false => SU0_A::UNKNOWN_NOT_PERMITTED,
            true => SU0_A::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_not_permitted(&self) -> bool {
        **self == SU0_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        **self == SU0_A::UNKNOWN_PERMITTED
    }
}
impl core::ops::Deref for SU0_R {
    type Target = crate::FieldReader<bool, SU0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SU0` writer - State UNKNOWN 0."]
pub struct SU0_W<'a> {
    w: &'a mut W,
}
impl<'a> SU0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SU0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU0_A::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU0_A::UNKNOWN_PERMITTED)
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
#[doc = "State UNKNOWN Secure only 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUS0_A {
    #[doc = "0: The SU0 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0,
    #[doc = "1: The SU0 field is only accessible from the Secure state."]
    SECURE_ONLY = 1,
}
impl From<SUS0_A> for bool {
    #[inline(always)]
    fn from(variant: SUS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUS0` reader - State UNKNOWN Secure only 0."]
pub struct SUS0_R(crate::FieldReader<bool, SUS0_A>);
impl SUS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUS0_A {
        match self.bits {
            false => SUS0_A::SECURE_AND_NON_SECURE,
            true => SUS0_A::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline(always)]
    pub fn is_secure_and_non_secure(&self) -> bool {
        **self == SUS0_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        **self == SUS0_A::SECURE_ONLY
    }
}
impl core::ops::Deref for SUS0_R {
    type Target = crate::FieldReader<bool, SUS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUS0` writer - State UNKNOWN Secure only 0."]
pub struct SUS0_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The SU0 field is accessible from both Security states."]
    #[inline(always)]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS0_A::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU0 field is only accessible from the Secure state."]
    #[inline(always)]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS0_A::SECURE_ONLY)
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
#[doc = "State UNKNOWN 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SU1_A {
    #[doc = "0: The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0,
    #[doc = "1: The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 1,
}
impl From<SU1_A> for bool {
    #[inline(always)]
    fn from(variant: SU1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SU1` reader - State UNKNOWN 1."]
pub struct SU1_R(crate::FieldReader<bool, SU1_A>);
impl SU1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SU1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SU1_A {
        match self.bits {
            false => SU1_A::UNKNOWN_NOT_PERMITTED,
            true => SU1_A::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_not_permitted(&self) -> bool {
        **self == SU1_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        **self == SU1_A::UNKNOWN_PERMITTED
    }
}
impl core::ops::Deref for SU1_R {
    type Target = crate::FieldReader<bool, SU1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SU1` writer - State UNKNOWN 1."]
pub struct SU1_W<'a> {
    w: &'a mut W,
}
impl<'a> SU1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SU1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU1_A::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU1_A::UNKNOWN_PERMITTED)
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
#[doc = "State UNKNOWN Secure only 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUS1_A {
    #[doc = "0: The SU7 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0,
    #[doc = "1: The SU7 field is only accessible from the Secure state."]
    SECURE_ONLY = 1,
}
impl From<SUS1_A> for bool {
    #[inline(always)]
    fn from(variant: SUS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUS1` reader - State UNKNOWN Secure only 1."]
pub struct SUS1_R(crate::FieldReader<bool, SUS1_A>);
impl SUS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUS1_A {
        match self.bits {
            false => SUS1_A::SECURE_AND_NON_SECURE,
            true => SUS1_A::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline(always)]
    pub fn is_secure_and_non_secure(&self) -> bool {
        **self == SUS1_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        **self == SUS1_A::SECURE_ONLY
    }
}
impl core::ops::Deref for SUS1_R {
    type Target = crate::FieldReader<bool, SUS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUS1` writer - State UNKNOWN Secure only 1."]
pub struct SUS1_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The SU7 field is accessible from both Security states."]
    #[inline(always)]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS1_A::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU7 field is only accessible from the Secure state."]
    #[inline(always)]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS1_A::SECURE_ONLY)
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
#[doc = "State UNKNOWN 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SU2_A {
    #[doc = "0: The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0,
    #[doc = "1: The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 1,
}
impl From<SU2_A> for bool {
    #[inline(always)]
    fn from(variant: SU2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SU2` reader - State UNKNOWN 2."]
pub struct SU2_R(crate::FieldReader<bool, SU2_A>);
impl SU2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SU2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SU2_A {
        match self.bits {
            false => SU2_A::UNKNOWN_NOT_PERMITTED,
            true => SU2_A::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_not_permitted(&self) -> bool {
        **self == SU2_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        **self == SU2_A::UNKNOWN_PERMITTED
    }
}
impl core::ops::Deref for SU2_R {
    type Target = crate::FieldReader<bool, SU2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SU2` writer - State UNKNOWN 2."]
pub struct SU2_W<'a> {
    w: &'a mut W,
}
impl<'a> SU2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SU2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU2_A::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU2_A::UNKNOWN_PERMITTED)
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
#[doc = "State UNKNOWN Secure only 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUS2_A {
    #[doc = "0: The SU2 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0,
    #[doc = "1: The SU2 field is only accessible from the Secure state."]
    SECURE_ONLY = 1,
}
impl From<SUS2_A> for bool {
    #[inline(always)]
    fn from(variant: SUS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUS2` reader - State UNKNOWN Secure only 2."]
pub struct SUS2_R(crate::FieldReader<bool, SUS2_A>);
impl SUS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUS2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUS2_A {
        match self.bits {
            false => SUS2_A::SECURE_AND_NON_SECURE,
            true => SUS2_A::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline(always)]
    pub fn is_secure_and_non_secure(&self) -> bool {
        **self == SUS2_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        **self == SUS2_A::SECURE_ONLY
    }
}
impl core::ops::Deref for SUS2_R {
    type Target = crate::FieldReader<bool, SUS2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUS2` writer - State UNKNOWN Secure only 2."]
pub struct SUS2_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The SU2 field is accessible from both Security states."]
    #[inline(always)]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS2_A::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU2 field is only accessible from the Secure state."]
    #[inline(always)]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS2_A::SECURE_ONLY)
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
#[doc = "State UNKNOWN 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SU3_A {
    #[doc = "0: The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0,
    #[doc = "1: The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 1,
}
impl From<SU3_A> for bool {
    #[inline(always)]
    fn from(variant: SU3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SU3` reader - State UNKNOWN 3."]
pub struct SU3_R(crate::FieldReader<bool, SU3_A>);
impl SU3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SU3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SU3_A {
        match self.bits {
            false => SU3_A::UNKNOWN_NOT_PERMITTED,
            true => SU3_A::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_not_permitted(&self) -> bool {
        **self == SU3_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        **self == SU3_A::UNKNOWN_PERMITTED
    }
}
impl core::ops::Deref for SU3_R {
    type Target = crate::FieldReader<bool, SU3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SU3` writer - State UNKNOWN 3."]
pub struct SU3_W<'a> {
    w: &'a mut W,
}
impl<'a> SU3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SU3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU3_A::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU3_A::UNKNOWN_PERMITTED)
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
#[doc = "State UNKNOWN Secure only 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUS3_A {
    #[doc = "0: The SU3 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0,
    #[doc = "1: The SU3 field is only accessible from the Secure state."]
    SECURE_ONLY = 1,
}
impl From<SUS3_A> for bool {
    #[inline(always)]
    fn from(variant: SUS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUS3` reader - State UNKNOWN Secure only 3."]
pub struct SUS3_R(crate::FieldReader<bool, SUS3_A>);
impl SUS3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUS3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUS3_A {
        match self.bits {
            false => SUS3_A::SECURE_AND_NON_SECURE,
            true => SUS3_A::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline(always)]
    pub fn is_secure_and_non_secure(&self) -> bool {
        **self == SUS3_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        **self == SUS3_A::SECURE_ONLY
    }
}
impl core::ops::Deref for SUS3_R {
    type Target = crate::FieldReader<bool, SUS3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUS3` writer - State UNKNOWN Secure only 3."]
pub struct SUS3_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The SU3 field is accessible from both Security states."]
    #[inline(always)]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS3_A::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU3 field is only accessible from the Secure state."]
    #[inline(always)]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS3_A::SECURE_ONLY)
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
#[doc = "State UNKNOWN 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SU4_A {
    #[doc = "0: The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0,
    #[doc = "1: The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 1,
}
impl From<SU4_A> for bool {
    #[inline(always)]
    fn from(variant: SU4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SU4` reader - State UNKNOWN 4."]
pub struct SU4_R(crate::FieldReader<bool, SU4_A>);
impl SU4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SU4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SU4_A {
        match self.bits {
            false => SU4_A::UNKNOWN_NOT_PERMITTED,
            true => SU4_A::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_not_permitted(&self) -> bool {
        **self == SU4_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        **self == SU4_A::UNKNOWN_PERMITTED
    }
}
impl core::ops::Deref for SU4_R {
    type Target = crate::FieldReader<bool, SU4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SU4` writer - State UNKNOWN 4."]
pub struct SU4_W<'a> {
    w: &'a mut W,
}
impl<'a> SU4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SU4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU4_A::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU4_A::UNKNOWN_PERMITTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "State UNKNOWN Secure only 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUS4_A {
    #[doc = "0: The SU4 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0,
    #[doc = "1: The SU4 field is only accessible from the Secure state."]
    SECURE_ONLY = 1,
}
impl From<SUS4_A> for bool {
    #[inline(always)]
    fn from(variant: SUS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUS4` reader - State UNKNOWN Secure only 4."]
pub struct SUS4_R(crate::FieldReader<bool, SUS4_A>);
impl SUS4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUS4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUS4_A {
        match self.bits {
            false => SUS4_A::SECURE_AND_NON_SECURE,
            true => SUS4_A::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline(always)]
    pub fn is_secure_and_non_secure(&self) -> bool {
        **self == SUS4_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        **self == SUS4_A::SECURE_ONLY
    }
}
impl core::ops::Deref for SUS4_R {
    type Target = crate::FieldReader<bool, SUS4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUS4` writer - State UNKNOWN Secure only 4."]
pub struct SUS4_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The SU4 field is accessible from both Security states."]
    #[inline(always)]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS4_A::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU4 field is only accessible from the Secure state."]
    #[inline(always)]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS4_A::SECURE_ONLY)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "State UNKNOWN 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SU5_A {
    #[doc = "0: The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0,
    #[doc = "1: The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 1,
}
impl From<SU5_A> for bool {
    #[inline(always)]
    fn from(variant: SU5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SU5` reader - State UNKNOWN 5."]
pub struct SU5_R(crate::FieldReader<bool, SU5_A>);
impl SU5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SU5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SU5_A {
        match self.bits {
            false => SU5_A::UNKNOWN_NOT_PERMITTED,
            true => SU5_A::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_not_permitted(&self) -> bool {
        **self == SU5_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        **self == SU5_A::UNKNOWN_PERMITTED
    }
}
impl core::ops::Deref for SU5_R {
    type Target = crate::FieldReader<bool, SU5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SU5` writer - State UNKNOWN 5."]
pub struct SU5_W<'a> {
    w: &'a mut W,
}
impl<'a> SU5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SU5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU5_A::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU5_A::UNKNOWN_PERMITTED)
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
#[doc = "State UNKNOWN Secure only 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUS5_A {
    #[doc = "0: The SU5 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0,
    #[doc = "1: The SU5 field is only accessible from the Secure state."]
    SECURE_ONLY = 1,
}
impl From<SUS5_A> for bool {
    #[inline(always)]
    fn from(variant: SUS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUS5` reader - State UNKNOWN Secure only 5."]
pub struct SUS5_R(crate::FieldReader<bool, SUS5_A>);
impl SUS5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUS5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUS5_A {
        match self.bits {
            false => SUS5_A::SECURE_AND_NON_SECURE,
            true => SUS5_A::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline(always)]
    pub fn is_secure_and_non_secure(&self) -> bool {
        **self == SUS5_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        **self == SUS5_A::SECURE_ONLY
    }
}
impl core::ops::Deref for SUS5_R {
    type Target = crate::FieldReader<bool, SUS5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUS5` writer - State UNKNOWN Secure only 5."]
pub struct SUS5_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The SU5 field is accessible from both Security states."]
    #[inline(always)]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS5_A::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU5 field is only accessible from the Secure state."]
    #[inline(always)]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS5_A::SECURE_ONLY)
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
#[doc = "State UNKNOWN 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SU6_A {
    #[doc = "0: The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0,
    #[doc = "1: The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 1,
}
impl From<SU6_A> for bool {
    #[inline(always)]
    fn from(variant: SU6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SU6` reader - State UNKNOWN 6."]
pub struct SU6_R(crate::FieldReader<bool, SU6_A>);
impl SU6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SU6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SU6_A {
        match self.bits {
            false => SU6_A::UNKNOWN_NOT_PERMITTED,
            true => SU6_A::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_not_permitted(&self) -> bool {
        **self == SU6_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        **self == SU6_A::UNKNOWN_PERMITTED
    }
}
impl core::ops::Deref for SU6_R {
    type Target = crate::FieldReader<bool, SU6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SU6` writer - State UNKNOWN 6."]
pub struct SU6_W<'a> {
    w: &'a mut W,
}
impl<'a> SU6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SU6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU6_A::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU6_A::UNKNOWN_PERMITTED)
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
#[doc = "State UNKNOWN Secure only 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUS6_A {
    #[doc = "0: The SU6 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0,
    #[doc = "1: The SU6 field is only accessible from the Secure state."]
    SECURE_ONLY = 1,
}
impl From<SUS6_A> for bool {
    #[inline(always)]
    fn from(variant: SUS6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUS6` reader - State UNKNOWN Secure only 6."]
pub struct SUS6_R(crate::FieldReader<bool, SUS6_A>);
impl SUS6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUS6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUS6_A {
        match self.bits {
            false => SUS6_A::SECURE_AND_NON_SECURE,
            true => SUS6_A::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline(always)]
    pub fn is_secure_and_non_secure(&self) -> bool {
        **self == SUS6_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        **self == SUS6_A::SECURE_ONLY
    }
}
impl core::ops::Deref for SUS6_R {
    type Target = crate::FieldReader<bool, SUS6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUS6` writer - State UNKNOWN Secure only 6."]
pub struct SUS6_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The SU6 field is accessible from both Security states."]
    #[inline(always)]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS6_A::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU6 field is only accessible from the Secure state."]
    #[inline(always)]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS6_A::SECURE_ONLY)
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
#[doc = "State UNKNOWN 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SU7_A {
    #[doc = "0: The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0,
    #[doc = "1: The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 1,
}
impl From<SU7_A> for bool {
    #[inline(always)]
    fn from(variant: SU7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SU7` reader - State UNKNOWN 7."]
pub struct SU7_R(crate::FieldReader<bool, SU7_A>);
impl SU7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SU7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SU7_A {
        match self.bits {
            false => SU7_A::UNKNOWN_NOT_PERMITTED,
            true => SU7_A::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_not_permitted(&self) -> bool {
        **self == SU7_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        **self == SU7_A::UNKNOWN_PERMITTED
    }
}
impl core::ops::Deref for SU7_R {
    type Target = crate::FieldReader<bool, SU7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SU7` writer - State UNKNOWN 7."]
pub struct SU7_W<'a> {
    w: &'a mut W,
}
impl<'a> SU7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SU7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU7_A::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU7_A::UNKNOWN_PERMITTED)
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
#[doc = "State UNKNOWN Secure only 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUS7_A {
    #[doc = "0: The SU7 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0,
    #[doc = "1: The SU7 field is only accessible from the Secure state."]
    SECURE_ONLY = 1,
}
impl From<SUS7_A> for bool {
    #[inline(always)]
    fn from(variant: SUS7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUS7` reader - State UNKNOWN Secure only 7."]
pub struct SUS7_R(crate::FieldReader<bool, SUS7_A>);
impl SUS7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUS7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUS7_A {
        match self.bits {
            false => SUS7_A::SECURE_AND_NON_SECURE,
            true => SUS7_A::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline(always)]
    pub fn is_secure_and_non_secure(&self) -> bool {
        **self == SUS7_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        **self == SUS7_A::SECURE_ONLY
    }
}
impl core::ops::Deref for SUS7_R {
    type Target = crate::FieldReader<bool, SUS7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUS7` writer - State UNKNOWN Secure only 7."]
pub struct SUS7_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The SU7 field is accessible from both Security states."]
    #[inline(always)]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS7_A::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU7 field is only accessible from the Secure state."]
    #[inline(always)]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS7_A::SECURE_ONLY)
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
#[doc = "State UNKNOWN 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SU10_A {
    #[doc = "0: The floating-point state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0,
    #[doc = "1: The floating-point state is permitted to become UNKNOWN"]
    UNKNOWN_PERMITTED = 1,
}
impl From<SU10_A> for bool {
    #[inline(always)]
    fn from(variant: SU10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SU10` reader - State UNKNOWN 10."]
pub struct SU10_R(crate::FieldReader<bool, SU10_A>);
impl SU10_R {
    pub(crate) fn new(bits: bool) -> Self {
        SU10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SU10_A {
        match self.bits {
            false => SU10_A::UNKNOWN_NOT_PERMITTED,
            true => SU10_A::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_not_permitted(&self) -> bool {
        **self == SU10_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        **self == SU10_A::UNKNOWN_PERMITTED
    }
}
impl core::ops::Deref for SU10_R {
    type Target = crate::FieldReader<bool, SU10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SU10` writer - State UNKNOWN 10."]
pub struct SU10_W<'a> {
    w: &'a mut W,
}
impl<'a> SU10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SU10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The floating-point state is not permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU10_A::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The floating-point state is permitted to become UNKNOWN"]
    #[inline(always)]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU10_A::UNKNOWN_PERMITTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "State UNKNOWN Secure only 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUS10_A {
    #[doc = "0: The SU10 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0,
    #[doc = "1: The SU10 field is only accessible from the Secure state."]
    SECURE_ONLY = 1,
}
impl From<SUS10_A> for bool {
    #[inline(always)]
    fn from(variant: SUS10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUS10` reader - State UNKNOWN Secure only 10."]
pub struct SUS10_R(crate::FieldReader<bool, SUS10_A>);
impl SUS10_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUS10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUS10_A {
        match self.bits {
            false => SUS10_A::SECURE_AND_NON_SECURE,
            true => SUS10_A::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline(always)]
    pub fn is_secure_and_non_secure(&self) -> bool {
        **self == SUS10_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        **self == SUS10_A::SECURE_ONLY
    }
}
impl core::ops::Deref for SUS10_R {
    type Target = crate::FieldReader<bool, SUS10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUS10` writer - State UNKNOWN Secure only 10."]
pub struct SUS10_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The SU10 field is accessible from both Security states."]
    #[inline(always)]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS10_A::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU10 field is only accessible from the Secure state."]
    #[inline(always)]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS10_A::SECURE_ONLY)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `SU11` reader - State UNKNOWN 11."]
pub struct SU11_R(crate::FieldReader<bool, bool>);
impl SU11_R {
    pub(crate) fn new(bits: bool) -> Self {
        SU11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SU11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SU11` writer - State UNKNOWN 11."]
pub struct SU11_W<'a> {
    w: &'a mut W,
}
impl<'a> SU11_W<'a> {
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
#[doc = "Field `SUS11` reader - State UNKNOWN Secure only 11."]
pub struct SUS11_R(crate::FieldReader<bool, bool>);
impl SUS11_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUS11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUS11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUS11` writer - State UNKNOWN Secure only 11."]
pub struct SUS11_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - State UNKNOWN 0."]
    #[inline(always)]
    pub fn su0(&self) -> SU0_R {
        SU0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - State UNKNOWN Secure only 0."]
    #[inline(always)]
    pub fn sus0(&self) -> SUS0_R {
        SUS0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - State UNKNOWN 1."]
    #[inline(always)]
    pub fn su1(&self) -> SU1_R {
        SU1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - State UNKNOWN Secure only 1."]
    #[inline(always)]
    pub fn sus1(&self) -> SUS1_R {
        SUS1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - State UNKNOWN 2."]
    #[inline(always)]
    pub fn su2(&self) -> SU2_R {
        SU2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - State UNKNOWN Secure only 2."]
    #[inline(always)]
    pub fn sus2(&self) -> SUS2_R {
        SUS2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - State UNKNOWN 3."]
    #[inline(always)]
    pub fn su3(&self) -> SU3_R {
        SU3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - State UNKNOWN Secure only 3."]
    #[inline(always)]
    pub fn sus3(&self) -> SUS3_R {
        SUS3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - State UNKNOWN 4."]
    #[inline(always)]
    pub fn su4(&self) -> SU4_R {
        SU4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - State UNKNOWN Secure only 4."]
    #[inline(always)]
    pub fn sus4(&self) -> SUS4_R {
        SUS4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - State UNKNOWN 5."]
    #[inline(always)]
    pub fn su5(&self) -> SU5_R {
        SU5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - State UNKNOWN Secure only 5."]
    #[inline(always)]
    pub fn sus5(&self) -> SUS5_R {
        SUS5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - State UNKNOWN 6."]
    #[inline(always)]
    pub fn su6(&self) -> SU6_R {
        SU6_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - State UNKNOWN Secure only 6."]
    #[inline(always)]
    pub fn sus6(&self) -> SUS6_R {
        SUS6_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - State UNKNOWN 7."]
    #[inline(always)]
    pub fn su7(&self) -> SU7_R {
        SU7_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - State UNKNOWN Secure only 7."]
    #[inline(always)]
    pub fn sus7(&self) -> SUS7_R {
        SUS7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 20 - State UNKNOWN 10."]
    #[inline(always)]
    pub fn su10(&self) -> SU10_R {
        SU10_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - State UNKNOWN Secure only 10."]
    #[inline(always)]
    pub fn sus10(&self) -> SUS10_R {
        SUS10_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - State UNKNOWN 11."]
    #[inline(always)]
    pub fn su11(&self) -> SU11_R {
        SU11_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - State UNKNOWN Secure only 11."]
    #[inline(always)]
    pub fn sus11(&self) -> SUS11_R {
        SUS11_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - State UNKNOWN 0."]
    #[inline(always)]
    pub fn su0(&mut self) -> SU0_W {
        SU0_W { w: self }
    }
    #[doc = "Bit 1 - State UNKNOWN Secure only 0."]
    #[inline(always)]
    pub fn sus0(&mut self) -> SUS0_W {
        SUS0_W { w: self }
    }
    #[doc = "Bit 2 - State UNKNOWN 1."]
    #[inline(always)]
    pub fn su1(&mut self) -> SU1_W {
        SU1_W { w: self }
    }
    #[doc = "Bit 3 - State UNKNOWN Secure only 1."]
    #[inline(always)]
    pub fn sus1(&mut self) -> SUS1_W {
        SUS1_W { w: self }
    }
    #[doc = "Bit 4 - State UNKNOWN 2."]
    #[inline(always)]
    pub fn su2(&mut self) -> SU2_W {
        SU2_W { w: self }
    }
    #[doc = "Bit 5 - State UNKNOWN Secure only 2."]
    #[inline(always)]
    pub fn sus2(&mut self) -> SUS2_W {
        SUS2_W { w: self }
    }
    #[doc = "Bit 6 - State UNKNOWN 3."]
    #[inline(always)]
    pub fn su3(&mut self) -> SU3_W {
        SU3_W { w: self }
    }
    #[doc = "Bit 7 - State UNKNOWN Secure only 3."]
    #[inline(always)]
    pub fn sus3(&mut self) -> SUS3_W {
        SUS3_W { w: self }
    }
    #[doc = "Bit 8 - State UNKNOWN 4."]
    #[inline(always)]
    pub fn su4(&mut self) -> SU4_W {
        SU4_W { w: self }
    }
    #[doc = "Bit 9 - State UNKNOWN Secure only 4."]
    #[inline(always)]
    pub fn sus4(&mut self) -> SUS4_W {
        SUS4_W { w: self }
    }
    #[doc = "Bit 10 - State UNKNOWN 5."]
    #[inline(always)]
    pub fn su5(&mut self) -> SU5_W {
        SU5_W { w: self }
    }
    #[doc = "Bit 11 - State UNKNOWN Secure only 5."]
    #[inline(always)]
    pub fn sus5(&mut self) -> SUS5_W {
        SUS5_W { w: self }
    }
    #[doc = "Bit 12 - State UNKNOWN 6."]
    #[inline(always)]
    pub fn su6(&mut self) -> SU6_W {
        SU6_W { w: self }
    }
    #[doc = "Bit 13 - State UNKNOWN Secure only 6."]
    #[inline(always)]
    pub fn sus6(&mut self) -> SUS6_W {
        SUS6_W { w: self }
    }
    #[doc = "Bit 14 - State UNKNOWN 7."]
    #[inline(always)]
    pub fn su7(&mut self) -> SU7_W {
        SU7_W { w: self }
    }
    #[doc = "Bit 15 - State UNKNOWN Secure only 7."]
    #[inline(always)]
    pub fn sus7(&mut self) -> SUS7_W {
        SUS7_W { w: self }
    }
    #[doc = "Bit 20 - State UNKNOWN 10."]
    #[inline(always)]
    pub fn su10(&mut self) -> SU10_W {
        SU10_W { w: self }
    }
    #[doc = "Bit 21 - State UNKNOWN Secure only 10."]
    #[inline(always)]
    pub fn sus10(&mut self) -> SUS10_W {
        SUS10_W { w: self }
    }
    #[doc = "Bit 22 - State UNKNOWN 11."]
    #[inline(always)]
    pub fn su11(&mut self) -> SU11_W {
        SU11_W { w: self }
    }
    #[doc = "Bit 23 - State UNKNOWN Secure only 11."]
    #[inline(always)]
    pub fn sus11(&mut self) -> SUS11_W {
        SUS11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Coprocessor Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cppwr](index.html) module"]
pub struct CPPWR_SPEC;
impl crate::RegisterSpec for CPPWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cppwr::R](R) reader structure"]
impl crate::Readable for CPPWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cppwr::W](W) writer structure"]
impl crate::Writable for CPPWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPPWR to value 0"]
impl crate::Resettable for CPPWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
