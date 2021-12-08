#[doc = "Register `RINGO0_CTRL` reader"]
pub struct R(crate::R<RINGO0_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RINGO0_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RINGO0_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RINGO0_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RINGO0_CTRL` writer"]
pub struct W(crate::W<RINGO0_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RINGO0_CTRL_SPEC>;
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
impl From<crate::W<RINGO0_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RINGO0_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select short or long ringo (for all ringos types).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SL_A {
    #[doc = "0: Select short ringo (few elements)."]
    SHORT = 0,
    #[doc = "1: Select long ringo (many elements)."]
    LONG = 1,
}
impl From<SL_A> for bool {
    #[inline(always)]
    fn from(variant: SL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SL` reader - Select short or long ringo (for all ringos types)."]
pub struct SL_R(crate::FieldReader<bool, SL_A>);
impl SL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SL_A {
        match self.bits {
            false => SL_A::SHORT,
            true => SL_A::LONG,
        }
    }
    #[doc = "Checks if the value of the field is `SHORT`"]
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        **self == SL_A::SHORT
    }
    #[doc = "Checks if the value of the field is `LONG`"]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        **self == SL_A::LONG
    }
}
impl core::ops::Deref for SL_R {
    type Target = crate::FieldReader<bool, SL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SL` writer - Select short or long ringo (for all ringos types)."]
pub struct SL_W<'a> {
    w: &'a mut W,
}
impl<'a> SL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select short ringo (few elements)."]
    #[inline(always)]
    pub fn short(self) -> &'a mut W {
        self.variant(SL_A::SHORT)
    }
    #[doc = "Select long ringo (many elements)."]
    #[inline(always)]
    pub fn long(self) -> &'a mut W {
        self.variant(SL_A::LONG)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Ringo frequency output divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FS_A {
    #[doc = "0: High frequency output (frequency lower than 100 MHz)."]
    FAST = 0,
    #[doc = "1: Low frequency output (frequency lower than 10 MHz)."]
    SLOW = 1,
}
impl From<FS_A> for bool {
    #[inline(always)]
    fn from(variant: FS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FS` reader - Ringo frequency output divider."]
pub struct FS_R(crate::FieldReader<bool, FS_A>);
impl FS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FS_A {
        match self.bits {
            false => FS_A::FAST,
            true => FS_A::SLOW,
        }
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        **self == FS_A::FAST
    }
    #[doc = "Checks if the value of the field is `SLOW`"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        **self == FS_A::SLOW
    }
}
impl core::ops::Deref for FS_R {
    type Target = crate::FieldReader<bool, FS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FS` writer - Ringo frequency output divider."]
pub struct FS_W<'a> {
    w: &'a mut W,
}
impl<'a> FS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "High frequency output (frequency lower than 100 MHz)."]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(FS_A::FAST)
    }
    #[doc = "Low frequency output (frequency lower than 10 MHz)."]
    #[inline(always)]
    pub fn slow(self) -> &'a mut W {
        self.variant(FS_A::SLOW)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "PN-Ringos (P-Transistor and N-Transistor processing) control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWN_SWP_A {
    #[doc = "0: Normal mode."]
    NORMAL = 0,
    #[doc = "1: P-Monitor mode. Measure with weak P transistor."]
    P_MONITOR = 1,
    #[doc = "2: P-Monitor mode. Measure with weak N transistor."]
    N_MONITOR = 2,
    #[doc = "3: Don't use."]
    FORBIDDEN = 3,
}
impl From<SWN_SWP_A> for u8 {
    #[inline(always)]
    fn from(variant: SWN_SWP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SWN_SWP` reader - PN-Ringos (P-Transistor and N-Transistor processing) control."]
pub struct SWN_SWP_R(crate::FieldReader<u8, SWN_SWP_A>);
impl SWN_SWP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SWN_SWP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWN_SWP_A {
        match self.bits {
            0 => SWN_SWP_A::NORMAL,
            1 => SWN_SWP_A::P_MONITOR,
            2 => SWN_SWP_A::N_MONITOR,
            3 => SWN_SWP_A::FORBIDDEN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == SWN_SWP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `P_MONITOR`"]
    #[inline(always)]
    pub fn is_p_monitor(&self) -> bool {
        **self == SWN_SWP_A::P_MONITOR
    }
    #[doc = "Checks if the value of the field is `N_MONITOR`"]
    #[inline(always)]
    pub fn is_n_monitor(&self) -> bool {
        **self == SWN_SWP_A::N_MONITOR
    }
    #[doc = "Checks if the value of the field is `FORBIDDEN`"]
    #[inline(always)]
    pub fn is_forbidden(&self) -> bool {
        **self == SWN_SWP_A::FORBIDDEN
    }
}
impl core::ops::Deref for SWN_SWP_R {
    type Target = crate::FieldReader<u8, SWN_SWP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWN_SWP` writer - PN-Ringos (P-Transistor and N-Transistor processing) control."]
pub struct SWN_SWP_W<'a> {
    w: &'a mut W,
}
impl<'a> SWN_SWP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWN_SWP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SWN_SWP_A::NORMAL)
    }
    #[doc = "P-Monitor mode. Measure with weak P transistor."]
    #[inline(always)]
    pub fn p_monitor(self) -> &'a mut W {
        self.variant(SWN_SWP_A::P_MONITOR)
    }
    #[doc = "P-Monitor mode. Measure with weak N transistor."]
    #[inline(always)]
    pub fn n_monitor(self) -> &'a mut W {
        self.variant(SWN_SWP_A::N_MONITOR)
    }
    #[doc = "Don't use."]
    #[inline(always)]
    pub fn forbidden(self) -> &'a mut W {
        self.variant(SWN_SWP_A::FORBIDDEN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Ringo module Power control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_A {
    #[doc = "0: The Ringo module is enabled."]
    POWERED_ON = 0,
    #[doc = "1: The Ringo module is disabled."]
    POWERED_DOWN = 1,
}
impl From<PD_A> for bool {
    #[inline(always)]
    fn from(variant: PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD` reader - Ringo module Power control."]
pub struct PD_R(crate::FieldReader<bool, PD_A>);
impl PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_A {
        match self.bits {
            false => PD_A::POWERED_ON,
            true => PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED_ON`"]
    #[inline(always)]
    pub fn is_powered_on(&self) -> bool {
        **self == PD_A::POWERED_ON
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        **self == PD_A::POWERED_DOWN
    }
}
impl core::ops::Deref for PD_R {
    type Target = crate::FieldReader<bool, PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD` writer - Ringo module Power control."]
pub struct PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The Ringo module is enabled."]
    #[inline(always)]
    pub fn powered_on(self) -> &'a mut W {
        self.variant(PD_A::POWERED_ON)
    }
    #[doc = "The Ringo module is disabled."]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(PD_A::POWERED_DOWN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "First NAND2-based ringo control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_ND0_A {
    #[doc = "0: First NAND2-based ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: First NAND2-based ringo is enabled."]
    ENABLE = 1,
}
impl From<E_ND0_A> for bool {
    #[inline(always)]
    fn from(variant: E_ND0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_ND0` reader - First NAND2-based ringo control."]
pub struct E_ND0_R(crate::FieldReader<bool, E_ND0_A>);
impl E_ND0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        E_ND0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_ND0_A {
        match self.bits {
            false => E_ND0_A::DISABLE,
            true => E_ND0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == E_ND0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == E_ND0_A::ENABLE
    }
}
impl core::ops::Deref for E_ND0_R {
    type Target = crate::FieldReader<bool, E_ND0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E_ND0` writer - First NAND2-based ringo control."]
pub struct E_ND0_W<'a> {
    w: &'a mut W,
}
impl<'a> E_ND0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_ND0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "First NAND2-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_ND0_A::DISABLE)
    }
    #[doc = "First NAND2-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_ND0_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Second NAND2-based ringo control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_ND1_A {
    #[doc = "0: Second NAND2-based ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: Second NAND2-based ringo is enabled."]
    ENABLE = 1,
}
impl From<E_ND1_A> for bool {
    #[inline(always)]
    fn from(variant: E_ND1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_ND1` reader - Second NAND2-based ringo control."]
pub struct E_ND1_R(crate::FieldReader<bool, E_ND1_A>);
impl E_ND1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        E_ND1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_ND1_A {
        match self.bits {
            false => E_ND1_A::DISABLE,
            true => E_ND1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == E_ND1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == E_ND1_A::ENABLE
    }
}
impl core::ops::Deref for E_ND1_R {
    type Target = crate::FieldReader<bool, E_ND1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E_ND1` writer - Second NAND2-based ringo control."]
pub struct E_ND1_W<'a> {
    w: &'a mut W,
}
impl<'a> E_ND1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_ND1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Second NAND2-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_ND1_A::DISABLE)
    }
    #[doc = "Second NAND2-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_ND1_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "First NOR2-based ringo control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_NR0_A {
    #[doc = "0: First NOR2-based ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: First NOR2-based ringo is enabled."]
    ENABLE = 1,
}
impl From<E_NR0_A> for bool {
    #[inline(always)]
    fn from(variant: E_NR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_NR0` reader - First NOR2-based ringo control."]
pub struct E_NR0_R(crate::FieldReader<bool, E_NR0_A>);
impl E_NR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        E_NR0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_NR0_A {
        match self.bits {
            false => E_NR0_A::DISABLE,
            true => E_NR0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == E_NR0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == E_NR0_A::ENABLE
    }
}
impl core::ops::Deref for E_NR0_R {
    type Target = crate::FieldReader<bool, E_NR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E_NR0` writer - First NOR2-based ringo control."]
pub struct E_NR0_W<'a> {
    w: &'a mut W,
}
impl<'a> E_NR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_NR0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "First NOR2-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_NR0_A::DISABLE)
    }
    #[doc = "First NOR2-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_NR0_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Second NOR2-based ringo control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_NR1_A {
    #[doc = "0: Second NORD2-based ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: Second NORD2-based ringo is enabled."]
    ENABLE = 1,
}
impl From<E_NR1_A> for bool {
    #[inline(always)]
    fn from(variant: E_NR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_NR1` reader - Second NOR2-based ringo control."]
pub struct E_NR1_R(crate::FieldReader<bool, E_NR1_A>);
impl E_NR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        E_NR1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_NR1_A {
        match self.bits {
            false => E_NR1_A::DISABLE,
            true => E_NR1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == E_NR1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == E_NR1_A::ENABLE
    }
}
impl core::ops::Deref for E_NR1_R {
    type Target = crate::FieldReader<bool, E_NR1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E_NR1` writer - Second NOR2-based ringo control."]
pub struct E_NR1_W<'a> {
    w: &'a mut W,
}
impl<'a> E_NR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_NR1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Second NORD2-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_NR1_A::DISABLE)
    }
    #[doc = "Second NORD2-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_NR1_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "First Inverter-based ringo control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_IV0_A {
    #[doc = "0: First INV-based ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: First INV-based ringo is enabled."]
    ENABLE = 1,
}
impl From<E_IV0_A> for bool {
    #[inline(always)]
    fn from(variant: E_IV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_IV0` reader - First Inverter-based ringo control."]
pub struct E_IV0_R(crate::FieldReader<bool, E_IV0_A>);
impl E_IV0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        E_IV0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_IV0_A {
        match self.bits {
            false => E_IV0_A::DISABLE,
            true => E_IV0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == E_IV0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == E_IV0_A::ENABLE
    }
}
impl core::ops::Deref for E_IV0_R {
    type Target = crate::FieldReader<bool, E_IV0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E_IV0` writer - First Inverter-based ringo control."]
pub struct E_IV0_W<'a> {
    w: &'a mut W,
}
impl<'a> E_IV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_IV0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "First INV-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_IV0_A::DISABLE)
    }
    #[doc = "First INV-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_IV0_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Second Inverter-based ringo control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_IV1_A {
    #[doc = "0: Second INV-based ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: Second INV-based ringo is enabled."]
    ENABLE = 1,
}
impl From<E_IV1_A> for bool {
    #[inline(always)]
    fn from(variant: E_IV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_IV1` reader - Second Inverter-based ringo control."]
pub struct E_IV1_R(crate::FieldReader<bool, E_IV1_A>);
impl E_IV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        E_IV1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_IV1_A {
        match self.bits {
            false => E_IV1_A::DISABLE,
            true => E_IV1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == E_IV1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == E_IV1_A::ENABLE
    }
}
impl core::ops::Deref for E_IV1_R {
    type Target = crate::FieldReader<bool, E_IV1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E_IV1` writer - Second Inverter-based ringo control."]
pub struct E_IV1_W<'a> {
    w: &'a mut W,
}
impl<'a> E_IV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_IV1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Second INV-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_IV1_A::DISABLE)
    }
    #[doc = "Second INV-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_IV1_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "First PN (P-Transistor and N-Transistor processing) monitor control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_PN0_A {
    #[doc = "0: First PN-based ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: First PN-based ringo is enabled."]
    ENABLE = 1,
}
impl From<E_PN0_A> for bool {
    #[inline(always)]
    fn from(variant: E_PN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_PN0` reader - First PN (P-Transistor and N-Transistor processing) monitor control."]
pub struct E_PN0_R(crate::FieldReader<bool, E_PN0_A>);
impl E_PN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        E_PN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_PN0_A {
        match self.bits {
            false => E_PN0_A::DISABLE,
            true => E_PN0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == E_PN0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == E_PN0_A::ENABLE
    }
}
impl core::ops::Deref for E_PN0_R {
    type Target = crate::FieldReader<bool, E_PN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E_PN0` writer - First PN (P-Transistor and N-Transistor processing) monitor control."]
pub struct E_PN0_W<'a> {
    w: &'a mut W,
}
impl<'a> E_PN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_PN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "First PN-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_PN0_A::DISABLE)
    }
    #[doc = "First PN-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_PN0_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Second PN (P-Transistor and N-Transistor processing) monitor control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_PN1_A {
    #[doc = "0: Second PN-based ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: Second PN-based ringo is enabled."]
    ENABLE = 1,
}
impl From<E_PN1_A> for bool {
    #[inline(always)]
    fn from(variant: E_PN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_PN1` reader - Second PN (P-Transistor and N-Transistor processing) monitor control."]
pub struct E_PN1_R(crate::FieldReader<bool, E_PN1_A>);
impl E_PN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        E_PN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_PN1_A {
        match self.bits {
            false => E_PN1_A::DISABLE,
            true => E_PN1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == E_PN1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == E_PN1_A::ENABLE
    }
}
impl core::ops::Deref for E_PN1_R {
    type Target = crate::FieldReader<bool, E_PN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E_PN1` writer - Second PN (P-Transistor and N-Transistor processing) monitor control."]
pub struct E_PN1_W<'a> {
    w: &'a mut W,
}
impl<'a> E_PN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_PN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Second PN-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_PN1_A::DISABLE)
    }
    #[doc = "Second PN-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_PN1_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `DIVISOR` reader - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
pub struct DIVISOR_R(crate::FieldReader<u8, u8>);
impl DIVISOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIVISOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVISOR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVISOR` writer - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
pub struct DIVISOR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVISOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `DIV_UPDATE_REQ` reader - Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
pub struct DIV_UPDATE_REQ_R(crate::FieldReader<bool, bool>);
impl DIV_UPDATE_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIV_UPDATE_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_UPDATE_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Select short or long ringo (for all ringos types)."]
    #[inline(always)]
    pub fn sl(&self) -> SL_R {
        SL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Ringo frequency output divider."]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - PN-Ringos (P-Transistor and N-Transistor processing) control."]
    #[inline(always)]
    pub fn swn_swp(&self) -> SWN_SWP_R {
        SWN_SWP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Ringo module Power control."]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - First NAND2-based ringo control."]
    #[inline(always)]
    pub fn e_nd0(&self) -> E_ND0_R {
        E_ND0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Second NAND2-based ringo control."]
    #[inline(always)]
    pub fn e_nd1(&self) -> E_ND1_R {
        E_ND1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - First NOR2-based ringo control."]
    #[inline(always)]
    pub fn e_nr0(&self) -> E_NR0_R {
        E_NR0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Second NOR2-based ringo control."]
    #[inline(always)]
    pub fn e_nr1(&self) -> E_NR1_R {
        E_NR1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - First Inverter-based ringo control."]
    #[inline(always)]
    pub fn e_iv0(&self) -> E_IV0_R {
        E_IV0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Second Inverter-based ringo control."]
    #[inline(always)]
    pub fn e_iv1(&self) -> E_IV1_R {
        E_IV1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - First PN (P-Transistor and N-Transistor processing) monitor control."]
    #[inline(always)]
    pub fn e_pn0(&self) -> E_PN0_R {
        E_PN0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Second PN (P-Transistor and N-Transistor processing) monitor control."]
    #[inline(always)]
    pub fn e_pn1(&self) -> E_PN1_R {
        E_PN1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
    #[inline(always)]
    pub fn divisor(&self) -> DIVISOR_R {
        DIVISOR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn div_update_req(&self) -> DIV_UPDATE_REQ_R {
        DIV_UPDATE_REQ_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select short or long ringo (for all ringos types)."]
    #[inline(always)]
    pub fn sl(&mut self) -> SL_W {
        SL_W { w: self }
    }
    #[doc = "Bit 1 - Ringo frequency output divider."]
    #[inline(always)]
    pub fn fs(&mut self) -> FS_W {
        FS_W { w: self }
    }
    #[doc = "Bits 2:3 - PN-Ringos (P-Transistor and N-Transistor processing) control."]
    #[inline(always)]
    pub fn swn_swp(&mut self) -> SWN_SWP_W {
        SWN_SWP_W { w: self }
    }
    #[doc = "Bit 4 - Ringo module Power control."]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W { w: self }
    }
    #[doc = "Bit 5 - First NAND2-based ringo control."]
    #[inline(always)]
    pub fn e_nd0(&mut self) -> E_ND0_W {
        E_ND0_W { w: self }
    }
    #[doc = "Bit 6 - Second NAND2-based ringo control."]
    #[inline(always)]
    pub fn e_nd1(&mut self) -> E_ND1_W {
        E_ND1_W { w: self }
    }
    #[doc = "Bit 7 - First NOR2-based ringo control."]
    #[inline(always)]
    pub fn e_nr0(&mut self) -> E_NR0_W {
        E_NR0_W { w: self }
    }
    #[doc = "Bit 8 - Second NOR2-based ringo control."]
    #[inline(always)]
    pub fn e_nr1(&mut self) -> E_NR1_W {
        E_NR1_W { w: self }
    }
    #[doc = "Bit 9 - First Inverter-based ringo control."]
    #[inline(always)]
    pub fn e_iv0(&mut self) -> E_IV0_W {
        E_IV0_W { w: self }
    }
    #[doc = "Bit 10 - Second Inverter-based ringo control."]
    #[inline(always)]
    pub fn e_iv1(&mut self) -> E_IV1_W {
        E_IV1_W { w: self }
    }
    #[doc = "Bit 11 - First PN (P-Transistor and N-Transistor processing) monitor control."]
    #[inline(always)]
    pub fn e_pn0(&mut self) -> E_PN0_W {
        E_PN0_W { w: self }
    }
    #[doc = "Bit 12 - Second PN (P-Transistor and N-Transistor processing) monitor control."]
    #[inline(always)]
    pub fn e_pn1(&mut self) -> E_PN1_W {
        E_PN1_W { w: self }
    }
    #[doc = "Bits 16:19 - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
    #[inline(always)]
    pub fn divisor(&mut self) -> DIVISOR_W {
        DIVISOR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "First Ring Oscillator module control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ringo0_ctrl](index.html) module"]
pub struct RINGO0_CTRL_SPEC;
impl crate::RegisterSpec for RINGO0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ringo0_ctrl::R](R) reader structure"]
impl crate::Readable for RINGO0_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ringo0_ctrl::W](W) writer structure"]
impl crate::Writable for RINGO0_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RINGO0_CTRL to value 0x40"]
impl crate::Resettable for RINGO0_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
