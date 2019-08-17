#[doc = "Writer for register STARTERCLR0"]
pub type W = crate::W<u32, super::STARTERCLR0>;
#[doc = "Register STARTERCLR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTERCLR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SYS_CLR`"]
pub struct SYS_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_CLR_W<'a> {
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
#[doc = "Write proxy for field `SDMA0_CLR`"]
pub struct SDMA0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA0_CLR_W<'a> {
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
#[doc = "Write proxy for field `GPIO_GLOBALINT0_CLR`"]
pub struct GPIO_GLOBALINT0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_GLOBALINT0_CLR_W<'a> {
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
#[doc = "Write proxy for field `GPIO_GLOBALINT1_CLR`"]
pub struct GPIO_GLOBALINT1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_GLOBALINT1_CLR_W<'a> {
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
#[doc = "Write proxy for field `GPIO_INT00_CLR`"]
pub struct GPIO_INT00_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT00_CLR_W<'a> {
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
#[doc = "Write proxy for field `GPIO_INT01_CLR`"]
pub struct GPIO_INT01_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT01_CLR_W<'a> {
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
#[doc = "Write proxy for field `GPIO_INT02_CLR`"]
pub struct GPIO_INT02_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT02_CLR_W<'a> {
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
#[doc = "Write proxy for field `GPIO_INT03_CLR`"]
pub struct GPIO_INT03_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT03_CLR_W<'a> {
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
#[doc = "Write proxy for field `UTICK0_CLR`"]
pub struct UTICK0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UTICK0_CLR_W<'a> {
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
#[doc = "Write proxy for field `MRT0_CLR`"]
pub struct MRT0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MRT0_CLR_W<'a> {
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
#[doc = "Write proxy for field `CTIMER0_CLR`"]
pub struct CTIMER0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER0_CLR_W<'a> {
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
#[doc = "Write proxy for field `CTIMER1_CLR`"]
pub struct CTIMER1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER1_CLR_W<'a> {
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
#[doc = "Write proxy for field `SCT0_CLR`"]
pub struct SCT0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT0_CLR_W<'a> {
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
#[doc = "Write proxy for field `CTIMER3_CLR`"]
pub struct CTIMER3_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER3_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Write proxy for field `FLEXINT0_CLR`"]
pub struct FLEXINT0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT0_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `FLEXINT1_CLR`"]
pub struct FLEXINT1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT1_CLR_W<'a> {
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
#[doc = "Write proxy for field `FLEXINT2_CLR`"]
pub struct FLEXINT2_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT2_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `FLEXINT3_CLR`"]
pub struct FLEXINT3_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT3_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Write proxy for field `FLEXINT4_CLR`"]
pub struct FLEXINT4_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT4_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Write proxy for field `FLEXINT5_CLR`"]
pub struct FLEXINT5_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT5_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Write proxy for field `FLEXINT6_CLR`"]
pub struct FLEXINT6_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT6_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Write proxy for field `FLEXINT7_CLR`"]
pub struct FLEXINT7_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT7_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Write proxy for field `ADC0_CLR`"]
pub struct ADC0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Write proxy for field `ADC0_THCMP_OVR_CLR`"]
pub struct ADC0_THCMP_OVR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_THCMP_OVR_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Write proxy for field `USB0_NEEDCLK_CLR`"]
pub struct USB0_NEEDCLK_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_NEEDCLK_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Write proxy for field `USB0_CLR`"]
pub struct USB0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Write proxy for field `RTC_LITE0_CLR`"]
pub struct RTC_LITE0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_LITE0_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Write proxy for field `EZH_ARCH_B0_CLR`"]
pub struct EZH_ARCH_B0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EZH_ARCH_B0_CLR_W<'a> {
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
#[doc = "Write proxy for field `WAKEUP_MAILBOX0_CLR`"]
pub struct WAKEUP_MAILBOX0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_MAILBOX0_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn sys_clr(&mut self) -> SYS_CLR_W {
        SYS_CLR_W { w: self }
    }
    #[doc = "Bit 1 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn sdma0_clr(&mut self) -> SDMA0_CLR_W {
        SDMA0_CLR_W { w: self }
    }
    #[doc = "Bit 2 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn gpio_globalint0_clr(&mut self) -> GPIO_GLOBALINT0_CLR_W {
        GPIO_GLOBALINT0_CLR_W { w: self }
    }
    #[doc = "Bit 3 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn gpio_globalint1_clr(&mut self) -> GPIO_GLOBALINT1_CLR_W {
        GPIO_GLOBALINT1_CLR_W { w: self }
    }
    #[doc = "Bit 4 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn gpio_int00_clr(&mut self) -> GPIO_INT00_CLR_W {
        GPIO_INT00_CLR_W { w: self }
    }
    #[doc = "Bit 5 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn gpio_int01_clr(&mut self) -> GPIO_INT01_CLR_W {
        GPIO_INT01_CLR_W { w: self }
    }
    #[doc = "Bit 6 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn gpio_int02_clr(&mut self) -> GPIO_INT02_CLR_W {
        GPIO_INT02_CLR_W { w: self }
    }
    #[doc = "Bit 7 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn gpio_int03_clr(&mut self) -> GPIO_INT03_CLR_W {
        GPIO_INT03_CLR_W { w: self }
    }
    #[doc = "Bit 8 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn utick0_clr(&mut self) -> UTICK0_CLR_W {
        UTICK0_CLR_W { w: self }
    }
    #[doc = "Bit 9 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn mrt0_clr(&mut self) -> MRT0_CLR_W {
        MRT0_CLR_W { w: self }
    }
    #[doc = "Bit 10 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn ctimer0_clr(&mut self) -> CTIMER0_CLR_W {
        CTIMER0_CLR_W { w: self }
    }
    #[doc = "Bit 11 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn ctimer1_clr(&mut self) -> CTIMER1_CLR_W {
        CTIMER1_CLR_W { w: self }
    }
    #[doc = "Bit 12 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn sct0_clr(&mut self) -> SCT0_CLR_W {
        SCT0_CLR_W { w: self }
    }
    #[doc = "Bit 13 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn ctimer3_clr(&mut self) -> CTIMER3_CLR_W {
        CTIMER3_CLR_W { w: self }
    }
    #[doc = "Bit 14 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn flexint0_clr(&mut self) -> FLEXINT0_CLR_W {
        FLEXINT0_CLR_W { w: self }
    }
    #[doc = "Bit 15 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn flexint1_clr(&mut self) -> FLEXINT1_CLR_W {
        FLEXINT1_CLR_W { w: self }
    }
    #[doc = "Bit 16 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn flexint2_clr(&mut self) -> FLEXINT2_CLR_W {
        FLEXINT2_CLR_W { w: self }
    }
    #[doc = "Bit 17 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn flexint3_clr(&mut self) -> FLEXINT3_CLR_W {
        FLEXINT3_CLR_W { w: self }
    }
    #[doc = "Bit 18 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn flexint4_clr(&mut self) -> FLEXINT4_CLR_W {
        FLEXINT4_CLR_W { w: self }
    }
    #[doc = "Bit 19 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn flexint5_clr(&mut self) -> FLEXINT5_CLR_W {
        FLEXINT5_CLR_W { w: self }
    }
    #[doc = "Bit 20 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn flexint6_clr(&mut self) -> FLEXINT6_CLR_W {
        FLEXINT6_CLR_W { w: self }
    }
    #[doc = "Bit 21 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn flexint7_clr(&mut self) -> FLEXINT7_CLR_W {
        FLEXINT7_CLR_W { w: self }
    }
    #[doc = "Bit 22 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn adc0_clr(&mut self) -> ADC0_CLR_W {
        ADC0_CLR_W { w: self }
    }
    #[doc = "Bit 24 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn adc0_thcmp_ovr_clr(&mut self) -> ADC0_THCMP_OVR_CLR_W {
        ADC0_THCMP_OVR_CLR_W { w: self }
    }
    #[doc = "Bit 27 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn usb0_needclk_clr(&mut self) -> USB0_NEEDCLK_CLR_W {
        USB0_NEEDCLK_CLR_W { w: self }
    }
    #[doc = "Bit 28 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn usb0_clr(&mut self) -> USB0_CLR_W {
        USB0_CLR_W { w: self }
    }
    #[doc = "Bit 29 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn rtc_lite0_clr(&mut self) -> RTC_LITE0_CLR_W {
        RTC_LITE0_CLR_W { w: self }
    }
    #[doc = "Bit 30 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn ezh_arch_b0_clr(&mut self) -> EZH_ARCH_B0_CLR_W {
        EZH_ARCH_B0_CLR_W { w: self }
    }
    #[doc = "Bit 31 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn wakeup_mailbox0_clr(&mut self) -> WAKEUP_MAILBOX0_CLR_W {
        WAKEUP_MAILBOX0_CLR_W { w: self }
    }
}
