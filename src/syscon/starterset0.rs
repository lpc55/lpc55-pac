#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STARTERSET0 {
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
pub struct _SYS_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_SETW<'a> {
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
pub struct _SDMA0_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _SDMA0_SETW<'a> {
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
pub struct _GPIO_GLOBALINT0_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GLOBALINT0_SETW<'a> {
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
pub struct _GPIO_GLOBALINT1_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GLOBALINT1_SETW<'a> {
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
pub struct _GPIO_INT00_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT00_SETW<'a> {
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
pub struct _GPIO_INT01_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT01_SETW<'a> {
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
pub struct _GPIO_INT02_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT02_SETW<'a> {
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
pub struct _GPIO_INT03_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT03_SETW<'a> {
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
pub struct _UTICK0_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _UTICK0_SETW<'a> {
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
pub struct _MRT0_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _MRT0_SETW<'a> {
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
pub struct _CTIMER0_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER0_SETW<'a> {
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
pub struct _CTIMER1_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER1_SETW<'a> {
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
pub struct _SCT0_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _SCT0_SETW<'a> {
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
pub struct _CTIMER3_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER3_SETW<'a> {
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
pub struct _FLEXINT0_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT0_SETW<'a> {
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
pub struct _FLEXINT1_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT1_SETW<'a> {
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
pub struct _FLEXINT2_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT2_SETW<'a> {
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
pub struct _FLEXINT3_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT3_SETW<'a> {
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
pub struct _FLEXINT4_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT4_SETW<'a> {
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
pub struct _FLEXINT5_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT5_SETW<'a> {
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
pub struct _FLEXINT6_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT6_SETW<'a> {
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
pub struct _FLEXINT7_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXINT7_SETW<'a> {
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
pub struct _ADC0_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0_SETW<'a> {
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
pub struct _ADC0_THCMP_OVR_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0_THCMP_OVR_SETW<'a> {
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
pub struct _USB0_NEEDCLK_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_NEEDCLK_SETW<'a> {
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
pub struct _USB0_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_SETW<'a> {
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
pub struct _RTC_LITE0_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_LITE0_SETW<'a> {
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
pub struct _EZH_ARCH_B0_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _EZH_ARCH_B0_SETW<'a> {
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
pub struct _WAKEUP_MAILBOX0_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP_MAILBOX0_SETW<'a> {
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
    #[doc = "Bit 0 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn sys_set(&mut self) -> _SYS_SETW {
        _SYS_SETW { w: self }
    }
    #[doc = "Bit 1 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn sdma0_set(&mut self) -> _SDMA0_SETW {
        _SDMA0_SETW { w: self }
    }
    #[doc = "Bit 2 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn gpio_globalint0_set(&mut self) -> _GPIO_GLOBALINT0_SETW {
        _GPIO_GLOBALINT0_SETW { w: self }
    }
    #[doc = "Bit 3 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn gpio_globalint1_set(&mut self) -> _GPIO_GLOBALINT1_SETW {
        _GPIO_GLOBALINT1_SETW { w: self }
    }
    #[doc = "Bit 4 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn gpio_int00_set(&mut self) -> _GPIO_INT00_SETW {
        _GPIO_INT00_SETW { w: self }
    }
    #[doc = "Bit 5 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn gpio_int01_set(&mut self) -> _GPIO_INT01_SETW {
        _GPIO_INT01_SETW { w: self }
    }
    #[doc = "Bit 6 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn gpio_int02_set(&mut self) -> _GPIO_INT02_SETW {
        _GPIO_INT02_SETW { w: self }
    }
    #[doc = "Bit 7 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn gpio_int03_set(&mut self) -> _GPIO_INT03_SETW {
        _GPIO_INT03_SETW { w: self }
    }
    #[doc = "Bit 8 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn utick0_set(&mut self) -> _UTICK0_SETW {
        _UTICK0_SETW { w: self }
    }
    #[doc = "Bit 9 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn mrt0_set(&mut self) -> _MRT0_SETW {
        _MRT0_SETW { w: self }
    }
    #[doc = "Bit 10 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn ctimer0_set(&mut self) -> _CTIMER0_SETW {
        _CTIMER0_SETW { w: self }
    }
    #[doc = "Bit 11 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn ctimer1_set(&mut self) -> _CTIMER1_SETW {
        _CTIMER1_SETW { w: self }
    }
    #[doc = "Bit 12 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn sct0_set(&mut self) -> _SCT0_SETW {
        _SCT0_SETW { w: self }
    }
    #[doc = "Bit 13 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn ctimer3_set(&mut self) -> _CTIMER3_SETW {
        _CTIMER3_SETW { w: self }
    }
    #[doc = "Bit 14 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn flexint0_set(&mut self) -> _FLEXINT0_SETW {
        _FLEXINT0_SETW { w: self }
    }
    #[doc = "Bit 15 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn flexint1_set(&mut self) -> _FLEXINT1_SETW {
        _FLEXINT1_SETW { w: self }
    }
    #[doc = "Bit 16 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn flexint2_set(&mut self) -> _FLEXINT2_SETW {
        _FLEXINT2_SETW { w: self }
    }
    #[doc = "Bit 17 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn flexint3_set(&mut self) -> _FLEXINT3_SETW {
        _FLEXINT3_SETW { w: self }
    }
    #[doc = "Bit 18 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn flexint4_set(&mut self) -> _FLEXINT4_SETW {
        _FLEXINT4_SETW { w: self }
    }
    #[doc = "Bit 19 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn flexint5_set(&mut self) -> _FLEXINT5_SETW {
        _FLEXINT5_SETW { w: self }
    }
    #[doc = "Bit 20 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn flexint6_set(&mut self) -> _FLEXINT6_SETW {
        _FLEXINT6_SETW { w: self }
    }
    #[doc = "Bit 21 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn flexint7_set(&mut self) -> _FLEXINT7_SETW {
        _FLEXINT7_SETW { w: self }
    }
    #[doc = "Bit 22 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn adc0_set(&mut self) -> _ADC0_SETW {
        _ADC0_SETW { w: self }
    }
    #[doc = "Bit 24 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn adc0_thcmp_ovr_set(&mut self) -> _ADC0_THCMP_OVR_SETW {
        _ADC0_THCMP_OVR_SETW { w: self }
    }
    #[doc = "Bit 27 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn usb0_needclk_set(&mut self) -> _USB0_NEEDCLK_SETW {
        _USB0_NEEDCLK_SETW { w: self }
    }
    #[doc = "Bit 28 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn usb0_set(&mut self) -> _USB0_SETW {
        _USB0_SETW { w: self }
    }
    #[doc = "Bit 29 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn rtc_lite0_set(&mut self) -> _RTC_LITE0_SETW {
        _RTC_LITE0_SETW { w: self }
    }
    #[doc = "Bit 30 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn ezh_arch_b0_set(&mut self) -> _EZH_ARCH_B0_SETW {
        _EZH_ARCH_B0_SETW { w: self }
    }
    #[doc = "Bit 31 - Writing ones to this register sets the corresponding bit in the STARTER0 register."]
    #[inline]
    pub fn wakeup_mailbox0_set(&mut self) -> _WAKEUP_MAILBOX0_SETW {
        _WAKEUP_MAILBOX0_SETW { w: self }
    }
}
