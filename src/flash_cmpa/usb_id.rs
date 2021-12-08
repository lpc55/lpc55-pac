#[doc = "Register `USB_ID` reader"]
pub struct R(crate::R<USB_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_ID` writer"]
pub struct W(crate::W<USB_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_ID_SPEC>;
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
impl From<crate::W<USB_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_VENDOR_ID` reader - ."]
pub struct USB_VENDOR_ID_R(crate::FieldReader<u16, u16>);
impl USB_VENDOR_ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        USB_VENDOR_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_VENDOR_ID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_VENDOR_ID` writer - ."]
pub struct USB_VENDOR_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_VENDOR_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `USB_PRODUCT_ID` reader - ."]
pub struct USB_PRODUCT_ID_R(crate::FieldReader<u16, u16>);
impl USB_PRODUCT_ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        USB_PRODUCT_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_PRODUCT_ID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_PRODUCT_ID` writer - ."]
pub struct USB_PRODUCT_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_PRODUCT_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - ."]
    #[inline(always)]
    pub fn usb_vendor_id(&self) -> USB_VENDOR_ID_R {
        USB_VENDOR_ID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ."]
    #[inline(always)]
    pub fn usb_product_id(&self) -> USB_PRODUCT_ID_R {
        USB_PRODUCT_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ."]
    #[inline(always)]
    pub fn usb_vendor_id(&mut self) -> USB_VENDOR_ID_W {
        USB_VENDOR_ID_W { w: self }
    }
    #[doc = "Bits 16:31 - ."]
    #[inline(always)]
    pub fn usb_product_id(&mut self) -> USB_PRODUCT_ID_W {
        USB_PRODUCT_ID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_id](index.html) module"]
pub struct USB_ID_SPEC;
impl crate::RegisterSpec for USB_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_id::R](R) reader structure"]
impl crate::Readable for USB_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_id::W](W) writer structure"]
impl crate::Writable for USB_ID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_ID to value 0"]
impl crate::Resettable for USB_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
