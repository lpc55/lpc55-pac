#[doc = "Register `USB1NEEDCLKCTRL` reader"]
pub struct R(crate::R<USB1NEEDCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1NEEDCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<USB1NEEDCLKCTRL_SPEC>> for R {
    fn from(reader: crate::R<USB1NEEDCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB1NEEDCLKCTRL` writer"]
pub struct W(crate::W<USB1NEEDCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB1NEEDCLKCTRL_SPEC>;
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
impl core::convert::From<crate::W<USB1NEEDCLKCTRL_SPEC>> for W {
    fn from(writer: crate::W<USB1NEEDCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB1 Device need_clock signal control:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AP_HS_DEV_NEEDCLK_A {
    #[doc = "0: HOST_NEEDCLK is under hardware control."]
    HW_CTRL = 0,
    #[doc = "1: HOST_NEEDCLK is forced high."]
    FORCED = 1,
}
impl From<AP_HS_DEV_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: AP_HS_DEV_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AP_HS_DEV_NEEDCLK` reader - USB1 Device need_clock signal control:"]
pub struct AP_HS_DEV_NEEDCLK_R(crate::FieldReader<bool, AP_HS_DEV_NEEDCLK_A>);
impl AP_HS_DEV_NEEDCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        AP_HS_DEV_NEEDCLK_R(crate::FieldReader::new(bits))
    }
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
        **self == AP_HS_DEV_NEEDCLK_A::HW_CTRL
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        **self == AP_HS_DEV_NEEDCLK_A::FORCED
    }
}
impl core::ops::Deref for AP_HS_DEV_NEEDCLK_R {
    type Target = crate::FieldReader<bool, AP_HS_DEV_NEEDCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AP_HS_DEV_NEEDCLK` writer - USB1 Device need_clock signal control:"]
pub struct AP_HS_DEV_NEEDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> AP_HS_DEV_NEEDCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AP_HS_DEV_NEEDCLK_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "USB1 device need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_HS_DEV_NEEDCLK_A {
    #[doc = "0: Falling edge of DEV_NEEDCLK triggers wake-up."]
    FALLING = 0,
    #[doc = "1: Rising edge of DEV_NEEDCLK triggers wake-up."]
    RISING = 1,
}
impl From<POL_HS_DEV_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: POL_HS_DEV_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL_HS_DEV_NEEDCLK` reader - USB1 device need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt:"]
pub struct POL_HS_DEV_NEEDCLK_R(crate::FieldReader<bool, POL_HS_DEV_NEEDCLK_A>);
impl POL_HS_DEV_NEEDCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        POL_HS_DEV_NEEDCLK_R(crate::FieldReader::new(bits))
    }
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
        **self == POL_HS_DEV_NEEDCLK_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        **self == POL_HS_DEV_NEEDCLK_A::RISING
    }
}
impl core::ops::Deref for POL_HS_DEV_NEEDCLK_R {
    type Target = crate::FieldReader<bool, POL_HS_DEV_NEEDCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_HS_DEV_NEEDCLK` writer - USB1 device need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt:"]
pub struct POL_HS_DEV_NEEDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_HS_DEV_NEEDCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL_HS_DEV_NEEDCLK_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "USB1 Host need clock signal control:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AP_HS_HOST_NEEDCLK_A {
    #[doc = "0: HOST_NEEDCLK is under hardware control."]
    HW_CTRL = 0,
    #[doc = "1: HOST_NEEDCLK is forced high."]
    FORCED = 1,
}
impl From<AP_HS_HOST_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: AP_HS_HOST_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AP_HS_HOST_NEEDCLK` reader - USB1 Host need clock signal control:"]
pub struct AP_HS_HOST_NEEDCLK_R(crate::FieldReader<bool, AP_HS_HOST_NEEDCLK_A>);
impl AP_HS_HOST_NEEDCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        AP_HS_HOST_NEEDCLK_R(crate::FieldReader::new(bits))
    }
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
        **self == AP_HS_HOST_NEEDCLK_A::HW_CTRL
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        **self == AP_HS_HOST_NEEDCLK_A::FORCED
    }
}
impl core::ops::Deref for AP_HS_HOST_NEEDCLK_R {
    type Target = crate::FieldReader<bool, AP_HS_HOST_NEEDCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AP_HS_HOST_NEEDCLK` writer - USB1 Host need clock signal control:"]
pub struct AP_HS_HOST_NEEDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> AP_HS_HOST_NEEDCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AP_HS_HOST_NEEDCLK_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "USB1 host need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_HS_HOST_NEEDCLK_A {
    #[doc = "0: Falling edge of HOST_NEEDCLK triggers wake-up."]
    FALLING = 0,
    #[doc = "1: Rising edge of HOST_NEEDCLK triggers wake-up."]
    RISING = 1,
}
impl From<POL_HS_HOST_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: POL_HS_HOST_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL_HS_HOST_NEEDCLK` reader - USB1 host need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt."]
pub struct POL_HS_HOST_NEEDCLK_R(crate::FieldReader<bool, POL_HS_HOST_NEEDCLK_A>);
impl POL_HS_HOST_NEEDCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        POL_HS_HOST_NEEDCLK_R(crate::FieldReader::new(bits))
    }
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
        **self == POL_HS_HOST_NEEDCLK_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        **self == POL_HS_HOST_NEEDCLK_A::RISING
    }
}
impl core::ops::Deref for POL_HS_HOST_NEEDCLK_R {
    type Target = crate::FieldReader<bool, POL_HS_HOST_NEEDCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_HS_HOST_NEEDCLK` writer - USB1 host need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt."]
pub struct POL_HS_HOST_NEEDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_HS_HOST_NEEDCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL_HS_HOST_NEEDCLK_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Software override of device controller PHY wake up logic.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_DEV_WAKEUP_N_A {
    #[doc = "0: Forces USB1_PHY to wake-up."]
    FORCE_WUP = 0,
    #[doc = "1: Normal USB1_PHY behavior."]
    NORMAL_WUP = 1,
}
impl From<HS_DEV_WAKEUP_N_A> for bool {
    #[inline(always)]
    fn from(variant: HS_DEV_WAKEUP_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS_DEV_WAKEUP_N` reader - Software override of device controller PHY wake up logic."]
pub struct HS_DEV_WAKEUP_N_R(crate::FieldReader<bool, HS_DEV_WAKEUP_N_A>);
impl HS_DEV_WAKEUP_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        HS_DEV_WAKEUP_N_R(crate::FieldReader::new(bits))
    }
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
        **self == HS_DEV_WAKEUP_N_A::FORCE_WUP
    }
    #[doc = "Checks if the value of the field is `NORMAL_WUP`"]
    #[inline(always)]
    pub fn is_normal_wup(&self) -> bool {
        **self == HS_DEV_WAKEUP_N_A::NORMAL_WUP
    }
}
impl core::ops::Deref for HS_DEV_WAKEUP_N_R {
    type Target = crate::FieldReader<bool, HS_DEV_WAKEUP_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HS_DEV_WAKEUP_N` writer - Software override of device controller PHY wake up logic."]
pub struct HS_DEV_WAKEUP_N_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_DEV_WAKEUP_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS_DEV_WAKEUP_N_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
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
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB1 need clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1needclkctrl](index.html) module"]
pub struct USB1NEEDCLKCTRL_SPEC;
impl crate::RegisterSpec for USB1NEEDCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1needclkctrl::R](R) reader structure"]
impl crate::Readable for USB1NEEDCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb1needclkctrl::W](W) writer structure"]
impl crate::Writable for USB1NEEDCLKCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB1NEEDCLKCTRL to value 0x10"]
impl crate::Resettable for USB1NEEDCLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
