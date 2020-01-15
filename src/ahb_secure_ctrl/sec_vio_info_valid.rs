#[doc = "Reader of register SEC_VIO_INFO_VALID"]
pub type R = crate::R<u32, super::SEC_VIO_INFO_VALID>;
#[doc = "Writer for register SEC_VIO_INFO_VALID"]
pub type W = crate::W<u32, super::SEC_VIO_INFO_VALID>;
#[doc = "Register SEC_VIO_INFO_VALID `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC_VIO_INFO_VALID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "violation information valid flag for AHB port 0. Write 1 to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID0_A {
    #[doc = "0: Not valid."]
    NOT_VALID = 0,
    #[doc = "1: Valid (violation occurred)."]
    VALID = 1,
}
impl From<VIO_INFO_VALID0_A> for bool {
    #[inline(always)]
    fn from(variant: VIO_INFO_VALID0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VIO_INFO_VALID0`"]
pub type VIO_INFO_VALID0_R = crate::R<bool, VIO_INFO_VALID0_A>;
impl VIO_INFO_VALID0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VIO_INFO_VALID0_A {
        match self.bits {
            false => VIO_INFO_VALID0_A::NOT_VALID,
            true => VIO_INFO_VALID0_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID0_A::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID0_A::VALID
    }
}
#[doc = "Write proxy for field `VIO_INFO_VALID0`"]
pub struct VIO_INFO_VALID0_W<'a> {
    w: &'a mut W,
}
impl<'a> VIO_INFO_VALID0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VIO_INFO_VALID0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID0_A::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID0_A::VALID)
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
#[doc = "violation information valid flag for AHB port 1. Write 1 to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID1_A {
    #[doc = "0: Not valid."]
    NOT_VALID = 0,
    #[doc = "1: Valid (violation occurred)."]
    VALID = 1,
}
impl From<VIO_INFO_VALID1_A> for bool {
    #[inline(always)]
    fn from(variant: VIO_INFO_VALID1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VIO_INFO_VALID1`"]
pub type VIO_INFO_VALID1_R = crate::R<bool, VIO_INFO_VALID1_A>;
impl VIO_INFO_VALID1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VIO_INFO_VALID1_A {
        match self.bits {
            false => VIO_INFO_VALID1_A::NOT_VALID,
            true => VIO_INFO_VALID1_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID1_A::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID1_A::VALID
    }
}
#[doc = "Write proxy for field `VIO_INFO_VALID1`"]
pub struct VIO_INFO_VALID1_W<'a> {
    w: &'a mut W,
}
impl<'a> VIO_INFO_VALID1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VIO_INFO_VALID1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID1_A::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID1_A::VALID)
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
#[doc = "violation information valid flag for AHB port 2. Write 1 to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID2_A {
    #[doc = "0: Not valid."]
    NOT_VALID = 0,
    #[doc = "1: Valid (violation occurred)."]
    VALID = 1,
}
impl From<VIO_INFO_VALID2_A> for bool {
    #[inline(always)]
    fn from(variant: VIO_INFO_VALID2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VIO_INFO_VALID2`"]
pub type VIO_INFO_VALID2_R = crate::R<bool, VIO_INFO_VALID2_A>;
impl VIO_INFO_VALID2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VIO_INFO_VALID2_A {
        match self.bits {
            false => VIO_INFO_VALID2_A::NOT_VALID,
            true => VIO_INFO_VALID2_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID2_A::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID2_A::VALID
    }
}
#[doc = "Write proxy for field `VIO_INFO_VALID2`"]
pub struct VIO_INFO_VALID2_W<'a> {
    w: &'a mut W,
}
impl<'a> VIO_INFO_VALID2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VIO_INFO_VALID2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID2_A::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID2_A::VALID)
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
#[doc = "violation information valid flag for AHB port 3. Write 1 to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID3_A {
    #[doc = "0: Not valid."]
    NOT_VALID = 0,
    #[doc = "1: Valid (violation occurred)."]
    VALID = 1,
}
impl From<VIO_INFO_VALID3_A> for bool {
    #[inline(always)]
    fn from(variant: VIO_INFO_VALID3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VIO_INFO_VALID3`"]
pub type VIO_INFO_VALID3_R = crate::R<bool, VIO_INFO_VALID3_A>;
impl VIO_INFO_VALID3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VIO_INFO_VALID3_A {
        match self.bits {
            false => VIO_INFO_VALID3_A::NOT_VALID,
            true => VIO_INFO_VALID3_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID3_A::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID3_A::VALID
    }
}
#[doc = "Write proxy for field `VIO_INFO_VALID3`"]
pub struct VIO_INFO_VALID3_W<'a> {
    w: &'a mut W,
}
impl<'a> VIO_INFO_VALID3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VIO_INFO_VALID3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID3_A::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID3_A::VALID)
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
#[doc = "violation information valid flag for AHB port 4. Write 1 to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID4_A {
    #[doc = "0: Not valid."]
    NOT_VALID = 0,
    #[doc = "1: Valid (violation occurred)."]
    VALID = 1,
}
impl From<VIO_INFO_VALID4_A> for bool {
    #[inline(always)]
    fn from(variant: VIO_INFO_VALID4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VIO_INFO_VALID4`"]
pub type VIO_INFO_VALID4_R = crate::R<bool, VIO_INFO_VALID4_A>;
impl VIO_INFO_VALID4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VIO_INFO_VALID4_A {
        match self.bits {
            false => VIO_INFO_VALID4_A::NOT_VALID,
            true => VIO_INFO_VALID4_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID4_A::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID4_A::VALID
    }
}
#[doc = "Write proxy for field `VIO_INFO_VALID4`"]
pub struct VIO_INFO_VALID4_W<'a> {
    w: &'a mut W,
}
impl<'a> VIO_INFO_VALID4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VIO_INFO_VALID4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID4_A::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID4_A::VALID)
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
#[doc = "violation information valid flag for AHB port 5. Write 1 to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID5_A {
    #[doc = "0: Not valid."]
    NOT_VALID = 0,
    #[doc = "1: Valid (violation occurred)."]
    VALID = 1,
}
impl From<VIO_INFO_VALID5_A> for bool {
    #[inline(always)]
    fn from(variant: VIO_INFO_VALID5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VIO_INFO_VALID5`"]
pub type VIO_INFO_VALID5_R = crate::R<bool, VIO_INFO_VALID5_A>;
impl VIO_INFO_VALID5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VIO_INFO_VALID5_A {
        match self.bits {
            false => VIO_INFO_VALID5_A::NOT_VALID,
            true => VIO_INFO_VALID5_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID5_A::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID5_A::VALID
    }
}
#[doc = "Write proxy for field `VIO_INFO_VALID5`"]
pub struct VIO_INFO_VALID5_W<'a> {
    w: &'a mut W,
}
impl<'a> VIO_INFO_VALID5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VIO_INFO_VALID5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID5_A::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID5_A::VALID)
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
#[doc = "violation information valid flag for AHB port 6. Write 1 to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID6_A {
    #[doc = "0: Not valid."]
    NOT_VALID = 0,
    #[doc = "1: Valid (violation occurred)."]
    VALID = 1,
}
impl From<VIO_INFO_VALID6_A> for bool {
    #[inline(always)]
    fn from(variant: VIO_INFO_VALID6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VIO_INFO_VALID6`"]
pub type VIO_INFO_VALID6_R = crate::R<bool, VIO_INFO_VALID6_A>;
impl VIO_INFO_VALID6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VIO_INFO_VALID6_A {
        match self.bits {
            false => VIO_INFO_VALID6_A::NOT_VALID,
            true => VIO_INFO_VALID6_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID6_A::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID6_A::VALID
    }
}
#[doc = "Write proxy for field `VIO_INFO_VALID6`"]
pub struct VIO_INFO_VALID6_W<'a> {
    w: &'a mut W,
}
impl<'a> VIO_INFO_VALID6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VIO_INFO_VALID6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID6_A::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID6_A::VALID)
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
#[doc = "violation information valid flag for AHB port 7. Write 1 to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID7_A {
    #[doc = "0: Not valid."]
    NOT_VALID = 0,
    #[doc = "1: Valid (violation occurred)."]
    VALID = 1,
}
impl From<VIO_INFO_VALID7_A> for bool {
    #[inline(always)]
    fn from(variant: VIO_INFO_VALID7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VIO_INFO_VALID7`"]
pub type VIO_INFO_VALID7_R = crate::R<bool, VIO_INFO_VALID7_A>;
impl VIO_INFO_VALID7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VIO_INFO_VALID7_A {
        match self.bits {
            false => VIO_INFO_VALID7_A::NOT_VALID,
            true => VIO_INFO_VALID7_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID7_A::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID7_A::VALID
    }
}
#[doc = "Write proxy for field `VIO_INFO_VALID7`"]
pub struct VIO_INFO_VALID7_W<'a> {
    w: &'a mut W,
}
impl<'a> VIO_INFO_VALID7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VIO_INFO_VALID7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID7_A::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID7_A::VALID)
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
#[doc = "violation information valid flag for AHB port 8. Write 1 to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID8_A {
    #[doc = "0: Not valid."]
    NOT_VALID = 0,
    #[doc = "1: Valid (violation occurred)."]
    VALID = 1,
}
impl From<VIO_INFO_VALID8_A> for bool {
    #[inline(always)]
    fn from(variant: VIO_INFO_VALID8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VIO_INFO_VALID8`"]
pub type VIO_INFO_VALID8_R = crate::R<bool, VIO_INFO_VALID8_A>;
impl VIO_INFO_VALID8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VIO_INFO_VALID8_A {
        match self.bits {
            false => VIO_INFO_VALID8_A::NOT_VALID,
            true => VIO_INFO_VALID8_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID8_A::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID8_A::VALID
    }
}
#[doc = "Write proxy for field `VIO_INFO_VALID8`"]
pub struct VIO_INFO_VALID8_W<'a> {
    w: &'a mut W,
}
impl<'a> VIO_INFO_VALID8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VIO_INFO_VALID8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID8_A::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID8_A::VALID)
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
#[doc = "violation information valid flag for AHB port 9. Write 1 to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID9_A {
    #[doc = "0: Not valid."]
    NOT_VALID = 0,
    #[doc = "1: Valid (violation occurred)."]
    VALID = 1,
}
impl From<VIO_INFO_VALID9_A> for bool {
    #[inline(always)]
    fn from(variant: VIO_INFO_VALID9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VIO_INFO_VALID9`"]
pub type VIO_INFO_VALID9_R = crate::R<bool, VIO_INFO_VALID9_A>;
impl VIO_INFO_VALID9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VIO_INFO_VALID9_A {
        match self.bits {
            false => VIO_INFO_VALID9_A::NOT_VALID,
            true => VIO_INFO_VALID9_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID9_A::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID9_A::VALID
    }
}
#[doc = "Write proxy for field `VIO_INFO_VALID9`"]
pub struct VIO_INFO_VALID9_W<'a> {
    w: &'a mut W,
}
impl<'a> VIO_INFO_VALID9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VIO_INFO_VALID9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID9_A::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID9_A::VALID)
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
#[doc = "violation information valid flag for AHB port 10. Write 1 to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID10_A {
    #[doc = "0: Not valid."]
    NOT_VALID = 0,
    #[doc = "1: Valid (violation occurred)."]
    VALID = 1,
}
impl From<VIO_INFO_VALID10_A> for bool {
    #[inline(always)]
    fn from(variant: VIO_INFO_VALID10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VIO_INFO_VALID10`"]
pub type VIO_INFO_VALID10_R = crate::R<bool, VIO_INFO_VALID10_A>;
impl VIO_INFO_VALID10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VIO_INFO_VALID10_A {
        match self.bits {
            false => VIO_INFO_VALID10_A::NOT_VALID,
            true => VIO_INFO_VALID10_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID10_A::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID10_A::VALID
    }
}
#[doc = "Write proxy for field `VIO_INFO_VALID10`"]
pub struct VIO_INFO_VALID10_W<'a> {
    w: &'a mut W,
}
impl<'a> VIO_INFO_VALID10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VIO_INFO_VALID10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID10_A::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID10_A::VALID)
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
#[doc = "violation information valid flag for AHB port 11. Write 1 to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIO_INFO_VALID11_A {
    #[doc = "0: Not valid."]
    NOT_VALID = 0,
    #[doc = "1: Valid (violation occurred)."]
    VALID = 1,
}
impl From<VIO_INFO_VALID11_A> for bool {
    #[inline(always)]
    fn from(variant: VIO_INFO_VALID11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VIO_INFO_VALID11`"]
pub type VIO_INFO_VALID11_R = crate::R<bool, VIO_INFO_VALID11_A>;
impl VIO_INFO_VALID11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VIO_INFO_VALID11_A {
        match self.bits {
            false => VIO_INFO_VALID11_A::NOT_VALID,
            true => VIO_INFO_VALID11_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VIO_INFO_VALID11_A::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VIO_INFO_VALID11_A::VALID
    }
}
#[doc = "Write proxy for field `VIO_INFO_VALID11`"]
pub struct VIO_INFO_VALID11_W<'a> {
    w: &'a mut W,
}
impl<'a> VIO_INFO_VALID11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VIO_INFO_VALID11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID11_A::NOT_VALID)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(VIO_INFO_VALID11_A::VALID)
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
impl R {
    #[doc = "Bit 0 - violation information valid flag for AHB port 0. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid0(&self) -> VIO_INFO_VALID0_R {
        VIO_INFO_VALID0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - violation information valid flag for AHB port 1. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid1(&self) -> VIO_INFO_VALID1_R {
        VIO_INFO_VALID1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - violation information valid flag for AHB port 2. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid2(&self) -> VIO_INFO_VALID2_R {
        VIO_INFO_VALID2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - violation information valid flag for AHB port 3. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid3(&self) -> VIO_INFO_VALID3_R {
        VIO_INFO_VALID3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - violation information valid flag for AHB port 4. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid4(&self) -> VIO_INFO_VALID4_R {
        VIO_INFO_VALID4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - violation information valid flag for AHB port 5. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid5(&self) -> VIO_INFO_VALID5_R {
        VIO_INFO_VALID5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - violation information valid flag for AHB port 6. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid6(&self) -> VIO_INFO_VALID6_R {
        VIO_INFO_VALID6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - violation information valid flag for AHB port 7. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid7(&self) -> VIO_INFO_VALID7_R {
        VIO_INFO_VALID7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - violation information valid flag for AHB port 8. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid8(&self) -> VIO_INFO_VALID8_R {
        VIO_INFO_VALID8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - violation information valid flag for AHB port 9. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid9(&self) -> VIO_INFO_VALID9_R {
        VIO_INFO_VALID9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - violation information valid flag for AHB port 10. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid10(&self) -> VIO_INFO_VALID10_R {
        VIO_INFO_VALID10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - violation information valid flag for AHB port 11. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid11(&self) -> VIO_INFO_VALID11_R {
        VIO_INFO_VALID11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - violation information valid flag for AHB port 0. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid0(&mut self) -> VIO_INFO_VALID0_W {
        VIO_INFO_VALID0_W { w: self }
    }
    #[doc = "Bit 1 - violation information valid flag for AHB port 1. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid1(&mut self) -> VIO_INFO_VALID1_W {
        VIO_INFO_VALID1_W { w: self }
    }
    #[doc = "Bit 2 - violation information valid flag for AHB port 2. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid2(&mut self) -> VIO_INFO_VALID2_W {
        VIO_INFO_VALID2_W { w: self }
    }
    #[doc = "Bit 3 - violation information valid flag for AHB port 3. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid3(&mut self) -> VIO_INFO_VALID3_W {
        VIO_INFO_VALID3_W { w: self }
    }
    #[doc = "Bit 4 - violation information valid flag for AHB port 4. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid4(&mut self) -> VIO_INFO_VALID4_W {
        VIO_INFO_VALID4_W { w: self }
    }
    #[doc = "Bit 5 - violation information valid flag for AHB port 5. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid5(&mut self) -> VIO_INFO_VALID5_W {
        VIO_INFO_VALID5_W { w: self }
    }
    #[doc = "Bit 6 - violation information valid flag for AHB port 6. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid6(&mut self) -> VIO_INFO_VALID6_W {
        VIO_INFO_VALID6_W { w: self }
    }
    #[doc = "Bit 7 - violation information valid flag for AHB port 7. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid7(&mut self) -> VIO_INFO_VALID7_W {
        VIO_INFO_VALID7_W { w: self }
    }
    #[doc = "Bit 8 - violation information valid flag for AHB port 8. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid8(&mut self) -> VIO_INFO_VALID8_W {
        VIO_INFO_VALID8_W { w: self }
    }
    #[doc = "Bit 9 - violation information valid flag for AHB port 9. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid9(&mut self) -> VIO_INFO_VALID9_W {
        VIO_INFO_VALID9_W { w: self }
    }
    #[doc = "Bit 10 - violation information valid flag for AHB port 10. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid10(&mut self) -> VIO_INFO_VALID10_W {
        VIO_INFO_VALID10_W { w: self }
    }
    #[doc = "Bit 11 - violation information valid flag for AHB port 11. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid11(&mut self) -> VIO_INFO_VALID11_W {
        VIO_INFO_VALID11_W { w: self }
    }
}
