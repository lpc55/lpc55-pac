#[doc = "Register `BASE_ADDR0` reader"]
pub struct R(crate::R<BASE_ADDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BASE_ADDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BASE_ADDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BASE_ADDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BASE_ADDR0` writer"]
pub struct W(crate::W<BASE_ADDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BASE_ADDR0_SPEC>;
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
impl From<crate::W<BASE_ADDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BASE_ADDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR_FIXED` reader - Fixed portion of the base address of region 0."]
pub struct ADDR_FIXED_R(crate::FieldReader<u32, u32>);
impl ADDR_FIXED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ADDR_FIXED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_FIXED_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR_PRG` reader - Programmable portion of the base address of region 0."]
pub struct ADDR_PRG_R(crate::FieldReader<u8, u8>);
impl ADDR_PRG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADDR_PRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_PRG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR_PRG` writer - Programmable portion of the base address of region 0."]
pub struct ADDR_PRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_PRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Fixed portion of the base address of region 0."]
    #[inline(always)]
    pub fn addr_fixed(&self) -> ADDR_FIXED_R {
        ADDR_FIXED_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:19 - Programmable portion of the base address of region 0."]
    #[inline(always)]
    pub fn addr_prg(&self) -> ADDR_PRG_R {
        ADDR_PRG_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 18:19 - Programmable portion of the base address of region 0."]
    #[inline(always)]
    pub fn addr_prg(&mut self) -> ADDR_PRG_W {
        ADDR_PRG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Base Address for region 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [base_addr0](index.html) module"]
pub struct BASE_ADDR0_SPEC;
impl crate::RegisterSpec for BASE_ADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [base_addr0::R](R) reader structure"]
impl crate::Readable for BASE_ADDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [base_addr0::W](W) writer structure"]
impl crate::Writable for BASE_ADDR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BASE_ADDR0 to value 0"]
impl crate::Resettable for BASE_ADDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
