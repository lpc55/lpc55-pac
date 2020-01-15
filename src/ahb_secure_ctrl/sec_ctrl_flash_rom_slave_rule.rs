#[doc = "Reader of register SEC_CTRL_FLASH_ROM_SLAVE_RULE"]
pub type R = crate::R<u32, super::SEC_CTRL_FLASH_ROM_SLAVE_RULE>;
#[doc = "Writer for register SEC_CTRL_FLASH_ROM_SLAVE_RULE"]
pub type W = crate::W<u32, super::SEC_CTRL_FLASH_ROM_SLAVE_RULE>;
#[doc = "Register SEC_CTRL_FLASH_ROM_SLAVE_RULE `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC_CTRL_FLASH_ROM_SLAVE_RULE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Security access rules for the whole FLASH : 0x0000_0000 - 0x0009_FFFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASH_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<FLASH_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASH_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLASH_RULE`"]
pub type FLASH_RULE_R = crate::R<u8, FLASH_RULE_A>;
impl FLASH_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_RULE_A {
        match self.bits {
            0 => FLASH_RULE_A::ENUM_NS_NP,
            1 => FLASH_RULE_A::ENUM_NS_P,
            2 => FLASH_RULE_A::ENUM_S_NP,
            3 => FLASH_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == FLASH_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == FLASH_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == FLASH_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == FLASH_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `FLASH_RULE`"]
pub struct FLASH_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(FLASH_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(FLASH_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(FLASH_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(FLASH_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Security access rules for the whole ROM : 0x0300_0000 - 0x0301_FFFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ROM_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<ROM_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: ROM_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ROM_RULE`"]
pub type ROM_RULE_R = crate::R<u8, ROM_RULE_A>;
impl ROM_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROM_RULE_A {
        match self.bits {
            0 => ROM_RULE_A::ENUM_NS_NP,
            1 => ROM_RULE_A::ENUM_NS_P,
            2 => ROM_RULE_A::ENUM_S_NP,
            3 => ROM_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == ROM_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == ROM_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == ROM_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == ROM_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `ROM_RULE`"]
pub struct ROM_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROM_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(ROM_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(ROM_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(ROM_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(ROM_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Security access rules for the whole FLASH : 0x0000_0000 - 0x0009_FFFF"]
    #[inline(always)]
    pub fn flash_rule(&self) -> FLASH_RULE_R {
        FLASH_RULE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Security access rules for the whole ROM : 0x0300_0000 - 0x0301_FFFF"]
    #[inline(always)]
    pub fn rom_rule(&self) -> ROM_RULE_R {
        ROM_RULE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Security access rules for the whole FLASH : 0x0000_0000 - 0x0009_FFFF"]
    #[inline(always)]
    pub fn flash_rule(&mut self) -> FLASH_RULE_W {
        FLASH_RULE_W { w: self }
    }
    #[doc = "Bits 4:5 - Security access rules for the whole ROM : 0x0300_0000 - 0x0301_FFFF"]
    #[inline(always)]
    pub fn rom_rule(&mut self) -> ROM_RULE_W {
        ROM_RULE_W { w: self }
    }
}
