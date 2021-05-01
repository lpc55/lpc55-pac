#[doc = "Register `GCC[%s]` reader"]
pub struct R(crate::R<GCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GCC_SPEC>> for R {
    fn from(reader: crate::R<GCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GAIN_CAL` reader - Gain Calibration Value"]
pub struct GAIN_CAL_R(crate::FieldReader<u16, u16>);
impl GAIN_CAL_R {
    pub(crate) fn new(bits: u16) -> Self {
        GAIN_CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Gain Calibration Value Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDY_A {
    #[doc = "0: The gain calibration value is invalid. Run the auto-calibration routine for this value to be written."]
    RDY_0 = 0,
    #[doc = "1: The gain calibration value is valid. It should be used to update the GCRa\\[GCALR\\]
register field."]
    RDY_1 = 1,
}
impl From<RDY_A> for bool {
    #[inline(always)]
    fn from(variant: RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDY` reader - Gain Calibration Value Valid"]
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
impl R {
    #[doc = "Bits 0:15 - Gain Calibration Value"]
    #[inline(always)]
    pub fn gain_cal(&self) -> GAIN_CAL_R {
        GAIN_CAL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - Gain Calibration Value Valid"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
#[doc = "Gain Calibration Control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcc](index.html) module"]
pub struct GCC_SPEC;
impl crate::RegisterSpec for GCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcc::R](R) reader structure"]
impl crate::Readable for GCC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GCC[%s]
to value 0"]
impl crate::Resettable for GCC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
