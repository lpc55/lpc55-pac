#[doc = "Register `INTREN` reader"]
pub struct R(crate::R<INTREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTREN` writer"]
pub struct W(crate::W<INTREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTREN_SPEC>;
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
impl From<crate::W<INTREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `intr_oflow` reader - 1 : Enable interrupt on Floating point overflow"]
pub struct INTR_OFLOW_R(crate::FieldReader<bool, bool>);
impl INTR_OFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTR_OFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTR_OFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `intr_oflow` writer - 1 : Enable interrupt on Floating point overflow"]
pub struct INTR_OFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_OFLOW_W<'a> {
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
#[doc = "Field `intr_nan` reader - 1 : Enable interrupt on Floating point NaN"]
pub struct INTR_NAN_R(crate::FieldReader<bool, bool>);
impl INTR_NAN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTR_NAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTR_NAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `intr_nan` writer - 1 : Enable interrupt on Floating point NaN"]
pub struct INTR_NAN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_NAN_W<'a> {
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
#[doc = "Field `intr_fixed` reader - 1: Enable interrupt on Fixed point Overflow"]
pub struct INTR_FIXED_R(crate::FieldReader<bool, bool>);
impl INTR_FIXED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTR_FIXED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTR_FIXED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `intr_fixed` writer - 1: Enable interrupt on Fixed point Overflow"]
pub struct INTR_FIXED_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_FIXED_W<'a> {
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
#[doc = "Field `intr_uflow` reader - 1 : Enable interrupt on Subnormal truncation"]
pub struct INTR_UFLOW_R(crate::FieldReader<bool, bool>);
impl INTR_UFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTR_UFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTR_UFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `intr_uflow` writer - 1 : Enable interrupt on Subnormal truncation"]
pub struct INTR_UFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_UFLOW_W<'a> {
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
#[doc = "Field `intr_berr` reader - 1: Enable interrupt on AHBM Buss Error"]
pub struct INTR_BERR_R(crate::FieldReader<bool, bool>);
impl INTR_BERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTR_BERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTR_BERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `intr_berr` writer - 1: Enable interrupt on AHBM Buss Error"]
pub struct INTR_BERR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_BERR_W<'a> {
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
#[doc = "Field `intr_comp` reader - 1: Enable interrupt on instruction completion"]
pub struct INTR_COMP_R(crate::FieldReader<bool, bool>);
impl INTR_COMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTR_COMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTR_COMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `intr_comp` writer - 1: Enable interrupt on instruction completion"]
pub struct INTR_COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_COMP_W<'a> {
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
    #[doc = "Bit 0 - 1 : Enable interrupt on Floating point overflow"]
    #[inline(always)]
    pub fn intr_oflow(&self) -> INTR_OFLOW_R {
        INTR_OFLOW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1 : Enable interrupt on Floating point NaN"]
    #[inline(always)]
    pub fn intr_nan(&self) -> INTR_NAN_R {
        INTR_NAN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1: Enable interrupt on Fixed point Overflow"]
    #[inline(always)]
    pub fn intr_fixed(&self) -> INTR_FIXED_R {
        INTR_FIXED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1 : Enable interrupt on Subnormal truncation"]
    #[inline(always)]
    pub fn intr_uflow(&self) -> INTR_UFLOW_R {
        INTR_UFLOW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1: Enable interrupt on AHBM Buss Error"]
    #[inline(always)]
    pub fn intr_berr(&self) -> INTR_BERR_R {
        INTR_BERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 1: Enable interrupt on instruction completion"]
    #[inline(always)]
    pub fn intr_comp(&self) -> INTR_COMP_R {
        INTR_COMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1 : Enable interrupt on Floating point overflow"]
    #[inline(always)]
    pub fn intr_oflow(&mut self) -> INTR_OFLOW_W {
        INTR_OFLOW_W { w: self }
    }
    #[doc = "Bit 1 - 1 : Enable interrupt on Floating point NaN"]
    #[inline(always)]
    pub fn intr_nan(&mut self) -> INTR_NAN_W {
        INTR_NAN_W { w: self }
    }
    #[doc = "Bit 2 - 1: Enable interrupt on Fixed point Overflow"]
    #[inline(always)]
    pub fn intr_fixed(&mut self) -> INTR_FIXED_W {
        INTR_FIXED_W { w: self }
    }
    #[doc = "Bit 3 - 1 : Enable interrupt on Subnormal truncation"]
    #[inline(always)]
    pub fn intr_uflow(&mut self) -> INTR_UFLOW_W {
        INTR_UFLOW_W { w: self }
    }
    #[doc = "Bit 4 - 1: Enable interrupt on AHBM Buss Error"]
    #[inline(always)]
    pub fn intr_berr(&mut self) -> INTR_BERR_W {
        INTR_BERR_W { w: self }
    }
    #[doc = "Bit 7 - 1: Enable interrupt on instruction completion"]
    #[inline(always)]
    pub fn intr_comp(&mut self) -> INTR_COMP_W {
        INTR_COMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INTERRUPT enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intren](index.html) module"]
pub struct INTREN_SPEC;
impl crate::RegisterSpec for INTREN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intren::R](R) reader structure"]
impl crate::Readable for INTREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intren::W](W) writer structure"]
impl crate::Writable for INTREN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTREN to value 0"]
impl crate::Resettable for INTREN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
