#[doc = "Reader of register SEC_CTRL_AHB1_1_SLAVE_RULE"]
pub type R = crate::R<u32, super::SEC_CTRL_AHB1_1_SLAVE_RULE>;
#[doc = "Writer for register SEC_CTRL_AHB1_1_SLAVE_RULE"]
pub type W = crate::W<u32, super::SEC_CTRL_AHB1_1_SLAVE_RULE>;
#[doc = "Register SEC_CTRL_AHB1_1_SLAVE_RULE `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC_CTRL_AHB1_1_SLAVE_RULE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `FLEXCOMM7_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCOMM7_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<FLEXCOMM7_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM7_RULE_A) -> Self {
        match variant {
            FLEXCOMM7_RULE_A::ENUM_NS_NP => 0,
            FLEXCOMM7_RULE_A::ENUM_NS_P => 1,
            FLEXCOMM7_RULE_A::ENUM_S_NP => 2,
            FLEXCOMM7_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `FLEXCOMM7_RULE`"]
pub type FLEXCOMM7_RULE_R = crate::R<u8, FLEXCOMM7_RULE_A>;
impl FLEXCOMM7_RULE_R {
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
        *self == FLEXCOMM7_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == FLEXCOMM7_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == FLEXCOMM7_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == FLEXCOMM7_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `FLEXCOMM7_RULE`"]
pub struct FLEXCOMM7_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM7_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCOMM7_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `SDIO_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIO_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<SDIO_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: SDIO_RULE_A) -> Self {
        match variant {
            SDIO_RULE_A::ENUM_NS_NP => 0,
            SDIO_RULE_A::ENUM_NS_P => 1,
            SDIO_RULE_A::ENUM_S_NP => 2,
            SDIO_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `SDIO_RULE`"]
pub type SDIO_RULE_R = crate::R<u8, SDIO_RULE_A>;
impl SDIO_RULE_R {
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
        *self == SDIO_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SDIO_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SDIO_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SDIO_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `SDIO_RULE`"]
pub struct SDIO_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIO_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `DBG_MAILBOX_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_MAILBOX_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<DBG_MAILBOX_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: DBG_MAILBOX_RULE_A) -> Self {
        match variant {
            DBG_MAILBOX_RULE_A::ENUM_NS_NP => 0,
            DBG_MAILBOX_RULE_A::ENUM_NS_P => 1,
            DBG_MAILBOX_RULE_A::ENUM_S_NP => 2,
            DBG_MAILBOX_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `DBG_MAILBOX_RULE`"]
pub type DBG_MAILBOX_RULE_R = crate::R<u8, DBG_MAILBOX_RULE_A>;
impl DBG_MAILBOX_RULE_R {
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
        *self == DBG_MAILBOX_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == DBG_MAILBOX_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == DBG_MAILBOX_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == DBG_MAILBOX_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `DBG_MAILBOX_RULE`"]
pub struct DBG_MAILBOX_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_MAILBOX_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_MAILBOX_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `HS_LSPI_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_LSPI_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<HS_LSPI_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: HS_LSPI_RULE_A) -> Self {
        match variant {
            HS_LSPI_RULE_A::ENUM_NS_NP => 0,
            HS_LSPI_RULE_A::ENUM_NS_P => 1,
            HS_LSPI_RULE_A::ENUM_S_NP => 2,
            HS_LSPI_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `HS_LSPI_RULE`"]
pub type HS_LSPI_RULE_R = crate::R<u8, HS_LSPI_RULE_A>;
impl HS_LSPI_RULE_R {
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
        *self == HS_LSPI_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == HS_LSPI_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == HS_LSPI_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == HS_LSPI_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `HS_LSPI_RULE`"]
pub struct HS_LSPI_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_LSPI_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS_LSPI_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
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
}
