#[doc = "Reader of register OSEVENT_CTRL"]
pub type R = crate::R<u32, super::OSEVENT_CTRL>;
#[doc = "Writer for register OSEVENT_CTRL"]
pub type W = crate::W<u32, super::OSEVENT_CTRL>;
#[doc = "Register OSEVENT_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::OSEVENT_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OSTIMER_INTRFLAG`"]
pub type OSTIMER_INTRFLAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSTIMER_INTRFLAG`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `OSTIMER_INTENA`"]
pub type OSTIMER_INTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSTIMER_INTENA`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - This bit is set when a match occurs between the central 64-bit EVTIMER and the value programmed in the Match-register pair for the associated CPU This bit is cleared by writing a '1'. Writes to clear this bit are asynchronous. This should be done before a new match value is written into the MATCH_L/H registers"]
    #[inline(always)]
    pub fn ostimer_intrflag(&self) -> OSTIMER_INTRFLAG_R {
        OSTIMER_INTRFLAG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When this bit is '1' an interrupt/wakeup request to the Domainn processor will be asserted when the OSTIMER_INTR flag is set. When this bit is '0', interrupt/wakeup requests due to the OSTIMER_INTR flag are blocked.A separate OSEVENT_CTRL register is implemented for each CPU. Each CPU reads its own local value at the same address."]
    #[inline(always)]
    pub fn ostimer_intena(&self) -> OSTIMER_INTENA_R {
        OSTIMER_INTENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set when a match occurs between the central 64-bit EVTIMER and the value programmed in the Match-register pair for the associated CPU This bit is cleared by writing a '1'. Writes to clear this bit are asynchronous. This should be done before a new match value is written into the MATCH_L/H registers"]
    #[inline(always)]
    pub fn ostimer_intrflag(&mut self) -> OSTIMER_INTRFLAG_W {
        OSTIMER_INTRFLAG_W { w: self }
    }
    #[doc = "Bit 1 - When this bit is '1' an interrupt/wakeup request to the Domainn processor will be asserted when the OSTIMER_INTR flag is set. When this bit is '0', interrupt/wakeup requests due to the OSTIMER_INTR flag are blocked.A separate OSEVENT_CTRL register is implemented for each CPU. Each CPU reads its own local value at the same address."]
    #[inline(always)]
    pub fn ostimer_intena(&mut self) -> OSTIMER_INTENA_W {
        OSTIMER_INTENA_W { w: self }
    }
}
