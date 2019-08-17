#[doc = "Reader of register SEC_CTRL_AHB0_0_SLAVE_RULE"]
pub type R = crate::R<u32, super::SEC_CTRL_AHB0_0_SLAVE_RULE>;
#[doc = "Writer for register SEC_CTRL_AHB0_0_SLAVE_RULE"]
pub type W = crate::W<u32, super::SEC_CTRL_AHB0_0_SLAVE_RULE>;
#[doc = "Register SEC_CTRL_AHB0_0_SLAVE_RULE `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC_CTRL_AHB0_0_SLAVE_RULE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `DMA0_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA0_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<DMA0_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA0_RULE_A) -> Self {
        match variant {
            DMA0_RULE_A::ENUM_NS_NP => 0,
            DMA0_RULE_A::ENUM_NS_P => 1,
            DMA0_RULE_A::ENUM_S_NP => 2,
            DMA0_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `DMA0_RULE`"]
pub type DMA0_RULE_R = crate::R<u8, DMA0_RULE_A>;
impl DMA0_RULE_R {
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
        *self == DMA0_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == DMA0_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == DMA0_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == DMA0_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `DMA0_RULE`"]
pub struct DMA0_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA0_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `FS_USB_DEV_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FS_USB_DEV_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<FS_USB_DEV_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FS_USB_DEV_RULE_A) -> Self {
        match variant {
            FS_USB_DEV_RULE_A::ENUM_NS_NP => 0,
            FS_USB_DEV_RULE_A::ENUM_NS_P => 1,
            FS_USB_DEV_RULE_A::ENUM_S_NP => 2,
            FS_USB_DEV_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `FS_USB_DEV_RULE`"]
pub type FS_USB_DEV_RULE_R = crate::R<u8, FS_USB_DEV_RULE_A>;
impl FS_USB_DEV_RULE_R {
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
        *self == FS_USB_DEV_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == FS_USB_DEV_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == FS_USB_DEV_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == FS_USB_DEV_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `FS_USB_DEV_RULE`"]
pub struct FS_USB_DEV_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> FS_USB_DEV_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FS_USB_DEV_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `SCT_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<SCT_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: SCT_RULE_A) -> Self {
        match variant {
            SCT_RULE_A::ENUM_NS_NP => 0,
            SCT_RULE_A::ENUM_NS_P => 1,
            SCT_RULE_A::ENUM_S_NP => 2,
            SCT_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `SCT_RULE`"]
pub type SCT_RULE_R = crate::R<u8, SCT_RULE_A>;
impl SCT_RULE_R {
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
        *self == SCT_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SCT_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SCT_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SCT_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `SCT_RULE`"]
pub struct SCT_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCT_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `FLEXCOMM0_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCOMM0_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<FLEXCOMM0_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM0_RULE_A) -> Self {
        match variant {
            FLEXCOMM0_RULE_A::ENUM_NS_NP => 0,
            FLEXCOMM0_RULE_A::ENUM_NS_P => 1,
            FLEXCOMM0_RULE_A::ENUM_S_NP => 2,
            FLEXCOMM0_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `FLEXCOMM0_RULE`"]
pub type FLEXCOMM0_RULE_R = crate::R<u8, FLEXCOMM0_RULE_A>;
impl FLEXCOMM0_RULE_R {
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
        *self == FLEXCOMM0_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == FLEXCOMM0_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == FLEXCOMM0_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == FLEXCOMM0_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `FLEXCOMM0_RULE`"]
pub struct FLEXCOMM0_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM0_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCOMM0_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `FLEXCOMM1_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCOMM1_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<FLEXCOMM1_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM1_RULE_A) -> Self {
        match variant {
            FLEXCOMM1_RULE_A::ENUM_NS_NP => 0,
            FLEXCOMM1_RULE_A::ENUM_NS_P => 1,
            FLEXCOMM1_RULE_A::ENUM_S_NP => 2,
            FLEXCOMM1_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `FLEXCOMM1_RULE`"]
pub type FLEXCOMM1_RULE_R = crate::R<u8, FLEXCOMM1_RULE_A>;
impl FLEXCOMM1_RULE_R {
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
        *self == FLEXCOMM1_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == FLEXCOMM1_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == FLEXCOMM1_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == FLEXCOMM1_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `FLEXCOMM1_RULE`"]
pub struct FLEXCOMM1_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM1_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCOMM1_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
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
}
