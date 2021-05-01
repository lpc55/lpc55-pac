#[doc = "Register `COUNTER_VAL` reader"]
pub struct R(crate::R<COUNTER_VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNTER_VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<COUNTER_VAL_SPEC>> for R {
    fn from(reader: crate::R<COUNTER_VAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNTER_VAL` writer"]
pub struct W(crate::W<COUNTER_VAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNTER_VAL_SPEC>;
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
impl core::convert::From<crate::W<COUNTER_VAL_SPEC>> for W {
    fn from(writer: crate::W<COUNTER_VAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_RATIO` reader - Gives the ratio between the internal clocks frequencies and the register clock frequency for evaluation and certification purposes."]
pub struct CLK_RATIO_R(crate::FieldReader<u8, u8>);
impl CLK_RATIO_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLK_RATIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_RATIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFRESH_CNT` reader - Incremented (till max possible value) each time COUNTER was updated since last reading to any *_NUMBER."]
pub struct REFRESH_CNT_R(crate::FieldReader<u8, u8>);
impl REFRESH_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        REFRESH_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFRESH_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Gives the ratio between the internal clocks frequencies and the register clock frequency for evaluation and certification purposes."]
    #[inline(always)]
    pub fn clk_ratio(&self) -> CLK_RATIO_R {
        CLK_RATIO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Incremented (till max possible value) each time COUNTER was updated since last reading to any *_NUMBER."]
    #[inline(always)]
    pub fn refresh_cnt(&self) -> REFRESH_CNT_R {
        REFRESH_CNT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [counter_val](index.html) module"]
pub struct COUNTER_VAL_SPEC;
impl crate::RegisterSpec for COUNTER_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [counter_val::R](R) reader structure"]
impl crate::Readable for COUNTER_VAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [counter_val::W](W) writer structure"]
impl crate::Writable for COUNTER_VAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COUNTER_VAL to value 0"]
impl crate::Resettable for COUNTER_VAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
