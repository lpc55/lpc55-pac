#[doc = "Register `CAL_GBR[%s]` reader"]
pub struct R(crate::R<CAL_GBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_GBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_GBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_GBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL_GBR[%s]` writer"]
pub struct W(crate::W<CAL_GBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL_GBR_SPEC>;
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
impl From<crate::W<CAL_GBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL_GBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL_GBR_VAL` reader - Calibration General B Side Register Element"]
pub struct CAL_GBR_VAL_R(crate::FieldReader<u16, u16>);
impl CAL_GBR_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CAL_GBR_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL_GBR_VAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAL_GBR_VAL` writer - Calibration General B Side Register Element"]
pub struct CAL_GBR_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_GBR_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Calibration General B Side Register Element"]
    #[inline(always)]
    pub fn cal_gbr_val(&self) -> CAL_GBR_VAL_R {
        CAL_GBR_VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Calibration General B Side Register Element"]
    #[inline(always)]
    pub fn cal_gbr_val(&mut self) -> CAL_GBR_VAL_W {
        CAL_GBR_VAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration General B-Side Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_gbr](index.html) module"]
pub struct CAL_GBR_SPEC;
impl crate::RegisterSpec for CAL_GBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal_gbr::R](R) reader structure"]
impl crate::Readable for CAL_GBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal_gbr::W](W) writer structure"]
impl crate::Writable for CAL_GBR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAL_GBR[%s]
to value 0"]
impl crate::Resettable for CAL_GBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
