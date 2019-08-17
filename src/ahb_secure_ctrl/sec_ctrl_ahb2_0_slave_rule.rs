#[doc = "Reader of register SEC_CTRL_AHB2_0_SLAVE_RULE"]
pub type R = crate::R<u32, super::SEC_CTRL_AHB2_0_SLAVE_RULE>;
#[doc = "Writer for register SEC_CTRL_AHB2_0_SLAVE_RULE"]
pub type W = crate::W<u32, super::SEC_CTRL_AHB2_0_SLAVE_RULE>;
#[doc = "Register SEC_CTRL_AHB2_0_SLAVE_RULE `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC_CTRL_AHB2_0_SLAVE_RULE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `ADC_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<ADC_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_RULE_A) -> Self {
        match variant {
            ADC_RULE_A::ENUM_NS_NP => 0,
            ADC_RULE_A::ENUM_NS_P => 1,
            ADC_RULE_A::ENUM_S_NP => 2,
            ADC_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `ADC_RULE`"]
pub type ADC_RULE_R = crate::R<u8, ADC_RULE_A>;
impl ADC_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_RULE_A {
        match self.bits {
            0 => ADC_RULE_A::ENUM_NS_NP,
            1 => ADC_RULE_A::ENUM_NS_P,
            2 => ADC_RULE_A::ENUM_S_NP,
            3 => ADC_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == ADC_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == ADC_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == ADC_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == ADC_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `ADC_RULE`"]
pub struct ADC_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(ADC_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(ADC_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(ADC_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(ADC_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `USB_FS_HOST_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_FS_HOST_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<USB_FS_HOST_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_FS_HOST_RULE_A) -> Self {
        match variant {
            USB_FS_HOST_RULE_A::ENUM_NS_NP => 0,
            USB_FS_HOST_RULE_A::ENUM_NS_P => 1,
            USB_FS_HOST_RULE_A::ENUM_S_NP => 2,
            USB_FS_HOST_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `USB_FS_HOST_RULE`"]
pub type USB_FS_HOST_RULE_R = crate::R<u8, USB_FS_HOST_RULE_A>;
impl USB_FS_HOST_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_FS_HOST_RULE_A {
        match self.bits {
            0 => USB_FS_HOST_RULE_A::ENUM_NS_NP,
            1 => USB_FS_HOST_RULE_A::ENUM_NS_P,
            2 => USB_FS_HOST_RULE_A::ENUM_S_NP,
            3 => USB_FS_HOST_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == USB_FS_HOST_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == USB_FS_HOST_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == USB_FS_HOST_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == USB_FS_HOST_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `USB_FS_HOST_RULE`"]
pub struct USB_FS_HOST_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_FS_HOST_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_FS_HOST_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(USB_FS_HOST_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(USB_FS_HOST_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(USB_FS_HOST_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(USB_FS_HOST_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `USB_HS_HOST_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_HS_HOST_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<USB_HS_HOST_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_HS_HOST_RULE_A) -> Self {
        match variant {
            USB_HS_HOST_RULE_A::ENUM_NS_NP => 0,
            USB_HS_HOST_RULE_A::ENUM_NS_P => 1,
            USB_HS_HOST_RULE_A::ENUM_S_NP => 2,
            USB_HS_HOST_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `USB_HS_HOST_RULE`"]
pub type USB_HS_HOST_RULE_R = crate::R<u8, USB_HS_HOST_RULE_A>;
impl USB_HS_HOST_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_HS_HOST_RULE_A {
        match self.bits {
            0 => USB_HS_HOST_RULE_A::ENUM_NS_NP,
            1 => USB_HS_HOST_RULE_A::ENUM_NS_P,
            2 => USB_HS_HOST_RULE_A::ENUM_S_NP,
            3 => USB_HS_HOST_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == USB_HS_HOST_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == USB_HS_HOST_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == USB_HS_HOST_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == USB_HS_HOST_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `USB_HS_HOST_RULE`"]
pub struct USB_HS_HOST_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_HS_HOST_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_HS_HOST_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(USB_HS_HOST_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(USB_HS_HOST_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(USB_HS_HOST_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(USB_HS_HOST_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `HASH_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASH_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<HASH_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: HASH_RULE_A) -> Self {
        match variant {
            HASH_RULE_A::ENUM_NS_NP => 0,
            HASH_RULE_A::ENUM_NS_P => 1,
            HASH_RULE_A::ENUM_S_NP => 2,
            HASH_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `HASH_RULE`"]
pub type HASH_RULE_R = crate::R<u8, HASH_RULE_A>;
impl HASH_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASH_RULE_A {
        match self.bits {
            0 => HASH_RULE_A::ENUM_NS_NP,
            1 => HASH_RULE_A::ENUM_NS_P,
            2 => HASH_RULE_A::ENUM_S_NP,
            3 => HASH_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == HASH_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == HASH_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == HASH_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == HASH_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `HASH_RULE`"]
pub struct HASH_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HASH_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(HASH_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(HASH_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(HASH_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(HASH_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `CASPER_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CASPER_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<CASPER_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: CASPER_RULE_A) -> Self {
        match variant {
            CASPER_RULE_A::ENUM_NS_NP => 0,
            CASPER_RULE_A::ENUM_NS_P => 1,
            CASPER_RULE_A::ENUM_S_NP => 2,
            CASPER_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `CASPER_RULE`"]
pub type CASPER_RULE_R = crate::R<u8, CASPER_RULE_A>;
impl CASPER_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CASPER_RULE_A {
        match self.bits {
            0 => CASPER_RULE_A::ENUM_NS_NP,
            1 => CASPER_RULE_A::ENUM_NS_P,
            2 => CASPER_RULE_A::ENUM_S_NP,
            3 => CASPER_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == CASPER_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == CASPER_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == CASPER_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == CASPER_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `CASPER_RULE`"]
pub struct CASPER_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> CASPER_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CASPER_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(CASPER_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(CASPER_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(CASPER_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(CASPER_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `PQ_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PQ_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<PQ_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: PQ_RULE_A) -> Self {
        match variant {
            PQ_RULE_A::ENUM_NS_NP => 0,
            PQ_RULE_A::ENUM_NS_P => 1,
            PQ_RULE_A::ENUM_S_NP => 2,
            PQ_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `PQ_RULE`"]
pub type PQ_RULE_R = crate::R<u8, PQ_RULE_A>;
impl PQ_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PQ_RULE_A {
        match self.bits {
            0 => PQ_RULE_A::ENUM_NS_NP,
            1 => PQ_RULE_A::ENUM_NS_P,
            2 => PQ_RULE_A::ENUM_S_NP,
            3 => PQ_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == PQ_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == PQ_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == PQ_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == PQ_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `PQ_RULE`"]
pub struct PQ_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> PQ_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PQ_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(PQ_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(PQ_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(PQ_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(PQ_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `DMA1_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<DMA1_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA1_RULE_A) -> Self {
        match variant {
            DMA1_RULE_A::ENUM_NS_NP => 0,
            DMA1_RULE_A::ENUM_NS_P => 1,
            DMA1_RULE_A::ENUM_S_NP => 2,
            DMA1_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `DMA1_RULE`"]
pub type DMA1_RULE_R = crate::R<u8, DMA1_RULE_A>;
impl DMA1_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1_RULE_A {
        match self.bits {
            0 => DMA1_RULE_A::ENUM_NS_NP,
            1 => DMA1_RULE_A::ENUM_NS_P,
            2 => DMA1_RULE_A::ENUM_S_NP,
            3 => DMA1_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == DMA1_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == DMA1_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == DMA1_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == DMA1_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `DMA1_RULE`"]
pub struct DMA1_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA1_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(DMA1_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(DMA1_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(DMA1_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(DMA1_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ADC"]
    #[inline(always)]
    pub fn adc_rule(&self) -> ADC_RULE_R {
        ADC_RULE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - USB Full Speed Host registers."]
    #[inline(always)]
    pub fn usb_fs_host_rule(&self) -> USB_FS_HOST_RULE_R {
        USB_FS_HOST_RULE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - USB High speed host registers"]
    #[inline(always)]
    pub fn usb_hs_host_rule(&self) -> USB_HS_HOST_RULE_R {
        USB_HS_HOST_RULE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - SHA-2 crypto registers"]
    #[inline(always)]
    pub fn hash_rule(&self) -> HASH_RULE_R {
        HASH_RULE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - RSA/ECC crypto accelerator"]
    #[inline(always)]
    pub fn casper_rule(&self) -> CASPER_RULE_R {
        CASPER_RULE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Power Quad (CM33 processor hardware accelerator)"]
    #[inline(always)]
    pub fn pq_rule(&self) -> PQ_RULE_R {
        PQ_RULE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - DMA Controller (Secure)"]
    #[inline(always)]
    pub fn dma1_rule(&self) -> DMA1_RULE_R {
        DMA1_RULE_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC"]
    #[inline(always)]
    pub fn adc_rule(&mut self) -> ADC_RULE_W {
        ADC_RULE_W { w: self }
    }
    #[doc = "Bits 8:9 - USB Full Speed Host registers."]
    #[inline(always)]
    pub fn usb_fs_host_rule(&mut self) -> USB_FS_HOST_RULE_W {
        USB_FS_HOST_RULE_W { w: self }
    }
    #[doc = "Bits 12:13 - USB High speed host registers"]
    #[inline(always)]
    pub fn usb_hs_host_rule(&mut self) -> USB_HS_HOST_RULE_W {
        USB_HS_HOST_RULE_W { w: self }
    }
    #[doc = "Bits 16:17 - SHA-2 crypto registers"]
    #[inline(always)]
    pub fn hash_rule(&mut self) -> HASH_RULE_W {
        HASH_RULE_W { w: self }
    }
    #[doc = "Bits 20:21 - RSA/ECC crypto accelerator"]
    #[inline(always)]
    pub fn casper_rule(&mut self) -> CASPER_RULE_W {
        CASPER_RULE_W { w: self }
    }
    #[doc = "Bits 24:25 - Power Quad (CM33 processor hardware accelerator)"]
    #[inline(always)]
    pub fn pq_rule(&mut self) -> PQ_RULE_W {
        PQ_RULE_W { w: self }
    }
    #[doc = "Bits 28:29 - DMA Controller (Secure)"]
    #[inline(always)]
    pub fn dma1_rule(&mut self) -> DMA1_RULE_W {
        DMA1_RULE_W { w: self }
    }
}
