#[doc = "Register `SEC_CTRL_APB_BRIDGE0_MEM_CTRL0` reader"]
pub struct R(crate::R<SEC_CTRL_APB_BRIDGE0_MEM_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CTRL_APB_BRIDGE0_MEM_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SEC_CTRL_APB_BRIDGE0_MEM_CTRL0_SPEC>> for R {
    fn from(reader: crate::R<SEC_CTRL_APB_BRIDGE0_MEM_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CTRL_APB_BRIDGE0_MEM_CTRL0` writer"]
pub struct W(crate::W<SEC_CTRL_APB_BRIDGE0_MEM_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CTRL_APB_BRIDGE0_MEM_CTRL0_SPEC>;
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
impl core::convert::From<crate::W<SEC_CTRL_APB_BRIDGE0_MEM_CTRL0_SPEC>> for W {
    fn from(writer: crate::W<SEC_CTRL_APB_BRIDGE0_MEM_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "System Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCON_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<SYSCON_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCON_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCON_RULE` reader - System Configuration"]
pub struct SYSCON_RULE_R(crate::FieldReader<u8, SYSCON_RULE_A>);
impl SYSCON_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYSCON_RULE_R(crate::FieldReader::new(bits))
    }
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
        **self == SYSCON_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == SYSCON_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == SYSCON_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == SYSCON_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for SYSCON_RULE_R {
    type Target = crate::FieldReader<u8, SYSCON_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCON_RULE` writer - System Configuration"]
pub struct SYSCON_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCON_RULE_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "I/O Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IOCON_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<IOCON_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: IOCON_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IOCON_RULE` reader - I/O Configuration"]
pub struct IOCON_RULE_R(crate::FieldReader<u8, IOCON_RULE_A>);
impl IOCON_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        IOCON_RULE_R(crate::FieldReader::new(bits))
    }
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
        **self == IOCON_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == IOCON_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == IOCON_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == IOCON_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for IOCON_RULE_R {
    type Target = crate::FieldReader<u8, IOCON_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOCON_RULE` writer - I/O Configuration"]
pub struct IOCON_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCON_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOCON_RULE_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "GPIO input Interrupt 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GINT0_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<GINT0_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: GINT0_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GINT0_RULE` reader - GPIO input Interrupt 0"]
pub struct GINT0_RULE_R(crate::FieldReader<u8, GINT0_RULE_A>);
impl GINT0_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        GINT0_RULE_R(crate::FieldReader::new(bits))
    }
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
        **self == GINT0_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == GINT0_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == GINT0_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == GINT0_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for GINT0_RULE_R {
    type Target = crate::FieldReader<u8, GINT0_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GINT0_RULE` writer - GPIO input Interrupt 0"]
pub struct GINT0_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> GINT0_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GINT0_RULE_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "GPIO input Interrupt 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GINT1_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<GINT1_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: GINT1_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GINT1_RULE` reader - GPIO input Interrupt 1"]
pub struct GINT1_RULE_R(crate::FieldReader<u8, GINT1_RULE_A>);
impl GINT1_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        GINT1_RULE_R(crate::FieldReader::new(bits))
    }
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
        **self == GINT1_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == GINT1_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == GINT1_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == GINT1_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for GINT1_RULE_R {
    type Target = crate::FieldReader<u8, GINT1_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GINT1_RULE` writer - GPIO input Interrupt 1"]
pub struct GINT1_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> GINT1_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GINT1_RULE_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Pin Interrupt and Pattern match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PINT_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<PINT_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: PINT_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PINT_RULE` reader - Pin Interrupt and Pattern match"]
pub struct PINT_RULE_R(crate::FieldReader<u8, PINT_RULE_A>);
impl PINT_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PINT_RULE_R(crate::FieldReader::new(bits))
    }
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
        **self == PINT_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == PINT_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == PINT_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == PINT_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for PINT_RULE_R {
    type Target = crate::FieldReader<u8, PINT_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINT_RULE` writer - Pin Interrupt and Pattern match"]
pub struct PINT_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> PINT_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT_RULE_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Secure Pin Interrupt and Pattern match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEC_PINT_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<SEC_PINT_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_PINT_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEC_PINT_RULE` reader - Secure Pin Interrupt and Pattern match"]
pub struct SEC_PINT_RULE_R(crate::FieldReader<u8, SEC_PINT_RULE_A>);
impl SEC_PINT_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEC_PINT_RULE_R(crate::FieldReader::new(bits))
    }
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
        **self == SEC_PINT_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == SEC_PINT_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == SEC_PINT_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == SEC_PINT_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for SEC_PINT_RULE_R {
    type Target = crate::FieldReader<u8, SEC_PINT_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC_PINT_RULE` writer - Secure Pin Interrupt and Pattern match"]
pub struct SEC_PINT_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_PINT_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEC_PINT_RULE_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Peripheral input multiplexing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUTMUX_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<INPUTMUX_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUTMUX_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPUTMUX_RULE` reader - Peripheral input multiplexing"]
pub struct INPUTMUX_RULE_R(crate::FieldReader<u8, INPUTMUX_RULE_A>);
impl INPUTMUX_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        INPUTMUX_RULE_R(crate::FieldReader::new(bits))
    }
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
        **self == INPUTMUX_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == INPUTMUX_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == INPUTMUX_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == INPUTMUX_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for INPUTMUX_RULE_R {
    type Target = crate::FieldReader<u8, INPUTMUX_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INPUTMUX_RULE` writer - Peripheral input multiplexing"]
pub struct INPUTMUX_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUTMUX_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUTMUX_RULE_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
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
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_apb_bridge0_mem_ctrl0](index.html) module"]
pub struct SEC_CTRL_APB_BRIDGE0_MEM_CTRL0_SPEC;
impl crate::RegisterSpec for SEC_CTRL_APB_BRIDGE0_MEM_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_ctrl_apb_bridge0_mem_ctrl0::R](R) reader structure"]
impl crate::Readable for SEC_CTRL_APB_BRIDGE0_MEM_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_apb_bridge0_mem_ctrl0::W](W) writer structure"]
impl crate::Writable for SEC_CTRL_APB_BRIDGE0_MEM_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC_CTRL_APB_BRIDGE0_MEM_CTRL0 to value 0"]
impl crate::Resettable for SEC_CTRL_APB_BRIDGE0_MEM_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
