#[doc = "Reader of register PRESETCTRL2"]
pub type R = crate::R<u32, super::PRESETCTRL2>;
#[doc = "Writer for register PRESETCTRL2"]
pub type W = crate::W<u32, super::PRESETCTRL2>;
#[doc = "Register PRESETCTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRESETCTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA1 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<DMA1_RST_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA1_RST`"]
pub type DMA1_RST_R = crate::R<bool, DMA1_RST_A>;
impl DMA1_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1_RST_A {
        match self.bits {
            false => DMA1_RST_A::RELEASED,
            true => DMA1_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == DMA1_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == DMA1_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `DMA1_RST`"]
pub struct DMA1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA1_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(DMA1_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(DMA1_RST_A::ASSERTED)
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
#[doc = "Comparator reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<COMP_RST_A> for bool {
    #[inline(always)]
    fn from(variant: COMP_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMP_RST`"]
pub type COMP_RST_R = crate::R<bool, COMP_RST_A>;
impl COMP_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP_RST_A {
        match self.bits {
            false => COMP_RST_A::RELEASED,
            true => COMP_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == COMP_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == COMP_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `COMP_RST`"]
pub struct COMP_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(COMP_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(COMP_RST_A::ASSERTED)
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
#[doc = "SDIO reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIO_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<SDIO_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SDIO_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDIO_RST`"]
pub type SDIO_RST_R = crate::R<bool, SDIO_RST_A>;
impl SDIO_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIO_RST_A {
        match self.bits {
            false => SDIO_RST_A::RELEASED,
            true => SDIO_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == SDIO_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SDIO_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `SDIO_RST`"]
pub struct SDIO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIO_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(SDIO_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SDIO_RST_A::ASSERTED)
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
#[doc = "USB1 Host reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_HOST_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<USB1_HOST_RST_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_HOST_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USB1_HOST_RST`"]
pub type USB1_HOST_RST_R = crate::R<bool, USB1_HOST_RST_A>;
impl USB1_HOST_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_HOST_RST_A {
        match self.bits {
            false => USB1_HOST_RST_A::RELEASED,
            true => USB1_HOST_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == USB1_HOST_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == USB1_HOST_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `USB1_HOST_RST`"]
pub struct USB1_HOST_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_HOST_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_HOST_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(USB1_HOST_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB1_HOST_RST_A::ASSERTED)
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
#[doc = "USB1 dev reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_DEV_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<USB1_DEV_RST_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_DEV_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USB1_DEV_RST`"]
pub type USB1_DEV_RST_R = crate::R<bool, USB1_DEV_RST_A>;
impl USB1_DEV_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_DEV_RST_A {
        match self.bits {
            false => USB1_DEV_RST_A::RELEASED,
            true => USB1_DEV_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == USB1_DEV_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == USB1_DEV_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `USB1_DEV_RST`"]
pub struct USB1_DEV_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_DEV_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_DEV_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(USB1_DEV_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB1_DEV_RST_A::ASSERTED)
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
#[doc = "USB1 RAM reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_RAM_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<USB1_RAM_RST_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_RAM_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USB1_RAM_RST`"]
pub type USB1_RAM_RST_R = crate::R<bool, USB1_RAM_RST_A>;
impl USB1_RAM_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_RAM_RST_A {
        match self.bits {
            false => USB1_RAM_RST_A::RELEASED,
            true => USB1_RAM_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == USB1_RAM_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == USB1_RAM_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `USB1_RAM_RST`"]
pub struct USB1_RAM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_RAM_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_RAM_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(USB1_RAM_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB1_RAM_RST_A::ASSERTED)
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
#[doc = "USB1 PHY reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_PHY_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<USB1_PHY_RST_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_PHY_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USB1_PHY_RST`"]
pub type USB1_PHY_RST_R = crate::R<bool, USB1_PHY_RST_A>;
impl USB1_PHY_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_PHY_RST_A {
        match self.bits {
            false => USB1_PHY_RST_A::RELEASED,
            true => USB1_PHY_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == USB1_PHY_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == USB1_PHY_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `USB1_PHY_RST`"]
pub struct USB1_PHY_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_PHY_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_PHY_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(USB1_PHY_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB1_PHY_RST_A::ASSERTED)
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
#[doc = "Frequency meter reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQME_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FREQME_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FREQME_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FREQME_RST`"]
pub type FREQME_RST_R = crate::R<bool, FREQME_RST_A>;
impl FREQME_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQME_RST_A {
        match self.bits {
            false => FREQME_RST_A::RELEASED,
            true => FREQME_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FREQME_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FREQME_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `FREQME_RST`"]
pub struct FREQME_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQME_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQME_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FREQME_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FREQME_RST_A::ASSERTED)
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
#[doc = "RNG reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNG_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<RNG_RST_A> for bool {
    #[inline(always)]
    fn from(variant: RNG_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RNG_RST`"]
pub type RNG_RST_R = crate::R<bool, RNG_RST_A>;
impl RNG_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG_RST_A {
        match self.bits {
            false => RNG_RST_A::RELEASED,
            true => RNG_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == RNG_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == RNG_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `RNG_RST`"]
pub struct RNG_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNG_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(RNG_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(RNG_RST_A::ASSERTED)
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
#[doc = "SYSCTL Block reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<SYSCTL_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCTL_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSCTL_RST`"]
pub type SYSCTL_RST_R = crate::R<bool, SYSCTL_RST_A>;
impl SYSCTL_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCTL_RST_A {
        match self.bits {
            false => SYSCTL_RST_A::RELEASED,
            true => SYSCTL_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == SYSCTL_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SYSCTL_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `SYSCTL_RST`"]
pub struct SYSCTL_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(SYSCTL_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SYSCTL_RST_A::ASSERTED)
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
#[doc = "USB0 Host Master reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_HOSTM_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<USB0_HOSTM_RST_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_HOSTM_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USB0_HOSTM_RST`"]
pub type USB0_HOSTM_RST_R = crate::R<bool, USB0_HOSTM_RST_A>;
impl USB0_HOSTM_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_HOSTM_RST_A {
        match self.bits {
            false => USB0_HOSTM_RST_A::RELEASED,
            true => USB0_HOSTM_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == USB0_HOSTM_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == USB0_HOSTM_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `USB0_HOSTM_RST`"]
pub struct USB0_HOSTM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_HOSTM_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB0_HOSTM_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(USB0_HOSTM_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB0_HOSTM_RST_A::ASSERTED)
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
#[doc = "USB0 Host Slave reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_HOSTS_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<USB0_HOSTS_RST_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_HOSTS_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USB0_HOSTS_RST`"]
pub type USB0_HOSTS_RST_R = crate::R<bool, USB0_HOSTS_RST_A>;
impl USB0_HOSTS_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_HOSTS_RST_A {
        match self.bits {
            false => USB0_HOSTS_RST_A::RELEASED,
            true => USB0_HOSTS_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == USB0_HOSTS_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == USB0_HOSTS_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `USB0_HOSTS_RST`"]
pub struct USB0_HOSTS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_HOSTS_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB0_HOSTS_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(USB0_HOSTS_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB0_HOSTS_RST_A::ASSERTED)
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
#[doc = "HASH_AES reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASH_AES_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<HASH_AES_RST_A> for bool {
    #[inline(always)]
    fn from(variant: HASH_AES_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HASH_AES_RST`"]
pub type HASH_AES_RST_R = crate::R<bool, HASH_AES_RST_A>;
impl HASH_AES_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASH_AES_RST_A {
        match self.bits {
            false => HASH_AES_RST_A::RELEASED,
            true => HASH_AES_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == HASH_AES_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == HASH_AES_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `HASH_AES_RST`"]
pub struct HASH_AES_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH_AES_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HASH_AES_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(HASH_AES_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(HASH_AES_RST_A::ASSERTED)
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
#[doc = "Power Quad reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PQ_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<PQ_RST_A> for bool {
    #[inline(always)]
    fn from(variant: PQ_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PQ_RST`"]
pub type PQ_RST_R = crate::R<bool, PQ_RST_A>;
impl PQ_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PQ_RST_A {
        match self.bits {
            false => PQ_RST_A::RELEASED,
            true => PQ_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == PQ_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == PQ_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `PQ_RST`"]
pub struct PQ_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PQ_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PQ_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(PQ_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(PQ_RST_A::ASSERTED)
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
#[doc = "PLU LUT reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLULUT_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<PLULUT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: PLULUT_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLULUT_RST`"]
pub type PLULUT_RST_R = crate::R<bool, PLULUT_RST_A>;
impl PLULUT_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLULUT_RST_A {
        match self.bits {
            false => PLULUT_RST_A::RELEASED,
            true => PLULUT_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == PLULUT_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == PLULUT_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `PLULUT_RST`"]
pub struct PLULUT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PLULUT_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLULUT_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(PLULUT_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(PLULUT_RST_A::ASSERTED)
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
#[doc = "Timer 3 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER3_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<TIMER3_RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER3_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMER3_RST`"]
pub type TIMER3_RST_R = crate::R<bool, TIMER3_RST_A>;
impl TIMER3_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER3_RST_A {
        match self.bits {
            false => TIMER3_RST_A::RELEASED,
            true => TIMER3_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == TIMER3_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == TIMER3_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `TIMER3_RST`"]
pub struct TIMER3_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER3_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER3_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(TIMER3_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TIMER3_RST_A::ASSERTED)
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
#[doc = "Timer 4 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER4_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<TIMER4_RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER4_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMER4_RST`"]
pub type TIMER4_RST_R = crate::R<bool, TIMER4_RST_A>;
impl TIMER4_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER4_RST_A {
        match self.bits {
            false => TIMER4_RST_A::RELEASED,
            true => TIMER4_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == TIMER4_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == TIMER4_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `TIMER4_RST`"]
pub struct TIMER4_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER4_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER4_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(TIMER4_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TIMER4_RST_A::ASSERTED)
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
#[doc = "PUF reset control reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUF_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<PUF_RST_A> for bool {
    #[inline(always)]
    fn from(variant: PUF_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PUF_RST`"]
pub type PUF_RST_R = crate::R<bool, PUF_RST_A>;
impl PUF_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUF_RST_A {
        match self.bits {
            false => PUF_RST_A::RELEASED,
            true => PUF_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == PUF_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == PUF_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `PUF_RST`"]
pub struct PUF_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PUF_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUF_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(PUF_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(PUF_RST_A::ASSERTED)
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
#[doc = "Casper reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CASPER_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<CASPER_RST_A> for bool {
    #[inline(always)]
    fn from(variant: CASPER_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CASPER_RST`"]
pub type CASPER_RST_R = crate::R<bool, CASPER_RST_A>;
impl CASPER_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CASPER_RST_A {
        match self.bits {
            false => CASPER_RST_A::RELEASED,
            true => CASPER_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == CASPER_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == CASPER_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `CASPER_RST`"]
pub struct CASPER_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CASPER_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CASPER_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(CASPER_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(CASPER_RST_A::ASSERTED)
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
#[doc = "analog control reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANALOG_CTRL_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<ANALOG_CTRL_RST_A> for bool {
    #[inline(always)]
    fn from(variant: ANALOG_CTRL_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ANALOG_CTRL_RST`"]
pub type ANALOG_CTRL_RST_R = crate::R<bool, ANALOG_CTRL_RST_A>;
impl ANALOG_CTRL_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANALOG_CTRL_RST_A {
        match self.bits {
            false => ANALOG_CTRL_RST_A::RELEASED,
            true => ANALOG_CTRL_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == ANALOG_CTRL_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == ANALOG_CTRL_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `ANALOG_CTRL_RST`"]
pub struct ANALOG_CTRL_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_CTRL_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANALOG_CTRL_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(ANALOG_CTRL_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(ANALOG_CTRL_RST_A::ASSERTED)
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
#[doc = "HS LSPI reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_LSPI_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<HS_LSPI_RST_A> for bool {
    #[inline(always)]
    fn from(variant: HS_LSPI_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HS_LSPI_RST`"]
pub type HS_LSPI_RST_R = crate::R<bool, HS_LSPI_RST_A>;
impl HS_LSPI_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_LSPI_RST_A {
        match self.bits {
            false => HS_LSPI_RST_A::RELEASED,
            true => HS_LSPI_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == HS_LSPI_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == HS_LSPI_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `HS_LSPI_RST`"]
pub struct HS_LSPI_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_LSPI_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS_LSPI_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(HS_LSPI_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(HS_LSPI_RST_A::ASSERTED)
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
#[doc = "GPIO secure reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_SEC_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<GPIO_SEC_RST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_SEC_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO_SEC_RST`"]
pub type GPIO_SEC_RST_R = crate::R<bool, GPIO_SEC_RST_A>;
impl GPIO_SEC_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_SEC_RST_A {
        match self.bits {
            false => GPIO_SEC_RST_A::RELEASED,
            true => GPIO_SEC_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == GPIO_SEC_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == GPIO_SEC_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `GPIO_SEC_RST`"]
pub struct GPIO_SEC_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_SEC_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_SEC_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(GPIO_SEC_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GPIO_SEC_RST_A::ASSERTED)
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
#[doc = "GPIO secure int reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_SEC_INT_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<GPIO_SEC_INT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_SEC_INT_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO_SEC_INT_RST`"]
pub type GPIO_SEC_INT_RST_R = crate::R<bool, GPIO_SEC_INT_RST_A>;
impl GPIO_SEC_INT_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_SEC_INT_RST_A {
        match self.bits {
            false => GPIO_SEC_INT_RST_A::RELEASED,
            true => GPIO_SEC_INT_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == GPIO_SEC_INT_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == GPIO_SEC_INT_RST_A::ASSERTED
    }
}
#[doc = "Write proxy for field `GPIO_SEC_INT_RST`"]
pub struct GPIO_SEC_INT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_SEC_INT_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_SEC_INT_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(GPIO_SEC_INT_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GPIO_SEC_INT_RST_A::ASSERTED)
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
impl R {
    #[doc = "Bit 1 - DMA1 reset control."]
    #[inline(always)]
    pub fn dma1_rst(&self) -> DMA1_RST_R {
        DMA1_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comparator reset control."]
    #[inline(always)]
    pub fn comp_rst(&self) -> COMP_RST_R {
        COMP_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SDIO reset control."]
    #[inline(always)]
    pub fn sdio_rst(&self) -> SDIO_RST_R {
        SDIO_RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USB1 Host reset control."]
    #[inline(always)]
    pub fn usb1_host_rst(&self) -> USB1_HOST_RST_R {
        USB1_HOST_RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB1 dev reset control."]
    #[inline(always)]
    pub fn usb1_dev_rst(&self) -> USB1_DEV_RST_R {
        USB1_DEV_RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USB1 RAM reset control."]
    #[inline(always)]
    pub fn usb1_ram_rst(&self) -> USB1_RAM_RST_R {
        USB1_RAM_RST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB1 PHY reset control."]
    #[inline(always)]
    pub fn usb1_phy_rst(&self) -> USB1_PHY_RST_R {
        USB1_PHY_RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Frequency meter reset control."]
    #[inline(always)]
    pub fn freqme_rst(&self) -> FREQME_RST_R {
        FREQME_RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RNG reset control."]
    #[inline(always)]
    pub fn rng_rst(&self) -> RNG_RST_R {
        RNG_RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SYSCTL Block reset."]
    #[inline(always)]
    pub fn sysctl_rst(&self) -> SYSCTL_RST_R {
        SYSCTL_RST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USB0 Host Master reset control."]
    #[inline(always)]
    pub fn usb0_hostm_rst(&self) -> USB0_HOSTM_RST_R {
        USB0_HOSTM_RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - USB0 Host Slave reset control."]
    #[inline(always)]
    pub fn usb0_hosts_rst(&self) -> USB0_HOSTS_RST_R {
        USB0_HOSTS_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - HASH_AES reset control."]
    #[inline(always)]
    pub fn hash_aes_rst(&self) -> HASH_AES_RST_R {
        HASH_AES_RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Power Quad reset control."]
    #[inline(always)]
    pub fn pq_rst(&self) -> PQ_RST_R {
        PQ_RST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PLU LUT reset control."]
    #[inline(always)]
    pub fn plulut_rst(&self) -> PLULUT_RST_R {
        PLULUT_RST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Timer 3 reset control."]
    #[inline(always)]
    pub fn timer3_rst(&self) -> TIMER3_RST_R {
        TIMER3_RST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Timer 4 reset control."]
    #[inline(always)]
    pub fn timer4_rst(&self) -> TIMER4_RST_R {
        TIMER4_RST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PUF reset control reset control."]
    #[inline(always)]
    pub fn puf_rst(&self) -> PUF_RST_R {
        PUF_RST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Casper reset control."]
    #[inline(always)]
    pub fn casper_rst(&self) -> CASPER_RST_R {
        CASPER_RST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 27 - analog control reset control."]
    #[inline(always)]
    pub fn analog_ctrl_rst(&self) -> ANALOG_CTRL_RST_R {
        ANALOG_CTRL_RST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - HS LSPI reset control."]
    #[inline(always)]
    pub fn hs_lspi_rst(&self) -> HS_LSPI_RST_R {
        HS_LSPI_RST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - GPIO secure reset control."]
    #[inline(always)]
    pub fn gpio_sec_rst(&self) -> GPIO_SEC_RST_R {
        GPIO_SEC_RST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - GPIO secure int reset control."]
    #[inline(always)]
    pub fn gpio_sec_int_rst(&self) -> GPIO_SEC_INT_RST_R {
        GPIO_SEC_INT_RST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DMA1 reset control."]
    #[inline(always)]
    pub fn dma1_rst(&mut self) -> DMA1_RST_W {
        DMA1_RST_W { w: self }
    }
    #[doc = "Bit 2 - Comparator reset control."]
    #[inline(always)]
    pub fn comp_rst(&mut self) -> COMP_RST_W {
        COMP_RST_W { w: self }
    }
    #[doc = "Bit 3 - SDIO reset control."]
    #[inline(always)]
    pub fn sdio_rst(&mut self) -> SDIO_RST_W {
        SDIO_RST_W { w: self }
    }
    #[doc = "Bit 4 - USB1 Host reset control."]
    #[inline(always)]
    pub fn usb1_host_rst(&mut self) -> USB1_HOST_RST_W {
        USB1_HOST_RST_W { w: self }
    }
    #[doc = "Bit 5 - USB1 dev reset control."]
    #[inline(always)]
    pub fn usb1_dev_rst(&mut self) -> USB1_DEV_RST_W {
        USB1_DEV_RST_W { w: self }
    }
    #[doc = "Bit 6 - USB1 RAM reset control."]
    #[inline(always)]
    pub fn usb1_ram_rst(&mut self) -> USB1_RAM_RST_W {
        USB1_RAM_RST_W { w: self }
    }
    #[doc = "Bit 7 - USB1 PHY reset control."]
    #[inline(always)]
    pub fn usb1_phy_rst(&mut self) -> USB1_PHY_RST_W {
        USB1_PHY_RST_W { w: self }
    }
    #[doc = "Bit 8 - Frequency meter reset control."]
    #[inline(always)]
    pub fn freqme_rst(&mut self) -> FREQME_RST_W {
        FREQME_RST_W { w: self }
    }
    #[doc = "Bit 13 - RNG reset control."]
    #[inline(always)]
    pub fn rng_rst(&mut self) -> RNG_RST_W {
        RNG_RST_W { w: self }
    }
    #[doc = "Bit 15 - SYSCTL Block reset."]
    #[inline(always)]
    pub fn sysctl_rst(&mut self) -> SYSCTL_RST_W {
        SYSCTL_RST_W { w: self }
    }
    #[doc = "Bit 16 - USB0 Host Master reset control."]
    #[inline(always)]
    pub fn usb0_hostm_rst(&mut self) -> USB0_HOSTM_RST_W {
        USB0_HOSTM_RST_W { w: self }
    }
    #[doc = "Bit 17 - USB0 Host Slave reset control."]
    #[inline(always)]
    pub fn usb0_hosts_rst(&mut self) -> USB0_HOSTS_RST_W {
        USB0_HOSTS_RST_W { w: self }
    }
    #[doc = "Bit 18 - HASH_AES reset control."]
    #[inline(always)]
    pub fn hash_aes_rst(&mut self) -> HASH_AES_RST_W {
        HASH_AES_RST_W { w: self }
    }
    #[doc = "Bit 19 - Power Quad reset control."]
    #[inline(always)]
    pub fn pq_rst(&mut self) -> PQ_RST_W {
        PQ_RST_W { w: self }
    }
    #[doc = "Bit 20 - PLU LUT reset control."]
    #[inline(always)]
    pub fn plulut_rst(&mut self) -> PLULUT_RST_W {
        PLULUT_RST_W { w: self }
    }
    #[doc = "Bit 21 - Timer 3 reset control."]
    #[inline(always)]
    pub fn timer3_rst(&mut self) -> TIMER3_RST_W {
        TIMER3_RST_W { w: self }
    }
    #[doc = "Bit 22 - Timer 4 reset control."]
    #[inline(always)]
    pub fn timer4_rst(&mut self) -> TIMER4_RST_W {
        TIMER4_RST_W { w: self }
    }
    #[doc = "Bit 23 - PUF reset control reset control."]
    #[inline(always)]
    pub fn puf_rst(&mut self) -> PUF_RST_W {
        PUF_RST_W { w: self }
    }
    #[doc = "Bit 24 - Casper reset control."]
    #[inline(always)]
    pub fn casper_rst(&mut self) -> CASPER_RST_W {
        CASPER_RST_W { w: self }
    }
    #[doc = "Bit 27 - analog control reset control."]
    #[inline(always)]
    pub fn analog_ctrl_rst(&mut self) -> ANALOG_CTRL_RST_W {
        ANALOG_CTRL_RST_W { w: self }
    }
    #[doc = "Bit 28 - HS LSPI reset control."]
    #[inline(always)]
    pub fn hs_lspi_rst(&mut self) -> HS_LSPI_RST_W {
        HS_LSPI_RST_W { w: self }
    }
    #[doc = "Bit 29 - GPIO secure reset control."]
    #[inline(always)]
    pub fn gpio_sec_rst(&mut self) -> GPIO_SEC_RST_W {
        GPIO_SEC_RST_W { w: self }
    }
    #[doc = "Bit 30 - GPIO secure int reset control."]
    #[inline(always)]
    pub fn gpio_sec_int_rst(&mut self) -> GPIO_SEC_INT_RST_W {
        GPIO_SEC_INT_RST_W { w: self }
    }
}
