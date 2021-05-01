#[doc = "Register `SEC_CTRL_AHB_PORT9_SLAVE1_RULE` reader"]
pub struct R(crate::R<SEC_CTRL_AHB_PORT9_SLAVE1_RULE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CTRL_AHB_PORT9_SLAVE1_RULE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SEC_CTRL_AHB_PORT9_SLAVE1_RULE_SPEC>> for R {
    fn from(reader: crate::R<SEC_CTRL_AHB_PORT9_SLAVE1_RULE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CTRL_AHB_PORT9_SLAVE1_RULE` writer"]
pub struct W(crate::W<SEC_CTRL_AHB_PORT9_SLAVE1_RULE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CTRL_AHB_PORT9_SLAVE1_RULE_SPEC>;
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
impl core::convert::From<crate::W<SEC_CTRL_AHB_PORT9_SLAVE1_RULE_SPEC>> for W {
    fn from(writer: crate::W<SEC_CTRL_AHB_PORT9_SLAVE1_RULE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Flexcomm interface 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLEXCOMM7_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<FLEXCOMM7_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM7_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLEXCOMM7_RULE` reader - Flexcomm interface 7"]
pub struct FLEXCOMM7_RULE_R(crate::FieldReader<u8, FLEXCOMM7_RULE_A>);
impl FLEXCOMM7_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLEXCOMM7_RULE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM7_RULE_A {
        match self.bits {
            0 => FLEXCOMM7_RULE_A::ENUM_NS_NP,
            1 => FLEXCOMM7_RULE_A::ENUM_NS_P,
            2 => FLEXCOMM7_RULE_A::ENUM_S_NP,
            3 => FLEXCOMM7_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == FLEXCOMM7_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == FLEXCOMM7_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == FLEXCOMM7_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == FLEXCOMM7_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for FLEXCOMM7_RULE_R {
    type Target = crate::FieldReader<u8, FLEXCOMM7_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM7_RULE` writer - Flexcomm interface 7"]
pub struct FLEXCOMM7_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM7_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCOMM7_RULE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(FLEXCOMM7_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(FLEXCOMM7_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(FLEXCOMM7_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(FLEXCOMM7_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "SDMMC card interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDIO_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<SDIO_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: SDIO_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SDIO_RULE` reader - SDMMC card interface"]
pub struct SDIO_RULE_R(crate::FieldReader<u8, SDIO_RULE_A>);
impl SDIO_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDIO_RULE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIO_RULE_A {
        match self.bits {
            0 => SDIO_RULE_A::ENUM_NS_NP,
            1 => SDIO_RULE_A::ENUM_NS_P,
            2 => SDIO_RULE_A::ENUM_S_NP,
            3 => SDIO_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == SDIO_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == SDIO_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == SDIO_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == SDIO_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for SDIO_RULE_R {
    type Target = crate::FieldReader<u8, SDIO_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_RULE` writer - SDMMC card interface"]
pub struct SDIO_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIO_RULE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SDIO_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SDIO_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SDIO_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SDIO_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Debug mailbox (aka ISP-AP)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DBG_MAILBOX_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<DBG_MAILBOX_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: DBG_MAILBOX_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DBG_MAILBOX_RULE` reader - Debug mailbox (aka ISP-AP)"]
pub struct DBG_MAILBOX_RULE_R(crate::FieldReader<u8, DBG_MAILBOX_RULE_A>);
impl DBG_MAILBOX_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DBG_MAILBOX_RULE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_MAILBOX_RULE_A {
        match self.bits {
            0 => DBG_MAILBOX_RULE_A::ENUM_NS_NP,
            1 => DBG_MAILBOX_RULE_A::ENUM_NS_P,
            2 => DBG_MAILBOX_RULE_A::ENUM_S_NP,
            3 => DBG_MAILBOX_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == DBG_MAILBOX_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == DBG_MAILBOX_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == DBG_MAILBOX_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == DBG_MAILBOX_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for DBG_MAILBOX_RULE_R {
    type Target = crate::FieldReader<u8, DBG_MAILBOX_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_MAILBOX_RULE` writer - Debug mailbox (aka ISP-AP)"]
pub struct DBG_MAILBOX_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_MAILBOX_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_MAILBOX_RULE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(DBG_MAILBOX_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(DBG_MAILBOX_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(DBG_MAILBOX_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(DBG_MAILBOX_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "High Speed SPI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HS_LSPI_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<HS_LSPI_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: HS_LSPI_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HS_LSPI_RULE` reader - High Speed SPI"]
pub struct HS_LSPI_RULE_R(crate::FieldReader<u8, HS_LSPI_RULE_A>);
impl HS_LSPI_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        HS_LSPI_RULE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_LSPI_RULE_A {
        match self.bits {
            0 => HS_LSPI_RULE_A::ENUM_NS_NP,
            1 => HS_LSPI_RULE_A::ENUM_NS_P,
            2 => HS_LSPI_RULE_A::ENUM_S_NP,
            3 => HS_LSPI_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == HS_LSPI_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == HS_LSPI_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == HS_LSPI_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == HS_LSPI_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for HS_LSPI_RULE_R {
    type Target = crate::FieldReader<u8, HS_LSPI_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HS_LSPI_RULE` writer - High Speed SPI"]
pub struct HS_LSPI_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_LSPI_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS_LSPI_RULE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(HS_LSPI_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(HS_LSPI_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(HS_LSPI_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(HS_LSPI_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Flexcomm interface 7"]
    #[inline(always)]
    pub fn flexcomm7_rule(&self) -> FLEXCOMM7_RULE_R {
        FLEXCOMM7_RULE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - SDMMC card interface"]
    #[inline(always)]
    pub fn sdio_rule(&self) -> SDIO_RULE_R {
        SDIO_RULE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Debug mailbox (aka ISP-AP)"]
    #[inline(always)]
    pub fn dbg_mailbox_rule(&self) -> DBG_MAILBOX_RULE_R {
        DBG_MAILBOX_RULE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - High Speed SPI"]
    #[inline(always)]
    pub fn hs_lspi_rule(&self) -> HS_LSPI_RULE_R {
        HS_LSPI_RULE_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Flexcomm interface 7"]
    #[inline(always)]
    pub fn flexcomm7_rule(&mut self) -> FLEXCOMM7_RULE_W {
        FLEXCOMM7_RULE_W { w: self }
    }
    #[doc = "Bits 12:13 - SDMMC card interface"]
    #[inline(always)]
    pub fn sdio_rule(&mut self) -> SDIO_RULE_W {
        SDIO_RULE_W { w: self }
    }
    #[doc = "Bits 16:17 - Debug mailbox (aka ISP-AP)"]
    #[inline(always)]
    pub fn dbg_mailbox_rule(&mut self) -> DBG_MAILBOX_RULE_W {
        DBG_MAILBOX_RULE_W { w: self }
    }
    #[doc = "Bits 28:29 - High Speed SPI"]
    #[inline(always)]
    pub fn hs_lspi_rule(&mut self) -> HS_LSPI_RULE_W {
        HS_LSPI_RULE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security access rules for AHB peripherals.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ahb_port9_slave1_rule](index.html) module"]
pub struct SEC_CTRL_AHB_PORT9_SLAVE1_RULE_SPEC;
impl crate::RegisterSpec for SEC_CTRL_AHB_PORT9_SLAVE1_RULE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_ctrl_ahb_port9_slave1_rule::R](R) reader structure"]
impl crate::Readable for SEC_CTRL_AHB_PORT9_SLAVE1_RULE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ahb_port9_slave1_rule::W](W) writer structure"]
impl crate::Writable for SEC_CTRL_AHB_PORT9_SLAVE1_RULE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC_CTRL_AHB_PORT9_SLAVE1_RULE to value 0"]
impl crate::Resettable for SEC_CTRL_AHB_PORT9_SLAVE1_RULE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
