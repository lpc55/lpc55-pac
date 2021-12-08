#[doc = "Register `ONLINE_TEST_VAL` reader"]
pub struct R(crate::R<ONLINE_TEST_VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ONLINE_TEST_VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ONLINE_TEST_VAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ONLINE_TEST_VAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ONLINE_TEST_VAL` writer"]
pub struct W(crate::W<ONLINE_TEST_VAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ONLINE_TEST_VAL_SPEC>;
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
impl From<crate::W<ONLINE_TEST_VAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ONLINE_TEST_VAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LIVE_CHI_SQUARED` reader - This value is updated as described in field 'activate'."]
pub struct LIVE_CHI_SQUARED_R(crate::FieldReader<u8, u8>);
impl LIVE_CHI_SQUARED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LIVE_CHI_SQUARED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LIVE_CHI_SQUARED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIN_CHI_SQUARED` reader - This field is reset when 'activate'==0."]
pub struct MIN_CHI_SQUARED_R(crate::FieldReader<u8, u8>);
impl MIN_CHI_SQUARED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MIN_CHI_SQUARED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIN_CHI_SQUARED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAX_CHI_SQUARED` reader - This field is reset when 'activate'==0."]
pub struct MAX_CHI_SQUARED_R(crate::FieldReader<u8, u8>);
impl MAX_CHI_SQUARED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAX_CHI_SQUARED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAX_CHI_SQUARED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - This value is updated as described in field 'activate'."]
    #[inline(always)]
    pub fn live_chi_squared(&self) -> LIVE_CHI_SQUARED_R {
        LIVE_CHI_SQUARED_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - This field is reset when 'activate'==0."]
    #[inline(always)]
    pub fn min_chi_squared(&self) -> MIN_CHI_SQUARED_R {
        MIN_CHI_SQUARED_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - This field is reset when 'activate'==0."]
    #[inline(always)]
    pub fn max_chi_squared(&self) -> MAX_CHI_SQUARED_R {
        MAX_CHI_SQUARED_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [online_test_val](index.html) module"]
pub struct ONLINE_TEST_VAL_SPEC;
impl crate::RegisterSpec for ONLINE_TEST_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [online_test_val::R](R) reader structure"]
impl crate::Readable for ONLINE_TEST_VAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [online_test_val::W](W) writer structure"]
impl crate::Writable for ONLINE_TEST_VAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ONLINE_TEST_VAL to value 0"]
impl crate::Resettable for ONLINE_TEST_VAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
