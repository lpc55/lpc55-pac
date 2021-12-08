#[doc = "Register `INT_SET_ENABLE` writer"]
pub struct W(crate::W<INT_SET_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_SET_ENABLE_SPEC>;
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
impl From<crate::W<INT_SET_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_SET_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAIL` writer - When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
pub struct FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> FAIL_W<'a> {
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
#[doc = "Field `ERR` writer - When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
pub struct ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_W<'a> {
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
#[doc = "Field `DONE` writer - When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
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
#[doc = "Field `ECC_ERR` writer - When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
pub struct ECC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_ERR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[inline(always)]
    pub fn fail(&mut self) -> FAIL_W {
        FAIL_W { w: self }
    }
    #[doc = "Bit 1 - When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W { w: self }
    }
    #[doc = "Bit 2 - When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    #[doc = "Bit 3 - When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[inline(always)]
    pub fn ecc_err(&mut self) -> ECC_ERR_W {
        ECC_ERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set interrupt enable bits\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_set_enable](index.html) module"]
pub struct INT_SET_ENABLE_SPEC;
impl crate::RegisterSpec for INT_SET_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_set_enable::W](W) writer structure"]
impl crate::Writable for INT_SET_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_SET_ENABLE to value 0"]
impl crate::Resettable for INT_SET_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
