#[doc = "Reader of register SEC_CTRL_APB_BRIDGE1_MEM_CTRL2"]
pub type R = crate::R<u32, super::SEC_CTRL_APB_BRIDGE1_MEM_CTRL2>;
#[doc = "Writer for register SEC_CTRL_APB_BRIDGE1_MEM_CTRL2"]
pub type W = crate::W<u32, super::SEC_CTRL_APB_BRIDGE1_MEM_CTRL2>;
#[doc = "Register SEC_CTRL_APB_BRIDGE1_MEM_CTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC_CTRL_APB_BRIDGE1_MEM_CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Flash Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASH_CTRL_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<FLASH_CTRL_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASH_CTRL_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLASH_CTRL_RULE`"]
pub type FLASH_CTRL_RULE_R = crate::R<u8, FLASH_CTRL_RULE_A>;
impl FLASH_CTRL_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_CTRL_RULE_A {
        match self.bits {
            0 => FLASH_CTRL_RULE_A::ENUM_NS_NP,
            1 => FLASH_CTRL_RULE_A::ENUM_NS_P,
            2 => FLASH_CTRL_RULE_A::ENUM_S_NP,
            3 => FLASH_CTRL_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == FLASH_CTRL_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == FLASH_CTRL_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == FLASH_CTRL_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == FLASH_CTRL_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `FLASH_CTRL_RULE`"]
pub struct FLASH_CTRL_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_CTRL_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_CTRL_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(FLASH_CTRL_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(FLASH_CTRL_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(FLASH_CTRL_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(FLASH_CTRL_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Prince\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRINCE_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<PRINCE_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRINCE_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRINCE_RULE`"]
pub type PRINCE_RULE_R = crate::R<u8, PRINCE_RULE_A>;
impl PRINCE_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRINCE_RULE_A {
        match self.bits {
            0 => PRINCE_RULE_A::ENUM_NS_NP,
            1 => PRINCE_RULE_A::ENUM_NS_P,
            2 => PRINCE_RULE_A::ENUM_S_NP,
            3 => PRINCE_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == PRINCE_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == PRINCE_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == PRINCE_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == PRINCE_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `PRINCE_RULE`"]
pub struct PRINCE_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRINCE_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRINCE_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(PRINCE_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(PRINCE_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(PRINCE_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(PRINCE_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17 - Flash Controller"]
    #[inline(always)]
    pub fn flash_ctrl_rule(&self) -> FLASH_CTRL_RULE_R {
        FLASH_CTRL_RULE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Prince"]
    #[inline(always)]
    pub fn prince_rule(&self) -> PRINCE_RULE_R {
        PRINCE_RULE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - Flash Controller"]
    #[inline(always)]
    pub fn flash_ctrl_rule(&mut self) -> FLASH_CTRL_RULE_W {
        FLASH_CTRL_RULE_W { w: self }
    }
    #[doc = "Bits 20:21 - Prince"]
    #[inline(always)]
    pub fn prince_rule(&mut self) -> PRINCE_RULE_W {
        PRINCE_RULE_W { w: self }
    }
}
