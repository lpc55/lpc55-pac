#[doc = "Register `XTAL_16MHZ_CAPABANK_TRIM` reader"]
pub struct R(crate::R<XTAL_16MHZ_CAPABANK_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL_16MHZ_CAPABANK_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL_16MHZ_CAPABANK_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL_16MHZ_CAPABANK_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL_16MHZ_CAPABANK_TRIM` writer"]
pub struct W(crate::W<XTAL_16MHZ_CAPABANK_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL_16MHZ_CAPABANK_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<XTAL_16MHZ_CAPABANK_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL_16MHZ_CAPABANK_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIM_VALID` reader - 0 : Capa Bank trimmings not valid. Default trimmings value are used. 1 : Capa Bank trimmings valid."]
pub struct TRIM_VALID_R(crate::FieldReader<bool, bool>);
impl TRIM_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIM_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIM_VALID` writer - 0 : Capa Bank trimmings not valid. Default trimmings value are used. 1 : Capa Bank trimmings valid."]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `XTAL_LOAD_CAP_IEC_PF_X100` reader - Load capacitance, pF x 100. For example, 6pF becomes 600."]
pub struct XTAL_LOAD_CAP_IEC_PF_X100_R(crate::FieldReader<u16, u16>);
impl XTAL_LOAD_CAP_IEC_PF_X100_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        XTAL_LOAD_CAP_IEC_PF_X100_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL_LOAD_CAP_IEC_PF_X100_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL_LOAD_CAP_IEC_PF_X100` writer - Load capacitance, pF x 100. For example, 6pF becomes 600."]
pub struct XTAL_LOAD_CAP_IEC_PF_X100_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_LOAD_CAP_IEC_PF_X100_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 1)) | ((value as u32 & 0x03ff) << 1);
        self.w
    }
}
#[doc = "Field `PCB_XIN_PARA_CAP_PF_X100` reader - PCB XIN parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
pub struct PCB_XIN_PARA_CAP_PF_X100_R(crate::FieldReader<u16, u16>);
impl PCB_XIN_PARA_CAP_PF_X100_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PCB_XIN_PARA_CAP_PF_X100_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCB_XIN_PARA_CAP_PF_X100_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCB_XIN_PARA_CAP_PF_X100` writer - PCB XIN parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
pub struct PCB_XIN_PARA_CAP_PF_X100_W<'a> {
    w: &'a mut W,
}
impl<'a> PCB_XIN_PARA_CAP_PF_X100_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 11)) | ((value as u32 & 0x03ff) << 11);
        self.w
    }
}
#[doc = "Field `PCB_XOUT_PARA_CAP_PF_X100` reader - PCB XOUT parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
pub struct PCB_XOUT_PARA_CAP_PF_X100_R(crate::FieldReader<u16, u16>);
impl PCB_XOUT_PARA_CAP_PF_X100_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PCB_XOUT_PARA_CAP_PF_X100_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCB_XOUT_PARA_CAP_PF_X100_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCB_XOUT_PARA_CAP_PF_X100` writer - PCB XOUT parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
pub struct PCB_XOUT_PARA_CAP_PF_X100_W<'a> {
    w: &'a mut W,
}
impl<'a> PCB_XOUT_PARA_CAP_PF_X100_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 21)) | ((value as u32 & 0x03ff) << 21);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Xtal 16MHz capabank triming.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal_16mhz_capabank_trim](index.html) module"]
pub struct XTAL_16MHZ_CAPABANK_TRIM_SPEC;
impl crate::RegisterSpec for XTAL_16MHZ_CAPABANK_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal_16mhz_capabank_trim::R](R) reader structure"]
impl crate::Readable for XTAL_16MHZ_CAPABANK_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal_16mhz_capabank_trim::W](W) writer structure"]
impl crate::Writable for XTAL_16MHZ_CAPABANK_TRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTAL_16MHZ_CAPABANK_TRIM to value 0"]
impl crate::Resettable for XTAL_16MHZ_CAPABANK_TRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
