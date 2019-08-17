#[doc = "Reader of register PDSLEEPCFG0"]
pub type R = crate::R<u32, super::PDSLEEPCFG0>;
#[doc = "Writer for register PDSLEEPCFG0"]
pub type W = crate::W<u32, super::PDSLEEPCFG0>;
#[doc = "Register PDSLEEPCFG0 `reset()`'s with value 0xc0"]
impl crate::ResetValue for super::PDSLEEPCFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc0
    }
}
#[doc = "Possible values of the field `PDEN_DCDC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_DCDC_A {
    #[doc = "DCDC is powered on during low power mode.."]
    POWEREDON,
    #[doc = "DCDC is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_DCDC_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_DCDC_A) -> Self {
        match variant {
            PDEN_DCDC_A::POWEREDON => false,
            PDEN_DCDC_A::POWEREDOFF => true,
        }
    }
}
#[doc = "Reader of field `PDEN_DCDC`"]
pub type PDEN_DCDC_R = crate::R<bool, PDEN_DCDC_A>;
impl PDEN_DCDC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_DCDC_A {
        match self.bits {
            false => PDEN_DCDC_A::POWEREDON,
            true => PDEN_DCDC_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_DCDC_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_DCDC_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_DCDC`"]
pub struct PDEN_DCDC_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_DCDC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_DCDC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DCDC is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_DCDC_A::POWEREDON)
    }
    #[doc = "DCDC is powered off during low power mode.."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_DCDC_A::POWEREDOFF)
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
#[doc = "Possible values of the field `PDEN_BIAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_BIAS_A {
    #[doc = "Analog Bias is powered on during low power mode.."]
    POWEREDON,
    #[doc = "Analog Bias is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_BIAS_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_BIAS_A) -> Self {
        match variant {
            PDEN_BIAS_A::POWEREDON => false,
            PDEN_BIAS_A::POWEREDOFF => true,
        }
    }
}
#[doc = "Reader of field `PDEN_BIAS`"]
pub type PDEN_BIAS_R = crate::R<bool, PDEN_BIAS_A>;
impl PDEN_BIAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_BIAS_A {
        match self.bits {
            false => PDEN_BIAS_A::POWEREDON,
            true => PDEN_BIAS_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_BIAS_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_BIAS_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_BIAS`"]
pub struct PDEN_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_BIAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_BIAS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Analog Bias is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_BIAS_A::POWEREDON)
    }
    #[doc = "Analog Bias is powered off during low power mode.."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_BIAS_A::POWEREDOFF)
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
#[doc = "Possible values of the field `PDEN_BODCORE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_BODCORE_A {
    #[doc = "BOD CORE is powered on during low power mode.."]
    POWEREDON,
    #[doc = "BOD CORE is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_BODCORE_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_BODCORE_A) -> Self {
        match variant {
            PDEN_BODCORE_A::POWEREDON => false,
            PDEN_BODCORE_A::POWEREDOFF => true,
        }
    }
}
#[doc = "Reader of field `PDEN_BODCORE`"]
pub type PDEN_BODCORE_R = crate::R<bool, PDEN_BODCORE_A>;
impl PDEN_BODCORE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_BODCORE_A {
        match self.bits {
            false => PDEN_BODCORE_A::POWEREDON,
            true => PDEN_BODCORE_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_BODCORE_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_BODCORE_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_BODCORE`"]
pub struct PDEN_BODCORE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_BODCORE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_BODCORE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BOD CORE is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_BODCORE_A::POWEREDON)
    }
    #[doc = "BOD CORE is powered off during low power mode.."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_BODCORE_A::POWEREDOFF)
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
#[doc = "Possible values of the field `PDEN_BODVBAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_BODVBAT_A {
    #[doc = "BOD VBAT is powered on during low power mode.."]
    POWEREDON,
    #[doc = "BOD VBAT is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_BODVBAT_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_BODVBAT_A) -> Self {
        match variant {
            PDEN_BODVBAT_A::POWEREDON => false,
            PDEN_BODVBAT_A::POWEREDOFF => true,
        }
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
    #[doc = "BOD VBAT is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_BODVBAT_A::POWEREDON)
    }
    #[doc = "BOD VBAT is powered off during low power mode.."]
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
#[doc = "Possible values of the field `PDEN_FRO1M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_FRO1M_A {
    #[doc = "FRO 1MHz is powered on during low power mode.."]
    POWEREDON,
    #[doc = "FRO 1MHz is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_FRO1M_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_FRO1M_A) -> Self {
        match variant {
            PDEN_FRO1M_A::POWEREDON => false,
            PDEN_FRO1M_A::POWEREDOFF => true,
        }
    }
}
#[doc = "Reader of field `PDEN_FRO1M`"]
pub type PDEN_FRO1M_R = crate::R<bool, PDEN_FRO1M_A>;
impl PDEN_FRO1M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_FRO1M_A {
        match self.bits {
            false => PDEN_FRO1M_A::POWEREDON,
            true => PDEN_FRO1M_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_FRO1M_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_FRO1M_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_FRO1M`"]
pub struct PDEN_FRO1M_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_FRO1M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_FRO1M_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FRO 1MHz is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_FRO1M_A::POWEREDON)
    }
    #[doc = "FRO 1MHz is powered off during low power mode.."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_FRO1M_A::POWEREDOFF)
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
#[doc = "Possible values of the field `PDEN_FRO192M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_FRO192M_A {
    #[doc = "FRO 192 MHz is powered on during low power mode.."]
    POWEREDON,
    #[doc = "FRO 192 MHz is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_FRO192M_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_FRO192M_A) -> Self {
        match variant {
            PDEN_FRO192M_A::POWEREDON => false,
            PDEN_FRO192M_A::POWEREDOFF => true,
        }
    }
}
#[doc = "Reader of field `PDEN_FRO192M`"]
pub type PDEN_FRO192M_R = crate::R<bool, PDEN_FRO192M_A>;
impl PDEN_FRO192M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_FRO192M_A {
        match self.bits {
            false => PDEN_FRO192M_A::POWEREDON,
            true => PDEN_FRO192M_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_FRO192M_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_FRO192M_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_FRO192M`"]
pub struct PDEN_FRO192M_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_FRO192M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_FRO192M_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FRO 192 MHz is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_FRO192M_A::POWEREDON)
    }
    #[doc = "FRO 192 MHz is powered off during low power mode.."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_FRO192M_A::POWEREDOFF)
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
#[doc = "Possible values of the field `PDEN_FRO32K`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_FRO32K_A {
    #[doc = "FRO 32 KHz is powered on during low power mode.."]
    POWEREDON,
    #[doc = "FRO 32 KHz is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_FRO32K_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_FRO32K_A) -> Self {
        match variant {
            PDEN_FRO32K_A::POWEREDON => false,
            PDEN_FRO32K_A::POWEREDOFF => true,
        }
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
    #[doc = "FRO 32 KHz is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_FRO32K_A::POWEREDON)
    }
    #[doc = "FRO 32 KHz is powered off during low power mode.."]
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
#[doc = "Possible values of the field `PDEN_XTAL32K`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_XTAL32K_A {
    #[doc = "crystal 32 KHz is powered on during low power mode.."]
    POWEREDON,
    #[doc = "crystal 32 KHz is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_XTAL32K_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_XTAL32K_A) -> Self {
        match variant {
            PDEN_XTAL32K_A::POWEREDON => false,
            PDEN_XTAL32K_A::POWEREDOFF => true,
        }
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
    #[doc = "crystal 32 KHz is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_XTAL32K_A::POWEREDON)
    }
    #[doc = "crystal 32 KHz is powered off during low power mode.."]
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
#[doc = "Possible values of the field `PDEN_XTAL32M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_XTAL32M_A {
    #[doc = "crystal 32 MHz is powered on during low power mode.."]
    POWEREDON,
    #[doc = "crystal 32 MHz is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_XTAL32M_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_XTAL32M_A) -> Self {
        match variant {
            PDEN_XTAL32M_A::POWEREDON => false,
            PDEN_XTAL32M_A::POWEREDOFF => true,
        }
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
    #[doc = "crystal 32 MHz is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_XTAL32M_A::POWEREDON)
    }
    #[doc = "crystal 32 MHz is powered off during low power mode.."]
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
#[doc = "Possible values of the field `PDEN_PLL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_PLL0_A {
    #[doc = "System PLL (also refered as PLL0) is powered on during low power mode.."]
    POWEREDON,
    #[doc = "System PLL (also refered as PLL0) is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_PLL0_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_PLL0_A) -> Self {
        match variant {
            PDEN_PLL0_A::POWEREDON => false,
            PDEN_PLL0_A::POWEREDOFF => true,
        }
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
    #[doc = "System PLL (also refered as PLL0) is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_PLL0_A::POWEREDON)
    }
    #[doc = "System PLL (also refered as PLL0) is powered off during low power mode.."]
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
#[doc = "Possible values of the field `PDEN_PLL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_PLL1_A {
    #[doc = "USB PLL (also refered as PLL1) is powered on during low power mode.."]
    POWEREDON,
    #[doc = "USB PLL (also refered as PLL1) is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_PLL1_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_PLL1_A) -> Self {
        match variant {
            PDEN_PLL1_A::POWEREDON => false,
            PDEN_PLL1_A::POWEREDOFF => true,
        }
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
    #[doc = "USB PLL (also refered as PLL1) is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_PLL1_A::POWEREDON)
    }
    #[doc = "USB PLL (also refered as PLL1) is powered off during low power mode.."]
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
#[doc = "Possible values of the field `PDEN_USBFSPHY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_USBFSPHY_A {
    #[doc = "USB Full Speed phy is powered on during low power mode.."]
    POWEREDON,
    #[doc = "USB Full Speed phy is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_USBFSPHY_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_USBFSPHY_A) -> Self {
        match variant {
            PDEN_USBFSPHY_A::POWEREDON => false,
            PDEN_USBFSPHY_A::POWEREDOFF => true,
        }
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
    #[doc = "USB Full Speed phy is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_USBFSPHY_A::POWEREDON)
    }
    #[doc = "USB Full Speed phy is powered off during low power mode.."]
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
#[doc = "Possible values of the field `PDEN_USBHSPHY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_USBHSPHY_A {
    #[doc = "USB High Speed Phy is powered on during low power mode.."]
    POWEREDON,
    #[doc = "USB High Speed Phy is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_USBHSPHY_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_USBHSPHY_A) -> Self {
        match variant {
            PDEN_USBHSPHY_A::POWEREDON => false,
            PDEN_USBHSPHY_A::POWEREDOFF => true,
        }
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
    #[doc = "USB High Speed Phy is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_USBHSPHY_A::POWEREDON)
    }
    #[doc = "USB High Speed Phy is powered off during low power mode.."]
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
#[doc = "Possible values of the field `PDEN_COMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_COMP_A {
    #[doc = "Analog Comparator is powered on during low power mode.."]
    POWEREDON,
    #[doc = "Analog Comparator is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_COMP_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_COMP_A) -> Self {
        match variant {
            PDEN_COMP_A::POWEREDON => false,
            PDEN_COMP_A::POWEREDOFF => true,
        }
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
    #[doc = "Analog Comparator is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_COMP_A::POWEREDON)
    }
    #[doc = "Analog Comparator is powered off during low power mode.."]
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
#[doc = "Possible values of the field `PDEN_TEMPSENS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_TEMPSENS_A {
    #[doc = "Temperature Sensor is powered on during low power mode.."]
    POWEREDON,
    #[doc = "Temperature Sensor is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_TEMPSENS_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_TEMPSENS_A) -> Self {
        match variant {
            PDEN_TEMPSENS_A::POWEREDON => false,
            PDEN_TEMPSENS_A::POWEREDOFF => true,
        }
    }
}
#[doc = "Reader of field `PDEN_TEMPSENS`"]
pub type PDEN_TEMPSENS_R = crate::R<bool, PDEN_TEMPSENS_A>;
impl PDEN_TEMPSENS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_TEMPSENS_A {
        match self.bits {
            false => PDEN_TEMPSENS_A::POWEREDON,
            true => PDEN_TEMPSENS_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_TEMPSENS_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_TEMPSENS_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_TEMPSENS`"]
pub struct PDEN_TEMPSENS_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_TEMPSENS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_TEMPSENS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Temperature Sensor is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_TEMPSENS_A::POWEREDON)
    }
    #[doc = "Temperature Sensor is powered off during low power mode.."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_TEMPSENS_A::POWEREDOFF)
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
#[doc = "Possible values of the field `PDEN_GPADC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_GPADC_A {
    #[doc = "General Purpose ADC (GPADC) is powered on during low power mode.."]
    POWEREDON,
    #[doc = "General Purpose ADC (GPADC) is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_GPADC_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_GPADC_A) -> Self {
        match variant {
            PDEN_GPADC_A::POWEREDON => false,
            PDEN_GPADC_A::POWEREDOFF => true,
        }
    }
}
#[doc = "Reader of field `PDEN_GPADC`"]
pub type PDEN_GPADC_R = crate::R<bool, PDEN_GPADC_A>;
impl PDEN_GPADC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_GPADC_A {
        match self.bits {
            false => PDEN_GPADC_A::POWEREDON,
            true => PDEN_GPADC_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_GPADC_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_GPADC_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_GPADC`"]
pub struct PDEN_GPADC_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_GPADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_GPADC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "General Purpose ADC (GPADC) is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_GPADC_A::POWEREDON)
    }
    #[doc = "General Purpose ADC (GPADC) is powered off during low power mode.."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_GPADC_A::POWEREDOFF)
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
#[doc = "Possible values of the field `PDEN_LDOMEM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_LDOMEM_A {
    #[doc = "Memories LDO is powered on during low power mode.."]
    POWEREDON,
    #[doc = "Memories LDO is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_LDOMEM_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_LDOMEM_A) -> Self {
        match variant {
            PDEN_LDOMEM_A::POWEREDON => false,
            PDEN_LDOMEM_A::POWEREDOFF => true,
        }
    }
}
#[doc = "Reader of field `PDEN_LDOMEM`"]
pub type PDEN_LDOMEM_R = crate::R<bool, PDEN_LDOMEM_A>;
impl PDEN_LDOMEM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_LDOMEM_A {
        match self.bits {
            false => PDEN_LDOMEM_A::POWEREDON,
            true => PDEN_LDOMEM_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_LDOMEM_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_LDOMEM_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_LDOMEM`"]
pub struct PDEN_LDOMEM_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_LDOMEM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_LDOMEM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Memories LDO is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_LDOMEM_A::POWEREDON)
    }
    #[doc = "Memories LDO is powered off during low power mode.."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_LDOMEM_A::POWEREDOFF)
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
#[doc = "Possible values of the field `PDEN_LDODEEPSLEEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_LDODEEPSLEEP_A {
    #[doc = "Deep Sleep LDO is powered on during low power mode.."]
    POWEREDON,
    #[doc = "Deep Sleep LDO is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_LDODEEPSLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_LDODEEPSLEEP_A) -> Self {
        match variant {
            PDEN_LDODEEPSLEEP_A::POWEREDON => false,
            PDEN_LDODEEPSLEEP_A::POWEREDOFF => true,
        }
    }
}
#[doc = "Reader of field `PDEN_LDODEEPSLEEP`"]
pub type PDEN_LDODEEPSLEEP_R = crate::R<bool, PDEN_LDODEEPSLEEP_A>;
impl PDEN_LDODEEPSLEEP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_LDODEEPSLEEP_A {
        match self.bits {
            false => PDEN_LDODEEPSLEEP_A::POWEREDON,
            true => PDEN_LDODEEPSLEEP_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_LDODEEPSLEEP_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_LDODEEPSLEEP_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_LDODEEPSLEEP`"]
pub struct PDEN_LDODEEPSLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_LDODEEPSLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_LDODEEPSLEEP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Deep Sleep LDO is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_LDODEEPSLEEP_A::POWEREDON)
    }
    #[doc = "Deep Sleep LDO is powered off during low power mode.."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_LDODEEPSLEEP_A::POWEREDOFF)
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
#[doc = "Possible values of the field `PDEN_LDOUSBHS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_LDOUSBHS_A {
    #[doc = "USB high speed LDO is powered on during low power mode.."]
    POWEREDON,
    #[doc = "USB high speed LDO is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_LDOUSBHS_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_LDOUSBHS_A) -> Self {
        match variant {
            PDEN_LDOUSBHS_A::POWEREDON => false,
            PDEN_LDOUSBHS_A::POWEREDOFF => true,
        }
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
    #[doc = "USB high speed LDO is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_LDOUSBHS_A::POWEREDON)
    }
    #[doc = "USB high speed LDO is powered off during low power mode.."]
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
#[doc = "Possible values of the field `PDEN_AUXBIAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_AUXBIAS_A {
    #[doc = "is powered on during low power mode.."]
    POWEREDON,
    #[doc = "is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_AUXBIAS_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_AUXBIAS_A) -> Self {
        match variant {
            PDEN_AUXBIAS_A::POWEREDON => false,
            PDEN_AUXBIAS_A::POWEREDOFF => true,
        }
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
    #[doc = "is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_AUXBIAS_A::POWEREDON)
    }
    #[doc = "is powered off during low power mode.."]
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
#[doc = "Possible values of the field `PDEN_LDOXO32M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_LDOXO32M_A {
    #[doc = "crystal 32 MHz LDO is powered on during low power mode.."]
    POWEREDON,
    #[doc = "crystal 32 MHz LDO is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_LDOXO32M_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_LDOXO32M_A) -> Self {
        match variant {
            PDEN_LDOXO32M_A::POWEREDON => false,
            PDEN_LDOXO32M_A::POWEREDOFF => true,
        }
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
    #[doc = "crystal 32 MHz LDO is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_LDOXO32M_A::POWEREDON)
    }
    #[doc = "crystal 32 MHz LDO is powered off during low power mode.."]
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
#[doc = "Possible values of the field `PDEN_LDOFLASHNV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_LDOFLASHNV_A {
    #[doc = "Flash NV (high voltage) is powered on during low power mode.."]
    POWEREDON,
    #[doc = "Flash NV (high voltage) is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_LDOFLASHNV_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_LDOFLASHNV_A) -> Self {
        match variant {
            PDEN_LDOFLASHNV_A::POWEREDON => false,
            PDEN_LDOFLASHNV_A::POWEREDOFF => true,
        }
    }
}
#[doc = "Reader of field `PDEN_LDOFLASHNV`"]
pub type PDEN_LDOFLASHNV_R = crate::R<bool, PDEN_LDOFLASHNV_A>;
impl PDEN_LDOFLASHNV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_LDOFLASHNV_A {
        match self.bits {
            false => PDEN_LDOFLASHNV_A::POWEREDON,
            true => PDEN_LDOFLASHNV_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_LDOFLASHNV_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_LDOFLASHNV_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_LDOFLASHNV`"]
pub struct PDEN_LDOFLASHNV_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_LDOFLASHNV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_LDOFLASHNV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash NV (high voltage) is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_LDOFLASHNV_A::POWEREDON)
    }
    #[doc = "Flash NV (high voltage) is powered off during low power mode.."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_LDOFLASHNV_A::POWEREDOFF)
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
#[doc = "Possible values of the field `PDEN_RNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_RNG_A {
    #[doc = "True Random Number Genetaor (TRNG) clock sources are powered on during low power mode.."]
    POWEREDON,
    #[doc = "True Random Number Genetaor (TRNG) clock sources are powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_RNG_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_RNG_A) -> Self {
        match variant {
            PDEN_RNG_A::POWEREDON => false,
            PDEN_RNG_A::POWEREDOFF => true,
        }
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
    #[doc = "True Random Number Genetaor (TRNG) clock sources are powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_RNG_A::POWEREDON)
    }
    #[doc = "True Random Number Genetaor (TRNG) clock sources are powered off during low power mode.."]
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
#[doc = "Possible values of the field `PDEN_PLL0_SSCG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_PLL0_SSCG_A {
    #[doc = "PLL0 Spread Sprectrum module is powered on during low power mode.."]
    POWEREDON,
    #[doc = "PLL0 Spread Sprectrum module is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_PLL0_SSCG_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_PLL0_SSCG_A) -> Self {
        match variant {
            PDEN_PLL0_SSCG_A::POWEREDON => false,
            PDEN_PLL0_SSCG_A::POWEREDOFF => true,
        }
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
    #[doc = "PLL0 Spread Sprectrum module is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_PLL0_SSCG_A::POWEREDON)
    }
    #[doc = "PLL0 Spread Sprectrum module is powered off during low power mode.."]
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
#[doc = "Possible values of the field `PDEN_ROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_ROM_A {
    #[doc = "ROM is powered on during low power mode.."]
    POWEREDON,
    #[doc = "ROM is powered off during low power mode.."]
    POWEREDOFF,
}
impl From<PDEN_ROM_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_ROM_A) -> Self {
        match variant {
            PDEN_ROM_A::POWEREDON => false,
            PDEN_ROM_A::POWEREDOFF => true,
        }
    }
}
#[doc = "Reader of field `PDEN_ROM`"]
pub type PDEN_ROM_R = crate::R<bool, PDEN_ROM_A>;
impl PDEN_ROM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_ROM_A {
        match self.bits {
            false => PDEN_ROM_A::POWEREDON,
            true => PDEN_ROM_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_ROM_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_ROM_A::POWEREDOFF
    }
}
#[doc = "Write proxy for field `PDEN_ROM`"]
pub struct PDEN_ROM_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_ROM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_ROM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ROM is powered on during low power mode.."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_ROM_A::POWEREDON)
    }
    #[doc = "ROM is powered off during low power mode.."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_ROM_A::POWEREDOFF)
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
impl R {
    #[doc = "Bit 0 - Controls DCDC power during DEEP SLEEP (DCDC is always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_dcdc(&self) -> PDEN_DCDC_R {
        PDEN_DCDC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Controls Analog Bias power during DEEP SLEEP and POWER DOWN (always shut down during DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_bias(&self) -> PDEN_BIAS_R {
        PDEN_BIAS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Controls Core Logic BoD power during DEEP SLEEP and POWER DOWN (always shut down during DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_bodcore(&self) -> PDEN_BODCORE_R {
        PDEN_BODCORE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Controls VBAT BoD power during DEEP SLEEP and POWER DOWN (always shut down during DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_bodvbat(&self) -> PDEN_BODVBAT_R {
        PDEN_BODVBAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Controls 1 MHz Free Running Oscillator power during DEEP SLEEP, POWER DOWN and DEEP POWER DOWN."]
    #[inline(always)]
    pub fn pden_fro1m(&self) -> PDEN_FRO1M_R {
        PDEN_FRO1M_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Controls 192MHz Free Running Oscillator power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_fro192m(&self) -> PDEN_FRO192M_R {
        PDEN_FRO192M_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Controls power during DEEP SLEEP, POWER DOWN and DEEP POWER DOWN."]
    #[inline(always)]
    pub fn pden_fro32k(&self) -> PDEN_FRO32K_R {
        PDEN_FRO32K_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Controls crystal 32 KHz power during DEEP SLEEP, POWER DOWN and DEEP POWER DOWN."]
    #[inline(always)]
    pub fn pden_xtal32k(&self) -> PDEN_XTAL32K_R {
        PDEN_XTAL32K_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Controls crystal 32 MHz power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_xtal32m(&self) -> PDEN_XTAL32M_R {
        PDEN_XTAL32M_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Controls System PLL (also refered as PLL0) power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_pll0(&self) -> PDEN_PLL0_R {
        PDEN_PLL0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Controls USB PLL (also refered as PLL1) power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_pll1(&self) -> PDEN_PLL1_R {
        PDEN_PLL1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Controls USB Full Speed phy power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_usbfsphy(&self) -> PDEN_USBFSPHY_R {
        PDEN_USBFSPHY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Controls USB High Speed Phy power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_usbhsphy(&self) -> PDEN_USBHSPHY_R {
        PDEN_USBHSPHY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Controls Analog Comparator power during DEEP SLEEP and POWER DOWN (always shut down during DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_comp(&self) -> PDEN_COMP_R {
        PDEN_COMP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Controls Temperature Sensor power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_tempsens(&self) -> PDEN_TEMPSENS_R {
        PDEN_TEMPSENS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Controls General Purpose ADC (GPADC) power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_gpadc(&self) -> PDEN_GPADC_R {
        PDEN_GPADC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Controls Memories LDO power during DEEP SLEEP, POWER DOWN and DEEP POWER DOWN."]
    #[inline(always)]
    pub fn pden_ldomem(&self) -> PDEN_LDOMEM_R {
        PDEN_LDOMEM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Controls Deep Sleep LDO power during DEEP SLEEP and POWER DOWN (always shut down during DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_ldodeepsleep(&self) -> PDEN_LDODEEPSLEEP_R {
        PDEN_LDODEEPSLEEP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Controls USB high speed LDO power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_ldousbhs(&self) -> PDEN_LDOUSBHS_R {
        PDEN_LDOUSBHS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - during DEEP SLEEP and POWER DOWN (always shut down during DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_auxbias(&self) -> PDEN_AUXBIAS_R {
        PDEN_AUXBIAS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Controls crystal 32 MHz LDO power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_ldoxo32m(&self) -> PDEN_LDOXO32M_R {
        PDEN_LDOXO32M_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Controls Flash NV (high voltage) LDO power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_ldoflashnv(&self) -> PDEN_LDOFLASHNV_R {
        PDEN_LDOFLASHNV_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Controls True Random Number Genetaor (TRNG) clock sources power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_rng(&self) -> PDEN_RNG_R {
        PDEN_RNG_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Controls PLL0 Spread Sprectrum module power during DEEP SLEEP (PLL0 Spread Spectrum is always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_pll0_sscg(&self) -> PDEN_PLL0_SSCG_R {
        PDEN_PLL0_SSCG_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Controls ROM power during DEEP SLEEP (ROM is always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_rom(&self) -> PDEN_ROM_R {
        PDEN_ROM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls DCDC power during DEEP SLEEP (DCDC is always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_dcdc(&mut self) -> PDEN_DCDC_W {
        PDEN_DCDC_W { w: self }
    }
    #[doc = "Bit 1 - Controls Analog Bias power during DEEP SLEEP and POWER DOWN (always shut down during DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_bias(&mut self) -> PDEN_BIAS_W {
        PDEN_BIAS_W { w: self }
    }
    #[doc = "Bit 2 - Controls Core Logic BoD power during DEEP SLEEP and POWER DOWN (always shut down during DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_bodcore(&mut self) -> PDEN_BODCORE_W {
        PDEN_BODCORE_W { w: self }
    }
    #[doc = "Bit 3 - Controls VBAT BoD power during DEEP SLEEP and POWER DOWN (always shut down during DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_bodvbat(&mut self) -> PDEN_BODVBAT_W {
        PDEN_BODVBAT_W { w: self }
    }
    #[doc = "Bit 4 - Controls 1 MHz Free Running Oscillator power during DEEP SLEEP, POWER DOWN and DEEP POWER DOWN."]
    #[inline(always)]
    pub fn pden_fro1m(&mut self) -> PDEN_FRO1M_W {
        PDEN_FRO1M_W { w: self }
    }
    #[doc = "Bit 5 - Controls 192MHz Free Running Oscillator power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_fro192m(&mut self) -> PDEN_FRO192M_W {
        PDEN_FRO192M_W { w: self }
    }
    #[doc = "Bit 6 - Controls power during DEEP SLEEP, POWER DOWN and DEEP POWER DOWN."]
    #[inline(always)]
    pub fn pden_fro32k(&mut self) -> PDEN_FRO32K_W {
        PDEN_FRO32K_W { w: self }
    }
    #[doc = "Bit 7 - Controls crystal 32 KHz power during DEEP SLEEP, POWER DOWN and DEEP POWER DOWN."]
    #[inline(always)]
    pub fn pden_xtal32k(&mut self) -> PDEN_XTAL32K_W {
        PDEN_XTAL32K_W { w: self }
    }
    #[doc = "Bit 8 - Controls crystal 32 MHz power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_xtal32m(&mut self) -> PDEN_XTAL32M_W {
        PDEN_XTAL32M_W { w: self }
    }
    #[doc = "Bit 9 - Controls System PLL (also refered as PLL0) power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_pll0(&mut self) -> PDEN_PLL0_W {
        PDEN_PLL0_W { w: self }
    }
    #[doc = "Bit 10 - Controls USB PLL (also refered as PLL1) power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_pll1(&mut self) -> PDEN_PLL1_W {
        PDEN_PLL1_W { w: self }
    }
    #[doc = "Bit 11 - Controls USB Full Speed phy power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_usbfsphy(&mut self) -> PDEN_USBFSPHY_W {
        PDEN_USBFSPHY_W { w: self }
    }
    #[doc = "Bit 12 - Controls USB High Speed Phy power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_usbhsphy(&mut self) -> PDEN_USBHSPHY_W {
        PDEN_USBHSPHY_W { w: self }
    }
    #[doc = "Bit 13 - Controls Analog Comparator power during DEEP SLEEP and POWER DOWN (always shut down during DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_comp(&mut self) -> PDEN_COMP_W {
        PDEN_COMP_W { w: self }
    }
    #[doc = "Bit 14 - Controls Temperature Sensor power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_tempsens(&mut self) -> PDEN_TEMPSENS_W {
        PDEN_TEMPSENS_W { w: self }
    }
    #[doc = "Bit 15 - Controls General Purpose ADC (GPADC) power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_gpadc(&mut self) -> PDEN_GPADC_W {
        PDEN_GPADC_W { w: self }
    }
    #[doc = "Bit 16 - Controls Memories LDO power during DEEP SLEEP, POWER DOWN and DEEP POWER DOWN."]
    #[inline(always)]
    pub fn pden_ldomem(&mut self) -> PDEN_LDOMEM_W {
        PDEN_LDOMEM_W { w: self }
    }
    #[doc = "Bit 17 - Controls Deep Sleep LDO power during DEEP SLEEP and POWER DOWN (always shut down during DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_ldodeepsleep(&mut self) -> PDEN_LDODEEPSLEEP_W {
        PDEN_LDODEEPSLEEP_W { w: self }
    }
    #[doc = "Bit 18 - Controls USB high speed LDO power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_ldousbhs(&mut self) -> PDEN_LDOUSBHS_W {
        PDEN_LDOUSBHS_W { w: self }
    }
    #[doc = "Bit 19 - during DEEP SLEEP and POWER DOWN (always shut down during DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_auxbias(&mut self) -> PDEN_AUXBIAS_W {
        PDEN_AUXBIAS_W { w: self }
    }
    #[doc = "Bit 20 - Controls crystal 32 MHz LDO power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_ldoxo32m(&mut self) -> PDEN_LDOXO32M_W {
        PDEN_LDOXO32M_W { w: self }
    }
    #[doc = "Bit 21 - Controls Flash NV (high voltage) LDO power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_ldoflashnv(&mut self) -> PDEN_LDOFLASHNV_W {
        PDEN_LDOFLASHNV_W { w: self }
    }
    #[doc = "Bit 22 - Controls True Random Number Genetaor (TRNG) clock sources power during DEEP SLEEP (always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_rng(&mut self) -> PDEN_RNG_W {
        PDEN_RNG_W { w: self }
    }
    #[doc = "Bit 23 - Controls PLL0 Spread Sprectrum module power during DEEP SLEEP (PLL0 Spread Spectrum is always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_pll0_sscg(&mut self) -> PDEN_PLL0_SSCG_W {
        PDEN_PLL0_SSCG_W { w: self }
    }
    #[doc = "Bit 24 - Controls ROM power during DEEP SLEEP (ROM is always shut down during POWER DOWN and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn pden_rom(&mut self) -> PDEN_ROM_W {
        PDEN_ROM_W { w: self }
    }
}
