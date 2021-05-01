#[doc = "Register `SR_ENABLE1` reader"]
pub struct R(crate::R<SR_ENABLE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_ENABLE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SR_ENABLE1_SPEC>> for R {
    fn from(reader: crate::R<SR_ENABLE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR_ENABLE1` writer"]
pub struct W(crate::W<SR_ENABLE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_ENABLE1_SPEC>;
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
impl core::convert::From<crate::W<SR_ENABLE1_SPEC>> for W {
    fn from(writer: crate::W<SR_ENABLE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 1."]
pub struct EN_R(crate::FieldReader<u32, u32>);
impl EN_R {
    pub(crate) fn new(bits: u32) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 1."]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 1."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 1."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sub-Region Enable register for region 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr_enable1](index.html) module"]
pub struct SR_ENABLE1_SPEC;
impl crate::RegisterSpec for SR_ENABLE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr_enable1::R](R) reader structure"]
impl crate::Readable for SR_ENABLE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr_enable1::W](W) writer structure"]
impl crate::Writable for SR_ENABLE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR_ENABLE1 to value 0"]
impl crate::Resettable for SR_ENABLE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
