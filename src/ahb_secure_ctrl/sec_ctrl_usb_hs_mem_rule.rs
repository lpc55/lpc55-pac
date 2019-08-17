#[doc = "Reader of register SEC_CTRL_USB_HS_MEM_RULE"]
pub type R = crate::R<u32, super::SEC_CTRL_USB_HS_MEM_RULE>;
#[doc = "Writer for register SEC_CTRL_USB_HS_MEM_RULE"]
pub type W = crate::W<u32, super::SEC_CTRL_USB_HS_MEM_RULE>;
#[doc = "Register SEC_CTRL_USB_HS_MEM_RULE `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC_CTRL_USB_HS_MEM_RULE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `SRAM_SECT_0_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_SECT_0_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<SRAM_SECT_0_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAM_SECT_0_RULE_A) -> Self {
        match variant {
            SRAM_SECT_0_RULE_A::ENUM_NS_NP => 0,
            SRAM_SECT_0_RULE_A::ENUM_NS_P => 1,
            SRAM_SECT_0_RULE_A::ENUM_S_NP => 2,
            SRAM_SECT_0_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `SRAM_SECT_0_RULE`"]
pub type SRAM_SECT_0_RULE_R = crate::R<u8, SRAM_SECT_0_RULE_A>;
impl SRAM_SECT_0_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_SECT_0_RULE_A {
        match self.bits {
            0 => SRAM_SECT_0_RULE_A::ENUM_NS_NP,
            1 => SRAM_SECT_0_RULE_A::ENUM_NS_P,
            2 => SRAM_SECT_0_RULE_A::ENUM_S_NP,
            3 => SRAM_SECT_0_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SRAM_SECT_0_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SRAM_SECT_0_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SRAM_SECT_0_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SRAM_SECT_0_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `SRAM_SECT_0_RULE`"]
pub struct SRAM_SECT_0_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_SECT_0_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_SECT_0_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SRAM_SECT_0_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SRAM_SECT_0_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SRAM_SECT_0_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SRAM_SECT_0_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `SRAM_SECT_1_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_SECT_1_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<SRAM_SECT_1_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAM_SECT_1_RULE_A) -> Self {
        match variant {
            SRAM_SECT_1_RULE_A::ENUM_NS_NP => 0,
            SRAM_SECT_1_RULE_A::ENUM_NS_P => 1,
            SRAM_SECT_1_RULE_A::ENUM_S_NP => 2,
            SRAM_SECT_1_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `SRAM_SECT_1_RULE`"]
pub type SRAM_SECT_1_RULE_R = crate::R<u8, SRAM_SECT_1_RULE_A>;
impl SRAM_SECT_1_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_SECT_1_RULE_A {
        match self.bits {
            0 => SRAM_SECT_1_RULE_A::ENUM_NS_NP,
            1 => SRAM_SECT_1_RULE_A::ENUM_NS_P,
            2 => SRAM_SECT_1_RULE_A::ENUM_S_NP,
            3 => SRAM_SECT_1_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SRAM_SECT_1_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SRAM_SECT_1_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SRAM_SECT_1_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SRAM_SECT_1_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `SRAM_SECT_1_RULE`"]
pub struct SRAM_SECT_1_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_SECT_1_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_SECT_1_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SRAM_SECT_1_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SRAM_SECT_1_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SRAM_SECT_1_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SRAM_SECT_1_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `SRAM_SECT_2_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_SECT_2_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<SRAM_SECT_2_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAM_SECT_2_RULE_A) -> Self {
        match variant {
            SRAM_SECT_2_RULE_A::ENUM_NS_NP => 0,
            SRAM_SECT_2_RULE_A::ENUM_NS_P => 1,
            SRAM_SECT_2_RULE_A::ENUM_S_NP => 2,
            SRAM_SECT_2_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `SRAM_SECT_2_RULE`"]
pub type SRAM_SECT_2_RULE_R = crate::R<u8, SRAM_SECT_2_RULE_A>;
impl SRAM_SECT_2_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_SECT_2_RULE_A {
        match self.bits {
            0 => SRAM_SECT_2_RULE_A::ENUM_NS_NP,
            1 => SRAM_SECT_2_RULE_A::ENUM_NS_P,
            2 => SRAM_SECT_2_RULE_A::ENUM_S_NP,
            3 => SRAM_SECT_2_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SRAM_SECT_2_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SRAM_SECT_2_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SRAM_SECT_2_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SRAM_SECT_2_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `SRAM_SECT_2_RULE`"]
pub struct SRAM_SECT_2_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_SECT_2_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_SECT_2_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SRAM_SECT_2_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SRAM_SECT_2_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SRAM_SECT_2_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SRAM_SECT_2_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `SRAM_SECT_3_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_SECT_3_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<SRAM_SECT_3_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAM_SECT_3_RULE_A) -> Self {
        match variant {
            SRAM_SECT_3_RULE_A::ENUM_NS_NP => 0,
            SRAM_SECT_3_RULE_A::ENUM_NS_P => 1,
            SRAM_SECT_3_RULE_A::ENUM_S_NP => 2,
            SRAM_SECT_3_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `SRAM_SECT_3_RULE`"]
pub type SRAM_SECT_3_RULE_R = crate::R<u8, SRAM_SECT_3_RULE_A>;
impl SRAM_SECT_3_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_SECT_3_RULE_A {
        match self.bits {
            0 => SRAM_SECT_3_RULE_A::ENUM_NS_NP,
            1 => SRAM_SECT_3_RULE_A::ENUM_NS_P,
            2 => SRAM_SECT_3_RULE_A::ENUM_S_NP,
            3 => SRAM_SECT_3_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SRAM_SECT_3_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SRAM_SECT_3_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SRAM_SECT_3_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SRAM_SECT_3_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `SRAM_SECT_3_RULE`"]
pub struct SRAM_SECT_3_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_SECT_3_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_SECT_3_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SRAM_SECT_3_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SRAM_SECT_3_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SRAM_SECT_3_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SRAM_SECT_3_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Address space: 0x4010_0000 - 0x4010_0FFF"]
    #[inline(always)]
    pub fn sram_sect_0_rule(&self) -> SRAM_SECT_0_RULE_R {
        SRAM_SECT_0_RULE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Address space: 0x4010_1000 - 0x4010_1FFF"]
    #[inline(always)]
    pub fn sram_sect_1_rule(&self) -> SRAM_SECT_1_RULE_R {
        SRAM_SECT_1_RULE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Address space: 0x4010_2000 - 0x4010_2FFF"]
    #[inline(always)]
    pub fn sram_sect_2_rule(&self) -> SRAM_SECT_2_RULE_R {
        SRAM_SECT_2_RULE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Address space: 0x4010_3000 - 0x4010_3FFF"]
    #[inline(always)]
    pub fn sram_sect_3_rule(&self) -> SRAM_SECT_3_RULE_R {
        SRAM_SECT_3_RULE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Address space: 0x4010_0000 - 0x4010_0FFF"]
    #[inline(always)]
    pub fn sram_sect_0_rule(&mut self) -> SRAM_SECT_0_RULE_W {
        SRAM_SECT_0_RULE_W { w: self }
    }
    #[doc = "Bits 4:5 - Address space: 0x4010_1000 - 0x4010_1FFF"]
    #[inline(always)]
    pub fn sram_sect_1_rule(&mut self) -> SRAM_SECT_1_RULE_W {
        SRAM_SECT_1_RULE_W { w: self }
    }
    #[doc = "Bits 8:9 - Address space: 0x4010_2000 - 0x4010_2FFF"]
    #[inline(always)]
    pub fn sram_sect_2_rule(&mut self) -> SRAM_SECT_2_RULE_W {
        SRAM_SECT_2_RULE_W { w: self }
    }
    #[doc = "Bits 12:13 - Address space: 0x4010_3000 - 0x4010_3FFF"]
    #[inline(always)]
    pub fn sram_sect_3_rule(&mut self) -> SRAM_SECT_3_RULE_W {
        SRAM_SECT_3_RULE_W { w: self }
    }
}
