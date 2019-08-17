#[doc = "Reader of register STARTER0"]
pub type R = crate::R<u32, super::STARTER0>;
#[doc = "Writer for register STARTER0"]
pub type W = crate::W<u32, super::STARTER0>;
#[doc = "Register STARTER0 `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTER0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `SYS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYS_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<SYS_A> for bool {
    #[inline(always)]
    fn from(variant: SYS_A) -> Self {
        match variant {
            SYS_A::DISABLE => false,
            SYS_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SYS`"]
pub type SYS_R = crate::R<bool, SYS_A>;
impl SYS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYS_A {
        match self.bits {
            false => SYS_A::DISABLE,
            true => SYS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SYS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SYS_A::ENABLE
    }
}
#[doc = "Write proxy for field `SYS`"]
pub struct SYS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYS_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYS_A::ENABLE)
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
#[doc = "Possible values of the field `SDMA0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMA0_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<SDMA0_A> for bool {
    #[inline(always)]
    fn from(variant: SDMA0_A) -> Self {
        match variant {
            SDMA0_A::DISABLE => false,
            SDMA0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SDMA0`"]
pub type SDMA0_R = crate::R<bool, SDMA0_A>;
impl SDMA0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMA0_A {
        match self.bits {
            false => SDMA0_A::DISABLE,
            true => SDMA0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SDMA0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SDMA0_A::ENABLE
    }
}
#[doc = "Write proxy for field `SDMA0`"]
pub struct SDMA0_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMA0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDMA0_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDMA0_A::ENABLE)
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
#[doc = "Possible values of the field `GINT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GINT0_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<GINT0_A> for bool {
    #[inline(always)]
    fn from(variant: GINT0_A) -> Self {
        match variant {
            GINT0_A::DISABLE => false,
            GINT0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `GINT0`"]
pub type GINT0_R = crate::R<bool, GINT0_A>;
impl GINT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GINT0_A {
        match self.bits {
            false => GINT0_A::DISABLE,
            true => GINT0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GINT0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GINT0_A::ENABLE
    }
}
#[doc = "Write proxy for field `GINT0`"]
pub struct GINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> GINT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GINT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GINT0_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GINT0_A::ENABLE)
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
#[doc = "Possible values of the field `GINT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GINT1_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<GINT1_A> for bool {
    #[inline(always)]
    fn from(variant: GINT1_A) -> Self {
        match variant {
            GINT1_A::DISABLE => false,
            GINT1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `GINT1`"]
pub type GINT1_R = crate::R<bool, GINT1_A>;
impl GINT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GINT1_A {
        match self.bits {
            false => GINT1_A::DISABLE,
            true => GINT1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GINT1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GINT1_A::ENABLE
    }
}
#[doc = "Write proxy for field `GINT1`"]
pub struct GINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> GINT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GINT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GINT1_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GINT1_A::ENABLE)
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
#[doc = "Possible values of the field `PIO_INT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO_INT0_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<PIO_INT0_A> for bool {
    #[inline(always)]
    fn from(variant: PIO_INT0_A) -> Self {
        match variant {
            PIO_INT0_A::DISABLE => false,
            PIO_INT0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `PIO_INT0`"]
pub type PIO_INT0_R = crate::R<bool, PIO_INT0_A>;
impl PIO_INT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO_INT0_A {
        match self.bits {
            false => PIO_INT0_A::DISABLE,
            true => PIO_INT0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PIO_INT0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PIO_INT0_A::ENABLE
    }
}
#[doc = "Write proxy for field `PIO_INT0`"]
pub struct PIO_INT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO_INT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO_INT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PIO_INT0_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PIO_INT0_A::ENABLE)
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
#[doc = "Possible values of the field `PIO_INT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO_INT1_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<PIO_INT1_A> for bool {
    #[inline(always)]
    fn from(variant: PIO_INT1_A) -> Self {
        match variant {
            PIO_INT1_A::DISABLE => false,
            PIO_INT1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `PIO_INT1`"]
pub type PIO_INT1_R = crate::R<bool, PIO_INT1_A>;
impl PIO_INT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO_INT1_A {
        match self.bits {
            false => PIO_INT1_A::DISABLE,
            true => PIO_INT1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PIO_INT1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PIO_INT1_A::ENABLE
    }
}
#[doc = "Write proxy for field `PIO_INT1`"]
pub struct PIO_INT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO_INT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO_INT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PIO_INT1_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PIO_INT1_A::ENABLE)
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
#[doc = "Possible values of the field `PIO_INT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO_INT2_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<PIO_INT2_A> for bool {
    #[inline(always)]
    fn from(variant: PIO_INT2_A) -> Self {
        match variant {
            PIO_INT2_A::DISABLE => false,
            PIO_INT2_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `PIO_INT2`"]
pub type PIO_INT2_R = crate::R<bool, PIO_INT2_A>;
impl PIO_INT2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO_INT2_A {
        match self.bits {
            false => PIO_INT2_A::DISABLE,
            true => PIO_INT2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PIO_INT2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PIO_INT2_A::ENABLE
    }
}
#[doc = "Write proxy for field `PIO_INT2`"]
pub struct PIO_INT2_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO_INT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO_INT2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PIO_INT2_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PIO_INT2_A::ENABLE)
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
#[doc = "Possible values of the field `PIO_INT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO_INT3_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<PIO_INT3_A> for bool {
    #[inline(always)]
    fn from(variant: PIO_INT3_A) -> Self {
        match variant {
            PIO_INT3_A::DISABLE => false,
            PIO_INT3_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `PIO_INT3`"]
pub type PIO_INT3_R = crate::R<bool, PIO_INT3_A>;
impl PIO_INT3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO_INT3_A {
        match self.bits {
            false => PIO_INT3_A::DISABLE,
            true => PIO_INT3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PIO_INT3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PIO_INT3_A::ENABLE
    }
}
#[doc = "Write proxy for field `PIO_INT3`"]
pub struct PIO_INT3_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO_INT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO_INT3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PIO_INT3_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PIO_INT3_A::ENABLE)
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
#[doc = "Possible values of the field `UTICK0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UTICK0_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<UTICK0_A> for bool {
    #[inline(always)]
    fn from(variant: UTICK0_A) -> Self {
        match variant {
            UTICK0_A::DISABLE => false,
            UTICK0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `UTICK0`"]
pub type UTICK0_R = crate::R<bool, UTICK0_A>;
impl UTICK0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTICK0_A {
        match self.bits {
            false => UTICK0_A::DISABLE,
            true => UTICK0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UTICK0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UTICK0_A::ENABLE
    }
}
#[doc = "Write proxy for field `UTICK0`"]
pub struct UTICK0_W<'a> {
    w: &'a mut W,
}
impl<'a> UTICK0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UTICK0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UTICK0_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UTICK0_A::ENABLE)
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
#[doc = "Possible values of the field `MRT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRT0_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<MRT0_A> for bool {
    #[inline(always)]
    fn from(variant: MRT0_A) -> Self {
        match variant {
            MRT0_A::DISABLE => false,
            MRT0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `MRT0`"]
pub type MRT0_R = crate::R<bool, MRT0_A>;
impl MRT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRT0_A {
        match self.bits {
            false => MRT0_A::DISABLE,
            true => MRT0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MRT0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MRT0_A::ENABLE
    }
}
#[doc = "Write proxy for field `MRT0`"]
pub struct MRT0_W<'a> {
    w: &'a mut W,
}
impl<'a> MRT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MRT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MRT0_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MRT0_A::ENABLE)
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
#[doc = "Possible values of the field `CTIMER0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER0_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<CTIMER0_A> for bool {
    #[inline(always)]
    fn from(variant: CTIMER0_A) -> Self {
        match variant {
            CTIMER0_A::DISABLE => false,
            CTIMER0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CTIMER0`"]
pub type CTIMER0_R = crate::R<bool, CTIMER0_A>;
impl CTIMER0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER0_A {
        match self.bits {
            false => CTIMER0_A::DISABLE,
            true => CTIMER0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTIMER0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTIMER0_A::ENABLE
    }
}
#[doc = "Write proxy for field `CTIMER0`"]
pub struct CTIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMER0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTIMER0_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTIMER0_A::ENABLE)
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
#[doc = "Possible values of the field `CTIMER1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER1_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<CTIMER1_A> for bool {
    #[inline(always)]
    fn from(variant: CTIMER1_A) -> Self {
        match variant {
            CTIMER1_A::DISABLE => false,
            CTIMER1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CTIMER1`"]
pub type CTIMER1_R = crate::R<bool, CTIMER1_A>;
impl CTIMER1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER1_A {
        match self.bits {
            false => CTIMER1_A::DISABLE,
            true => CTIMER1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTIMER1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTIMER1_A::ENABLE
    }
}
#[doc = "Write proxy for field `CTIMER1`"]
pub struct CTIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMER1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTIMER1_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTIMER1_A::ENABLE)
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
#[doc = "Possible values of the field `SCT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT0_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<SCT0_A> for bool {
    #[inline(always)]
    fn from(variant: SCT0_A) -> Self {
        match variant {
            SCT0_A::DISABLE => false,
            SCT0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SCT0`"]
pub type SCT0_R = crate::R<bool, SCT0_A>;
impl SCT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT0_A {
        match self.bits {
            false => SCT0_A::DISABLE,
            true => SCT0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SCT0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SCT0_A::ENABLE
    }
}
#[doc = "Write proxy for field `SCT0`"]
pub struct SCT0_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCT0_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCT0_A::ENABLE)
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
#[doc = "Possible values of the field `CTIMER3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER3_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<CTIMER3_A> for bool {
    #[inline(always)]
    fn from(variant: CTIMER3_A) -> Self {
        match variant {
            CTIMER3_A::DISABLE => false,
            CTIMER3_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CTIMER3`"]
pub type CTIMER3_R = crate::R<bool, CTIMER3_A>;
impl CTIMER3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER3_A {
        match self.bits {
            false => CTIMER3_A::DISABLE,
            true => CTIMER3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTIMER3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTIMER3_A::ENABLE
    }
}
#[doc = "Write proxy for field `CTIMER3`"]
pub struct CTIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMER3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTIMER3_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTIMER3_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Possible values of the field `FLEXINT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXINT0_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<FLEXINT0_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXINT0_A) -> Self {
        match variant {
            FLEXINT0_A::DISABLE => false,
            FLEXINT0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `FLEXINT0`"]
pub type FLEXINT0_R = crate::R<bool, FLEXINT0_A>;
impl FLEXINT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXINT0_A {
        match self.bits {
            false => FLEXINT0_A::DISABLE,
            true => FLEXINT0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXINT0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXINT0_A::ENABLE
    }
}
#[doc = "Write proxy for field `FLEXINT0`"]
pub struct FLEXINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXINT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXINT0_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXINT0_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `FLEXINT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXINT1_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<FLEXINT1_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXINT1_A) -> Self {
        match variant {
            FLEXINT1_A::DISABLE => false,
            FLEXINT1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `FLEXINT1`"]
pub type FLEXINT1_R = crate::R<bool, FLEXINT1_A>;
impl FLEXINT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXINT1_A {
        match self.bits {
            false => FLEXINT1_A::DISABLE,
            true => FLEXINT1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXINT1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXINT1_A::ENABLE
    }
}
#[doc = "Write proxy for field `FLEXINT1`"]
pub struct FLEXINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXINT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXINT1_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXINT1_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Possible values of the field `FLEXINT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXINT2_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<FLEXINT2_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXINT2_A) -> Self {
        match variant {
            FLEXINT2_A::DISABLE => false,
            FLEXINT2_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `FLEXINT2`"]
pub type FLEXINT2_R = crate::R<bool, FLEXINT2_A>;
impl FLEXINT2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXINT2_A {
        match self.bits {
            false => FLEXINT2_A::DISABLE,
            true => FLEXINT2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXINT2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXINT2_A::ENABLE
    }
}
#[doc = "Write proxy for field `FLEXINT2`"]
pub struct FLEXINT2_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXINT2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXINT2_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXINT2_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `FLEXINT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXINT3_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<FLEXINT3_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXINT3_A) -> Self {
        match variant {
            FLEXINT3_A::DISABLE => false,
            FLEXINT3_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `FLEXINT3`"]
pub type FLEXINT3_R = crate::R<bool, FLEXINT3_A>;
impl FLEXINT3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXINT3_A {
        match self.bits {
            false => FLEXINT3_A::DISABLE,
            true => FLEXINT3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXINT3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXINT3_A::ENABLE
    }
}
#[doc = "Write proxy for field `FLEXINT3`"]
pub struct FLEXINT3_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXINT3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXINT3_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXINT3_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Possible values of the field `FLEXINT4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXINT4_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<FLEXINT4_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXINT4_A) -> Self {
        match variant {
            FLEXINT4_A::DISABLE => false,
            FLEXINT4_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `FLEXINT4`"]
pub type FLEXINT4_R = crate::R<bool, FLEXINT4_A>;
impl FLEXINT4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXINT4_A {
        match self.bits {
            false => FLEXINT4_A::DISABLE,
            true => FLEXINT4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXINT4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXINT4_A::ENABLE
    }
}
#[doc = "Write proxy for field `FLEXINT4`"]
pub struct FLEXINT4_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXINT4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXINT4_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXINT4_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `FLEXINT5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXINT5_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<FLEXINT5_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXINT5_A) -> Self {
        match variant {
            FLEXINT5_A::DISABLE => false,
            FLEXINT5_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `FLEXINT5`"]
pub type FLEXINT5_R = crate::R<bool, FLEXINT5_A>;
impl FLEXINT5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXINT5_A {
        match self.bits {
            false => FLEXINT5_A::DISABLE,
            true => FLEXINT5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXINT5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXINT5_A::ENABLE
    }
}
#[doc = "Write proxy for field `FLEXINT5`"]
pub struct FLEXINT5_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXINT5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXINT5_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXINT5_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Possible values of the field `FLEXINT6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXINT6_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<FLEXINT6_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXINT6_A) -> Self {
        match variant {
            FLEXINT6_A::DISABLE => false,
            FLEXINT6_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `FLEXINT6`"]
pub type FLEXINT6_R = crate::R<bool, FLEXINT6_A>;
impl FLEXINT6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXINT6_A {
        match self.bits {
            false => FLEXINT6_A::DISABLE,
            true => FLEXINT6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXINT6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXINT6_A::ENABLE
    }
}
#[doc = "Write proxy for field `FLEXINT6`"]
pub struct FLEXINT6_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXINT6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXINT6_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXINT6_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `FLEXINT7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXINT7_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<FLEXINT7_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXINT7_A) -> Self {
        match variant {
            FLEXINT7_A::DISABLE => false,
            FLEXINT7_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `FLEXINT7`"]
pub type FLEXINT7_R = crate::R<bool, FLEXINT7_A>;
impl FLEXINT7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXINT7_A {
        match self.bits {
            false => FLEXINT7_A::DISABLE,
            true => FLEXINT7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXINT7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXINT7_A::ENABLE
    }
}
#[doc = "Write proxy for field `FLEXINT7`"]
pub struct FLEXINT7_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXINT7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXINT7_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXINT7_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Possible values of the field `ADC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<ADC0_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_A) -> Self {
        match variant {
            ADC0_A::DISABLE => false,
            ADC0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `ADC0`"]
pub type ADC0_R = crate::R<bool, ADC0_A>;
impl ADC0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0_A {
        match self.bits {
            false => ADC0_A::DISABLE,
            true => ADC0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADC0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADC0_A::ENABLE
    }
}
#[doc = "Write proxy for field `ADC0`"]
pub struct ADC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC0_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC0_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `ADC0_THCMP_OVR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0_THCMP_OVR_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<ADC0_THCMP_OVR_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_THCMP_OVR_A) -> Self {
        match variant {
            ADC0_THCMP_OVR_A::DISABLE => false,
            ADC0_THCMP_OVR_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `ADC0_THCMP_OVR`"]
pub type ADC0_THCMP_OVR_R = crate::R<bool, ADC0_THCMP_OVR_A>;
impl ADC0_THCMP_OVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0_THCMP_OVR_A {
        match self.bits {
            false => ADC0_THCMP_OVR_A::DISABLE,
            true => ADC0_THCMP_OVR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADC0_THCMP_OVR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADC0_THCMP_OVR_A::ENABLE
    }
}
#[doc = "Write proxy for field `ADC0_THCMP_OVR`"]
pub struct ADC0_THCMP_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_THCMP_OVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0_THCMP_OVR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC0_THCMP_OVR_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC0_THCMP_OVR_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `USB0_NEEDCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_NEEDCLK_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<USB0_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_NEEDCLK_A) -> Self {
        match variant {
            USB0_NEEDCLK_A::DISABLE => false,
            USB0_NEEDCLK_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `USB0_NEEDCLK`"]
pub type USB0_NEEDCLK_R = crate::R<bool, USB0_NEEDCLK_A>;
impl USB0_NEEDCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_NEEDCLK_A {
        match self.bits {
            false => USB0_NEEDCLK_A::DISABLE,
            true => USB0_NEEDCLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB0_NEEDCLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB0_NEEDCLK_A::ENABLE
    }
}
#[doc = "Write proxy for field `USB0_NEEDCLK`"]
pub struct USB0_NEEDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_NEEDCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB0_NEEDCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB0_NEEDCLK_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB0_NEEDCLK_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Possible values of the field `USB0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<USB0_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_A) -> Self {
        match variant {
            USB0_A::DISABLE => false,
            USB0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `USB0`"]
pub type USB0_R = crate::R<bool, USB0_A>;
impl USB0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_A {
        match self.bits {
            false => USB0_A::DISABLE,
            true => USB0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB0_A::ENABLE
    }
}
#[doc = "Write proxy for field `USB0`"]
pub struct USB0_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB0_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB0_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Possible values of the field `RTC_LITE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_LITE0_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<RTC_LITE0_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_LITE0_A) -> Self {
        match variant {
            RTC_LITE0_A::DISABLE => false,
            RTC_LITE0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `RTC_LITE0`"]
pub type RTC_LITE0_R = crate::R<bool, RTC_LITE0_A>;
impl RTC_LITE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_LITE0_A {
        match self.bits {
            false => RTC_LITE0_A::DISABLE,
            true => RTC_LITE0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RTC_LITE0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTC_LITE0_A::ENABLE
    }
}
#[doc = "Write proxy for field `RTC_LITE0`"]
pub struct RTC_LITE0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_LITE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_LITE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTC_LITE0_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTC_LITE0_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Possible values of the field `EZH_ARCH_B0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EZH_ARCH_B0_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<EZH_ARCH_B0_A> for bool {
    #[inline(always)]
    fn from(variant: EZH_ARCH_B0_A) -> Self {
        match variant {
            EZH_ARCH_B0_A::DISABLE => false,
            EZH_ARCH_B0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `EZH_ARCH_B0`"]
pub type EZH_ARCH_B0_R = crate::R<bool, EZH_ARCH_B0_A>;
impl EZH_ARCH_B0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EZH_ARCH_B0_A {
        match self.bits {
            false => EZH_ARCH_B0_A::DISABLE,
            true => EZH_ARCH_B0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EZH_ARCH_B0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EZH_ARCH_B0_A::ENABLE
    }
}
#[doc = "Write proxy for field `EZH_ARCH_B0`"]
pub struct EZH_ARCH_B0_W<'a> {
    w: &'a mut W,
}
impl<'a> EZH_ARCH_B0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EZH_ARCH_B0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EZH_ARCH_B0_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EZH_ARCH_B0_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Possible values of the field `WAKEUP_MAILBOX0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_MAILBOX0_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<WAKEUP_MAILBOX0_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_MAILBOX0_A) -> Self {
        match variant {
            WAKEUP_MAILBOX0_A::DISABLE => false,
            WAKEUP_MAILBOX0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `WAKEUP_MAILBOX0`"]
pub type WAKEUP_MAILBOX0_R = crate::R<bool, WAKEUP_MAILBOX0_A>;
impl WAKEUP_MAILBOX0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP_MAILBOX0_A {
        match self.bits {
            false => WAKEUP_MAILBOX0_A::DISABLE,
            true => WAKEUP_MAILBOX0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WAKEUP_MAILBOX0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WAKEUP_MAILBOX0_A::ENABLE
    }
}
#[doc = "Write proxy for field `WAKEUP_MAILBOX0`"]
pub struct WAKEUP_MAILBOX0_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_MAILBOX0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUP_MAILBOX0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WAKEUP_MAILBOX0_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WAKEUP_MAILBOX0_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SYS interrupt wake-up."]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SDMA0 interrupt wake-up."]
    #[inline(always)]
    pub fn sdma0(&self) -> SDMA0_R {
        SDMA0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GINT0 interrupt wake-up."]
    #[inline(always)]
    pub fn gint0(&self) -> GINT0_R {
        GINT0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GINT1 interrupt wake-up."]
    #[inline(always)]
    pub fn gint1(&self) -> GINT1_R {
        GINT1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PIO_INT0 interrupt wake-up."]
    #[inline(always)]
    pub fn pio_int0(&self) -> PIO_INT0_R {
        PIO_INT0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PIO_INT1 interrupt wake-up."]
    #[inline(always)]
    pub fn pio_int1(&self) -> PIO_INT1_R {
        PIO_INT1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PIO_INT2 interrupt wake-up."]
    #[inline(always)]
    pub fn pio_int2(&self) -> PIO_INT2_R {
        PIO_INT2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PIO_INT3 interrupt wake-up."]
    #[inline(always)]
    pub fn pio_int3(&self) -> PIO_INT3_R {
        PIO_INT3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UTICK0 interrupt wake-up."]
    #[inline(always)]
    pub fn utick0(&self) -> UTICK0_R {
        UTICK0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MRT0 interrupt wake-up."]
    #[inline(always)]
    pub fn mrt0(&self) -> MRT0_R {
        MRT0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CTIMER0 interrupt wake-up."]
    #[inline(always)]
    pub fn ctimer0(&self) -> CTIMER0_R {
        CTIMER0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CTIMER1 interrupt wake-up."]
    #[inline(always)]
    pub fn ctimer1(&self) -> CTIMER1_R {
        CTIMER1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SCT0 interrupt wake-up."]
    #[inline(always)]
    pub fn sct0(&self) -> SCT0_R {
        SCT0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CTIMER3 interrupt wake-up."]
    #[inline(always)]
    pub fn ctimer3(&self) -> CTIMER3_R {
        CTIMER3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - FLEXINT0 interrupt wake-up."]
    #[inline(always)]
    pub fn flexint0(&self) -> FLEXINT0_R {
        FLEXINT0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - FLEXINT1 interrupt wake-up."]
    #[inline(always)]
    pub fn flexint1(&self) -> FLEXINT1_R {
        FLEXINT1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FLEXINT2 interrupt wake-up."]
    #[inline(always)]
    pub fn flexint2(&self) -> FLEXINT2_R {
        FLEXINT2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FLEXINT3 interrupt wake-up."]
    #[inline(always)]
    pub fn flexint3(&self) -> FLEXINT3_R {
        FLEXINT3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - FLEXINT4 interrupt wake-up."]
    #[inline(always)]
    pub fn flexint4(&self) -> FLEXINT4_R {
        FLEXINT4_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - FLEXINT5 interrupt wake-up."]
    #[inline(always)]
    pub fn flexint5(&self) -> FLEXINT5_R {
        FLEXINT5_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - FLEXINT6 interrupt wake-up."]
    #[inline(always)]
    pub fn flexint6(&self) -> FLEXINT6_R {
        FLEXINT6_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - FLEXINT7 interrupt wake-up."]
    #[inline(always)]
    pub fn flexint7(&self) -> FLEXINT7_R {
        FLEXINT7_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ADC0 interrupt wake-up."]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADC0_THCMP_OVR interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_thcmp_ovr(&self) -> ADC0_THCMP_OVR_R {
        ADC0_THCMP_OVR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 27 - USB0_NEEDCLK interrupt wake-up."]
    #[inline(always)]
    pub fn usb0_needclk(&self) -> USB0_NEEDCLK_R {
        USB0_NEEDCLK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - USB0 interrupt wake-up."]
    #[inline(always)]
    pub fn usb0(&self) -> USB0_R {
        USB0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - RTC_LITE0 interrupt wake-up."]
    #[inline(always)]
    pub fn rtc_lite0(&self) -> RTC_LITE0_R {
        RTC_LITE0_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - EZH_ARCH_B0 interrupt wake-up."]
    #[inline(always)]
    pub fn ezh_arch_b0(&self) -> EZH_ARCH_B0_R {
        EZH_ARCH_B0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - WAKEUP_MAILBOX0 interrupt wake-up."]
    #[inline(always)]
    pub fn wakeup_mailbox0(&self) -> WAKEUP_MAILBOX0_R {
        WAKEUP_MAILBOX0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYS interrupt wake-up."]
    #[inline(always)]
    pub fn sys(&mut self) -> SYS_W {
        SYS_W { w: self }
    }
    #[doc = "Bit 1 - SDMA0 interrupt wake-up."]
    #[inline(always)]
    pub fn sdma0(&mut self) -> SDMA0_W {
        SDMA0_W { w: self }
    }
    #[doc = "Bit 2 - GINT0 interrupt wake-up."]
    #[inline(always)]
    pub fn gint0(&mut self) -> GINT0_W {
        GINT0_W { w: self }
    }
    #[doc = "Bit 3 - GINT1 interrupt wake-up."]
    #[inline(always)]
    pub fn gint1(&mut self) -> GINT1_W {
        GINT1_W { w: self }
    }
    #[doc = "Bit 4 - PIO_INT0 interrupt wake-up."]
    #[inline(always)]
    pub fn pio_int0(&mut self) -> PIO_INT0_W {
        PIO_INT0_W { w: self }
    }
    #[doc = "Bit 5 - PIO_INT1 interrupt wake-up."]
    #[inline(always)]
    pub fn pio_int1(&mut self) -> PIO_INT1_W {
        PIO_INT1_W { w: self }
    }
    #[doc = "Bit 6 - PIO_INT2 interrupt wake-up."]
    #[inline(always)]
    pub fn pio_int2(&mut self) -> PIO_INT2_W {
        PIO_INT2_W { w: self }
    }
    #[doc = "Bit 7 - PIO_INT3 interrupt wake-up."]
    #[inline(always)]
    pub fn pio_int3(&mut self) -> PIO_INT3_W {
        PIO_INT3_W { w: self }
    }
    #[doc = "Bit 8 - UTICK0 interrupt wake-up."]
    #[inline(always)]
    pub fn utick0(&mut self) -> UTICK0_W {
        UTICK0_W { w: self }
    }
    #[doc = "Bit 9 - MRT0 interrupt wake-up."]
    #[inline(always)]
    pub fn mrt0(&mut self) -> MRT0_W {
        MRT0_W { w: self }
    }
    #[doc = "Bit 10 - CTIMER0 interrupt wake-up."]
    #[inline(always)]
    pub fn ctimer0(&mut self) -> CTIMER0_W {
        CTIMER0_W { w: self }
    }
    #[doc = "Bit 11 - CTIMER1 interrupt wake-up."]
    #[inline(always)]
    pub fn ctimer1(&mut self) -> CTIMER1_W {
        CTIMER1_W { w: self }
    }
    #[doc = "Bit 12 - SCT0 interrupt wake-up."]
    #[inline(always)]
    pub fn sct0(&mut self) -> SCT0_W {
        SCT0_W { w: self }
    }
    #[doc = "Bit 13 - CTIMER3 interrupt wake-up."]
    #[inline(always)]
    pub fn ctimer3(&mut self) -> CTIMER3_W {
        CTIMER3_W { w: self }
    }
    #[doc = "Bit 14 - FLEXINT0 interrupt wake-up."]
    #[inline(always)]
    pub fn flexint0(&mut self) -> FLEXINT0_W {
        FLEXINT0_W { w: self }
    }
    #[doc = "Bit 15 - FLEXINT1 interrupt wake-up."]
    #[inline(always)]
    pub fn flexint1(&mut self) -> FLEXINT1_W {
        FLEXINT1_W { w: self }
    }
    #[doc = "Bit 16 - FLEXINT2 interrupt wake-up."]
    #[inline(always)]
    pub fn flexint2(&mut self) -> FLEXINT2_W {
        FLEXINT2_W { w: self }
    }
    #[doc = "Bit 17 - FLEXINT3 interrupt wake-up."]
    #[inline(always)]
    pub fn flexint3(&mut self) -> FLEXINT3_W {
        FLEXINT3_W { w: self }
    }
    #[doc = "Bit 18 - FLEXINT4 interrupt wake-up."]
    #[inline(always)]
    pub fn flexint4(&mut self) -> FLEXINT4_W {
        FLEXINT4_W { w: self }
    }
    #[doc = "Bit 19 - FLEXINT5 interrupt wake-up."]
    #[inline(always)]
    pub fn flexint5(&mut self) -> FLEXINT5_W {
        FLEXINT5_W { w: self }
    }
    #[doc = "Bit 20 - FLEXINT6 interrupt wake-up."]
    #[inline(always)]
    pub fn flexint6(&mut self) -> FLEXINT6_W {
        FLEXINT6_W { w: self }
    }
    #[doc = "Bit 21 - FLEXINT7 interrupt wake-up."]
    #[inline(always)]
    pub fn flexint7(&mut self) -> FLEXINT7_W {
        FLEXINT7_W { w: self }
    }
    #[doc = "Bit 22 - ADC0 interrupt wake-up."]
    #[inline(always)]
    pub fn adc0(&mut self) -> ADC0_W {
        ADC0_W { w: self }
    }
    #[doc = "Bit 24 - ADC0_THCMP_OVR interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_thcmp_ovr(&mut self) -> ADC0_THCMP_OVR_W {
        ADC0_THCMP_OVR_W { w: self }
    }
    #[doc = "Bit 27 - USB0_NEEDCLK interrupt wake-up."]
    #[inline(always)]
    pub fn usb0_needclk(&mut self) -> USB0_NEEDCLK_W {
        USB0_NEEDCLK_W { w: self }
    }
    #[doc = "Bit 28 - USB0 interrupt wake-up."]
    #[inline(always)]
    pub fn usb0(&mut self) -> USB0_W {
        USB0_W { w: self }
    }
    #[doc = "Bit 29 - RTC_LITE0 interrupt wake-up."]
    #[inline(always)]
    pub fn rtc_lite0(&mut self) -> RTC_LITE0_W {
        RTC_LITE0_W { w: self }
    }
    #[doc = "Bit 30 - EZH_ARCH_B0 interrupt wake-up."]
    #[inline(always)]
    pub fn ezh_arch_b0(&mut self) -> EZH_ARCH_B0_W {
        EZH_ARCH_B0_W { w: self }
    }
    #[doc = "Bit 31 - WAKEUP_MAILBOX0 interrupt wake-up."]
    #[inline(always)]
    pub fn wakeup_mailbox0(&mut self) -> WAKEUP_MAILBOX0_W {
        WAKEUP_MAILBOX0_W { w: self }
    }
}
