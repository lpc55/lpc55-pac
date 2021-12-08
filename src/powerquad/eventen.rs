#[doc = "Register `EVENTEN` reader"]
pub struct R(crate::R<EVENTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTEN` writer"]
pub struct W(crate::W<EVENTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTEN_SPEC>;
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
impl From<crate::W<EVENTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `event_oflow` reader - 1 : Enable event trigger on Floating point overflow"]
pub struct EVENT_OFLOW_R(crate::FieldReader<bool, bool>);
impl EVENT_OFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVENT_OFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENT_OFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `event_oflow` writer - 1 : Enable event trigger on Floating point overflow"]
pub struct EVENT_OFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENT_OFLOW_W<'a> {
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
#[doc = "Field `event_nan` reader - 1 : Enable event trigger on Floating point NaN"]
pub struct EVENT_NAN_R(crate::FieldReader<bool, bool>);
impl EVENT_NAN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVENT_NAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENT_NAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `event_nan` writer - 1 : Enable event trigger on Floating point NaN"]
pub struct EVENT_NAN_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENT_NAN_W<'a> {
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
#[doc = "Field `event_fixed` reader - 1: Enable event trigger on Fixed point Overflow"]
pub struct EVENT_FIXED_R(crate::FieldReader<bool, bool>);
impl EVENT_FIXED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVENT_FIXED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENT_FIXED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `event_fixed` writer - 1: Enable event trigger on Fixed point Overflow"]
pub struct EVENT_FIXED_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENT_FIXED_W<'a> {
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
#[doc = "Field `event_uflow` reader - 1 : Enable event trigger on Subnormal truncation"]
pub struct EVENT_UFLOW_R(crate::FieldReader<bool, bool>);
impl EVENT_UFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVENT_UFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENT_UFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `event_uflow` writer - 1 : Enable event trigger on Subnormal truncation"]
pub struct EVENT_UFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENT_UFLOW_W<'a> {
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
#[doc = "Field `event_berr` reader - 1: Enable event trigger on AHBM Buss Error"]
pub struct EVENT_BERR_R(crate::FieldReader<bool, bool>);
impl EVENT_BERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVENT_BERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENT_BERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `event_berr` writer - 1: Enable event trigger on AHBM Buss Error"]
pub struct EVENT_BERR_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENT_BERR_W<'a> {
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
#[doc = "Field `event_comp` reader - 1: Enable event trigger on instruction completion"]
pub struct EVENT_COMP_R(crate::FieldReader<bool, bool>);
impl EVENT_COMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVENT_COMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENT_COMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `event_comp` writer - 1: Enable event trigger on instruction completion"]
pub struct EVENT_COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENT_COMP_W<'a> {
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
    #[doc = "Bit 0 - 1 : Enable event trigger on Floating point overflow"]
    #[inline(always)]
    pub fn event_oflow(&self) -> EVENT_OFLOW_R {
        EVENT_OFLOW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1 : Enable event trigger on Floating point NaN"]
    #[inline(always)]
    pub fn event_nan(&self) -> EVENT_NAN_R {
        EVENT_NAN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1: Enable event trigger on Fixed point Overflow"]
    #[inline(always)]
    pub fn event_fixed(&self) -> EVENT_FIXED_R {
        EVENT_FIXED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1 : Enable event trigger on Subnormal truncation"]
    #[inline(always)]
    pub fn event_uflow(&self) -> EVENT_UFLOW_R {
        EVENT_UFLOW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1: Enable event trigger on AHBM Buss Error"]
    #[inline(always)]
    pub fn event_berr(&self) -> EVENT_BERR_R {
        EVENT_BERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 1: Enable event trigger on instruction completion"]
    #[inline(always)]
    pub fn event_comp(&self) -> EVENT_COMP_R {
        EVENT_COMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1 : Enable event trigger on Floating point overflow"]
    #[inline(always)]
    pub fn event_oflow(&mut self) -> EVENT_OFLOW_W {
        EVENT_OFLOW_W { w: self }
    }
    #[doc = "Bit 1 - 1 : Enable event trigger on Floating point NaN"]
    #[inline(always)]
    pub fn event_nan(&mut self) -> EVENT_NAN_W {
        EVENT_NAN_W { w: self }
    }
    #[doc = "Bit 2 - 1: Enable event trigger on Fixed point Overflow"]
    #[inline(always)]
    pub fn event_fixed(&mut self) -> EVENT_FIXED_W {
        EVENT_FIXED_W { w: self }
    }
    #[doc = "Bit 3 - 1 : Enable event trigger on Subnormal truncation"]
    #[inline(always)]
    pub fn event_uflow(&mut self) -> EVENT_UFLOW_W {
        EVENT_UFLOW_W { w: self }
    }
    #[doc = "Bit 4 - 1: Enable event trigger on AHBM Buss Error"]
    #[inline(always)]
    pub fn event_berr(&mut self) -> EVENT_BERR_W {
        EVENT_BERR_W { w: self }
    }
    #[doc = "Bit 7 - 1: Enable event trigger on instruction completion"]
    #[inline(always)]
    pub fn event_comp(&mut self) -> EVENT_COMP_W {
        EVENT_COMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eventen](index.html) module"]
pub struct EVENTEN_SPEC;
impl crate::RegisterSpec for EVENTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eventen::R](R) reader structure"]
impl crate::Readable for EVENTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eventen::W](W) writer structure"]
impl crate::Writable for EVENTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTEN to value 0"]
impl crate::Resettable for EVENTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
