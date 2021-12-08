#[doc = "Register `ENABLE_FA_MODE` reader"]
pub struct R(crate::R<ENABLE_FA_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLE_FA_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLE_FA_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLE_FA_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENABLE_FA_MODE` writer"]
pub struct W(crate::W<ENABLE_FA_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLE_FA_MODE_SPEC>;
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
impl From<crate::W<ENABLE_FA_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLE_FA_MODE_SPEC>) -> Self {
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
#[doc = "Enable FA mode. SET_FA_MODE Command should write 0xC33CA55A to this word to indicate boot ROM to enter FA mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable_fa_mode](index.html) module"]
pub struct ENABLE_FA_MODE_SPEC;
impl crate::RegisterSpec for ENABLE_FA_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enable_fa_mode::R](R) reader structure"]
impl crate::Readable for ENABLE_FA_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enable_fa_mode::W](W) writer structure"]
impl crate::Writable for ENABLE_FA_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENABLE_FA_MODE to value 0"]
impl crate::Resettable for ENABLE_FA_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
