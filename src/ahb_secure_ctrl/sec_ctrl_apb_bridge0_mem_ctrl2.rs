#[doc = "Reader of register SEC_CTRL_APB_BRIDGE0_MEM_CTRL2"]
pub type R = crate::R<u32, super::SEC_CTRL_APB_BRIDGE0_MEM_CTRL2>;
#[doc = "Writer for register SEC_CTRL_APB_BRIDGE0_MEM_CTRL2"]
pub type W = crate::W<u32, super::SEC_CTRL_APB_BRIDGE0_MEM_CTRL2>;
#[doc = "Register SEC_CTRL_APB_BRIDGE0_MEM_CTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC_CTRL_APB_BRIDGE0_MEM_CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Analog Modules controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ANACTRL_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<ANACTRL_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: ANACTRL_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ANACTRL_RULE`"]
pub type ANACTRL_RULE_R = crate::R<u8, ANACTRL_RULE_A>;
impl ANACTRL_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANACTRL_RULE_A {
        match self.bits {
            0 => ANACTRL_RULE_A::ENUM_NS_NP,
            1 => ANACTRL_RULE_A::ENUM_NS_P,
            2 => ANACTRL_RULE_A::ENUM_S_NP,
            3 => ANACTRL_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == ANACTRL_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == ANACTRL_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == ANACTRL_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == ANACTRL_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `ANACTRL_RULE`"]
pub struct ANACTRL_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> ANACTRL_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANACTRL_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(ANACTRL_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(ANACTRL_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(ANACTRL_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(ANACTRL_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:13 - Analog Modules controller"]
    #[inline(always)]
    pub fn anactrl_rule(&self) -> ANACTRL_RULE_R {
        ANACTRL_RULE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 12:13 - Analog Modules controller"]
    #[inline(always)]
    pub fn anactrl_rule(&mut self) -> ANACTRL_RULE_W {
        ANACTRL_RULE_W { w: self }
    }
}
