#[doc = "Register `CPBOOT` reader"]
pub struct R(crate::R<CPBOOT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPBOOT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPBOOT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPBOOT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPBOOT` writer"]
pub struct W(crate::W<CPBOOT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPBOOT_SPEC>;
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
impl From<crate::W<CPBOOT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPBOOT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPBOOT` reader - Coprocessor Boot Address for CPU1."]
pub struct CPBOOT_R(crate::FieldReader<u32, u32>);
impl CPBOOT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CPBOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPBOOT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPBOOT` writer - Coprocessor Boot Address for CPU1."]
pub struct CPBOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> CPBOOT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Coprocessor Boot Address for CPU1."]
    #[inline(always)]
    pub fn cpboot(&self) -> CPBOOT_R {
        CPBOOT_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Coprocessor Boot Address for CPU1."]
    #[inline(always)]
    pub fn cpboot(&mut self) -> CPBOOT_W {
        CPBOOT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Coprocessor Boot Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpboot](index.html) module"]
pub struct CPBOOT_SPEC;
impl crate::RegisterSpec for CPBOOT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpboot::R](R) reader structure"]
impl crate::Readable for CPBOOT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpboot::W](W) writer structure"]
impl crate::Writable for CPBOOT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPBOOT to value 0"]
impl crate::Resettable for CPBOOT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
