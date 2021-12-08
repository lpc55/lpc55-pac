#[doc = "Register `MASTER_SEC_ANTI_POL_REG` reader"]
pub struct R(crate::R<MASTER_SEC_ANTI_POL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASTER_SEC_ANTI_POL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASTER_SEC_ANTI_POL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASTER_SEC_ANTI_POL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASTER_SEC_ANTI_POL_REG` writer"]
pub struct W(crate::W<MASTER_SEC_ANTI_POL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASTER_SEC_ANTI_POL_REG_SPEC>;
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
impl From<crate::W<MASTER_SEC_ANTI_POL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASTER_SEC_ANTI_POL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Micro-Cortex M33 (CPU1) Code bus. Must be equal to NOT(MASTER_SEC_LEVEL.CPU1C)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPU1C_A {
    #[doc = "0: Secure and Priviledge user access allowed."]
    ENUM_S_P = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 3,
}
impl From<CPU1C_A> for u8 {
    #[inline(always)]
    fn from(variant: CPU1C_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CPU1C` reader - Micro-Cortex M33 (CPU1) Code bus. Must be equal to NOT(MASTER_SEC_LEVEL.CPU1C)"]
pub struct CPU1C_R(crate::FieldReader<u8, CPU1C_A>);
impl CPU1C_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPU1C_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1C_A {
        match self.bits {
            0 => CPU1C_A::ENUM_S_P,
            1 => CPU1C_A::ENUM_S_NP,
            2 => CPU1C_A::ENUM_NS_P,
            3 => CPU1C_A::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == CPU1C_A::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == CPU1C_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == CPU1C_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == CPU1C_A::ENUM_NS_NP
    }
}
impl core::ops::Deref for CPU1C_R {
    type Target = crate::FieldReader<u8, CPU1C_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU1C` writer - Micro-Cortex M33 (CPU1) Code bus. Must be equal to NOT(MASTER_SEC_LEVEL.CPU1C)"]
pub struct CPU1C_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU1C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU1C_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(CPU1C_A::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(CPU1C_A::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(CPU1C_A::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(CPU1C_A::ENUM_NS_NP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Micro-Cortex M33 (CPU1) System bus. Must be equal to NOT(MASTER_SEC_LEVEL.CPU1S)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPU1S_A {
    #[doc = "0: Secure and Priviledge user access allowed."]
    ENUM_S_P = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 3,
}
impl From<CPU1S_A> for u8 {
    #[inline(always)]
    fn from(variant: CPU1S_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CPU1S` reader - Micro-Cortex M33 (CPU1) System bus. Must be equal to NOT(MASTER_SEC_LEVEL.CPU1S)"]
pub struct CPU1S_R(crate::FieldReader<u8, CPU1S_A>);
impl CPU1S_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPU1S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1S_A {
        match self.bits {
            0 => CPU1S_A::ENUM_S_P,
            1 => CPU1S_A::ENUM_S_NP,
            2 => CPU1S_A::ENUM_NS_P,
            3 => CPU1S_A::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == CPU1S_A::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == CPU1S_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == CPU1S_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == CPU1S_A::ENUM_NS_NP
    }
}
impl core::ops::Deref for CPU1S_R {
    type Target = crate::FieldReader<u8, CPU1S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU1S` writer - Micro-Cortex M33 (CPU1) System bus. Must be equal to NOT(MASTER_SEC_LEVEL.CPU1S)"]
pub struct CPU1S_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU1S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU1S_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(CPU1S_A::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(CPU1S_A::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(CPU1S_A::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(CPU1S_A::ENUM_NS_NP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "USB Full Speed Device. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSD)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USBFSD_A {
    #[doc = "0: Secure and Priviledge user access allowed."]
    ENUM_S_P = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 3,
}
impl From<USBFSD_A> for u8 {
    #[inline(always)]
    fn from(variant: USBFSD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USBFSD` reader - USB Full Speed Device. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSD)"]
pub struct USBFSD_R(crate::FieldReader<u8, USBFSD_A>);
impl USBFSD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USBFSD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBFSD_A {
        match self.bits {
            0 => USBFSD_A::ENUM_S_P,
            1 => USBFSD_A::ENUM_S_NP,
            2 => USBFSD_A::ENUM_NS_P,
            3 => USBFSD_A::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == USBFSD_A::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == USBFSD_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == USBFSD_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == USBFSD_A::ENUM_NS_NP
    }
}
impl core::ops::Deref for USBFSD_R {
    type Target = crate::FieldReader<u8, USBFSD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBFSD` writer - USB Full Speed Device. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSD)"]
pub struct USBFSD_W<'a> {
    w: &'a mut W,
}
impl<'a> USBFSD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBFSD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(USBFSD_A::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(USBFSD_A::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(USBFSD_A::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(USBFSD_A::ENUM_NS_NP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDMA0_A {
    #[doc = "0: Secure and Priviledge user access allowed."]
    ENUM_S_P = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 3,
}
impl From<SDMA0_A> for u8 {
    #[inline(always)]
    fn from(variant: SDMA0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SDMA0` reader - System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)"]
pub struct SDMA0_R(crate::FieldReader<u8, SDMA0_A>);
impl SDMA0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDMA0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMA0_A {
        match self.bits {
            0 => SDMA0_A::ENUM_S_P,
            1 => SDMA0_A::ENUM_S_NP,
            2 => SDMA0_A::ENUM_NS_P,
            3 => SDMA0_A::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == SDMA0_A::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == SDMA0_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == SDMA0_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == SDMA0_A::ENUM_NS_NP
    }
}
impl core::ops::Deref for SDMA0_R {
    type Target = crate::FieldReader<u8, SDMA0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMA0` writer - System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)"]
pub struct SDMA0_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMA0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SDMA0_A::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SDMA0_A::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SDMA0_A::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SDMA0_A::ENUM_NS_NP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "SDIO. Must be equal to NOT(MASTER_SEC_LEVEL.SDIO)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDIO_A {
    #[doc = "0: Secure and Priviledge user access allowed."]
    ENUM_S_P = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 3,
}
impl From<SDIO_A> for u8 {
    #[inline(always)]
    fn from(variant: SDIO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SDIO` reader - SDIO. Must be equal to NOT(MASTER_SEC_LEVEL.SDIO)"]
pub struct SDIO_R(crate::FieldReader<u8, SDIO_A>);
impl SDIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDIO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIO_A {
        match self.bits {
            0 => SDIO_A::ENUM_S_P,
            1 => SDIO_A::ENUM_S_NP,
            2 => SDIO_A::ENUM_NS_P,
            3 => SDIO_A::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == SDIO_A::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == SDIO_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == SDIO_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == SDIO_A::ENUM_NS_NP
    }
}
impl core::ops::Deref for SDIO_R {
    type Target = crate::FieldReader<u8, SDIO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO` writer - SDIO. Must be equal to NOT(MASTER_SEC_LEVEL.SDIO)"]
pub struct SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIO_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SDIO_A::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SDIO_A::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SDIO_A::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SDIO_A::ENUM_NS_NP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Power Quad. Must be equal to NOT(MASTER_SEC_LEVEL.PQ)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PQ_A {
    #[doc = "0: Secure and Priviledge user access allowed."]
    ENUM_S_P = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 3,
}
impl From<PQ_A> for u8 {
    #[inline(always)]
    fn from(variant: PQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PQ` reader - Power Quad. Must be equal to NOT(MASTER_SEC_LEVEL.PQ)"]
pub struct PQ_R(crate::FieldReader<u8, PQ_A>);
impl PQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PQ_A {
        match self.bits {
            0 => PQ_A::ENUM_S_P,
            1 => PQ_A::ENUM_S_NP,
            2 => PQ_A::ENUM_NS_P,
            3 => PQ_A::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == PQ_A::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == PQ_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == PQ_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == PQ_A::ENUM_NS_NP
    }
}
impl core::ops::Deref for PQ_R {
    type Target = crate::FieldReader<u8, PQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PQ` writer - Power Quad. Must be equal to NOT(MASTER_SEC_LEVEL.PQ)"]
pub struct PQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PQ_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(PQ_A::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(PQ_A::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(PQ_A::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(PQ_A::ENUM_NS_NP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HASH_A {
    #[doc = "0: Secure and Priviledge user access allowed."]
    ENUM_S_P = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 3,
}
impl From<HASH_A> for u8 {
    #[inline(always)]
    fn from(variant: HASH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HASH` reader - Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)"]
pub struct HASH_R(crate::FieldReader<u8, HASH_A>);
impl HASH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HASH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASH_A {
        match self.bits {
            0 => HASH_A::ENUM_S_P,
            1 => HASH_A::ENUM_S_NP,
            2 => HASH_A::ENUM_NS_P,
            3 => HASH_A::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == HASH_A::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == HASH_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == HASH_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == HASH_A::ENUM_NS_NP
    }
}
impl core::ops::Deref for HASH_R {
    type Target = crate::FieldReader<u8, HASH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASH` writer - Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)"]
pub struct HASH_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HASH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(HASH_A::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(HASH_A::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(HASH_A::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(HASH_A::ENUM_NS_NP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "USB Full speed Host. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSH)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USBFSH_A {
    #[doc = "0: Secure and Priviledge user access allowed."]
    ENUM_S_P = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 3,
}
impl From<USBFSH_A> for u8 {
    #[inline(always)]
    fn from(variant: USBFSH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USBFSH` reader - USB Full speed Host. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSH)"]
pub struct USBFSH_R(crate::FieldReader<u8, USBFSH_A>);
impl USBFSH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USBFSH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBFSH_A {
        match self.bits {
            0 => USBFSH_A::ENUM_S_P,
            1 => USBFSH_A::ENUM_S_NP,
            2 => USBFSH_A::ENUM_NS_P,
            3 => USBFSH_A::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == USBFSH_A::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == USBFSH_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == USBFSH_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == USBFSH_A::ENUM_NS_NP
    }
}
impl core::ops::Deref for USBFSH_R {
    type Target = crate::FieldReader<u8, USBFSH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBFSH` writer - USB Full speed Host. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSH)"]
pub struct USBFSH_W<'a> {
    w: &'a mut W,
}
impl<'a> USBFSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBFSH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(USBFSH_A::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(USBFSH_A::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(USBFSH_A::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(USBFSH_A::ENUM_NS_NP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDMA1_A {
    #[doc = "0: Secure and Priviledge user access allowed."]
    ENUM_S_P = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 3,
}
impl From<SDMA1_A> for u8 {
    #[inline(always)]
    fn from(variant: SDMA1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SDMA1` reader - System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)"]
pub struct SDMA1_R(crate::FieldReader<u8, SDMA1_A>);
impl SDMA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDMA1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMA1_A {
        match self.bits {
            0 => SDMA1_A::ENUM_S_P,
            1 => SDMA1_A::ENUM_S_NP,
            2 => SDMA1_A::ENUM_NS_P,
            3 => SDMA1_A::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == SDMA1_A::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == SDMA1_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == SDMA1_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == SDMA1_A::ENUM_NS_NP
    }
}
impl core::ops::Deref for SDMA1_R {
    type Target = crate::FieldReader<u8, SDMA1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMA1` writer - System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)"]
pub struct SDMA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMA1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SDMA1_A::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SDMA1_A::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SDMA1_A::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SDMA1_A::ENUM_NS_NP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "MASTER_SEC_ANTI_POL_REG register write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MASTER_SEC_LEVEL_ANTIPOL_LOCK_A {
    #[doc = "1: Restricted mode."]
    BLOCKED = 1,
    #[doc = "2: Writable."]
    WRITABLE = 2,
}
impl From<MASTER_SEC_LEVEL_ANTIPOL_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: MASTER_SEC_LEVEL_ANTIPOL_LOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MASTER_SEC_LEVEL_ANTIPOL_LOCK` reader - MASTER_SEC_ANTI_POL_REG register write-lock."]
pub struct MASTER_SEC_LEVEL_ANTIPOL_LOCK_R(crate::FieldReader<u8, MASTER_SEC_LEVEL_ANTIPOL_LOCK_A>);
impl MASTER_SEC_LEVEL_ANTIPOL_LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MASTER_SEC_LEVEL_ANTIPOL_LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MASTER_SEC_LEVEL_ANTIPOL_LOCK_A> {
        match self.bits {
            1 => Some(MASTER_SEC_LEVEL_ANTIPOL_LOCK_A::BLOCKED),
            2 => Some(MASTER_SEC_LEVEL_ANTIPOL_LOCK_A::WRITABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == MASTER_SEC_LEVEL_ANTIPOL_LOCK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        **self == MASTER_SEC_LEVEL_ANTIPOL_LOCK_A::WRITABLE
    }
}
impl core::ops::Deref for MASTER_SEC_LEVEL_ANTIPOL_LOCK_R {
    type Target = crate::FieldReader<u8, MASTER_SEC_LEVEL_ANTIPOL_LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER_SEC_LEVEL_ANTIPOL_LOCK` writer - MASTER_SEC_ANTI_POL_REG register write-lock."]
pub struct MASTER_SEC_LEVEL_ANTIPOL_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_SEC_LEVEL_ANTIPOL_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASTER_SEC_LEVEL_ANTIPOL_LOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(MASTER_SEC_LEVEL_ANTIPOL_LOCK_A::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut W {
        self.variant(MASTER_SEC_LEVEL_ANTIPOL_LOCK_A::WRITABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:5 - Micro-Cortex M33 (CPU1) Code bus. Must be equal to NOT(MASTER_SEC_LEVEL.CPU1C)"]
    #[inline(always)]
    pub fn cpu1c(&self) -> CPU1C_R {
        CPU1C_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Micro-Cortex M33 (CPU1) System bus. Must be equal to NOT(MASTER_SEC_LEVEL.CPU1S)"]
    #[inline(always)]
    pub fn cpu1s(&self) -> CPU1S_R {
        CPU1S_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - USB Full Speed Device. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSD)"]
    #[inline(always)]
    pub fn usbfsd(&self) -> USBFSD_R {
        USBFSD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)"]
    #[inline(always)]
    pub fn sdma0(&self) -> SDMA0_R {
        SDMA0_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - SDIO. Must be equal to NOT(MASTER_SEC_LEVEL.SDIO)"]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Power Quad. Must be equal to NOT(MASTER_SEC_LEVEL.PQ)"]
    #[inline(always)]
    pub fn pq(&self) -> PQ_R {
        PQ_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)"]
    #[inline(always)]
    pub fn hash(&self) -> HASH_R {
        HASH_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - USB Full speed Host. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSH)"]
    #[inline(always)]
    pub fn usbfsh(&self) -> USBFSH_R {
        USBFSH_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)"]
    #[inline(always)]
    pub fn sdma1(&self) -> SDMA1_R {
        SDMA1_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - MASTER_SEC_ANTI_POL_REG register write-lock."]
    #[inline(always)]
    pub fn master_sec_level_antipol_lock(&self) -> MASTER_SEC_LEVEL_ANTIPOL_LOCK_R {
        MASTER_SEC_LEVEL_ANTIPOL_LOCK_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Micro-Cortex M33 (CPU1) Code bus. Must be equal to NOT(MASTER_SEC_LEVEL.CPU1C)"]
    #[inline(always)]
    pub fn cpu1c(&mut self) -> CPU1C_W {
        CPU1C_W { w: self }
    }
    #[doc = "Bits 6:7 - Micro-Cortex M33 (CPU1) System bus. Must be equal to NOT(MASTER_SEC_LEVEL.CPU1S)"]
    #[inline(always)]
    pub fn cpu1s(&mut self) -> CPU1S_W {
        CPU1S_W { w: self }
    }
    #[doc = "Bits 8:9 - USB Full Speed Device. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSD)"]
    #[inline(always)]
    pub fn usbfsd(&mut self) -> USBFSD_W {
        USBFSD_W { w: self }
    }
    #[doc = "Bits 10:11 - System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)"]
    #[inline(always)]
    pub fn sdma0(&mut self) -> SDMA0_W {
        SDMA0_W { w: self }
    }
    #[doc = "Bits 16:17 - SDIO. Must be equal to NOT(MASTER_SEC_LEVEL.SDIO)"]
    #[inline(always)]
    pub fn sdio(&mut self) -> SDIO_W {
        SDIO_W { w: self }
    }
    #[doc = "Bits 18:19 - Power Quad. Must be equal to NOT(MASTER_SEC_LEVEL.PQ)"]
    #[inline(always)]
    pub fn pq(&mut self) -> PQ_W {
        PQ_W { w: self }
    }
    #[doc = "Bits 20:21 - Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)"]
    #[inline(always)]
    pub fn hash(&mut self) -> HASH_W {
        HASH_W { w: self }
    }
    #[doc = "Bits 22:23 - USB Full speed Host. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSH)"]
    #[inline(always)]
    pub fn usbfsh(&mut self) -> USBFSH_W {
        USBFSH_W { w: self }
    }
    #[doc = "Bits 24:25 - System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)"]
    #[inline(always)]
    pub fn sdma1(&mut self) -> SDMA1_W {
        SDMA1_W { w: self }
    }
    #[doc = "Bits 30:31 - MASTER_SEC_ANTI_POL_REG register write-lock."]
    #[inline(always)]
    pub fn master_sec_level_antipol_lock(&mut self) -> MASTER_SEC_LEVEL_ANTIPOL_LOCK_W {
        MASTER_SEC_LEVEL_ANTIPOL_LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "master secure level anti-pole register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [master_sec_anti_pol_reg](index.html) module"]
pub struct MASTER_SEC_ANTI_POL_REG_SPEC;
impl crate::RegisterSpec for MASTER_SEC_ANTI_POL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [master_sec_anti_pol_reg::R](R) reader structure"]
impl crate::Readable for MASTER_SEC_ANTI_POL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [master_sec_anti_pol_reg::W](W) writer structure"]
impl crate::Writable for MASTER_SEC_ANTI_POL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASTER_SEC_ANTI_POL_REG to value 0xbfff_ffff"]
impl crate::Resettable for MASTER_SEC_ANTI_POL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xbfff_ffff
    }
}
