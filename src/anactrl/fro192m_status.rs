#[doc = "Reader of register FRO192M_STATUS"]
pub type R = crate::R<u32, super::FRO192M_STATUS>;
#[doc = "Writer for register FRO192M_STATUS"]
pub type W = crate::W<u32, super::FRO192M_STATUS>;
#[doc = "Register FRO192M_STATUS `reset()`'s with value 0x03"]
impl crate::ResetValue for super::FRO192M_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Possible values of the field `CLK_VALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_VALID_A {
    #[doc = "No output clock present (None of 12 MHz, 48 MHz or 96 MHz clock is available)."]
    NOCLKOUT,
    #[doc = "Clock is present (12 MHz, 48 MHz or 96 MHz can be output if they are enable respectively by FRO192M_CTRL.ENA_12MHZCLK/ENA_48MHZCLK/ENA_96MHZCLK)."]
    CLKOUT,
}
impl From<CLK_VALID_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_VALID_A) -> Self {
        match variant {
            CLK_VALID_A::NOCLKOUT => false,
            CLK_VALID_A::CLKOUT => true,
        }
    }
}
#[doc = "Reader of field `CLK_VALID`"]
pub type CLK_VALID_R = crate::R<bool, CLK_VALID_A>;
impl CLK_VALID_R {
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
        *self == CLK_VALID_A::NOCLKOUT
    }
    #[doc = "Checks if the value of the field is `CLKOUT`"]
    #[inline(always)]
    pub fn is_clkout(&self) -> bool {
        *self == CLK_VALID_A::CLKOUT
    }
}
#[doc = "Reader of field `ATB_VCTRL`"]
pub type ATB_VCTRL_R = crate::R<bool, bool>;
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
impl W {}
