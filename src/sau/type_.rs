#[doc = "Register `TYPE` reader"]
pub struct R(crate::R<TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TYPE_SPEC>> for R {
    fn from(reader: crate::R<TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TYPE` writer"]
pub struct W(crate::W<TYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TYPE_SPEC>;
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
impl core::convert::From<crate::W<TYPE_SPEC>> for W {
    fn from(writer: crate::W<TYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SREGION` reader - SAU regions. The number of implemented SAU regions."]
pub struct SREGION_R(crate::FieldReader<u8, u8>);
impl SREGION_R {
    pub(crate) fn new(bits: u8) -> Self {
        SREGION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SREGION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SREGION` writer - SAU regions. The number of implemented SAU regions."]
pub struct SREGION_W<'a> {
    w: &'a mut W,
}
impl<'a> SREGION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SAU regions. The number of implemented SAU regions."]
    #[inline(always)]
    pub fn sregion(&self) -> SREGION_R {
        SREGION_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SAU regions. The number of implemented SAU regions."]
    #[inline(always)]
    pub fn sregion(&mut self) -> SREGION_W {
        SREGION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security Attribution Unit Type Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [type_](index.html) module"]
pub struct TYPE_SPEC;
impl crate::RegisterSpec for TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [type_::R](R) reader structure"]
impl crate::Readable for TYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [type_::W](W) writer structure"]
impl crate::Writable for TYPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TYPE to value 0"]
impl crate::Resettable for TYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
