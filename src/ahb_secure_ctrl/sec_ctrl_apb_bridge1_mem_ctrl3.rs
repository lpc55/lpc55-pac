#[doc = "Register `SEC_CTRL_APB_BRIDGE1_MEM_CTRL3` reader"]
pub struct R(crate::R<SEC_CTRL_APB_BRIDGE1_MEM_CTRL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CTRL_APB_BRIDGE1_MEM_CTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SEC_CTRL_APB_BRIDGE1_MEM_CTRL3_SPEC>> for R {
    fn from(reader: crate::R<SEC_CTRL_APB_BRIDGE1_MEM_CTRL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CTRL_APB_BRIDGE1_MEM_CTRL3` writer"]
pub struct W(crate::W<SEC_CTRL_APB_BRIDGE1_MEM_CTRL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CTRL_APB_BRIDGE1_MEM_CTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<SEC_CTRL_APB_BRIDGE1_MEM_CTRL3_SPEC>> for W {
    fn from(writer: crate::W<SEC_CTRL_APB_BRIDGE1_MEM_CTRL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB High Speed Phy controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USBHPHY_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<USBHPHY_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: USBHPHY_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USBHPHY_RULE` reader - USB High Speed Phy controller"]
pub struct USBHPHY_RULE_R(crate::FieldReader<u8, USBHPHY_RULE_A>);
impl USBHPHY_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        USBHPHY_RULE_R(crate::FieldReader::new(bits))
    }
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
        **self == USBHPHY_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == USBHPHY_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == USBHPHY_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == USBHPHY_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for USBHPHY_RULE_R {
    type Target = crate::FieldReader<u8, USBHPHY_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBHPHY_RULE` writer - USB High Speed Phy controller"]
pub struct USBHPHY_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBHPHY_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBHPHY_RULE_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "True Random Number Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RNG_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RNG_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: RNG_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RNG_RULE` reader - True Random Number Generator"]
pub struct RNG_RULE_R(crate::FieldReader<u8, RNG_RULE_A>);
impl RNG_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RNG_RULE_R(crate::FieldReader::new(bits))
    }
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
        **self == RNG_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == RNG_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == RNG_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == RNG_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for RNG_RULE_R {
    type Target = crate::FieldReader<u8, RNG_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNG_RULE` writer - True Random Number Generator"]
pub struct RNG_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNG_RULE_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "PUF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PUF_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<PUF_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: PUF_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PUF_RULE` reader - PUF"]
pub struct PUF_RULE_R(crate::FieldReader<u8, PUF_RULE_A>);
impl PUF_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PUF_RULE_R(crate::FieldReader::new(bits))
    }
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
        **self == PUF_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == PUF_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == PUF_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == PUF_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for PUF_RULE_R {
    type Target = crate::FieldReader<u8, PUF_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUF_RULE` writer - PUF"]
pub struct PUF_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> PUF_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUF_RULE_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Programmable Look-Up logic\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLU_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<PLU_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: PLU_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLU_RULE` reader - Programmable Look-Up logic"]
pub struct PLU_RULE_R(crate::FieldReader<u8, PLU_RULE_A>);
impl PLU_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLU_RULE_R(crate::FieldReader::new(bits))
    }
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
        **self == PLU_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == PLU_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == PLU_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == PLU_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for PLU_RULE_R {
    type Target = crate::FieldReader<u8, PLU_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLU_RULE` writer - Programmable Look-Up logic"]
pub struct PLU_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLU_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLU_RULE_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "ROM patch unit access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RPU_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Priviledge user access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RPU_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: RPU_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RPU_RULE` reader - ROM patch unit access"]
pub struct RPU_RULE_R(crate::FieldReader<u8, RPU_RULE_A>);
impl RPU_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPU_RULE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPU_RULE_A {
        match self.bits {
            0 => RPU_RULE_A::ENUM_NS_NP,
            1 => RPU_RULE_A::ENUM_NS_P,
            2 => RPU_RULE_A::ENUM_S_NP,
            3 => RPU_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == RPU_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == RPU_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == RPU_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == RPU_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for RPU_RULE_R {
    type Target = crate::FieldReader<u8, RPU_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPU_RULE` writer - ROM patch unit access"]
pub struct RPU_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> RPU_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPU_RULE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RPU_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RPU_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RPU_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RPU_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
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
    #[doc = "Bits 24:25 - ROM patch unit access"]
    #[inline(always)]
    pub fn rpu_rule(&self) -> RPU_RULE_R {
        RPU_RULE_R::new(((self.bits >> 24) & 0x03) as u8)
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
    #[doc = "Bits 24:25 - ROM patch unit access"]
    #[inline(always)]
    pub fn rpu_rule(&mut self) -> RPU_RULE_W {
        RPU_RULE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_apb_bridge1_mem_ctrl3](index.html) module"]
pub struct SEC_CTRL_APB_BRIDGE1_MEM_CTRL3_SPEC;
impl crate::RegisterSpec for SEC_CTRL_APB_BRIDGE1_MEM_CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_ctrl_apb_bridge1_mem_ctrl3::R](R) reader structure"]
impl crate::Readable for SEC_CTRL_APB_BRIDGE1_MEM_CTRL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_apb_bridge1_mem_ctrl3::W](W) writer structure"]
impl crate::Writable for SEC_CTRL_APB_BRIDGE1_MEM_CTRL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC_CTRL_APB_BRIDGE1_MEM_CTRL3 to value 0"]
impl crate::Resettable for SEC_CTRL_APB_BRIDGE1_MEM_CTRL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
