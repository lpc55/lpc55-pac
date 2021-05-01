#[doc = "Register `AOREG1` reader"]
pub struct R(crate::R<AOREG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AOREG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AOREG1_SPEC>> for R {
    fn from(reader: crate::R<AOREG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AOREG1` writer"]
pub struct W(crate::W<AOREG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AOREG1_SPEC>;
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
impl core::convert::From<crate::W<AOREG1_SPEC>> for W {
    fn from(writer: crate::W<AOREG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POR` reader - The last chip reset was caused by a Power On Reset."]
pub struct POR_R(crate::FieldReader<bool, bool>);
impl POR_R {
    pub(crate) fn new(bits: bool) -> Self {
        POR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POR` writer - The last chip reset was caused by a Power On Reset."]
pub struct POR_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_W<'a> {
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
#[doc = "Field `PADRESET` reader - The last chip reset was caused by a Pin Reset."]
pub struct PADRESET_R(crate::FieldReader<bool, bool>);
impl PADRESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        PADRESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PADRESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PADRESET` writer - The last chip reset was caused by a Pin Reset."]
pub struct PADRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> PADRESET_W<'a> {
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
#[doc = "Field `BODRESET` reader - The last chip reset was caused by a Brown Out Detector (BoD), either VBAT BoD or Core Logic BoD."]
pub struct BODRESET_R(crate::FieldReader<bool, bool>);
impl BODRESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODRESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODRESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODRESET` writer - The last chip reset was caused by a Brown Out Detector (BoD), either VBAT BoD or Core Logic BoD."]
pub struct BODRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> BODRESET_W<'a> {
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
#[doc = "Field `SYSTEMRESET` reader - The last chip reset was caused by a System Reset requested by the ARM CPU."]
pub struct SYSTEMRESET_R(crate::FieldReader<bool, bool>);
impl SYSTEMRESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSTEMRESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSTEMRESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSTEMRESET` writer - The last chip reset was caused by a System Reset requested by the ARM CPU."]
pub struct SYSTEMRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEMRESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `WDTRESET` reader - The last chip reset was caused by the Watchdog Timer."]
pub struct WDTRESET_R(crate::FieldReader<bool, bool>);
impl WDTRESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTRESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTRESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTRESET` writer - The last chip reset was caused by the Watchdog Timer."]
pub struct WDTRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTRESET_W<'a> {
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
#[doc = "Field `SWRRESET` reader - The last chip reset was caused by a Software event."]
pub struct SWRRESET_R(crate::FieldReader<bool, bool>);
impl SWRRESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRRESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRRESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRRESET` writer - The last chip reset was caused by a Software event."]
pub struct SWRRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRRESET_W<'a> {
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
#[doc = "Field `DPDRESET_WAKEUPIO` reader - The last chip reset was caused by a Wake-up I/O reset event during a Deep Power-Down mode."]
pub struct DPDRESET_WAKEUPIO_R(crate::FieldReader<bool, bool>);
impl DPDRESET_WAKEUPIO_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPDRESET_WAKEUPIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPDRESET_WAKEUPIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPDRESET_WAKEUPIO` writer - The last chip reset was caused by a Wake-up I/O reset event during a Deep Power-Down mode."]
pub struct DPDRESET_WAKEUPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPDRESET_WAKEUPIO_W<'a> {
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
#[doc = "Field `DPDRESET_RTC` reader - The last chip reset was caused by an RTC (either RTC Alarm or RTC wake up) reset event during a Deep Power-Down mode."]
pub struct DPDRESET_RTC_R(crate::FieldReader<bool, bool>);
impl DPDRESET_RTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPDRESET_RTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPDRESET_RTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPDRESET_RTC` writer - The last chip reset was caused by an RTC (either RTC Alarm or RTC wake up) reset event during a Deep Power-Down mode."]
pub struct DPDRESET_RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> DPDRESET_RTC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `DPDRESET_OSTIMER` reader - The last chip reset was caused by an OS Event Timer reset event during a Deep Power-Down mode."]
pub struct DPDRESET_OSTIMER_R(crate::FieldReader<bool, bool>);
impl DPDRESET_OSTIMER_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPDRESET_OSTIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPDRESET_OSTIMER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPDRESET_OSTIMER` writer - The last chip reset was caused by an OS Event Timer reset event during a Deep Power-Down mode."]
pub struct DPDRESET_OSTIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> DPDRESET_OSTIMER_W<'a> {
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
#[doc = "Field `BOOTERRORCOUNTER` reader - ROM Boot Fatal Error Counter."]
pub struct BOOTERRORCOUNTER_R(crate::FieldReader<u8, u8>);
impl BOOTERRORCOUNTER_R {
    pub(crate) fn new(bits: u8) -> Self {
        BOOTERRORCOUNTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOTERRORCOUNTER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOTERRORCOUNTER` writer - ROM Boot Fatal Error Counter."]
pub struct BOOTERRORCOUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTERRORCOUNTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - The last chip reset was caused by a Power On Reset."]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The last chip reset was caused by a Pin Reset."]
    #[inline(always)]
    pub fn padreset(&self) -> PADRESET_R {
        PADRESET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The last chip reset was caused by a Brown Out Detector (BoD), either VBAT BoD or Core Logic BoD."]
    #[inline(always)]
    pub fn bodreset(&self) -> BODRESET_R {
        BODRESET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The last chip reset was caused by a System Reset requested by the ARM CPU."]
    #[inline(always)]
    pub fn systemreset(&self) -> SYSTEMRESET_R {
        SYSTEMRESET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The last chip reset was caused by the Watchdog Timer."]
    #[inline(always)]
    pub fn wdtreset(&self) -> WDTRESET_R {
        WDTRESET_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The last chip reset was caused by a Software event."]
    #[inline(always)]
    pub fn swrreset(&self) -> SWRRESET_R {
        SWRRESET_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The last chip reset was caused by a Wake-up I/O reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub fn dpdreset_wakeupio(&self) -> DPDRESET_WAKEUPIO_R {
        DPDRESET_WAKEUPIO_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - The last chip reset was caused by an RTC (either RTC Alarm or RTC wake up) reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub fn dpdreset_rtc(&self) -> DPDRESET_RTC_R {
        DPDRESET_RTC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - The last chip reset was caused by an OS Event Timer reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub fn dpdreset_ostimer(&self) -> DPDRESET_OSTIMER_R {
        DPDRESET_OSTIMER_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - ROM Boot Fatal Error Counter."]
    #[inline(always)]
    pub fn booterrorcounter(&self) -> BOOTERRORCOUNTER_R {
        BOOTERRORCOUNTER_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - The last chip reset was caused by a Power On Reset."]
    #[inline(always)]
    pub fn por(&mut self) -> POR_W {
        POR_W { w: self }
    }
    #[doc = "Bit 5 - The last chip reset was caused by a Pin Reset."]
    #[inline(always)]
    pub fn padreset(&mut self) -> PADRESET_W {
        PADRESET_W { w: self }
    }
    #[doc = "Bit 6 - The last chip reset was caused by a Brown Out Detector (BoD), either VBAT BoD or Core Logic BoD."]
    #[inline(always)]
    pub fn bodreset(&mut self) -> BODRESET_W {
        BODRESET_W { w: self }
    }
    #[doc = "Bit 7 - The last chip reset was caused by a System Reset requested by the ARM CPU."]
    #[inline(always)]
    pub fn systemreset(&mut self) -> SYSTEMRESET_W {
        SYSTEMRESET_W { w: self }
    }
    #[doc = "Bit 8 - The last chip reset was caused by the Watchdog Timer."]
    #[inline(always)]
    pub fn wdtreset(&mut self) -> WDTRESET_W {
        WDTRESET_W { w: self }
    }
    #[doc = "Bit 9 - The last chip reset was caused by a Software event."]
    #[inline(always)]
    pub fn swrreset(&mut self) -> SWRRESET_W {
        SWRRESET_W { w: self }
    }
    #[doc = "Bit 10 - The last chip reset was caused by a Wake-up I/O reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub fn dpdreset_wakeupio(&mut self) -> DPDRESET_WAKEUPIO_W {
        DPDRESET_WAKEUPIO_W { w: self }
    }
    #[doc = "Bit 11 - The last chip reset was caused by an RTC (either RTC Alarm or RTC wake up) reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub fn dpdreset_rtc(&mut self) -> DPDRESET_RTC_W {
        DPDRESET_RTC_W { w: self }
    }
    #[doc = "Bit 12 - The last chip reset was caused by an OS Event Timer reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub fn dpdreset_ostimer(&mut self) -> DPDRESET_OSTIMER_W {
        DPDRESET_OSTIMER_W { w: self }
    }
    #[doc = "Bits 16:19 - ROM Boot Fatal Error Counter."]
    #[inline(always)]
    pub fn booterrorcounter(&mut self) -> BOOTERRORCOUNTER_W {
        BOOTERRORCOUNTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aoreg1](index.html) module"]
pub struct AOREG1_SPEC;
impl crate::RegisterSpec for AOREG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aoreg1::R](R) reader structure"]
impl crate::Readable for AOREG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aoreg1::W](W) writer structure"]
impl crate::Writable for AOREG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AOREG1 to value 0"]
impl crate::Resettable for AOREG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
