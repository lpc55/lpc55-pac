#[doc = "Writer for register CODESECURITYPROTTEST"]
pub type W = crate::W<u32, super::CODESECURITYPROTTEST>;
#[doc = "Register CODESECURITYPROTTEST `reset()`'s with value 0"]
impl crate::ResetValue for super::CODESECURITYPROTTEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `SEC_CODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_CODE_AW {
    #[doc = "test access is not allowed."]
    DISABLE,
    #[doc = "Security code to allow test access."]
    ENABLE,
}
impl From<SEC_CODE_AW> for u32 {
    #[inline(always)]
    fn from(variant: SEC_CODE_AW) -> Self {
        match variant {
            SEC_CODE_AW::DISABLE => 0,
            SEC_CODE_AW::ENABLE => 305419896,
        }
    }
}
#[doc = "Write proxy for field `SEC_CODE`"]
pub struct SEC_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_CODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEC_CODE_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "test access is not allowed."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SEC_CODE_AW::DISABLE)
    }
    #[doc = "Security code to allow test access."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SEC_CODE_AW::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Security code to allow test access : 0x12345678."]
    #[inline(always)]
    pub fn sec_code(&mut self) -> SEC_CODE_W {
        SEC_CODE_W { w: self }
    }
}
