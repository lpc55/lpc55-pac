#[doc = "Reader of register SEC_CTRL_APB_BRIDGE1_MEM_CTRL0"]
pub type R = crate::R<u32, super::SEC_CTRL_APB_BRIDGE1_MEM_CTRL0>;
#[doc = "Writer for register SEC_CTRL_APB_BRIDGE1_MEM_CTRL0"]
pub type W = crate::W<u32, super::SEC_CTRL_APB_BRIDGE1_MEM_CTRL0>;
#[doc = "Register SEC_CTRL_APB_BRIDGE1_MEM_CTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC_CTRL_APB_BRIDGE1_MEM_CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Power Management Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PMC_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<PMC_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: PMC_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PMC_RULE`"]
pub type PMC_RULE_R = crate::R<u8, PMC_RULE_A>;
impl PMC_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMC_RULE_A {
        match self.bits {
            0 => PMC_RULE_A::ENUM_NS_NP,
            1 => PMC_RULE_A::ENUM_NS_P,
            2 => PMC_RULE_A::ENUM_S_NP,
            3 => PMC_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == PMC_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == PMC_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == PMC_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == PMC_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `PMC_RULE`"]
pub struct PMC_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> PMC_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMC_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(PMC_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(PMC_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(PMC_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(PMC_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "System Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTRL_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<SYSCTRL_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTRL_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCTRL_RULE`"]
pub type SYSCTRL_RULE_R = crate::R<u8, SYSCTRL_RULE_A>;
impl SYSCTRL_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCTRL_RULE_A {
        match self.bits {
            0 => SYSCTRL_RULE_A::ENUM_NS_NP,
            1 => SYSCTRL_RULE_A::ENUM_NS_P,
            2 => SYSCTRL_RULE_A::ENUM_S_NP,
            3 => SYSCTRL_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SYSCTRL_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SYSCTRL_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SYSCTRL_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SYSCTRL_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `SYSCTRL_RULE`"]
pub struct SYSCTRL_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTRL_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTRL_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SYSCTRL_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SYSCTRL_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SYSCTRL_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SYSCTRL_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Power Management Controller"]
    #[inline(always)]
    pub fn pmc_rule(&self) -> PMC_RULE_R {
        PMC_RULE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - System Controller"]
    #[inline(always)]
    pub fn sysctrl_rule(&self) -> SYSCTRL_RULE_R {
        SYSCTRL_RULE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power Management Controller"]
    #[inline(always)]
    pub fn pmc_rule(&mut self) -> PMC_RULE_W {
        PMC_RULE_W { w: self }
    }
    #[doc = "Bits 12:13 - System Controller"]
    #[inline(always)]
    pub fn sysctrl_rule(&mut self) -> SYSCTRL_RULE_W {
        SYSCTRL_RULE_W { w: self }
    }
}
