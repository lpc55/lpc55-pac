#[doc = "Reader of register SEC_CTRL_APB_BRIDGE1_MEM_CTRL1"]
pub type R = crate::R<u32, super::SEC_CTRL_APB_BRIDGE1_MEM_CTRL1>;
#[doc = "Writer for register SEC_CTRL_APB_BRIDGE1_MEM_CTRL1"]
pub type W = crate::W<u32, super::SEC_CTRL_APB_BRIDGE1_MEM_CTRL1>;
#[doc = "Register SEC_CTRL_APB_BRIDGE1_MEM_CTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC_CTRL_APB_BRIDGE1_MEM_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Standard counter/Timer 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTIMER2_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<CTIMER2_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTIMER2_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CTIMER2_RULE`"]
pub type CTIMER2_RULE_R = crate::R<u8, CTIMER2_RULE_A>;
impl CTIMER2_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER2_RULE_A {
        match self.bits {
            0 => CTIMER2_RULE_A::ENUM_NS_NP,
            1 => CTIMER2_RULE_A::ENUM_NS_P,
            2 => CTIMER2_RULE_A::ENUM_S_NP,
            3 => CTIMER2_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == CTIMER2_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == CTIMER2_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == CTIMER2_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == CTIMER2_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `CTIMER2_RULE`"]
pub struct CTIMER2_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER2_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMER2_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(CTIMER2_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(CTIMER2_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(CTIMER2_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(CTIMER2_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Standard counter/Timer 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTIMER3_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<CTIMER3_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTIMER3_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CTIMER3_RULE`"]
pub type CTIMER3_RULE_R = crate::R<u8, CTIMER3_RULE_A>;
impl CTIMER3_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER3_RULE_A {
        match self.bits {
            0 => CTIMER3_RULE_A::ENUM_NS_NP,
            1 => CTIMER3_RULE_A::ENUM_NS_P,
            2 => CTIMER3_RULE_A::ENUM_S_NP,
            3 => CTIMER3_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == CTIMER3_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == CTIMER3_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == CTIMER3_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == CTIMER3_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `CTIMER3_RULE`"]
pub struct CTIMER3_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER3_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMER3_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(CTIMER3_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(CTIMER3_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(CTIMER3_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(CTIMER3_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Standard counter/Timer 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTIMER4_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<CTIMER4_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTIMER4_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CTIMER4_RULE`"]
pub type CTIMER4_RULE_R = crate::R<u8, CTIMER4_RULE_A>;
impl CTIMER4_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER4_RULE_A {
        match self.bits {
            0 => CTIMER4_RULE_A::ENUM_NS_NP,
            1 => CTIMER4_RULE_A::ENUM_NS_P,
            2 => CTIMER4_RULE_A::ENUM_S_NP,
            3 => CTIMER4_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == CTIMER4_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == CTIMER4_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == CTIMER4_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == CTIMER4_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `CTIMER4_RULE`"]
pub struct CTIMER4_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER4_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMER4_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(CTIMER4_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(CTIMER4_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(CTIMER4_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(CTIMER4_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Real Time Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTC_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RTC_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: RTC_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTC_RULE`"]
pub type RTC_RULE_R = crate::R<u8, RTC_RULE_A>;
impl RTC_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_RULE_A {
        match self.bits {
            0 => RTC_RULE_A::ENUM_NS_NP,
            1 => RTC_RULE_A::ENUM_NS_P,
            2 => RTC_RULE_A::ENUM_S_NP,
            3 => RTC_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RTC_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RTC_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RTC_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RTC_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `RTC_RULE`"]
pub struct RTC_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RTC_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RTC_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RTC_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RTC_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "OS Event Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSEVENT_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<OSEVENT_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: OSEVENT_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OSEVENT_RULE`"]
pub type OSEVENT_RULE_R = crate::R<u8, OSEVENT_RULE_A>;
impl OSEVENT_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSEVENT_RULE_A {
        match self.bits {
            0 => OSEVENT_RULE_A::ENUM_NS_NP,
            1 => OSEVENT_RULE_A::ENUM_NS_P,
            2 => OSEVENT_RULE_A::ENUM_S_NP,
            3 => OSEVENT_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == OSEVENT_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == OSEVENT_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == OSEVENT_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == OSEVENT_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `OSEVENT_RULE`"]
pub struct OSEVENT_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> OSEVENT_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSEVENT_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(OSEVENT_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(OSEVENT_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(OSEVENT_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(OSEVENT_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Standard counter/Timer 2"]
    #[inline(always)]
    pub fn ctimer2_rule(&self) -> CTIMER2_RULE_R {
        CTIMER2_RULE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Standard counter/Timer 3"]
    #[inline(always)]
    pub fn ctimer3_rule(&self) -> CTIMER3_RULE_R {
        CTIMER3_RULE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Standard counter/Timer 4"]
    #[inline(always)]
    pub fn ctimer4_rule(&self) -> CTIMER4_RULE_R {
        CTIMER4_RULE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Real Time Counter"]
    #[inline(always)]
    pub fn rtc_rule(&self) -> RTC_RULE_R {
        RTC_RULE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - OS Event Timer"]
    #[inline(always)]
    pub fn osevent_rule(&self) -> OSEVENT_RULE_R {
        OSEVENT_RULE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Standard counter/Timer 2"]
    #[inline(always)]
    pub fn ctimer2_rule(&mut self) -> CTIMER2_RULE_W {
        CTIMER2_RULE_W { w: self }
    }
    #[doc = "Bits 4:5 - Standard counter/Timer 3"]
    #[inline(always)]
    pub fn ctimer3_rule(&mut self) -> CTIMER3_RULE_W {
        CTIMER3_RULE_W { w: self }
    }
    #[doc = "Bits 8:9 - Standard counter/Timer 4"]
    #[inline(always)]
    pub fn ctimer4_rule(&mut self) -> CTIMER4_RULE_W {
        CTIMER4_RULE_W { w: self }
    }
    #[doc = "Bits 16:17 - Real Time Counter"]
    #[inline(always)]
    pub fn rtc_rule(&mut self) -> RTC_RULE_W {
        RTC_RULE_W { w: self }
    }
    #[doc = "Bits 20:21 - OS Event Timer"]
    #[inline(always)]
    pub fn osevent_rule(&mut self) -> OSEVENT_RULE_W {
        OSEVENT_RULE_W { w: self }
    }
}
