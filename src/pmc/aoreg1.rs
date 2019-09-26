#[doc = "Reader of register AOREG1"]
pub type R = crate::R<u32, super::AOREG1>;
#[doc = "Writer for register AOREG1"]
pub type W = crate::W<u32, super::AOREG1>;
#[doc = "Register AOREG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::AOREG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `POR`"]
pub type POR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POR`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `PADRESET`"]
pub type PADRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PADRESET`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `BODRESET`"]
pub type BODRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BODRESET`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SYSTEMRESET`"]
pub type SYSTEMRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEMRESET`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `WDTRESET`"]
pub type WDTRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTRESET`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SWRRESET`"]
pub type SWRRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWRRESET`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DPDRESET_WAKEUPIO`"]
pub type DPDRESET_WAKEUPIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPDRESET_WAKEUPIO`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DPDRESET_RTC`"]
pub type DPDRESET_RTC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPDRESET_RTC`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `DPDRESET_OSTIMER`"]
pub type DPDRESET_OSTIMER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPDRESET_OSTIMER`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `BOOTERRORCOUNTER`"]
pub type BOOTERRORCOUNTER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BOOTERRORCOUNTER`"]
pub struct BOOTERRORCOUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTERRORCOUNTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
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
}
