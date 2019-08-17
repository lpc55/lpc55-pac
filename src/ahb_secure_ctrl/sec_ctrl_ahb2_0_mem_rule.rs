#[doc = "Reader of register SEC_CTRL_AHB2_0_MEM_RULE"]
pub type R = crate::R<u32, super::SEC_CTRL_AHB2_0_MEM_RULE>;
#[doc = "Writer for register SEC_CTRL_AHB2_0_MEM_RULE"]
pub type W = crate::W<u32, super::SEC_CTRL_AHB2_0_MEM_RULE>;
#[doc = "Register SEC_CTRL_AHB2_0_MEM_RULE `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC_CTRL_AHB2_0_MEM_RULE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `AHB_SEC_CTRL_SECT_0_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_SEC_CTRL_SECT_0_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<AHB_SEC_CTRL_SECT_0_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: AHB_SEC_CTRL_SECT_0_RULE_A) -> Self {
        match variant {
            AHB_SEC_CTRL_SECT_0_RULE_A::ENUM_NS_NP => 0,
            AHB_SEC_CTRL_SECT_0_RULE_A::ENUM_NS_P => 1,
            AHB_SEC_CTRL_SECT_0_RULE_A::ENUM_S_NP => 2,
            AHB_SEC_CTRL_SECT_0_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `AHB_SEC_CTRL_SECT_0_RULE`"]
pub type AHB_SEC_CTRL_SECT_0_RULE_R = crate::R<u8, AHB_SEC_CTRL_SECT_0_RULE_A>;
impl AHB_SEC_CTRL_SECT_0_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_SEC_CTRL_SECT_0_RULE_A {
        match self.bits {
            0 => AHB_SEC_CTRL_SECT_0_RULE_A::ENUM_NS_NP,
            1 => AHB_SEC_CTRL_SECT_0_RULE_A::ENUM_NS_P,
            2 => AHB_SEC_CTRL_SECT_0_RULE_A::ENUM_S_NP,
            3 => AHB_SEC_CTRL_SECT_0_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == AHB_SEC_CTRL_SECT_0_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == AHB_SEC_CTRL_SECT_0_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == AHB_SEC_CTRL_SECT_0_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == AHB_SEC_CTRL_SECT_0_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `AHB_SEC_CTRL_SECT_0_RULE`"]
pub struct AHB_SEC_CTRL_SECT_0_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_SEC_CTRL_SECT_0_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB_SEC_CTRL_SECT_0_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(AHB_SEC_CTRL_SECT_0_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(AHB_SEC_CTRL_SECT_0_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(AHB_SEC_CTRL_SECT_0_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(AHB_SEC_CTRL_SECT_0_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `AHB_SEC_CTRL_SECT_1_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_SEC_CTRL_SECT_1_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<AHB_SEC_CTRL_SECT_1_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: AHB_SEC_CTRL_SECT_1_RULE_A) -> Self {
        match variant {
            AHB_SEC_CTRL_SECT_1_RULE_A::ENUM_NS_NP => 0,
            AHB_SEC_CTRL_SECT_1_RULE_A::ENUM_NS_P => 1,
            AHB_SEC_CTRL_SECT_1_RULE_A::ENUM_S_NP => 2,
            AHB_SEC_CTRL_SECT_1_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `AHB_SEC_CTRL_SECT_1_RULE`"]
pub type AHB_SEC_CTRL_SECT_1_RULE_R = crate::R<u8, AHB_SEC_CTRL_SECT_1_RULE_A>;
impl AHB_SEC_CTRL_SECT_1_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_SEC_CTRL_SECT_1_RULE_A {
        match self.bits {
            0 => AHB_SEC_CTRL_SECT_1_RULE_A::ENUM_NS_NP,
            1 => AHB_SEC_CTRL_SECT_1_RULE_A::ENUM_NS_P,
            2 => AHB_SEC_CTRL_SECT_1_RULE_A::ENUM_S_NP,
            3 => AHB_SEC_CTRL_SECT_1_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == AHB_SEC_CTRL_SECT_1_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == AHB_SEC_CTRL_SECT_1_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == AHB_SEC_CTRL_SECT_1_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == AHB_SEC_CTRL_SECT_1_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `AHB_SEC_CTRL_SECT_1_RULE`"]
pub struct AHB_SEC_CTRL_SECT_1_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_SEC_CTRL_SECT_1_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB_SEC_CTRL_SECT_1_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(AHB_SEC_CTRL_SECT_1_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(AHB_SEC_CTRL_SECT_1_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(AHB_SEC_CTRL_SECT_1_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(AHB_SEC_CTRL_SECT_1_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `AHB_SEC_CTRL_SECT_2_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_SEC_CTRL_SECT_2_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<AHB_SEC_CTRL_SECT_2_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: AHB_SEC_CTRL_SECT_2_RULE_A) -> Self {
        match variant {
            AHB_SEC_CTRL_SECT_2_RULE_A::ENUM_NS_NP => 0,
            AHB_SEC_CTRL_SECT_2_RULE_A::ENUM_NS_P => 1,
            AHB_SEC_CTRL_SECT_2_RULE_A::ENUM_S_NP => 2,
            AHB_SEC_CTRL_SECT_2_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `AHB_SEC_CTRL_SECT_2_RULE`"]
pub type AHB_SEC_CTRL_SECT_2_RULE_R = crate::R<u8, AHB_SEC_CTRL_SECT_2_RULE_A>;
impl AHB_SEC_CTRL_SECT_2_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_SEC_CTRL_SECT_2_RULE_A {
        match self.bits {
            0 => AHB_SEC_CTRL_SECT_2_RULE_A::ENUM_NS_NP,
            1 => AHB_SEC_CTRL_SECT_2_RULE_A::ENUM_NS_P,
            2 => AHB_SEC_CTRL_SECT_2_RULE_A::ENUM_S_NP,
            3 => AHB_SEC_CTRL_SECT_2_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == AHB_SEC_CTRL_SECT_2_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == AHB_SEC_CTRL_SECT_2_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == AHB_SEC_CTRL_SECT_2_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == AHB_SEC_CTRL_SECT_2_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `AHB_SEC_CTRL_SECT_2_RULE`"]
pub struct AHB_SEC_CTRL_SECT_2_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_SEC_CTRL_SECT_2_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB_SEC_CTRL_SECT_2_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(AHB_SEC_CTRL_SECT_2_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(AHB_SEC_CTRL_SECT_2_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(AHB_SEC_CTRL_SECT_2_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(AHB_SEC_CTRL_SECT_2_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `AHB_SEC_CTRL_SECT_3_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_SEC_CTRL_SECT_3_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<AHB_SEC_CTRL_SECT_3_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: AHB_SEC_CTRL_SECT_3_RULE_A) -> Self {
        match variant {
            AHB_SEC_CTRL_SECT_3_RULE_A::ENUM_NS_NP => 0,
            AHB_SEC_CTRL_SECT_3_RULE_A::ENUM_NS_P => 1,
            AHB_SEC_CTRL_SECT_3_RULE_A::ENUM_S_NP => 2,
            AHB_SEC_CTRL_SECT_3_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `AHB_SEC_CTRL_SECT_3_RULE`"]
pub type AHB_SEC_CTRL_SECT_3_RULE_R = crate::R<u8, AHB_SEC_CTRL_SECT_3_RULE_A>;
impl AHB_SEC_CTRL_SECT_3_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_SEC_CTRL_SECT_3_RULE_A {
        match self.bits {
            0 => AHB_SEC_CTRL_SECT_3_RULE_A::ENUM_NS_NP,
            1 => AHB_SEC_CTRL_SECT_3_RULE_A::ENUM_NS_P,
            2 => AHB_SEC_CTRL_SECT_3_RULE_A::ENUM_S_NP,
            3 => AHB_SEC_CTRL_SECT_3_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == AHB_SEC_CTRL_SECT_3_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == AHB_SEC_CTRL_SECT_3_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == AHB_SEC_CTRL_SECT_3_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == AHB_SEC_CTRL_SECT_3_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `AHB_SEC_CTRL_SECT_3_RULE`"]
pub struct AHB_SEC_CTRL_SECT_3_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_SEC_CTRL_SECT_3_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB_SEC_CTRL_SECT_3_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(AHB_SEC_CTRL_SECT_3_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(AHB_SEC_CTRL_SECT_3_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(AHB_SEC_CTRL_SECT_3_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(AHB_SEC_CTRL_SECT_3_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Address space: 0x400A_0000 - 0x400A_CFFF"]
    #[inline(always)]
    pub fn ahb_sec_ctrl_sect_0_rule(&self) -> AHB_SEC_CTRL_SECT_0_RULE_R {
        AHB_SEC_CTRL_SECT_0_RULE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Address space: 0x400A_D000 - 0x400A_DFFF"]
    #[inline(always)]
    pub fn ahb_sec_ctrl_sect_1_rule(&self) -> AHB_SEC_CTRL_SECT_1_RULE_R {
        AHB_SEC_CTRL_SECT_1_RULE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Address space: 0x400A_E000 - 0x400A_EFFF"]
    #[inline(always)]
    pub fn ahb_sec_ctrl_sect_2_rule(&self) -> AHB_SEC_CTRL_SECT_2_RULE_R {
        AHB_SEC_CTRL_SECT_2_RULE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Address space: 0x400A_F000 - 0x400A_FFFF"]
    #[inline(always)]
    pub fn ahb_sec_ctrl_sect_3_rule(&self) -> AHB_SEC_CTRL_SECT_3_RULE_R {
        AHB_SEC_CTRL_SECT_3_RULE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Address space: 0x400A_0000 - 0x400A_CFFF"]
    #[inline(always)]
    pub fn ahb_sec_ctrl_sect_0_rule(&mut self) -> AHB_SEC_CTRL_SECT_0_RULE_W {
        AHB_SEC_CTRL_SECT_0_RULE_W { w: self }
    }
    #[doc = "Bits 4:5 - Address space: 0x400A_D000 - 0x400A_DFFF"]
    #[inline(always)]
    pub fn ahb_sec_ctrl_sect_1_rule(&mut self) -> AHB_SEC_CTRL_SECT_1_RULE_W {
        AHB_SEC_CTRL_SECT_1_RULE_W { w: self }
    }
    #[doc = "Bits 8:9 - Address space: 0x400A_E000 - 0x400A_EFFF"]
    #[inline(always)]
    pub fn ahb_sec_ctrl_sect_2_rule(&mut self) -> AHB_SEC_CTRL_SECT_2_RULE_W {
        AHB_SEC_CTRL_SECT_2_RULE_W { w: self }
    }
    #[doc = "Bits 12:13 - Address space: 0x400A_F000 - 0x400A_FFFF"]
    #[inline(always)]
    pub fn ahb_sec_ctrl_sect_3_rule(&mut self) -> AHB_SEC_CTRL_SECT_3_RULE_W {
        AHB_SEC_CTRL_SECT_3_RULE_W { w: self }
    }
}
