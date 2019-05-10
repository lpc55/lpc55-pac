#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STARTERCLR0 {
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
}
#[doc = r" Proxy"]
pub struct _SYS_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SDMA0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _SDMA0_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_GLOBALINT0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GLOBALINT0_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_GLOBALINT1_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GLOBALINT1_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_INT00_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT00_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_INT01_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT01_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_INT02_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT02_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_INT03_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT03_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UTICK0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _UTICK0_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MRT0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _MRT0_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTIMER0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER0_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTIMER1_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER1_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SCT0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _SCT0_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTIMER3_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER3_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FLEXINT0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT0_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FLEXINT1_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT1_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FLEXINT2_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT2_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FLEXINT3_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT3_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FLEXINT4_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT4_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FLEXINT5_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT5_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FLEXINT6_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT6_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FLEXINT7_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT7_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADC0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADC0_THCMP_OVR_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0_THCMP_OVR_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USB0_NEEDCLK_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_NEEDCLK_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USB0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RTC_LITE0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_LITE0_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EZH_ARCH_B0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _EZH_ARCH_B0_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUP_MAILBOX0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP_MAILBOX0_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn sys_clr(&mut self) -> _SYS_CLRW {
        _SYS_CLRW { w: self }
    }
    #[doc = "Bit 1 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn sdma0_clr(&mut self) -> _SDMA0_CLRW {
        _SDMA0_CLRW { w: self }
    }
    #[doc = "Bit 2 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn gpio_globalint0_clr(&mut self) -> _GPIO_GLOBALINT0_CLRW {
        _GPIO_GLOBALINT0_CLRW { w: self }
    }
    #[doc = "Bit 3 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn gpio_globalint1_clr(&mut self) -> _GPIO_GLOBALINT1_CLRW {
        _GPIO_GLOBALINT1_CLRW { w: self }
    }
    #[doc = "Bit 4 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn gpio_int00_clr(&mut self) -> _GPIO_INT00_CLRW {
        _GPIO_INT00_CLRW { w: self }
    }
    #[doc = "Bit 5 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn gpio_int01_clr(&mut self) -> _GPIO_INT01_CLRW {
        _GPIO_INT01_CLRW { w: self }
    }
    #[doc = "Bit 6 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn gpio_int02_clr(&mut self) -> _GPIO_INT02_CLRW {
        _GPIO_INT02_CLRW { w: self }
    }
    #[doc = "Bit 7 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn gpio_int03_clr(&mut self) -> _GPIO_INT03_CLRW {
        _GPIO_INT03_CLRW { w: self }
    }
    #[doc = "Bit 8 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn utick0_clr(&mut self) -> _UTICK0_CLRW {
        _UTICK0_CLRW { w: self }
    }
    #[doc = "Bit 9 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn mrt0_clr(&mut self) -> _MRT0_CLRW {
        _MRT0_CLRW { w: self }
    }
    #[doc = "Bit 10 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn ctimer0_clr(&mut self) -> _CTIMER0_CLRW {
        _CTIMER0_CLRW { w: self }
    }
    #[doc = "Bit 11 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn ctimer1_clr(&mut self) -> _CTIMER1_CLRW {
        _CTIMER1_CLRW { w: self }
    }
    #[doc = "Bit 12 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn sct0_clr(&mut self) -> _SCT0_CLRW {
        _SCT0_CLRW { w: self }
    }
    #[doc = "Bit 13 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn ctimer3_clr(&mut self) -> _CTIMER3_CLRW {
        _CTIMER3_CLRW { w: self }
    }
    #[doc = "Bit 14 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn flexint0_clr(&mut self) -> _FLEXINT0_CLRW {
        _FLEXINT0_CLRW { w: self }
    }
    #[doc = "Bit 15 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn flexint1_clr(&mut self) -> _FLEXINT1_CLRW {
        _FLEXINT1_CLRW { w: self }
    }
    #[doc = "Bit 16 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn flexint2_clr(&mut self) -> _FLEXINT2_CLRW {
        _FLEXINT2_CLRW { w: self }
    }
    #[doc = "Bit 17 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn flexint3_clr(&mut self) -> _FLEXINT3_CLRW {
        _FLEXINT3_CLRW { w: self }
    }
    #[doc = "Bit 18 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn flexint4_clr(&mut self) -> _FLEXINT4_CLRW {
        _FLEXINT4_CLRW { w: self }
    }
    #[doc = "Bit 19 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn flexint5_clr(&mut self) -> _FLEXINT5_CLRW {
        _FLEXINT5_CLRW { w: self }
    }
    #[doc = "Bit 20 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn flexint6_clr(&mut self) -> _FLEXINT6_CLRW {
        _FLEXINT6_CLRW { w: self }
    }
    #[doc = "Bit 21 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn flexint7_clr(&mut self) -> _FLEXINT7_CLRW {
        _FLEXINT7_CLRW { w: self }
    }
    #[doc = "Bit 22 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn adc0_clr(&mut self) -> _ADC0_CLRW {
        _ADC0_CLRW { w: self }
    }
    #[doc = "Bit 24 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn adc0_thcmp_ovr_clr(&mut self) -> _ADC0_THCMP_OVR_CLRW {
        _ADC0_THCMP_OVR_CLRW { w: self }
    }
    #[doc = "Bit 27 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn usb0_needclk_clr(&mut self) -> _USB0_NEEDCLK_CLRW {
        _USB0_NEEDCLK_CLRW { w: self }
    }
    #[doc = "Bit 28 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn usb0_clr(&mut self) -> _USB0_CLRW {
        _USB0_CLRW { w: self }
    }
    #[doc = "Bit 29 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn rtc_lite0_clr(&mut self) -> _RTC_LITE0_CLRW {
        _RTC_LITE0_CLRW { w: self }
    }
    #[doc = "Bit 30 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn ezh_arch_b0_clr(&mut self) -> _EZH_ARCH_B0_CLRW {
        _EZH_ARCH_B0_CLRW { w: self }
    }
    #[doc = "Bit 31 - Writing ones to this register clears the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn wakeup_mailbox0_clr(&mut self) -> _WAKEUP_MAILBOX0_CLRW {
        _WAKEUP_MAILBOX0_CLRW { w: self }
    }
}
