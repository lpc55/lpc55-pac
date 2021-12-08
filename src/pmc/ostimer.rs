#[doc = "Register `OSTIMER` reader"]
pub struct R(crate::R<OSTIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSTIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSTIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSTIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSTIMER` writer"]
pub struct W(crate::W<OSTIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSTIMER_SPEC>;
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
impl From<crate::W<OSTIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSTIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFTRESET` reader - Active high reset."]
pub struct SOFTRESET_R(crate::FieldReader<bool, bool>);
impl SOFTRESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOFTRESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTRESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTRESET` writer - Active high reset."]
pub struct SOFTRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTRESET_W<'a> {
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
#[doc = "Field `CLOCKENABLE` reader - Enable OSTIMER 32 KHz clock."]
pub struct CLOCKENABLE_R(crate::FieldReader<bool, bool>);
impl CLOCKENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLOCKENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLOCKENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLOCKENABLE` writer - Enable OSTIMER 32 KHz clock."]
pub struct CLOCKENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCKENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DPDWAKEUPENABLE` reader - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
pub struct DPDWAKEUPENABLE_R(crate::FieldReader<bool, bool>);
impl DPDWAKEUPENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPDWAKEUPENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPDWAKEUPENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPDWAKEUPENABLE` writer - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
pub struct DPDWAKEUPENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPDWAKEUPENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `OSC32KPD` reader - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
pub struct OSC32KPD_R(crate::FieldReader<bool, bool>);
impl OSC32KPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC32KPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC32KPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC32KPD` writer - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
pub struct OSC32KPD_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC32KPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Active high reset."]
    #[inline(always)]
    pub fn softreset(&self) -> SOFTRESET_R {
        SOFTRESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable OSTIMER 32 KHz clock."]
    #[inline(always)]
    pub fn clockenable(&self) -> CLOCKENABLE_R {
        CLOCKENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
    #[inline(always)]
    pub fn dpdwakeupenable(&self) -> DPDWAKEUPENABLE_R {
        DPDWAKEUPENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
    #[inline(always)]
    pub fn osc32kpd(&self) -> OSC32KPD_R {
        OSC32KPD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active high reset."]
    #[inline(always)]
    pub fn softreset(&mut self) -> SOFTRESET_W {
        SOFTRESET_W { w: self }
    }
    #[doc = "Bit 1 - Enable OSTIMER 32 KHz clock."]
    #[inline(always)]
    pub fn clockenable(&mut self) -> CLOCKENABLE_W {
        CLOCKENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
    #[inline(always)]
    pub fn dpdwakeupenable(&mut self) -> DPDWAKEUPENABLE_W {
        DPDWAKEUPENABLE_W { w: self }
    }
    #[doc = "Bit 3 - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
    #[inline(always)]
    pub fn osc32kpd(&mut self) -> OSC32KPD_W {
        OSC32KPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ostimer](index.html) module"]
pub struct OSTIMER_SPEC;
impl crate::RegisterSpec for OSTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ostimer::R](R) reader structure"]
impl crate::Readable for OSTIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ostimer::W](W) writer structure"]
impl crate::Writable for OSTIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSTIMER to value 0x08"]
impl crate::Resettable for OSTIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
