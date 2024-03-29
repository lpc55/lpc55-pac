#[doc = "Register `DIRNOT[%s]` writer"]
pub struct W(crate::W<DIRNOT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIRNOT_SPEC>;
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
impl From<crate::W<DIRNOT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIRNOT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRNOTP` writer - Toggle direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle direction bit."]
pub struct DIRNOTP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRNOTP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Toggle direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle direction bit."]
    #[inline(always)]
    pub fn dirnotp(&mut self) -> DIRNOTP_W {
        DIRNOTP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Toggle pin direction bits for port\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirnot](index.html) module"]
pub struct DIRNOT_SPEC;
impl crate::RegisterSpec for DIRNOT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dirnot::W](W) writer structure"]
impl crate::Writable for DIRNOT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIRNOT[%s]
to value 0"]
impl crate::Resettable for DIRNOT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
