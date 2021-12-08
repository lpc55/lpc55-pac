#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENHOSTDISCONDETECT` reader - For host mode, enables high-speed disconnect detector"]
pub struct ENHOSTDISCONDETECT_R(crate::FieldReader<bool, bool>);
impl ENHOSTDISCONDETECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENHOSTDISCONDETECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENHOSTDISCONDETECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENHOSTDISCONDETECT` writer - For host mode, enables high-speed disconnect detector"]
pub struct ENHOSTDISCONDETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENHOSTDISCONDETECT_W<'a> {
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
#[doc = "Field `ENIRQHOSTDISCON` reader - Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
pub struct ENIRQHOSTDISCON_R(crate::FieldReader<bool, bool>);
impl ENIRQHOSTDISCON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENIRQHOSTDISCON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENIRQHOSTDISCON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENIRQHOSTDISCON` writer - Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
pub struct ENIRQHOSTDISCON_W<'a> {
    w: &'a mut W,
}
impl<'a> ENIRQHOSTDISCON_W<'a> {
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
#[doc = "Field `HOSTDISCONDETECT_IRQ` reader - Indicates that the device has disconnected in High-Speed mode"]
pub struct HOSTDISCONDETECT_IRQ_R(crate::FieldReader<bool, bool>);
impl HOSTDISCONDETECT_IRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOSTDISCONDETECT_IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOSTDISCONDETECT_IRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOSTDISCONDETECT_IRQ` writer - Indicates that the device has disconnected in High-Speed mode"]
pub struct HOSTDISCONDETECT_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HOSTDISCONDETECT_IRQ_W<'a> {
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
#[doc = "Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEVPLUGINDET_A {
    #[doc = "0: Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)"]
    VALUE0 = 0,
    #[doc = "1: Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    VALUE1 = 1,
}
impl From<ENDEVPLUGINDET_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEVPLUGINDET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEVPLUGINDET` reader - Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
pub struct ENDEVPLUGINDET_R(crate::FieldReader<bool, ENDEVPLUGINDET_A>);
impl ENDEVPLUGINDET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDEVPLUGINDET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEVPLUGINDET_A {
        match self.bits {
            false => ENDEVPLUGINDET_A::VALUE0,
            true => ENDEVPLUGINDET_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        **self == ENDEVPLUGINDET_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ENDEVPLUGINDET_A::VALUE1
    }
}
impl core::ops::Deref for ENDEVPLUGINDET_R {
    type Target = crate::FieldReader<bool, ENDEVPLUGINDET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDEVPLUGINDET` writer - Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
pub struct ENDEVPLUGINDET_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEVPLUGINDET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEVPLUGINDET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(ENDEVPLUGINDET_A::VALUE0)
    }
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENDEVPLUGINDET_A::VALUE1)
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
#[doc = "Field `DEVPLUGIN_POLARITY` reader - Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
pub struct DEVPLUGIN_POLARITY_R(crate::FieldReader<bool, bool>);
impl DEVPLUGIN_POLARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEVPLUGIN_POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVPLUGIN_POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEVPLUGIN_POLARITY` writer - Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
pub struct DEVPLUGIN_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVPLUGIN_POLARITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RESUMEIRQSTICKY` reader - Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
pub struct RESUMEIRQSTICKY_R(crate::FieldReader<bool, bool>);
impl RESUMEIRQSTICKY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESUMEIRQSTICKY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESUMEIRQSTICKY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUMEIRQSTICKY` writer - Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
pub struct RESUMEIRQSTICKY_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUMEIRQSTICKY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `ENIRQRESUMEDETECT` reader - Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line"]
pub struct ENIRQRESUMEDETECT_R(crate::FieldReader<bool, bool>);
impl ENIRQRESUMEDETECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENIRQRESUMEDETECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENIRQRESUMEDETECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENIRQRESUMEDETECT` writer - Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line"]
pub struct ENIRQRESUMEDETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENIRQRESUMEDETECT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `RESUME_IRQ` reader - Resume IRQ: Indicates that the host is sending a wake-up after suspend"]
pub struct RESUME_IRQ_R(crate::FieldReader<bool, bool>);
impl RESUME_IRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESUME_IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESUME_IRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUME_IRQ` writer - Resume IRQ: Indicates that the host is sending a wake-up after suspend"]
pub struct RESUME_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_IRQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `DEVPLUGIN_IRQ` reader - Indicates that the device is connected"]
pub struct DEVPLUGIN_IRQ_R(crate::FieldReader<bool, bool>);
impl DEVPLUGIN_IRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEVPLUGIN_IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVPLUGIN_IRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEVPLUGIN_IRQ` writer - Indicates that the device is connected"]
pub struct DEVPLUGIN_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVPLUGIN_IRQ_W<'a> {
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
#[doc = "Field `ENUTMILEVEL2` reader - Enables UTMI+ Level 2 operation for the USB HS PHY"]
pub struct ENUTMILEVEL2_R(crate::FieldReader<bool, bool>);
impl ENUTMILEVEL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENUTMILEVEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENUTMILEVEL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENUTMILEVEL2` writer - Enables UTMI+ Level 2 operation for the USB HS PHY"]
pub struct ENUTMILEVEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUTMILEVEL2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `ENUTMILEVEL3` reader - Enables UTMI+ Level 3 operation for the USB HS PHY"]
pub struct ENUTMILEVEL3_R(crate::FieldReader<bool, bool>);
impl ENUTMILEVEL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENUTMILEVEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENUTMILEVEL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENUTMILEVEL3` writer - Enables UTMI+ Level 3 operation for the USB HS PHY"]
pub struct ENUTMILEVEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUTMILEVEL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `ENIRQWAKEUP` reader - Enable wake-up IRQ: Enables interrupt for the wake-up events."]
pub struct ENIRQWAKEUP_R(crate::FieldReader<bool, bool>);
impl ENIRQWAKEUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENIRQWAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENIRQWAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENIRQWAKEUP` writer - Enable wake-up IRQ: Enables interrupt for the wake-up events."]
pub struct ENIRQWAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENIRQWAKEUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `WAKEUP_IRQ` reader - Wake-up IRQ: Indicates that there is a wak-eup event"]
pub struct WAKEUP_IRQ_R(crate::FieldReader<bool, bool>);
impl WAKEUP_IRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUP_IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUP_IRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUP_IRQ` writer - Wake-up IRQ: Indicates that there is a wak-eup event"]
pub struct WAKEUP_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_IRQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `AUTORESUME_EN` reader - Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
pub struct AUTORESUME_EN_R(crate::FieldReader<bool, bool>);
impl AUTORESUME_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTORESUME_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTORESUME_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTORESUME_EN` writer - Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
pub struct AUTORESUME_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTORESUME_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `ENAUTOCLR_CLKGATE` reader - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
pub struct ENAUTOCLR_CLKGATE_R(crate::FieldReader<bool, bool>);
impl ENAUTOCLR_CLKGATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENAUTOCLR_CLKGATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENAUTOCLR_CLKGATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENAUTOCLR_CLKGATE` writer - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
pub struct ENAUTOCLR_CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENAUTOCLR_CLKGATE_W<'a> {
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
#[doc = "Field `ENAUTOCLR_PHY_PWD` reader - Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
pub struct ENAUTOCLR_PHY_PWD_R(crate::FieldReader<bool, bool>);
impl ENAUTOCLR_PHY_PWD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENAUTOCLR_PHY_PWD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENAUTOCLR_PHY_PWD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENAUTOCLR_PHY_PWD` writer - Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
pub struct ENAUTOCLR_PHY_PWD_W<'a> {
    w: &'a mut W,
}
impl<'a> ENAUTOCLR_PHY_PWD_W<'a> {
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
#[doc = "Field `ENDPDMCHG_WKUP` reader - Enable DP DM change wake-up: Not for customer use"]
pub struct ENDPDMCHG_WKUP_R(crate::FieldReader<bool, bool>);
impl ENDPDMCHG_WKUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDPDMCHG_WKUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENDPDMCHG_WKUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDPDMCHG_WKUP` writer - Enable DP DM change wake-up: Not for customer use"]
pub struct ENDPDMCHG_WKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDPDMCHG_WKUP_W<'a> {
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
#[doc = "Field `ENVBUSCHG_WKUP` reader - Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended"]
pub struct ENVBUSCHG_WKUP_R(crate::FieldReader<bool, bool>);
impl ENVBUSCHG_WKUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENVBUSCHG_WKUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENVBUSCHG_WKUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENVBUSCHG_WKUP` writer - Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended"]
pub struct ENVBUSCHG_WKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENVBUSCHG_WKUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `ENAUTOCLR_USBCLKGATE` reader - Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
pub struct ENAUTOCLR_USBCLKGATE_R(crate::FieldReader<bool, bool>);
impl ENAUTOCLR_USBCLKGATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENAUTOCLR_USBCLKGATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENAUTOCLR_USBCLKGATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENAUTOCLR_USBCLKGATE` writer - Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
pub struct ENAUTOCLR_USBCLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENAUTOCLR_USBCLKGATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `ENAUTOSET_USBCLKS` reader - Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
pub struct ENAUTOSET_USBCLKS_R(crate::FieldReader<bool, bool>);
impl ENAUTOSET_USBCLKS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENAUTOSET_USBCLKS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENAUTOSET_USBCLKS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENAUTOSET_USBCLKS` writer - Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
pub struct ENAUTOSET_USBCLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> ENAUTOSET_USBCLKS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `HOST_FORCE_LS_SE0` reader - Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
pub struct HOST_FORCE_LS_SE0_R(crate::FieldReader<bool, bool>);
impl HOST_FORCE_LS_SE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_FORCE_LS_SE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_FORCE_LS_SE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_FORCE_LS_SE0` writer - Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
pub struct HOST_FORCE_LS_SE0_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_FORCE_LS_SE0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `UTMI_SUSPENDM` reader - Used by the PHY to indicate a powered-down state"]
pub struct UTMI_SUSPENDM_R(crate::FieldReader<bool, bool>);
impl UTMI_SUSPENDM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UTMI_SUSPENDM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTMI_SUSPENDM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTMI_SUSPENDM` writer - Used by the PHY to indicate a powered-down state"]
pub struct UTMI_SUSPENDM_W<'a> {
    w: &'a mut W,
}
impl<'a> UTMI_SUSPENDM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `CLKGATE` reader - Gate UTMI Clocks"]
pub struct CLKGATE_R(crate::FieldReader<bool, bool>);
impl CLKGATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKGATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKGATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKGATE` writer - Gate UTMI Clocks"]
pub struct CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKGATE_W<'a> {
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
#[doc = "Field `SFTRST` reader - Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
pub struct SFTRST_R(crate::FieldReader<bool, bool>);
impl SFTRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SFTRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFTRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFTRST` writer - Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
pub struct SFTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SFTRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub fn enhostdiscondetect(&self) -> ENHOSTDISCONDETECT_R {
        ENHOSTDISCONDETECT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub fn enirqhostdiscon(&self) -> ENIRQHOSTDISCON_R {
        ENIRQHOSTDISCON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicates that the device has disconnected in High-Speed mode"]
    #[inline(always)]
    pub fn hostdiscondetect_irq(&self) -> HOSTDISCONDETECT_IRQ_R {
        HOSTDISCONDETECT_IRQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
    #[inline(always)]
    pub fn endevplugindet(&self) -> ENDEVPLUGINDET_R {
        ENDEVPLUGINDET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub fn devplugin_polarity(&self) -> DEVPLUGIN_POLARITY_R {
        DEVPLUGIN_POLARITY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub fn resumeirqsticky(&self) -> RESUMEIRQSTICKY_R {
        RESUMEIRQSTICKY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub fn enirqresumedetect(&self) -> ENIRQRESUMEDETECT_R {
        ENIRQRESUMEDETECT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Resume IRQ: Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub fn resume_irq(&self) -> RESUME_IRQ_R {
        RESUME_IRQ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Indicates that the device is connected"]
    #[inline(always)]
    pub fn devplugin_irq(&self) -> DEVPLUGIN_IRQ_R {
        DEVPLUGIN_IRQ_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enables UTMI+ Level 2 operation for the USB HS PHY"]
    #[inline(always)]
    pub fn enutmilevel2(&self) -> ENUTMILEVEL2_R {
        ENUTMILEVEL2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enables UTMI+ Level 3 operation for the USB HS PHY"]
    #[inline(always)]
    pub fn enutmilevel3(&self) -> ENUTMILEVEL3_R {
        ENUTMILEVEL3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[inline(always)]
    pub fn enirqwakeup(&self) -> ENIRQWAKEUP_R {
        ENIRQWAKEUP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Wake-up IRQ: Indicates that there is a wak-eup event"]
    #[inline(always)]
    pub fn wakeup_irq(&self) -> WAKEUP_IRQ_R {
        WAKEUP_IRQ_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
    #[inline(always)]
    pub fn autoresume_en(&self) -> AUTORESUME_EN_R {
        AUTORESUME_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_clkgate(&self) -> ENAUTOCLR_CLKGATE_R {
        ENAUTOCLR_CLKGATE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_phy_pwd(&self) -> ENAUTOCLR_PHY_PWD_R {
        ENAUTOCLR_PHY_PWD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable DP DM change wake-up: Not for customer use"]
    #[inline(always)]
    pub fn endpdmchg_wkup(&self) -> ENDPDMCHG_WKUP_R {
        ENDPDMCHG_WKUP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended"]
    #[inline(always)]
    pub fn envbuschg_wkup(&self) -> ENVBUSCHG_WKUP_R {
        ENVBUSCHG_WKUP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[inline(always)]
    pub fn enautoclr_usbclkgate(&self) -> ENAUTOCLR_USBCLKGATE_R {
        ENAUTOCLR_USBCLKGATE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[inline(always)]
    pub fn enautoset_usbclks(&self) -> ENAUTOSET_USBCLKS_R {
        ENAUTOSET_USBCLKS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
    #[inline(always)]
    pub fn host_force_ls_se0(&self) -> HOST_FORCE_LS_SE0_R {
        HOST_FORCE_LS_SE0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub fn utmi_suspendm(&self) -> UTMI_SUSPENDM_R {
        UTMI_SUSPENDM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Gate UTMI Clocks"]
    #[inline(always)]
    pub fn clkgate(&self) -> CLKGATE_R {
        CLKGATE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
    #[inline(always)]
    pub fn sftrst(&self) -> SFTRST_R {
        SFTRST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub fn enhostdiscondetect(&mut self) -> ENHOSTDISCONDETECT_W {
        ENHOSTDISCONDETECT_W { w: self }
    }
    #[doc = "Bit 2 - Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub fn enirqhostdiscon(&mut self) -> ENIRQHOSTDISCON_W {
        ENIRQHOSTDISCON_W { w: self }
    }
    #[doc = "Bit 3 - Indicates that the device has disconnected in High-Speed mode"]
    #[inline(always)]
    pub fn hostdiscondetect_irq(&mut self) -> HOSTDISCONDETECT_IRQ_W {
        HOSTDISCONDETECT_IRQ_W { w: self }
    }
    #[doc = "Bit 4 - Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
    #[inline(always)]
    pub fn endevplugindet(&mut self) -> ENDEVPLUGINDET_W {
        ENDEVPLUGINDET_W { w: self }
    }
    #[doc = "Bit 5 - Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub fn devplugin_polarity(&mut self) -> DEVPLUGIN_POLARITY_W {
        DEVPLUGIN_POLARITY_W { w: self }
    }
    #[doc = "Bit 8 - Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub fn resumeirqsticky(&mut self) -> RESUMEIRQSTICKY_W {
        RESUMEIRQSTICKY_W { w: self }
    }
    #[doc = "Bit 9 - Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub fn enirqresumedetect(&mut self) -> ENIRQRESUMEDETECT_W {
        ENIRQRESUMEDETECT_W { w: self }
    }
    #[doc = "Bit 10 - Resume IRQ: Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub fn resume_irq(&mut self) -> RESUME_IRQ_W {
        RESUME_IRQ_W { w: self }
    }
    #[doc = "Bit 12 - Indicates that the device is connected"]
    #[inline(always)]
    pub fn devplugin_irq(&mut self) -> DEVPLUGIN_IRQ_W {
        DEVPLUGIN_IRQ_W { w: self }
    }
    #[doc = "Bit 14 - Enables UTMI+ Level 2 operation for the USB HS PHY"]
    #[inline(always)]
    pub fn enutmilevel2(&mut self) -> ENUTMILEVEL2_W {
        ENUTMILEVEL2_W { w: self }
    }
    #[doc = "Bit 15 - Enables UTMI+ Level 3 operation for the USB HS PHY"]
    #[inline(always)]
    pub fn enutmilevel3(&mut self) -> ENUTMILEVEL3_W {
        ENUTMILEVEL3_W { w: self }
    }
    #[doc = "Bit 16 - Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[inline(always)]
    pub fn enirqwakeup(&mut self) -> ENIRQWAKEUP_W {
        ENIRQWAKEUP_W { w: self }
    }
    #[doc = "Bit 17 - Wake-up IRQ: Indicates that there is a wak-eup event"]
    #[inline(always)]
    pub fn wakeup_irq(&mut self) -> WAKEUP_IRQ_W {
        WAKEUP_IRQ_W { w: self }
    }
    #[doc = "Bit 18 - Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
    #[inline(always)]
    pub fn autoresume_en(&mut self) -> AUTORESUME_EN_W {
        AUTORESUME_EN_W { w: self }
    }
    #[doc = "Bit 19 - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_clkgate(&mut self) -> ENAUTOCLR_CLKGATE_W {
        ENAUTOCLR_CLKGATE_W { w: self }
    }
    #[doc = "Bit 20 - Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_phy_pwd(&mut self) -> ENAUTOCLR_PHY_PWD_W {
        ENAUTOCLR_PHY_PWD_W { w: self }
    }
    #[doc = "Bit 21 - Enable DP DM change wake-up: Not for customer use"]
    #[inline(always)]
    pub fn endpdmchg_wkup(&mut self) -> ENDPDMCHG_WKUP_W {
        ENDPDMCHG_WKUP_W { w: self }
    }
    #[doc = "Bit 23 - Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended"]
    #[inline(always)]
    pub fn envbuschg_wkup(&mut self) -> ENVBUSCHG_WKUP_W {
        ENVBUSCHG_WKUP_W { w: self }
    }
    #[doc = "Bit 25 - Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[inline(always)]
    pub fn enautoclr_usbclkgate(&mut self) -> ENAUTOCLR_USBCLKGATE_W {
        ENAUTOCLR_USBCLKGATE_W { w: self }
    }
    #[doc = "Bit 26 - Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[inline(always)]
    pub fn enautoset_usbclks(&mut self) -> ENAUTOSET_USBCLKS_W {
        ENAUTOSET_USBCLKS_W { w: self }
    }
    #[doc = "Bit 28 - Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
    #[inline(always)]
    pub fn host_force_ls_se0(&mut self) -> HOST_FORCE_LS_SE0_W {
        HOST_FORCE_LS_SE0_W { w: self }
    }
    #[doc = "Bit 29 - Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub fn utmi_suspendm(&mut self) -> UTMI_SUSPENDM_W {
        UTMI_SUSPENDM_W { w: self }
    }
    #[doc = "Bit 30 - Gate UTMI Clocks"]
    #[inline(always)]
    pub fn clkgate(&mut self) -> CLKGATE_W {
        CLKGATE_W { w: self }
    }
    #[doc = "Bit 31 - Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
    #[inline(always)]
    pub fn sftrst(&mut self) -> SFTRST_W {
        SFTRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0xc000_0000"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc000_0000
    }
}
