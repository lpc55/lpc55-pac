#[doc = "Register `WAKEINT_CTRL` reader"]
pub struct R(crate::R<WAKEINT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKEINT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WAKEINT_CTRL_SPEC>> for R {
    fn from(reader: crate::R<WAKEINT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKEINT_CTRL` writer"]
pub struct W(crate::W<WAKEINT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKEINT_CTRL_SPEC>;
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
impl core::convert::From<crate::W<WAKEINT_CTRL_SPEC>> for W {
    fn from(writer: crate::W<WAKEINT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK` reader - Interrupt mask (which of the 8 PLU Outputs contribute to interrupt)"]
pub struct MASK_R(crate::FieldReader<u8, u8>);
impl MASK_R {
    pub(crate) fn new(bits: u8) -> Self {
        MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK` writer - Interrupt mask (which of the 8 PLU Outputs contribute to interrupt)"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "control input of the PLU, add filtering for glitch.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FILTER_MODE_A {
    #[doc = "0: Bypass mode."]
    BYPASS = 0,
    #[doc = "1: Filter 1 clock period."]
    FILTER1CLK = 1,
    #[doc = "2: Filter 2 clock period."]
    FILTER2CLK = 2,
    #[doc = "3: Filter 3 clock period."]
    FILTER3CLK = 3,
}
impl From<FILTER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTER_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FILTER_MODE` reader - control input of the PLU, add filtering for glitch."]
pub struct FILTER_MODE_R(crate::FieldReader<u8, FILTER_MODE_A>);
impl FILTER_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FILTER_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTER_MODE_A {
        match self.bits {
            0 => FILTER_MODE_A::BYPASS,
            1 => FILTER_MODE_A::FILTER1CLK,
            2 => FILTER_MODE_A::FILTER2CLK,
            3 => FILTER_MODE_A::FILTER3CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        **self == FILTER_MODE_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `FILTER1CLK`"]
    #[inline(always)]
    pub fn is_filter1clk(&self) -> bool {
        **self == FILTER_MODE_A::FILTER1CLK
    }
    #[doc = "Checks if the value of the field is `FILTER2CLK`"]
    #[inline(always)]
    pub fn is_filter2clk(&self) -> bool {
        **self == FILTER_MODE_A::FILTER2CLK
    }
    #[doc = "Checks if the value of the field is `FILTER3CLK`"]
    #[inline(always)]
    pub fn is_filter3clk(&self) -> bool {
        **self == FILTER_MODE_A::FILTER3CLK
    }
}
impl core::ops::Deref for FILTER_MODE_R {
    type Target = crate::FieldReader<u8, FILTER_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTER_MODE` writer - control input of the PLU, add filtering for glitch."]
pub struct FILTER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTER_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Bypass mode."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(FILTER_MODE_A::BYPASS)
    }
    #[doc = "Filter 1 clock period."]
    #[inline(always)]
    pub fn filter1clk(self) -> &'a mut W {
        self.variant(FILTER_MODE_A::FILTER1CLK)
    }
    #[doc = "Filter 2 clock period."]
    #[inline(always)]
    pub fn filter2clk(self) -> &'a mut W {
        self.variant(FILTER_MODE_A::FILTER2CLK)
    }
    #[doc = "Filter 3 clock period."]
    #[inline(always)]
    pub fn filter3clk(self) -> &'a mut W {
        self.variant(FILTER_MODE_A::FILTER3CLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "hclk is divided by 2**filter_clksel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FILTER_CLKSEL_A {
    #[doc = "0: Selects the 1 MHz low-power oscillator as the filter clock."]
    FRO1MHZ = 0,
    #[doc = "1: Selects the 12 Mhz FRO as the filter clock."]
    FRO12MHZ = 1,
    #[doc = "2: Selects a third filter clock source, if provided."]
    OTHER_CLOCK = 2,
}
impl From<FILTER_CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTER_CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FILTER_CLKSEL` reader - hclk is divided by 2**filter_clksel."]
pub struct FILTER_CLKSEL_R(crate::FieldReader<u8, FILTER_CLKSEL_A>);
impl FILTER_CLKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FILTER_CLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FILTER_CLKSEL_A> {
        match self.bits {
            0 => Some(FILTER_CLKSEL_A::FRO1MHZ),
            1 => Some(FILTER_CLKSEL_A::FRO12MHZ),
            2 => Some(FILTER_CLKSEL_A::OTHER_CLOCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FRO1MHZ`"]
    #[inline(always)]
    pub fn is_fro1mhz(&self) -> bool {
        **self == FILTER_CLKSEL_A::FRO1MHZ
    }
    #[doc = "Checks if the value of the field is `FRO12MHZ`"]
    #[inline(always)]
    pub fn is_fro12mhz(&self) -> bool {
        **self == FILTER_CLKSEL_A::FRO12MHZ
    }
    #[doc = "Checks if the value of the field is `OTHER_CLOCK`"]
    #[inline(always)]
    pub fn is_other_clock(&self) -> bool {
        **self == FILTER_CLKSEL_A::OTHER_CLOCK
    }
}
impl core::ops::Deref for FILTER_CLKSEL_R {
    type Target = crate::FieldReader<u8, FILTER_CLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTER_CLKSEL` writer - hclk is divided by 2**filter_clksel."]
pub struct FILTER_CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTER_CLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selects the 1 MHz low-power oscillator as the filter clock."]
    #[inline(always)]
    pub fn fro1mhz(self) -> &'a mut W {
        self.variant(FILTER_CLKSEL_A::FRO1MHZ)
    }
    #[doc = "Selects the 12 Mhz FRO as the filter clock."]
    #[inline(always)]
    pub fn fro12mhz(self) -> &'a mut W {
        self.variant(FILTER_CLKSEL_A::FRO12MHZ)
    }
    #[doc = "Selects a third filter clock source, if provided."]
    #[inline(always)]
    pub fn other_clock(self) -> &'a mut W {
        self.variant(FILTER_CLKSEL_A::OTHER_CLOCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `LATCH_ENABLE` reader - latch the interrupt , then can be cleared with next bit INTR_CLEAR"]
pub struct LATCH_ENABLE_R(crate::FieldReader<bool, bool>);
impl LATCH_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATCH_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATCH_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATCH_ENABLE` writer - latch the interrupt , then can be cleared with next bit INTR_CLEAR"]
pub struct LATCH_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LATCH_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `INTR_CLEAR` reader - Write to clear wakeint_latched"]
pub struct INTR_CLEAR_R(crate::FieldReader<bool, bool>);
impl INTR_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTR_CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTR_CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTR_CLEAR` writer - Write to clear wakeint_latched"]
pub struct INTR_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_CLEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt mask (which of the 8 PLU Outputs contribute to interrupt)"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - control input of the PLU, add filtering for glitch."]
    #[inline(always)]
    pub fn filter_mode(&self) -> FILTER_MODE_R {
        FILTER_MODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - hclk is divided by 2**filter_clksel."]
    #[inline(always)]
    pub fn filter_clksel(&self) -> FILTER_CLKSEL_R {
        FILTER_CLKSEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - latch the interrupt , then can be cleared with next bit INTR_CLEAR"]
    #[inline(always)]
    pub fn latch_enable(&self) -> LATCH_ENABLE_R {
        LATCH_ENABLE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Write to clear wakeint_latched"]
    #[inline(always)]
    pub fn intr_clear(&self) -> INTR_CLEAR_R {
        INTR_CLEAR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt mask (which of the 8 PLU Outputs contribute to interrupt)"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
    #[doc = "Bits 8:9 - control input of the PLU, add filtering for glitch."]
    #[inline(always)]
    pub fn filter_mode(&mut self) -> FILTER_MODE_W {
        FILTER_MODE_W { w: self }
    }
    #[doc = "Bits 10:11 - hclk is divided by 2**filter_clksel."]
    #[inline(always)]
    pub fn filter_clksel(&mut self) -> FILTER_CLKSEL_W {
        FILTER_CLKSEL_W { w: self }
    }
    #[doc = "Bit 12 - latch the interrupt , then can be cleared with next bit INTR_CLEAR"]
    #[inline(always)]
    pub fn latch_enable(&mut self) -> LATCH_ENABLE_W {
        LATCH_ENABLE_W { w: self }
    }
    #[doc = "Bit 13 - Write to clear wakeint_latched"]
    #[inline(always)]
    pub fn intr_clear(&mut self) -> INTR_CLEAR_W {
        INTR_CLEAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wakeup interrupt control for PLU\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeint_ctrl](index.html) module"]
pub struct WAKEINT_CTRL_SPEC;
impl crate::RegisterSpec for WAKEINT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakeint_ctrl::R](R) reader structure"]
impl crate::Readable for WAKEINT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakeint_ctrl::W](W) writer structure"]
impl crate::Writable for WAKEINT_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKEINT_CTRL to value 0"]
impl crate::Resettable for WAKEINT_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
