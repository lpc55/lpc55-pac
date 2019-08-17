#[doc = "Reader of register USB0CLKCTRL"]
pub type R = crate::R<u32, super::USB0CLKCTRL>;
#[doc = "Writer for register USB0CLKCTRL"]
pub type W = crate::W<u32, super::USB0CLKCTRL>;
#[doc = "Register USB0CLKCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::USB0CLKCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `AP_FS_DEV_CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AP_FS_DEV_CLK_A {
    #[doc = "Under hardware control."]
    HW_CTRL,
    #[doc = "Forced high."]
    FORCED,
}
impl From<AP_FS_DEV_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: AP_FS_DEV_CLK_A) -> Self {
        match variant {
            AP_FS_DEV_CLK_A::HW_CTRL => false,
            AP_FS_DEV_CLK_A::FORCED => true,
        }
    }
}
#[doc = "Reader of field `AP_FS_DEV_CLK`"]
pub type AP_FS_DEV_CLK_R = crate::R<bool, AP_FS_DEV_CLK_A>;
impl AP_FS_DEV_CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AP_FS_DEV_CLK_A {
        match self.bits {
            false => AP_FS_DEV_CLK_A::HW_CTRL,
            true => AP_FS_DEV_CLK_A::FORCED,
        }
    }
    #[doc = "Checks if the value of the field is `HW_CTRL`"]
    #[inline(always)]
    pub fn is_hw_ctrl(&self) -> bool {
        *self == AP_FS_DEV_CLK_A::HW_CTRL
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == AP_FS_DEV_CLK_A::FORCED
    }
}
#[doc = "Write proxy for field `AP_FS_DEV_CLK`"]
pub struct AP_FS_DEV_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> AP_FS_DEV_CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AP_FS_DEV_CLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Under hardware control."]
    #[inline(always)]
    pub fn hw_ctrl(self) -> &'a mut W {
        self.variant(AP_FS_DEV_CLK_A::HW_CTRL)
    }
    #[doc = "Forced high."]
    #[inline(always)]
    pub fn forced(self) -> &'a mut W {
        self.variant(AP_FS_DEV_CLK_A::FORCED)
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
#[doc = "Possible values of the field `POL_FS_DEV_CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_FS_DEV_CLK_A {
    #[doc = "Falling edge of device USB0_NEEDCLK triggers wake-up."]
    FALLING,
    #[doc = "Rising edge of device USB0_NEEDCLK triggers wake-up."]
    RISING,
}
impl From<POL_FS_DEV_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: POL_FS_DEV_CLK_A) -> Self {
        match variant {
            POL_FS_DEV_CLK_A::FALLING => false,
            POL_FS_DEV_CLK_A::RISING => true,
        }
    }
}
#[doc = "Reader of field `POL_FS_DEV_CLK`"]
pub type POL_FS_DEV_CLK_R = crate::R<bool, POL_FS_DEV_CLK_A>;
impl POL_FS_DEV_CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_FS_DEV_CLK_A {
        match self.bits {
            false => POL_FS_DEV_CLK_A::FALLING,
            true => POL_FS_DEV_CLK_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == POL_FS_DEV_CLK_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == POL_FS_DEV_CLK_A::RISING
    }
}
#[doc = "Write proxy for field `POL_FS_DEV_CLK`"]
pub struct POL_FS_DEV_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_FS_DEV_CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL_FS_DEV_CLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Falling edge of device USB0_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(POL_FS_DEV_CLK_A::FALLING)
    }
    #[doc = "Rising edge of device USB0_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(POL_FS_DEV_CLK_A::RISING)
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
#[doc = "Possible values of the field `AP_FS_HOST_CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AP_FS_HOST_CLK_A {
    #[doc = "Under hardware control."]
    HW_CTRL,
    #[doc = "Forced high."]
    FORCED,
}
impl From<AP_FS_HOST_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: AP_FS_HOST_CLK_A) -> Self {
        match variant {
            AP_FS_HOST_CLK_A::HW_CTRL => false,
            AP_FS_HOST_CLK_A::FORCED => true,
        }
    }
}
#[doc = "Reader of field `AP_FS_HOST_CLK`"]
pub type AP_FS_HOST_CLK_R = crate::R<bool, AP_FS_HOST_CLK_A>;
impl AP_FS_HOST_CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AP_FS_HOST_CLK_A {
        match self.bits {
            false => AP_FS_HOST_CLK_A::HW_CTRL,
            true => AP_FS_HOST_CLK_A::FORCED,
        }
    }
    #[doc = "Checks if the value of the field is `HW_CTRL`"]
    #[inline(always)]
    pub fn is_hw_ctrl(&self) -> bool {
        *self == AP_FS_HOST_CLK_A::HW_CTRL
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == AP_FS_HOST_CLK_A::FORCED
    }
}
#[doc = "Write proxy for field `AP_FS_HOST_CLK`"]
pub struct AP_FS_HOST_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> AP_FS_HOST_CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AP_FS_HOST_CLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Under hardware control."]
    #[inline(always)]
    pub fn hw_ctrl(self) -> &'a mut W {
        self.variant(AP_FS_HOST_CLK_A::HW_CTRL)
    }
    #[doc = "Forced high."]
    #[inline(always)]
    pub fn forced(self) -> &'a mut W {
        self.variant(AP_FS_HOST_CLK_A::FORCED)
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
#[doc = "Possible values of the field `POL_FS_HOST_CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_FS_HOST_CLK_A {
    #[doc = "Falling edge of device USB0_NEEDCLK triggers wake-up."]
    FALLING,
    #[doc = "Rising edge of device USB0_NEEDCLK triggers wake-up."]
    RISING,
}
impl From<POL_FS_HOST_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: POL_FS_HOST_CLK_A) -> Self {
        match variant {
            POL_FS_HOST_CLK_A::FALLING => false,
            POL_FS_HOST_CLK_A::RISING => true,
        }
    }
}
#[doc = "Reader of field `POL_FS_HOST_CLK`"]
pub type POL_FS_HOST_CLK_R = crate::R<bool, POL_FS_HOST_CLK_A>;
impl POL_FS_HOST_CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_FS_HOST_CLK_A {
        match self.bits {
            false => POL_FS_HOST_CLK_A::FALLING,
            true => POL_FS_HOST_CLK_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == POL_FS_HOST_CLK_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == POL_FS_HOST_CLK_A::RISING
    }
}
#[doc = "Write proxy for field `POL_FS_HOST_CLK`"]
pub struct POL_FS_HOST_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_FS_HOST_CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL_FS_HOST_CLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Falling edge of device USB0_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(POL_FS_HOST_CLK_A::FALLING)
    }
    #[doc = "Rising edge of device USB0_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(POL_FS_HOST_CLK_A::RISING)
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
#[doc = "Possible values of the field `PU_DISABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PU_DISABLE_A {
    #[doc = "Internal pull-up enable."]
    ENABLE,
    #[doc = "Internal pull-up disable."]
    DISABLE,
}
impl From<PU_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PU_DISABLE_A) -> Self {
        match variant {
            PU_DISABLE_A::ENABLE => false,
            PU_DISABLE_A::DISABLE => true,
        }
    }
}
#[doc = "Reader of field `PU_DISABLE`"]
pub type PU_DISABLE_R = crate::R<bool, PU_DISABLE_A>;
impl PU_DISABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PU_DISABLE_A {
        match self.bits {
            false => PU_DISABLE_A::ENABLE,
            true => PU_DISABLE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PU_DISABLE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PU_DISABLE_A::DISABLE
    }
}
#[doc = "Write proxy for field `PU_DISABLE`"]
pub struct PU_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PU_DISABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Internal pull-up enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PU_DISABLE_A::ENABLE)
    }
    #[doc = "Internal pull-up disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PU_DISABLE_A::DISABLE)
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
impl R {
    #[doc = "Bit 0 - USB0 Device USB0_NEEDCLK signal control:."]
    #[inline(always)]
    pub fn ap_fs_dev_clk(&self) -> AP_FS_DEV_CLK_R {
        AP_FS_DEV_CLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
    #[inline(always)]
    pub fn pol_fs_dev_clk(&self) -> POL_FS_DEV_CLK_R {
        POL_FS_DEV_CLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB0 Host USB0_NEEDCLK signal control:."]
    #[inline(always)]
    pub fn ap_fs_host_clk(&self) -> AP_FS_HOST_CLK_R {
        AP_FS_HOST_CLK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
    #[inline(always)]
    pub fn pol_fs_host_clk(&self) -> POL_FS_HOST_CLK_R {
        POL_FS_HOST_CLK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Internal pull-up disable control."]
    #[inline(always)]
    pub fn pu_disable(&self) -> PU_DISABLE_R {
        PU_DISABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB0 Device USB0_NEEDCLK signal control:."]
    #[inline(always)]
    pub fn ap_fs_dev_clk(&mut self) -> AP_FS_DEV_CLK_W {
        AP_FS_DEV_CLK_W { w: self }
    }
    #[doc = "Bit 1 - USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
    #[inline(always)]
    pub fn pol_fs_dev_clk(&mut self) -> POL_FS_DEV_CLK_W {
        POL_FS_DEV_CLK_W { w: self }
    }
    #[doc = "Bit 2 - USB0 Host USB0_NEEDCLK signal control:."]
    #[inline(always)]
    pub fn ap_fs_host_clk(&mut self) -> AP_FS_HOST_CLK_W {
        AP_FS_HOST_CLK_W { w: self }
    }
    #[doc = "Bit 3 - USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
    #[inline(always)]
    pub fn pol_fs_host_clk(&mut self) -> POL_FS_HOST_CLK_W {
        POL_FS_HOST_CLK_W { w: self }
    }
    #[doc = "Bit 4 - Internal pull-up disable control."]
    #[inline(always)]
    pub fn pu_disable(&mut self) -> PU_DISABLE_W {
        PU_DISABLE_W { w: self }
    }
}
