#[doc = "Register `FLEXFRG7CTRL` reader"]
pub struct R(crate::R<FLEXFRG7CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLEXFRG7CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLEXFRG7CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLEXFRG7CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLEXFRG7CTRL` writer"]
pub struct W(crate::W<FLEXFRG7CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLEXFRG7CTRL_SPEC>;
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
impl From<crate::W<FLEXFRG7CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLEXFRG7CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - Denominator of the fractional rate divider."]
pub struct DIV_R(crate::FieldReader<u8, u8>);
impl DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV` writer - Denominator of the fractional rate divider."]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `MULT` reader - Numerator of the fractional rate divider."]
pub struct MULT_R(crate::FieldReader<u8, u8>);
impl MULT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULT` writer - Numerator of the fractional rate divider."]
pub struct MULT_W<'a> {
    w: &'a mut W,
}
impl<'a> MULT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Denominator of the fractional rate divider."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Numerator of the fractional rate divider."]
    #[inline(always)]
    pub fn mult(&self) -> MULT_R {
        MULT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Denominator of the fractional rate divider."]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Bits 8:15 - Numerator of the fractional rate divider."]
    #[inline(always)]
    pub fn mult(&mut self) -> MULT_W {
        MULT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fractional rate divider for flexcomm 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flexfrg7ctrl](index.html) module"]
pub struct FLEXFRG7CTRL_SPEC;
impl crate::RegisterSpec for FLEXFRG7CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flexfrg7ctrl::R](R) reader structure"]
impl crate::Readable for FLEXFRG7CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flexfrg7ctrl::W](W) writer structure"]
impl crate::Writable for FLEXFRG7CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLEXFRG7CTRL to value 0xff"]
impl crate::Resettable for FLEXFRG7CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
