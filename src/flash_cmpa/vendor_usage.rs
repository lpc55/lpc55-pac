#[doc = "Register `VENDOR_USAGE` reader"]
pub struct R(crate::R<VENDOR_USAGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VENDOR_USAGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<VENDOR_USAGE_SPEC>> for R {
    fn from(reader: crate::R<VENDOR_USAGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VENDOR_USAGE` writer"]
pub struct W(crate::W<VENDOR_USAGE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VENDOR_USAGE_SPEC>;
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
impl core::convert::From<crate::W<VENDOR_USAGE_SPEC>> for W {
    fn from(writer: crate::W<VENDOR_USAGE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VENDOR_USAGE` reader - Upper 16 bits of vendor usage field defined in DAP. Lower 16-bits come from customer field area."]
pub struct VENDOR_USAGE_R(crate::FieldReader<u16, u16>);
impl VENDOR_USAGE_R {
    pub(crate) fn new(bits: u16) -> Self {
        VENDOR_USAGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VENDOR_USAGE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VENDOR_USAGE` writer - Upper 16 bits of vendor usage field defined in DAP. Lower 16-bits come from customer field area."]
pub struct VENDOR_USAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> VENDOR_USAGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Upper 16 bits of vendor usage field defined in DAP. Lower 16-bits come from customer field area."]
    #[inline(always)]
    pub fn vendor_usage(&self) -> VENDOR_USAGE_R {
        VENDOR_USAGE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Upper 16 bits of vendor usage field defined in DAP. Lower 16-bits come from customer field area."]
    #[inline(always)]
    pub fn vendor_usage(&mut self) -> VENDOR_USAGE_W {
        VENDOR_USAGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vendor_usage](index.html) module"]
pub struct VENDOR_USAGE_SPEC;
impl crate::RegisterSpec for VENDOR_USAGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vendor_usage::R](R) reader structure"]
impl crate::Readable for VENDOR_USAGE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vendor_usage::W](W) writer structure"]
impl crate::Writable for VENDOR_USAGE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VENDOR_USAGE to value 0"]
impl crate::Resettable for VENDOR_USAGE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
