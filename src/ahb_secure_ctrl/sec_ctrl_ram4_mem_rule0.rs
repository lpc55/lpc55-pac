#[doc = "Register `SEC_CTRL_RAM4_MEM_RULE0` reader"]
pub struct R(crate::R<SEC_CTRL_RAM4_MEM_RULE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CTRL_RAM4_MEM_RULE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_CTRL_RAM4_MEM_RULE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_CTRL_RAM4_MEM_RULE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CTRL_RAM4_MEM_RULE0` writer"]
pub struct W(crate::W<SEC_CTRL_RAM4_MEM_RULE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CTRL_RAM4_MEM_RULE0_SPEC>;
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
impl From<crate::W<SEC_CTRL_RAM4_MEM_RULE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_CTRL_RAM4_MEM_RULE0_SPEC>) -> Self {
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security access rules for RAM4 slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ram4_mem_rule0](index.html) module"]
pub struct SEC_CTRL_RAM4_MEM_RULE0_SPEC;
impl crate::RegisterSpec for SEC_CTRL_RAM4_MEM_RULE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_ctrl_ram4_mem_rule0::R](R) reader structure"]
impl crate::Readable for SEC_CTRL_RAM4_MEM_RULE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ram4_mem_rule0::W](W) writer structure"]
impl crate::Writable for SEC_CTRL_RAM4_MEM_RULE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC_CTRL_RAM4_MEM_RULE0 to value 0"]
impl crate::Resettable for SEC_CTRL_RAM4_MEM_RULE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
