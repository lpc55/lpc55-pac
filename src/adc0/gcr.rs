#[doc = "Register `GCR[%s]` reader"]
pub struct R(crate::R<GCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GCR_SPEC>> for R {
    fn from(reader: crate::R<GCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCR[%s]` writer"]
pub struct W(crate::W<GCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCR_SPEC>;
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
impl core::convert::From<crate::W<GCR_SPEC>> for W {
    fn from(writer: crate::W<GCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GCALR` reader - Gain Calculation Result"]
pub struct GCALR_R(crate::FieldReader<u16, u16>);
impl GCALR_R {
    pub(crate) fn new(bits: u16) -> Self {
        GCALR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCALR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCALR` writer - Gain Calculation Result"]
pub struct GCALR_W<'a> {
    w: &'a mut W,
}
impl<'a> GCALR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Gain Calculation Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDY_A {
    #[doc = "0: The gain offset calculation value is invalid."]
    RDY_0 = 0,
    #[doc = "1: The gain calibration value is valid."]
    RDY_1 = 1,
}
impl From<RDY_A> for bool {
    #[inline(always)]
    fn from(variant: RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDY` reader - Gain Calculation Ready"]
pub struct RDY_R(crate::FieldReader<bool, RDY_A>);
impl RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDY_A {
        match self.bits {
            false => RDY_A::RDY_0,
            true => RDY_A::RDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDY_0`"]
    #[inline(always)]
    pub fn is_rdy_0(&self) -> bool {
        **self == RDY_A::RDY_0
    }
    #[doc = "Checks if the value of the field is `RDY_1`"]
    #[inline(always)]
    pub fn is_rdy_1(&self) -> bool {
        **self == RDY_A::RDY_1
    }
}
impl core::ops::Deref for RDY_R {
    type Target = crate::FieldReader<bool, RDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDY` writer - Gain Calculation Ready"]
pub struct RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The gain offset calculation value is invalid."]
    #[inline(always)]
    pub fn rdy_0(self) -> &'a mut W {
        self.variant(RDY_A::RDY_0)
    }
    #[doc = "The gain calibration value is valid."]
    #[inline(always)]
    pub fn rdy_1(self) -> &'a mut W {
        self.variant(RDY_A::RDY_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Gain Calculation Result"]
    #[inline(always)]
    pub fn gcalr(&self) -> GCALR_R {
        GCALR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - Gain Calculation Ready"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Gain Calculation Result"]
    #[inline(always)]
    pub fn gcalr(&mut self) -> GCALR_W {
        GCALR_W { w: self }
    }
    #[doc = "Bit 24 - Gain Calculation Ready"]
    #[inline(always)]
    pub fn rdy(&mut self) -> RDY_W {
        RDY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gain Calculation Result\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcr](index.html) module"]
pub struct GCR_SPEC;
impl crate::RegisterSpec for GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcr::R](R) reader structure"]
impl crate::Readable for GCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcr::W](W) writer structure"]
impl crate::Writable for GCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCR[%s]
to value 0"]
impl crate::Resettable for GCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
