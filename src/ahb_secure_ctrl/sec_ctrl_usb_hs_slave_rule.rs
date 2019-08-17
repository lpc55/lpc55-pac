#[doc = "Reader of register SEC_CTRL_USB_HS_SLAVE_RULE"]
pub type R = crate::R<u32, super::SEC_CTRL_USB_HS_SLAVE_RULE>;
#[doc = "Writer for register SEC_CTRL_USB_HS_SLAVE_RULE"]
pub type W = crate::W<u32, super::SEC_CTRL_USB_HS_SLAVE_RULE>;
#[doc = "Register SEC_CTRL_USB_HS_SLAVE_RULE `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC_CTRL_USB_HS_SLAVE_RULE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `RAM_USB_HS_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_USB_HS_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<RAM_USB_HS_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM_USB_HS_RULE_A) -> Self {
        match variant {
            RAM_USB_HS_RULE_A::ENUM_NS_NP => 0,
            RAM_USB_HS_RULE_A::ENUM_NS_P => 1,
            RAM_USB_HS_RULE_A::ENUM_S_NP => 2,
            RAM_USB_HS_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `RAM_USB_HS_RULE`"]
pub type RAM_USB_HS_RULE_R = crate::R<u8, RAM_USB_HS_RULE_A>;
impl RAM_USB_HS_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_USB_HS_RULE_A {
        match self.bits {
            0 => RAM_USB_HS_RULE_A::ENUM_NS_NP,
            1 => RAM_USB_HS_RULE_A::ENUM_NS_P,
            2 => RAM_USB_HS_RULE_A::ENUM_S_NP,
            3 => RAM_USB_HS_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RAM_USB_HS_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RAM_USB_HS_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RAM_USB_HS_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RAM_USB_HS_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `RAM_USB_HS_RULE`"]
pub struct RAM_USB_HS_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_USB_HS_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_USB_HS_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RAM_USB_HS_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RAM_USB_HS_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RAM_USB_HS_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RAM_USB_HS_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Security access rules for the whole USB High Speed RAM : 0x4010_0000 - 0x4010_3FFF"]
    #[inline(always)]
    pub fn ram_usb_hs_rule(&self) -> RAM_USB_HS_RULE_R {
        RAM_USB_HS_RULE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Security access rules for the whole USB High Speed RAM : 0x4010_0000 - 0x4010_3FFF"]
    #[inline(always)]
    pub fn ram_usb_hs_rule(&mut self) -> RAM_USB_HS_RULE_W {
        RAM_USB_HS_RULE_W { w: self }
    }
}
