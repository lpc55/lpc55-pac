#[doc = "Reader of register PRESETCTRL0"]
pub type R = crate::R<u32, super::PRESETCTRL0>;
#[doc = "Writer for register PRESETCTRL0"]
pub type W = crate::W<u32, super::PRESETCTRL0>;
#[doc = "Register PRESETCTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRESETCTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `ROM_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROM_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<ROM_RST_A> for bool {
    #[inline(always)]
    fn from(variant: ROM_RST_A) -> Self {
        match variant {
            ROM_RST_A::RELEASED => false,
            ROM_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `ROM_RST`"]
pub type ROM_RST_R = crate::R<bool, ROM_RST_A>;
impl ROM_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROM_RST_A {
        match self.bits {
            false => ROM_RST_A::RELEASED,
            true => ROM_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == ROM_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == ROM_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `ROM_RST`"]
pub struct ROM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROM_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(ROM_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(ROM_RST_A::ASSERTED)
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
#[doc = "Possible values of the field `SRAM_CTRL1_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL1_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<SRAM_CTRL1_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CTRL1_RST_A) -> Self {
        match variant {
            SRAM_CTRL1_RST_A::RELEASED => false,
            SRAM_CTRL1_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `SRAM_CTRL1_RST`"]
pub type SRAM_CTRL1_RST_R = crate::R<bool, SRAM_CTRL1_RST_A>;
impl SRAM_CTRL1_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CTRL1_RST_A {
        match self.bits {
            false => SRAM_CTRL1_RST_A::RELEASED,
            true => SRAM_CTRL1_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == SRAM_CTRL1_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SRAM_CTRL1_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `SRAM_CTRL1_RST`"]
pub struct SRAM_CTRL1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_CTRL1_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_CTRL1_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(SRAM_CTRL1_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SRAM_CTRL1_RST_A::ASSERTED)
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
#[doc = "Possible values of the field `SRAM_CTRL2_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL2_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<SRAM_CTRL2_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CTRL2_RST_A) -> Self {
        match variant {
            SRAM_CTRL2_RST_A::RELEASED => false,
            SRAM_CTRL2_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `SRAM_CTRL2_RST`"]
pub type SRAM_CTRL2_RST_R = crate::R<bool, SRAM_CTRL2_RST_A>;
impl SRAM_CTRL2_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CTRL2_RST_A {
        match self.bits {
            false => SRAM_CTRL2_RST_A::RELEASED,
            true => SRAM_CTRL2_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == SRAM_CTRL2_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SRAM_CTRL2_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `SRAM_CTRL2_RST`"]
pub struct SRAM_CTRL2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_CTRL2_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_CTRL2_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(SRAM_CTRL2_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SRAM_CTRL2_RST_A::ASSERTED)
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
#[doc = "Possible values of the field `SRAM_CTRL3_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL3_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<SRAM_CTRL3_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CTRL3_RST_A) -> Self {
        match variant {
            SRAM_CTRL3_RST_A::RELEASED => false,
            SRAM_CTRL3_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `SRAM_CTRL3_RST`"]
pub type SRAM_CTRL3_RST_R = crate::R<bool, SRAM_CTRL3_RST_A>;
impl SRAM_CTRL3_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CTRL3_RST_A {
        match self.bits {
            false => SRAM_CTRL3_RST_A::RELEASED,
            true => SRAM_CTRL3_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == SRAM_CTRL3_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SRAM_CTRL3_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `SRAM_CTRL3_RST`"]
pub struct SRAM_CTRL3_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_CTRL3_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_CTRL3_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(SRAM_CTRL3_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SRAM_CTRL3_RST_A::ASSERTED)
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
#[doc = "Possible values of the field `SRAM_CTRL4_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL4_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<SRAM_CTRL4_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CTRL4_RST_A) -> Self {
        match variant {
            SRAM_CTRL4_RST_A::RELEASED => false,
            SRAM_CTRL4_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `SRAM_CTRL4_RST`"]
pub type SRAM_CTRL4_RST_R = crate::R<bool, SRAM_CTRL4_RST_A>;
impl SRAM_CTRL4_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CTRL4_RST_A {
        match self.bits {
            false => SRAM_CTRL4_RST_A::RELEASED,
            true => SRAM_CTRL4_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == SRAM_CTRL4_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SRAM_CTRL4_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `SRAM_CTRL4_RST`"]
pub struct SRAM_CTRL4_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_CTRL4_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_CTRL4_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(SRAM_CTRL4_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SRAM_CTRL4_RST_A::ASSERTED)
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
#[doc = "Possible values of the field `FLASH_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<FLASH_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_RST_A) -> Self {
        match variant {
            FLASH_RST_A::RELEASED => false,
            FLASH_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `FLASH_RST`"]
pub type FLASH_RST_R = crate::R<bool, FLASH_RST_A>;
impl FLASH_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_RST_A {
        match self.bits {
            false => FLASH_RST_A::RELEASED,
            true => FLASH_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FLASH_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FLASH_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `FLASH_RST`"]
pub struct FLASH_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FLASH_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FLASH_RST_A::ASSERTED)
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
#[doc = "Possible values of the field `FMC_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMC_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<FMC_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FMC_RST_A) -> Self {
        match variant {
            FMC_RST_A::RELEASED => false,
            FMC_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `FMC_RST`"]
pub type FMC_RST_R = crate::R<bool, FMC_RST_A>;
impl FMC_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMC_RST_A {
        match self.bits {
            false => FMC_RST_A::RELEASED,
            true => FMC_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FMC_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FMC_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `FMC_RST`"]
pub struct FMC_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FMC_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMC_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FMC_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FMC_RST_A::ASSERTED)
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
#[doc = "Possible values of the field `MUX_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<MUX_RST_A> for bool {
    #[inline(always)]
    fn from(variant: MUX_RST_A) -> Self {
        match variant {
            MUX_RST_A::RELEASED => false,
            MUX_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `MUX_RST`"]
pub type MUX_RST_R = crate::R<bool, MUX_RST_A>;
impl MUX_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUX_RST_A {
        match self.bits {
            false => MUX_RST_A::RELEASED,
            true => MUX_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == MUX_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == MUX_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `MUX_RST`"]
pub struct MUX_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUX_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(MUX_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(MUX_RST_A::ASSERTED)
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
#[doc = "Possible values of the field `IOCON_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCON_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<IOCON_RST_A> for bool {
    #[inline(always)]
    fn from(variant: IOCON_RST_A) -> Self {
        match variant {
            IOCON_RST_A::RELEASED => false,
            IOCON_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `IOCON_RST`"]
pub type IOCON_RST_R = crate::R<bool, IOCON_RST_A>;
impl IOCON_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCON_RST_A {
        match self.bits {
            false => IOCON_RST_A::RELEASED,
            true => IOCON_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == IOCON_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == IOCON_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `IOCON_RST`"]
pub struct IOCON_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCON_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOCON_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(IOCON_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(IOCON_RST_A::ASSERTED)
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
#[doc = "Possible values of the field `GPIO0_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<GPIO0_RST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0_RST_A) -> Self {
        match variant {
            GPIO0_RST_A::RELEASED => false,
            GPIO0_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `GPIO0_RST`"]
pub type GPIO0_RST_R = crate::R<bool, GPIO0_RST_A>;
impl GPIO0_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0_RST_A {
        match self.bits {
            false => GPIO0_RST_A::RELEASED,
            true => GPIO0_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == GPIO0_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == GPIO0_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `GPIO0_RST`"]
pub struct GPIO0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO0_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO0_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(GPIO0_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GPIO0_RST_A::ASSERTED)
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
#[doc = "Possible values of the field `GPIO1_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO1_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<GPIO1_RST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1_RST_A) -> Self {
        match variant {
            GPIO1_RST_A::RELEASED => false,
            GPIO1_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `GPIO1_RST`"]
pub type GPIO1_RST_R = crate::R<bool, GPIO1_RST_A>;
impl GPIO1_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1_RST_A {
        match self.bits {
            false => GPIO1_RST_A::RELEASED,
            true => GPIO1_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == GPIO1_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == GPIO1_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `GPIO1_RST`"]
pub struct GPIO1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO1_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO1_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(GPIO1_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GPIO1_RST_A::ASSERTED)
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
#[doc = "Possible values of the field `GPIO2_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO2_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<GPIO2_RST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO2_RST_A) -> Self {
        match variant {
            GPIO2_RST_A::RELEASED => false,
            GPIO2_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `GPIO2_RST`"]
pub type GPIO2_RST_R = crate::R<bool, GPIO2_RST_A>;
impl GPIO2_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO2_RST_A {
        match self.bits {
            false => GPIO2_RST_A::RELEASED,
            true => GPIO2_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == GPIO2_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == GPIO2_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `GPIO2_RST`"]
pub struct GPIO2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO2_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO2_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(GPIO2_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GPIO2_RST_A::ASSERTED)
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
#[doc = "Possible values of the field `GPIO3_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO3_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<GPIO3_RST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO3_RST_A) -> Self {
        match variant {
            GPIO3_RST_A::RELEASED => false,
            GPIO3_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `GPIO3_RST`"]
pub type GPIO3_RST_R = crate::R<bool, GPIO3_RST_A>;
impl GPIO3_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO3_RST_A {
        match self.bits {
            false => GPIO3_RST_A::RELEASED,
            true => GPIO3_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == GPIO3_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == GPIO3_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `GPIO3_RST`"]
pub struct GPIO3_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO3_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO3_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(GPIO3_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GPIO3_RST_A::ASSERTED)
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
#[doc = "Possible values of the field `PINT_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<PINT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: PINT_RST_A) -> Self {
        match variant {
            PINT_RST_A::RELEASED => false,
            PINT_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `PINT_RST`"]
pub type PINT_RST_R = crate::R<bool, PINT_RST_A>;
impl PINT_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT_RST_A {
        match self.bits {
            false => PINT_RST_A::RELEASED,
            true => PINT_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == PINT_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == PINT_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `PINT_RST`"]
pub struct PINT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PINT_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(PINT_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(PINT_RST_A::ASSERTED)
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
#[doc = "Possible values of the field `GINT_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GINT_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<GINT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: GINT_RST_A) -> Self {
        match variant {
            GINT_RST_A::RELEASED => false,
            GINT_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `GINT_RST`"]
pub type GINT_RST_R = crate::R<bool, GINT_RST_A>;
impl GINT_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GINT_RST_A {
        match self.bits {
            false => GINT_RST_A::RELEASED,
            true => GINT_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == GINT_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == GINT_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `GINT_RST`"]
pub struct GINT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GINT_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GINT_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(GINT_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GINT_RST_A::ASSERTED)
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
#[doc = "Possible values of the field `DMA0_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA0_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<DMA0_RST_A> for bool {
    #[inline(always)]
    fn from(variant: DMA0_RST_A) -> Self {
        match variant {
            DMA0_RST_A::RELEASED => false,
            DMA0_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `DMA0_RST`"]
pub type DMA0_RST_R = crate::R<bool, DMA0_RST_A>;
impl DMA0_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA0_RST_A {
        match self.bits {
            false => DMA0_RST_A::RELEASED,
            true => DMA0_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == DMA0_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == DMA0_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `DMA0_RST`"]
pub struct DMA0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA0_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(DMA0_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(DMA0_RST_A::ASSERTED)
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
#[doc = "Possible values of the field `CRCGEN_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCGEN_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<CRCGEN_RST_A> for bool {
    #[inline(always)]
    fn from(variant: CRCGEN_RST_A) -> Self {
        match variant {
            CRCGEN_RST_A::RELEASED => false,
            CRCGEN_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `CRCGEN_RST`"]
pub type CRCGEN_RST_R = crate::R<bool, CRCGEN_RST_A>;
impl CRCGEN_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCGEN_RST_A {
        match self.bits {
            false => CRCGEN_RST_A::RELEASED,
            true => CRCGEN_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == CRCGEN_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == CRCGEN_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `CRCGEN_RST`"]
pub struct CRCGEN_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCGEN_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCGEN_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(CRCGEN_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(CRCGEN_RST_A::ASSERTED)
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
#[doc = "Possible values of the field `WWDT_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDT_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<WWDT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: WWDT_RST_A) -> Self {
        match variant {
            WWDT_RST_A::RELEASED => false,
            WWDT_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `WWDT_RST`"]
pub type WWDT_RST_R = crate::R<bool, WWDT_RST_A>;
impl WWDT_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDT_RST_A {
        match self.bits {
            false => WWDT_RST_A::RELEASED,
            true => WWDT_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == WWDT_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == WWDT_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `WWDT_RST`"]
pub struct WWDT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDT_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDT_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(WWDT_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(WWDT_RST_A::ASSERTED)
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
#[doc = "Possible values of the field `RTC_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<RTC_RST_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_RST_A) -> Self {
        match variant {
            RTC_RST_A::RELEASED => false,
            RTC_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `RTC_RST`"]
pub type RTC_RST_R = crate::R<bool, RTC_RST_A>;
impl RTC_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_RST_A {
        match self.bits {
            false => RTC_RST_A::RELEASED,
            true => RTC_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == RTC_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == RTC_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `RTC_RST`"]
pub struct RTC_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(RTC_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(RTC_RST_A::ASSERTED)
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
#[doc = "Possible values of the field `MAILBOX_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAILBOX_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<MAILBOX_RST_A> for bool {
    #[inline(always)]
    fn from(variant: MAILBOX_RST_A) -> Self {
        match variant {
            MAILBOX_RST_A::RELEASED => false,
            MAILBOX_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `MAILBOX_RST`"]
pub type MAILBOX_RST_R = crate::R<bool, MAILBOX_RST_A>;
impl MAILBOX_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAILBOX_RST_A {
        match self.bits {
            false => MAILBOX_RST_A::RELEASED,
            true => MAILBOX_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == MAILBOX_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == MAILBOX_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `MAILBOX_RST`"]
pub struct MAILBOX_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> MAILBOX_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAILBOX_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(MAILBOX_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(MAILBOX_RST_A::ASSERTED)
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
#[doc = "Possible values of the field `ADC_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_RST_A {
    #[doc = "Bloc is not reset."]
    RELEASED,
    #[doc = "Bloc is reset."]
    ASSERTED,
}
impl From<ADC_RST_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_RST_A) -> Self {
        match variant {
            ADC_RST_A::RELEASED => false,
            ADC_RST_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `ADC_RST`"]
pub type ADC_RST_R = crate::R<bool, ADC_RST_A>;
impl ADC_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_RST_A {
        match self.bits {
            false => ADC_RST_A::RELEASED,
            true => ADC_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == ADC_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == ADC_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `ADC_RST`"]
pub struct ADC_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(ADC_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(ADC_RST_A::ASSERTED)
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
    #[doc = "Bit 1 - ROM reset control."]
    #[inline(always)]
    pub fn rom_rst(&self) -> ROM_RST_R {
        ROM_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SRAM Controller 1 reset control."]
    #[inline(always)]
    pub fn sram_ctrl1_rst(&self) -> SRAM_CTRL1_RST_R {
        SRAM_CTRL1_RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SRAM Controller 2 reset control."]
    #[inline(always)]
    pub fn sram_ctrl2_rst(&self) -> SRAM_CTRL2_RST_R {
        SRAM_CTRL2_RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SRAM Controller 3 reset control."]
    #[inline(always)]
    pub fn sram_ctrl3_rst(&self) -> SRAM_CTRL3_RST_R {
        SRAM_CTRL3_RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SRAM Controller 4 reset control."]
    #[inline(always)]
    pub fn sram_ctrl4_rst(&self) -> SRAM_CTRL4_RST_R {
        SRAM_CTRL4_RST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Flash controller reset control."]
    #[inline(always)]
    pub fn flash_rst(&self) -> FLASH_RST_R {
        FLASH_RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FMC controller reset control."]
    #[inline(always)]
    pub fn fmc_rst(&self) -> FMC_RST_R {
        FMC_RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Input Mux reset control."]
    #[inline(always)]
    pub fn mux_rst(&self) -> MUX_RST_R {
        MUX_RST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - I/O controller reset control."]
    #[inline(always)]
    pub fn iocon_rst(&self) -> IOCON_RST_R {
        IOCON_RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - GPIO0 reset control."]
    #[inline(always)]
    pub fn gpio0_rst(&self) -> GPIO0_RST_R {
        GPIO0_RST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIO1 reset control."]
    #[inline(always)]
    pub fn gpio1_rst(&self) -> GPIO1_RST_R {
        GPIO1_RST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - GPIO2 reset control."]
    #[inline(always)]
    pub fn gpio2_rst(&self) -> GPIO2_RST_R {
        GPIO2_RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - GPIO3 reset control."]
    #[inline(always)]
    pub fn gpio3_rst(&self) -> GPIO3_RST_R {
        GPIO3_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Pin interrupt (PINT) reset control."]
    #[inline(always)]
    pub fn pint_rst(&self) -> PINT_RST_R {
        PINT_RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Group interrupt (GINT) reset control."]
    #[inline(always)]
    pub fn gint_rst(&self) -> GINT_RST_R {
        GINT_RST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DMA0 reset control."]
    #[inline(always)]
    pub fn dma0_rst(&self) -> DMA0_RST_R {
        DMA0_RST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CRCGEN reset control."]
    #[inline(always)]
    pub fn crcgen_rst(&self) -> CRCGEN_RST_R {
        CRCGEN_RST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Watchdog Timer reset control."]
    #[inline(always)]
    pub fn wwdt_rst(&self) -> WWDT_RST_R {
        WWDT_RST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Real Time Clock (RTC) reset control."]
    #[inline(always)]
    pub fn rtc_rst(&self) -> RTC_RST_R {
        RTC_RST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Inter CPU communication Mailbox reset control."]
    #[inline(always)]
    pub fn mailbox_rst(&self) -> MAILBOX_RST_R {
        MAILBOX_RST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - ADC reset control."]
    #[inline(always)]
    pub fn adc_rst(&self) -> ADC_RST_R {
        ADC_RST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ROM reset control."]
    #[inline(always)]
    pub fn rom_rst(&mut self) -> ROM_RST_W {
        ROM_RST_W { w: self }
    }
    #[doc = "Bit 3 - SRAM Controller 1 reset control."]
    #[inline(always)]
    pub fn sram_ctrl1_rst(&mut self) -> SRAM_CTRL1_RST_W {
        SRAM_CTRL1_RST_W { w: self }
    }
    #[doc = "Bit 4 - SRAM Controller 2 reset control."]
    #[inline(always)]
    pub fn sram_ctrl2_rst(&mut self) -> SRAM_CTRL2_RST_W {
        SRAM_CTRL2_RST_W { w: self }
    }
    #[doc = "Bit 5 - SRAM Controller 3 reset control."]
    #[inline(always)]
    pub fn sram_ctrl3_rst(&mut self) -> SRAM_CTRL3_RST_W {
        SRAM_CTRL3_RST_W { w: self }
    }
    #[doc = "Bit 6 - SRAM Controller 4 reset control."]
    #[inline(always)]
    pub fn sram_ctrl4_rst(&mut self) -> SRAM_CTRL4_RST_W {
        SRAM_CTRL4_RST_W { w: self }
    }
    #[doc = "Bit 7 - Flash controller reset control."]
    #[inline(always)]
    pub fn flash_rst(&mut self) -> FLASH_RST_W {
        FLASH_RST_W { w: self }
    }
    #[doc = "Bit 8 - FMC controller reset control."]
    #[inline(always)]
    pub fn fmc_rst(&mut self) -> FMC_RST_W {
        FMC_RST_W { w: self }
    }
    #[doc = "Bit 11 - Input Mux reset control."]
    #[inline(always)]
    pub fn mux_rst(&mut self) -> MUX_RST_W {
        MUX_RST_W { w: self }
    }
    #[doc = "Bit 13 - I/O controller reset control."]
    #[inline(always)]
    pub fn iocon_rst(&mut self) -> IOCON_RST_W {
        IOCON_RST_W { w: self }
    }
    #[doc = "Bit 14 - GPIO0 reset control."]
    #[inline(always)]
    pub fn gpio0_rst(&mut self) -> GPIO0_RST_W {
        GPIO0_RST_W { w: self }
    }
    #[doc = "Bit 15 - GPIO1 reset control."]
    #[inline(always)]
    pub fn gpio1_rst(&mut self) -> GPIO1_RST_W {
        GPIO1_RST_W { w: self }
    }
    #[doc = "Bit 16 - GPIO2 reset control."]
    #[inline(always)]
    pub fn gpio2_rst(&mut self) -> GPIO2_RST_W {
        GPIO2_RST_W { w: self }
    }
    #[doc = "Bit 17 - GPIO3 reset control."]
    #[inline(always)]
    pub fn gpio3_rst(&mut self) -> GPIO3_RST_W {
        GPIO3_RST_W { w: self }
    }
    #[doc = "Bit 18 - Pin interrupt (PINT) reset control."]
    #[inline(always)]
    pub fn pint_rst(&mut self) -> PINT_RST_W {
        PINT_RST_W { w: self }
    }
    #[doc = "Bit 19 - Group interrupt (GINT) reset control."]
    #[inline(always)]
    pub fn gint_rst(&mut self) -> GINT_RST_W {
        GINT_RST_W { w: self }
    }
    #[doc = "Bit 20 - DMA0 reset control."]
    #[inline(always)]
    pub fn dma0_rst(&mut self) -> DMA0_RST_W {
        DMA0_RST_W { w: self }
    }
    #[doc = "Bit 21 - CRCGEN reset control."]
    #[inline(always)]
    pub fn crcgen_rst(&mut self) -> CRCGEN_RST_W {
        CRCGEN_RST_W { w: self }
    }
    #[doc = "Bit 22 - Watchdog Timer reset control."]
    #[inline(always)]
    pub fn wwdt_rst(&mut self) -> WWDT_RST_W {
        WWDT_RST_W { w: self }
    }
    #[doc = "Bit 23 - Real Time Clock (RTC) reset control."]
    #[inline(always)]
    pub fn rtc_rst(&mut self) -> RTC_RST_W {
        RTC_RST_W { w: self }
    }
    #[doc = "Bit 26 - Inter CPU communication Mailbox reset control."]
    #[inline(always)]
    pub fn mailbox_rst(&mut self) -> MAILBOX_RST_W {
        MAILBOX_RST_W { w: self }
    }
    #[doc = "Bit 27 - ADC reset control."]
    #[inline(always)]
    pub fn adc_rst(&mut self) -> ADC_RST_W {
        ADC_RST_W { w: self }
    }
}
