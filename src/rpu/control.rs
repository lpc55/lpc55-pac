#[doc = "Register `CONTROL` reader"]
pub struct R(crate::R<CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTROL` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl From<crate::W<CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PATCH0` reader - Patch 0 control bit"]
pub struct PATCH0_R(crate::FieldReader<bool, bool>);
impl PATCH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATCH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATCH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATCH0` writer - Patch 0 control bit"]
pub struct PATCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH0_W<'a> {
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
#[doc = "Field `PATCH1` reader - Patch 1 control bit"]
pub struct PATCH1_R(crate::FieldReader<bool, bool>);
impl PATCH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATCH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATCH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATCH1` writer - Patch 1 control bit"]
pub struct PATCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH1_W<'a> {
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
#[doc = "Field `PATCH2` reader - Patch 2 control bit"]
pub struct PATCH2_R(crate::FieldReader<bool, bool>);
impl PATCH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATCH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATCH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATCH2` writer - Patch 2 control bit"]
pub struct PATCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH2_W<'a> {
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
#[doc = "Field `PATCH3` reader - Patch 3 control bit"]
pub struct PATCH3_R(crate::FieldReader<bool, bool>);
impl PATCH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATCH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATCH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATCH3` writer - Patch 3 control bit"]
pub struct PATCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH3_W<'a> {
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
#[doc = "Field `PATCH4` reader - Patch 4 control bit"]
pub struct PATCH4_R(crate::FieldReader<bool, bool>);
impl PATCH4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATCH4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATCH4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATCH4` writer - Patch 4 control bit"]
pub struct PATCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH4_W<'a> {
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
#[doc = "Field `PATCH5` reader - Patch 5 control bit"]
pub struct PATCH5_R(crate::FieldReader<bool, bool>);
impl PATCH5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATCH5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATCH5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATCH5` writer - Patch 5 control bit"]
pub struct PATCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH5_W<'a> {
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
#[doc = "Field `PATCH6` reader - Patch 6 control bit"]
pub struct PATCH6_R(crate::FieldReader<bool, bool>);
impl PATCH6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATCH6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATCH6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATCH6` writer - Patch 6 control bit"]
pub struct PATCH6_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH6_W<'a> {
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
#[doc = "Field `PATCH7` reader - Patch 7 control bit"]
pub struct PATCH7_R(crate::FieldReader<bool, bool>);
impl PATCH7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATCH7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATCH7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATCH7` writer - Patch 7 control bit"]
pub struct PATCH7_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH7_W<'a> {
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
#[doc = "Field `PATCH8` reader - Patch 8 control bit"]
pub struct PATCH8_R(crate::FieldReader<bool, bool>);
impl PATCH8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATCH8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATCH8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATCH8` writer - Patch 8 control bit"]
pub struct PATCH8_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH8_W<'a> {
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
#[doc = "Field `PATCH9` reader - Patch 9 control bit"]
pub struct PATCH9_R(crate::FieldReader<bool, bool>);
impl PATCH9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATCH9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATCH9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATCH9` writer - Patch 9 control bit"]
pub struct PATCH9_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH9_W<'a> {
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
#[doc = "Field `PATCH10` reader - Patch 10 control bit"]
pub struct PATCH10_R(crate::FieldReader<bool, bool>);
impl PATCH10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATCH10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATCH10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATCH10` writer - Patch 10 control bit"]
pub struct PATCH10_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH10_W<'a> {
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
#[doc = "Field `PATCH11` reader - Patch 11 control bit"]
pub struct PATCH11_R(crate::FieldReader<bool, bool>);
impl PATCH11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATCH11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATCH11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATCH11` writer - Patch 11 control bit"]
pub struct PATCH11_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH11_W<'a> {
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
#[doc = "Field `PATCH12` reader - Patch 12 control bit"]
pub struct PATCH12_R(crate::FieldReader<bool, bool>);
impl PATCH12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATCH12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATCH12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATCH12` writer - Patch 12 control bit"]
pub struct PATCH12_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH12_W<'a> {
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
#[doc = "Field `PATCH13` reader - Patch 13 control bit"]
pub struct PATCH13_R(crate::FieldReader<bool, bool>);
impl PATCH13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATCH13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATCH13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATCH13` writer - Patch 13 control bit"]
pub struct PATCH13_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH13_W<'a> {
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
#[doc = "Field `PATCH14` reader - Patch 14 control bit"]
pub struct PATCH14_R(crate::FieldReader<bool, bool>);
impl PATCH14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATCH14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATCH14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATCH14` writer - Patch 14 control bit"]
pub struct PATCH14_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH14_W<'a> {
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
#[doc = "Field `PATCH15` reader - Patch 15 control bit"]
pub struct PATCH15_R(crate::FieldReader<bool, bool>);
impl PATCH15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATCH15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATCH15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATCH15` writer - Patch 15 control bit"]
pub struct PATCH15_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH15_W<'a> {
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
#[doc = "Field `DISABLE` reader - disable bit"]
pub struct DISABLE_R(crate::FieldReader<bool, bool>);
impl DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISABLE` writer - disable bit"]
pub struct DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Patch 0 control bit"]
    #[inline(always)]
    pub fn patch0(&self) -> PATCH0_R {
        PATCH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Patch 1 control bit"]
    #[inline(always)]
    pub fn patch1(&self) -> PATCH1_R {
        PATCH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Patch 2 control bit"]
    #[inline(always)]
    pub fn patch2(&self) -> PATCH2_R {
        PATCH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Patch 3 control bit"]
    #[inline(always)]
    pub fn patch3(&self) -> PATCH3_R {
        PATCH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Patch 4 control bit"]
    #[inline(always)]
    pub fn patch4(&self) -> PATCH4_R {
        PATCH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Patch 5 control bit"]
    #[inline(always)]
    pub fn patch5(&self) -> PATCH5_R {
        PATCH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Patch 6 control bit"]
    #[inline(always)]
    pub fn patch6(&self) -> PATCH6_R {
        PATCH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Patch 7 control bit"]
    #[inline(always)]
    pub fn patch7(&self) -> PATCH7_R {
        PATCH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Patch 8 control bit"]
    #[inline(always)]
    pub fn patch8(&self) -> PATCH8_R {
        PATCH8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Patch 9 control bit"]
    #[inline(always)]
    pub fn patch9(&self) -> PATCH9_R {
        PATCH9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Patch 10 control bit"]
    #[inline(always)]
    pub fn patch10(&self) -> PATCH10_R {
        PATCH10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Patch 11 control bit"]
    #[inline(always)]
    pub fn patch11(&self) -> PATCH11_R {
        PATCH11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Patch 12 control bit"]
    #[inline(always)]
    pub fn patch12(&self) -> PATCH12_R {
        PATCH12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Patch 13 control bit"]
    #[inline(always)]
    pub fn patch13(&self) -> PATCH13_R {
        PATCH13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Patch 14 control bit"]
    #[inline(always)]
    pub fn patch14(&self) -> PATCH14_R {
        PATCH14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Patch 15 control bit"]
    #[inline(always)]
    pub fn patch15(&self) -> PATCH15_R {
        PATCH15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 29 - disable bit"]
    #[inline(always)]
    pub fn disable(&self) -> DISABLE_R {
        DISABLE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Patch 0 control bit"]
    #[inline(always)]
    pub fn patch0(&mut self) -> PATCH0_W {
        PATCH0_W { w: self }
    }
    #[doc = "Bit 1 - Patch 1 control bit"]
    #[inline(always)]
    pub fn patch1(&mut self) -> PATCH1_W {
        PATCH1_W { w: self }
    }
    #[doc = "Bit 2 - Patch 2 control bit"]
    #[inline(always)]
    pub fn patch2(&mut self) -> PATCH2_W {
        PATCH2_W { w: self }
    }
    #[doc = "Bit 3 - Patch 3 control bit"]
    #[inline(always)]
    pub fn patch3(&mut self) -> PATCH3_W {
        PATCH3_W { w: self }
    }
    #[doc = "Bit 4 - Patch 4 control bit"]
    #[inline(always)]
    pub fn patch4(&mut self) -> PATCH4_W {
        PATCH4_W { w: self }
    }
    #[doc = "Bit 5 - Patch 5 control bit"]
    #[inline(always)]
    pub fn patch5(&mut self) -> PATCH5_W {
        PATCH5_W { w: self }
    }
    #[doc = "Bit 6 - Patch 6 control bit"]
    #[inline(always)]
    pub fn patch6(&mut self) -> PATCH6_W {
        PATCH6_W { w: self }
    }
    #[doc = "Bit 7 - Patch 7 control bit"]
    #[inline(always)]
    pub fn patch7(&mut self) -> PATCH7_W {
        PATCH7_W { w: self }
    }
    #[doc = "Bit 8 - Patch 8 control bit"]
    #[inline(always)]
    pub fn patch8(&mut self) -> PATCH8_W {
        PATCH8_W { w: self }
    }
    #[doc = "Bit 9 - Patch 9 control bit"]
    #[inline(always)]
    pub fn patch9(&mut self) -> PATCH9_W {
        PATCH9_W { w: self }
    }
    #[doc = "Bit 10 - Patch 10 control bit"]
    #[inline(always)]
    pub fn patch10(&mut self) -> PATCH10_W {
        PATCH10_W { w: self }
    }
    #[doc = "Bit 11 - Patch 11 control bit"]
    #[inline(always)]
    pub fn patch11(&mut self) -> PATCH11_W {
        PATCH11_W { w: self }
    }
    #[doc = "Bit 12 - Patch 12 control bit"]
    #[inline(always)]
    pub fn patch12(&mut self) -> PATCH12_W {
        PATCH12_W { w: self }
    }
    #[doc = "Bit 13 - Patch 13 control bit"]
    #[inline(always)]
    pub fn patch13(&mut self) -> PATCH13_W {
        PATCH13_W { w: self }
    }
    #[doc = "Bit 14 - Patch 14 control bit"]
    #[inline(always)]
    pub fn patch14(&mut self) -> PATCH14_W {
        PATCH14_W { w: self }
    }
    #[doc = "Bit 15 - Patch 15 control bit"]
    #[inline(always)]
    pub fn patch15(&mut self) -> PATCH15_W {
        PATCH15_W { w: self }
    }
    #[doc = "Bit 29 - disable bit"]
    #[inline(always)]
    pub fn disable(&mut self) -> DISABLE_W {
        DISABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control::R](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
