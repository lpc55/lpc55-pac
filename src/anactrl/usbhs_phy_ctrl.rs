#[doc = "Reader of register USBHS_PHY_CTRL"]
pub type R = crate::R<u32, super::USBHS_PHY_CTRL>;
#[doc = "Writer for register USBHS_PHY_CTRL"]
pub type W = crate::W<u32, super::USBHS_PHY_CTRL>;
#[doc = "Register USBHS_PHY_CTRL `reset()`'s with value 0x08"]
impl crate::ResetValue for super::USBHS_PHY_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Reader of field `usb_vbusvalid_ext`"]
pub type USB_VBUSVALID_EXT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `usb_vbusvalid_ext`"]
pub struct USB_VBUSVALID_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_VBUSVALID_EXT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `usb_id_ext`"]
pub type USB_ID_EXT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `usb_id_ext`"]
pub struct USB_ID_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_ID_EXT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `iso_atx`"]
pub type ISO_ATX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `iso_atx`"]
pub struct ISO_ATX_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_ATX_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Override value for Vbus if using external detectors."]
    #[inline(always)]
    pub fn usb_vbusvalid_ext(&self) -> USB_VBUSVALID_EXT_R {
        USB_VBUSVALID_EXT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Override value for ID if using external detectors."]
    #[inline(always)]
    pub fn usb_id_ext(&self) -> USB_ID_EXT_R {
        USB_ID_EXT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ."]
    #[inline(always)]
    pub fn iso_atx(&self) -> ISO_ATX_R {
        ISO_ATX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Override value for Vbus if using external detectors."]
    #[inline(always)]
    pub fn usb_vbusvalid_ext(&mut self) -> USB_VBUSVALID_EXT_W {
        USB_VBUSVALID_EXT_W { w: self }
    }
    #[doc = "Bit 1 - Override value for ID if using external detectors."]
    #[inline(always)]
    pub fn usb_id_ext(&mut self) -> USB_ID_EXT_W {
        USB_ID_EXT_W { w: self }
    }
    #[doc = "Bit 3 - ."]
    #[inline(always)]
    pub fn iso_atx(&mut self) -> ISO_ATX_W {
        ISO_ATX_W { w: self }
    }
}
