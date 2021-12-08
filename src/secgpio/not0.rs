#[doc = "Register `NOT0` writer"]
pub struct W(crate::W<NOT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NOT0_SPEC>;
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
impl From<crate::W<NOT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NOT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NOTP` writer - Toggle output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Toggle output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp(&mut self) -> NOTP_W {
        NOTP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Toggle port for all port GPIO pins\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [not0](index.html) module"]
pub struct NOT0_SPEC;
impl crate::RegisterSpec for NOT0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [not0::W](W) writer structure"]
impl crate::Writable for NOT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NOT0 to value 0"]
impl crate::Resettable for NOT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
