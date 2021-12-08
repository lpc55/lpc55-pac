#[doc = "Register `FRO192M_STATUS` reader"]
pub struct R(crate::R<FRO192M_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRO192M_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRO192M_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRO192M_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRO192M_STATUS` writer"]
pub struct W(crate::W<FRO192M_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRO192M_STATUS_SPEC>;
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
impl From<crate::W<FRO192M_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRO192M_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Output clock valid signal. Indicates that CCO clock has settled.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_VALID_A {
    #[doc = "0: No output clock present (None of 12 MHz, 48 MHz or 96 MHz clock is available)."]
    NOCLKOUT = 0,
    #[doc = "1: Clock is present (12 MHz, 48 MHz or 96 MHz can be output if they are enable respectively by FRO192M_CTRL.ENA_12MHZCLK/ENA_48MHZCLK/ENA_96MHZCLK)."]
    CLKOUT = 1,
}
impl From<CLK_VALID_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_VALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_VALID` reader - Output clock valid signal. Indicates that CCO clock has settled."]
pub struct CLK_VALID_R(crate::FieldReader<bool, CLK_VALID_A>);
impl CLK_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_VALID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_VALID_A {
        match self.bits {
            false => CLK_VALID_A::NOCLKOUT,
            true => CLK_VALID_A::CLKOUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOCLKOUT`"]
    #[inline(always)]
    pub fn is_noclkout(&self) -> bool {
        **self == CLK_VALID_A::NOCLKOUT
    }
    #[doc = "Checks if the value of the field is `CLKOUT`"]
    #[inline(always)]
    pub fn is_clkout(&self) -> bool {
        **self == CLK_VALID_A::CLKOUT
    }
}
impl core::ops::Deref for CLK_VALID_R {
    type Target = crate::FieldReader<bool, CLK_VALID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATB_VCTRL` reader - CCO threshold voltage detector output (signal vcco_ok). Once the CCO voltage crosses the threshold voltage of a SLVT transistor, this output signal will go high. It is also possible to observe the clk_valid signal."]
pub struct ATB_VCTRL_R(crate::FieldReader<bool, bool>);
impl ATB_VCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ATB_VCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATB_VCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Output clock valid signal. Indicates that CCO clock has settled."]
    #[inline(always)]
    pub fn clk_valid(&self) -> CLK_VALID_R {
        CLK_VALID_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CCO threshold voltage detector output (signal vcco_ok). Once the CCO voltage crosses the threshold voltage of a SLVT transistor, this output signal will go high. It is also possible to observe the clk_valid signal."]
    #[inline(always)]
    pub fn atb_vctrl(&self) -> ATB_VCTRL_R {
        ATB_VCTRL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "192MHz Free Running OScillator (FRO) Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fro192m_status](index.html) module"]
pub struct FRO192M_STATUS_SPEC;
impl crate::RegisterSpec for FRO192M_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fro192m_status::R](R) reader structure"]
impl crate::Readable for FRO192M_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fro192m_status::W](W) writer structure"]
impl crate::Writable for FRO192M_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRO192M_STATUS to value 0x03"]
impl crate::Resettable for FRO192M_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
