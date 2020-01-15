#[doc = "Reader of register CPPWR"]
pub type R = crate::R<u32, super::CPPWR>;
#[doc = "Writer for register CPPWR"]
pub type W = crate::W<u32, super::CPPWR>;
#[doc = "Register CPPWR `reset()`'s with value 0"]
impl crate::ResetValue for super::CPPWR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `SU0`"]
pub type SU0_R = crate::R<bool, SU0_A>;
impl SU0_R {
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
        *self == SU0_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU0_A::UNKNOWN_PERMITTED
    }
}
#[doc = "Write proxy for field `SU0`"]
pub struct SU0_W<'a> {
    w: &'a mut W,
}
impl<'a> SU0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SU0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
#[doc = "Reader of field `SUS0`"]
pub type SUS0_R = crate::R<bool, SUS0_A>;
impl SUS0_R {
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
        *self == SUS0_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS0_A::SECURE_ONLY
    }
}
#[doc = "Write proxy for field `SUS0`"]
pub struct SUS0_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
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
#[doc = "Reader of field `SU1`"]
pub type SU1_R = crate::R<bool, SU1_A>;
impl SU1_R {
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
        *self == SU1_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU1_A::UNKNOWN_PERMITTED
    }
}
#[doc = "Write proxy for field `SU1`"]
pub struct SU1_W<'a> {
    w: &'a mut W,
}
impl<'a> SU1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SU1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
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
#[doc = "Reader of field `SUS1`"]
pub type SUS1_R = crate::R<bool, SUS1_A>;
impl SUS1_R {
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
        *self == SUS1_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS1_A::SECURE_ONLY
    }
}
#[doc = "Write proxy for field `SUS1`"]
pub struct SUS1_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
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
#[doc = "Reader of field `SU2`"]
pub type SU2_R = crate::R<bool, SU2_A>;
impl SU2_R {
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
        *self == SU2_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU2_A::UNKNOWN_PERMITTED
    }
}
#[doc = "Write proxy for field `SU2`"]
pub struct SU2_W<'a> {
    w: &'a mut W,
}
impl<'a> SU2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SU2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
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
#[doc = "Reader of field `SUS2`"]
pub type SUS2_R = crate::R<bool, SUS2_A>;
impl SUS2_R {
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
        *self == SUS2_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS2_A::SECURE_ONLY
    }
}
#[doc = "Write proxy for field `SUS2`"]
pub struct SUS2_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
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
#[doc = "Reader of field `SU3`"]
pub type SU3_R = crate::R<bool, SU3_A>;
impl SU3_R {
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
        *self == SU3_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU3_A::UNKNOWN_PERMITTED
    }
}
#[doc = "Write proxy for field `SU3`"]
pub struct SU3_W<'a> {
    w: &'a mut W,
}
impl<'a> SU3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SU3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
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
#[doc = "Reader of field `SUS3`"]
pub type SUS3_R = crate::R<bool, SUS3_A>;
impl SUS3_R {
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
        *self == SUS3_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS3_A::SECURE_ONLY
    }
}
#[doc = "Write proxy for field `SUS3`"]
pub struct SUS3_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
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
#[doc = "Reader of field `SU4`"]
pub type SU4_R = crate::R<bool, SU4_A>;
impl SU4_R {
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
        *self == SU4_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU4_A::UNKNOWN_PERMITTED
    }
}
#[doc = "Write proxy for field `SU4`"]
pub struct SU4_W<'a> {
    w: &'a mut W,
}
impl<'a> SU4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SU4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
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
#[doc = "Reader of field `SUS4`"]
pub type SUS4_R = crate::R<bool, SUS4_A>;
impl SUS4_R {
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
        *self == SUS4_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS4_A::SECURE_ONLY
    }
}
#[doc = "Write proxy for field `SUS4`"]
pub struct SUS4_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
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
#[doc = "Reader of field `SU5`"]
pub type SU5_R = crate::R<bool, SU5_A>;
impl SU5_R {
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
        *self == SU5_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU5_A::UNKNOWN_PERMITTED
    }
}
#[doc = "Write proxy for field `SU5`"]
pub struct SU5_W<'a> {
    w: &'a mut W,
}
impl<'a> SU5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SU5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
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
#[doc = "Reader of field `SUS5`"]
pub type SUS5_R = crate::R<bool, SUS5_A>;
impl SUS5_R {
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
        *self == SUS5_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS5_A::SECURE_ONLY
    }
}
#[doc = "Write proxy for field `SUS5`"]
pub struct SUS5_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
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
#[doc = "Reader of field `SU6`"]
pub type SU6_R = crate::R<bool, SU6_A>;
impl SU6_R {
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
        *self == SU6_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU6_A::UNKNOWN_PERMITTED
    }
}
#[doc = "Write proxy for field `SU6`"]
pub struct SU6_W<'a> {
    w: &'a mut W,
}
impl<'a> SU6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SU6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
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
#[doc = "Reader of field `SUS6`"]
pub type SUS6_R = crate::R<bool, SUS6_A>;
impl SUS6_R {
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
        *self == SUS6_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS6_A::SECURE_ONLY
    }
}
#[doc = "Write proxy for field `SUS6`"]
pub struct SUS6_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
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
#[doc = "Reader of field `SU7`"]
pub type SU7_R = crate::R<bool, SU7_A>;
impl SU7_R {
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
        *self == SU7_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU7_A::UNKNOWN_PERMITTED
    }
}
#[doc = "Write proxy for field `SU7`"]
pub struct SU7_W<'a> {
    w: &'a mut W,
}
impl<'a> SU7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SU7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
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
#[doc = "Reader of field `SUS7`"]
pub type SUS7_R = crate::R<bool, SUS7_A>;
impl SUS7_R {
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
        *self == SUS7_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS7_A::SECURE_ONLY
    }
}
#[doc = "Write proxy for field `SUS7`"]
pub struct SUS7_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
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
#[doc = "Reader of field `SU10`"]
pub type SU10_R = crate::R<bool, SU10_A>;
impl SU10_R {
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
        *self == SU10_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU10_A::UNKNOWN_PERMITTED
    }
}
#[doc = "Write proxy for field `SU10`"]
pub struct SU10_W<'a> {
    w: &'a mut W,
}
impl<'a> SU10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SU10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
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
#[doc = "Reader of field `SUS10`"]
pub type SUS10_R = crate::R<bool, SUS10_A>;
impl SUS10_R {
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
        *self == SUS10_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS10_A::SECURE_ONLY
    }
}
#[doc = "Write proxy for field `SUS10`"]
pub struct SUS10_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `SU11`"]
pub type SU11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SU11`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `SUS11`"]
pub type SUS11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUS11`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
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
}
