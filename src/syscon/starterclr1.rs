#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STARTERCLR1 {
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
pub struct _GPIO_INT04_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT04_CLRW<'a> {
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
pub struct _GPIO_INT05_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT05_CLRW<'a> {
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
pub struct _GPIO_INT06_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT06_CLRW<'a> {
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
pub struct _GPIO_INT07_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT07_CLRW<'a> {
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
pub struct _CTIMER2_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER2_CLRW<'a> {
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
pub struct _CTIMER4_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER4_CLRW<'a> {
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
pub struct _OS_EVENT_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _OS_EVENT_CLRW<'a> {
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
pub struct _SDIO_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _SDIO_CLRW<'a> {
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
pub struct _USB1_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_CLRW<'a> {
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
pub struct _USB1_NEEDCLK_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_NEEDCLK_CLRW<'a> {
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
pub struct _SEC_HYPERVISOR_CALL_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_HYPERVISOR_CALL_CLRW<'a> {
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
pub struct _SEC_GPIO_INT00_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_GPIO_INT00_CLRW<'a> {
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
pub struct _SEC_GPIO_INT01_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_GPIO_INT01_CLRW<'a> {
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
pub struct _PLU_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _PLU_CLRW<'a> {
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
pub struct _SEC_VIO_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_VIO_CLRW<'a> {
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
pub struct _SHA_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _SHA_CLRW<'a> {
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
pub struct _CASER_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CASER_CLRW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _QDDKEY_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _QDDKEY_CLRW<'a> {
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
pub struct _PQ_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _PQ_CLRW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SDMA1_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _SDMA1_CLRW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LSPI_HS_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _LSPI_HS_CLRW<'a> {
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
pub struct _WAKEUPPADS_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUPPADS_CLRW<'a> {
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
    #[doc = "Bit 0 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn gpio_int04_clr(&mut self) -> _GPIO_INT04_CLRW {
        _GPIO_INT04_CLRW { w: self }
    }
    #[doc = "Bit 1 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn gpio_int05_clr(&mut self) -> _GPIO_INT05_CLRW {
        _GPIO_INT05_CLRW { w: self }
    }
    #[doc = "Bit 2 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn gpio_int06_clr(&mut self) -> _GPIO_INT06_CLRW {
        _GPIO_INT06_CLRW { w: self }
    }
    #[doc = "Bit 3 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn gpio_int07_clr(&mut self) -> _GPIO_INT07_CLRW {
        _GPIO_INT07_CLRW { w: self }
    }
    #[doc = "Bit 4 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn ctimer2_clr(&mut self) -> _CTIMER2_CLRW {
        _CTIMER2_CLRW { w: self }
    }
    #[doc = "Bit 5 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn ctimer4_clr(&mut self) -> _CTIMER4_CLRW {
        _CTIMER4_CLRW { w: self }
    }
    #[doc = "Bit 6 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn os_event_clr(&mut self) -> _OS_EVENT_CLRW {
        _OS_EVENT_CLRW { w: self }
    }
    #[doc = "Bit 10 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn sdio_clr(&mut self) -> _SDIO_CLRW {
        _SDIO_CLRW { w: self }
    }
    #[doc = "Bit 15 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn usb1_clr(&mut self) -> _USB1_CLRW {
        _USB1_CLRW { w: self }
    }
    #[doc = "Bit 16 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn usb1_needclk_clr(&mut self) -> _USB1_NEEDCLK_CLRW {
        _USB1_NEEDCLK_CLRW { w: self }
    }
    #[doc = "Bit 17 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn sec_hypervisor_call_clr(&mut self) -> _SEC_HYPERVISOR_CALL_CLRW {
        _SEC_HYPERVISOR_CALL_CLRW { w: self }
    }
    #[doc = "Bit 18 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn sec_gpio_int00_clr(&mut self) -> _SEC_GPIO_INT00_CLRW {
        _SEC_GPIO_INT00_CLRW { w: self }
    }
    #[doc = "Bit 19 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn sec_gpio_int01_clr(&mut self) -> _SEC_GPIO_INT01_CLRW {
        _SEC_GPIO_INT01_CLRW { w: self }
    }
    #[doc = "Bit 20 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn plu_clr(&mut self) -> _PLU_CLRW {
        _PLU_CLRW { w: self }
    }
    #[doc = "Bit 21 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn sec_vio_clr(&mut self) -> _SEC_VIO_CLRW {
        _SEC_VIO_CLRW { w: self }
    }
    #[doc = "Bit 22 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn sha_clr(&mut self) -> _SHA_CLRW {
        _SHA_CLRW { w: self }
    }
    #[doc = "Bit 23 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn caser_clr(&mut self) -> _CASER_CLRW {
        _CASER_CLRW { w: self }
    }
    #[doc = "Bit 24 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn qddkey_clr(&mut self) -> _QDDKEY_CLRW {
        _QDDKEY_CLRW { w: self }
    }
    #[doc = "Bit 25 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn pq_clr(&mut self) -> _PQ_CLRW {
        _PQ_CLRW { w: self }
    }
    #[doc = "Bit 26 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn sdma1_clr(&mut self) -> _SDMA1_CLRW {
        _SDMA1_CLRW { w: self }
    }
    #[doc = "Bit 27 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn lspi_hs_clr(&mut self) -> _LSPI_HS_CLRW {
        _LSPI_HS_CLRW { w: self }
    }
    #[doc = "Bit 31 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn wakeuppads_clr(&mut self) -> _WAKEUPPADS_CLRW {
        _WAKEUPPADS_CLRW { w: self }
    }
}
