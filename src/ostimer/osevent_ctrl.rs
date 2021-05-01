#[doc = "Register `OSEVENT_CTRL` reader"]
pub struct R(crate::R<OSEVENT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSEVENT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OSEVENT_CTRL_SPEC>> for R {
    fn from(reader: crate::R<OSEVENT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSEVENT_CTRL` writer"]
pub struct W(crate::W<OSEVENT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSEVENT_CTRL_SPEC>;
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
impl core::convert::From<crate::W<OSEVENT_CTRL_SPEC>> for W {
    fn from(writer: crate::W<OSEVENT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSTIMER_INTRFLAG` reader - This bit is set when a match occurs between the central 42-bits EVTIMER and the value programmed in the match-register pair. This bit is cleared by writing a '1'. Writes to clear this bit are asynchronous. It should be done before a new match value is written into the MATCH_L/H registers."]
pub struct OSTIMER_INTRFLAG_R(crate::FieldReader<bool, bool>);
impl OSTIMER_INTRFLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSTIMER_INTRFLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSTIMER_INTRFLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSTIMER_INTRFLAG` writer - This bit is set when a match occurs between the central 42-bits EVTIMER and the value programmed in the match-register pair. This bit is cleared by writing a '1'. Writes to clear this bit are asynchronous. It should be done before a new match value is written into the MATCH_L/H registers."]
pub struct OSTIMER_INTRFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> OSTIMER_INTRFLAG_W<'a> {
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
#[doc = "Field `OSTIMER_INTENA` reader - When this bit is '1' an interrupt/wakeup request to the domain processor will be asserted when the OSTIMER_INTR flag is set. When this bit is '0', interrupt/wakeup requests due to the OSTIMER_INTR flag are blocked."]
pub struct OSTIMER_INTENA_R(crate::FieldReader<bool, bool>);
impl OSTIMER_INTENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSTIMER_INTENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSTIMER_INTENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSTIMER_INTENA` writer - When this bit is '1' an interrupt/wakeup request to the domain processor will be asserted when the OSTIMER_INTR flag is set. When this bit is '0', interrupt/wakeup requests due to the OSTIMER_INTR flag are blocked."]
pub struct OSTIMER_INTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OSTIMER_INTENA_W<'a> {
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
#[doc = "Field `MATCH_WR_RDY` reader - This bit will be low when it is safe to write to reload the Match Registers. In typical applications it should not be necessary to test this bit. \\[1\\]"]
pub struct MATCH_WR_RDY_R(crate::FieldReader<bool, bool>);
impl MATCH_WR_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        MATCH_WR_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MATCH_WR_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - This bit is set when a match occurs between the central 42-bits EVTIMER and the value programmed in the match-register pair. This bit is cleared by writing a '1'. Writes to clear this bit are asynchronous. It should be done before a new match value is written into the MATCH_L/H registers."]
    #[inline(always)]
    pub fn ostimer_intrflag(&self) -> OSTIMER_INTRFLAG_R {
        OSTIMER_INTRFLAG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When this bit is '1' an interrupt/wakeup request to the domain processor will be asserted when the OSTIMER_INTR flag is set. When this bit is '0', interrupt/wakeup requests due to the OSTIMER_INTR flag are blocked."]
    #[inline(always)]
    pub fn ostimer_intena(&self) -> OSTIMER_INTENA_R {
        OSTIMER_INTENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit will be low when it is safe to write to reload the Match Registers. In typical applications it should not be necessary to test this bit. \\[1\\]"]
    #[inline(always)]
    pub fn match_wr_rdy(&self) -> MATCH_WR_RDY_R {
        MATCH_WR_RDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set when a match occurs between the central 42-bits EVTIMER and the value programmed in the match-register pair. This bit is cleared by writing a '1'. Writes to clear this bit are asynchronous. It should be done before a new match value is written into the MATCH_L/H registers."]
    #[inline(always)]
    pub fn ostimer_intrflag(&mut self) -> OSTIMER_INTRFLAG_W {
        OSTIMER_INTRFLAG_W { w: self }
    }
    #[doc = "Bit 1 - When this bit is '1' an interrupt/wakeup request to the domain processor will be asserted when the OSTIMER_INTR flag is set. When this bit is '0', interrupt/wakeup requests due to the OSTIMER_INTR flag are blocked."]
    #[inline(always)]
    pub fn ostimer_intena(&mut self) -> OSTIMER_INTENA_W {
        OSTIMER_INTENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OS_EVENT TIMER Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osevent_ctrl](index.html) module"]
pub struct OSEVENT_CTRL_SPEC;
impl crate::RegisterSpec for OSEVENT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osevent_ctrl::R](R) reader structure"]
impl crate::Readable for OSEVENT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osevent_ctrl::W](W) writer structure"]
impl crate::Writable for OSEVENT_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSEVENT_CTRL to value 0"]
impl crate::Resettable for OSEVENT_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
