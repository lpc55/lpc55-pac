#[doc = "Register `INDATA` writer"]
pub struct W(crate::W<INDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INDATA_SPEC>;
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
impl From<crate::W<INDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` writer - Write next word in little-endian form. The hash requires big endian word data, but this block swaps the bytes automatically. That is, SHA assumes the data coming in is treated as bytes (e.g. \"abcd\") and since the ARM core will treat \"abcd\" as a word as 0x64636261, the block will swap the word to restore into big endian."]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Write next word in little-endian form. The hash requires big endian word data, but this block swaps the bytes automatically. That is, SHA assumes the data coming in is treated as bytes (e.g. \"abcd\") and since the ARM core will treat \"abcd\" as a word as 0x64636261, the block will swap the word to restore into big endian."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input of 16 words at a time to load up buffer.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indata](index.html) module"]
pub struct INDATA_SPEC;
impl crate::RegisterSpec for INDATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [indata::W](W) writer structure"]
impl crate::Writable for INDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INDATA to value 0"]
impl crate::Resettable for INDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
