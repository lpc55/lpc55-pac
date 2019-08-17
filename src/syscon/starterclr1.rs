#[doc = "Writer for register STARTERCLR1"]
pub type W = crate::W<u32, super::STARTERCLR1>;
#[doc = "Register STARTERCLR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTERCLR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `GPIO_INT04_CLR`"]
pub struct GPIO_INT04_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT04_CLR_W<'a> {
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
#[doc = "Write proxy for field `GPIO_INT05_CLR`"]
pub struct GPIO_INT05_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT05_CLR_W<'a> {
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
#[doc = "Write proxy for field `GPIO_INT06_CLR`"]
pub struct GPIO_INT06_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT06_CLR_W<'a> {
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
#[doc = "Write proxy for field `GPIO_INT07_CLR`"]
pub struct GPIO_INT07_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT07_CLR_W<'a> {
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
#[doc = "Write proxy for field `CTIMER2_CLR`"]
pub struct CTIMER2_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER2_CLR_W<'a> {
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
#[doc = "Write proxy for field `CTIMER4_CLR`"]
pub struct CTIMER4_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER4_CLR_W<'a> {
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
#[doc = "Write proxy for field `OS_EVENT_CLR`"]
pub struct OS_EVENT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OS_EVENT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SDIO_CLR`"]
pub struct SDIO_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_CLR_W<'a> {
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
#[doc = "Write proxy for field `USB1_CLR`"]
pub struct USB1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_CLR_W<'a> {
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
#[doc = "Write proxy for field `USB1_NEEDCLK_CLR`"]
pub struct USB1_NEEDCLK_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_NEEDCLK_CLR_W<'a> {
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
#[doc = "Write proxy for field `SEC_HYPERVISOR_CALL_CLR`"]
pub struct SEC_HYPERVISOR_CALL_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_HYPERVISOR_CALL_CLR_W<'a> {
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
#[doc = "Write proxy for field `SEC_GPIO_INT00_CLR`"]
pub struct SEC_GPIO_INT00_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_GPIO_INT00_CLR_W<'a> {
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
#[doc = "Write proxy for field `SEC_GPIO_INT01_CLR`"]
pub struct SEC_GPIO_INT01_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_GPIO_INT01_CLR_W<'a> {
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
#[doc = "Write proxy for field `PLU_CLR`"]
pub struct PLU_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLU_CLR_W<'a> {
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
#[doc = "Write proxy for field `SEC_VIO_CLR`"]
pub struct SEC_VIO_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_VIO_CLR_W<'a> {
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
#[doc = "Write proxy for field `SHA_CLR`"]
pub struct SHA_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA_CLR_W<'a> {
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
#[doc = "Write proxy for field `CASER_CLR`"]
pub struct CASER_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CASER_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Write proxy for field `QDDKEY_CLR`"]
pub struct QDDKEY_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> QDDKEY_CLR_W<'a> {
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
#[doc = "Write proxy for field `PQ_CLR`"]
pub struct PQ_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PQ_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Write proxy for field `SDMA1_CLR`"]
pub struct SDMA1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA1_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Write proxy for field `LSPI_HS_CLR`"]
pub struct LSPI_HS_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LSPI_HS_CLR_W<'a> {
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
#[doc = "Write proxy for field `WAKEUPPADS_CLR`"]
pub struct WAKEUPPADS_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPPADS_CLR_W<'a> {
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
    #[doc = "Bit 0 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn gpio_int04_clr(&mut self) -> GPIO_INT04_CLR_W {
        GPIO_INT04_CLR_W { w: self }
    }
    #[doc = "Bit 1 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn gpio_int05_clr(&mut self) -> GPIO_INT05_CLR_W {
        GPIO_INT05_CLR_W { w: self }
    }
    #[doc = "Bit 2 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn gpio_int06_clr(&mut self) -> GPIO_INT06_CLR_W {
        GPIO_INT06_CLR_W { w: self }
    }
    #[doc = "Bit 3 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn gpio_int07_clr(&mut self) -> GPIO_INT07_CLR_W {
        GPIO_INT07_CLR_W { w: self }
    }
    #[doc = "Bit 4 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn ctimer2_clr(&mut self) -> CTIMER2_CLR_W {
        CTIMER2_CLR_W { w: self }
    }
    #[doc = "Bit 5 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn ctimer4_clr(&mut self) -> CTIMER4_CLR_W {
        CTIMER4_CLR_W { w: self }
    }
    #[doc = "Bit 6 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn os_event_clr(&mut self) -> OS_EVENT_CLR_W {
        OS_EVENT_CLR_W { w: self }
    }
    #[doc = "Bit 10 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn sdio_clr(&mut self) -> SDIO_CLR_W {
        SDIO_CLR_W { w: self }
    }
    #[doc = "Bit 15 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn usb1_clr(&mut self) -> USB1_CLR_W {
        USB1_CLR_W { w: self }
    }
    #[doc = "Bit 16 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn usb1_needclk_clr(&mut self) -> USB1_NEEDCLK_CLR_W {
        USB1_NEEDCLK_CLR_W { w: self }
    }
    #[doc = "Bit 17 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn sec_hypervisor_call_clr(&mut self) -> SEC_HYPERVISOR_CALL_CLR_W {
        SEC_HYPERVISOR_CALL_CLR_W { w: self }
    }
    #[doc = "Bit 18 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn sec_gpio_int00_clr(&mut self) -> SEC_GPIO_INT00_CLR_W {
        SEC_GPIO_INT00_CLR_W { w: self }
    }
    #[doc = "Bit 19 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn sec_gpio_int01_clr(&mut self) -> SEC_GPIO_INT01_CLR_W {
        SEC_GPIO_INT01_CLR_W { w: self }
    }
    #[doc = "Bit 20 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn plu_clr(&mut self) -> PLU_CLR_W {
        PLU_CLR_W { w: self }
    }
    #[doc = "Bit 21 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn sec_vio_clr(&mut self) -> SEC_VIO_CLR_W {
        SEC_VIO_CLR_W { w: self }
    }
    #[doc = "Bit 22 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn sha_clr(&mut self) -> SHA_CLR_W {
        SHA_CLR_W { w: self }
    }
    #[doc = "Bit 23 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn caser_clr(&mut self) -> CASER_CLR_W {
        CASER_CLR_W { w: self }
    }
    #[doc = "Bit 24 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn qddkey_clr(&mut self) -> QDDKEY_CLR_W {
        QDDKEY_CLR_W { w: self }
    }
    #[doc = "Bit 25 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn pq_clr(&mut self) -> PQ_CLR_W {
        PQ_CLR_W { w: self }
    }
    #[doc = "Bit 26 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn sdma1_clr(&mut self) -> SDMA1_CLR_W {
        SDMA1_CLR_W { w: self }
    }
    #[doc = "Bit 27 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn lspi_hs_clr(&mut self) -> LSPI_HS_CLR_W {
        LSPI_HS_CLR_W { w: self }
    }
    #[doc = "Bit 31 - Writing ones to this register clears the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn wakeuppads_clr(&mut self) -> WAKEUPPADS_CLR_W {
        WAKEUPPADS_CLR_W { w: self }
    }
}
