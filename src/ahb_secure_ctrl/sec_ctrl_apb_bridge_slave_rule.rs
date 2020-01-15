#[doc = "Reader of register SEC_CTRL_APB_BRIDGE_SLAVE_RULE"]
pub type R = crate::R<u32, super::SEC_CTRL_APB_BRIDGE_SLAVE_RULE>;
#[doc = "Writer for register SEC_CTRL_APB_BRIDGE_SLAVE_RULE"]
pub type W = crate::W<u32, super::SEC_CTRL_APB_BRIDGE_SLAVE_RULE>;
#[doc = "Register SEC_CTRL_APB_BRIDGE_SLAVE_RULE `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC_CTRL_APB_BRIDGE_SLAVE_RULE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Security access rules for the whole APB Bridge 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum APBBRIDGE0_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<APBBRIDGE0_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: APBBRIDGE0_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `APBBRIDGE0_RULE`"]
pub type APBBRIDGE0_RULE_R = crate::R<u8, APBBRIDGE0_RULE_A>;
impl APBBRIDGE0_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APBBRIDGE0_RULE_A {
        match self.bits {
            0 => APBBRIDGE0_RULE_A::ENUM_NS_NP,
            1 => APBBRIDGE0_RULE_A::ENUM_NS_P,
            2 => APBBRIDGE0_RULE_A::ENUM_S_NP,
            3 => APBBRIDGE0_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == APBBRIDGE0_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == APBBRIDGE0_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == APBBRIDGE0_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == APBBRIDGE0_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `APBBRIDGE0_RULE`"]
pub struct APBBRIDGE0_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> APBBRIDGE0_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APBBRIDGE0_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(APBBRIDGE0_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(APBBRIDGE0_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(APBBRIDGE0_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(APBBRIDGE0_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Security access rules for the whole APB Bridge 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum APBBRIDGE1_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<APBBRIDGE1_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: APBBRIDGE1_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `APBBRIDGE1_RULE`"]
pub type APBBRIDGE1_RULE_R = crate::R<u8, APBBRIDGE1_RULE_A>;
impl APBBRIDGE1_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APBBRIDGE1_RULE_A {
        match self.bits {
            0 => APBBRIDGE1_RULE_A::ENUM_NS_NP,
            1 => APBBRIDGE1_RULE_A::ENUM_NS_P,
            2 => APBBRIDGE1_RULE_A::ENUM_S_NP,
            3 => APBBRIDGE1_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == APBBRIDGE1_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == APBBRIDGE1_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == APBBRIDGE1_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == APBBRIDGE1_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `APBBRIDGE1_RULE`"]
pub struct APBBRIDGE1_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> APBBRIDGE1_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APBBRIDGE1_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(APBBRIDGE1_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(APBBRIDGE1_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(APBBRIDGE1_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(APBBRIDGE1_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Security access rules for the whole APB Bridge 0"]
    #[inline(always)]
    pub fn apbbridge0_rule(&self) -> APBBRIDGE0_RULE_R {
        APBBRIDGE0_RULE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Security access rules for the whole APB Bridge 1"]
    #[inline(always)]
    pub fn apbbridge1_rule(&self) -> APBBRIDGE1_RULE_R {
        APBBRIDGE1_RULE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Security access rules for the whole APB Bridge 0"]
    #[inline(always)]
    pub fn apbbridge0_rule(&mut self) -> APBBRIDGE0_RULE_W {
        APBBRIDGE0_RULE_W { w: self }
    }
    #[doc = "Bits 4:5 - Security access rules for the whole APB Bridge 1"]
    #[inline(always)]
    pub fn apbbridge1_rule(&mut self) -> APBBRIDGE1_RULE_W {
        APBBRIDGE1_RULE_W { w: self }
    }
}
