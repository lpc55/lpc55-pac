#[doc = "Reader of register DEBUG_FEATURES_DP"]
pub type R = crate::R<u32, super::DEBUG_FEATURES_DP>;
#[doc = "Writer for register DEBUG_FEATURES_DP"]
pub type W = crate::W<u32, super::DEBUG_FEATURES_DP>;
#[doc = "Register DEBUG_FEATURES_DP `reset()`'s with value 0x0555"]
impl crate::ResetValue for super::DEBUG_FEATURES_DP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0555
    }
}
#[doc = "CPU0 (CPU0) Invasive debug control:.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPU0_DBGEN_A {
    #[doc = "1: Any other value than b10: invasive debug is disable."]
    DISABLE = 1,
    #[doc = "2: 10: Invasive debug is enabled."]
    ENABLE = 2,
}
impl From<CPU0_DBGEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CPU0_DBGEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CPU0_DBGEN`"]
pub type CPU0_DBGEN_R = crate::R<u8, CPU0_DBGEN_A>;
impl CPU0_DBGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CPU0_DBGEN_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CPU0_DBGEN_A::DISABLE),
            2 => Val(CPU0_DBGEN_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CPU0_DBGEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPU0_DBGEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `CPU0_DBGEN`"]
pub struct CPU0_DBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU0_DBGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU0_DBGEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CPU0_DBGEN_A::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU0_DBGEN_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "CPU0 Non Invasive debug control:.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPU0_NIDEN_A {
    #[doc = "1: Any other value than b10: invasive debug is disable."]
    DISABLE = 1,
    #[doc = "2: 10: Invasive debug is enabled."]
    ENABLE = 2,
}
impl From<CPU0_NIDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CPU0_NIDEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CPU0_NIDEN`"]
pub type CPU0_NIDEN_R = crate::R<u8, CPU0_NIDEN_A>;
impl CPU0_NIDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CPU0_NIDEN_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CPU0_NIDEN_A::DISABLE),
            2 => Val(CPU0_NIDEN_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CPU0_NIDEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPU0_NIDEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `CPU0_NIDEN`"]
pub struct CPU0_NIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU0_NIDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU0_NIDEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CPU0_NIDEN_A::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU0_NIDEN_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "CPU0 Secure Invasive debug control:.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPU0_SPIDEN_A {
    #[doc = "1: Any other value than b10: invasive debug is disable."]
    DISABLE = 1,
    #[doc = "2: 10: Invasive debug is enabled."]
    ENABLE = 2,
}
impl From<CPU0_SPIDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CPU0_SPIDEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CPU0_SPIDEN`"]
pub type CPU0_SPIDEN_R = crate::R<u8, CPU0_SPIDEN_A>;
impl CPU0_SPIDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CPU0_SPIDEN_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CPU0_SPIDEN_A::DISABLE),
            2 => Val(CPU0_SPIDEN_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CPU0_SPIDEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPU0_SPIDEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `CPU0_SPIDEN`"]
pub struct CPU0_SPIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU0_SPIDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU0_SPIDEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CPU0_SPIDEN_A::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU0_SPIDEN_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "CPU0 Secure Non Invasive debug control:.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPU0_SPNIDEN_A {
    #[doc = "1: Any other value than b10: invasive debug is disable."]
    DISABLE = 1,
    #[doc = "2: 10: Invasive debug is enabled."]
    ENABLE = 2,
}
impl From<CPU0_SPNIDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CPU0_SPNIDEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CPU0_SPNIDEN`"]
pub type CPU0_SPNIDEN_R = crate::R<u8, CPU0_SPNIDEN_A>;
impl CPU0_SPNIDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CPU0_SPNIDEN_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CPU0_SPNIDEN_A::DISABLE),
            2 => Val(CPU0_SPNIDEN_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CPU0_SPNIDEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPU0_SPNIDEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `CPU0_SPNIDEN`"]
pub struct CPU0_SPNIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU0_SPNIDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU0_SPNIDEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CPU0_SPNIDEN_A::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU0_SPNIDEN_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "CPU1 Invasive debug control:.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPU1_DBGEN_A {
    #[doc = "1: Any other value than b10: invasive debug is disable."]
    DISABLE = 1,
    #[doc = "2: 10: Invasive debug is enabled."]
    ENABLE = 2,
}
impl From<CPU1_DBGEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CPU1_DBGEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CPU1_DBGEN`"]
pub type CPU1_DBGEN_R = crate::R<u8, CPU1_DBGEN_A>;
impl CPU1_DBGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CPU1_DBGEN_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CPU1_DBGEN_A::DISABLE),
            2 => Val(CPU1_DBGEN_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CPU1_DBGEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPU1_DBGEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `CPU1_DBGEN`"]
pub struct CPU1_DBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU1_DBGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU1_DBGEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CPU1_DBGEN_A::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU1_DBGEN_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "CPU1 Non Invasive debug control:.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPU1_NIDEN_A {
    #[doc = "1: Any other value than b10: invasive debug is disable."]
    DISABLE = 1,
    #[doc = "2: 10: Invasive debug is enabled."]
    ENABLE = 2,
}
impl From<CPU1_NIDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CPU1_NIDEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CPU1_NIDEN`"]
pub type CPU1_NIDEN_R = crate::R<u8, CPU1_NIDEN_A>;
impl CPU1_NIDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CPU1_NIDEN_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CPU1_NIDEN_A::DISABLE),
            2 => Val(CPU1_NIDEN_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CPU1_NIDEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPU1_NIDEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `CPU1_NIDEN`"]
pub struct CPU1_NIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU1_NIDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU1_NIDEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CPU1_NIDEN_A::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU1_NIDEN_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CPU0 (CPU0) Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_dbgen(&self) -> CPU0_DBGEN_R {
        CPU0_DBGEN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - CPU0 Non Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_niden(&self) -> CPU0_NIDEN_R {
        CPU0_NIDEN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - CPU0 Secure Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_spiden(&self) -> CPU0_SPIDEN_R {
        CPU0_SPIDEN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - CPU0 Secure Non Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_spniden(&self) -> CPU0_SPNIDEN_R {
        CPU0_SPNIDEN_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - CPU1 Invasive debug control:."]
    #[inline(always)]
    pub fn cpu1_dbgen(&self) -> CPU1_DBGEN_R {
        CPU1_DBGEN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - CPU1 Non Invasive debug control:."]
    #[inline(always)]
    pub fn cpu1_niden(&self) -> CPU1_NIDEN_R {
        CPU1_NIDEN_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CPU0 (CPU0) Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_dbgen(&mut self) -> CPU0_DBGEN_W {
        CPU0_DBGEN_W { w: self }
    }
    #[doc = "Bits 2:3 - CPU0 Non Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_niden(&mut self) -> CPU0_NIDEN_W {
        CPU0_NIDEN_W { w: self }
    }
    #[doc = "Bits 4:5 - CPU0 Secure Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_spiden(&mut self) -> CPU0_SPIDEN_W {
        CPU0_SPIDEN_W { w: self }
    }
    #[doc = "Bits 6:7 - CPU0 Secure Non Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_spniden(&mut self) -> CPU0_SPNIDEN_W {
        CPU0_SPNIDEN_W { w: self }
    }
    #[doc = "Bits 8:9 - CPU1 Invasive debug control:."]
    #[inline(always)]
    pub fn cpu1_dbgen(&mut self) -> CPU1_DBGEN_W {
        CPU1_DBGEN_W { w: self }
    }
    #[doc = "Bits 10:11 - CPU1 Non Invasive debug control:."]
    #[inline(always)]
    pub fn cpu1_niden(&mut self) -> CPU1_NIDEN_W {
        CPU1_NIDEN_W { w: self }
    }
}
