#[doc = "Register `CPU0STCKCAL` reader"]
pub struct R(crate::R<CPU0STCKCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU0STCKCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CPU0STCKCAL_SPEC>> for R {
    fn from(reader: crate::R<CPU0STCKCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU0STCKCAL` writer"]
pub struct W(crate::W<CPU0STCKCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU0STCKCAL_SPEC>;
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
impl core::convert::From<crate::W<CPU0STCKCAL_SPEC>> for W {
    fn from(writer: crate::W<CPU0STCKCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TENMS` reader - Reload value for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
pub struct TENMS_R(crate::FieldReader<u32, u32>);
impl TENMS_R {
    pub(crate) fn new(bits: u32) -> Self {
        TENMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TENMS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TENMS` writer - Reload value for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
pub struct TENMS_W<'a> {
    w: &'a mut W,
}
impl<'a> TENMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Field `SKEW` reader - Initial value for the Systick timer."]
pub struct SKEW_R(crate::FieldReader<bool, bool>);
impl SKEW_R {
    pub(crate) fn new(bits: bool) -> Self {
        SKEW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SKEW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SKEW` writer - Initial value for the Systick timer."]
pub struct SKEW_W<'a> {
    w: &'a mut W,
}
impl<'a> SKEW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `NOREF` reader - Indicates whether the device provides a reference clock to the processor: 0 = reference clock provided; 1 = no reference clock provided."]
pub struct NOREF_R(crate::FieldReader<bool, bool>);
impl NOREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        NOREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOREF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOREF` writer - Indicates whether the device provides a reference clock to the processor: 0 = reference clock provided; 1 = no reference clock provided."]
pub struct NOREF_W<'a> {
    w: &'a mut W,
}
impl<'a> NOREF_W<'a> {
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
impl R {
    #[doc = "Bits 0:23 - Reload value for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - Initial value for the Systick timer."]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Indicates whether the device provides a reference clock to the processor: 0 = reference clock provided; 1 = no reference clock provided."]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Reload value for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub fn tenms(&mut self) -> TENMS_W {
        TENMS_W { w: self }
    }
    #[doc = "Bit 24 - Initial value for the Systick timer."]
    #[inline(always)]
    pub fn skew(&mut self) -> SKEW_W {
        SKEW_W { w: self }
    }
    #[doc = "Bit 25 - Indicates whether the device provides a reference clock to the processor: 0 = reference clock provided; 1 = no reference clock provided."]
    #[inline(always)]
    pub fn noref(&mut self) -> NOREF_W {
        NOREF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System tick calibration for secure part of CPU0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu0stckcal](index.html) module"]
pub struct CPU0STCKCAL_SPEC;
impl crate::RegisterSpec for CPU0STCKCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu0stckcal::R](R) reader structure"]
impl crate::Readable for CPU0STCKCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu0stckcal::W](W) writer structure"]
impl crate::Writable for CPU0STCKCAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU0STCKCAL to value 0"]
impl crate::Resettable for CPU0STCKCAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
