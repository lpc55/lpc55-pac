#[doc = "Reader of register AHBCLKCTRL0"]
pub type R = crate::R<u32, super::AHBCLKCTRL0>;
#[doc = "Writer for register AHBCLKCTRL0"]
pub type W = crate::W<u32, super::AHBCLKCTRL0>;
#[doc = "Register AHBCLKCTRL0 `reset()`'s with value 0x0180"]
impl crate::ResetValue for super::AHBCLKCTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0180
    }
}
#[doc = "Possible values of the field `ROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROM_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<ROM_A> for bool {
    #[inline(always)]
    fn from(variant: ROM_A) -> Self {
        match variant {
            ROM_A::DISABLE => false,
            ROM_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `ROM`"]
pub type ROM_R = crate::R<bool, ROM_A>;
impl ROM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROM_A {
        match self.bits {
            false => ROM_A::DISABLE,
            true => ROM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ROM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ROM_A::ENABLE
    }
}
#[doc = "Write proxy for field `ROM`"]
pub struct ROM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ROM_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ROM_A::ENABLE)
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
#[doc = "Possible values of the field `SRAM_CTRL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL1_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<SRAM_CTRL1_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CTRL1_A) -> Self {
        match variant {
            SRAM_CTRL1_A::DISABLE => false,
            SRAM_CTRL1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SRAM_CTRL1`"]
pub type SRAM_CTRL1_R = crate::R<bool, SRAM_CTRL1_A>;
impl SRAM_CTRL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CTRL1_A {
        match self.bits {
            false => SRAM_CTRL1_A::DISABLE,
            true => SRAM_CTRL1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SRAM_CTRL1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SRAM_CTRL1_A::ENABLE
    }
}
#[doc = "Write proxy for field `SRAM_CTRL1`"]
pub struct SRAM_CTRL1_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_CTRL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_CTRL1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_CTRL1_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_CTRL1_A::ENABLE)
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
#[doc = "Possible values of the field `SRAM_CTRL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL2_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<SRAM_CTRL2_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CTRL2_A) -> Self {
        match variant {
            SRAM_CTRL2_A::DISABLE => false,
            SRAM_CTRL2_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SRAM_CTRL2`"]
pub type SRAM_CTRL2_R = crate::R<bool, SRAM_CTRL2_A>;
impl SRAM_CTRL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CTRL2_A {
        match self.bits {
            false => SRAM_CTRL2_A::DISABLE,
            true => SRAM_CTRL2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SRAM_CTRL2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SRAM_CTRL2_A::ENABLE
    }
}
#[doc = "Write proxy for field `SRAM_CTRL2`"]
pub struct SRAM_CTRL2_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_CTRL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_CTRL2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_CTRL2_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_CTRL2_A::ENABLE)
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
#[doc = "Possible values of the field `SRAM_CTRL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL3_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<SRAM_CTRL3_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CTRL3_A) -> Self {
        match variant {
            SRAM_CTRL3_A::DISABLE => false,
            SRAM_CTRL3_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SRAM_CTRL3`"]
pub type SRAM_CTRL3_R = crate::R<bool, SRAM_CTRL3_A>;
impl SRAM_CTRL3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CTRL3_A {
        match self.bits {
            false => SRAM_CTRL3_A::DISABLE,
            true => SRAM_CTRL3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SRAM_CTRL3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SRAM_CTRL3_A::ENABLE
    }
}
#[doc = "Write proxy for field `SRAM_CTRL3`"]
pub struct SRAM_CTRL3_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_CTRL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_CTRL3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_CTRL3_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_CTRL3_A::ENABLE)
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
#[doc = "Possible values of the field `SRAM_CTRL4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL4_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<SRAM_CTRL4_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CTRL4_A) -> Self {
        match variant {
            SRAM_CTRL4_A::DISABLE => false,
            SRAM_CTRL4_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SRAM_CTRL4`"]
pub type SRAM_CTRL4_R = crate::R<bool, SRAM_CTRL4_A>;
impl SRAM_CTRL4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CTRL4_A {
        match self.bits {
            false => SRAM_CTRL4_A::DISABLE,
            true => SRAM_CTRL4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SRAM_CTRL4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SRAM_CTRL4_A::ENABLE
    }
}
#[doc = "Write proxy for field `SRAM_CTRL4`"]
pub struct SRAM_CTRL4_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_CTRL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_CTRL4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_CTRL4_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_CTRL4_A::ENABLE)
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
#[doc = "Possible values of the field `FLASH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<FLASH_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_A) -> Self {
        match variant {
            FLASH_A::DISABLE => false,
            FLASH_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `FLASH`"]
pub type FLASH_R = crate::R<bool, FLASH_A>;
impl FLASH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_A {
        match self.bits {
            false => FLASH_A::DISABLE,
            true => FLASH_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLASH_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLASH_A::ENABLE
    }
}
#[doc = "Write proxy for field `FLASH`"]
pub struct FLASH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLASH_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLASH_A::ENABLE)
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
#[doc = "Possible values of the field `FMC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMC_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<FMC_A> for bool {
    #[inline(always)]
    fn from(variant: FMC_A) -> Self {
        match variant {
            FMC_A::DISABLE => false,
            FMC_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `FMC`"]
pub type FMC_R = crate::R<bool, FMC_A>;
impl FMC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMC_A {
        match self.bits {
            false => FMC_A::DISABLE,
            true => FMC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FMC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FMC_A::ENABLE
    }
}
#[doc = "Write proxy for field `FMC`"]
pub struct FMC_W<'a> {
    w: &'a mut W,
}
impl<'a> FMC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FMC_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FMC_A::ENABLE)
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
#[doc = "Possible values of the field `MUX0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX0_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<MUX0_A> for bool {
    #[inline(always)]
    fn from(variant: MUX0_A) -> Self {
        match variant {
            MUX0_A::DISABLE => false,
            MUX0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `MUX0`"]
pub type MUX0_R = crate::R<bool, MUX0_A>;
impl MUX0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUX0_A {
        match self.bits {
            false => MUX0_A::DISABLE,
            true => MUX0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MUX0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MUX0_A::ENABLE
    }
}
#[doc = "Write proxy for field `MUX0`"]
pub struct MUX0_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUX0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MUX0_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MUX0_A::ENABLE)
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
#[doc = "Possible values of the field `IOCON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCON_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<IOCON_A> for bool {
    #[inline(always)]
    fn from(variant: IOCON_A) -> Self {
        match variant {
            IOCON_A::DISABLE => false,
            IOCON_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `IOCON`"]
pub type IOCON_R = crate::R<bool, IOCON_A>;
impl IOCON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCON_A {
        match self.bits {
            false => IOCON_A::DISABLE,
            true => IOCON_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IOCON_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IOCON_A::ENABLE
    }
}
#[doc = "Write proxy for field `IOCON`"]
pub struct IOCON_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOCON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IOCON_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IOCON_A::ENABLE)
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
#[doc = "Possible values of the field `GPIO0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<GPIO0_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0_A) -> Self {
        match variant {
            GPIO0_A::DISABLE => false,
            GPIO0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `GPIO0`"]
pub type GPIO0_R = crate::R<bool, GPIO0_A>;
impl GPIO0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0_A {
        match self.bits {
            false => GPIO0_A::DISABLE,
            true => GPIO0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO0_A::ENABLE
    }
}
#[doc = "Write proxy for field `GPIO0`"]
pub struct GPIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO0_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO0_A::ENABLE)
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
#[doc = "Possible values of the field `GPIO1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO1_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<GPIO1_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1_A) -> Self {
        match variant {
            GPIO1_A::DISABLE => false,
            GPIO1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `GPIO1`"]
pub type GPIO1_R = crate::R<bool, GPIO1_A>;
impl GPIO1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1_A {
        match self.bits {
            false => GPIO1_A::DISABLE,
            true => GPIO1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO1_A::ENABLE
    }
}
#[doc = "Write proxy for field `GPIO1`"]
pub struct GPIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO1_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO1_A::ENABLE)
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
#[doc = "Possible values of the field `GPIO2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO2_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<GPIO2_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO2_A) -> Self {
        match variant {
            GPIO2_A::DISABLE => false,
            GPIO2_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `GPIO2`"]
pub type GPIO2_R = crate::R<bool, GPIO2_A>;
impl GPIO2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO2_A {
        match self.bits {
            false => GPIO2_A::DISABLE,
            true => GPIO2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO2_A::ENABLE
    }
}
#[doc = "Write proxy for field `GPIO2`"]
pub struct GPIO2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO2_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO2_A::ENABLE)
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
#[doc = "Possible values of the field `GPIO3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO3_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<GPIO3_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO3_A) -> Self {
        match variant {
            GPIO3_A::DISABLE => false,
            GPIO3_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `GPIO3`"]
pub type GPIO3_R = crate::R<bool, GPIO3_A>;
impl GPIO3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO3_A {
        match self.bits {
            false => GPIO3_A::DISABLE,
            true => GPIO3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO3_A::ENABLE
    }
}
#[doc = "Write proxy for field `GPIO3`"]
pub struct GPIO3_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO3_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO3_A::ENABLE)
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
#[doc = "Possible values of the field `PINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<PINT_A> for bool {
    #[inline(always)]
    fn from(variant: PINT_A) -> Self {
        match variant {
            PINT_A::DISABLE => false,
            PINT_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `PINT`"]
pub type PINT_R = crate::R<bool, PINT_A>;
impl PINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT_A {
        match self.bits {
            false => PINT_A::DISABLE,
            true => PINT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PINT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PINT_A::ENABLE
    }
}
#[doc = "Write proxy for field `PINT`"]
pub struct PINT_W<'a> {
    w: &'a mut W,
}
impl<'a> PINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PINT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PINT_A::ENABLE)
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
#[doc = "Possible values of the field `GINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GINT_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<GINT_A> for bool {
    #[inline(always)]
    fn from(variant: GINT_A) -> Self {
        match variant {
            GINT_A::DISABLE => false,
            GINT_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `GINT`"]
pub type GINT_R = crate::R<bool, GINT_A>;
impl GINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GINT_A {
        match self.bits {
            false => GINT_A::DISABLE,
            true => GINT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GINT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GINT_A::ENABLE
    }
}
#[doc = "Write proxy for field `GINT`"]
pub struct GINT_W<'a> {
    w: &'a mut W,
}
impl<'a> GINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GINT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GINT_A::ENABLE)
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
#[doc = "Possible values of the field `DMA0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA0_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<DMA0_A> for bool {
    #[inline(always)]
    fn from(variant: DMA0_A) -> Self {
        match variant {
            DMA0_A::DISABLE => false,
            DMA0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `DMA0`"]
pub type DMA0_R = crate::R<bool, DMA0_A>;
impl DMA0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA0_A {
        match self.bits {
            false => DMA0_A::DISABLE,
            true => DMA0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMA0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMA0_A::ENABLE
    }
}
#[doc = "Write proxy for field `DMA0`"]
pub struct DMA0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA0_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA0_A::ENABLE)
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
#[doc = "Possible values of the field `CRCGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCGEN_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<CRCGEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCGEN_A) -> Self {
        match variant {
            CRCGEN_A::DISABLE => false,
            CRCGEN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CRCGEN`"]
pub type CRCGEN_R = crate::R<bool, CRCGEN_A>;
impl CRCGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCGEN_A {
        match self.bits {
            false => CRCGEN_A::DISABLE,
            true => CRCGEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CRCGEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CRCGEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `CRCGEN`"]
pub struct CRCGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CRCGEN_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CRCGEN_A::ENABLE)
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
#[doc = "Possible values of the field `WWDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDT_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<WWDT_A> for bool {
    #[inline(always)]
    fn from(variant: WWDT_A) -> Self {
        match variant {
            WWDT_A::DISABLE => false,
            WWDT_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `WWDT`"]
pub type WWDT_R = crate::R<bool, WWDT_A>;
impl WWDT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDT_A {
        match self.bits {
            false => WWDT_A::DISABLE,
            true => WWDT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WWDT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WWDT_A::ENABLE
    }
}
#[doc = "Write proxy for field `WWDT`"]
pub struct WWDT_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WWDT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WWDT_A::ENABLE)
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
#[doc = "Possible values of the field `RTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<RTC_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_A) -> Self {
        match variant {
            RTC_A::DISABLE => false,
            RTC_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<bool, RTC_A>;
impl RTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_A {
        match self.bits {
            false => RTC_A::DISABLE,
            true => RTC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RTC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTC_A::ENABLE
    }
}
#[doc = "Write proxy for field `RTC`"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTC_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTC_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Possible values of the field `MAILBOX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAILBOX_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<MAILBOX_A> for bool {
    #[inline(always)]
    fn from(variant: MAILBOX_A) -> Self {
        match variant {
            MAILBOX_A::DISABLE => false,
            MAILBOX_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `MAILBOX`"]
pub type MAILBOX_R = crate::R<bool, MAILBOX_A>;
impl MAILBOX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAILBOX_A {
        match self.bits {
            false => MAILBOX_A::DISABLE,
            true => MAILBOX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MAILBOX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MAILBOX_A::ENABLE
    }
}
#[doc = "Write proxy for field `MAILBOX`"]
pub struct MAILBOX_W<'a> {
    w: &'a mut W,
}
impl<'a> MAILBOX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAILBOX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MAILBOX_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MAILBOX_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Possible values of the field `ADC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        match variant {
            ADC_A::DISABLE => false,
            ADC_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `ADC`"]
pub type ADC_R = crate::R<bool, ADC_A>;
impl ADC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_A {
        match self.bits {
            false => ADC_A::DISABLE,
            true => ADC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADC_A::ENABLE
    }
}
#[doc = "Write proxy for field `ADC`"]
pub struct ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC_A::ENABLE)
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
impl R {
    #[doc = "Bit 1 - Enables the clock for the ROM."]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables the clock for the SRAM Controller 1."]
    #[inline(always)]
    pub fn sram_ctrl1(&self) -> SRAM_CTRL1_R {
        SRAM_CTRL1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables the clock for the SRAM Controller 2."]
    #[inline(always)]
    pub fn sram_ctrl2(&self) -> SRAM_CTRL2_R {
        SRAM_CTRL2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enables the clock for the SRAM Controller 3."]
    #[inline(always)]
    pub fn sram_ctrl3(&self) -> SRAM_CTRL3_R {
        SRAM_CTRL3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enables the clock for the SRAM Controller 4."]
    #[inline(always)]
    pub fn sram_ctrl4(&self) -> SRAM_CTRL4_R {
        SRAM_CTRL4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables the clock for the Flash controller."]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables the clock for the FMC controller."]
    #[inline(always)]
    pub fn fmc(&self) -> FMC_R {
        FMC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for the Input Mux 0."]
    #[inline(always)]
    pub fn mux0(&self) -> MUX0_R {
        MUX0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for the I/O controller."]
    #[inline(always)]
    pub fn iocon(&self) -> IOCON_R {
        IOCON_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enables the clock for the GPIO0."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enables the clock for the GPIO1."]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO1_R {
        GPIO1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for the GPIO2."]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO2_R {
        GPIO2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for the GPIO3."]
    #[inline(always)]
    pub fn gpio3(&self) -> GPIO3_R {
        GPIO3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for the Pin interrupt (PINT)."]
    #[inline(always)]
    pub fn pint(&self) -> PINT_R {
        PINT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables the clock for the Group interrupt (GINT)."]
    #[inline(always)]
    pub fn gint(&self) -> GINT_R {
        GINT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enables the clock for the DMA0."]
    #[inline(always)]
    pub fn dma0(&self) -> DMA0_R {
        DMA0_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enables the clock for the CRCGEN."]
    #[inline(always)]
    pub fn crcgen(&self) -> CRCGEN_R {
        CRCGEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enables the clock for the Watchdog Timer."]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enables the clock for the Real Time Clock (RTC)."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enables the clock for the Inter CPU communication Mailbox."]
    #[inline(always)]
    pub fn mailbox(&self) -> MAILBOX_R {
        MAILBOX_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enables the clock for the ADC."]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enables the clock for the ROM."]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W {
        ROM_W { w: self }
    }
    #[doc = "Bit 3 - Enables the clock for the SRAM Controller 1."]
    #[inline(always)]
    pub fn sram_ctrl1(&mut self) -> SRAM_CTRL1_W {
        SRAM_CTRL1_W { w: self }
    }
    #[doc = "Bit 4 - Enables the clock for the SRAM Controller 2."]
    #[inline(always)]
    pub fn sram_ctrl2(&mut self) -> SRAM_CTRL2_W {
        SRAM_CTRL2_W { w: self }
    }
    #[doc = "Bit 5 - Enables the clock for the SRAM Controller 3."]
    #[inline(always)]
    pub fn sram_ctrl3(&mut self) -> SRAM_CTRL3_W {
        SRAM_CTRL3_W { w: self }
    }
    #[doc = "Bit 6 - Enables the clock for the SRAM Controller 4."]
    #[inline(always)]
    pub fn sram_ctrl4(&mut self) -> SRAM_CTRL4_W {
        SRAM_CTRL4_W { w: self }
    }
    #[doc = "Bit 7 - Enables the clock for the Flash controller."]
    #[inline(always)]
    pub fn flash(&mut self) -> FLASH_W {
        FLASH_W { w: self }
    }
    #[doc = "Bit 8 - Enables the clock for the FMC controller."]
    #[inline(always)]
    pub fn fmc(&mut self) -> FMC_W {
        FMC_W { w: self }
    }
    #[doc = "Bit 11 - Enables the clock for the Input Mux 0."]
    #[inline(always)]
    pub fn mux0(&mut self) -> MUX0_W {
        MUX0_W { w: self }
    }
    #[doc = "Bit 13 - Enables the clock for the I/O controller."]
    #[inline(always)]
    pub fn iocon(&mut self) -> IOCON_W {
        IOCON_W { w: self }
    }
    #[doc = "Bit 14 - Enables the clock for the GPIO0."]
    #[inline(always)]
    pub fn gpio0(&mut self) -> GPIO0_W {
        GPIO0_W { w: self }
    }
    #[doc = "Bit 15 - Enables the clock for the GPIO1."]
    #[inline(always)]
    pub fn gpio1(&mut self) -> GPIO1_W {
        GPIO1_W { w: self }
    }
    #[doc = "Bit 16 - Enables the clock for the GPIO2."]
    #[inline(always)]
    pub fn gpio2(&mut self) -> GPIO2_W {
        GPIO2_W { w: self }
    }
    #[doc = "Bit 17 - Enables the clock for the GPIO3."]
    #[inline(always)]
    pub fn gpio3(&mut self) -> GPIO3_W {
        GPIO3_W { w: self }
    }
    #[doc = "Bit 18 - Enables the clock for the Pin interrupt (PINT)."]
    #[inline(always)]
    pub fn pint(&mut self) -> PINT_W {
        PINT_W { w: self }
    }
    #[doc = "Bit 19 - Enables the clock for the Group interrupt (GINT)."]
    #[inline(always)]
    pub fn gint(&mut self) -> GINT_W {
        GINT_W { w: self }
    }
    #[doc = "Bit 20 - Enables the clock for the DMA0."]
    #[inline(always)]
    pub fn dma0(&mut self) -> DMA0_W {
        DMA0_W { w: self }
    }
    #[doc = "Bit 21 - Enables the clock for the CRCGEN."]
    #[inline(always)]
    pub fn crcgen(&mut self) -> CRCGEN_W {
        CRCGEN_W { w: self }
    }
    #[doc = "Bit 22 - Enables the clock for the Watchdog Timer."]
    #[inline(always)]
    pub fn wwdt(&mut self) -> WWDT_W {
        WWDT_W { w: self }
    }
    #[doc = "Bit 23 - Enables the clock for the Real Time Clock (RTC)."]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Bit 26 - Enables the clock for the Inter CPU communication Mailbox."]
    #[inline(always)]
    pub fn mailbox(&mut self) -> MAILBOX_W {
        MAILBOX_W { w: self }
    }
    #[doc = "Bit 27 - Enables the clock for the ADC."]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W {
        ADC_W { w: self }
    }
}
