#[doc = "Register `CPU0NSTCKCAL` reader"]
pub struct R(crate::R<CPU0NSTCKCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU0NSTCKCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU0NSTCKCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU0NSTCKCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU0NSTCKCAL` writer"]
pub struct W(crate::W<CPU0NSTCKCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU0NSTCKCAL_SPEC>;
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
impl From<crate::W<CPU0NSTCKCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU0NSTCKCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TENMS` reader - Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
pub struct TENMS_R(crate::FieldReader<u32, u32>);
impl TENMS_R {
    #[inline(always)]
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
#[doc = "Field `TENMS` writer - Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
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
#[doc = "Field `SKEW` reader - Indicates whether the TENMS value is exact: 0 = TENMS value is exact; 1 = TENMS value is inexact, or not given."]
pub struct SKEW_R(crate::FieldReader<bool, bool>);
impl SKEW_R {
    #[inline(always)]
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
#[doc = "Field `SKEW` writer - Indicates whether the TENMS value is exact: 0 = TENMS value is exact; 1 = TENMS value is inexact, or not given."]
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
#[doc = "Field `NOREF` reader - Initial value for the Systick timer."]
pub struct NOREF_R(crate::FieldReader<bool, bool>);
impl NOREF_R {
    #[inline(always)]
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
#[doc = "Field `NOREF` writer - Initial value for the Systick timer."]
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
    #[doc = "Bits 0:23 - Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - Indicates whether the TENMS value is exact: 0 = TENMS value is exact; 1 = TENMS value is inexact, or not given."]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Initial value for the Systick timer."]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub fn tenms(&mut self) -> TENMS_W {
        TENMS_W { w: self }
    }
    #[doc = "Bit 24 - Indicates whether the TENMS value is exact: 0 = TENMS value is exact; 1 = TENMS value is inexact, or not given."]
    #[inline(always)]
    pub fn skew(&mut self) -> SKEW_W {
        SKEW_W { w: self }
    }
    #[doc = "Bit 25 - Initial value for the Systick timer."]
    #[inline(always)]
    pub fn noref(&mut self) -> NOREF_W {
        NOREF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System tick calibration for non-secure part of CPU0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu0nstckcal](index.html) module"]
pub struct CPU0NSTCKCAL_SPEC;
impl crate::RegisterSpec for CPU0NSTCKCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu0nstckcal::R](R) reader structure"]
impl crate::Readable for CPU0NSTCKCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu0nstckcal::W](W) writer structure"]
impl crate::Writable for CPU0NSTCKCAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU0NSTCKCAL to value 0"]
impl crate::Resettable for CPU0NSTCKCAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
