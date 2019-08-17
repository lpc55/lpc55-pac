#[doc = "Writer for register STARTERSET0"]
pub type W = crate::W<u32, super::STARTERSET0>;
#[doc = "Register STARTERSET0 `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTERSET0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SYS_SET`"]
pub struct SYS_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_SET_W<'a> {
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
#[doc = "Write proxy for field `SDMA0_SET`"]
pub struct SDMA0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA0_SET_W<'a> {
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
#[doc = "Write proxy for field `GPIO_GLOBALINT0_SET`"]
pub struct GPIO_GLOBALINT0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_GLOBALINT0_SET_W<'a> {
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
#[doc = "Write proxy for field `GPIO_GLOBALINT1_SET`"]
pub struct GPIO_GLOBALINT1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_GLOBALINT1_SET_W<'a> {
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
#[doc = "Write proxy for field `GPIO_INT00_SET`"]
pub struct GPIO_INT00_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT00_SET_W<'a> {
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
#[doc = "Write proxy for field `GPIO_INT01_SET`"]
pub struct GPIO_INT01_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT01_SET_W<'a> {
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
#[doc = "Write proxy for field `GPIO_INT02_SET`"]
pub struct GPIO_INT02_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT02_SET_W<'a> {
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
#[doc = "Write proxy for field `GPIO_INT03_SET`"]
pub struct GPIO_INT03_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT03_SET_W<'a> {
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
#[doc = "Write proxy for field `UTICK0_SET`"]
pub struct UTICK0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> UTICK0_SET_W<'a> {
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
#[doc = "Write proxy for field `MRT0_SET`"]
pub struct MRT0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> MRT0_SET_W<'a> {
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
#[doc = "Write proxy for field `CTIMER0_SET`"]
pub struct CTIMER0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER0_SET_W<'a> {
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
#[doc = "Write proxy for field `CTIMER1_SET`"]
pub struct CTIMER1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER1_SET_W<'a> {
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
#[doc = "Write proxy for field `SCT0_SET`"]
pub struct SCT0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT0_SET_W<'a> {
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
#[doc = "Write proxy for field `CTIMER3_SET`"]
pub struct CTIMER3_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER3_SET_W<'a> {
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
#[doc = "Write proxy for field `FLEXINT0_SET`"]
pub struct FLEXINT0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT0_SET_W<'a> {
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
#[doc = "Write proxy for field `FLEXINT1_SET`"]
pub struct FLEXINT1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT1_SET_W<'a> {
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
#[doc = "Write proxy for field `FLEXINT2_SET`"]
pub struct FLEXINT2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT2_SET_W<'a> {
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
#[doc = "Write proxy for field `FLEXINT3_SET`"]
pub struct FLEXINT3_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT3_SET_W<'a> {
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
#[doc = "Write proxy for field `FLEXINT4_SET`"]
pub struct FLEXINT4_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT4_SET_W<'a> {
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
#[doc = "Write proxy for field `FLEXINT5_SET`"]
pub struct FLEXINT5_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT5_SET_W<'a> {
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
#[doc = "Write proxy for field `FLEXINT6_SET`"]
pub struct FLEXINT6_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT6_SET_W<'a> {
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
#[doc = "Write proxy for field `FLEXINT7_SET`"]
pub struct FLEXINT7_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXINT7_SET_W<'a> {
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
#[doc = "Write proxy for field `ADC0_SET`"]
pub struct ADC0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_SET_W<'a> {
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
#[doc = "Write proxy for field `ADC0_THCMP_OVR_SET`"]
pub struct ADC0_THCMP_OVR_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_THCMP_OVR_SET_W<'a> {
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
#[doc = "Write proxy for field `USB0_NEEDCLK_SET`"]
pub struct USB0_NEEDCLK_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_NEEDCLK_SET_W<'a> {
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
#[doc = "Write proxy for field `USB0_SET`"]
pub struct USB0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_SET_W<'a> {
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
#[doc = "Write proxy for field `RTC_LITE0_SET`"]
pub struct RTC_LITE0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_LITE0_SET_W<'a> {
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
#[doc = "Write proxy for field `EZH_ARCH_B0_SET`"]
pub struct EZH_ARCH_B0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> EZH_ARCH_B0_SET_W<'a> {
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
#[doc = "Write proxy for field `WAKEUP_MAILBOX0_SET`"]
pub struct WAKEUP_MAILBOX0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_MAILBOX0_SET_W<'a> {
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
    #[doc = "Bit 0 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn sys_set(&mut self) -> SYS_SET_W {
        SYS_SET_W { w: self }
    }
    #[doc = "Bit 1 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn sdma0_set(&mut self) -> SDMA0_SET_W {
        SDMA0_SET_W { w: self }
    }
    #[doc = "Bit 2 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn gpio_globalint0_set(&mut self) -> GPIO_GLOBALINT0_SET_W {
        GPIO_GLOBALINT0_SET_W { w: self }
    }
    #[doc = "Bit 3 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn gpio_globalint1_set(&mut self) -> GPIO_GLOBALINT1_SET_W {
        GPIO_GLOBALINT1_SET_W { w: self }
    }
    #[doc = "Bit 4 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn gpio_int00_set(&mut self) -> GPIO_INT00_SET_W {
        GPIO_INT00_SET_W { w: self }
    }
    #[doc = "Bit 5 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn gpio_int01_set(&mut self) -> GPIO_INT01_SET_W {
        GPIO_INT01_SET_W { w: self }
    }
    #[doc = "Bit 6 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn gpio_int02_set(&mut self) -> GPIO_INT02_SET_W {
        GPIO_INT02_SET_W { w: self }
    }
    #[doc = "Bit 7 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn gpio_int03_set(&mut self) -> GPIO_INT03_SET_W {
        GPIO_INT03_SET_W { w: self }
    }
    #[doc = "Bit 8 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn utick0_set(&mut self) -> UTICK0_SET_W {
        UTICK0_SET_W { w: self }
    }
    #[doc = "Bit 9 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn mrt0_set(&mut self) -> MRT0_SET_W {
        MRT0_SET_W { w: self }
    }
    #[doc = "Bit 10 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn ctimer0_set(&mut self) -> CTIMER0_SET_W {
        CTIMER0_SET_W { w: self }
    }
    #[doc = "Bit 11 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn ctimer1_set(&mut self) -> CTIMER1_SET_W {
        CTIMER1_SET_W { w: self }
    }
    #[doc = "Bit 12 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn sct0_set(&mut self) -> SCT0_SET_W {
        SCT0_SET_W { w: self }
    }
    #[doc = "Bit 13 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn ctimer3_set(&mut self) -> CTIMER3_SET_W {
        CTIMER3_SET_W { w: self }
    }
    #[doc = "Bit 14 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn flexint0_set(&mut self) -> FLEXINT0_SET_W {
        FLEXINT0_SET_W { w: self }
    }
    #[doc = "Bit 15 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn flexint1_set(&mut self) -> FLEXINT1_SET_W {
        FLEXINT1_SET_W { w: self }
    }
    #[doc = "Bit 16 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn flexint2_set(&mut self) -> FLEXINT2_SET_W {
        FLEXINT2_SET_W { w: self }
    }
    #[doc = "Bit 17 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn flexint3_set(&mut self) -> FLEXINT3_SET_W {
        FLEXINT3_SET_W { w: self }
    }
    #[doc = "Bit 18 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn flexint4_set(&mut self) -> FLEXINT4_SET_W {
        FLEXINT4_SET_W { w: self }
    }
    #[doc = "Bit 19 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn flexint5_set(&mut self) -> FLEXINT5_SET_W {
        FLEXINT5_SET_W { w: self }
    }
    #[doc = "Bit 20 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn flexint6_set(&mut self) -> FLEXINT6_SET_W {
        FLEXINT6_SET_W { w: self }
    }
    #[doc = "Bit 21 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn flexint7_set(&mut self) -> FLEXINT7_SET_W {
        FLEXINT7_SET_W { w: self }
    }
    #[doc = "Bit 22 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn adc0_set(&mut self) -> ADC0_SET_W {
        ADC0_SET_W { w: self }
    }
    #[doc = "Bit 24 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn adc0_thcmp_ovr_set(&mut self) -> ADC0_THCMP_OVR_SET_W {
        ADC0_THCMP_OVR_SET_W { w: self }
    }
    #[doc = "Bit 27 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn usb0_needclk_set(&mut self) -> USB0_NEEDCLK_SET_W {
        USB0_NEEDCLK_SET_W { w: self }
    }
    #[doc = "Bit 28 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn usb0_set(&mut self) -> USB0_SET_W {
        USB0_SET_W { w: self }
    }
    #[doc = "Bit 29 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn rtc_lite0_set(&mut self) -> RTC_LITE0_SET_W {
        RTC_LITE0_SET_W { w: self }
    }
    #[doc = "Bit 30 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn ezh_arch_b0_set(&mut self) -> EZH_ARCH_B0_SET_W {
        EZH_ARCH_B0_SET_W { w: self }
    }
    #[doc = "Bit 31 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline(always)]
    pub fn wakeup_mailbox0_set(&mut self) -> WAKEUP_MAILBOX0_SET_W {
        WAKEUP_MAILBOX0_SET_W { w: self }
    }
}
