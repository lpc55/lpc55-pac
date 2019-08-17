#[doc = "Reader of register RINGO0_CTRL"]
pub type R = crate::R<u32, super::RINGO0_CTRL>;
#[doc = "Writer for register RINGO0_CTRL"]
pub type W = crate::W<u32, super::RINGO0_CTRL>;
#[doc = "Register RINGO0_CTRL `reset()`'s with value 0x40"]
impl crate::ResetValue for super::RINGO0_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x40
    }
}
#[doc = "Possible values of the field `SL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SL_A {
    #[doc = "Select short ringo (few elements)."]
    SHORT,
    #[doc = "Select long ringo (many elements)."]
    LONG,
}
impl From<SL_A> for bool {
    #[inline(always)]
    fn from(variant: SL_A) -> Self {
        match variant {
            SL_A::SHORT => false,
            SL_A::LONG => true,
        }
    }
}
#[doc = "Reader of field `SL`"]
pub type SL_R = crate::R<bool, SL_A>;
impl SL_R {
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
        *self == SL_A::SHORT
    }
    #[doc = "Checks if the value of the field is `LONG`"]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == SL_A::LONG
    }
}
#[doc = "Write proxy for field `SL`"]
pub struct SL_W<'a> {
    w: &'a mut W,
}
impl<'a> SL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Possible values of the field `FS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FS_A {
    #[doc = "High frequency output (frequency lower than 100 MHz)."]
    FAST,
    #[doc = "Low frequency output (frequency lower than 10 MHz)."]
    SLOW,
}
impl From<FS_A> for bool {
    #[inline(always)]
    fn from(variant: FS_A) -> Self {
        match variant {
            FS_A::FAST => false,
            FS_A::SLOW => true,
        }
    }
}
#[doc = "Reader of field `FS`"]
pub type FS_R = crate::R<bool, FS_A>;
impl FS_R {
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
        *self == FS_A::FAST
    }
    #[doc = "Checks if the value of the field is `SLOW`"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == FS_A::SLOW
    }
}
#[doc = "Write proxy for field `FS`"]
pub struct FS_W<'a> {
    w: &'a mut W,
}
impl<'a> FS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `SWN_SWP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWN_SWP_A {
    #[doc = "Normal mode."]
    NORMAL,
    #[doc = "P-Monitor mode. Measure with weak P transistor."]
    P_MONITOR,
    #[doc = "P-Monitor mode. Measure with weak N transistor."]
    N_MONITOR,
    #[doc = "Don't use."]
    FORBIDDEN,
}
impl From<SWN_SWP_A> for u8 {
    #[inline(always)]
    fn from(variant: SWN_SWP_A) -> Self {
        match variant {
            SWN_SWP_A::NORMAL => 0,
            SWN_SWP_A::P_MONITOR => 1,
            SWN_SWP_A::N_MONITOR => 2,
            SWN_SWP_A::FORBIDDEN => 3,
        }
    }
}
#[doc = "Reader of field `SWN_SWP`"]
pub type SWN_SWP_R = crate::R<u8, SWN_SWP_A>;
impl SWN_SWP_R {
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
        *self == SWN_SWP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `P_MONITOR`"]
    #[inline(always)]
    pub fn is_p_monitor(&self) -> bool {
        *self == SWN_SWP_A::P_MONITOR
    }
    #[doc = "Checks if the value of the field is `N_MONITOR`"]
    #[inline(always)]
    pub fn is_n_monitor(&self) -> bool {
        *self == SWN_SWP_A::N_MONITOR
    }
    #[doc = "Checks if the value of the field is `FORBIDDEN`"]
    #[inline(always)]
    pub fn is_forbidden(&self) -> bool {
        *self == SWN_SWP_A::FORBIDDEN
    }
}
#[doc = "Write proxy for field `SWN_SWP`"]
pub struct SWN_SWP_W<'a> {
    w: &'a mut W,
}
impl<'a> SWN_SWP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWN_SWP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_A {
    #[doc = "The Ringo module is enabled."]
    POWERED_ON,
    #[doc = "The Ringo module is disabled."]
    POWERED_DOWN,
}
impl From<PD_A> for bool {
    #[inline(always)]
    fn from(variant: PD_A) -> Self {
        match variant {
            PD_A::POWERED_ON => false,
            PD_A::POWERED_DOWN => true,
        }
    }
}
#[doc = "Reader of field `PD`"]
pub type PD_R = crate::R<bool, PD_A>;
impl PD_R {
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
        *self == PD_A::POWERED_ON
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == PD_A::POWERED_DOWN
    }
}
#[doc = "Write proxy for field `PD`"]
pub struct PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `E_ND0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_ND0_A {
    #[doc = "First NAND2-based ringo is disabled."]
    DISABLE,
    #[doc = "First NAND2-based ringo is enabled."]
    ENABLE,
}
impl From<E_ND0_A> for bool {
    #[inline(always)]
    fn from(variant: E_ND0_A) -> Self {
        match variant {
            E_ND0_A::DISABLE => false,
            E_ND0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `E_ND0`"]
pub type E_ND0_R = crate::R<bool, E_ND0_A>;
impl E_ND0_R {
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
        *self == E_ND0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_ND0_A::ENABLE
    }
}
#[doc = "Write proxy for field `E_ND0`"]
pub struct E_ND0_W<'a> {
    w: &'a mut W,
}
impl<'a> E_ND0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_ND0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Possible values of the field `E_ND1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_ND1_A {
    #[doc = "Second NAND2-based ringo is disabled."]
    DISABLE,
    #[doc = "Second NAND2-based ringo is enabled."]
    ENABLE,
}
impl From<E_ND1_A> for bool {
    #[inline(always)]
    fn from(variant: E_ND1_A) -> Self {
        match variant {
            E_ND1_A::DISABLE => false,
            E_ND1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `E_ND1`"]
pub type E_ND1_R = crate::R<bool, E_ND1_A>;
impl E_ND1_R {
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
        *self == E_ND1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_ND1_A::ENABLE
    }
}
#[doc = "Write proxy for field `E_ND1`"]
pub struct E_ND1_W<'a> {
    w: &'a mut W,
}
impl<'a> E_ND1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_ND1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `E_NR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_NR0_A {
    #[doc = "First NOR2-based ringo is disabled."]
    DISABLE,
    #[doc = "First NOR2-based ringo is enabled."]
    ENABLE,
}
impl From<E_NR0_A> for bool {
    #[inline(always)]
    fn from(variant: E_NR0_A) -> Self {
        match variant {
            E_NR0_A::DISABLE => false,
            E_NR0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `E_NR0`"]
pub type E_NR0_R = crate::R<bool, E_NR0_A>;
impl E_NR0_R {
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
        *self == E_NR0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_NR0_A::ENABLE
    }
}
#[doc = "Write proxy for field `E_NR0`"]
pub struct E_NR0_W<'a> {
    w: &'a mut W,
}
impl<'a> E_NR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_NR0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Possible values of the field `E_NR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_NR1_A {
    #[doc = "Second NORD2-based ringo is disabled."]
    DISABLE,
    #[doc = "Second NORD2-based ringo is enabled."]
    ENABLE,
}
impl From<E_NR1_A> for bool {
    #[inline(always)]
    fn from(variant: E_NR1_A) -> Self {
        match variant {
            E_NR1_A::DISABLE => false,
            E_NR1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `E_NR1`"]
pub type E_NR1_R = crate::R<bool, E_NR1_A>;
impl E_NR1_R {
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
        *self == E_NR1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_NR1_A::ENABLE
    }
}
#[doc = "Write proxy for field `E_NR1`"]
pub struct E_NR1_W<'a> {
    w: &'a mut W,
}
impl<'a> E_NR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_NR1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `E_IV0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_IV0_A {
    #[doc = "First INV-based ringo is disabled."]
    DISABLE,
    #[doc = "First INV-based ringo is enabled."]
    ENABLE,
}
impl From<E_IV0_A> for bool {
    #[inline(always)]
    fn from(variant: E_IV0_A) -> Self {
        match variant {
            E_IV0_A::DISABLE => false,
            E_IV0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `E_IV0`"]
pub type E_IV0_R = crate::R<bool, E_IV0_A>;
impl E_IV0_R {
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
        *self == E_IV0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_IV0_A::ENABLE
    }
}
#[doc = "Write proxy for field `E_IV0`"]
pub struct E_IV0_W<'a> {
    w: &'a mut W,
}
impl<'a> E_IV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_IV0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Possible values of the field `E_IV1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_IV1_A {
    #[doc = "Second INV-based ringo is disabled."]
    DISABLE,
    #[doc = "Second INV-based ringo is enabled."]
    ENABLE,
}
impl From<E_IV1_A> for bool {
    #[inline(always)]
    fn from(variant: E_IV1_A) -> Self {
        match variant {
            E_IV1_A::DISABLE => false,
            E_IV1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `E_IV1`"]
pub type E_IV1_R = crate::R<bool, E_IV1_A>;
impl E_IV1_R {
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
        *self == E_IV1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_IV1_A::ENABLE
    }
}
#[doc = "Write proxy for field `E_IV1`"]
pub struct E_IV1_W<'a> {
    w: &'a mut W,
}
impl<'a> E_IV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_IV1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `E_PN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_PN0_A {
    #[doc = "First PN-based ringo is disabled."]
    DISABLE,
    #[doc = "First PN-based ringo is enabled."]
    ENABLE,
}
impl From<E_PN0_A> for bool {
    #[inline(always)]
    fn from(variant: E_PN0_A) -> Self {
        match variant {
            E_PN0_A::DISABLE => false,
            E_PN0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `E_PN0`"]
pub type E_PN0_R = crate::R<bool, E_PN0_A>;
impl E_PN0_R {
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
        *self == E_PN0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_PN0_A::ENABLE
    }
}
#[doc = "Write proxy for field `E_PN0`"]
pub struct E_PN0_W<'a> {
    w: &'a mut W,
}
impl<'a> E_PN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_PN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Possible values of the field `E_PN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_PN1_A {
    #[doc = "Second PN-based ringo is disabled."]
    DISABLE,
    #[doc = "Second PN-based ringo is enabled."]
    ENABLE,
}
impl From<E_PN1_A> for bool {
    #[inline(always)]
    fn from(variant: E_PN1_A) -> Self {
        match variant {
            E_PN1_A::DISABLE => false,
            E_PN1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `E_PN1`"]
pub type E_PN1_R = crate::R<bool, E_PN1_A>;
impl E_PN1_R {
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
        *self == E_PN1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_PN1_A::ENABLE
    }
}
#[doc = "Write proxy for field `E_PN1`"]
pub struct E_PN1_W<'a> {
    w: &'a mut W,
}
impl<'a> E_PN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_PN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `DIVISOR`"]
pub type DIVISOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVISOR`"]
pub struct DIVISOR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVISOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DIV_UPDATE_REQ`"]
pub type DIV_UPDATE_REQ_R = crate::R<bool, bool>;
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
}
