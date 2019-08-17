#[doc = "Reader of register RTCOSC32K"]
pub type R = crate::R<u32, super::RTCOSC32K>;
#[doc = "Writer for register RTCOSC32K"]
pub type W = crate::W<u32, super::RTCOSC32K>;
#[doc = "Register RTCOSC32K `reset()`'s with value 0x03ff_0008"]
impl crate::ResetValue for super::RTCOSC32K {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03ff_0008
    }
}
#[doc = "Possible values of the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_A {
    #[doc = "FRO 32 KHz."]
    FRO32K,
    #[doc = "XTAL 32KHz."]
    XTAL32K,
}
impl From<SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        match variant {
            SEL_A::FRO32K => false,
            SEL_A::XTAL32K => true,
        }
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<bool, SEL_A>;
impl SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            false => SEL_A::FRO32K,
            true => SEL_A::XTAL32K,
        }
    }
    #[doc = "Checks if the value of the field is `FRO32K`"]
    #[inline(always)]
    pub fn is_fro32k(&self) -> bool {
        *self == SEL_A::FRO32K
    }
    #[doc = "Checks if the value of the field is `XTAL32K`"]
    #[inline(always)]
    pub fn is_xtal32k(&self) -> bool {
        *self == SEL_A::XTAL32K
    }
}
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FRO 32 KHz."]
    #[inline(always)]
    pub fn fro32k(self) -> &'a mut W {
        self.variant(SEL_A::FRO32K)
    }
    #[doc = "XTAL 32KHz."]
    #[inline(always)]
    pub fn xtal32k(self) -> &'a mut W {
        self.variant(SEL_A::XTAL32K)
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
#[doc = "Reader of field `CLK1KHZDIV`"]
pub type CLK1KHZDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLK1KHZDIV`"]
pub struct CLK1KHZDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK1KHZDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `CLK1KHZDIVUPDATEREQ`"]
pub type CLK1KHZDIVUPDATEREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK1KHZDIVUPDATEREQ`"]
pub struct CLK1KHZDIVUPDATEREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK1KHZDIVUPDATEREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `CLK1HZDIV`"]
pub type CLK1HZDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CLK1HZDIV`"]
pub struct CLK1HZDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK1HZDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CLK1HZDIVHALT`"]
pub type CLK1HZDIVHALT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK1HZDIVHALT`"]
pub struct CLK1HZDIVHALT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK1HZDIVHALT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `CLK1HZDIVUPDATEREQ`"]
pub type CLK1HZDIVUPDATEREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK1HZDIVUPDATEREQ`"]
pub struct CLK1HZDIVUPDATEREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK1HZDIVUPDATEREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) ."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Actual division ratio is : 28 + CLK1KHZDIV."]
    #[inline(always)]
    pub fn clk1khzdiv(&self) -> CLK1KHZDIV_R {
        CLK1KHZDIV_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 15 - RTC 1KHz clock Divider status flag."]
    #[inline(always)]
    pub fn clk1khzdivupdatereq(&self) -> CLK1KHZDIVUPDATEREQ_R {
        CLK1KHZDIVUPDATEREQ_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:26 - Actual division ratio is : 31744 + CLK1HZDIV."]
    #[inline(always)]
    pub fn clk1hzdiv(&self) -> CLK1HZDIV_R {
        CLK1HZDIV_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline(always)]
    pub fn clk1hzdivhalt(&self) -> CLK1HZDIVHALT_R {
        CLK1HZDIVHALT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - RTC 1Hz Divider status flag."]
    #[inline(always)]
    pub fn clk1hzdivupdatereq(&self) -> CLK1HZDIVUPDATEREQ_R {
        CLK1HZDIVUPDATEREQ_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) ."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Bits 1:3 - Actual division ratio is : 28 + CLK1KHZDIV."]
    #[inline(always)]
    pub fn clk1khzdiv(&mut self) -> CLK1KHZDIV_W {
        CLK1KHZDIV_W { w: self }
    }
    #[doc = "Bit 15 - RTC 1KHz clock Divider status flag."]
    #[inline(always)]
    pub fn clk1khzdivupdatereq(&mut self) -> CLK1KHZDIVUPDATEREQ_W {
        CLK1KHZDIVUPDATEREQ_W { w: self }
    }
    #[doc = "Bits 16:26 - Actual division ratio is : 31744 + CLK1HZDIV."]
    #[inline(always)]
    pub fn clk1hzdiv(&mut self) -> CLK1HZDIV_W {
        CLK1HZDIV_W { w: self }
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline(always)]
    pub fn clk1hzdivhalt(&mut self) -> CLK1HZDIVHALT_W {
        CLK1HZDIVHALT_W { w: self }
    }
    #[doc = "Bit 31 - RTC 1Hz Divider status flag."]
    #[inline(always)]
    pub fn clk1hzdivupdatereq(&mut self) -> CLK1HZDIVUPDATEREQ_W {
        CLK1HZDIVUPDATEREQ_W { w: self }
    }
}
