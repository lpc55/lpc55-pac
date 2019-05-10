#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STARTERSET1 {
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
pub struct _GPIO_INT04_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT04_SETW<'a> {
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
pub struct _GPIO_INT05_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT05_SETW<'a> {
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
pub struct _GPIO_INT06_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT06_SETW<'a> {
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
pub struct _GPIO_INT07_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT07_SETW<'a> {
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
pub struct _CTIMER2_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER2_SETW<'a> {
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
pub struct _CTIMER4_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER4_SETW<'a> {
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
pub struct _OS_EVENT_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _OS_EVENT_SETW<'a> {
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
pub struct _SDIO_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _SDIO_SETW<'a> {
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
pub struct _USB1_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_SETW<'a> {
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
pub struct _USB1_NEEDCLK_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_NEEDCLK_SETW<'a> {
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
pub struct _SEC_HYPERVISOR_CALL_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_HYPERVISOR_CALL_SETW<'a> {
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
pub struct _SEC_GPIO_INT00_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_GPIO_INT00_SETW<'a> {
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
pub struct _SEC_GPIO_INT01_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_GPIO_INT01_SETW<'a> {
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
pub struct _PLU_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _PLU_SETW<'a> {
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
pub struct _SEC_VIO_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_VIO_SETW<'a> {
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
pub struct _SHA_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _SHA_SETW<'a> {
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
pub struct _CASER_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _CASER_SETW<'a> {
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
pub struct _QDDKEY_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _QDDKEY_SETW<'a> {
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
pub struct _PQ_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _PQ_SETW<'a> {
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
pub struct _SDMA1_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _SDMA1_SETW<'a> {
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
pub struct _LSPI_HS_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _LSPI_HS_SETW<'a> {
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
pub struct _WAKEUPPADS_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUPPADS_SETW<'a> {
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
    #[doc = "Bit 0 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn gpio_int04_set(&mut self) -> _GPIO_INT04_SETW {
        _GPIO_INT04_SETW { w: self }
    }
    #[doc = "Bit 1 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn gpio_int05_set(&mut self) -> _GPIO_INT05_SETW {
        _GPIO_INT05_SETW { w: self }
    }
    #[doc = "Bit 2 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn gpio_int06_set(&mut self) -> _GPIO_INT06_SETW {
        _GPIO_INT06_SETW { w: self }
    }
    #[doc = "Bit 3 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn gpio_int07_set(&mut self) -> _GPIO_INT07_SETW {
        _GPIO_INT07_SETW { w: self }
    }
    #[doc = "Bit 4 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn ctimer2_set(&mut self) -> _CTIMER2_SETW {
        _CTIMER2_SETW { w: self }
    }
    #[doc = "Bit 5 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn ctimer4_set(&mut self) -> _CTIMER4_SETW {
        _CTIMER4_SETW { w: self }
    }
    #[doc = "Bit 6 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn os_event_set(&mut self) -> _OS_EVENT_SETW {
        _OS_EVENT_SETW { w: self }
    }
    #[doc = "Bit 10 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn sdio_set(&mut self) -> _SDIO_SETW {
        _SDIO_SETW { w: self }
    }
    #[doc = "Bit 15 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn usb1_set(&mut self) -> _USB1_SETW {
        _USB1_SETW { w: self }
    }
    #[doc = "Bit 16 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn usb1_needclk_set(&mut self) -> _USB1_NEEDCLK_SETW {
        _USB1_NEEDCLK_SETW { w: self }
    }
    #[doc = "Bit 17 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn sec_hypervisor_call_set(&mut self) -> _SEC_HYPERVISOR_CALL_SETW {
        _SEC_HYPERVISOR_CALL_SETW { w: self }
    }
    #[doc = "Bit 18 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn sec_gpio_int00_set(&mut self) -> _SEC_GPIO_INT00_SETW {
        _SEC_GPIO_INT00_SETW { w: self }
    }
    #[doc = "Bit 19 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn sec_gpio_int01_set(&mut self) -> _SEC_GPIO_INT01_SETW {
        _SEC_GPIO_INT01_SETW { w: self }
    }
    #[doc = "Bit 20 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn plu_set(&mut self) -> _PLU_SETW {
        _PLU_SETW { w: self }
    }
    #[doc = "Bit 21 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn sec_vio_set(&mut self) -> _SEC_VIO_SETW {
        _SEC_VIO_SETW { w: self }
    }
    #[doc = "Bit 22 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn sha_set(&mut self) -> _SHA_SETW {
        _SHA_SETW { w: self }
    }
    #[doc = "Bit 23 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn caser_set(&mut self) -> _CASER_SETW {
        _CASER_SETW { w: self }
    }
    #[doc = "Bit 24 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn qddkey_set(&mut self) -> _QDDKEY_SETW {
        _QDDKEY_SETW { w: self }
    }
    #[doc = "Bit 25 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn pq_set(&mut self) -> _PQ_SETW {
        _PQ_SETW { w: self }
    }
    #[doc = "Bit 26 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn sdma1_set(&mut self) -> _SDMA1_SETW {
        _SDMA1_SETW { w: self }
    }
    #[doc = "Bit 27 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn lspi_hs_set(&mut self) -> _LSPI_HS_SETW {
        _LSPI_HS_SETW { w: self }
    }
    #[doc = "Bit 31 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline]
    pub fn wakeuppads_set(&mut self) -> _WAKEUPPADS_SETW {
        _WAKEUPPADS_SETW { w: self }
    }
}
