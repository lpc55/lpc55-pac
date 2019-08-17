#[doc = "Reader of register SEC_CTRL_RAM0_MEM_RULE1"]
pub type R = crate::R<u32, super::SEC_CTRL_RAM0_MEM_RULE1>;
#[doc = "Writer for register SEC_CTRL_RAM0_MEM_RULE1"]
pub type W = crate::W<u32, super::SEC_CTRL_RAM0_MEM_RULE1>;
#[doc = "Register SEC_CTRL_RAM0_MEM_RULE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC_CTRL_RAM0_MEM_RULE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `RULE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RULE0_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<RULE0_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE0_A) -> Self {
        match variant {
            RULE0_A::ENUM_NS_NP => 0,
            RULE0_A::ENUM_NS_P => 1,
            RULE0_A::ENUM_S_NP => 2,
            RULE0_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `RULE0`"]
pub type RULE0_R = crate::R<u8, RULE0_A>;
impl RULE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RULE0_A {
        match self.bits {
            0 => RULE0_A::ENUM_NS_NP,
            1 => RULE0_A::ENUM_NS_P,
            2 => RULE0_A::ENUM_S_NP,
            3 => RULE0_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RULE0_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RULE0_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RULE0_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RULE0_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `RULE0`"]
pub struct RULE0_W<'a> {
    w: &'a mut W,
}
impl<'a> RULE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RULE0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RULE0_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RULE0_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RULE0_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RULE0_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `RULE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RULE1_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<RULE1_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE1_A) -> Self {
        match variant {
            RULE1_A::ENUM_NS_NP => 0,
            RULE1_A::ENUM_NS_P => 1,
            RULE1_A::ENUM_S_NP => 2,
            RULE1_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `RULE1`"]
pub type RULE1_R = crate::R<u8, RULE1_A>;
impl RULE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RULE1_A {
        match self.bits {
            0 => RULE1_A::ENUM_NS_NP,
            1 => RULE1_A::ENUM_NS_P,
            2 => RULE1_A::ENUM_S_NP,
            3 => RULE1_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RULE1_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RULE1_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RULE1_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RULE1_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `RULE1`"]
pub struct RULE1_W<'a> {
    w: &'a mut W,
}
impl<'a> RULE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RULE1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RULE1_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RULE1_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RULE1_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RULE1_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `RULE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RULE2_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<RULE2_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE2_A) -> Self {
        match variant {
            RULE2_A::ENUM_NS_NP => 0,
            RULE2_A::ENUM_NS_P => 1,
            RULE2_A::ENUM_S_NP => 2,
            RULE2_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `RULE2`"]
pub type RULE2_R = crate::R<u8, RULE2_A>;
impl RULE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RULE2_A {
        match self.bits {
            0 => RULE2_A::ENUM_NS_NP,
            1 => RULE2_A::ENUM_NS_P,
            2 => RULE2_A::ENUM_S_NP,
            3 => RULE2_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RULE2_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RULE2_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RULE2_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RULE2_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `RULE2`"]
pub struct RULE2_W<'a> {
    w: &'a mut W,
}
impl<'a> RULE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RULE2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RULE2_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RULE2_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RULE2_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RULE2_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `RULE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RULE3_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<RULE3_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE3_A) -> Self {
        match variant {
            RULE3_A::ENUM_NS_NP => 0,
            RULE3_A::ENUM_NS_P => 1,
            RULE3_A::ENUM_S_NP => 2,
            RULE3_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `RULE3`"]
pub type RULE3_R = crate::R<u8, RULE3_A>;
impl RULE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RULE3_A {
        match self.bits {
            0 => RULE3_A::ENUM_NS_NP,
            1 => RULE3_A::ENUM_NS_P,
            2 => RULE3_A::ENUM_S_NP,
            3 => RULE3_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RULE3_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RULE3_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RULE3_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RULE3_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `RULE3`"]
pub struct RULE3_W<'a> {
    w: &'a mut W,
}
impl<'a> RULE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RULE3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RULE3_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RULE3_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RULE3_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RULE3_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `RULE4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RULE4_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<RULE4_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE4_A) -> Self {
        match variant {
            RULE4_A::ENUM_NS_NP => 0,
            RULE4_A::ENUM_NS_P => 1,
            RULE4_A::ENUM_S_NP => 2,
            RULE4_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `RULE4`"]
pub type RULE4_R = crate::R<u8, RULE4_A>;
impl RULE4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RULE4_A {
        match self.bits {
            0 => RULE4_A::ENUM_NS_NP,
            1 => RULE4_A::ENUM_NS_P,
            2 => RULE4_A::ENUM_S_NP,
            3 => RULE4_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RULE4_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RULE4_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RULE4_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RULE4_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `RULE4`"]
pub struct RULE4_W<'a> {
    w: &'a mut W,
}
impl<'a> RULE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RULE4_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RULE4_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RULE4_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RULE4_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RULE4_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `RULE5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RULE5_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<RULE5_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE5_A) -> Self {
        match variant {
            RULE5_A::ENUM_NS_NP => 0,
            RULE5_A::ENUM_NS_P => 1,
            RULE5_A::ENUM_S_NP => 2,
            RULE5_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `RULE5`"]
pub type RULE5_R = crate::R<u8, RULE5_A>;
impl RULE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RULE5_A {
        match self.bits {
            0 => RULE5_A::ENUM_NS_NP,
            1 => RULE5_A::ENUM_NS_P,
            2 => RULE5_A::ENUM_S_NP,
            3 => RULE5_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RULE5_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RULE5_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RULE5_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RULE5_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `RULE5`"]
pub struct RULE5_W<'a> {
    w: &'a mut W,
}
impl<'a> RULE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RULE5_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RULE5_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RULE5_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RULE5_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RULE5_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `RULE6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RULE6_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<RULE6_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE6_A) -> Self {
        match variant {
            RULE6_A::ENUM_NS_NP => 0,
            RULE6_A::ENUM_NS_P => 1,
            RULE6_A::ENUM_S_NP => 2,
            RULE6_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `RULE6`"]
pub type RULE6_R = crate::R<u8, RULE6_A>;
impl RULE6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RULE6_A {
        match self.bits {
            0 => RULE6_A::ENUM_NS_NP,
            1 => RULE6_A::ENUM_NS_P,
            2 => RULE6_A::ENUM_S_NP,
            3 => RULE6_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RULE6_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RULE6_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RULE6_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RULE6_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `RULE6`"]
pub struct RULE6_W<'a> {
    w: &'a mut W,
}
impl<'a> RULE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RULE6_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RULE6_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RULE6_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RULE6_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RULE6_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `RULE7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RULE7_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<RULE7_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE7_A) -> Self {
        match variant {
            RULE7_A::ENUM_NS_NP => 0,
            RULE7_A::ENUM_NS_P => 1,
            RULE7_A::ENUM_S_NP => 2,
            RULE7_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `RULE7`"]
pub type RULE7_R = crate::R<u8, RULE7_A>;
impl RULE7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RULE7_A {
        match self.bits {
            0 => RULE7_A::ENUM_NS_NP,
            1 => RULE7_A::ENUM_NS_P,
            2 => RULE7_A::ENUM_S_NP,
            3 => RULE7_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RULE7_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RULE7_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RULE7_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RULE7_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `RULE7`"]
pub struct RULE7_W<'a> {
    w: &'a mut W,
}
impl<'a> RULE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RULE7_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RULE7_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RULE7_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RULE7_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RULE7_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule0(&self) -> RULE0_R {
        RULE0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule1(&self) -> RULE1_R {
        RULE1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule2(&self) -> RULE2_R {
        RULE2_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule3(&self) -> RULE3_R {
        RULE3_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule4(&self) -> RULE4_R {
        RULE4_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule5(&self) -> RULE5_R {
        RULE5_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule6(&self) -> RULE6_R {
        RULE6_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule7(&self) -> RULE7_R {
        RULE7_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule0(&mut self) -> RULE0_W {
        RULE0_W { w: self }
    }
    #[doc = "Bits 4:5 - secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule1(&mut self) -> RULE1_W {
        RULE1_W { w: self }
    }
    #[doc = "Bits 8:9 - secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule2(&mut self) -> RULE2_W {
        RULE2_W { w: self }
    }
    #[doc = "Bits 12:13 - secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule3(&mut self) -> RULE3_W {
        RULE3_W { w: self }
    }
    #[doc = "Bits 16:17 - secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule4(&mut self) -> RULE4_W {
        RULE4_W { w: self }
    }
    #[doc = "Bits 20:21 - secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule5(&mut self) -> RULE5_W {
        RULE5_W { w: self }
    }
    #[doc = "Bits 24:25 - secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule6(&mut self) -> RULE6_W {
        RULE6_W { w: self }
    }
    #[doc = "Bits 28:29 - secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule7(&mut self) -> RULE7_W {
        RULE7_W { w: self }
    }
}
