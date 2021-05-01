#[doc = "Register `SEC_CTRL_APB_BRIDGE0_MEM_CTRL1` reader"]
pub struct R(crate::R<SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC>> for R {
    fn from(reader: crate::R<SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CTRL_APB_BRIDGE0_MEM_CTRL1` writer"]
pub struct W(crate::W<SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC>;
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
impl core::convert::From<crate::W<SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC>> for W {
    fn from(writer: crate::W<SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Standard counter/Timer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTIMER0_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<CTIMER0_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTIMER0_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CTIMER0_RULE` reader - Standard counter/Timer 0"]
pub struct CTIMER0_RULE_R(crate::FieldReader<u8, CTIMER0_RULE_A>);
impl CTIMER0_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTIMER0_RULE_R(crate::FieldReader::new(bits))
    }
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
        **self == CTIMER0_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == CTIMER0_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == CTIMER0_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == CTIMER0_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for CTIMER0_RULE_R {
    type Target = crate::FieldReader<u8, CTIMER0_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMER0_RULE` writer - Standard counter/Timer 0"]
pub struct CTIMER0_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER0_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMER0_RULE_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Standard counter/Timer 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTIMER1_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<CTIMER1_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTIMER1_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CTIMER1_RULE` reader - Standard counter/Timer 1"]
pub struct CTIMER1_RULE_R(crate::FieldReader<u8, CTIMER1_RULE_A>);
impl CTIMER1_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTIMER1_RULE_R(crate::FieldReader::new(bits))
    }
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
        **self == CTIMER1_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == CTIMER1_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == CTIMER1_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == CTIMER1_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for CTIMER1_RULE_R {
    type Target = crate::FieldReader<u8, CTIMER1_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMER1_RULE` writer - Standard counter/Timer 1"]
pub struct CTIMER1_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER1_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMER1_RULE_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Windiwed wtachdog Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WWDT_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<WWDT_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: WWDT_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WWDT_RULE` reader - Windiwed wtachdog Timer"]
pub struct WWDT_RULE_R(crate::FieldReader<u8, WWDT_RULE_A>);
impl WWDT_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        WWDT_RULE_R(crate::FieldReader::new(bits))
    }
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
        **self == WWDT_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == WWDT_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == WWDT_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == WWDT_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for WWDT_RULE_R {
    type Target = crate::FieldReader<u8, WWDT_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WWDT_RULE` writer - Windiwed wtachdog Timer"]
pub struct WWDT_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDT_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDT_RULE_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Multi-rate Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MRT_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<MRT_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: MRT_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MRT_RULE` reader - Multi-rate Timer"]
pub struct MRT_RULE_R(crate::FieldReader<u8, MRT_RULE_A>);
impl MRT_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MRT_RULE_R(crate::FieldReader::new(bits))
    }
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
        **self == MRT_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == MRT_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == MRT_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == MRT_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for MRT_RULE_R {
    type Target = crate::FieldReader<u8, MRT_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRT_RULE` writer - Multi-rate Timer"]
pub struct MRT_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> MRT_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MRT_RULE_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Micro-Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UTICK_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<UTICK_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: UTICK_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UTICK_RULE` reader - Micro-Timer"]
pub struct UTICK_RULE_R(crate::FieldReader<u8, UTICK_RULE_A>);
impl UTICK_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        UTICK_RULE_R(crate::FieldReader::new(bits))
    }
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
        **self == UTICK_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == UTICK_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == UTICK_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == UTICK_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for UTICK_RULE_R {
    type Target = crate::FieldReader<u8, UTICK_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTICK_RULE` writer - Micro-Timer"]
pub struct UTICK_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> UTICK_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UTICK_RULE_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
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
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_apb_bridge0_mem_ctrl1](index.html) module"]
pub struct SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC;
impl crate::RegisterSpec for SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_ctrl_apb_bridge0_mem_ctrl1::R](R) reader structure"]
impl crate::Readable for SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_apb_bridge0_mem_ctrl1::W](W) writer structure"]
impl crate::Writable for SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC_CTRL_APB_BRIDGE0_MEM_CTRL1 to value 0"]
impl crate::Resettable for SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
