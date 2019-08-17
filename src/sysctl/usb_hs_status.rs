#[doc = "Reader of register USB_HS_STATUS"]
pub type R = crate::R<u32, super::USB_HS_STATUS>;
#[doc = "Writer for register USB_HS_STATUS"]
pub type W = crate::W<u32, super::USB_HS_STATUS>;
#[doc = "Register USB_HS_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_HS_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `USBHS_3V_NOK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBHS_3V_NOK_A {
    #[doc = "3v3 supply is good."]
    SUPPLY_3V_OK,
    #[doc = "3v3 supply is too low."]
    SUPPLY_3V_LOW,
}
impl From<USBHS_3V_NOK_A> for bool {
    #[inline(always)]
    fn from(variant: USBHS_3V_NOK_A) -> Self {
        match variant {
            USBHS_3V_NOK_A::SUPPLY_3V_OK => false,
            USBHS_3V_NOK_A::SUPPLY_3V_LOW => true,
        }
    }
}
#[doc = "Reader of field `USBHS_3V_NOK`"]
pub type USBHS_3V_NOK_R = crate::R<bool, USBHS_3V_NOK_A>;
impl USBHS_3V_NOK_R {
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
        *self == USBHS_3V_NOK_A::SUPPLY_3V_OK
    }
    #[doc = "Checks if the value of the field is `SUPPLY_3V_LOW`"]
    #[inline(always)]
    pub fn is_supply_3v_low(&self) -> bool {
        *self == USBHS_3V_NOK_A::SUPPLY_3V_LOW
    }
}
impl R {
    #[doc = "Bit 0 - USB_HS: Low voltage detection on 3.3V supply."]
    #[inline(always)]
    pub fn usbhs_3v_nok(&self) -> USBHS_3V_NOK_R {
        USBHS_3V_NOK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {}
