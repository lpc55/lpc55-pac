#[doc = "Register `INT_STATUS` reader"]
pub struct R(crate::R<INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_STATUS` writer"]
pub struct W(crate::W<INT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_STATUS_SPEC>;
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
impl From<crate::W<INT_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAIL` reader - This status bit is set if execution of a (legal) command failed."]
pub struct FAIL_R(crate::FieldReader<bool, bool>);
impl FAIL_R {
    #[inline(always)]
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
#[doc = "Field `ERR` reader - This status bit is set if execution of an illegal command is detected."]
pub struct ERR_R(crate::FieldReader<bool, bool>);
impl ERR_R {
    #[inline(always)]
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
#[doc = "Field `DONE` reader - This status bit is set at the end of command execution."]
pub struct DONE_R(crate::FieldReader<bool, bool>);
impl DONE_R {
    #[inline(always)]
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
#[doc = "Field `ECC_ERR` reader - This status bit is set if, during a memory read operation (either a user-requested read, or a speculative read, or reads performed by a controller command), a correctable or uncorrectable error is detected by ECC decoding logic."]
pub struct ECC_ERR_R(crate::FieldReader<bool, bool>);
impl ECC_ERR_R {
    #[inline(always)]
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
    #[doc = "Bit 0 - This status bit is set if execution of a (legal) command failed."]
    #[inline(always)]
    pub fn fail(&self) -> FAIL_R {
        FAIL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This status bit is set if execution of an illegal command is detected."]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This status bit is set at the end of command execution."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This status bit is set if, during a memory read operation (either a user-requested read, or a speculative read, or reads performed by a controller command), a correctable or uncorrectable error is detected by ECC decoding logic."]
    #[inline(always)]
    pub fn ecc_err(&self) -> ECC_ERR_R {
        ECC_ERR_R::new(((self.bits >> 3) & 0x01) != 0)
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
#[doc = "Interrupt status bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status](index.html) module"]
pub struct INT_STATUS_SPEC;
impl crate::RegisterSpec for INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_status::R](R) reader structure"]
impl crate::Readable for INT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_status::W](W) writer structure"]
impl crate::Writable for INT_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_STATUS to value 0"]
impl crate::Resettable for INT_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
