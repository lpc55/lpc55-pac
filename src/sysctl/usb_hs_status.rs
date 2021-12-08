#[doc = "Register `USB_HS_STATUS` reader"]
pub struct R(crate::R<USB_HS_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_HS_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_HS_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_HS_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_HS_STATUS` writer"]
pub struct W(crate::W<USB_HS_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_HS_STATUS_SPEC>;
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
impl From<crate::W<USB_HS_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_HS_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB_HS: Low voltage detection on 3.3V supply.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBHS_3V_NOK_A {
    #[doc = "0: 3v3 supply is good."]
    SUPPLY_3V_OK = 0,
    #[doc = "1: 3v3 supply is too low."]
    SUPPLY_3V_LOW = 1,
}
impl From<USBHS_3V_NOK_A> for bool {
    #[inline(always)]
    fn from(variant: USBHS_3V_NOK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_3V_NOK` reader - USB_HS: Low voltage detection on 3.3V supply."]
pub struct USBHS_3V_NOK_R(crate::FieldReader<bool, USBHS_3V_NOK_A>);
impl USBHS_3V_NOK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBHS_3V_NOK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBHS_3V_NOK_A {
        match self.bits {
            false => USBHS_3V_NOK_A::SUPPLY_3V_OK,
            true => USBHS_3V_NOK_A::SUPPLY_3V_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `SUPPLY_3V_OK`"]
    #[inline(always)]
    pub fn is_supply_3v_ok(&self) -> bool {
        **self == USBHS_3V_NOK_A::SUPPLY_3V_OK
    }
    #[doc = "Checks if the value of the field is `SUPPLY_3V_LOW`"]
    #[inline(always)]
    pub fn is_supply_3v_low(&self) -> bool {
        **self == USBHS_3V_NOK_A::SUPPLY_3V_LOW
    }
}
impl core::ops::Deref for USBHS_3V_NOK_R {
    type Target = crate::FieldReader<bool, USBHS_3V_NOK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - USB_HS: Low voltage detection on 3.3V supply."]
    #[inline(always)]
    pub fn usbhs_3v_nok(&self) -> USBHS_3V_NOK_R {
        USBHS_3V_NOK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register for USB HS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_hs_status](index.html) module"]
pub struct USB_HS_STATUS_SPEC;
impl crate::RegisterSpec for USB_HS_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_hs_status::R](R) reader structure"]
impl crate::Readable for USB_HS_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_hs_status::W](W) writer structure"]
impl crate::Writable for USB_HS_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_HS_STATUS to value 0"]
impl crate::Resettable for USB_HS_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
