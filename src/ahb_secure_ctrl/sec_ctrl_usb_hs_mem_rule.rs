#[doc = "Register `SEC_CTRL_USB_HS_MEM_RULE` reader"]
pub struct R(crate::R<SEC_CTRL_USB_HS_MEM_RULE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CTRL_USB_HS_MEM_RULE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_CTRL_USB_HS_MEM_RULE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_CTRL_USB_HS_MEM_RULE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CTRL_USB_HS_MEM_RULE` writer"]
pub struct W(crate::W<SEC_CTRL_USB_HS_MEM_RULE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CTRL_USB_HS_MEM_RULE_SPEC>;
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
impl From<crate::W<SEC_CTRL_USB_HS_MEM_RULE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_CTRL_USB_HS_MEM_RULE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Address space: 0x4010_0000 - 0x4010_0FFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRAM_SECT_0_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<SRAM_SECT_0_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAM_SECT_0_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRAM_SECT_0_RULE` reader - Address space: 0x4010_0000 - 0x4010_0FFF"]
pub struct SRAM_SECT_0_RULE_R(crate::FieldReader<u8, SRAM_SECT_0_RULE_A>);
impl SRAM_SECT_0_RULE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAM_SECT_0_RULE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_SECT_0_RULE_A {
        match self.bits {
            0 => SRAM_SECT_0_RULE_A::ENUM_NS_NP,
            1 => SRAM_SECT_0_RULE_A::ENUM_NS_P,
            2 => SRAM_SECT_0_RULE_A::ENUM_S_NP,
            3 => SRAM_SECT_0_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == SRAM_SECT_0_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == SRAM_SECT_0_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == SRAM_SECT_0_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == SRAM_SECT_0_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for SRAM_SECT_0_RULE_R {
    type Target = crate::FieldReader<u8, SRAM_SECT_0_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_SECT_0_RULE` writer - Address space: 0x4010_0000 - 0x4010_0FFF"]
pub struct SRAM_SECT_0_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_SECT_0_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_SECT_0_RULE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SRAM_SECT_0_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SRAM_SECT_0_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SRAM_SECT_0_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SRAM_SECT_0_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Address space: 0x4010_1000 - 0x4010_1FFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRAM_SECT_1_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<SRAM_SECT_1_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAM_SECT_1_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRAM_SECT_1_RULE` reader - Address space: 0x4010_1000 - 0x4010_1FFF"]
pub struct SRAM_SECT_1_RULE_R(crate::FieldReader<u8, SRAM_SECT_1_RULE_A>);
impl SRAM_SECT_1_RULE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAM_SECT_1_RULE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_SECT_1_RULE_A {
        match self.bits {
            0 => SRAM_SECT_1_RULE_A::ENUM_NS_NP,
            1 => SRAM_SECT_1_RULE_A::ENUM_NS_P,
            2 => SRAM_SECT_1_RULE_A::ENUM_S_NP,
            3 => SRAM_SECT_1_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == SRAM_SECT_1_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == SRAM_SECT_1_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == SRAM_SECT_1_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == SRAM_SECT_1_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for SRAM_SECT_1_RULE_R {
    type Target = crate::FieldReader<u8, SRAM_SECT_1_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_SECT_1_RULE` writer - Address space: 0x4010_1000 - 0x4010_1FFF"]
pub struct SRAM_SECT_1_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_SECT_1_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_SECT_1_RULE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SRAM_SECT_1_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SRAM_SECT_1_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SRAM_SECT_1_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SRAM_SECT_1_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Address space: 0x4010_2000 - 0x4010_2FFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRAM_SECT_2_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<SRAM_SECT_2_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAM_SECT_2_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRAM_SECT_2_RULE` reader - Address space: 0x4010_2000 - 0x4010_2FFF"]
pub struct SRAM_SECT_2_RULE_R(crate::FieldReader<u8, SRAM_SECT_2_RULE_A>);
impl SRAM_SECT_2_RULE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAM_SECT_2_RULE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_SECT_2_RULE_A {
        match self.bits {
            0 => SRAM_SECT_2_RULE_A::ENUM_NS_NP,
            1 => SRAM_SECT_2_RULE_A::ENUM_NS_P,
            2 => SRAM_SECT_2_RULE_A::ENUM_S_NP,
            3 => SRAM_SECT_2_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == SRAM_SECT_2_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == SRAM_SECT_2_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == SRAM_SECT_2_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == SRAM_SECT_2_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for SRAM_SECT_2_RULE_R {
    type Target = crate::FieldReader<u8, SRAM_SECT_2_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_SECT_2_RULE` writer - Address space: 0x4010_2000 - 0x4010_2FFF"]
pub struct SRAM_SECT_2_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_SECT_2_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_SECT_2_RULE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SRAM_SECT_2_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SRAM_SECT_2_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SRAM_SECT_2_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SRAM_SECT_2_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Address space: 0x4010_3000 - 0x4010_3FFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRAM_SECT_3_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<SRAM_SECT_3_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAM_SECT_3_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRAM_SECT_3_RULE` reader - Address space: 0x4010_3000 - 0x4010_3FFF"]
pub struct SRAM_SECT_3_RULE_R(crate::FieldReader<u8, SRAM_SECT_3_RULE_A>);
impl SRAM_SECT_3_RULE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAM_SECT_3_RULE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_SECT_3_RULE_A {
        match self.bits {
            0 => SRAM_SECT_3_RULE_A::ENUM_NS_NP,
            1 => SRAM_SECT_3_RULE_A::ENUM_NS_P,
            2 => SRAM_SECT_3_RULE_A::ENUM_S_NP,
            3 => SRAM_SECT_3_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == SRAM_SECT_3_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == SRAM_SECT_3_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == SRAM_SECT_3_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == SRAM_SECT_3_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for SRAM_SECT_3_RULE_R {
    type Target = crate::FieldReader<u8, SRAM_SECT_3_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_SECT_3_RULE` writer - Address space: 0x4010_3000 - 0x4010_3FFF"]
pub struct SRAM_SECT_3_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_SECT_3_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_SECT_3_RULE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SRAM_SECT_3_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SRAM_SECT_3_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SRAM_SECT_3_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SRAM_SECT_3_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Address space: 0x4010_0000 - 0x4010_0FFF"]
    #[inline(always)]
    pub fn sram_sect_0_rule(&self) -> SRAM_SECT_0_RULE_R {
        SRAM_SECT_0_RULE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Address space: 0x4010_1000 - 0x4010_1FFF"]
    #[inline(always)]
    pub fn sram_sect_1_rule(&self) -> SRAM_SECT_1_RULE_R {
        SRAM_SECT_1_RULE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Address space: 0x4010_2000 - 0x4010_2FFF"]
    #[inline(always)]
    pub fn sram_sect_2_rule(&self) -> SRAM_SECT_2_RULE_R {
        SRAM_SECT_2_RULE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Address space: 0x4010_3000 - 0x4010_3FFF"]
    #[inline(always)]
    pub fn sram_sect_3_rule(&self) -> SRAM_SECT_3_RULE_R {
        SRAM_SECT_3_RULE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Address space: 0x4010_0000 - 0x4010_0FFF"]
    #[inline(always)]
    pub fn sram_sect_0_rule(&mut self) -> SRAM_SECT_0_RULE_W {
        SRAM_SECT_0_RULE_W { w: self }
    }
    #[doc = "Bits 4:5 - Address space: 0x4010_1000 - 0x4010_1FFF"]
    #[inline(always)]
    pub fn sram_sect_1_rule(&mut self) -> SRAM_SECT_1_RULE_W {
        SRAM_SECT_1_RULE_W { w: self }
    }
    #[doc = "Bits 8:9 - Address space: 0x4010_2000 - 0x4010_2FFF"]
    #[inline(always)]
    pub fn sram_sect_2_rule(&mut self) -> SRAM_SECT_2_RULE_W {
        SRAM_SECT_2_RULE_W { w: self }
    }
    #[doc = "Bits 12:13 - Address space: 0x4010_3000 - 0x4010_3FFF"]
    #[inline(always)]
    pub fn sram_sect_3_rule(&mut self) -> SRAM_SECT_3_RULE_W {
        SRAM_SECT_3_RULE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security access rules for RAM_USB_HS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_usb_hs_mem_rule](index.html) module"]
pub struct SEC_CTRL_USB_HS_MEM_RULE_SPEC;
impl crate::RegisterSpec for SEC_CTRL_USB_HS_MEM_RULE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_ctrl_usb_hs_mem_rule::R](R) reader structure"]
impl crate::Readable for SEC_CTRL_USB_HS_MEM_RULE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_usb_hs_mem_rule::W](W) writer structure"]
impl crate::Writable for SEC_CTRL_USB_HS_MEM_RULE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC_CTRL_USB_HS_MEM_RULE to value 0"]
impl crate::Resettable for SEC_CTRL_USB_HS_MEM_RULE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
