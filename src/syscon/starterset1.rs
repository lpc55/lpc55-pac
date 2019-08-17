#[doc = "Writer for register STARTERSET1"]
pub type W = crate::W<u32, super::STARTERSET1>;
#[doc = "Register STARTERSET1 `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTERSET1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `GPIO_INT04_SET`"]
pub struct GPIO_INT04_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT04_SET_W<'a> {
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
#[doc = "Write proxy for field `GPIO_INT05_SET`"]
pub struct GPIO_INT05_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT05_SET_W<'a> {
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
#[doc = "Write proxy for field `GPIO_INT06_SET`"]
pub struct GPIO_INT06_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT06_SET_W<'a> {
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
#[doc = "Write proxy for field `GPIO_INT07_SET`"]
pub struct GPIO_INT07_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT07_SET_W<'a> {
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
#[doc = "Write proxy for field `CTIMER2_SET`"]
pub struct CTIMER2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER2_SET_W<'a> {
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
#[doc = "Write proxy for field `CTIMER4_SET`"]
pub struct CTIMER4_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER4_SET_W<'a> {
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
#[doc = "Write proxy for field `OS_EVENT_SET`"]
pub struct OS_EVENT_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> OS_EVENT_SET_W<'a> {
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
#[doc = "Write proxy for field `SDIO_SET`"]
pub struct SDIO_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_SET_W<'a> {
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
#[doc = "Write proxy for field `USB1_SET`"]
pub struct USB1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_SET_W<'a> {
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
#[doc = "Write proxy for field `USB1_NEEDCLK_SET`"]
pub struct USB1_NEEDCLK_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_NEEDCLK_SET_W<'a> {
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
#[doc = "Write proxy for field `SEC_HYPERVISOR_CALL_SET`"]
pub struct SEC_HYPERVISOR_CALL_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_HYPERVISOR_CALL_SET_W<'a> {
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
#[doc = "Write proxy for field `SEC_GPIO_INT00_SET`"]
pub struct SEC_GPIO_INT00_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_GPIO_INT00_SET_W<'a> {
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
#[doc = "Write proxy for field `SEC_GPIO_INT01_SET`"]
pub struct SEC_GPIO_INT01_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_GPIO_INT01_SET_W<'a> {
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
#[doc = "Write proxy for field `PLU_SET`"]
pub struct PLU_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> PLU_SET_W<'a> {
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
#[doc = "Write proxy for field `SEC_VIO_SET`"]
pub struct SEC_VIO_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_VIO_SET_W<'a> {
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
#[doc = "Write proxy for field `SHA_SET`"]
pub struct SHA_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA_SET_W<'a> {
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
#[doc = "Write proxy for field `CASER_SET`"]
pub struct CASER_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CASER_SET_W<'a> {
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
#[doc = "Write proxy for field `QDDKEY_SET`"]
pub struct QDDKEY_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> QDDKEY_SET_W<'a> {
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
#[doc = "Write proxy for field `PQ_SET`"]
pub struct PQ_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> PQ_SET_W<'a> {
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
#[doc = "Write proxy for field `SDMA1_SET`"]
pub struct SDMA1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA1_SET_W<'a> {
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
#[doc = "Write proxy for field `LSPI_HS_SET`"]
pub struct LSPI_HS_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> LSPI_HS_SET_W<'a> {
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
#[doc = "Write proxy for field `WAKEUPPADS_SET`"]
pub struct WAKEUPPADS_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPPADS_SET_W<'a> {
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
    #[doc = "Bit 0 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn gpio_int04_set(&mut self) -> GPIO_INT04_SET_W {
        GPIO_INT04_SET_W { w: self }
    }
    #[doc = "Bit 1 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn gpio_int05_set(&mut self) -> GPIO_INT05_SET_W {
        GPIO_INT05_SET_W { w: self }
    }
    #[doc = "Bit 2 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn gpio_int06_set(&mut self) -> GPIO_INT06_SET_W {
        GPIO_INT06_SET_W { w: self }
    }
    #[doc = "Bit 3 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn gpio_int07_set(&mut self) -> GPIO_INT07_SET_W {
        GPIO_INT07_SET_W { w: self }
    }
    #[doc = "Bit 4 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn ctimer2_set(&mut self) -> CTIMER2_SET_W {
        CTIMER2_SET_W { w: self }
    }
    #[doc = "Bit 5 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn ctimer4_set(&mut self) -> CTIMER4_SET_W {
        CTIMER4_SET_W { w: self }
    }
    #[doc = "Bit 6 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn os_event_set(&mut self) -> OS_EVENT_SET_W {
        OS_EVENT_SET_W { w: self }
    }
    #[doc = "Bit 10 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn sdio_set(&mut self) -> SDIO_SET_W {
        SDIO_SET_W { w: self }
    }
    #[doc = "Bit 15 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn usb1_set(&mut self) -> USB1_SET_W {
        USB1_SET_W { w: self }
    }
    #[doc = "Bit 16 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn usb1_needclk_set(&mut self) -> USB1_NEEDCLK_SET_W {
        USB1_NEEDCLK_SET_W { w: self }
    }
    #[doc = "Bit 17 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn sec_hypervisor_call_set(&mut self) -> SEC_HYPERVISOR_CALL_SET_W {
        SEC_HYPERVISOR_CALL_SET_W { w: self }
    }
    #[doc = "Bit 18 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn sec_gpio_int00_set(&mut self) -> SEC_GPIO_INT00_SET_W {
        SEC_GPIO_INT00_SET_W { w: self }
    }
    #[doc = "Bit 19 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn sec_gpio_int01_set(&mut self) -> SEC_GPIO_INT01_SET_W {
        SEC_GPIO_INT01_SET_W { w: self }
    }
    #[doc = "Bit 20 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn plu_set(&mut self) -> PLU_SET_W {
        PLU_SET_W { w: self }
    }
    #[doc = "Bit 21 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn sec_vio_set(&mut self) -> SEC_VIO_SET_W {
        SEC_VIO_SET_W { w: self }
    }
    #[doc = "Bit 22 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn sha_set(&mut self) -> SHA_SET_W {
        SHA_SET_W { w: self }
    }
    #[doc = "Bit 23 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn caser_set(&mut self) -> CASER_SET_W {
        CASER_SET_W { w: self }
    }
    #[doc = "Bit 24 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn qddkey_set(&mut self) -> QDDKEY_SET_W {
        QDDKEY_SET_W { w: self }
    }
    #[doc = "Bit 25 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn pq_set(&mut self) -> PQ_SET_W {
        PQ_SET_W { w: self }
    }
    #[doc = "Bit 26 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn sdma1_set(&mut self) -> SDMA1_SET_W {
        SDMA1_SET_W { w: self }
    }
    #[doc = "Bit 27 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn lspi_hs_set(&mut self) -> LSPI_HS_SET_W {
        LSPI_HS_SET_W { w: self }
    }
    #[doc = "Bit 31 - Writing ones to this register sets the corresponding bit in the STARTER1 register."]
    #[inline(always)]
    pub fn wakeuppads_set(&mut self) -> WAKEUPPADS_SET_W {
        WAKEUPPADS_SET_W { w: self }
    }
}
