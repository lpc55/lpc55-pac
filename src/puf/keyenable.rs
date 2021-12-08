#[doc = "Register `KEYENABLE` reader"]
pub struct R(crate::R<KEYENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYENABLE` writer"]
pub struct W(crate::W<KEYENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYENABLE_SPEC>;
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
impl From<crate::W<KEYENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY0` reader - \"10: Data coming out from PUF Index 0 interface are shifted in KEY0 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY0 register.\""]
pub struct KEY0_R(crate::FieldReader<u8, u8>);
impl KEY0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY0` writer - \"10: Data coming out from PUF Index 0 interface are shifted in KEY0 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY0 register.\""]
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
#[doc = "Field `KEY1` reader - \"10: Data coming out from PUF Index 0 interface are shifted in KEY1 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY1 register.\""]
pub struct KEY1_R(crate::FieldReader<u8, u8>);
impl KEY1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY1` writer - \"10: Data coming out from PUF Index 0 interface are shifted in KEY1 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY1 register.\""]
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
#[doc = "Field `KEY2` reader - \"10: Data coming out from PUF Index 0 interface are shifted in KEY2 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY2 register.\""]
pub struct KEY2_R(crate::FieldReader<u8, u8>);
impl KEY2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY2` writer - \"10: Data coming out from PUF Index 0 interface are shifted in KEY2 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY2 register.\""]
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
#[doc = "Field `KEY3` reader - \"10: Data coming out from PUF Index 0 interface are shifted in KEY3 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY3 register.\""]
pub struct KEY3_R(crate::FieldReader<u8, u8>);
impl KEY3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY3` writer - \"10: Data coming out from PUF Index 0 interface are shifted in KEY3 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY3 register.\""]
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
#[doc = "Key destination for PUF key.\n\nValue on reset: 85"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEY_A {
    #[doc = "86: Send key to AES engine."]
    AES = 86,
    #[doc = "89: Send key to PRINCE engine for memory layout 0."]
    PRINCE0 = 89,
    #[doc = "101: Send key to PRINCE engine for memory layout 1."]
    PRINCE1 = 101,
    #[doc = "149: Send key to PRINCE engine for memory layout 2."]
    PRINCE2 = 149,
    #[doc = "85: Do not send key to any hardware engine."]
    NONE = 85,
}
impl From<KEY_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` reader - Key destination for PUF key."]
pub struct KEY_R(crate::FieldReader<u8, KEY_A>);
impl KEY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEY_A> {
        match self.bits {
            86 => Some(KEY_A::AES),
            89 => Some(KEY_A::PRINCE0),
            101 => Some(KEY_A::PRINCE1),
            149 => Some(KEY_A::PRINCE2),
            85 => Some(KEY_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AES`"]
    #[inline(always)]
    pub fn is_aes(&self) -> bool {
        **self == KEY_A::AES
    }
    #[doc = "Checks if the value of the field is `PRINCE0`"]
    #[inline(always)]
    pub fn is_prince0(&self) -> bool {
        **self == KEY_A::PRINCE0
    }
    #[doc = "Checks if the value of the field is `PRINCE1`"]
    #[inline(always)]
    pub fn is_prince1(&self) -> bool {
        **self == KEY_A::PRINCE1
    }
    #[doc = "Checks if the value of the field is `PRINCE2`"]
    #[inline(always)]
    pub fn is_prince2(&self) -> bool {
        **self == KEY_A::PRINCE2
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == KEY_A::NONE
    }
}
impl core::ops::Deref for KEY_R {
    type Target = crate::FieldReader<u8, KEY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY` writer - Key destination for PUF key."]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Send key to AES engine."]
    #[inline(always)]
    pub fn aes(self) -> &'a mut W {
        self.variant(KEY_A::AES)
    }
    #[doc = "Send key to PRINCE engine for memory layout 0."]
    #[inline(always)]
    pub fn prince0(self) -> &'a mut W {
        self.variant(KEY_A::PRINCE0)
    }
    #[doc = "Send key to PRINCE engine for memory layout 1."]
    #[inline(always)]
    pub fn prince1(self) -> &'a mut W {
        self.variant(KEY_A::PRINCE1)
    }
    #[doc = "Send key to PRINCE engine for memory layout 2."]
    #[inline(always)]
    pub fn prince2(self) -> &'a mut W {
        self.variant(KEY_A::PRINCE2)
    }
    #[doc = "Do not send key to any hardware engine."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(KEY_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY0 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY0 register.\""]
    #[inline(always)]
    pub fn key0(&self) -> KEY0_R {
        KEY0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY1 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY1 register.\""]
    #[inline(always)]
    pub fn key1(&self) -> KEY1_R {
        KEY1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY2 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY2 register.\""]
    #[inline(always)]
    pub fn key2(&self) -> KEY2_R {
        KEY2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY3 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY3 register.\""]
    #[inline(always)]
    pub fn key3(&self) -> KEY3_R {
        KEY3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 0:7 - Key destination for PUF key."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY0 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY0 register.\""]
    #[inline(always)]
    pub fn key0(&mut self) -> KEY0_W {
        KEY0_W { w: self }
    }
    #[doc = "Bits 2:3 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY1 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY1 register.\""]
    #[inline(always)]
    pub fn key1(&mut self) -> KEY1_W {
        KEY1_W { w: self }
    }
    #[doc = "Bits 4:5 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY2 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY2 register.\""]
    #[inline(always)]
    pub fn key2(&mut self) -> KEY2_W {
        KEY2_W { w: self }
    }
    #[doc = "Bits 6:7 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY3 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY3 register.\""]
    #[inline(always)]
    pub fn key3(&mut self) -> KEY3_W {
        KEY3_W { w: self }
    }
    #[doc = "Bits 0:7 - Key destination for PUF key."]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyenable](index.html) module"]
pub struct KEYENABLE_SPEC;
impl crate::RegisterSpec for KEYENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyenable::R](R) reader structure"]
impl crate::Readable for KEYENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyenable::W](W) writer structure"]
impl crate::Writable for KEYENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYENABLE to value 0x55"]
impl crate::Resettable for KEYENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x55
    }
}
