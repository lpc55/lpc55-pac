#[doc = "Register `RETURN` reader"]
pub struct R(crate::R<RETURN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETURN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETURN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETURN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RETURN` writer"]
pub struct W(crate::W<RETURN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RETURN_SPEC>;
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
impl From<crate::W<RETURN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RETURN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RET` reader - The Return value from ROM."]
pub struct RET_R(crate::FieldReader<u32, u32>);
impl RET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RET_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RET` writer - The Return value from ROM."]
pub struct RET_W<'a> {
    w: &'a mut W,
}
impl<'a> RET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The Return value from ROM."]
    #[inline(always)]
    pub fn ret(&self) -> RET_R {
        RET_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The Return value from ROM."]
    #[inline(always)]
    pub fn ret(&mut self) -> RET_W {
        RET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Return value from ROM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [return_](index.html) module"]
pub struct RETURN_SPEC;
impl crate::RegisterSpec for RETURN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [return_::R](R) reader structure"]
impl crate::Readable for RETURN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [return_::W](W) writer structure"]
impl crate::Writable for RETURN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RETURN to value 0"]
impl crate::Resettable for RETURN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
