#[doc = "Register `PLL_SIC` reader"]
pub struct R(crate::R<PLL_SIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_SIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PLL_SIC_SPEC>> for R {
    fn from(reader: crate::R<PLL_SIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_SIC` writer"]
pub struct W(crate::W<PLL_SIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_SIC_SPEC>;
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
impl core::convert::From<crate::W<PLL_SIC_SPEC>> for W {
    fn from(writer: crate::W<PLL_SIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL_EN_USB_CLKS` reader - Enables the USB clock from PLL to USB PHY"]
pub struct PLL_EN_USB_CLKS_R(crate::FieldReader<bool, bool>);
impl PLL_EN_USB_CLKS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL_EN_USB_CLKS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_EN_USB_CLKS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_EN_USB_CLKS` writer - Enables the USB clock from PLL to USB PHY"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `PLL_POWER` reader - Power up the USB PLL"]
pub struct PLL_POWER_R(crate::FieldReader<bool, bool>);
impl PLL_POWER_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL_POWER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_POWER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_POWER` writer - Power up the USB PLL"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `PLL_ENABLE` reader - Enables the clock output from the USB PLL"]
pub struct PLL_ENABLE_R(crate::FieldReader<bool, bool>);
impl PLL_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_ENABLE` writer - Enables the clock output from the USB PLL"]
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
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
#[doc = "Field `REFBIAS_PWD_SEL` reader - Reference bias power down select."]
pub struct REFBIAS_PWD_SEL_R(crate::FieldReader<bool, REFBIAS_PWD_SEL_A>);
impl REFBIAS_PWD_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        REFBIAS_PWD_SEL_R(crate::FieldReader::new(bits))
    }
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
        **self == REFBIAS_PWD_SEL_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == REFBIAS_PWD_SEL_A::VALUE1
    }
}
impl core::ops::Deref for REFBIAS_PWD_SEL_R {
    type Target = crate::FieldReader<bool, REFBIAS_PWD_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFBIAS_PWD_SEL` writer - Reference bias power down select."]
pub struct REFBIAS_PWD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFBIAS_PWD_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFBIAS_PWD_SEL_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `REFBIAS_PWD` reader - Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
pub struct REFBIAS_PWD_R(crate::FieldReader<bool, bool>);
impl REFBIAS_PWD_R {
    pub(crate) fn new(bits: bool) -> Self {
        REFBIAS_PWD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFBIAS_PWD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFBIAS_PWD` writer - Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `PLL_REG_ENABLE` reader - This field controls the USB PLL regulator, set to enable the regulator"]
pub struct PLL_REG_ENABLE_R(crate::FieldReader<bool, bool>);
impl PLL_REG_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL_REG_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_REG_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_REG_ENABLE` writer - This field controls the USB PLL regulator, set to enable the regulator"]
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
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
#[doc = "Field `PLL_DIV_SEL` reader - This field controls the USB PLL feedback loop divider"]
pub struct PLL_DIV_SEL_R(crate::FieldReader<u8, PLL_DIV_SEL_A>);
impl PLL_DIV_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLL_DIV_SEL_R(crate::FieldReader::new(bits))
    }
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
        **self == PLL_DIV_SEL_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PLL_DIV_SEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PLL_DIV_SEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PLL_DIV_SEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PLL_DIV_SEL_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == PLL_DIV_SEL_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == PLL_DIV_SEL_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        **self == PLL_DIV_SEL_A::VALUE7
    }
}
impl core::ops::Deref for PLL_DIV_SEL_R {
    type Target = crate::FieldReader<u8, PLL_DIV_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_DIV_SEL` writer - This field controls the USB PLL feedback loop divider"]
pub struct PLL_DIV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_DIV_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_DIV_SEL_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x07 << 22)) | ((value as u32 & 0x07) << 22);
        self.w
    }
}
#[doc = "Field `PLL_PREDIV` reader - This is selection between /1 or /2 to expand the range of ref input clock."]
pub struct PLL_PREDIV_R(crate::FieldReader<bool, bool>);
impl PLL_PREDIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL_PREDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_PREDIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_PREDIV` writer - This is selection between /1 or /2 to expand the range of ref input clock."]
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
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
#[doc = "Field `PLL_LOCK` reader - USB PLL lock status indicator"]
pub struct PLL_LOCK_R(crate::FieldReader<bool, PLL_LOCK_A>);
impl PLL_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL_LOCK_R(crate::FieldReader::new(bits))
    }
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
        **self == PLL_LOCK_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PLL_LOCK_A::VALUE1
    }
}
impl core::ops::Deref for PLL_LOCK_R {
    type Target = crate::FieldReader<bool, PLL_LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY PLL Control/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_sic](index.html) module"]
pub struct PLL_SIC_SPEC;
impl crate::RegisterSpec for PLL_SIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_sic::R](R) reader structure"]
impl crate::Readable for PLL_SIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_sic::W](W) writer structure"]
impl crate::Writable for PLL_SIC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_SIC to value 0x00d1_2000"]
impl crate::Resettable for PLL_SIC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00d1_2000
    }
}
