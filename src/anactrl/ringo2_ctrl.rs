#[doc = "Reader of register RINGO2_CTRL"]
pub type R = crate::R<u32, super::RINGO2_CTRL>;
#[doc = "Writer for register RINGO2_CTRL"]
pub type W = crate::W<u32, super::RINGO2_CTRL>;
#[doc = "Register RINGO2_CTRL `reset()`'s with value 0x40"]
impl crate::ResetValue for super::RINGO2_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x40
    }
}
#[doc = "Possible values of the field `S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_A {
    #[doc = "Select short ringo (few elements)."]
    SHORT,
    #[doc = "Select long ringo (many elements)."]
    LONG,
}
impl From<S_A> for bool {
    #[inline(always)]
    fn from(variant: S_A) -> Self {
        match variant {
            S_A::SHORT => false,
            S_A::LONG => true,
        }
    }
}
#[doc = "Reader of field `S`"]
pub type S_R = crate::R<bool, S_A>;
impl S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S_A {
        match self.bits {
            false => S_A::SHORT,
            true => S_A::LONG,
        }
    }
    #[doc = "Checks if the value of the field is `SHORT`"]
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        *self == S_A::SHORT
    }
    #[doc = "Checks if the value of the field is `LONG`"]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == S_A::LONG
    }
}
#[doc = "Write proxy for field `S`"]
pub struct S_W<'a> {
    w: &'a mut W,
}
impl<'a> S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Select short ringo (few elements)."]
    #[inline(always)]
    pub fn short(self) -> &'a mut W {
        self.variant(S_A::SHORT)
    }
    #[doc = "Select long ringo (many elements)."]
    #[inline(always)]
    pub fn long(self) -> &'a mut W {
        self.variant(S_A::LONG)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `E_R24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_R24_A {
    #[doc = "Ringo is disabled."]
    DISABLE,
    #[doc = "Ringo is enabled."]
    ENABLE,
}
impl From<E_R24_A> for bool {
    #[inline(always)]
    fn from(variant: E_R24_A) -> Self {
        match variant {
            E_R24_A::DISABLE => false,
            E_R24_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `E_R24`"]
pub type E_R24_R = crate::R<bool, E_R24_A>;
impl E_R24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_R24_A {
        match self.bits {
            false => E_R24_A::DISABLE,
            true => E_R24_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == E_R24_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_R24_A::ENABLE
    }
}
#[doc = "Write proxy for field `E_R24`"]
pub struct E_R24_W<'a> {
    w: &'a mut W,
}
impl<'a> E_R24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_R24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_R24_A::DISABLE)
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_R24_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Possible values of the field `E_R35`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_R35_A {
    #[doc = "Ringo is disabled."]
    DISABLE,
    #[doc = "Ringo is enabled."]
    ENABLE,
}
impl From<E_R35_A> for bool {
    #[inline(always)]
    fn from(variant: E_R35_A) -> Self {
        match variant {
            E_R35_A::DISABLE => false,
            E_R35_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `E_R35`"]
pub type E_R35_R = crate::R<bool, E_R35_A>;
impl E_R35_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_R35_A {
        match self.bits {
            false => E_R35_A::DISABLE,
            true => E_R35_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == E_R35_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_R35_A::ENABLE
    }
}
#[doc = "Write proxy for field `E_R35`"]
pub struct E_R35_W<'a> {
    w: &'a mut W,
}
impl<'a> E_R35_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_R35_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_R35_A::DISABLE)
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_R35_A::ENABLE)
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
#[doc = "Possible values of the field `E_M2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_M2_A {
    #[doc = "Ringo is disabled."]
    DISABLE,
    #[doc = "Ringo is enabled."]
    ENABLE,
}
impl From<E_M2_A> for bool {
    #[inline(always)]
    fn from(variant: E_M2_A) -> Self {
        match variant {
            E_M2_A::DISABLE => false,
            E_M2_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `E_M2`"]
pub type E_M2_R = crate::R<bool, E_M2_A>;
impl E_M2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_M2_A {
        match self.bits {
            false => E_M2_A::DISABLE,
            true => E_M2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == E_M2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_M2_A::ENABLE
    }
}
#[doc = "Write proxy for field `E_M2`"]
pub struct E_M2_W<'a> {
    w: &'a mut W,
}
impl<'a> E_M2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_M2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_M2_A::DISABLE)
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_M2_A::ENABLE)
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
#[doc = "Possible values of the field `E_M3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_M3_A {
    #[doc = "Ringo is disabled."]
    DISABLE,
    #[doc = "Ringo is enabled."]
    ENABLE,
}
impl From<E_M3_A> for bool {
    #[inline(always)]
    fn from(variant: E_M3_A) -> Self {
        match variant {
            E_M3_A::DISABLE => false,
            E_M3_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `E_M3`"]
pub type E_M3_R = crate::R<bool, E_M3_A>;
impl E_M3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_M3_A {
        match self.bits {
            false => E_M3_A::DISABLE,
            true => E_M3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == E_M3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_M3_A::ENABLE
    }
}
#[doc = "Write proxy for field `E_M3`"]
pub struct E_M3_W<'a> {
    w: &'a mut W,
}
impl<'a> E_M3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_M3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_M3_A::DISABLE)
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_M3_A::ENABLE)
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
#[doc = "Possible values of the field `E_M4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_M4_A {
    #[doc = "Ringo is disabled."]
    DISABLE,
    #[doc = "Ringo is enabled."]
    ENABLE,
}
impl From<E_M4_A> for bool {
    #[inline(always)]
    fn from(variant: E_M4_A) -> Self {
        match variant {
            E_M4_A::DISABLE => false,
            E_M4_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `E_M4`"]
pub type E_M4_R = crate::R<bool, E_M4_A>;
impl E_M4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_M4_A {
        match self.bits {
            false => E_M4_A::DISABLE,
            true => E_M4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == E_M4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_M4_A::ENABLE
    }
}
#[doc = "Write proxy for field `E_M4`"]
pub struct E_M4_W<'a> {
    w: &'a mut W,
}
impl<'a> E_M4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_M4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_M4_A::DISABLE)
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_M4_A::ENABLE)
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
#[doc = "Possible values of the field `E_M5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_M5_A {
    #[doc = "Ringo is disabled."]
    DISABLE,
    #[doc = "Ringo is enabled."]
    ENABLE,
}
impl From<E_M5_A> for bool {
    #[inline(always)]
    fn from(variant: E_M5_A) -> Self {
        match variant {
            E_M5_A::DISABLE => false,
            E_M5_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `E_M5`"]
pub type E_M5_R = crate::R<bool, E_M5_A>;
impl E_M5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_M5_A {
        match self.bits {
            false => E_M5_A::DISABLE,
            true => E_M5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == E_M5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_M5_A::ENABLE
    }
}
#[doc = "Write proxy for field `E_M5`"]
pub struct E_M5_W<'a> {
    w: &'a mut W,
}
impl<'a> E_M5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_M5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_M5_A::DISABLE)
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_M5_A::ENABLE)
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
    pub fn s(&self) -> S_R {
        S_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Ringo frequency output divider."]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Ringo module Power control."]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ."]
    #[inline(always)]
    pub fn e_r24(&self) -> E_R24_R {
        E_R24_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ."]
    #[inline(always)]
    pub fn e_r35(&self) -> E_R35_R {
        E_R35_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Metal 2 (M2) monitor control."]
    #[inline(always)]
    pub fn e_m2(&self) -> E_M2_R {
        E_M2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Metal 3 (M3) monitor control."]
    #[inline(always)]
    pub fn e_m3(&self) -> E_M3_R {
        E_M3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Metal 4 (M4) monitor control."]
    #[inline(always)]
    pub fn e_m4(&self) -> E_M4_R {
        E_M4_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Metal 5 (M5) monitor control."]
    #[inline(always)]
    pub fn e_m5(&self) -> E_M5_R {
        E_M5_R::new(((self.bits >> 8) & 0x01) != 0)
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
    pub fn s(&mut self) -> S_W {
        S_W { w: self }
    }
    #[doc = "Bit 1 - Ringo frequency output divider."]
    #[inline(always)]
    pub fn fs(&mut self) -> FS_W {
        FS_W { w: self }
    }
    #[doc = "Bit 2 - Ringo module Power control."]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W { w: self }
    }
    #[doc = "Bit 3 - ."]
    #[inline(always)]
    pub fn e_r24(&mut self) -> E_R24_W {
        E_R24_W { w: self }
    }
    #[doc = "Bit 4 - ."]
    #[inline(always)]
    pub fn e_r35(&mut self) -> E_R35_W {
        E_R35_W { w: self }
    }
    #[doc = "Bit 5 - Metal 2 (M2) monitor control."]
    #[inline(always)]
    pub fn e_m2(&mut self) -> E_M2_W {
        E_M2_W { w: self }
    }
    #[doc = "Bit 6 - Metal 3 (M3) monitor control."]
    #[inline(always)]
    pub fn e_m3(&mut self) -> E_M3_W {
        E_M3_W { w: self }
    }
    #[doc = "Bit 7 - Metal 4 (M4) monitor control."]
    #[inline(always)]
    pub fn e_m4(&mut self) -> E_M4_W {
        E_M4_W { w: self }
    }
    #[doc = "Bit 8 - Metal 5 (M5) monitor control."]
    #[inline(always)]
    pub fn e_m5(&mut self) -> E_M5_W {
        E_M5_W { w: self }
    }
    #[doc = "Bits 16:19 - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
    #[inline(always)]
    pub fn divisor(&mut self) -> DIVISOR_W {
        DIVISOR_W { w: self }
    }
}
