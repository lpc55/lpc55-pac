#[doc = "Reader of register SEC_CTRL_APB_BRIDGE0_MEM_CTRL1"]
pub type R = crate::R<u32, super::SEC_CTRL_APB_BRIDGE0_MEM_CTRL1>;
#[doc = "Writer for register SEC_CTRL_APB_BRIDGE0_MEM_CTRL1"]
pub type W = crate::W<u32, super::SEC_CTRL_APB_BRIDGE0_MEM_CTRL1>;
#[doc = "Register SEC_CTRL_APB_BRIDGE0_MEM_CTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC_CTRL_APB_BRIDGE0_MEM_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `CTIMER0_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER0_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<CTIMER0_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTIMER0_RULE_A) -> Self {
        match variant {
            CTIMER0_RULE_A::ENUM_NS_NP => 0,
            CTIMER0_RULE_A::ENUM_NS_P => 1,
            CTIMER0_RULE_A::ENUM_S_NP => 2,
            CTIMER0_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `CTIMER0_RULE`"]
pub type CTIMER0_RULE_R = crate::R<u8, CTIMER0_RULE_A>;
impl CTIMER0_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER0_RULE_A {
        match self.bits {
            0 => CTIMER0_RULE_A::ENUM_NS_NP,
            1 => CTIMER0_RULE_A::ENUM_NS_P,
            2 => CTIMER0_RULE_A::ENUM_S_NP,
            3 => CTIMER0_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == CTIMER0_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == CTIMER0_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == CTIMER0_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == CTIMER0_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `CTIMER0_RULE`"]
pub struct CTIMER0_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER0_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMER0_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(CTIMER0_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(CTIMER0_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(CTIMER0_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(CTIMER0_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `CTIMER1_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER1_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<CTIMER1_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTIMER1_RULE_A) -> Self {
        match variant {
            CTIMER1_RULE_A::ENUM_NS_NP => 0,
            CTIMER1_RULE_A::ENUM_NS_P => 1,
            CTIMER1_RULE_A::ENUM_S_NP => 2,
            CTIMER1_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `CTIMER1_RULE`"]
pub type CTIMER1_RULE_R = crate::R<u8, CTIMER1_RULE_A>;
impl CTIMER1_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER1_RULE_A {
        match self.bits {
            0 => CTIMER1_RULE_A::ENUM_NS_NP,
            1 => CTIMER1_RULE_A::ENUM_NS_P,
            2 => CTIMER1_RULE_A::ENUM_S_NP,
            3 => CTIMER1_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == CTIMER1_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == CTIMER1_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == CTIMER1_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == CTIMER1_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `CTIMER1_RULE`"]
pub struct CTIMER1_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER1_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMER1_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(CTIMER1_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(CTIMER1_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(CTIMER1_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(CTIMER1_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `WWDT_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDT_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<WWDT_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: WWDT_RULE_A) -> Self {
        match variant {
            WWDT_RULE_A::ENUM_NS_NP => 0,
            WWDT_RULE_A::ENUM_NS_P => 1,
            WWDT_RULE_A::ENUM_S_NP => 2,
            WWDT_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `WWDT_RULE`"]
pub type WWDT_RULE_R = crate::R<u8, WWDT_RULE_A>;
impl WWDT_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDT_RULE_A {
        match self.bits {
            0 => WWDT_RULE_A::ENUM_NS_NP,
            1 => WWDT_RULE_A::ENUM_NS_P,
            2 => WWDT_RULE_A::ENUM_S_NP,
            3 => WWDT_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == WWDT_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == WWDT_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == WWDT_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == WWDT_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `WWDT_RULE`"]
pub struct WWDT_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDT_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDT_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(WWDT_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(WWDT_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(WWDT_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(WWDT_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `MRT_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRT_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<MRT_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: MRT_RULE_A) -> Self {
        match variant {
            MRT_RULE_A::ENUM_NS_NP => 0,
            MRT_RULE_A::ENUM_NS_P => 1,
            MRT_RULE_A::ENUM_S_NP => 2,
            MRT_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `MRT_RULE`"]
pub type MRT_RULE_R = crate::R<u8, MRT_RULE_A>;
impl MRT_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRT_RULE_A {
        match self.bits {
            0 => MRT_RULE_A::ENUM_NS_NP,
            1 => MRT_RULE_A::ENUM_NS_P,
            2 => MRT_RULE_A::ENUM_S_NP,
            3 => MRT_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == MRT_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == MRT_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == MRT_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == MRT_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `MRT_RULE`"]
pub struct MRT_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> MRT_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MRT_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(MRT_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(MRT_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(MRT_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(MRT_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `UTICK_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UTICK_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<UTICK_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: UTICK_RULE_A) -> Self {
        match variant {
            UTICK_RULE_A::ENUM_NS_NP => 0,
            UTICK_RULE_A::ENUM_NS_P => 1,
            UTICK_RULE_A::ENUM_S_NP => 2,
            UTICK_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `UTICK_RULE`"]
pub type UTICK_RULE_R = crate::R<u8, UTICK_RULE_A>;
impl UTICK_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTICK_RULE_A {
        match self.bits {
            0 => UTICK_RULE_A::ENUM_NS_NP,
            1 => UTICK_RULE_A::ENUM_NS_P,
            2 => UTICK_RULE_A::ENUM_S_NP,
            3 => UTICK_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == UTICK_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == UTICK_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == UTICK_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == UTICK_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `UTICK_RULE`"]
pub struct UTICK_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> UTICK_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UTICK_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(UTICK_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(UTICK_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(UTICK_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(UTICK_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Standard counter/Timer 0"]
    #[inline(always)]
    pub fn ctimer0_rule(&self) -> CTIMER0_RULE_R {
        CTIMER0_RULE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Standard counter/Timer 1"]
    #[inline(always)]
    pub fn ctimer1_rule(&self) -> CTIMER1_RULE_R {
        CTIMER1_RULE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Windiwed wtachdog Timer"]
    #[inline(always)]
    pub fn wwdt_rule(&self) -> WWDT_RULE_R {
        WWDT_RULE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Multi-rate Timer"]
    #[inline(always)]
    pub fn mrt_rule(&self) -> MRT_RULE_R {
        MRT_RULE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Micro-Timer"]
    #[inline(always)]
    pub fn utick_rule(&self) -> UTICK_RULE_R {
        UTICK_RULE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Standard counter/Timer 0"]
    #[inline(always)]
    pub fn ctimer0_rule(&mut self) -> CTIMER0_RULE_W {
        CTIMER0_RULE_W { w: self }
    }
    #[doc = "Bits 4:5 - Standard counter/Timer 1"]
    #[inline(always)]
    pub fn ctimer1_rule(&mut self) -> CTIMER1_RULE_W {
        CTIMER1_RULE_W { w: self }
    }
    #[doc = "Bits 16:17 - Windiwed wtachdog Timer"]
    #[inline(always)]
    pub fn wwdt_rule(&mut self) -> WWDT_RULE_W {
        WWDT_RULE_W { w: self }
    }
    #[doc = "Bits 20:21 - Multi-rate Timer"]
    #[inline(always)]
    pub fn mrt_rule(&mut self) -> MRT_RULE_W {
        MRT_RULE_W { w: self }
    }
    #[doc = "Bits 24:25 - Micro-Timer"]
    #[inline(always)]
    pub fn utick_rule(&mut self) -> UTICK_RULE_W {
        UTICK_RULE_W { w: self }
    }
}
