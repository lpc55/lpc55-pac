#[doc = "Register `KEYRESET` writer"]
pub struct W(crate::W<KEYRESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYRESET_SPEC>;
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
impl From<crate::W<KEYRESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYRESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY0` writer - 10: Reset KEY0 shift register. Self clearing. Must be done before loading any new key."]
pub struct KEY0_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `KEY1` writer - 10: Reset KEY1 shift register. Self clearing. Must be done before loading any new key."]
pub struct KEY1_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `KEY2` writer - 10: Reset KEY2 shift register. Self clearing. Must be done before loading any new key."]
pub struct KEY2_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `KEY3` writer - 10: Reset KEY3 shift register. Self clearing. Must be done before loading any new key."]
pub struct KEY3_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:1 - 10: Reset KEY0 shift register. Self clearing. Must be done before loading any new key."]
    #[inline(always)]
    pub fn key0(&mut self) -> KEY0_W {
        KEY0_W { w: self }
    }
    #[doc = "Bits 2:3 - 10: Reset KEY1 shift register. Self clearing. Must be done before loading any new key."]
    #[inline(always)]
    pub fn key1(&mut self) -> KEY1_W {
        KEY1_W { w: self }
    }
    #[doc = "Bits 4:5 - 10: Reset KEY2 shift register. Self clearing. Must be done before loading any new key."]
    #[inline(always)]
    pub fn key2(&mut self) -> KEY2_W {
        KEY2_W { w: self }
    }
    #[doc = "Bits 6:7 - 10: Reset KEY3 shift register. Self clearing. Must be done before loading any new key."]
    #[inline(always)]
    pub fn key3(&mut self) -> KEY3_W {
        KEY3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reinitialize Keys shift registers counters\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyreset](index.html) module"]
pub struct KEYRESET_SPEC;
impl crate::RegisterSpec for KEYRESET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [keyreset::W](W) writer structure"]
impl crate::Writable for KEYRESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYRESET to value 0"]
impl crate::Resettable for KEYRESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
