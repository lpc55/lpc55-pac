#[doc = "Reader of register PLL_SIC"]
pub type R = crate::R<u32, super::PLL_SIC>;
#[doc = "Writer for register PLL_SIC"]
pub type W = crate::W<u32, super::PLL_SIC>;
#[doc = "Register PLL_SIC `reset()`'s with value 0x00d1_2000"]
impl crate::ResetValue for super::PLL_SIC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00d1_2000
    }
}
#[doc = "Reader of field `PLL_EN_USB_CLKS`"]
pub type PLL_EN_USB_CLKS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL_EN_USB_CLKS`"]
pub struct PLL_EN_USB_CLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_EN_USB_CLKS_W<'a> {
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
#[doc = "Reader of field `PLL_POWER`"]
pub type PLL_POWER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL_POWER`"]
pub struct PLL_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_POWER_W<'a> {
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
#[doc = "Reader of field `PLL_ENABLE`"]
pub type PLL_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL_ENABLE`"]
pub struct PLL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_ENABLE_W<'a> {
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
#[doc = "Reference bias power down select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFBIAS_PWD_SEL_A {
    #[doc = "0: Selects PLL_POWER to control the reference bias"]
    VALUE0 = 0,
    #[doc = "1: Selects REFBIAS_PWD to control the reference bias"]
    VALUE1 = 1,
}
impl From<REFBIAS_PWD_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: REFBIAS_PWD_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFBIAS_PWD_SEL`"]
pub type REFBIAS_PWD_SEL_R = crate::R<bool, REFBIAS_PWD_SEL_A>;
impl REFBIAS_PWD_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFBIAS_PWD_SEL_A {
        match self.bits {
            false => REFBIAS_PWD_SEL_A::VALUE0,
            true => REFBIAS_PWD_SEL_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == REFBIAS_PWD_SEL_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REFBIAS_PWD_SEL_A::VALUE1
    }
}
#[doc = "Write proxy for field `REFBIAS_PWD_SEL`"]
pub struct REFBIAS_PWD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFBIAS_PWD_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFBIAS_PWD_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selects PLL_POWER to control the reference bias"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(REFBIAS_PWD_SEL_A::VALUE0)
    }
    #[doc = "Selects REFBIAS_PWD to control the reference bias"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REFBIAS_PWD_SEL_A::VALUE1)
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
#[doc = "Reader of field `REFBIAS_PWD`"]
pub type REFBIAS_PWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFBIAS_PWD`"]
pub struct REFBIAS_PWD_W<'a> {
    w: &'a mut W,
}
impl<'a> REFBIAS_PWD_W<'a> {
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
#[doc = "Reader of field `PLL_REG_ENABLE`"]
pub type PLL_REG_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL_REG_ENABLE`"]
pub struct PLL_REG_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_REG_ENABLE_W<'a> {
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
#[doc = "This field controls the USB PLL feedback loop divider\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLL_DIV_SEL_A {
    #[doc = "0: Divide by 13"]
    VALUE0 = 0,
    #[doc = "1: Divide by 15"]
    VALUE1 = 1,
    #[doc = "2: Divide by 16"]
    VALUE2 = 2,
    #[doc = "3: Divide by 20"]
    VALUE3 = 3,
    #[doc = "4: Divide by 22"]
    VALUE4 = 4,
    #[doc = "5: Divide by 25"]
    VALUE5 = 5,
    #[doc = "6: Divide by 30"]
    VALUE6 = 6,
    #[doc = "7: Divide by 240"]
    VALUE7 = 7,
}
impl From<PLL_DIV_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL_DIV_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLL_DIV_SEL`"]
pub type PLL_DIV_SEL_R = crate::R<u8, PLL_DIV_SEL_A>;
impl PLL_DIV_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_DIV_SEL_A {
        match self.bits {
            0 => PLL_DIV_SEL_A::VALUE0,
            1 => PLL_DIV_SEL_A::VALUE1,
            2 => PLL_DIV_SEL_A::VALUE2,
            3 => PLL_DIV_SEL_A::VALUE3,
            4 => PLL_DIV_SEL_A::VALUE4,
            5 => PLL_DIV_SEL_A::VALUE5,
            6 => PLL_DIV_SEL_A::VALUE6,
            7 => PLL_DIV_SEL_A::VALUE7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == PLL_DIV_SEL_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PLL_DIV_SEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PLL_DIV_SEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PLL_DIV_SEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PLL_DIV_SEL_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PLL_DIV_SEL_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PLL_DIV_SEL_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PLL_DIV_SEL_A::VALUE7
    }
}
#[doc = "Write proxy for field `PLL_DIV_SEL`"]
pub struct PLL_DIV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_DIV_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_DIV_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::VALUE0)
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::VALUE1)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::VALUE2)
    }
    #[doc = "Divide by 20"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::VALUE3)
    }
    #[doc = "Divide by 22"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::VALUE4)
    }
    #[doc = "Divide by 25"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::VALUE5)
    }
    #[doc = "Divide by 30"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::VALUE6)
    }
    #[doc = "Divide by 240"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::VALUE7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | (((value as u32) & 0x07) << 22);
        self.w
    }
}
#[doc = "Reader of field `PLL_PREDIV`"]
pub type PLL_PREDIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL_PREDIV`"]
pub struct PLL_PREDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_PREDIV_W<'a> {
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
#[doc = "USB PLL lock status indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_LOCK_A {
    #[doc = "0: PLL is not currently locked"]
    VALUE0 = 0,
    #[doc = "1: PLL is currently locked"]
    VALUE1 = 1,
}
impl From<PLL_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL_LOCK`"]
pub type PLL_LOCK_R = crate::R<bool, PLL_LOCK_A>;
impl PLL_LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_LOCK_A {
        match self.bits {
            false => PLL_LOCK_A::VALUE0,
            true => PLL_LOCK_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == PLL_LOCK_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PLL_LOCK_A::VALUE1
    }
}
impl R {
    #[doc = "Bit 6 - Enables the USB clock from PLL to USB PHY"]
    #[inline(always)]
    pub fn pll_en_usb_clks(&self) -> PLL_EN_USB_CLKS_R {
        PLL_EN_USB_CLKS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Power up the USB PLL"]
    #[inline(always)]
    pub fn pll_power(&self) -> PLL_POWER_R {
        PLL_POWER_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables the clock output from the USB PLL"]
    #[inline(always)]
    pub fn pll_enable(&self) -> PLL_ENABLE_R {
        PLL_ENABLE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Reference bias power down select."]
    #[inline(always)]
    pub fn refbias_pwd_sel(&self) -> REFBIAS_PWD_SEL_R {
        REFBIAS_PWD_SEL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[inline(always)]
    pub fn refbias_pwd(&self) -> REFBIAS_PWD_R {
        REFBIAS_PWD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - This field controls the USB PLL regulator, set to enable the regulator"]
    #[inline(always)]
    pub fn pll_reg_enable(&self) -> PLL_REG_ENABLE_R {
        PLL_REG_ENABLE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 22:24 - This field controls the USB PLL feedback loop divider"]
    #[inline(always)]
    pub fn pll_div_sel(&self) -> PLL_DIV_SEL_R {
        PLL_DIV_SEL_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bit 30 - This is selection between /1 or /2 to expand the range of ref input clock."]
    #[inline(always)]
    pub fn pll_prediv(&self) -> PLL_PREDIV_R {
        PLL_PREDIV_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - USB PLL lock status indicator"]
    #[inline(always)]
    pub fn pll_lock(&self) -> PLL_LOCK_R {
        PLL_LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Enables the USB clock from PLL to USB PHY"]
    #[inline(always)]
    pub fn pll_en_usb_clks(&mut self) -> PLL_EN_USB_CLKS_W {
        PLL_EN_USB_CLKS_W { w: self }
    }
    #[doc = "Bit 12 - Power up the USB PLL"]
    #[inline(always)]
    pub fn pll_power(&mut self) -> PLL_POWER_W {
        PLL_POWER_W { w: self }
    }
    #[doc = "Bit 13 - Enables the clock output from the USB PLL"]
    #[inline(always)]
    pub fn pll_enable(&mut self) -> PLL_ENABLE_W {
        PLL_ENABLE_W { w: self }
    }
    #[doc = "Bit 19 - Reference bias power down select."]
    #[inline(always)]
    pub fn refbias_pwd_sel(&mut self) -> REFBIAS_PWD_SEL_W {
        REFBIAS_PWD_SEL_W { w: self }
    }
    #[doc = "Bit 20 - Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[inline(always)]
    pub fn refbias_pwd(&mut self) -> REFBIAS_PWD_W {
        REFBIAS_PWD_W { w: self }
    }
    #[doc = "Bit 21 - This field controls the USB PLL regulator, set to enable the regulator"]
    #[inline(always)]
    pub fn pll_reg_enable(&mut self) -> PLL_REG_ENABLE_W {
        PLL_REG_ENABLE_W { w: self }
    }
    #[doc = "Bits 22:24 - This field controls the USB PLL feedback loop divider"]
    #[inline(always)]
    pub fn pll_div_sel(&mut self) -> PLL_DIV_SEL_W {
        PLL_DIV_SEL_W { w: self }
    }
    #[doc = "Bit 30 - This is selection between /1 or /2 to expand the range of ref input clock."]
    #[inline(always)]
    pub fn pll_prediv(&mut self) -> PLL_PREDIV_W {
        PLL_PREDIV_W { w: self }
    }
}
