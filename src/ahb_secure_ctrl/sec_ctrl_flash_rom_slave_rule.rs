#[doc = "Register `SEC_CTRL_FLASH_ROM_SLAVE_RULE` reader"]
pub struct R(crate::R<SEC_CTRL_FLASH_ROM_SLAVE_RULE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CTRL_FLASH_ROM_SLAVE_RULE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SEC_CTRL_FLASH_ROM_SLAVE_RULE_SPEC>> for R {
    fn from(reader: crate::R<SEC_CTRL_FLASH_ROM_SLAVE_RULE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CTRL_FLASH_ROM_SLAVE_RULE` writer"]
pub struct W(crate::W<SEC_CTRL_FLASH_ROM_SLAVE_RULE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CTRL_FLASH_ROM_SLAVE_RULE_SPEC>;
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
impl core::convert::From<crate::W<SEC_CTRL_FLASH_ROM_SLAVE_RULE_SPEC>> for W {
    fn from(writer: crate::W<SEC_CTRL_FLASH_ROM_SLAVE_RULE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Security access rules for the whole FLASH : 0x0000_0000 - 0x0009_FFFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASH_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<FLASH_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASH_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLASH_RULE` reader - Security access rules for the whole FLASH : 0x0000_0000 - 0x0009_FFFF"]
pub struct FLASH_RULE_R(crate::FieldReader<u8, FLASH_RULE_A>);
impl FLASH_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLASH_RULE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_RULE_A {
        match self.bits {
            0 => FLASH_RULE_A::ENUM_NS_NP,
            1 => FLASH_RULE_A::ENUM_NS_P,
            2 => FLASH_RULE_A::ENUM_S_NP,
            3 => FLASH_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == FLASH_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == FLASH_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == FLASH_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == FLASH_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for FLASH_RULE_R {
    type Target = crate::FieldReader<u8, FLASH_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_RULE` writer - Security access rules for the whole FLASH : 0x0000_0000 - 0x0009_FFFF"]
pub struct FLASH_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_RULE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(FLASH_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(FLASH_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(FLASH_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(FLASH_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Security access rules for the whole ROM : 0x0300_0000 - 0x0301_FFFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ROM_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<ROM_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: ROM_RULE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ROM_RULE` reader - Security access rules for the whole ROM : 0x0300_0000 - 0x0301_FFFF"]
pub struct ROM_RULE_R(crate::FieldReader<u8, ROM_RULE_A>);
impl ROM_RULE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ROM_RULE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROM_RULE_A {
        match self.bits {
            0 => ROM_RULE_A::ENUM_NS_NP,
            1 => ROM_RULE_A::ENUM_NS_P,
            2 => ROM_RULE_A::ENUM_S_NP,
            3 => ROM_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        **self == ROM_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        **self == ROM_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        **self == ROM_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        **self == ROM_RULE_A::ENUM_S_P
    }
}
impl core::ops::Deref for ROM_RULE_R {
    type Target = crate::FieldReader<u8, ROM_RULE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_RULE` writer - Security access rules for the whole ROM : 0x0300_0000 - 0x0301_FFFF"]
pub struct ROM_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROM_RULE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(ROM_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(ROM_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(ROM_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(ROM_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Security access rules for the whole FLASH : 0x0000_0000 - 0x0009_FFFF"]
    #[inline(always)]
    pub fn flash_rule(&self) -> FLASH_RULE_R {
        FLASH_RULE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Security access rules for the whole ROM : 0x0300_0000 - 0x0301_FFFF"]
    #[inline(always)]
    pub fn rom_rule(&self) -> ROM_RULE_R {
        ROM_RULE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Security access rules for the whole FLASH : 0x0000_0000 - 0x0009_FFFF"]
    #[inline(always)]
    pub fn flash_rule(&mut self) -> FLASH_RULE_W {
        FLASH_RULE_W { w: self }
    }
    #[doc = "Bits 4:5 - Security access rules for the whole ROM : 0x0300_0000 - 0x0301_FFFF"]
    #[inline(always)]
    pub fn rom_rule(&mut self) -> ROM_RULE_W {
        ROM_RULE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security access rules for Flash and ROM slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_flash_rom_slave_rule](index.html) module"]
pub struct SEC_CTRL_FLASH_ROM_SLAVE_RULE_SPEC;
impl crate::RegisterSpec for SEC_CTRL_FLASH_ROM_SLAVE_RULE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_ctrl_flash_rom_slave_rule::R](R) reader structure"]
impl crate::Readable for SEC_CTRL_FLASH_ROM_SLAVE_RULE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_flash_rom_slave_rule::W](W) writer structure"]
impl crate::Writable for SEC_CTRL_FLASH_ROM_SLAVE_RULE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC_CTRL_FLASH_ROM_SLAVE_RULE to value 0"]
impl crate::Resettable for SEC_CTRL_FLASH_ROM_SLAVE_RULE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
