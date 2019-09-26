#[doc = "Reader of register SEC_CTRL_APB_BRIDGE1_MEM_CTRL3"]
pub type R = crate::R<u32, super::SEC_CTRL_APB_BRIDGE1_MEM_CTRL3>;
#[doc = "Writer for register SEC_CTRL_APB_BRIDGE1_MEM_CTRL3"]
pub type W = crate::W<u32, super::SEC_CTRL_APB_BRIDGE1_MEM_CTRL3>;
#[doc = "Register SEC_CTRL_APB_BRIDGE1_MEM_CTRL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC_CTRL_APB_BRIDGE1_MEM_CTRL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `USBHPHY_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBHPHY_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<USBHPHY_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: USBHPHY_RULE_A) -> Self {
        match variant {
            USBHPHY_RULE_A::ENUM_NS_NP => 0,
            USBHPHY_RULE_A::ENUM_NS_P => 1,
            USBHPHY_RULE_A::ENUM_S_NP => 2,
            USBHPHY_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `USBHPHY_RULE`"]
pub type USBHPHY_RULE_R = crate::R<u8, USBHPHY_RULE_A>;
impl USBHPHY_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBHPHY_RULE_A {
        match self.bits {
            0 => USBHPHY_RULE_A::ENUM_NS_NP,
            1 => USBHPHY_RULE_A::ENUM_NS_P,
            2 => USBHPHY_RULE_A::ENUM_S_NP,
            3 => USBHPHY_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == USBHPHY_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == USBHPHY_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == USBHPHY_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == USBHPHY_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `USBHPHY_RULE`"]
pub struct USBHPHY_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBHPHY_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBHPHY_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(USBHPHY_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(USBHPHY_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(USBHPHY_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(USBHPHY_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `RNG_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNG_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<RNG_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: RNG_RULE_A) -> Self {
        match variant {
            RNG_RULE_A::ENUM_NS_NP => 0,
            RNG_RULE_A::ENUM_NS_P => 1,
            RNG_RULE_A::ENUM_S_NP => 2,
            RNG_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `RNG_RULE`"]
pub type RNG_RULE_R = crate::R<u8, RNG_RULE_A>;
impl RNG_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG_RULE_A {
        match self.bits {
            0 => RNG_RULE_A::ENUM_NS_NP,
            1 => RNG_RULE_A::ENUM_NS_P,
            2 => RNG_RULE_A::ENUM_S_NP,
            3 => RNG_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RNG_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RNG_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RNG_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RNG_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `RNG_RULE`"]
pub struct RNG_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNG_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RNG_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RNG_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RNG_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RNG_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `PUF_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUF_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<PUF_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: PUF_RULE_A) -> Self {
        match variant {
            PUF_RULE_A::ENUM_NS_NP => 0,
            PUF_RULE_A::ENUM_NS_P => 1,
            PUF_RULE_A::ENUM_S_NP => 2,
            PUF_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `PUF_RULE`"]
pub type PUF_RULE_R = crate::R<u8, PUF_RULE_A>;
impl PUF_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUF_RULE_A {
        match self.bits {
            0 => PUF_RULE_A::ENUM_NS_NP,
            1 => PUF_RULE_A::ENUM_NS_P,
            2 => PUF_RULE_A::ENUM_S_NP,
            3 => PUF_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == PUF_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == PUF_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == PUF_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == PUF_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `PUF_RULE`"]
pub struct PUF_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> PUF_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUF_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(PUF_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(PUF_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(PUF_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(PUF_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `PLU_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLU_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<PLU_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: PLU_RULE_A) -> Self {
        match variant {
            PLU_RULE_A::ENUM_NS_NP => 0,
            PLU_RULE_A::ENUM_NS_P => 1,
            PLU_RULE_A::ENUM_S_NP => 2,
            PLU_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `PLU_RULE`"]
pub type PLU_RULE_R = crate::R<u8, PLU_RULE_A>;
impl PLU_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLU_RULE_A {
        match self.bits {
            0 => PLU_RULE_A::ENUM_NS_NP,
            1 => PLU_RULE_A::ENUM_NS_P,
            2 => PLU_RULE_A::ENUM_S_NP,
            3 => PLU_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == PLU_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == PLU_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == PLU_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == PLU_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `PLU_RULE`"]
pub struct PLU_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLU_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLU_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(PLU_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(PLU_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(PLU_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(PLU_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - USB High Speed Phy controller"]
    #[inline(always)]
    pub fn usbhphy_rule(&self) -> USBHPHY_RULE_R {
        USBHPHY_RULE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - True Random Number Generator"]
    #[inline(always)]
    pub fn rng_rule(&self) -> RNG_RULE_R {
        RNG_RULE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - PUF"]
    #[inline(always)]
    pub fn puf_rule(&self) -> PUF_RULE_R {
        PUF_RULE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Programmable Look-Up logic"]
    #[inline(always)]
    pub fn plu_rule(&self) -> PLU_RULE_R {
        PLU_RULE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USB High Speed Phy controller"]
    #[inline(always)]
    pub fn usbhphy_rule(&mut self) -> USBHPHY_RULE_W {
        USBHPHY_RULE_W { w: self }
    }
    #[doc = "Bits 8:9 - True Random Number Generator"]
    #[inline(always)]
    pub fn rng_rule(&mut self) -> RNG_RULE_W {
        RNG_RULE_W { w: self }
    }
    #[doc = "Bits 12:13 - PUF"]
    #[inline(always)]
    pub fn puf_rule(&mut self) -> PUF_RULE_W {
        PUF_RULE_W { w: self }
    }
    #[doc = "Bits 20:21 - Programmable Look-Up logic"]
    #[inline(always)]
    pub fn plu_rule(&mut self) -> PLU_RULE_W {
        PLU_RULE_W { w: self }
    }
}
