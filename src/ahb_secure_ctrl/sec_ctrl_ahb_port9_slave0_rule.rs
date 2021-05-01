#[doc = "Register `SEC_CTRL_AHB_PORT9_SLAVE0_RULE` reader"]
pub struct R(crate::R<SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC>> for R {
    fn from(reader: crate::R<SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CTRL_AHB_PORT9_SLAVE0_RULE` writer"]
pub struct W(crate::W<SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC>;
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
impl core::convert::From<crate::W<SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC>> for W {
    fn from(writer: crate::W<SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB high Speed device registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USB_HS_DEV_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<USB_HS_DEV_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_HS_DEV_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USB_HS_DEV_RULE` reader - USB high Speed device registers"]
pub struct USB_HS_DEV_RULE_R(crate::FieldReader<u8, USB_HS_DEV_RULE_A>);
impl USB_HS_DEV_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        USB_HS_DEV_RULE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_HS_DEV_RULE_A {
        match self.bits {
            0 => USB_HS_DEV_RULE_A::ENUM_NS_NP,
            1 => USB_HS_DEV_RULE_A::ENUM_NS_P,
            2 => USB_HS_DEV_RULE_A::ENUM_S_NP,
            3 => USB_HS_DEV_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == USB_HS_DEV_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == USB_HS_DEV_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == USB_HS_DEV_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == USB_HS_DEV_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for USB_HS_DEV_RULE_R {
    type Target = crate::FieldReader<u8, USB_HS_DEV_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_HS_DEV_RULE` writer - USB high Speed device registers"]
pub struct USB_HS_DEV_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_HS_DEV_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_HS_DEV_RULE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(USB_HS_DEV_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(USB_HS_DEV_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(USB_HS_DEV_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(USB_HS_DEV_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "CRC engine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRC_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<CRC_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: CRC_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CRC_RULE` reader - CRC engine"]
pub struct CRC_RULE_R(crate::FieldReader<u8, CRC_RULE_A>);
impl CRC_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRC_RULE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_RULE_A {
        match self.bits {
            0 => CRC_RULE_A::ENUM_NS_NP,
            1 => CRC_RULE_A::ENUM_NS_P,
            2 => CRC_RULE_A::ENUM_S_NP,
            3 => CRC_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == CRC_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == CRC_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == CRC_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == CRC_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for CRC_RULE_R {
    type Target = crate::FieldReader<u8, CRC_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_RULE` writer - CRC engine"]
pub struct CRC_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_RULE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(CRC_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(CRC_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(CRC_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(CRC_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Flexcomm interface 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLEXCOMM5_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<FLEXCOMM5_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM5_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLEXCOMM5_RULE` reader - Flexcomm interface 5"]
pub struct FLEXCOMM5_RULE_R(crate::FieldReader<u8, FLEXCOMM5_RULE_A>);
impl FLEXCOMM5_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLEXCOMM5_RULE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM5_RULE_A {
        match self.bits {
            0 => FLEXCOMM5_RULE_A::ENUM_NS_NP,
            1 => FLEXCOMM5_RULE_A::ENUM_NS_P,
            2 => FLEXCOMM5_RULE_A::ENUM_S_NP,
            3 => FLEXCOMM5_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == FLEXCOMM5_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == FLEXCOMM5_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == FLEXCOMM5_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == FLEXCOMM5_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for FLEXCOMM5_RULE_R {
    type Target = crate::FieldReader<u8, FLEXCOMM5_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM5_RULE` writer - Flexcomm interface 5"]
pub struct FLEXCOMM5_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM5_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCOMM5_RULE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(FLEXCOMM5_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(FLEXCOMM5_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(FLEXCOMM5_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(FLEXCOMM5_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Flexcomm interface 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLEXCOMM6_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<FLEXCOMM6_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM6_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLEXCOMM6_RULE` reader - Flexcomm interface 6"]
pub struct FLEXCOMM6_RULE_R(crate::FieldReader<u8, FLEXCOMM6_RULE_A>);
impl FLEXCOMM6_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLEXCOMM6_RULE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM6_RULE_A {
        match self.bits {
            0 => FLEXCOMM6_RULE_A::ENUM_NS_NP,
            1 => FLEXCOMM6_RULE_A::ENUM_NS_P,
            2 => FLEXCOMM6_RULE_A::ENUM_S_NP,
            3 => FLEXCOMM6_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == FLEXCOMM6_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == FLEXCOMM6_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == FLEXCOMM6_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == FLEXCOMM6_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for FLEXCOMM6_RULE_R {
    type Target = crate::FieldReader<u8, FLEXCOMM6_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM6_RULE` writer - Flexcomm interface 6"]
pub struct FLEXCOMM6_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM6_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCOMM6_RULE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(FLEXCOMM6_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(FLEXCOMM6_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(FLEXCOMM6_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(FLEXCOMM6_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17 - USB high Speed device registers"]
    #[inline(always)]
    pub fn usb_hs_dev_rule(&self) -> USB_HS_DEV_RULE_R {
        USB_HS_DEV_RULE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - CRC engine"]
    #[inline(always)]
    pub fn crc_rule(&self) -> CRC_RULE_R {
        CRC_RULE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Flexcomm interface 5"]
    #[inline(always)]
    pub fn flexcomm5_rule(&self) -> FLEXCOMM5_RULE_R {
        FLEXCOMM5_RULE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Flexcomm interface 6"]
    #[inline(always)]
    pub fn flexcomm6_rule(&self) -> FLEXCOMM6_RULE_R {
        FLEXCOMM6_RULE_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - USB high Speed device registers"]
    #[inline(always)]
    pub fn usb_hs_dev_rule(&mut self) -> USB_HS_DEV_RULE_W {
        USB_HS_DEV_RULE_W { w: self }
    }
    #[doc = "Bits 20:21 - CRC engine"]
    #[inline(always)]
    pub fn crc_rule(&mut self) -> CRC_RULE_W {
        CRC_RULE_W { w: self }
    }
    #[doc = "Bits 24:25 - Flexcomm interface 5"]
    #[inline(always)]
    pub fn flexcomm5_rule(&mut self) -> FLEXCOMM5_RULE_W {
        FLEXCOMM5_RULE_W { w: self }
    }
    #[doc = "Bits 28:29 - Flexcomm interface 6"]
    #[inline(always)]
    pub fn flexcomm6_rule(&mut self) -> FLEXCOMM6_RULE_W {
        FLEXCOMM6_RULE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security access rules for AHB peripherals.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ahb_port9_slave0_rule](index.html) module"]
pub struct SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC;
impl crate::RegisterSpec for SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_ctrl_ahb_port9_slave0_rule::R](R) reader structure"]
impl crate::Readable for SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ahb_port9_slave0_rule::W](W) writer structure"]
impl crate::Writable for SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC_CTRL_AHB_PORT9_SLAVE0_RULE to value 0"]
impl crate::Resettable for SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
