#[doc = "Reader of register USB1NEEDCLKCTRL"]
pub type R = crate::R<u32, super::USB1NEEDCLKCTRL>;
#[doc = "Writer for register USB1NEEDCLKCTRL"]
pub type W = crate::W<u32, super::USB1NEEDCLKCTRL>;
#[doc = "Register USB1NEEDCLKCTRL `reset()`'s with value 0x10"]
impl crate::ResetValue for super::USB1NEEDCLKCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "Possible values of the field `AP_HS_DEV_NEEDCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AP_HS_DEV_NEEDCLK_A {
    #[doc = "HOST_NEEDCLK is under hardware control."]
    HW_CTRL,
    #[doc = "HOST_NEEDCLK is forced high."]
    FORCED,
}
impl From<AP_HS_DEV_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: AP_HS_DEV_NEEDCLK_A) -> Self {
        match variant {
            AP_HS_DEV_NEEDCLK_A::HW_CTRL => false,
            AP_HS_DEV_NEEDCLK_A::FORCED => true,
        }
    }
}
#[doc = "Reader of field `AP_HS_DEV_NEEDCLK`"]
pub type AP_HS_DEV_NEEDCLK_R = crate::R<bool, AP_HS_DEV_NEEDCLK_A>;
impl AP_HS_DEV_NEEDCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AP_HS_DEV_NEEDCLK_A {
        match self.bits {
            false => AP_HS_DEV_NEEDCLK_A::HW_CTRL,
            true => AP_HS_DEV_NEEDCLK_A::FORCED,
        }
    }
    #[doc = "Checks if the value of the field is `HW_CTRL`"]
    #[inline(always)]
    pub fn is_hw_ctrl(&self) -> bool {
        *self == AP_HS_DEV_NEEDCLK_A::HW_CTRL
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == AP_HS_DEV_NEEDCLK_A::FORCED
    }
}
#[doc = "Write proxy for field `AP_HS_DEV_NEEDCLK`"]
pub struct AP_HS_DEV_NEEDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> AP_HS_DEV_NEEDCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AP_HS_DEV_NEEDCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HOST_NEEDCLK is under hardware control."]
    #[inline(always)]
    pub fn hw_ctrl(self) -> &'a mut W {
        self.variant(AP_HS_DEV_NEEDCLK_A::HW_CTRL)
    }
    #[doc = "HOST_NEEDCLK is forced high."]
    #[inline(always)]
    pub fn forced(self) -> &'a mut W {
        self.variant(AP_HS_DEV_NEEDCLK_A::FORCED)
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
#[doc = "Possible values of the field `POL_HS_DEV_NEEDCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_HS_DEV_NEEDCLK_A {
    #[doc = "Falling edge of DEV_NEEDCLK triggers wake-up."]
    FALLING,
    #[doc = "Rising edge of DEV_NEEDCLK triggers wake-up."]
    RISING,
}
impl From<POL_HS_DEV_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: POL_HS_DEV_NEEDCLK_A) -> Self {
        match variant {
            POL_HS_DEV_NEEDCLK_A::FALLING => false,
            POL_HS_DEV_NEEDCLK_A::RISING => true,
        }
    }
}
#[doc = "Reader of field `POL_HS_DEV_NEEDCLK`"]
pub type POL_HS_DEV_NEEDCLK_R = crate::R<bool, POL_HS_DEV_NEEDCLK_A>;
impl POL_HS_DEV_NEEDCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_HS_DEV_NEEDCLK_A {
        match self.bits {
            false => POL_HS_DEV_NEEDCLK_A::FALLING,
            true => POL_HS_DEV_NEEDCLK_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == POL_HS_DEV_NEEDCLK_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == POL_HS_DEV_NEEDCLK_A::RISING
    }
}
#[doc = "Write proxy for field `POL_HS_DEV_NEEDCLK`"]
pub struct POL_HS_DEV_NEEDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_HS_DEV_NEEDCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL_HS_DEV_NEEDCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Falling edge of DEV_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(POL_HS_DEV_NEEDCLK_A::FALLING)
    }
    #[doc = "Rising edge of DEV_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(POL_HS_DEV_NEEDCLK_A::RISING)
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
#[doc = "Possible values of the field `AP_HS_HOST_NEEDCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AP_HS_HOST_NEEDCLK_A {
    #[doc = "HOST_NEEDCLK is under hardware control."]
    HW_CTRL,
    #[doc = "HOST_NEEDCLK is forced high."]
    FORCED,
}
impl From<AP_HS_HOST_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: AP_HS_HOST_NEEDCLK_A) -> Self {
        match variant {
            AP_HS_HOST_NEEDCLK_A::HW_CTRL => false,
            AP_HS_HOST_NEEDCLK_A::FORCED => true,
        }
    }
}
#[doc = "Reader of field `AP_HS_HOST_NEEDCLK`"]
pub type AP_HS_HOST_NEEDCLK_R = crate::R<bool, AP_HS_HOST_NEEDCLK_A>;
impl AP_HS_HOST_NEEDCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AP_HS_HOST_NEEDCLK_A {
        match self.bits {
            false => AP_HS_HOST_NEEDCLK_A::HW_CTRL,
            true => AP_HS_HOST_NEEDCLK_A::FORCED,
        }
    }
    #[doc = "Checks if the value of the field is `HW_CTRL`"]
    #[inline(always)]
    pub fn is_hw_ctrl(&self) -> bool {
        *self == AP_HS_HOST_NEEDCLK_A::HW_CTRL
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == AP_HS_HOST_NEEDCLK_A::FORCED
    }
}
#[doc = "Write proxy for field `AP_HS_HOST_NEEDCLK`"]
pub struct AP_HS_HOST_NEEDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> AP_HS_HOST_NEEDCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AP_HS_HOST_NEEDCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HOST_NEEDCLK is under hardware control."]
    #[inline(always)]
    pub fn hw_ctrl(self) -> &'a mut W {
        self.variant(AP_HS_HOST_NEEDCLK_A::HW_CTRL)
    }
    #[doc = "HOST_NEEDCLK is forced high."]
    #[inline(always)]
    pub fn forced(self) -> &'a mut W {
        self.variant(AP_HS_HOST_NEEDCLK_A::FORCED)
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
#[doc = "Possible values of the field `POL_HS_HOST_NEEDCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_HS_HOST_NEEDCLK_A {
    #[doc = "Falling edge of HOST_NEEDCLK triggers wake-up."]
    FALLING,
    #[doc = "Rising edge of HOST_NEEDCLK triggers wake-up."]
    RISING,
}
impl From<POL_HS_HOST_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: POL_HS_HOST_NEEDCLK_A) -> Self {
        match variant {
            POL_HS_HOST_NEEDCLK_A::FALLING => false,
            POL_HS_HOST_NEEDCLK_A::RISING => true,
        }
    }
}
#[doc = "Reader of field `POL_HS_HOST_NEEDCLK`"]
pub type POL_HS_HOST_NEEDCLK_R = crate::R<bool, POL_HS_HOST_NEEDCLK_A>;
impl POL_HS_HOST_NEEDCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_HS_HOST_NEEDCLK_A {
        match self.bits {
            false => POL_HS_HOST_NEEDCLK_A::FALLING,
            true => POL_HS_HOST_NEEDCLK_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == POL_HS_HOST_NEEDCLK_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == POL_HS_HOST_NEEDCLK_A::RISING
    }
}
#[doc = "Write proxy for field `POL_HS_HOST_NEEDCLK`"]
pub struct POL_HS_HOST_NEEDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_HS_HOST_NEEDCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL_HS_HOST_NEEDCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Falling edge of HOST_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(POL_HS_HOST_NEEDCLK_A::FALLING)
    }
    #[doc = "Rising edge of HOST_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(POL_HS_HOST_NEEDCLK_A::RISING)
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
#[doc = "Possible values of the field `HS_DEV_WAKEUP_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_DEV_WAKEUP_N_A {
    #[doc = "Forces USB1_PHY to wake-up."]
    FORCE_WUP,
    #[doc = "Normal USB1_PHY behavior."]
    NORMAL_WUP,
}
impl From<HS_DEV_WAKEUP_N_A> for bool {
    #[inline(always)]
    fn from(variant: HS_DEV_WAKEUP_N_A) -> Self {
        match variant {
            HS_DEV_WAKEUP_N_A::FORCE_WUP => false,
            HS_DEV_WAKEUP_N_A::NORMAL_WUP => true,
        }
    }
}
#[doc = "Reader of field `HS_DEV_WAKEUP_N`"]
pub type HS_DEV_WAKEUP_N_R = crate::R<bool, HS_DEV_WAKEUP_N_A>;
impl HS_DEV_WAKEUP_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_DEV_WAKEUP_N_A {
        match self.bits {
            false => HS_DEV_WAKEUP_N_A::FORCE_WUP,
            true => HS_DEV_WAKEUP_N_A::NORMAL_WUP,
        }
    }
    #[doc = "Checks if the value of the field is `FORCE_WUP`"]
    #[inline(always)]
    pub fn is_force_wup(&self) -> bool {
        *self == HS_DEV_WAKEUP_N_A::FORCE_WUP
    }
    #[doc = "Checks if the value of the field is `NORMAL_WUP`"]
    #[inline(always)]
    pub fn is_normal_wup(&self) -> bool {
        *self == HS_DEV_WAKEUP_N_A::NORMAL_WUP
    }
}
#[doc = "Write proxy for field `HS_DEV_WAKEUP_N`"]
pub struct HS_DEV_WAKEUP_N_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_DEV_WAKEUP_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS_DEV_WAKEUP_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Forces USB1_PHY to wake-up."]
    #[inline(always)]
    pub fn force_wup(self) -> &'a mut W {
        self.variant(HS_DEV_WAKEUP_N_A::FORCE_WUP)
    }
    #[doc = "Normal USB1_PHY behavior."]
    #[inline(always)]
    pub fn normal_wup(self) -> &'a mut W {
        self.variant(HS_DEV_WAKEUP_N_A::NORMAL_WUP)
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
    #[doc = "Bit 0 - USB1 Device need_clock signal control:"]
    #[inline(always)]
    pub fn ap_hs_dev_needclk(&self) -> AP_HS_DEV_NEEDCLK_R {
        AP_HS_DEV_NEEDCLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB1 device need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt:"]
    #[inline(always)]
    pub fn pol_hs_dev_needclk(&self) -> POL_HS_DEV_NEEDCLK_R {
        POL_HS_DEV_NEEDCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB1 Host need clock signal control:"]
    #[inline(always)]
    pub fn ap_hs_host_needclk(&self) -> AP_HS_HOST_NEEDCLK_R {
        AP_HS_HOST_NEEDCLK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB1 host need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt."]
    #[inline(always)]
    pub fn pol_hs_host_needclk(&self) -> POL_HS_HOST_NEEDCLK_R {
        POL_HS_HOST_NEEDCLK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Software override of device controller PHY wake up logic."]
    #[inline(always)]
    pub fn hs_dev_wakeup_n(&self) -> HS_DEV_WAKEUP_N_R {
        HS_DEV_WAKEUP_N_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB1 Device need_clock signal control:"]
    #[inline(always)]
    pub fn ap_hs_dev_needclk(&mut self) -> AP_HS_DEV_NEEDCLK_W {
        AP_HS_DEV_NEEDCLK_W { w: self }
    }
    #[doc = "Bit 1 - USB1 device need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt:"]
    #[inline(always)]
    pub fn pol_hs_dev_needclk(&mut self) -> POL_HS_DEV_NEEDCLK_W {
        POL_HS_DEV_NEEDCLK_W { w: self }
    }
    #[doc = "Bit 2 - USB1 Host need clock signal control:"]
    #[inline(always)]
    pub fn ap_hs_host_needclk(&mut self) -> AP_HS_HOST_NEEDCLK_W {
        AP_HS_HOST_NEEDCLK_W { w: self }
    }
    #[doc = "Bit 3 - USB1 host need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt."]
    #[inline(always)]
    pub fn pol_hs_host_needclk(&mut self) -> POL_HS_HOST_NEEDCLK_W {
        POL_HS_HOST_NEEDCLK_W { w: self }
    }
    #[doc = "Bit 4 - Software override of device controller PHY wake up logic."]
    #[inline(always)]
    pub fn hs_dev_wakeup_n(&mut self) -> HS_DEV_WAKEUP_N_W {
        HS_DEV_WAKEUP_N_W { w: self }
    }
}
