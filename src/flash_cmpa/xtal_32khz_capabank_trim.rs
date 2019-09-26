#[doc = "Reader of register XTAL_32KHZ_CAPABANK_TRIM"]
pub type R = crate::R<u32, super::XTAL_32KHZ_CAPABANK_TRIM>;
#[doc = "Writer for register XTAL_32KHZ_CAPABANK_TRIM"]
pub type W = crate::W<u32, super::XTAL_32KHZ_CAPABANK_TRIM>;
#[doc = "Register XTAL_32KHZ_CAPABANK_TRIM `reset()`'s with value 0"]
impl crate::ResetValue for super::XTAL_32KHZ_CAPABANK_TRIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRIM_VALID`"]
pub type TRIM_VALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIM_VALID`"]
pub struct TRIM_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_VALID_W<'a> {
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
#[doc = "Reader of field `XTAL_LOAD_CAP_IEC_PF_X100`"]
pub type XTAL_LOAD_CAP_IEC_PF_X100_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `XTAL_LOAD_CAP_IEC_PF_X100`"]
pub struct XTAL_LOAD_CAP_IEC_PF_X100_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_LOAD_CAP_IEC_PF_X100_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 1)) | (((value as u32) & 0x03ff) << 1);
        self.w
    }
}
#[doc = "Reader of field `PCB_XIN_PARA_CAP_PF_X100`"]
pub type PCB_XIN_PARA_CAP_PF_X100_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PCB_XIN_PARA_CAP_PF_X100`"]
pub struct PCB_XIN_PARA_CAP_PF_X100_W<'a> {
    w: &'a mut W,
}
impl<'a> PCB_XIN_PARA_CAP_PF_X100_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 11)) | (((value as u32) & 0x03ff) << 11);
        self.w
    }
}
#[doc = "Reader of field `PCB_XOUT_PARA_CAP_PF_X100`"]
pub type PCB_XOUT_PARA_CAP_PF_X100_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PCB_XOUT_PARA_CAP_PF_X100`"]
pub struct PCB_XOUT_PARA_CAP_PF_X100_W<'a> {
    w: &'a mut W,
}
impl<'a> PCB_XOUT_PARA_CAP_PF_X100_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 21)) | (((value as u32) & 0x03ff) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0 : Capa Bank trimmings not valid. Default trimmings value are used. 1 : Capa Bank trimmings valid."]
    #[inline(always)]
    pub fn trim_valid(&self) -> TRIM_VALID_R {
        TRIM_VALID_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:10 - Load capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub fn xtal_load_cap_iec_pf_x100(&self) -> XTAL_LOAD_CAP_IEC_PF_X100_R {
        XTAL_LOAD_CAP_IEC_PF_X100_R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bits 11:20 - PCB XIN parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub fn pcb_xin_para_cap_pf_x100(&self) -> PCB_XIN_PARA_CAP_PF_X100_R {
        PCB_XIN_PARA_CAP_PF_X100_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 21:30 - PCB XOUT parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub fn pcb_xout_para_cap_pf_x100(&self) -> PCB_XOUT_PARA_CAP_PF_X100_R {
        PCB_XOUT_PARA_CAP_PF_X100_R::new(((self.bits >> 21) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0 : Capa Bank trimmings not valid. Default trimmings value are used. 1 : Capa Bank trimmings valid."]
    #[inline(always)]
    pub fn trim_valid(&mut self) -> TRIM_VALID_W {
        TRIM_VALID_W { w: self }
    }
    #[doc = "Bits 1:10 - Load capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub fn xtal_load_cap_iec_pf_x100(&mut self) -> XTAL_LOAD_CAP_IEC_PF_X100_W {
        XTAL_LOAD_CAP_IEC_PF_X100_W { w: self }
    }
    #[doc = "Bits 11:20 - PCB XIN parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub fn pcb_xin_para_cap_pf_x100(&mut self) -> PCB_XIN_PARA_CAP_PF_X100_W {
        PCB_XIN_PARA_CAP_PF_X100_W { w: self }
    }
    #[doc = "Bits 21:30 - PCB XOUT parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub fn pcb_xout_para_cap_pf_x100(&mut self) -> PCB_XOUT_PARA_CAP_PF_X100_W {
        PCB_XOUT_PARA_CAP_PF_X100_W { w: self }
    }
}
