#[doc = "Register `ISOPTD` reader"]
pub struct R(crate::R<ISOPTD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISOPTD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISOPTD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISOPTD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISOPTD` writer"]
pub struct W(crate::W<ISOPTD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISOPTD_SPEC>;
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
impl From<crate::W<ISOPTD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISOPTD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISO_FIRST` reader - This indicates the first PTD that is used by the hardware when it is processing the ISO list."]
pub struct ISO_FIRST_R(crate::FieldReader<u8, u8>);
impl ISO_FIRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ISO_FIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISO_FIRST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISO_FIRST` writer - This indicates the first PTD that is used by the hardware when it is processing the ISO list."]
pub struct ISO_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_FIRST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `ISO_BASE` reader - Base address to be used by the hardware to find the start of the ISO list."]
pub struct ISO_BASE_R(crate::FieldReader<u32, u32>);
impl ISO_BASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ISO_BASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISO_BASE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISO_BASE` writer - Base address to be used by the hardware to find the start of the ISO list."]
pub struct ISO_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | ((value as u32 & 0x003f_ffff) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:9 - This indicates the first PTD that is used by the hardware when it is processing the ISO list."]
    #[inline(always)]
    pub fn iso_first(&self) -> ISO_FIRST_R {
        ISO_FIRST_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:31 - Base address to be used by the hardware to find the start of the ISO list."]
    #[inline(always)]
    pub fn iso_base(&self) -> ISO_BASE_R {
        ISO_BASE_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 5:9 - This indicates the first PTD that is used by the hardware when it is processing the ISO list."]
    #[inline(always)]
    pub fn iso_first(&mut self) -> ISO_FIRST_W {
        ISO_FIRST_W { w: self }
    }
    #[doc = "Bits 10:31 - Base address to be used by the hardware to find the start of the ISO list."]
    #[inline(always)]
    pub fn iso_base(&mut self) -> ISO_BASE_W {
        ISO_BASE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory base address where ISO PTD0 is stored\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isoptd](index.html) module"]
pub struct ISOPTD_SPEC;
impl crate::RegisterSpec for ISOPTD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isoptd::R](R) reader structure"]
impl crate::Readable for ISOPTD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isoptd::W](W) writer structure"]
impl crate::Writable for ISOPTD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISOPTD to value 0"]
impl crate::Resettable for ISOPTD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
