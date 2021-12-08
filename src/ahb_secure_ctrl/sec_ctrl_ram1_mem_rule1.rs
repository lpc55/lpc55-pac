#[doc = "Register `SEC_CTRL_RAM1_MEM_RULE1` reader"]
pub struct R(crate::R<SEC_CTRL_RAM1_MEM_RULE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CTRL_RAM1_MEM_RULE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_CTRL_RAM1_MEM_RULE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_CTRL_RAM1_MEM_RULE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CTRL_RAM1_MEM_RULE1` writer"]
pub struct W(crate::W<SEC_CTRL_RAM1_MEM_RULE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CTRL_RAM1_MEM_RULE1_SPEC>;
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
impl From<crate::W<SEC_CTRL_RAM1_MEM_RULE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_CTRL_RAM1_MEM_RULE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RULE0_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RULE0_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RULE0` reader - secure control rule0. it can be set when check_reg's write_lock is '0'"]
pub struct RULE0_R(crate::FieldReader<u8, RULE0_A>);
impl RULE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RULE0_R(crate::FieldReader::new(bits))
    }
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
        **self == RULE0_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == RULE0_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == RULE0_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == RULE0_A::ENUM_S_P
    }
}
impl core::ops::Deref for RULE0_R {
    type Target = crate::FieldReader<u8, RULE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RULE0` writer - secure control rule0. it can be set when check_reg's write_lock is '0'"]
pub struct RULE0_W<'a> {
    w: &'a mut W,
}
impl<'a> RULE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RULE0_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RULE1_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RULE1_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RULE1` reader - secure control rule1. it can be set when check_reg's write_lock is '0'"]
pub struct RULE1_R(crate::FieldReader<u8, RULE1_A>);
impl RULE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RULE1_R(crate::FieldReader::new(bits))
    }
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
        **self == RULE1_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == RULE1_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == RULE1_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == RULE1_A::ENUM_S_P
    }
}
impl core::ops::Deref for RULE1_R {
    type Target = crate::FieldReader<u8, RULE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RULE1` writer - secure control rule1. it can be set when check_reg's write_lock is '0'"]
pub struct RULE1_W<'a> {
    w: &'a mut W,
}
impl<'a> RULE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RULE1_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RULE2_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RULE2_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RULE2` reader - secure control rule2. it can be set when check_reg's write_lock is '0'"]
pub struct RULE2_R(crate::FieldReader<u8, RULE2_A>);
impl RULE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RULE2_R(crate::FieldReader::new(bits))
    }
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
        **self == RULE2_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == RULE2_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == RULE2_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == RULE2_A::ENUM_S_P
    }
}
impl core::ops::Deref for RULE2_R {
    type Target = crate::FieldReader<u8, RULE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RULE2` writer - secure control rule2. it can be set when check_reg's write_lock is '0'"]
pub struct RULE2_W<'a> {
    w: &'a mut W,
}
impl<'a> RULE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RULE2_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RULE3_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RULE3_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RULE3` reader - secure control rule3. it can be set when check_reg's write_lock is '0'"]
pub struct RULE3_R(crate::FieldReader<u8, RULE3_A>);
impl RULE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RULE3_R(crate::FieldReader::new(bits))
    }
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
        **self == RULE3_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == RULE3_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == RULE3_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == RULE3_A::ENUM_S_P
    }
}
impl core::ops::Deref for RULE3_R {
    type Target = crate::FieldReader<u8, RULE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RULE3` writer - secure control rule3. it can be set when check_reg's write_lock is '0'"]
pub struct RULE3_W<'a> {
    w: &'a mut W,
}
impl<'a> RULE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RULE3_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RULE4_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RULE4_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RULE4` reader - secure control rule4. it can be set when check_reg's write_lock is '0'"]
pub struct RULE4_R(crate::FieldReader<u8, RULE4_A>);
impl RULE4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RULE4_R(crate::FieldReader::new(bits))
    }
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
        **self == RULE4_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == RULE4_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == RULE4_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == RULE4_A::ENUM_S_P
    }
}
impl core::ops::Deref for RULE4_R {
    type Target = crate::FieldReader<u8, RULE4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RULE4` writer - secure control rule4. it can be set when check_reg's write_lock is '0'"]
pub struct RULE4_W<'a> {
    w: &'a mut W,
}
impl<'a> RULE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RULE4_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RULE5_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RULE5_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RULE5` reader - secure control rule5. it can be set when check_reg's write_lock is '0'"]
pub struct RULE5_R(crate::FieldReader<u8, RULE5_A>);
impl RULE5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RULE5_R(crate::FieldReader::new(bits))
    }
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
        **self == RULE5_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == RULE5_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == RULE5_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == RULE5_A::ENUM_S_P
    }
}
impl core::ops::Deref for RULE5_R {
    type Target = crate::FieldReader<u8, RULE5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RULE5` writer - secure control rule5. it can be set when check_reg's write_lock is '0'"]
pub struct RULE5_W<'a> {
    w: &'a mut W,
}
impl<'a> RULE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RULE5_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RULE6_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RULE6_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RULE6` reader - secure control rule6. it can be set when check_reg's write_lock is '0'"]
pub struct RULE6_R(crate::FieldReader<u8, RULE6_A>);
impl RULE6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RULE6_R(crate::FieldReader::new(bits))
    }
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
        **self == RULE6_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == RULE6_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == RULE6_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == RULE6_A::ENUM_S_P
    }
}
impl core::ops::Deref for RULE6_R {
    type Target = crate::FieldReader<u8, RULE6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RULE6` writer - secure control rule6. it can be set when check_reg's write_lock is '0'"]
pub struct RULE6_W<'a> {
    w: &'a mut W,
}
impl<'a> RULE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RULE6_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RULE7_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RULE7_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RULE7` reader - secure control rule7. it can be set when check_reg's write_lock is '0'"]
pub struct RULE7_R(crate::FieldReader<u8, RULE7_A>);
impl RULE7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RULE7_R(crate::FieldReader::new(bits))
    }
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
        **self == RULE7_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == RULE7_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == RULE7_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == RULE7_A::ENUM_S_P
    }
}
impl core::ops::Deref for RULE7_R {
    type Target = crate::FieldReader<u8, RULE7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RULE7` writer - secure control rule7. it can be set when check_reg's write_lock is '0'"]
pub struct RULE7_W<'a> {
    w: &'a mut W,
}
impl<'a> RULE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RULE7_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security access rules for RAM1 slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ram1_mem_rule1](index.html) module"]
pub struct SEC_CTRL_RAM1_MEM_RULE1_SPEC;
impl crate::RegisterSpec for SEC_CTRL_RAM1_MEM_RULE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_ctrl_ram1_mem_rule1::R](R) reader structure"]
impl crate::Readable for SEC_CTRL_RAM1_MEM_RULE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ram1_mem_rule1::W](W) writer structure"]
impl crate::Writable for SEC_CTRL_RAM1_MEM_RULE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC_CTRL_RAM1_MEM_RULE1 to value 0"]
impl crate::Resettable for SEC_CTRL_RAM1_MEM_RULE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
