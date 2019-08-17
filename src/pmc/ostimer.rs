#[doc = "Reader of register OSTIMER"]
pub type R = crate::R<u32, super::OSTIMER>;
#[doc = "Writer for register OSTIMER"]
pub type W = crate::W<u32, super::OSTIMER>;
#[doc = "Register OSTIMER `reset()`'s with value 0x08"]
impl crate::ResetValue for super::OSTIMER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Reader of field `SOFTRESET`"]
pub type SOFTRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTRESET`"]
pub struct SOFTRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTRESET_W<'a> {
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
#[doc = "Reader of field `CLOCKENABLE`"]
pub type CLOCKENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLOCKENABLE`"]
pub struct CLOCKENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCKENABLE_W<'a> {
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
#[doc = "Reader of field `DPDWAKEUPENABLE`"]
pub type DPDWAKEUPENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPDWAKEUPENABLE`"]
pub struct DPDWAKEUPENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPDWAKEUPENABLE_W<'a> {
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
#[doc = "Reader of field `OSC32KPD`"]
pub type OSC32KPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSC32KPD`"]
pub struct OSC32KPD_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC32KPD_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Active high reset."]
    #[inline(always)]
    pub fn softreset(&self) -> SOFTRESET_R {
        SOFTRESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable OSTIMER 32 KHz clock."]
    #[inline(always)]
    pub fn clockenable(&self) -> CLOCKENABLE_R {
        CLOCKENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
    #[inline(always)]
    pub fn dpdwakeupenable(&self) -> DPDWAKEUPENABLE_R {
        DPDWAKEUPENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
    #[inline(always)]
    pub fn osc32kpd(&self) -> OSC32KPD_R {
        OSC32KPD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active high reset."]
    #[inline(always)]
    pub fn softreset(&mut self) -> SOFTRESET_W {
        SOFTRESET_W { w: self }
    }
    #[doc = "Bit 1 - Enable OSTIMER 32 KHz clock."]
    #[inline(always)]
    pub fn clockenable(&mut self) -> CLOCKENABLE_W {
        CLOCKENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
    #[inline(always)]
    pub fn dpdwakeupenable(&mut self) -> DPDWAKEUPENABLE_W {
        DPDWAKEUPENABLE_W { w: self }
    }
    #[doc = "Bit 3 - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
    #[inline(always)]
    pub fn osc32kpd(&mut self) -> OSC32KPD_W {
        OSC32KPD_W { w: self }
    }
}
