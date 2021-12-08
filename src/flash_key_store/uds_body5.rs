#[doc = "Register `UDS_BODY5` reader"]
pub struct R(crate::R<UDS_BODY5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UDS_BODY5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UDS_BODY5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UDS_BODY5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UDS_BODY5` writer"]
pub struct W(crate::W<UDS_BODY5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDS_BODY5_SPEC>;
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
impl From<crate::W<UDS_BODY5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDS_BODY5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIELD` reader - ."]
pub struct FIELD_R(crate::FieldReader<u32, u32>);
impl FIELD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        FIELD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIELD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIELD` writer - ."]
pub struct FIELD_W<'a> {
    w: &'a mut W,
}
impl<'a> FIELD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&self) -> FIELD_R {
        FIELD_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&mut self) -> FIELD_W {
        FIELD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uds_body5](index.html) module"]
pub struct UDS_BODY5_SPEC;
impl crate::RegisterSpec for UDS_BODY5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uds_body5::R](R) reader structure"]
impl crate::Readable for UDS_BODY5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uds_body5::W](W) writer structure"]
impl crate::Writable for UDS_BODY5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UDS_BODY5 to value 0"]
impl crate::Resettable for UDS_BODY5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
