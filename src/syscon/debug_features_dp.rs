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
#[doc = "Possible values of the field `CM33_DBGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM33_DBGEN_A {
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE,
}
impl From<CM33_DBGEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CM33_DBGEN_A) -> Self {
        match variant {
            CM33_DBGEN_A::DISABLE => 1,
            CM33_DBGEN_A::ENABLE => 2,
        }
    }
}
#[doc = "Reader of field `CM33_DBGEN`"]
pub type CM33_DBGEN_R = crate::R<u8, CM33_DBGEN_A>;
impl CM33_DBGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CM33_DBGEN_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CM33_DBGEN_A::DISABLE),
            2 => Val(CM33_DBGEN_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CM33_DBGEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CM33_DBGEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `CM33_DBGEN`"]
pub struct CM33_DBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CM33_DBGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM33_DBGEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CM33_DBGEN_A::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CM33_DBGEN_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `CM33_NIDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM33_NIDEN_A {
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE,
}
impl From<CM33_NIDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CM33_NIDEN_A) -> Self {
        match variant {
            CM33_NIDEN_A::DISABLE => 1,
            CM33_NIDEN_A::ENABLE => 2,
        }
    }
}
#[doc = "Reader of field `CM33_NIDEN`"]
pub type CM33_NIDEN_R = crate::R<u8, CM33_NIDEN_A>;
impl CM33_NIDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CM33_NIDEN_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CM33_NIDEN_A::DISABLE),
            2 => Val(CM33_NIDEN_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CM33_NIDEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CM33_NIDEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `CM33_NIDEN`"]
pub struct CM33_NIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CM33_NIDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM33_NIDEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CM33_NIDEN_A::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CM33_NIDEN_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `CM33_SPIDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM33_SPIDEN_A {
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE,
}
impl From<CM33_SPIDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CM33_SPIDEN_A) -> Self {
        match variant {
            CM33_SPIDEN_A::DISABLE => 1,
            CM33_SPIDEN_A::ENABLE => 2,
        }
    }
}
#[doc = "Reader of field `CM33_SPIDEN`"]
pub type CM33_SPIDEN_R = crate::R<u8, CM33_SPIDEN_A>;
impl CM33_SPIDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CM33_SPIDEN_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CM33_SPIDEN_A::DISABLE),
            2 => Val(CM33_SPIDEN_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CM33_SPIDEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CM33_SPIDEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `CM33_SPIDEN`"]
pub struct CM33_SPIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CM33_SPIDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM33_SPIDEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CM33_SPIDEN_A::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CM33_SPIDEN_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `CM33_SPNIDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM33_SPNIDEN_A {
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE,
}
impl From<CM33_SPNIDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CM33_SPNIDEN_A) -> Self {
        match variant {
            CM33_SPNIDEN_A::DISABLE => 1,
            CM33_SPNIDEN_A::ENABLE => 2,
        }
    }
}
#[doc = "Reader of field `CM33_SPNIDEN`"]
pub type CM33_SPNIDEN_R = crate::R<u8, CM33_SPNIDEN_A>;
impl CM33_SPNIDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CM33_SPNIDEN_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CM33_SPNIDEN_A::DISABLE),
            2 => Val(CM33_SPNIDEN_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CM33_SPNIDEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CM33_SPNIDEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `CM33_SPNIDEN`"]
pub struct CM33_SPNIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CM33_SPNIDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM33_SPNIDEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CM33_SPNIDEN_A::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CM33_SPNIDEN_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `MCM33_DBGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCM33_DBGEN_A {
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE,
}
impl From<MCM33_DBGEN_A> for u8 {
    #[inline(always)]
    fn from(variant: MCM33_DBGEN_A) -> Self {
        match variant {
            MCM33_DBGEN_A::DISABLE => 1,
            MCM33_DBGEN_A::ENABLE => 2,
        }
    }
}
#[doc = "Reader of field `MCM33_DBGEN`"]
pub type MCM33_DBGEN_R = crate::R<u8, MCM33_DBGEN_A>;
impl MCM33_DBGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MCM33_DBGEN_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(MCM33_DBGEN_A::DISABLE),
            2 => Val(MCM33_DBGEN_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MCM33_DBGEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MCM33_DBGEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `MCM33_DBGEN`"]
pub struct MCM33_DBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCM33_DBGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCM33_DBGEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MCM33_DBGEN_A::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MCM33_DBGEN_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `MCM33_NIDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCM33_NIDEN_A {
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE,
}
impl From<MCM33_NIDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: MCM33_NIDEN_A) -> Self {
        match variant {
            MCM33_NIDEN_A::DISABLE => 1,
            MCM33_NIDEN_A::ENABLE => 2,
        }
    }
}
#[doc = "Reader of field `MCM33_NIDEN`"]
pub type MCM33_NIDEN_R = crate::R<u8, MCM33_NIDEN_A>;
impl MCM33_NIDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MCM33_NIDEN_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(MCM33_NIDEN_A::DISABLE),
            2 => Val(MCM33_NIDEN_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MCM33_NIDEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MCM33_NIDEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `MCM33_NIDEN`"]
pub struct MCM33_NIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCM33_NIDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCM33_NIDEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MCM33_NIDEN_A::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MCM33_NIDEN_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CM33 (CPU0) Invasive debug control:."]
    #[inline(always)]
    pub fn cm33_dbgen(&self) -> CM33_DBGEN_R {
        CM33_DBGEN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - CM33 (CPU0) Non Invasive debug control:."]
    #[inline(always)]
    pub fn cm33_niden(&self) -> CM33_NIDEN_R {
        CM33_NIDEN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - CM33 (CPU0) Secure Invasive debug control:."]
    #[inline(always)]
    pub fn cm33_spiden(&self) -> CM33_SPIDEN_R {
        CM33_SPIDEN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - CM33 (CPU0) Secure Non Invasive debug control:."]
    #[inline(always)]
    pub fn cm33_spniden(&self) -> CM33_SPNIDEN_R {
        CM33_SPNIDEN_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Micro-CM33 (CPU1) Invasive debug control:."]
    #[inline(always)]
    pub fn mcm33_dbgen(&self) -> MCM33_DBGEN_R {
        MCM33_DBGEN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Micro-CM33 (CPU1) Non Invasive debug control:."]
    #[inline(always)]
    pub fn mcm33_niden(&self) -> MCM33_NIDEN_R {
        MCM33_NIDEN_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CM33 (CPU0) Invasive debug control:."]
    #[inline(always)]
    pub fn cm33_dbgen(&mut self) -> CM33_DBGEN_W {
        CM33_DBGEN_W { w: self }
    }
    #[doc = "Bits 2:3 - CM33 (CPU0) Non Invasive debug control:."]
    #[inline(always)]
    pub fn cm33_niden(&mut self) -> CM33_NIDEN_W {
        CM33_NIDEN_W { w: self }
    }
    #[doc = "Bits 4:5 - CM33 (CPU0) Secure Invasive debug control:."]
    #[inline(always)]
    pub fn cm33_spiden(&mut self) -> CM33_SPIDEN_W {
        CM33_SPIDEN_W { w: self }
    }
    #[doc = "Bits 6:7 - CM33 (CPU0) Secure Non Invasive debug control:."]
    #[inline(always)]
    pub fn cm33_spniden(&mut self) -> CM33_SPNIDEN_W {
        CM33_SPNIDEN_W { w: self }
    }
    #[doc = "Bits 8:9 - Micro-CM33 (CPU1) Invasive debug control:."]
    #[inline(always)]
    pub fn mcm33_dbgen(&mut self) -> MCM33_DBGEN_W {
        MCM33_DBGEN_W { w: self }
    }
    #[doc = "Bits 10:11 - Micro-CM33 (CPU1) Non Invasive debug control:."]
    #[inline(always)]
    pub fn mcm33_niden(&mut self) -> MCM33_NIDEN_W {
        MCM33_NIDEN_W { w: self }
    }
}
