#[doc = "Register `ALLOW` reader"]
pub struct R(crate::R<ALLOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALLOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALLOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALLOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALLOW` writer"]
pub struct W(crate::W<ALLOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALLOW_SPEC>;
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
impl From<crate::W<ALLOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALLOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALLOWENROLL` reader - Enroll operation is allowed"]
pub struct ALLOWENROLL_R(crate::FieldReader<bool, bool>);
impl ALLOWENROLL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALLOWENROLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALLOWENROLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALLOWSTART` reader - Start operation is allowed"]
pub struct ALLOWSTART_R(crate::FieldReader<bool, bool>);
impl ALLOWSTART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALLOWSTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALLOWSTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALLOWSETKEY` reader - Set Key operations are allowed"]
pub struct ALLOWSETKEY_R(crate::FieldReader<bool, bool>);
impl ALLOWSETKEY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALLOWSETKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALLOWSETKEY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALLOWGETKEY` reader - Get Key operation is allowed"]
pub struct ALLOWGETKEY_R(crate::FieldReader<bool, bool>);
impl ALLOWGETKEY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALLOWGETKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALLOWGETKEY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Enroll operation is allowed"]
    #[inline(always)]
    pub fn allowenroll(&self) -> ALLOWENROLL_R {
        ALLOWENROLL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start operation is allowed"]
    #[inline(always)]
    pub fn allowstart(&self) -> ALLOWSTART_R {
        ALLOWSTART_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set Key operations are allowed"]
    #[inline(always)]
    pub fn allowsetkey(&self) -> ALLOWSETKEY_R {
        ALLOWSETKEY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Get Key operation is allowed"]
    #[inline(always)]
    pub fn allowgetkey(&self) -> ALLOWGETKEY_R {
        ALLOWGETKEY_R::new(((self.bits >> 3) & 0x01) != 0)
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
#[doc = "PUF Allow register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [allow](index.html) module"]
pub struct ALLOW_SPEC;
impl crate::RegisterSpec for ALLOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [allow::R](R) reader structure"]
impl crate::Readable for ALLOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [allow::W](W) writer structure"]
impl crate::Writable for ALLOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALLOW to value 0"]
impl crate::Resettable for ALLOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
