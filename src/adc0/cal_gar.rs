#[doc = "Register `CAL_GAR[%s]` reader"]
pub struct R(crate::R<CAL_GAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_GAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAL_GAR_SPEC>> for R {
    fn from(reader: crate::R<CAL_GAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL_GAR[%s]` writer"]
pub struct W(crate::W<CAL_GAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL_GAR_SPEC>;
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
impl core::convert::From<crate::W<CAL_GAR_SPEC>> for W {
    fn from(writer: crate::W<CAL_GAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL_GAR_VAL` reader - Calibration General A Side Register Element"]
pub struct CAL_GAR_VAL_R(crate::FieldReader<u16, u16>);
impl CAL_GAR_VAL_R {
    pub(crate) fn new(bits: u16) -> Self {
        CAL_GAR_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL_GAR_VAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAL_GAR_VAL` writer - Calibration General A Side Register Element"]
pub struct CAL_GAR_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_GAR_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Calibration General A Side Register Element"]
    #[inline(always)]
    pub fn cal_gar_val(&self) -> CAL_GAR_VAL_R {
        CAL_GAR_VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Calibration General A Side Register Element"]
    #[inline(always)]
    pub fn cal_gar_val(&mut self) -> CAL_GAR_VAL_W {
        CAL_GAR_VAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration General A-Side Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_gar](index.html) module"]
pub struct CAL_GAR_SPEC;
impl crate::RegisterSpec for CAL_GAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal_gar::R](R) reader structure"]
impl crate::Readable for CAL_GAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal_gar::W](W) writer structure"]
impl crate::Writable for CAL_GAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAL_GAR[%s]
to value 0"]
impl crate::Resettable for CAL_GAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
