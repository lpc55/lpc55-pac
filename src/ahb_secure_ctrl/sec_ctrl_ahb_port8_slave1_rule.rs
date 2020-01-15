#[doc = "Reader of register SEC_CTRL_AHB_PORT8_SLAVE1_RULE"]
pub type R = crate::R<u32, super::SEC_CTRL_AHB_PORT8_SLAVE1_RULE>;
#[doc = "Writer for register SEC_CTRL_AHB_PORT8_SLAVE1_RULE"]
pub type W = crate::W<u32, super::SEC_CTRL_AHB_PORT8_SLAVE1_RULE>;
#[doc = "Register SEC_CTRL_AHB_PORT8_SLAVE1_RULE `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC_CTRL_AHB_PORT8_SLAVE1_RULE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Flexcomm interface 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLEXCOMM2_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<FLEXCOMM2_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM2_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLEXCOMM2_RULE`"]
pub type FLEXCOMM2_RULE_R = crate::R<u8, FLEXCOMM2_RULE_A>;
impl FLEXCOMM2_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM2_RULE_A {
        match self.bits {
            0 => FLEXCOMM2_RULE_A::ENUM_NS_NP,
            1 => FLEXCOMM2_RULE_A::ENUM_NS_P,
            2 => FLEXCOMM2_RULE_A::ENUM_S_NP,
            3 => FLEXCOMM2_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == FLEXCOMM2_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == FLEXCOMM2_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == FLEXCOMM2_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == FLEXCOMM2_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `FLEXCOMM2_RULE`"]
pub struct FLEXCOMM2_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM2_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCOMM2_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(FLEXCOMM2_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(FLEXCOMM2_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(FLEXCOMM2_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(FLEXCOMM2_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Flexcomm interface 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLEXCOMM3_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<FLEXCOMM3_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM3_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLEXCOMM3_RULE`"]
pub type FLEXCOMM3_RULE_R = crate::R<u8, FLEXCOMM3_RULE_A>;
impl FLEXCOMM3_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM3_RULE_A {
        match self.bits {
            0 => FLEXCOMM3_RULE_A::ENUM_NS_NP,
            1 => FLEXCOMM3_RULE_A::ENUM_NS_P,
            2 => FLEXCOMM3_RULE_A::ENUM_S_NP,
            3 => FLEXCOMM3_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == FLEXCOMM3_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == FLEXCOMM3_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == FLEXCOMM3_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == FLEXCOMM3_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `FLEXCOMM3_RULE`"]
pub struct FLEXCOMM3_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM3_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCOMM3_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(FLEXCOMM3_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(FLEXCOMM3_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(FLEXCOMM3_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(FLEXCOMM3_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Flexcomm interface 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLEXCOMM4_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<FLEXCOMM4_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM4_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLEXCOMM4_RULE`"]
pub type FLEXCOMM4_RULE_R = crate::R<u8, FLEXCOMM4_RULE_A>;
impl FLEXCOMM4_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM4_RULE_A {
        match self.bits {
            0 => FLEXCOMM4_RULE_A::ENUM_NS_NP,
            1 => FLEXCOMM4_RULE_A::ENUM_NS_P,
            2 => FLEXCOMM4_RULE_A::ENUM_S_NP,
            3 => FLEXCOMM4_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == FLEXCOMM4_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == FLEXCOMM4_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == FLEXCOMM4_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == FLEXCOMM4_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `FLEXCOMM4_RULE`"]
pub struct FLEXCOMM4_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM4_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCOMM4_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(FLEXCOMM4_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(FLEXCOMM4_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(FLEXCOMM4_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(FLEXCOMM4_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Inter CPU communication Mailbox\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MAILBOX_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<MAILBOX_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: MAILBOX_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MAILBOX_RULE`"]
pub type MAILBOX_RULE_R = crate::R<u8, MAILBOX_RULE_A>;
impl MAILBOX_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAILBOX_RULE_A {
        match self.bits {
            0 => MAILBOX_RULE_A::ENUM_NS_NP,
            1 => MAILBOX_RULE_A::ENUM_NS_P,
            2 => MAILBOX_RULE_A::ENUM_S_NP,
            3 => MAILBOX_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == MAILBOX_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == MAILBOX_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == MAILBOX_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == MAILBOX_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `MAILBOX_RULE`"]
pub struct MAILBOX_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> MAILBOX_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAILBOX_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(MAILBOX_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(MAILBOX_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(MAILBOX_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(MAILBOX_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "High Speed GPIO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO0_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<GPIO0_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO0_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO0_RULE`"]
pub type GPIO0_RULE_R = crate::R<u8, GPIO0_RULE_A>;
impl GPIO0_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0_RULE_A {
        match self.bits {
            0 => GPIO0_RULE_A::ENUM_NS_NP,
            1 => GPIO0_RULE_A::ENUM_NS_P,
            2 => GPIO0_RULE_A::ENUM_S_NP,
            3 => GPIO0_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == GPIO0_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == GPIO0_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == GPIO0_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == GPIO0_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `GPIO0_RULE`"]
pub struct GPIO0_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO0_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO0_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(GPIO0_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(GPIO0_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(GPIO0_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(GPIO0_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Flexcomm interface 2"]
    #[inline(always)]
    pub fn flexcomm2_rule(&self) -> FLEXCOMM2_RULE_R {
        FLEXCOMM2_RULE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Flexcomm interface 3"]
    #[inline(always)]
    pub fn flexcomm3_rule(&self) -> FLEXCOMM3_RULE_R {
        FLEXCOMM3_RULE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Flexcomm interface 4"]
    #[inline(always)]
    pub fn flexcomm4_rule(&self) -> FLEXCOMM4_RULE_R {
        FLEXCOMM4_RULE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Inter CPU communication Mailbox"]
    #[inline(always)]
    pub fn mailbox_rule(&self) -> MAILBOX_RULE_R {
        MAILBOX_RULE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - High Speed GPIO"]
    #[inline(always)]
    pub fn gpio0_rule(&self) -> GPIO0_RULE_R {
        GPIO0_RULE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Flexcomm interface 2"]
    #[inline(always)]
    pub fn flexcomm2_rule(&mut self) -> FLEXCOMM2_RULE_W {
        FLEXCOMM2_RULE_W { w: self }
    }
    #[doc = "Bits 4:5 - Flexcomm interface 3"]
    #[inline(always)]
    pub fn flexcomm3_rule(&mut self) -> FLEXCOMM3_RULE_W {
        FLEXCOMM3_RULE_W { w: self }
    }
    #[doc = "Bits 8:9 - Flexcomm interface 4"]
    #[inline(always)]
    pub fn flexcomm4_rule(&mut self) -> FLEXCOMM4_RULE_W {
        FLEXCOMM4_RULE_W { w: self }
    }
    #[doc = "Bits 12:13 - Inter CPU communication Mailbox"]
    #[inline(always)]
    pub fn mailbox_rule(&mut self) -> MAILBOX_RULE_W {
        MAILBOX_RULE_W { w: self }
    }
    #[doc = "Bits 16:17 - High Speed GPIO"]
    #[inline(always)]
    pub fn gpio0_rule(&mut self) -> GPIO0_RULE_W {
        GPIO0_RULE_W { w: self }
    }
}
