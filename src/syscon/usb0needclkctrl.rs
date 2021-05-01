#[doc = "Register `USB0NEEDCLKCTRL` reader"]
pub struct R(crate::R<USB0NEEDCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB0NEEDCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<USB0NEEDCLKCTRL_SPEC>> for R {
    fn from(reader: crate::R<USB0NEEDCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB0NEEDCLKCTRL` writer"]
pub struct W(crate::W<USB0NEEDCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB0NEEDCLKCTRL_SPEC>;
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
impl core::convert::From<crate::W<USB0NEEDCLKCTRL_SPEC>> for W {
    fn from(writer: crate::W<USB0NEEDCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB0 Device USB0_NEEDCLK signal control:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AP_FS_DEV_NEEDCLK_A {
    #[doc = "0: Under hardware control."]
    HW_CTRL = 0,
    #[doc = "1: Forced high."]
    FORCED = 1,
}
impl From<AP_FS_DEV_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: AP_FS_DEV_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AP_FS_DEV_NEEDCLK` reader - USB0 Device USB0_NEEDCLK signal control:."]
pub struct AP_FS_DEV_NEEDCLK_R(crate::FieldReader<bool, AP_FS_DEV_NEEDCLK_A>);
impl AP_FS_DEV_NEEDCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        AP_FS_DEV_NEEDCLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AP_FS_DEV_NEEDCLK_A {
        match self.bits {
            false => AP_FS_DEV_NEEDCLK_A::HW_CTRL,
            true => AP_FS_DEV_NEEDCLK_A::FORCED,
        }
    }
    #[doc = "Checks if the value of the field is `HW_CTRL`"]
    #[inline(always)]
    pub fn is_hw_ctrl(&self) -> bool {
        **self == AP_FS_DEV_NEEDCLK_A::HW_CTRL
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        **self == AP_FS_DEV_NEEDCLK_A::FORCED
    }
}
impl core::ops::Deref for AP_FS_DEV_NEEDCLK_R {
    type Target = crate::FieldReader<bool, AP_FS_DEV_NEEDCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AP_FS_DEV_NEEDCLK` writer - USB0 Device USB0_NEEDCLK signal control:."]
pub struct AP_FS_DEV_NEEDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> AP_FS_DEV_NEEDCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AP_FS_DEV_NEEDCLK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Under hardware control."]
    #[inline(always)]
    pub fn hw_ctrl(self) -> &'a mut W {
        self.variant(AP_FS_DEV_NEEDCLK_A::HW_CTRL)
    }
    #[doc = "Forced high."]
    #[inline(always)]
    pub fn forced(self) -> &'a mut W {
        self.variant(AP_FS_DEV_NEEDCLK_A::FORCED)
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
#[doc = "USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_FS_DEV_NEEDCLK_A {
    #[doc = "0: Falling edge of device USB0_NEEDCLK triggers wake-up."]
    FALLING = 0,
    #[doc = "1: Rising edge of device USB0_NEEDCLK triggers wake-up."]
    RISING = 1,
}
impl From<POL_FS_DEV_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: POL_FS_DEV_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL_FS_DEV_NEEDCLK` reader - USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
pub struct POL_FS_DEV_NEEDCLK_R(crate::FieldReader<bool, POL_FS_DEV_NEEDCLK_A>);
impl POL_FS_DEV_NEEDCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        POL_FS_DEV_NEEDCLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_FS_DEV_NEEDCLK_A {
        match self.bits {
            false => POL_FS_DEV_NEEDCLK_A::FALLING,
            true => POL_FS_DEV_NEEDCLK_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        **self == POL_FS_DEV_NEEDCLK_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        **self == POL_FS_DEV_NEEDCLK_A::RISING
    }
}
impl core::ops::Deref for POL_FS_DEV_NEEDCLK_R {
    type Target = crate::FieldReader<bool, POL_FS_DEV_NEEDCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_FS_DEV_NEEDCLK` writer - USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
pub struct POL_FS_DEV_NEEDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_FS_DEV_NEEDCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL_FS_DEV_NEEDCLK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge of device USB0_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(POL_FS_DEV_NEEDCLK_A::FALLING)
    }
    #[doc = "Rising edge of device USB0_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(POL_FS_DEV_NEEDCLK_A::RISING)
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
#[doc = "USB0 Host USB0_NEEDCLK signal control:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AP_FS_HOST_NEEDCLK_A {
    #[doc = "0: Under hardware control."]
    HW_CTRL = 0,
    #[doc = "1: Forced high."]
    FORCED = 1,
}
impl From<AP_FS_HOST_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: AP_FS_HOST_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AP_FS_HOST_NEEDCLK` reader - USB0 Host USB0_NEEDCLK signal control:."]
pub struct AP_FS_HOST_NEEDCLK_R(crate::FieldReader<bool, AP_FS_HOST_NEEDCLK_A>);
impl AP_FS_HOST_NEEDCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        AP_FS_HOST_NEEDCLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AP_FS_HOST_NEEDCLK_A {
        match self.bits {
            false => AP_FS_HOST_NEEDCLK_A::HW_CTRL,
            true => AP_FS_HOST_NEEDCLK_A::FORCED,
        }
    }
    #[doc = "Checks if the value of the field is `HW_CTRL`"]
    #[inline(always)]
    pub fn is_hw_ctrl(&self) -> bool {
        **self == AP_FS_HOST_NEEDCLK_A::HW_CTRL
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        **self == AP_FS_HOST_NEEDCLK_A::FORCED
    }
}
impl core::ops::Deref for AP_FS_HOST_NEEDCLK_R {
    type Target = crate::FieldReader<bool, AP_FS_HOST_NEEDCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AP_FS_HOST_NEEDCLK` writer - USB0 Host USB0_NEEDCLK signal control:."]
pub struct AP_FS_HOST_NEEDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> AP_FS_HOST_NEEDCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AP_FS_HOST_NEEDCLK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Under hardware control."]
    #[inline(always)]
    pub fn hw_ctrl(self) -> &'a mut W {
        self.variant(AP_FS_HOST_NEEDCLK_A::HW_CTRL)
    }
    #[doc = "Forced high."]
    #[inline(always)]
    pub fn forced(self) -> &'a mut W {
        self.variant(AP_FS_HOST_NEEDCLK_A::FORCED)
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
#[doc = "USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_FS_HOST_NEEDCLK_A {
    #[doc = "0: Falling edge of device USB0_NEEDCLK triggers wake-up."]
    FALLING = 0,
    #[doc = "1: Rising edge of device USB0_NEEDCLK triggers wake-up."]
    RISING = 1,
}
impl From<POL_FS_HOST_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: POL_FS_HOST_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL_FS_HOST_NEEDCLK` reader - USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
pub struct POL_FS_HOST_NEEDCLK_R(crate::FieldReader<bool, POL_FS_HOST_NEEDCLK_A>);
impl POL_FS_HOST_NEEDCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        POL_FS_HOST_NEEDCLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_FS_HOST_NEEDCLK_A {
        match self.bits {
            false => POL_FS_HOST_NEEDCLK_A::FALLING,
            true => POL_FS_HOST_NEEDCLK_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        **self == POL_FS_HOST_NEEDCLK_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        **self == POL_FS_HOST_NEEDCLK_A::RISING
    }
}
impl core::ops::Deref for POL_FS_HOST_NEEDCLK_R {
    type Target = crate::FieldReader<bool, POL_FS_HOST_NEEDCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_FS_HOST_NEEDCLK` writer - USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
pub struct POL_FS_HOST_NEEDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_FS_HOST_NEEDCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL_FS_HOST_NEEDCLK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge of device USB0_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(POL_FS_HOST_NEEDCLK_A::FALLING)
    }
    #[doc = "Rising edge of device USB0_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(POL_FS_HOST_NEEDCLK_A::RISING)
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
impl R {
    #[doc = "Bit 0 - USB0 Device USB0_NEEDCLK signal control:."]
    #[inline(always)]
    pub fn ap_fs_dev_needclk(&self) -> AP_FS_DEV_NEEDCLK_R {
        AP_FS_DEV_NEEDCLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
    #[inline(always)]
    pub fn pol_fs_dev_needclk(&self) -> POL_FS_DEV_NEEDCLK_R {
        POL_FS_DEV_NEEDCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB0 Host USB0_NEEDCLK signal control:."]
    #[inline(always)]
    pub fn ap_fs_host_needclk(&self) -> AP_FS_HOST_NEEDCLK_R {
        AP_FS_HOST_NEEDCLK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
    #[inline(always)]
    pub fn pol_fs_host_needclk(&self) -> POL_FS_HOST_NEEDCLK_R {
        POL_FS_HOST_NEEDCLK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB0 Device USB0_NEEDCLK signal control:."]
    #[inline(always)]
    pub fn ap_fs_dev_needclk(&mut self) -> AP_FS_DEV_NEEDCLK_W {
        AP_FS_DEV_NEEDCLK_W { w: self }
    }
    #[doc = "Bit 1 - USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
    #[inline(always)]
    pub fn pol_fs_dev_needclk(&mut self) -> POL_FS_DEV_NEEDCLK_W {
        POL_FS_DEV_NEEDCLK_W { w: self }
    }
    #[doc = "Bit 2 - USB0 Host USB0_NEEDCLK signal control:."]
    #[inline(always)]
    pub fn ap_fs_host_needclk(&mut self) -> AP_FS_HOST_NEEDCLK_W {
        AP_FS_HOST_NEEDCLK_W { w: self }
    }
    #[doc = "Bit 3 - USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
    #[inline(always)]
    pub fn pol_fs_host_needclk(&mut self) -> POL_FS_HOST_NEEDCLK_W {
        POL_FS_HOST_NEEDCLK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB0 need clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb0needclkctrl](index.html) module"]
pub struct USB0NEEDCLKCTRL_SPEC;
impl crate::RegisterSpec for USB0NEEDCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb0needclkctrl::R](R) reader structure"]
impl crate::Readable for USB0NEEDCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb0needclkctrl::W](W) writer structure"]
impl crate::Writable for USB0NEEDCLKCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB0NEEDCLKCTRL to value 0"]
impl crate::Resettable for USB0NEEDCLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
