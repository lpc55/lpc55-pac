#[doc = "Register `SWTRIG` reader"]
pub struct R(crate::R<SWTRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWTRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SWTRIG_SPEC>> for R {
    fn from(reader: crate::R<SWTRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWTRIG` writer"]
pub struct W(crate::W<SWTRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWTRIG_SPEC>;
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
impl core::convert::From<crate::W<SWTRIG_SPEC>> for W {
    fn from(writer: crate::W<SWTRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Software trigger 0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT0_A {
    #[doc = "0: No trigger 0 event generated."]
    SWT0_0 = 0,
    #[doc = "1: Trigger 0 event generated."]
    SWT0_1 = 1,
}
impl From<SWT0_A> for bool {
    #[inline(always)]
    fn from(variant: SWT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT0` reader - Software trigger 0 event"]
pub struct SWT0_R(crate::FieldReader<bool, SWT0_A>);
impl SWT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT0_A {
        match self.bits {
            false => SWT0_A::SWT0_0,
            true => SWT0_A::SWT0_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT0_0`"]
    #[inline(always)]
    pub fn is_swt0_0(&self) -> bool {
        **self == SWT0_A::SWT0_0
    }
    #[doc = "Checks if the value of the field is `SWT0_1`"]
    #[inline(always)]
    pub fn is_swt0_1(&self) -> bool {
        **self == SWT0_A::SWT0_1
    }
}
impl core::ops::Deref for SWT0_R {
    type Target = crate::FieldReader<bool, SWT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWT0` writer - Software trigger 0 event"]
pub struct SWT0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No trigger 0 event generated."]
    #[inline(always)]
    pub fn swt0_0(self) -> &'a mut W {
        self.variant(SWT0_A::SWT0_0)
    }
    #[doc = "Trigger 0 event generated."]
    #[inline(always)]
    pub fn swt0_1(self) -> &'a mut W {
        self.variant(SWT0_A::SWT0_1)
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
#[doc = "Software trigger 1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT1_A {
    #[doc = "0: No trigger 1 event generated."]
    SWT1_0 = 0,
    #[doc = "1: Trigger 1 event generated."]
    SWT1_1 = 1,
}
impl From<SWT1_A> for bool {
    #[inline(always)]
    fn from(variant: SWT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT1` reader - Software trigger 1 event"]
pub struct SWT1_R(crate::FieldReader<bool, SWT1_A>);
impl SWT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT1_A {
        match self.bits {
            false => SWT1_A::SWT1_0,
            true => SWT1_A::SWT1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT1_0`"]
    #[inline(always)]
    pub fn is_swt1_0(&self) -> bool {
        **self == SWT1_A::SWT1_0
    }
    #[doc = "Checks if the value of the field is `SWT1_1`"]
    #[inline(always)]
    pub fn is_swt1_1(&self) -> bool {
        **self == SWT1_A::SWT1_1
    }
}
impl core::ops::Deref for SWT1_R {
    type Target = crate::FieldReader<bool, SWT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWT1` writer - Software trigger 1 event"]
pub struct SWT1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No trigger 1 event generated."]
    #[inline(always)]
    pub fn swt1_0(self) -> &'a mut W {
        self.variant(SWT1_A::SWT1_0)
    }
    #[doc = "Trigger 1 event generated."]
    #[inline(always)]
    pub fn swt1_1(self) -> &'a mut W {
        self.variant(SWT1_A::SWT1_1)
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
#[doc = "Software trigger 2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT2_A {
    #[doc = "0: No trigger 2 event generated."]
    SWT2_0 = 0,
    #[doc = "1: Trigger 2 event generated."]
    SWT2_1 = 1,
}
impl From<SWT2_A> for bool {
    #[inline(always)]
    fn from(variant: SWT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT2` reader - Software trigger 2 event"]
pub struct SWT2_R(crate::FieldReader<bool, SWT2_A>);
impl SWT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWT2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT2_A {
        match self.bits {
            false => SWT2_A::SWT2_0,
            true => SWT2_A::SWT2_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT2_0`"]
    #[inline(always)]
    pub fn is_swt2_0(&self) -> bool {
        **self == SWT2_A::SWT2_0
    }
    #[doc = "Checks if the value of the field is `SWT2_1`"]
    #[inline(always)]
    pub fn is_swt2_1(&self) -> bool {
        **self == SWT2_A::SWT2_1
    }
}
impl core::ops::Deref for SWT2_R {
    type Target = crate::FieldReader<bool, SWT2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWT2` writer - Software trigger 2 event"]
pub struct SWT2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No trigger 2 event generated."]
    #[inline(always)]
    pub fn swt2_0(self) -> &'a mut W {
        self.variant(SWT2_A::SWT2_0)
    }
    #[doc = "Trigger 2 event generated."]
    #[inline(always)]
    pub fn swt2_1(self) -> &'a mut W {
        self.variant(SWT2_A::SWT2_1)
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
#[doc = "Software trigger 3 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT3_A {
    #[doc = "0: No trigger 3 event generated."]
    SWT3_0 = 0,
    #[doc = "1: Trigger 3 event generated."]
    SWT3_1 = 1,
}
impl From<SWT3_A> for bool {
    #[inline(always)]
    fn from(variant: SWT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT3` reader - Software trigger 3 event"]
pub struct SWT3_R(crate::FieldReader<bool, SWT3_A>);
impl SWT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWT3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT3_A {
        match self.bits {
            false => SWT3_A::SWT3_0,
            true => SWT3_A::SWT3_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT3_0`"]
    #[inline(always)]
    pub fn is_swt3_0(&self) -> bool {
        **self == SWT3_A::SWT3_0
    }
    #[doc = "Checks if the value of the field is `SWT3_1`"]
    #[inline(always)]
    pub fn is_swt3_1(&self) -> bool {
        **self == SWT3_A::SWT3_1
    }
}
impl core::ops::Deref for SWT3_R {
    type Target = crate::FieldReader<bool, SWT3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWT3` writer - Software trigger 3 event"]
pub struct SWT3_W<'a> {
    w: &'a mut W,
}
impl<'a> SWT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWT3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No trigger 3 event generated."]
    #[inline(always)]
    pub fn swt3_0(self) -> &'a mut W {
        self.variant(SWT3_A::SWT3_0)
    }
    #[doc = "Trigger 3 event generated."]
    #[inline(always)]
    pub fn swt3_1(self) -> &'a mut W {
        self.variant(SWT3_A::SWT3_1)
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
#[doc = "Software trigger 4 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT4_A {
    #[doc = "0: No trigger 4 event generated."]
    SWT4_0 = 0,
    #[doc = "1: Trigger 4 event generated."]
    SWT4_1 = 1,
}
impl From<SWT4_A> for bool {
    #[inline(always)]
    fn from(variant: SWT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT4` reader - Software trigger 4 event"]
pub struct SWT4_R(crate::FieldReader<bool, SWT4_A>);
impl SWT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWT4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT4_A {
        match self.bits {
            false => SWT4_A::SWT4_0,
            true => SWT4_A::SWT4_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT4_0`"]
    #[inline(always)]
    pub fn is_swt4_0(&self) -> bool {
        **self == SWT4_A::SWT4_0
    }
    #[doc = "Checks if the value of the field is `SWT4_1`"]
    #[inline(always)]
    pub fn is_swt4_1(&self) -> bool {
        **self == SWT4_A::SWT4_1
    }
}
impl core::ops::Deref for SWT4_R {
    type Target = crate::FieldReader<bool, SWT4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWT4` writer - Software trigger 4 event"]
pub struct SWT4_W<'a> {
    w: &'a mut W,
}
impl<'a> SWT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWT4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No trigger 4 event generated."]
    #[inline(always)]
    pub fn swt4_0(self) -> &'a mut W {
        self.variant(SWT4_A::SWT4_0)
    }
    #[doc = "Trigger 4 event generated."]
    #[inline(always)]
    pub fn swt4_1(self) -> &'a mut W {
        self.variant(SWT4_A::SWT4_1)
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
#[doc = "Software trigger 5 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT5_A {
    #[doc = "0: No trigger 5 event generated."]
    SWT5_0 = 0,
    #[doc = "1: Trigger 5 event generated."]
    SWT5_1 = 1,
}
impl From<SWT5_A> for bool {
    #[inline(always)]
    fn from(variant: SWT5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT5` reader - Software trigger 5 event"]
pub struct SWT5_R(crate::FieldReader<bool, SWT5_A>);
impl SWT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWT5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT5_A {
        match self.bits {
            false => SWT5_A::SWT5_0,
            true => SWT5_A::SWT5_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT5_0`"]
    #[inline(always)]
    pub fn is_swt5_0(&self) -> bool {
        **self == SWT5_A::SWT5_0
    }
    #[doc = "Checks if the value of the field is `SWT5_1`"]
    #[inline(always)]
    pub fn is_swt5_1(&self) -> bool {
        **self == SWT5_A::SWT5_1
    }
}
impl core::ops::Deref for SWT5_R {
    type Target = crate::FieldReader<bool, SWT5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWT5` writer - Software trigger 5 event"]
pub struct SWT5_W<'a> {
    w: &'a mut W,
}
impl<'a> SWT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWT5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No trigger 5 event generated."]
    #[inline(always)]
    pub fn swt5_0(self) -> &'a mut W {
        self.variant(SWT5_A::SWT5_0)
    }
    #[doc = "Trigger 5 event generated."]
    #[inline(always)]
    pub fn swt5_1(self) -> &'a mut W {
        self.variant(SWT5_A::SWT5_1)
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
#[doc = "Software trigger 6 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT6_A {
    #[doc = "0: No trigger 6 event generated."]
    SWT6_0 = 0,
    #[doc = "1: Trigger 6 event generated."]
    SWT6_1 = 1,
}
impl From<SWT6_A> for bool {
    #[inline(always)]
    fn from(variant: SWT6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT6` reader - Software trigger 6 event"]
pub struct SWT6_R(crate::FieldReader<bool, SWT6_A>);
impl SWT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWT6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT6_A {
        match self.bits {
            false => SWT6_A::SWT6_0,
            true => SWT6_A::SWT6_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT6_0`"]
    #[inline(always)]
    pub fn is_swt6_0(&self) -> bool {
        **self == SWT6_A::SWT6_0
    }
    #[doc = "Checks if the value of the field is `SWT6_1`"]
    #[inline(always)]
    pub fn is_swt6_1(&self) -> bool {
        **self == SWT6_A::SWT6_1
    }
}
impl core::ops::Deref for SWT6_R {
    type Target = crate::FieldReader<bool, SWT6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWT6` writer - Software trigger 6 event"]
pub struct SWT6_W<'a> {
    w: &'a mut W,
}
impl<'a> SWT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWT6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No trigger 6 event generated."]
    #[inline(always)]
    pub fn swt6_0(self) -> &'a mut W {
        self.variant(SWT6_A::SWT6_0)
    }
    #[doc = "Trigger 6 event generated."]
    #[inline(always)]
    pub fn swt6_1(self) -> &'a mut W {
        self.variant(SWT6_A::SWT6_1)
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
#[doc = "Software trigger 7 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT7_A {
    #[doc = "0: No trigger 7 event generated."]
    SWT7_0 = 0,
    #[doc = "1: Trigger 7 event generated."]
    SWT7_1 = 1,
}
impl From<SWT7_A> for bool {
    #[inline(always)]
    fn from(variant: SWT7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT7` reader - Software trigger 7 event"]
pub struct SWT7_R(crate::FieldReader<bool, SWT7_A>);
impl SWT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWT7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT7_A {
        match self.bits {
            false => SWT7_A::SWT7_0,
            true => SWT7_A::SWT7_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT7_0`"]
    #[inline(always)]
    pub fn is_swt7_0(&self) -> bool {
        **self == SWT7_A::SWT7_0
    }
    #[doc = "Checks if the value of the field is `SWT7_1`"]
    #[inline(always)]
    pub fn is_swt7_1(&self) -> bool {
        **self == SWT7_A::SWT7_1
    }
}
impl core::ops::Deref for SWT7_R {
    type Target = crate::FieldReader<bool, SWT7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWT7` writer - Software trigger 7 event"]
pub struct SWT7_W<'a> {
    w: &'a mut W,
}
impl<'a> SWT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWT7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No trigger 7 event generated."]
    #[inline(always)]
    pub fn swt7_0(self) -> &'a mut W {
        self.variant(SWT7_A::SWT7_0)
    }
    #[doc = "Trigger 7 event generated."]
    #[inline(always)]
    pub fn swt7_1(self) -> &'a mut W {
        self.variant(SWT7_A::SWT7_1)
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
#[doc = "Software trigger 8 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT8_A {
    #[doc = "0: No trigger 8 event generated."]
    SWT8_0 = 0,
    #[doc = "1: Trigger 8 event generated."]
    SWT8_1 = 1,
}
impl From<SWT8_A> for bool {
    #[inline(always)]
    fn from(variant: SWT8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT8` reader - Software trigger 8 event"]
pub struct SWT8_R(crate::FieldReader<bool, SWT8_A>);
impl SWT8_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWT8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT8_A {
        match self.bits {
            false => SWT8_A::SWT8_0,
            true => SWT8_A::SWT8_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT8_0`"]
    #[inline(always)]
    pub fn is_swt8_0(&self) -> bool {
        **self == SWT8_A::SWT8_0
    }
    #[doc = "Checks if the value of the field is `SWT8_1`"]
    #[inline(always)]
    pub fn is_swt8_1(&self) -> bool {
        **self == SWT8_A::SWT8_1
    }
}
impl core::ops::Deref for SWT8_R {
    type Target = crate::FieldReader<bool, SWT8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWT8` writer - Software trigger 8 event"]
pub struct SWT8_W<'a> {
    w: &'a mut W,
}
impl<'a> SWT8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWT8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No trigger 8 event generated."]
    #[inline(always)]
    pub fn swt8_0(self) -> &'a mut W {
        self.variant(SWT8_A::SWT8_0)
    }
    #[doc = "Trigger 8 event generated."]
    #[inline(always)]
    pub fn swt8_1(self) -> &'a mut W {
        self.variant(SWT8_A::SWT8_1)
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
#[doc = "Software trigger 9 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT9_A {
    #[doc = "0: No trigger 9 event generated."]
    SWT9_0 = 0,
    #[doc = "1: Trigger 9 event generated."]
    SWT9_1 = 1,
}
impl From<SWT9_A> for bool {
    #[inline(always)]
    fn from(variant: SWT9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT9` reader - Software trigger 9 event"]
pub struct SWT9_R(crate::FieldReader<bool, SWT9_A>);
impl SWT9_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWT9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT9_A {
        match self.bits {
            false => SWT9_A::SWT9_0,
            true => SWT9_A::SWT9_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT9_0`"]
    #[inline(always)]
    pub fn is_swt9_0(&self) -> bool {
        **self == SWT9_A::SWT9_0
    }
    #[doc = "Checks if the value of the field is `SWT9_1`"]
    #[inline(always)]
    pub fn is_swt9_1(&self) -> bool {
        **self == SWT9_A::SWT9_1
    }
}
impl core::ops::Deref for SWT9_R {
    type Target = crate::FieldReader<bool, SWT9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWT9` writer - Software trigger 9 event"]
pub struct SWT9_W<'a> {
    w: &'a mut W,
}
impl<'a> SWT9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWT9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No trigger 9 event generated."]
    #[inline(always)]
    pub fn swt9_0(self) -> &'a mut W {
        self.variant(SWT9_A::SWT9_0)
    }
    #[doc = "Trigger 9 event generated."]
    #[inline(always)]
    pub fn swt9_1(self) -> &'a mut W {
        self.variant(SWT9_A::SWT9_1)
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
#[doc = "Software trigger 10 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT10_A {
    #[doc = "0: No trigger 10 event generated."]
    SWT10_0 = 0,
    #[doc = "1: Trigger 10 event generated."]
    SWT10_1 = 1,
}
impl From<SWT10_A> for bool {
    #[inline(always)]
    fn from(variant: SWT10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT10` reader - Software trigger 10 event"]
pub struct SWT10_R(crate::FieldReader<bool, SWT10_A>);
impl SWT10_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWT10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT10_A {
        match self.bits {
            false => SWT10_A::SWT10_0,
            true => SWT10_A::SWT10_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT10_0`"]
    #[inline(always)]
    pub fn is_swt10_0(&self) -> bool {
        **self == SWT10_A::SWT10_0
    }
    #[doc = "Checks if the value of the field is `SWT10_1`"]
    #[inline(always)]
    pub fn is_swt10_1(&self) -> bool {
        **self == SWT10_A::SWT10_1
    }
}
impl core::ops::Deref for SWT10_R {
    type Target = crate::FieldReader<bool, SWT10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWT10` writer - Software trigger 10 event"]
pub struct SWT10_W<'a> {
    w: &'a mut W,
}
impl<'a> SWT10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWT10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No trigger 10 event generated."]
    #[inline(always)]
    pub fn swt10_0(self) -> &'a mut W {
        self.variant(SWT10_A::SWT10_0)
    }
    #[doc = "Trigger 10 event generated."]
    #[inline(always)]
    pub fn swt10_1(self) -> &'a mut W {
        self.variant(SWT10_A::SWT10_1)
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
#[doc = "Software trigger 11 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT11_A {
    #[doc = "0: No trigger 11 event generated."]
    SWT11_0 = 0,
    #[doc = "1: Trigger 11 event generated."]
    SWT11_1 = 1,
}
impl From<SWT11_A> for bool {
    #[inline(always)]
    fn from(variant: SWT11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT11` reader - Software trigger 11 event"]
pub struct SWT11_R(crate::FieldReader<bool, SWT11_A>);
impl SWT11_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWT11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT11_A {
        match self.bits {
            false => SWT11_A::SWT11_0,
            true => SWT11_A::SWT11_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT11_0`"]
    #[inline(always)]
    pub fn is_swt11_0(&self) -> bool {
        **self == SWT11_A::SWT11_0
    }
    #[doc = "Checks if the value of the field is `SWT11_1`"]
    #[inline(always)]
    pub fn is_swt11_1(&self) -> bool {
        **self == SWT11_A::SWT11_1
    }
}
impl core::ops::Deref for SWT11_R {
    type Target = crate::FieldReader<bool, SWT11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWT11` writer - Software trigger 11 event"]
pub struct SWT11_W<'a> {
    w: &'a mut W,
}
impl<'a> SWT11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWT11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No trigger 11 event generated."]
    #[inline(always)]
    pub fn swt11_0(self) -> &'a mut W {
        self.variant(SWT11_A::SWT11_0)
    }
    #[doc = "Trigger 11 event generated."]
    #[inline(always)]
    pub fn swt11_1(self) -> &'a mut W {
        self.variant(SWT11_A::SWT11_1)
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
#[doc = "Software trigger 12 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT12_A {
    #[doc = "0: No trigger 12 event generated."]
    SWT12_0 = 0,
    #[doc = "1: Trigger 12 event generated."]
    SWT12_1 = 1,
}
impl From<SWT12_A> for bool {
    #[inline(always)]
    fn from(variant: SWT12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT12` reader - Software trigger 12 event"]
pub struct SWT12_R(crate::FieldReader<bool, SWT12_A>);
impl SWT12_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWT12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT12_A {
        match self.bits {
            false => SWT12_A::SWT12_0,
            true => SWT12_A::SWT12_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT12_0`"]
    #[inline(always)]
    pub fn is_swt12_0(&self) -> bool {
        **self == SWT12_A::SWT12_0
    }
    #[doc = "Checks if the value of the field is `SWT12_1`"]
    #[inline(always)]
    pub fn is_swt12_1(&self) -> bool {
        **self == SWT12_A::SWT12_1
    }
}
impl core::ops::Deref for SWT12_R {
    type Target = crate::FieldReader<bool, SWT12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWT12` writer - Software trigger 12 event"]
pub struct SWT12_W<'a> {
    w: &'a mut W,
}
impl<'a> SWT12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWT12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No trigger 12 event generated."]
    #[inline(always)]
    pub fn swt12_0(self) -> &'a mut W {
        self.variant(SWT12_A::SWT12_0)
    }
    #[doc = "Trigger 12 event generated."]
    #[inline(always)]
    pub fn swt12_1(self) -> &'a mut W {
        self.variant(SWT12_A::SWT12_1)
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
#[doc = "Software trigger 13 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT13_A {
    #[doc = "0: No trigger 13 event generated."]
    SWT13_0 = 0,
    #[doc = "1: Trigger 13 event generated."]
    SWT13_1 = 1,
}
impl From<SWT13_A> for bool {
    #[inline(always)]
    fn from(variant: SWT13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT13` reader - Software trigger 13 event"]
pub struct SWT13_R(crate::FieldReader<bool, SWT13_A>);
impl SWT13_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWT13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT13_A {
        match self.bits {
            false => SWT13_A::SWT13_0,
            true => SWT13_A::SWT13_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT13_0`"]
    #[inline(always)]
    pub fn is_swt13_0(&self) -> bool {
        **self == SWT13_A::SWT13_0
    }
    #[doc = "Checks if the value of the field is `SWT13_1`"]
    #[inline(always)]
    pub fn is_swt13_1(&self) -> bool {
        **self == SWT13_A::SWT13_1
    }
}
impl core::ops::Deref for SWT13_R {
    type Target = crate::FieldReader<bool, SWT13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWT13` writer - Software trigger 13 event"]
pub struct SWT13_W<'a> {
    w: &'a mut W,
}
impl<'a> SWT13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWT13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No trigger 13 event generated."]
    #[inline(always)]
    pub fn swt13_0(self) -> &'a mut W {
        self.variant(SWT13_A::SWT13_0)
    }
    #[doc = "Trigger 13 event generated."]
    #[inline(always)]
    pub fn swt13_1(self) -> &'a mut W {
        self.variant(SWT13_A::SWT13_1)
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
#[doc = "Software trigger 14 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT14_A {
    #[doc = "0: No trigger 14 event generated."]
    SWT14_0 = 0,
    #[doc = "1: Trigger 14 event generated."]
    SWT14_1 = 1,
}
impl From<SWT14_A> for bool {
    #[inline(always)]
    fn from(variant: SWT14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT14` reader - Software trigger 14 event"]
pub struct SWT14_R(crate::FieldReader<bool, SWT14_A>);
impl SWT14_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWT14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT14_A {
        match self.bits {
            false => SWT14_A::SWT14_0,
            true => SWT14_A::SWT14_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT14_0`"]
    #[inline(always)]
    pub fn is_swt14_0(&self) -> bool {
        **self == SWT14_A::SWT14_0
    }
    #[doc = "Checks if the value of the field is `SWT14_1`"]
    #[inline(always)]
    pub fn is_swt14_1(&self) -> bool {
        **self == SWT14_A::SWT14_1
    }
}
impl core::ops::Deref for SWT14_R {
    type Target = crate::FieldReader<bool, SWT14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWT14` writer - Software trigger 14 event"]
pub struct SWT14_W<'a> {
    w: &'a mut W,
}
impl<'a> SWT14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWT14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No trigger 14 event generated."]
    #[inline(always)]
    pub fn swt14_0(self) -> &'a mut W {
        self.variant(SWT14_A::SWT14_0)
    }
    #[doc = "Trigger 14 event generated."]
    #[inline(always)]
    pub fn swt14_1(self) -> &'a mut W {
        self.variant(SWT14_A::SWT14_1)
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
#[doc = "Software trigger 15 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWT15_A {
    #[doc = "0: No trigger 15 event generated."]
    SWT15_0 = 0,
    #[doc = "1: Trigger 15 event generated."]
    SWT15_1 = 1,
}
impl From<SWT15_A> for bool {
    #[inline(always)]
    fn from(variant: SWT15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT15` reader - Software trigger 15 event"]
pub struct SWT15_R(crate::FieldReader<bool, SWT15_A>);
impl SWT15_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWT15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT15_A {
        match self.bits {
            false => SWT15_A::SWT15_0,
            true => SWT15_A::SWT15_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT15_0`"]
    #[inline(always)]
    pub fn is_swt15_0(&self) -> bool {
        **self == SWT15_A::SWT15_0
    }
    #[doc = "Checks if the value of the field is `SWT15_1`"]
    #[inline(always)]
    pub fn is_swt15_1(&self) -> bool {
        **self == SWT15_A::SWT15_1
    }
}
impl core::ops::Deref for SWT15_R {
    type Target = crate::FieldReader<bool, SWT15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWT15` writer - Software trigger 15 event"]
pub struct SWT15_W<'a> {
    w: &'a mut W,
}
impl<'a> SWT15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWT15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No trigger 15 event generated."]
    #[inline(always)]
    pub fn swt15_0(self) -> &'a mut W {
        self.variant(SWT15_A::SWT15_0)
    }
    #[doc = "Trigger 15 event generated."]
    #[inline(always)]
    pub fn swt15_1(self) -> &'a mut W {
        self.variant(SWT15_A::SWT15_1)
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
impl R {
    #[doc = "Bit 0 - Software trigger 0 event"]
    #[inline(always)]
    pub fn swt0(&self) -> SWT0_R {
        SWT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software trigger 1 event"]
    #[inline(always)]
    pub fn swt1(&self) -> SWT1_R {
        SWT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software trigger 2 event"]
    #[inline(always)]
    pub fn swt2(&self) -> SWT2_R {
        SWT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Software trigger 3 event"]
    #[inline(always)]
    pub fn swt3(&self) -> SWT3_R {
        SWT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Software trigger 4 event"]
    #[inline(always)]
    pub fn swt4(&self) -> SWT4_R {
        SWT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Software trigger 5 event"]
    #[inline(always)]
    pub fn swt5(&self) -> SWT5_R {
        SWT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Software trigger 6 event"]
    #[inline(always)]
    pub fn swt6(&self) -> SWT6_R {
        SWT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Software trigger 7 event"]
    #[inline(always)]
    pub fn swt7(&self) -> SWT7_R {
        SWT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Software trigger 8 event"]
    #[inline(always)]
    pub fn swt8(&self) -> SWT8_R {
        SWT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Software trigger 9 event"]
    #[inline(always)]
    pub fn swt9(&self) -> SWT9_R {
        SWT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Software trigger 10 event"]
    #[inline(always)]
    pub fn swt10(&self) -> SWT10_R {
        SWT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Software trigger 11 event"]
    #[inline(always)]
    pub fn swt11(&self) -> SWT11_R {
        SWT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Software trigger 12 event"]
    #[inline(always)]
    pub fn swt12(&self) -> SWT12_R {
        SWT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Software trigger 13 event"]
    #[inline(always)]
    pub fn swt13(&self) -> SWT13_R {
        SWT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Software trigger 14 event"]
    #[inline(always)]
    pub fn swt14(&self) -> SWT14_R {
        SWT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Software trigger 15 event"]
    #[inline(always)]
    pub fn swt15(&self) -> SWT15_R {
        SWT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software trigger 0 event"]
    #[inline(always)]
    pub fn swt0(&mut self) -> SWT0_W {
        SWT0_W { w: self }
    }
    #[doc = "Bit 1 - Software trigger 1 event"]
    #[inline(always)]
    pub fn swt1(&mut self) -> SWT1_W {
        SWT1_W { w: self }
    }
    #[doc = "Bit 2 - Software trigger 2 event"]
    #[inline(always)]
    pub fn swt2(&mut self) -> SWT2_W {
        SWT2_W { w: self }
    }
    #[doc = "Bit 3 - Software trigger 3 event"]
    #[inline(always)]
    pub fn swt3(&mut self) -> SWT3_W {
        SWT3_W { w: self }
    }
    #[doc = "Bit 4 - Software trigger 4 event"]
    #[inline(always)]
    pub fn swt4(&mut self) -> SWT4_W {
        SWT4_W { w: self }
    }
    #[doc = "Bit 5 - Software trigger 5 event"]
    #[inline(always)]
    pub fn swt5(&mut self) -> SWT5_W {
        SWT5_W { w: self }
    }
    #[doc = "Bit 6 - Software trigger 6 event"]
    #[inline(always)]
    pub fn swt6(&mut self) -> SWT6_W {
        SWT6_W { w: self }
    }
    #[doc = "Bit 7 - Software trigger 7 event"]
    #[inline(always)]
    pub fn swt7(&mut self) -> SWT7_W {
        SWT7_W { w: self }
    }
    #[doc = "Bit 8 - Software trigger 8 event"]
    #[inline(always)]
    pub fn swt8(&mut self) -> SWT8_W {
        SWT8_W { w: self }
    }
    #[doc = "Bit 9 - Software trigger 9 event"]
    #[inline(always)]
    pub fn swt9(&mut self) -> SWT9_W {
        SWT9_W { w: self }
    }
    #[doc = "Bit 10 - Software trigger 10 event"]
    #[inline(always)]
    pub fn swt10(&mut self) -> SWT10_W {
        SWT10_W { w: self }
    }
    #[doc = "Bit 11 - Software trigger 11 event"]
    #[inline(always)]
    pub fn swt11(&mut self) -> SWT11_W {
        SWT11_W { w: self }
    }
    #[doc = "Bit 12 - Software trigger 12 event"]
    #[inline(always)]
    pub fn swt12(&mut self) -> SWT12_W {
        SWT12_W { w: self }
    }
    #[doc = "Bit 13 - Software trigger 13 event"]
    #[inline(always)]
    pub fn swt13(&mut self) -> SWT13_W {
        SWT13_W { w: self }
    }
    #[doc = "Bit 14 - Software trigger 14 event"]
    #[inline(always)]
    pub fn swt14(&mut self) -> SWT14_W {
        SWT14_W { w: self }
    }
    #[doc = "Bit 15 - Software trigger 15 event"]
    #[inline(always)]
    pub fn swt15(&mut self) -> SWT15_W {
        SWT15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swtrig](index.html) module"]
pub struct SWTRIG_SPEC;
impl crate::RegisterSpec for SWTRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swtrig::R](R) reader structure"]
impl crate::Readable for SWTRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swtrig::W](W) writer structure"]
impl crate::Writable for SWTRIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWTRIG to value 0"]
impl crate::Resettable for SWTRIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
