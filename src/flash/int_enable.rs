#[doc = "Register `INT_ENABLE` reader"]
pub struct R(crate::R<INT_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INT_ENABLE_SPEC>> for R {
    fn from(reader: crate::R<INT_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENABLE` writer"]
pub struct W(crate::W<INT_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENABLE_SPEC>;
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
impl core::convert::From<crate::W<INT_ENABLE_SPEC>> for W {
    fn from(writer: crate::W<INT_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAIL` reader - If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
pub struct FAIL_R(crate::FieldReader<bool, bool>);
impl FAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR` reader - If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
pub struct ERR_R(crate::FieldReader<bool, bool>);
impl ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DONE` reader - If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
pub struct DONE_R(crate::FieldReader<bool, bool>);
impl DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECC_ERR` reader - If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
pub struct ECC_ERR_R(crate::FieldReader<bool, bool>);
impl ECC_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECC_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECC_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[inline(always)]
    pub fn fail(&self) -> FAIL_R {
        FAIL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[inline(always)]
    pub fn ecc_err(&self) -> ECC_ERR_R {
        ECC_ERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_enable](index.html) module"]
pub struct INT_ENABLE_SPEC;
impl crate::RegisterSpec for INT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_enable::R](R) reader structure"]
impl crate::Readable for INT_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_enable::W](W) writer structure"]
impl crate::Writable for INT_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENABLE to value 0"]
impl crate::Resettable for INT_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
