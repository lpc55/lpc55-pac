#[doc = "Reader of register PDRUNCFG0"]
pub type R = crate::R<u32, super::PDRUNCFG0>;
#[doc = "Writer for register PDRUNCFG0"]
pub type W = crate::W<u32, super::PDRUNCFG0>;
#[doc = "Register PDRUNCFG0 `reset()`'s with value 0x00de_ffc4"]
impl crate::ResetValue for super::PDRUNCFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00de_ffc4
    }
}
#[doc = "Controls power to VBAT Brown Out Detector (BOD).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_BODVBAT_A {
    #[doc = "0: BOD VBAT is powered."]
    POWEREDON = 0,
    #[doc = "1: BOD VBAT is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_BODVBAT_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_BODVBAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDEN_BODVBAT`"]
pub type PDEN_BODVBAT_R = crate::R<bool, PDEN_BODVBAT_A>;
impl PDEN_BODVBAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_BODVBAT_A {
        match self.bits {
            false => PDEN_BODVBAT_A::POWEREDON,
            true => PDEN_BODVBAT_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_BODVBAT_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_BODVBAT_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_BODVBAT`"]
pub struct PDEN_BODVBAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_BODVBAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_BODVBAT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BOD VBAT is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_BODVBAT_A::POWEREDON)
    }
    #[doc = "BOD VBAT is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_BODVBAT_A::POWEREDOFF)
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
#[doc = "Controls power to the Free Running Oscillator (FRO) 32 KHz.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_FRO32K_A {
    #[doc = "0: FRO32KHz is powered."]
    POWEREDON = 0,
    #[doc = "1: FRO32KHz is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_FRO32K_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_FRO32K_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDEN_FRO32K`"]
pub type PDEN_FRO32K_R = crate::R<bool, PDEN_FRO32K_A>;
impl PDEN_FRO32K_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_FRO32K_A {
        match self.bits {
            false => PDEN_FRO32K_A::POWEREDON,
            true => PDEN_FRO32K_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_FRO32K_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_FRO32K_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_FRO32K`"]
pub struct PDEN_FRO32K_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_FRO32K_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_FRO32K_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FRO32KHz is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_FRO32K_A::POWEREDON)
    }
    #[doc = "FRO32KHz is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_FRO32K_A::POWEREDOFF)
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
#[doc = "Controls power to crystal 32 KHz.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_XTAL32K_A {
    #[doc = "0: Crystal 32KHz is powered."]
    POWEREDON = 0,
    #[doc = "1: Crystal 32KHz is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_XTAL32K_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_XTAL32K_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDEN_XTAL32K`"]
pub type PDEN_XTAL32K_R = crate::R<bool, PDEN_XTAL32K_A>;
impl PDEN_XTAL32K_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_XTAL32K_A {
        match self.bits {
            false => PDEN_XTAL32K_A::POWEREDON,
            true => PDEN_XTAL32K_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_XTAL32K_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_XTAL32K_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_XTAL32K`"]
pub struct PDEN_XTAL32K_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_XTAL32K_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_XTAL32K_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Crystal 32KHz is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_XTAL32K_A::POWEREDON)
    }
    #[doc = "Crystal 32KHz is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_XTAL32K_A::POWEREDOFF)
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
#[doc = "Controls power to crystal 32 MHz.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_XTAL32M_A {
    #[doc = "0: Crystal 32MHz is powered."]
    POWEREDON = 0,
    #[doc = "1: Crystal 32MHz is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_XTAL32M_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_XTAL32M_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDEN_XTAL32M`"]
pub type PDEN_XTAL32M_R = crate::R<bool, PDEN_XTAL32M_A>;
impl PDEN_XTAL32M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_XTAL32M_A {
        match self.bits {
            false => PDEN_XTAL32M_A::POWEREDON,
            true => PDEN_XTAL32M_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_XTAL32M_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_XTAL32M_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_XTAL32M`"]
pub struct PDEN_XTAL32M_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_XTAL32M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_XTAL32M_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Crystal 32MHz is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_XTAL32M_A::POWEREDON)
    }
    #[doc = "Crystal 32MHz is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_XTAL32M_A::POWEREDOFF)
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
#[doc = "Controls power to System PLL (also refered as PLL0).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_PLL0_A {
    #[doc = "0: PLL0 is powered."]
    POWEREDON = 0,
    #[doc = "1: PLL0 is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_PLL0_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_PLL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDEN_PLL0`"]
pub type PDEN_PLL0_R = crate::R<bool, PDEN_PLL0_A>;
impl PDEN_PLL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_PLL0_A {
        match self.bits {
            false => PDEN_PLL0_A::POWEREDON,
            true => PDEN_PLL0_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_PLL0_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_PLL0_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_PLL0`"]
pub struct PDEN_PLL0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_PLL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_PLL0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PLL0 is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_PLL0_A::POWEREDON)
    }
    #[doc = "PLL0 is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_PLL0_A::POWEREDOFF)
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
#[doc = "Controls power to USB PLL (also refered as PLL1).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_PLL1_A {
    #[doc = "0: PLL1 is powered."]
    POWEREDON = 0,
    #[doc = "1: PLL1 is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_PLL1_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_PLL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDEN_PLL1`"]
pub type PDEN_PLL1_R = crate::R<bool, PDEN_PLL1_A>;
impl PDEN_PLL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_PLL1_A {
        match self.bits {
            false => PDEN_PLL1_A::POWEREDON,
            true => PDEN_PLL1_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_PLL1_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_PLL1_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_PLL1`"]
pub struct PDEN_PLL1_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_PLL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_PLL1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PLL1 is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_PLL1_A::POWEREDON)
    }
    #[doc = "PLL1 is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_PLL1_A::POWEREDOFF)
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
#[doc = "Controls power to USB Full Speed phy.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_USBFSPHY_A {
    #[doc = "0: USB Full Speed phy is powered."]
    POWEREDON = 0,
    #[doc = "1: USB Full Speed phy is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_USBFSPHY_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_USBFSPHY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDEN_USBFSPHY`"]
pub type PDEN_USBFSPHY_R = crate::R<bool, PDEN_USBFSPHY_A>;
impl PDEN_USBFSPHY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_USBFSPHY_A {
        match self.bits {
            false => PDEN_USBFSPHY_A::POWEREDON,
            true => PDEN_USBFSPHY_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_USBFSPHY_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_USBFSPHY_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_USBFSPHY`"]
pub struct PDEN_USBFSPHY_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_USBFSPHY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_USBFSPHY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USB Full Speed phy is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_USBFSPHY_A::POWEREDON)
    }
    #[doc = "USB Full Speed phy is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_USBFSPHY_A::POWEREDOFF)
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
#[doc = "Controls power to USB High Speed Phy.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_USBHSPHY_A {
    #[doc = "0: USB HS phy is powered."]
    POWEREDON = 0,
    #[doc = "1: USB HS phy is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_USBHSPHY_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_USBHSPHY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDEN_USBHSPHY`"]
pub type PDEN_USBHSPHY_R = crate::R<bool, PDEN_USBHSPHY_A>;
impl PDEN_USBHSPHY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_USBHSPHY_A {
        match self.bits {
            false => PDEN_USBHSPHY_A::POWEREDON,
            true => PDEN_USBHSPHY_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_USBHSPHY_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_USBHSPHY_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_USBHSPHY`"]
pub struct PDEN_USBHSPHY_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_USBHSPHY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_USBHSPHY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USB HS phy is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_USBHSPHY_A::POWEREDON)
    }
    #[doc = "USB HS phy is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_USBHSPHY_A::POWEREDOFF)
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
#[doc = "Controls power to Analog Comparator.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_COMP_A {
    #[doc = "0: Analog Comparator is powered."]
    POWEREDON = 0,
    #[doc = "1: Analog Comparator is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_COMP_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_COMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDEN_COMP`"]
pub type PDEN_COMP_R = crate::R<bool, PDEN_COMP_A>;
impl PDEN_COMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_COMP_A {
        match self.bits {
            false => PDEN_COMP_A::POWEREDON,
            true => PDEN_COMP_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_COMP_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_COMP_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_COMP`"]
pub struct PDEN_COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_COMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_COMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Analog Comparator is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_COMP_A::POWEREDON)
    }
    #[doc = "Analog Comparator is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_COMP_A::POWEREDOFF)
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
#[doc = "Controls power to USB high speed LDO.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_LDOUSBHS_A {
    #[doc = "0: USB high speed LDO is powered."]
    POWEREDON = 0,
    #[doc = "1: USB high speed LDO is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_LDOUSBHS_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_LDOUSBHS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDEN_LDOUSBHS`"]
pub type PDEN_LDOUSBHS_R = crate::R<bool, PDEN_LDOUSBHS_A>;
impl PDEN_LDOUSBHS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_LDOUSBHS_A {
        match self.bits {
            false => PDEN_LDOUSBHS_A::POWEREDON,
            true => PDEN_LDOUSBHS_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_LDOUSBHS_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_LDOUSBHS_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_LDOUSBHS`"]
pub struct PDEN_LDOUSBHS_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_LDOUSBHS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_LDOUSBHS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USB high speed LDO is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_LDOUSBHS_A::POWEREDON)
    }
    #[doc = "USB high speed LDO is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_LDOUSBHS_A::POWEREDOFF)
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
#[doc = "Controls power to auxiliary biasing (AUXBIAS)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_AUXBIAS_A {
    #[doc = "0: auxiliary biasing is powered."]
    POWEREDON = 0,
    #[doc = "1: auxiliary biasing is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_AUXBIAS_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_AUXBIAS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDEN_AUXBIAS`"]
pub type PDEN_AUXBIAS_R = crate::R<bool, PDEN_AUXBIAS_A>;
impl PDEN_AUXBIAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_AUXBIAS_A {
        match self.bits {
            false => PDEN_AUXBIAS_A::POWEREDON,
            true => PDEN_AUXBIAS_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_AUXBIAS_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_AUXBIAS_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_AUXBIAS`"]
pub struct PDEN_AUXBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_AUXBIAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_AUXBIAS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "auxiliary biasing is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_AUXBIAS_A::POWEREDON)
    }
    #[doc = "auxiliary biasing is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_AUXBIAS_A::POWEREDOFF)
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
#[doc = "Controls power to crystal 32 MHz LDO.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_LDOXO32M_A {
    #[doc = "0: crystal 32 MHz LDO is powered."]
    POWEREDON = 0,
    #[doc = "1: crystal 32 MHz LDO is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_LDOXO32M_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_LDOXO32M_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDEN_LDOXO32M`"]
pub type PDEN_LDOXO32M_R = crate::R<bool, PDEN_LDOXO32M_A>;
impl PDEN_LDOXO32M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_LDOXO32M_A {
        match self.bits {
            false => PDEN_LDOXO32M_A::POWEREDON,
            true => PDEN_LDOXO32M_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_LDOXO32M_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_LDOXO32M_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_LDOXO32M`"]
pub struct PDEN_LDOXO32M_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_LDOXO32M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_LDOXO32M_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "crystal 32 MHz LDO is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_LDOXO32M_A::POWEREDON)
    }
    #[doc = "crystal 32 MHz LDO is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_LDOXO32M_A::POWEREDOFF)
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
#[doc = "Controls power to all True Random Number Genetaor (TRNG) clock sources.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_RNG_A {
    #[doc = "0: TRNG clocks are powered."]
    POWEREDON = 0,
    #[doc = "1: TRNG clocks are powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_RNG_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_RNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDEN_RNG`"]
pub type PDEN_RNG_R = crate::R<bool, PDEN_RNG_A>;
impl PDEN_RNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_RNG_A {
        match self.bits {
            false => PDEN_RNG_A::POWEREDON,
            true => PDEN_RNG_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_RNG_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_RNG_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_RNG`"]
pub struct PDEN_RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_RNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_RNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TRNG clocks are powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_RNG_A::POWEREDON)
    }
    #[doc = "TRNG clocks are powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_RNG_A::POWEREDOFF)
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
#[doc = "Controls power to System PLL (PLL0) Spread Spectrum module.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_PLL0_SSCG_A {
    #[doc = "0: PLL0 Sread spectrum module is powered."]
    POWEREDON = 0,
    #[doc = "1: PLL0 Sread spectrum module is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_PLL0_SSCG_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_PLL0_SSCG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDEN_PLL0_SSCG`"]
pub type PDEN_PLL0_SSCG_R = crate::R<bool, PDEN_PLL0_SSCG_A>;
impl PDEN_PLL0_SSCG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_PLL0_SSCG_A {
        match self.bits {
            false => PDEN_PLL0_SSCG_A::POWEREDON,
            true => PDEN_PLL0_SSCG_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_PLL0_SSCG_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_PLL0_SSCG_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_PLL0_SSCG`"]
pub struct PDEN_PLL0_SSCG_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_PLL0_SSCG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_PLL0_SSCG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PLL0 Sread spectrum module is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_PLL0_SSCG_A::POWEREDON)
    }
    #[doc = "PLL0 Sread spectrum module is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_PLL0_SSCG_A::POWEREDOFF)
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
impl R {
    #[doc = "Bit 3 - Controls power to VBAT Brown Out Detector (BOD)."]
    #[inline(always)]
    pub fn pden_bodvbat(&self) -> PDEN_BODVBAT_R {
        PDEN_BODVBAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Controls power to the Free Running Oscillator (FRO) 32 KHz."]
    #[inline(always)]
    pub fn pden_fro32k(&self) -> PDEN_FRO32K_R {
        PDEN_FRO32K_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Controls power to crystal 32 KHz."]
    #[inline(always)]
    pub fn pden_xtal32k(&self) -> PDEN_XTAL32K_R {
        PDEN_XTAL32K_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Controls power to crystal 32 MHz."]
    #[inline(always)]
    pub fn pden_xtal32m(&self) -> PDEN_XTAL32M_R {
        PDEN_XTAL32M_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Controls power to System PLL (also refered as PLL0)."]
    #[inline(always)]
    pub fn pden_pll0(&self) -> PDEN_PLL0_R {
        PDEN_PLL0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Controls power to USB PLL (also refered as PLL1)."]
    #[inline(always)]
    pub fn pden_pll1(&self) -> PDEN_PLL1_R {
        PDEN_PLL1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Controls power to USB Full Speed phy."]
    #[inline(always)]
    pub fn pden_usbfsphy(&self) -> PDEN_USBFSPHY_R {
        PDEN_USBFSPHY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Controls power to USB High Speed Phy."]
    #[inline(always)]
    pub fn pden_usbhsphy(&self) -> PDEN_USBHSPHY_R {
        PDEN_USBHSPHY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Controls power to Analog Comparator."]
    #[inline(always)]
    pub fn pden_comp(&self) -> PDEN_COMP_R {
        PDEN_COMP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Controls power to USB high speed LDO."]
    #[inline(always)]
    pub fn pden_ldousbhs(&self) -> PDEN_LDOUSBHS_R {
        PDEN_LDOUSBHS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Controls power to auxiliary biasing (AUXBIAS)"]
    #[inline(always)]
    pub fn pden_auxbias(&self) -> PDEN_AUXBIAS_R {
        PDEN_AUXBIAS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Controls power to crystal 32 MHz LDO."]
    #[inline(always)]
    pub fn pden_ldoxo32m(&self) -> PDEN_LDOXO32M_R {
        PDEN_LDOXO32M_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Controls power to all True Random Number Genetaor (TRNG) clock sources."]
    #[inline(always)]
    pub fn pden_rng(&self) -> PDEN_RNG_R {
        PDEN_RNG_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Controls power to System PLL (PLL0) Spread Spectrum module."]
    #[inline(always)]
    pub fn pden_pll0_sscg(&self) -> PDEN_PLL0_SSCG_R {
        PDEN_PLL0_SSCG_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Controls power to VBAT Brown Out Detector (BOD)."]
    #[inline(always)]
    pub fn pden_bodvbat(&mut self) -> PDEN_BODVBAT_W {
        PDEN_BODVBAT_W { w: self }
    }
    #[doc = "Bit 6 - Controls power to the Free Running Oscillator (FRO) 32 KHz."]
    #[inline(always)]
    pub fn pden_fro32k(&mut self) -> PDEN_FRO32K_W {
        PDEN_FRO32K_W { w: self }
    }
    #[doc = "Bit 7 - Controls power to crystal 32 KHz."]
    #[inline(always)]
    pub fn pden_xtal32k(&mut self) -> PDEN_XTAL32K_W {
        PDEN_XTAL32K_W { w: self }
    }
    #[doc = "Bit 8 - Controls power to crystal 32 MHz."]
    #[inline(always)]
    pub fn pden_xtal32m(&mut self) -> PDEN_XTAL32M_W {
        PDEN_XTAL32M_W { w: self }
    }
    #[doc = "Bit 9 - Controls power to System PLL (also refered as PLL0)."]
    #[inline(always)]
    pub fn pden_pll0(&mut self) -> PDEN_PLL0_W {
        PDEN_PLL0_W { w: self }
    }
    #[doc = "Bit 10 - Controls power to USB PLL (also refered as PLL1)."]
    #[inline(always)]
    pub fn pden_pll1(&mut self) -> PDEN_PLL1_W {
        PDEN_PLL1_W { w: self }
    }
    #[doc = "Bit 11 - Controls power to USB Full Speed phy."]
    #[inline(always)]
    pub fn pden_usbfsphy(&mut self) -> PDEN_USBFSPHY_W {
        PDEN_USBFSPHY_W { w: self }
    }
    #[doc = "Bit 12 - Controls power to USB High Speed Phy."]
    #[inline(always)]
    pub fn pden_usbhsphy(&mut self) -> PDEN_USBHSPHY_W {
        PDEN_USBHSPHY_W { w: self }
    }
    #[doc = "Bit 13 - Controls power to Analog Comparator."]
    #[inline(always)]
    pub fn pden_comp(&mut self) -> PDEN_COMP_W {
        PDEN_COMP_W { w: self }
    }
    #[doc = "Bit 18 - Controls power to USB high speed LDO."]
    #[inline(always)]
    pub fn pden_ldousbhs(&mut self) -> PDEN_LDOUSBHS_W {
        PDEN_LDOUSBHS_W { w: self }
    }
    #[doc = "Bit 19 - Controls power to auxiliary biasing (AUXBIAS)"]
    #[inline(always)]
    pub fn pden_auxbias(&mut self) -> PDEN_AUXBIAS_W {
        PDEN_AUXBIAS_W { w: self }
    }
    #[doc = "Bit 20 - Controls power to crystal 32 MHz LDO."]
    #[inline(always)]
    pub fn pden_ldoxo32m(&mut self) -> PDEN_LDOXO32M_W {
        PDEN_LDOXO32M_W { w: self }
    }
    #[doc = "Bit 22 - Controls power to all True Random Number Genetaor (TRNG) clock sources."]
    #[inline(always)]
    pub fn pden_rng(&mut self) -> PDEN_RNG_W {
        PDEN_RNG_W { w: self }
    }
    #[doc = "Bit 23 - Controls power to System PLL (PLL0) Spread Spectrum module."]
    #[inline(always)]
    pub fn pden_pll0_sscg(&mut self) -> PDEN_PLL0_SSCG_W {
        PDEN_PLL0_SSCG_W { w: self }
    }
}
