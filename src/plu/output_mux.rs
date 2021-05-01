#[doc = "Register `OUTPUT_MUX[%s]` reader"]
pub struct R(crate::R<OUTPUT_MUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTPUT_MUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OUTPUT_MUX_SPEC>> for R {
    fn from(reader: crate::R<OUTPUT_MUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTPUT_MUX[%s]` writer"]
pub struct W(crate::W<OUTPUT_MUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTPUT_MUX_SPEC>;
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
impl core::convert::From<crate::W<OUTPUT_MUX_SPEC>> for W {
    fn from(writer: crate::W<OUTPUT_MUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects the source to be connected to PLU Output 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUTPUTN_A {
    #[doc = "0: The PLU output 0."]
    PLU_OUTPUT0 = 0,
    #[doc = "1: The PLU output 1."]
    PLU_OUTPUT1 = 1,
    #[doc = "2: The PLU output 2."]
    PLU_OUTPUT2 = 2,
    #[doc = "3: The PLU output 3."]
    PLU_OUTPUT3 = 3,
    #[doc = "4: The PLU output 4."]
    PLU_OUTPUT4 = 4,
    #[doc = "5: The PLU output 5."]
    PLU_OUTPUT5 = 5,
    #[doc = "6: The PLU output 6."]
    PLU_OUTPUT6 = 6,
    #[doc = "7: The PLU output 7."]
    PLU_OUTPUT7 = 7,
    #[doc = "8: The PLU output 8."]
    PLU_OUTPUT8 = 8,
    #[doc = "9: The PLU output 9."]
    PLU_OUTPUT9 = 9,
    #[doc = "10: The PLU output 10."]
    PLU_OUTPUT10 = 10,
    #[doc = "11: The PLU output 11."]
    PLU_OUTPUT11 = 11,
    #[doc = "12: The PLU output 12."]
    PLU_OUTPUT12 = 12,
    #[doc = "13: The PLU output 13."]
    PLU_OUTPUT13 = 13,
    #[doc = "14: The PLU output 14."]
    PLU_OUTPUT14 = 14,
    #[doc = "15: The PLU output 15."]
    PLU_OUTPUT15 = 15,
    #[doc = "16: The PLU output 16."]
    PLU_OUTPUT16 = 16,
    #[doc = "17: The PLU output 17."]
    PLU_OUTPUT17 = 17,
    #[doc = "18: The PLU output 18."]
    PLU_OUTPUT18 = 18,
    #[doc = "19: The PLU output 19."]
    PLU_OUTPUT19 = 19,
    #[doc = "20: The PLU output 20."]
    PLU_OUTPUT20 = 20,
    #[doc = "21: The PLU output 21."]
    PLU_OUTPUT21 = 21,
    #[doc = "22: The PLU output 22."]
    PLU_OUTPUT22 = 22,
    #[doc = "23: The PLU output 23."]
    PLU_OUTPUT23 = 23,
    #[doc = "24: The PLU output 24."]
    PLU_OUTPUT24 = 24,
    #[doc = "25: The PLU output 25."]
    PLU_OUTPUT25 = 25,
    #[doc = "26: state(0)."]
    STATE0 = 26,
    #[doc = "27: state(1)."]
    STATE1 = 27,
    #[doc = "28: state(2)."]
    STATE2 = 28,
    #[doc = "29: state(3)."]
    STATE3 = 29,
}
impl From<OUTPUTN_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTPUTN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OUTPUTn` reader - Selects the source to be connected to PLU Output 0."]
pub struct OUTPUTN_R(crate::FieldReader<u8, OUTPUTN_A>);
impl OUTPUTN_R {
    pub(crate) fn new(bits: u8) -> Self {
        OUTPUTN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OUTPUTN_A> {
        match self.bits {
            0 => Some(OUTPUTN_A::PLU_OUTPUT0),
            1 => Some(OUTPUTN_A::PLU_OUTPUT1),
            2 => Some(OUTPUTN_A::PLU_OUTPUT2),
            3 => Some(OUTPUTN_A::PLU_OUTPUT3),
            4 => Some(OUTPUTN_A::PLU_OUTPUT4),
            5 => Some(OUTPUTN_A::PLU_OUTPUT5),
            6 => Some(OUTPUTN_A::PLU_OUTPUT6),
            7 => Some(OUTPUTN_A::PLU_OUTPUT7),
            8 => Some(OUTPUTN_A::PLU_OUTPUT8),
            9 => Some(OUTPUTN_A::PLU_OUTPUT9),
            10 => Some(OUTPUTN_A::PLU_OUTPUT10),
            11 => Some(OUTPUTN_A::PLU_OUTPUT11),
            12 => Some(OUTPUTN_A::PLU_OUTPUT12),
            13 => Some(OUTPUTN_A::PLU_OUTPUT13),
            14 => Some(OUTPUTN_A::PLU_OUTPUT14),
            15 => Some(OUTPUTN_A::PLU_OUTPUT15),
            16 => Some(OUTPUTN_A::PLU_OUTPUT16),
            17 => Some(OUTPUTN_A::PLU_OUTPUT17),
            18 => Some(OUTPUTN_A::PLU_OUTPUT18),
            19 => Some(OUTPUTN_A::PLU_OUTPUT19),
            20 => Some(OUTPUTN_A::PLU_OUTPUT20),
            21 => Some(OUTPUTN_A::PLU_OUTPUT21),
            22 => Some(OUTPUTN_A::PLU_OUTPUT22),
            23 => Some(OUTPUTN_A::PLU_OUTPUT23),
            24 => Some(OUTPUTN_A::PLU_OUTPUT24),
            25 => Some(OUTPUTN_A::PLU_OUTPUT25),
            26 => Some(OUTPUTN_A::STATE0),
            27 => Some(OUTPUTN_A::STATE1),
            28 => Some(OUTPUTN_A::STATE2),
            29 => Some(OUTPUTN_A::STATE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT0`"]
    #[inline(always)]
    pub fn is_plu_output0(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT0
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT1`"]
    #[inline(always)]
    pub fn is_plu_output1(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT1
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT2`"]
    #[inline(always)]
    pub fn is_plu_output2(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT2
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT3`"]
    #[inline(always)]
    pub fn is_plu_output3(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT3
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT4`"]
    #[inline(always)]
    pub fn is_plu_output4(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT4
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT5`"]
    #[inline(always)]
    pub fn is_plu_output5(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT5
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT6`"]
    #[inline(always)]
    pub fn is_plu_output6(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT6
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT7`"]
    #[inline(always)]
    pub fn is_plu_output7(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT7
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT8`"]
    #[inline(always)]
    pub fn is_plu_output8(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT8
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT9`"]
    #[inline(always)]
    pub fn is_plu_output9(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT9
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT10`"]
    #[inline(always)]
    pub fn is_plu_output10(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT10
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT11`"]
    #[inline(always)]
    pub fn is_plu_output11(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT11
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT12`"]
    #[inline(always)]
    pub fn is_plu_output12(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT12
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT13`"]
    #[inline(always)]
    pub fn is_plu_output13(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT13
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT14`"]
    #[inline(always)]
    pub fn is_plu_output14(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT14
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT15`"]
    #[inline(always)]
    pub fn is_plu_output15(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT15
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT16`"]
    #[inline(always)]
    pub fn is_plu_output16(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT16
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT17`"]
    #[inline(always)]
    pub fn is_plu_output17(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT17
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT18`"]
    #[inline(always)]
    pub fn is_plu_output18(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT18
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT19`"]
    #[inline(always)]
    pub fn is_plu_output19(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT19
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT20`"]
    #[inline(always)]
    pub fn is_plu_output20(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT20
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT21`"]
    #[inline(always)]
    pub fn is_plu_output21(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT21
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT22`"]
    #[inline(always)]
    pub fn is_plu_output22(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT22
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT23`"]
    #[inline(always)]
    pub fn is_plu_output23(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT23
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT24`"]
    #[inline(always)]
    pub fn is_plu_output24(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT24
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT25`"]
    #[inline(always)]
    pub fn is_plu_output25(&self) -> bool {
        **self == OUTPUTN_A::PLU_OUTPUT25
    }
    #[doc = "Checks if the value of the field is `STATE0`"]
    #[inline(always)]
    pub fn is_state0(&self) -> bool {
        **self == OUTPUTN_A::STATE0
    }
    #[doc = "Checks if the value of the field is `STATE1`"]
    #[inline(always)]
    pub fn is_state1(&self) -> bool {
        **self == OUTPUTN_A::STATE1
    }
    #[doc = "Checks if the value of the field is `STATE2`"]
    #[inline(always)]
    pub fn is_state2(&self) -> bool {
        **self == OUTPUTN_A::STATE2
    }
    #[doc = "Checks if the value of the field is `STATE3`"]
    #[inline(always)]
    pub fn is_state3(&self) -> bool {
        **self == OUTPUTN_A::STATE3
    }
}
impl core::ops::Deref for OUTPUTN_R {
    type Target = crate::FieldReader<u8, OUTPUTN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTPUTn` writer - Selects the source to be connected to PLU Output 0."]
pub struct OUTPUTN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUTN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTPUTN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The PLU output 0."]
    #[inline(always)]
    pub fn plu_output0(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT0)
    }
    #[doc = "The PLU output 1."]
    #[inline(always)]
    pub fn plu_output1(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT1)
    }
    #[doc = "The PLU output 2."]
    #[inline(always)]
    pub fn plu_output2(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT2)
    }
    #[doc = "The PLU output 3."]
    #[inline(always)]
    pub fn plu_output3(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT3)
    }
    #[doc = "The PLU output 4."]
    #[inline(always)]
    pub fn plu_output4(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT4)
    }
    #[doc = "The PLU output 5."]
    #[inline(always)]
    pub fn plu_output5(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT5)
    }
    #[doc = "The PLU output 6."]
    #[inline(always)]
    pub fn plu_output6(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT6)
    }
    #[doc = "The PLU output 7."]
    #[inline(always)]
    pub fn plu_output7(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT7)
    }
    #[doc = "The PLU output 8."]
    #[inline(always)]
    pub fn plu_output8(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT8)
    }
    #[doc = "The PLU output 9."]
    #[inline(always)]
    pub fn plu_output9(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT9)
    }
    #[doc = "The PLU output 10."]
    #[inline(always)]
    pub fn plu_output10(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT10)
    }
    #[doc = "The PLU output 11."]
    #[inline(always)]
    pub fn plu_output11(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT11)
    }
    #[doc = "The PLU output 12."]
    #[inline(always)]
    pub fn plu_output12(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT12)
    }
    #[doc = "The PLU output 13."]
    #[inline(always)]
    pub fn plu_output13(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT13)
    }
    #[doc = "The PLU output 14."]
    #[inline(always)]
    pub fn plu_output14(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT14)
    }
    #[doc = "The PLU output 15."]
    #[inline(always)]
    pub fn plu_output15(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT15)
    }
    #[doc = "The PLU output 16."]
    #[inline(always)]
    pub fn plu_output16(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT16)
    }
    #[doc = "The PLU output 17."]
    #[inline(always)]
    pub fn plu_output17(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT17)
    }
    #[doc = "The PLU output 18."]
    #[inline(always)]
    pub fn plu_output18(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT18)
    }
    #[doc = "The PLU output 19."]
    #[inline(always)]
    pub fn plu_output19(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT19)
    }
    #[doc = "The PLU output 20."]
    #[inline(always)]
    pub fn plu_output20(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT20)
    }
    #[doc = "The PLU output 21."]
    #[inline(always)]
    pub fn plu_output21(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT21)
    }
    #[doc = "The PLU output 22."]
    #[inline(always)]
    pub fn plu_output22(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT22)
    }
    #[doc = "The PLU output 23."]
    #[inline(always)]
    pub fn plu_output23(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT23)
    }
    #[doc = "The PLU output 24."]
    #[inline(always)]
    pub fn plu_output24(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT24)
    }
    #[doc = "The PLU output 25."]
    #[inline(always)]
    pub fn plu_output25(self) -> &'a mut W {
        self.variant(OUTPUTN_A::PLU_OUTPUT25)
    }
    #[doc = "state(0)."]
    #[inline(always)]
    pub fn state0(self) -> &'a mut W {
        self.variant(OUTPUTN_A::STATE0)
    }
    #[doc = "state(1)."]
    #[inline(always)]
    pub fn state1(self) -> &'a mut W {
        self.variant(OUTPUTN_A::STATE1)
    }
    #[doc = "state(2)."]
    #[inline(always)]
    pub fn state2(self) -> &'a mut W {
        self.variant(OUTPUTN_A::STATE2)
    }
    #[doc = "state(3)."]
    #[inline(always)]
    pub fn state3(self) -> &'a mut W {
        self.variant(OUTPUTN_A::STATE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Selects the source to be connected to PLU Output 0."]
    #[inline(always)]
    pub fn outputn(&self) -> OUTPUTN_R {
        OUTPUTN_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects the source to be connected to PLU Output 0."]
    #[inline(always)]
    pub fn outputn(&mut self) -> OUTPUTN_W {
        OUTPUTN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selects the source to be connected to PLU Output OUTPUT_n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [output_mux](index.html) module"]
pub struct OUTPUT_MUX_SPEC;
impl crate::RegisterSpec for OUTPUT_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [output_mux::R](R) reader structure"]
impl crate::Readable for OUTPUT_MUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [output_mux::W](W) writer structure"]
impl crate::Writable for OUTPUT_MUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTPUT_MUX[%s]
to value 0"]
impl crate::Resettable for OUTPUT_MUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
