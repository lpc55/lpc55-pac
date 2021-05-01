#[doc = "Register `SEC_CTRL_AHB_PORT8_SLAVE0_RULE` reader"]
pub struct R(crate::R<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>> for R {
    fn from(reader: crate::R<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CTRL_AHB_PORT8_SLAVE0_RULE` writer"]
pub struct W(crate::W<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>;
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
impl core::convert::From<crate::W<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>> for W {
    fn from(writer: crate::W<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMA Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMA0_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<DMA0_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA0_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DMA0_RULE` reader - DMA Controller"]
pub struct DMA0_RULE_R(crate::FieldReader<u8, DMA0_RULE_A>);
impl DMA0_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA0_RULE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA0_RULE_A {
        match self.bits {
            0 => DMA0_RULE_A::ENUM_NS_NP,
            1 => DMA0_RULE_A::ENUM_NS_P,
            2 => DMA0_RULE_A::ENUM_S_NP,
            3 => DMA0_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == DMA0_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == DMA0_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == DMA0_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == DMA0_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for DMA0_RULE_R {
    type Target = crate::FieldReader<u8, DMA0_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA0_RULE` writer - DMA Controller"]
pub struct DMA0_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA0_RULE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(DMA0_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(DMA0_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(DMA0_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(DMA0_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "USB Full-speed device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FS_USB_DEV_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<FS_USB_DEV_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FS_USB_DEV_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FS_USB_DEV_RULE` reader - USB Full-speed device"]
pub struct FS_USB_DEV_RULE_R(crate::FieldReader<u8, FS_USB_DEV_RULE_A>);
impl FS_USB_DEV_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FS_USB_DEV_RULE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FS_USB_DEV_RULE_A {
        match self.bits {
            0 => FS_USB_DEV_RULE_A::ENUM_NS_NP,
            1 => FS_USB_DEV_RULE_A::ENUM_NS_P,
            2 => FS_USB_DEV_RULE_A::ENUM_S_NP,
            3 => FS_USB_DEV_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == FS_USB_DEV_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == FS_USB_DEV_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == FS_USB_DEV_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == FS_USB_DEV_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for FS_USB_DEV_RULE_R {
    type Target = crate::FieldReader<u8, FS_USB_DEV_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FS_USB_DEV_RULE` writer - USB Full-speed device"]
pub struct FS_USB_DEV_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> FS_USB_DEV_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FS_USB_DEV_RULE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(FS_USB_DEV_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(FS_USB_DEV_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(FS_USB_DEV_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(FS_USB_DEV_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "SCTimer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCT_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<SCT_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: SCT_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SCT_RULE` reader - SCTimer"]
pub struct SCT_RULE_R(crate::FieldReader<u8, SCT_RULE_A>);
impl SCT_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCT_RULE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT_RULE_A {
        match self.bits {
            0 => SCT_RULE_A::ENUM_NS_NP,
            1 => SCT_RULE_A::ENUM_NS_P,
            2 => SCT_RULE_A::ENUM_S_NP,
            3 => SCT_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == SCT_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == SCT_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == SCT_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == SCT_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for SCT_RULE_R {
    type Target = crate::FieldReader<u8, SCT_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCT_RULE` writer - SCTimer"]
pub struct SCT_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCT_RULE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SCT_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SCT_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SCT_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SCT_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Flexcomm interface 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLEXCOMM0_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<FLEXCOMM0_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM0_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLEXCOMM0_RULE` reader - Flexcomm interface 0"]
pub struct FLEXCOMM0_RULE_R(crate::FieldReader<u8, FLEXCOMM0_RULE_A>);
impl FLEXCOMM0_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLEXCOMM0_RULE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM0_RULE_A {
        match self.bits {
            0 => FLEXCOMM0_RULE_A::ENUM_NS_NP,
            1 => FLEXCOMM0_RULE_A::ENUM_NS_P,
            2 => FLEXCOMM0_RULE_A::ENUM_S_NP,
            3 => FLEXCOMM0_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == FLEXCOMM0_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == FLEXCOMM0_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == FLEXCOMM0_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == FLEXCOMM0_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for FLEXCOMM0_RULE_R {
    type Target = crate::FieldReader<u8, FLEXCOMM0_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM0_RULE` writer - Flexcomm interface 0"]
pub struct FLEXCOMM0_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM0_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCOMM0_RULE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(FLEXCOMM0_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(FLEXCOMM0_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(FLEXCOMM0_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(FLEXCOMM0_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Flexcomm interface 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLEXCOMM1_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<FLEXCOMM1_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM1_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLEXCOMM1_RULE` reader - Flexcomm interface 1"]
pub struct FLEXCOMM1_RULE_R(crate::FieldReader<u8, FLEXCOMM1_RULE_A>);
impl FLEXCOMM1_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLEXCOMM1_RULE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM1_RULE_A {
        match self.bits {
            0 => FLEXCOMM1_RULE_A::ENUM_NS_NP,
            1 => FLEXCOMM1_RULE_A::ENUM_NS_P,
            2 => FLEXCOMM1_RULE_A::ENUM_S_NP,
            3 => FLEXCOMM1_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == FLEXCOMM1_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == FLEXCOMM1_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == FLEXCOMM1_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == FLEXCOMM1_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for FLEXCOMM1_RULE_R {
    type Target = crate::FieldReader<u8, FLEXCOMM1_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM1_RULE` writer - Flexcomm interface 1"]
pub struct FLEXCOMM1_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM1_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCOMM1_RULE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(FLEXCOMM1_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(FLEXCOMM1_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(FLEXCOMM1_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(FLEXCOMM1_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9 - DMA Controller"]
    #[inline(always)]
    pub fn dma0_rule(&self) -> DMA0_RULE_R {
        DMA0_RULE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - USB Full-speed device"]
    #[inline(always)]
    pub fn fs_usb_dev_rule(&self) -> FS_USB_DEV_RULE_R {
        FS_USB_DEV_RULE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - SCTimer"]
    #[inline(always)]
    pub fn sct_rule(&self) -> SCT_RULE_R {
        SCT_RULE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Flexcomm interface 0"]
    #[inline(always)]
    pub fn flexcomm0_rule(&self) -> FLEXCOMM0_RULE_R {
        FLEXCOMM0_RULE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Flexcomm interface 1"]
    #[inline(always)]
    pub fn flexcomm1_rule(&self) -> FLEXCOMM1_RULE_R {
        FLEXCOMM1_RULE_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - DMA Controller"]
    #[inline(always)]
    pub fn dma0_rule(&mut self) -> DMA0_RULE_W {
        DMA0_RULE_W { w: self }
    }
    #[doc = "Bits 16:17 - USB Full-speed device"]
    #[inline(always)]
    pub fn fs_usb_dev_rule(&mut self) -> FS_USB_DEV_RULE_W {
        FS_USB_DEV_RULE_W { w: self }
    }
    #[doc = "Bits 20:21 - SCTimer"]
    #[inline(always)]
    pub fn sct_rule(&mut self) -> SCT_RULE_W {
        SCT_RULE_W { w: self }
    }
    #[doc = "Bits 24:25 - Flexcomm interface 0"]
    #[inline(always)]
    pub fn flexcomm0_rule(&mut self) -> FLEXCOMM0_RULE_W {
        FLEXCOMM0_RULE_W { w: self }
    }
    #[doc = "Bits 28:29 - Flexcomm interface 1"]
    #[inline(always)]
    pub fn flexcomm1_rule(&mut self) -> FLEXCOMM1_RULE_W {
        FLEXCOMM1_RULE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security access rules for AHB peripherals.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ahb_port8_slave0_rule](index.html) module"]
pub struct SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC;
impl crate::RegisterSpec for SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_ctrl_ahb_port8_slave0_rule::R](R) reader structure"]
impl crate::Readable for SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ahb_port8_slave0_rule::W](W) writer structure"]
impl crate::Writable for SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC_CTRL_AHB_PORT8_SLAVE0_RULE to value 0"]
impl crate::Resettable for SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
