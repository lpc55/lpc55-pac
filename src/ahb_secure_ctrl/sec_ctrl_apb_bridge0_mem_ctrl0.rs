#[doc = "Reader of register SEC_CTRL_APB_BRIDGE0_MEM_CTRL0"]
pub type R = crate::R<u32, super::SEC_CTRL_APB_BRIDGE0_MEM_CTRL0>;
#[doc = "Writer for register SEC_CTRL_APB_BRIDGE0_MEM_CTRL0"]
pub type W = crate::W<u32, super::SEC_CTRL_APB_BRIDGE0_MEM_CTRL0>;
#[doc = "Register SEC_CTRL_APB_BRIDGE0_MEM_CTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC_CTRL_APB_BRIDGE0_MEM_CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `SYSCON_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCON_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<SYSCON_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCON_RULE_A) -> Self {
        match variant {
            SYSCON_RULE_A::ENUM_NS_NP => 0,
            SYSCON_RULE_A::ENUM_NS_P => 1,
            SYSCON_RULE_A::ENUM_S_NP => 2,
            SYSCON_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `SYSCON_RULE`"]
pub type SYSCON_RULE_R = crate::R<u8, SYSCON_RULE_A>;
impl SYSCON_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCON_RULE_A {
        match self.bits {
            0 => SYSCON_RULE_A::ENUM_NS_NP,
            1 => SYSCON_RULE_A::ENUM_NS_P,
            2 => SYSCON_RULE_A::ENUM_S_NP,
            3 => SYSCON_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SYSCON_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SYSCON_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SYSCON_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SYSCON_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `SYSCON_RULE`"]
pub struct SYSCON_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCON_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SYSCON_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SYSCON_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SYSCON_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SYSCON_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `IOCON_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCON_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<IOCON_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: IOCON_RULE_A) -> Self {
        match variant {
            IOCON_RULE_A::ENUM_NS_NP => 0,
            IOCON_RULE_A::ENUM_NS_P => 1,
            IOCON_RULE_A::ENUM_S_NP => 2,
            IOCON_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `IOCON_RULE`"]
pub type IOCON_RULE_R = crate::R<u8, IOCON_RULE_A>;
impl IOCON_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCON_RULE_A {
        match self.bits {
            0 => IOCON_RULE_A::ENUM_NS_NP,
            1 => IOCON_RULE_A::ENUM_NS_P,
            2 => IOCON_RULE_A::ENUM_S_NP,
            3 => IOCON_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == IOCON_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == IOCON_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == IOCON_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == IOCON_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `IOCON_RULE`"]
pub struct IOCON_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCON_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOCON_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(IOCON_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(IOCON_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(IOCON_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(IOCON_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `GINT0_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GINT0_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<GINT0_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: GINT0_RULE_A) -> Self {
        match variant {
            GINT0_RULE_A::ENUM_NS_NP => 0,
            GINT0_RULE_A::ENUM_NS_P => 1,
            GINT0_RULE_A::ENUM_S_NP => 2,
            GINT0_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `GINT0_RULE`"]
pub type GINT0_RULE_R = crate::R<u8, GINT0_RULE_A>;
impl GINT0_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GINT0_RULE_A {
        match self.bits {
            0 => GINT0_RULE_A::ENUM_NS_NP,
            1 => GINT0_RULE_A::ENUM_NS_P,
            2 => GINT0_RULE_A::ENUM_S_NP,
            3 => GINT0_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == GINT0_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == GINT0_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == GINT0_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == GINT0_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `GINT0_RULE`"]
pub struct GINT0_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> GINT0_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GINT0_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(GINT0_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(GINT0_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(GINT0_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(GINT0_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `GINT1_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GINT1_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<GINT1_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: GINT1_RULE_A) -> Self {
        match variant {
            GINT1_RULE_A::ENUM_NS_NP => 0,
            GINT1_RULE_A::ENUM_NS_P => 1,
            GINT1_RULE_A::ENUM_S_NP => 2,
            GINT1_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `GINT1_RULE`"]
pub type GINT1_RULE_R = crate::R<u8, GINT1_RULE_A>;
impl GINT1_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GINT1_RULE_A {
        match self.bits {
            0 => GINT1_RULE_A::ENUM_NS_NP,
            1 => GINT1_RULE_A::ENUM_NS_P,
            2 => GINT1_RULE_A::ENUM_S_NP,
            3 => GINT1_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == GINT1_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == GINT1_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == GINT1_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == GINT1_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `GINT1_RULE`"]
pub struct GINT1_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> GINT1_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GINT1_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(GINT1_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(GINT1_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(GINT1_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(GINT1_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `PINT_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<PINT_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: PINT_RULE_A) -> Self {
        match variant {
            PINT_RULE_A::ENUM_NS_NP => 0,
            PINT_RULE_A::ENUM_NS_P => 1,
            PINT_RULE_A::ENUM_S_NP => 2,
            PINT_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `PINT_RULE`"]
pub type PINT_RULE_R = crate::R<u8, PINT_RULE_A>;
impl PINT_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT_RULE_A {
        match self.bits {
            0 => PINT_RULE_A::ENUM_NS_NP,
            1 => PINT_RULE_A::ENUM_NS_P,
            2 => PINT_RULE_A::ENUM_S_NP,
            3 => PINT_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == PINT_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == PINT_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == PINT_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == PINT_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `PINT_RULE`"]
pub struct PINT_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> PINT_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(PINT_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(PINT_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(PINT_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(PINT_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `SEC_PINT_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_PINT_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<SEC_PINT_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_PINT_RULE_A) -> Self {
        match variant {
            SEC_PINT_RULE_A::ENUM_NS_NP => 0,
            SEC_PINT_RULE_A::ENUM_NS_P => 1,
            SEC_PINT_RULE_A::ENUM_S_NP => 2,
            SEC_PINT_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `SEC_PINT_RULE`"]
pub type SEC_PINT_RULE_R = crate::R<u8, SEC_PINT_RULE_A>;
impl SEC_PINT_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_PINT_RULE_A {
        match self.bits {
            0 => SEC_PINT_RULE_A::ENUM_NS_NP,
            1 => SEC_PINT_RULE_A::ENUM_NS_P,
            2 => SEC_PINT_RULE_A::ENUM_S_NP,
            3 => SEC_PINT_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SEC_PINT_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SEC_PINT_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SEC_PINT_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SEC_PINT_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `SEC_PINT_RULE`"]
pub struct SEC_PINT_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_PINT_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEC_PINT_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SEC_PINT_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SEC_PINT_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SEC_PINT_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SEC_PINT_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `INPUTMUX_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUTMUX_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<INPUTMUX_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUTMUX_RULE_A) -> Self {
        match variant {
            INPUTMUX_RULE_A::ENUM_NS_NP => 0,
            INPUTMUX_RULE_A::ENUM_NS_P => 1,
            INPUTMUX_RULE_A::ENUM_S_NP => 2,
            INPUTMUX_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `INPUTMUX_RULE`"]
pub type INPUTMUX_RULE_R = crate::R<u8, INPUTMUX_RULE_A>;
impl INPUTMUX_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUTMUX_RULE_A {
        match self.bits {
            0 => INPUTMUX_RULE_A::ENUM_NS_NP,
            1 => INPUTMUX_RULE_A::ENUM_NS_P,
            2 => INPUTMUX_RULE_A::ENUM_S_NP,
            3 => INPUTMUX_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == INPUTMUX_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == INPUTMUX_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == INPUTMUX_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == INPUTMUX_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `INPUTMUX_RULE`"]
pub struct INPUTMUX_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUTMUX_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUTMUX_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(INPUTMUX_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(INPUTMUX_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(INPUTMUX_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(INPUTMUX_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - System Configuration"]
    #[inline(always)]
    pub fn syscon_rule(&self) -> SYSCON_RULE_R {
        SYSCON_RULE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - I/O Configuration"]
    #[inline(always)]
    pub fn iocon_rule(&self) -> IOCON_RULE_R {
        IOCON_RULE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - GPIO input Interrupt 0"]
    #[inline(always)]
    pub fn gint0_rule(&self) -> GINT0_RULE_R {
        GINT0_RULE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - GPIO input Interrupt 1"]
    #[inline(always)]
    pub fn gint1_rule(&self) -> GINT1_RULE_R {
        GINT1_RULE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Pin Interrupt and Pattern match"]
    #[inline(always)]
    pub fn pint_rule(&self) -> PINT_RULE_R {
        PINT_RULE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Secure Pin Interrupt and Pattern match"]
    #[inline(always)]
    pub fn sec_pint_rule(&self) -> SEC_PINT_RULE_R {
        SEC_PINT_RULE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Peripheral input multiplexing"]
    #[inline(always)]
    pub fn inputmux_rule(&self) -> INPUTMUX_RULE_R {
        INPUTMUX_RULE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System Configuration"]
    #[inline(always)]
    pub fn syscon_rule(&mut self) -> SYSCON_RULE_W {
        SYSCON_RULE_W { w: self }
    }
    #[doc = "Bits 4:5 - I/O Configuration"]
    #[inline(always)]
    pub fn iocon_rule(&mut self) -> IOCON_RULE_W {
        IOCON_RULE_W { w: self }
    }
    #[doc = "Bits 8:9 - GPIO input Interrupt 0"]
    #[inline(always)]
    pub fn gint0_rule(&mut self) -> GINT0_RULE_W {
        GINT0_RULE_W { w: self }
    }
    #[doc = "Bits 12:13 - GPIO input Interrupt 1"]
    #[inline(always)]
    pub fn gint1_rule(&mut self) -> GINT1_RULE_W {
        GINT1_RULE_W { w: self }
    }
    #[doc = "Bits 16:17 - Pin Interrupt and Pattern match"]
    #[inline(always)]
    pub fn pint_rule(&mut self) -> PINT_RULE_W {
        PINT_RULE_W { w: self }
    }
    #[doc = "Bits 20:21 - Secure Pin Interrupt and Pattern match"]
    #[inline(always)]
    pub fn sec_pint_rule(&mut self) -> SEC_PINT_RULE_W {
        SEC_PINT_RULE_W { w: self }
    }
    #[doc = "Bits 24:25 - Peripheral input multiplexing"]
    #[inline(always)]
    pub fn inputmux_rule(&mut self) -> INPUTMUX_RULE_W {
        INPUTMUX_RULE_W { w: self }
    }
}
