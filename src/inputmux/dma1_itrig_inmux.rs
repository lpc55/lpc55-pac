#[doc = "Register `DMA1_ITRIG_INMUX[%s]` reader"]
pub struct R(crate::R<DMA1_ITRIG_INMUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA1_ITRIG_INMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA1_ITRIG_INMUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA1_ITRIG_INMUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA1_ITRIG_INMUX[%s]` writer"]
pub struct W(crate::W<DMA1_ITRIG_INMUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA1_ITRIG_INMUX_SPEC>;
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
impl From<crate::W<DMA1_ITRIG_INMUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA1_ITRIG_INMUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Trigger input number (decimal value) for DMA channel n (n = 0 to 9).\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INP_A {
    #[doc = "0: Pin interrupt 0"]
    VAL0 = 0,
    #[doc = "1: Pin interrupt 1"]
    VAL1 = 1,
    #[doc = "2: Pin interrupt 2"]
    VAL2 = 2,
    #[doc = "3: Pin interrupt 3"]
    VAL3 = 3,
    #[doc = "4: Timer CTIMER0 Match 0"]
    VAL4 = 4,
    #[doc = "5: Timer CTIMER0 Match 1"]
    VAL5 = 5,
    #[doc = "6: Timer CTIMER2 Match 0"]
    VAL6 = 6,
    #[doc = "7: Timer CTIMER4 Match 0"]
    VAL7 = 7,
    #[doc = "8: DMA1 output trigger mux 0"]
    VAL8 = 8,
    #[doc = "9: DMA1 output trigger mux 1"]
    VAL9 = 9,
    #[doc = "10: DMA1 output trigger mux 2"]
    VAL10 = 10,
    #[doc = "11: DMA1 output trigger mux 3"]
    VAL11 = 11,
    #[doc = "12: SCT0 DMA request 0"]
    VAL12 = 12,
    #[doc = "13: SCT0 DMA request 1"]
    VAL13 = 13,
    #[doc = "14: HASH DMA RX trigger"]
    VAL14 = 14,
    #[doc = "15: None"]
    VAL15 = 15,
}
impl From<INP_A> for u8 {
    #[inline(always)]
    fn from(variant: INP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INP` reader - Trigger input number (decimal value) for DMA channel n (n = 0 to 9)."]
pub struct INP_R(crate::FieldReader<u8, INP_A>);
impl INP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INP_A {
        match self.bits {
            0 => INP_A::VAL0,
            1 => INP_A::VAL1,
            2 => INP_A::VAL2,
            3 => INP_A::VAL3,
            4 => INP_A::VAL4,
            5 => INP_A::VAL5,
            6 => INP_A::VAL6,
            7 => INP_A::VAL7,
            8 => INP_A::VAL8,
            9 => INP_A::VAL9,
            10 => INP_A::VAL10,
            11 => INP_A::VAL11,
            12 => INP_A::VAL12,
            13 => INP_A::VAL13,
            14 => INP_A::VAL14,
            15 => INP_A::VAL15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        **self == INP_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        **self == INP_A::VAL1
    }
    #[doc = "Checks if the value of the field is `VAL2`"]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        **self == INP_A::VAL2
    }
    #[doc = "Checks if the value of the field is `VAL3`"]
    #[inline(always)]
    pub fn is_val3(&self) -> bool {
        **self == INP_A::VAL3
    }
    #[doc = "Checks if the value of the field is `VAL4`"]
    #[inline(always)]
    pub fn is_val4(&self) -> bool {
        **self == INP_A::VAL4
    }
    #[doc = "Checks if the value of the field is `VAL5`"]
    #[inline(always)]
    pub fn is_val5(&self) -> bool {
        **self == INP_A::VAL5
    }
    #[doc = "Checks if the value of the field is `VAL6`"]
    #[inline(always)]
    pub fn is_val6(&self) -> bool {
        **self == INP_A::VAL6
    }
    #[doc = "Checks if the value of the field is `VAL7`"]
    #[inline(always)]
    pub fn is_val7(&self) -> bool {
        **self == INP_A::VAL7
    }
    #[doc = "Checks if the value of the field is `VAL8`"]
    #[inline(always)]
    pub fn is_val8(&self) -> bool {
        **self == INP_A::VAL8
    }
    #[doc = "Checks if the value of the field is `VAL9`"]
    #[inline(always)]
    pub fn is_val9(&self) -> bool {
        **self == INP_A::VAL9
    }
    #[doc = "Checks if the value of the field is `VAL10`"]
    #[inline(always)]
    pub fn is_val10(&self) -> bool {
        **self == INP_A::VAL10
    }
    #[doc = "Checks if the value of the field is `VAL11`"]
    #[inline(always)]
    pub fn is_val11(&self) -> bool {
        **self == INP_A::VAL11
    }
    #[doc = "Checks if the value of the field is `VAL12`"]
    #[inline(always)]
    pub fn is_val12(&self) -> bool {
        **self == INP_A::VAL12
    }
    #[doc = "Checks if the value of the field is `VAL13`"]
    #[inline(always)]
    pub fn is_val13(&self) -> bool {
        **self == INP_A::VAL13
    }
    #[doc = "Checks if the value of the field is `VAL14`"]
    #[inline(always)]
    pub fn is_val14(&self) -> bool {
        **self == INP_A::VAL14
    }
    #[doc = "Checks if the value of the field is `VAL15`"]
    #[inline(always)]
    pub fn is_val15(&self) -> bool {
        **self == INP_A::VAL15
    }
}
impl core::ops::Deref for INP_R {
    type Target = crate::FieldReader<u8, INP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INP` writer - Trigger input number (decimal value) for DMA channel n (n = 0 to 9)."]
pub struct INP_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Pin interrupt 0"]
    #[inline(always)]
    pub fn val0(self) -> &'a mut W {
        self.variant(INP_A::VAL0)
    }
    #[doc = "Pin interrupt 1"]
    #[inline(always)]
    pub fn val1(self) -> &'a mut W {
        self.variant(INP_A::VAL1)
    }
    #[doc = "Pin interrupt 2"]
    #[inline(always)]
    pub fn val2(self) -> &'a mut W {
        self.variant(INP_A::VAL2)
    }
    #[doc = "Pin interrupt 3"]
    #[inline(always)]
    pub fn val3(self) -> &'a mut W {
        self.variant(INP_A::VAL3)
    }
    #[doc = "Timer CTIMER0 Match 0"]
    #[inline(always)]
    pub fn val4(self) -> &'a mut W {
        self.variant(INP_A::VAL4)
    }
    #[doc = "Timer CTIMER0 Match 1"]
    #[inline(always)]
    pub fn val5(self) -> &'a mut W {
        self.variant(INP_A::VAL5)
    }
    #[doc = "Timer CTIMER2 Match 0"]
    #[inline(always)]
    pub fn val6(self) -> &'a mut W {
        self.variant(INP_A::VAL6)
    }
    #[doc = "Timer CTIMER4 Match 0"]
    #[inline(always)]
    pub fn val7(self) -> &'a mut W {
        self.variant(INP_A::VAL7)
    }
    #[doc = "DMA1 output trigger mux 0"]
    #[inline(always)]
    pub fn val8(self) -> &'a mut W {
        self.variant(INP_A::VAL8)
    }
    #[doc = "DMA1 output trigger mux 1"]
    #[inline(always)]
    pub fn val9(self) -> &'a mut W {
        self.variant(INP_A::VAL9)
    }
    #[doc = "DMA1 output trigger mux 2"]
    #[inline(always)]
    pub fn val10(self) -> &'a mut W {
        self.variant(INP_A::VAL10)
    }
    #[doc = "DMA1 output trigger mux 3"]
    #[inline(always)]
    pub fn val11(self) -> &'a mut W {
        self.variant(INP_A::VAL11)
    }
    #[doc = "SCT0 DMA request 0"]
    #[inline(always)]
    pub fn val12(self) -> &'a mut W {
        self.variant(INP_A::VAL12)
    }
    #[doc = "SCT0 DMA request 1"]
    #[inline(always)]
    pub fn val13(self) -> &'a mut W {
        self.variant(INP_A::VAL13)
    }
    #[doc = "HASH DMA RX trigger"]
    #[inline(always)]
    pub fn val14(self) -> &'a mut W {
        self.variant(INP_A::VAL14)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val15(self) -> &'a mut W {
        self.variant(INP_A::VAL15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Trigger input number (decimal value) for DMA channel n (n = 0 to 9)."]
    #[inline(always)]
    pub fn inp(&self) -> INP_R {
        INP_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Trigger input number (decimal value) for DMA channel n (n = 0 to 9)."]
    #[inline(always)]
    pub fn inp(&mut self) -> INP_W {
        INP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger select register for DMA1 channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1_itrig_inmux](index.html) module"]
pub struct DMA1_ITRIG_INMUX_SPEC;
impl crate::RegisterSpec for DMA1_ITRIG_INMUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma1_itrig_inmux::R](R) reader structure"]
impl crate::Readable for DMA1_ITRIG_INMUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma1_itrig_inmux::W](W) writer structure"]
impl crate::Writable for DMA1_ITRIG_INMUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA1_ITRIG_INMUX[%s]
to value 0x0f"]
impl crate::Resettable for DMA1_ITRIG_INMUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
