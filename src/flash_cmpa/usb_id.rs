#[doc = "Reader of register USB_ID"]
pub type R = crate::R<u32, super::USB_ID>;
#[doc = "Writer for register USB_ID"]
pub type W = crate::W<u32, super::USB_ID>;
#[doc = "Register USB_ID `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_ID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USB_VENDOR_ID`"]
pub type USB_VENDOR_ID_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `USB_VENDOR_ID`"]
pub struct USB_VENDOR_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_VENDOR_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `USB_PRODUCT_ID`"]
pub type USB_PRODUCT_ID_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `USB_PRODUCT_ID`"]
pub struct USB_PRODUCT_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_PRODUCT_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
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
}
