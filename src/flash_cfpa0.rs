#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ."]
    pub header: crate::Reg<header::HEADER_SPEC>,
    #[doc = "0x04 - ."]
    pub version: crate::Reg<version::VERSION_SPEC>,
    #[doc = "0x08 - Secure firmware version (Monotonic counter)"]
    pub s_fw_version: crate::Reg<s_fw_version::S_FW_VERSION_SPEC>,
    #[doc = "0x0c - Non-Secure firmware version (Monotonic counter)"]
    pub ns_fw_version: crate::Reg<ns_fw_version::NS_FW_VERSION_SPEC>,
    #[doc = "0x10 - Image key revocation ID (Monotonic counter)"]
    pub image_key_revoke: crate::Reg<image_key_revoke::IMAGE_KEY_REVOKE_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - ."]
    pub rotkh_revoke: crate::Reg<rotkh_revoke::ROTKH_REVOKE_SPEC>,
    #[doc = "0x1c - ."]
    pub vendor_usage: crate::Reg<vendor_usage::VENDOR_USAGE_SPEC>,
    #[doc = "0x20 - With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
    pub dcfg_cc_socu_pin: crate::Reg<dcfg_cc_socu_pin::DCFG_CC_SOCU_PIN_SPEC>,
    #[doc = "0x24 - With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
    pub dcfg_cc_socu_dflt: crate::Reg<dcfg_cc_socu_dflt::DCFG_CC_SOCU_DFLT_SPEC>,
    #[doc = "0x28 - Enable FA mode. SET_FA_MODE Command should write 0xC33CA55A to this word to indicate boot ROM to enter FA mode."]
    pub enable_fa_mode: crate::Reg<enable_fa_mode::ENABLE_FA_MODE_SPEC>,
    #[doc = "0x2c - CMPA Page programming on going. This field shall be set to 0x5CC55AA5 in the active CFPA page each time CMPA page programming is going on. It shall always be set to 0x00000000 in the CFPA scratch area."]
    pub cmpa_prog_in_progress: crate::Reg<cmpa_prog_in_progress::CMPA_PROG_IN_PROGRESS_SPEC>,
    _reserved_11_prince_region0_iv: [u8; 0x04],
    _reserved_12_prince_region0_iv: [u8; 0x04],
    _reserved_13_prince_region0_iv: [u8; 0x04],
    _reserved_14_prince_region0_iv: [u8; 0x04],
    _reserved_15_prince_region0_iv: [u8; 0x04],
    _reserved_16_prince_region0_iv: [u8; 0x04],
    _reserved_17_prince_region0_iv: [u8; 0x04],
    _reserved_18_prince_region0_iv: [u8; 0x04],
    _reserved_19_prince_region0_iv: [u8; 0x04],
    _reserved_20_prince_region0_iv: [u8; 0x04],
    _reserved_21_prince_region0_iv: [u8; 0x04],
    _reserved_22_prince_region0_iv: [u8; 0x04],
    _reserved_23_prince_region0_iv: [u8; 0x04],
    _reserved_24_prince_region0_iv: [u8; 0x04],
    _reserved_25_prince_region1_iv: [u8; 0x04],
    _reserved_26_prince_region1_iv: [u8; 0x04],
    _reserved_27_prince_region1_iv: [u8; 0x04],
    _reserved_28_prince_region1_iv: [u8; 0x04],
    _reserved_29_prince_region1_iv: [u8; 0x04],
    _reserved_30_prince_region1_iv: [u8; 0x04],
    _reserved_31_prince_region1_iv: [u8; 0x04],
    _reserved_32_prince_region1_iv: [u8; 0x04],
    _reserved_33_prince_region1_iv: [u8; 0x04],
    _reserved_34_prince_region1_iv: [u8; 0x04],
    _reserved_35_prince_region1_iv: [u8; 0x04],
    _reserved_36_prince_region1_iv: [u8; 0x04],
    _reserved_37_prince_region1_iv: [u8; 0x04],
    _reserved_38_prince_region1_iv: [u8; 0x04],
    _reserved_39_prince_region2_iv: [u8; 0x04],
    _reserved_40_prince_region2_iv: [u8; 0x04],
    _reserved_41_prince_region2_iv: [u8; 0x04],
    _reserved_42_prince_region2_iv: [u8; 0x04],
    _reserved_43_prince_region2_iv: [u8; 0x04],
    _reserved_44_prince_region2_iv: [u8; 0x04],
    _reserved_45_prince_region2_iv: [u8; 0x04],
    _reserved_46_prince_region2_iv: [u8; 0x04],
    _reserved_47_prince_region2_iv: [u8; 0x04],
    _reserved_48_prince_region2_iv: [u8; 0x04],
    _reserved_49_prince_region2_iv: [u8; 0x04],
    _reserved_50_prince_region2_iv: [u8; 0x04],
    _reserved_51_prince_region2_iv: [u8; 0x04],
    _reserved_52_prince_region2_iv: [u8; 0x04],
    _reserved53: [u8; 0x28],
    #[doc = "0x100..0x1e0 - Customer Defined (Programable through ROM API)"]
    pub customer_defined: [crate::Reg<customer_defined::CUSTOMER_DEFINED_SPEC>; 56],
    #[doc = "0x1e0..0x200 - SHA256_DIGEST0 for DIGEST\\[31:0\\]
SHA256_DIGEST1 for DIGEST\\[63:32\\]
SHA256_DIGEST2 for DIGEST\\[95:64\\]
SHA256_DIGEST3 for DIGEST\\[127:96\\]
SHA256_DIGEST4 for DIGEST\\[159:128\\]
SHA256_DIGEST5 for DIGEST\\[191:160\\]
SHA256_DIGEST6 for DIGEST\\[223:192\\]
SHA256_DIGEST7 for DIGEST\\[255:224\\]"]
    pub sha256_digest: [crate::Reg<sha256_digest::SHA256_DIGEST_SPEC>; 8],
}
impl RegisterBlock {
    #[doc = "0x30 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_header0(
        &self,
    ) -> &crate::Reg<prince_region0_iv_header0::PRINCE_REGION0_IV_HEADER0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(48usize)
                as *const crate::Reg<prince_region0_iv_header0::PRINCE_REGION0_IV_HEADER0_SPEC>)
        }
    }
    #[doc = "0x30 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code0(
        &self,
    ) -> &crate::Reg<prince_region0_iv_code0::PRINCE_REGION0_IV_CODE0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(48usize)
                as *const crate::Reg<prince_region0_iv_code0::PRINCE_REGION0_IV_CODE0_SPEC>)
        }
    }
    #[doc = "0x34 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_header1(
        &self,
    ) -> &crate::Reg<prince_region0_iv_header1::PRINCE_REGION0_IV_HEADER1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(52usize)
                as *const crate::Reg<prince_region0_iv_header1::PRINCE_REGION0_IV_HEADER1_SPEC>)
        }
    }
    #[doc = "0x34 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code1(
        &self,
    ) -> &crate::Reg<prince_region0_iv_code1::PRINCE_REGION0_IV_CODE1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(52usize)
                as *const crate::Reg<prince_region0_iv_code1::PRINCE_REGION0_IV_CODE1_SPEC>)
        }
    }
    #[doc = "0x38 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code2(
        &self,
    ) -> &crate::Reg<prince_region0_iv_code2::PRINCE_REGION0_IV_CODE2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(56usize)
                as *const crate::Reg<prince_region0_iv_code2::PRINCE_REGION0_IV_CODE2_SPEC>)
        }
    }
    #[doc = "0x38 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body0(
        &self,
    ) -> &crate::Reg<prince_region0_iv_body0::PRINCE_REGION0_IV_BODY0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(56usize)
                as *const crate::Reg<prince_region0_iv_body0::PRINCE_REGION0_IV_BODY0_SPEC>)
        }
    }
    #[doc = "0x3c - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code3(
        &self,
    ) -> &crate::Reg<prince_region0_iv_code3::PRINCE_REGION0_IV_CODE3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(60usize)
                as *const crate::Reg<prince_region0_iv_code3::PRINCE_REGION0_IV_CODE3_SPEC>)
        }
    }
    #[doc = "0x3c - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body1(
        &self,
    ) -> &crate::Reg<prince_region0_iv_body1::PRINCE_REGION0_IV_BODY1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(60usize)
                as *const crate::Reg<prince_region0_iv_body1::PRINCE_REGION0_IV_BODY1_SPEC>)
        }
    }
    #[doc = "0x40 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code4(
        &self,
    ) -> &crate::Reg<prince_region0_iv_code4::PRINCE_REGION0_IV_CODE4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(64usize)
                as *const crate::Reg<prince_region0_iv_code4::PRINCE_REGION0_IV_CODE4_SPEC>)
        }
    }
    #[doc = "0x40 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body2(
        &self,
    ) -> &crate::Reg<prince_region0_iv_body2::PRINCE_REGION0_IV_BODY2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(64usize)
                as *const crate::Reg<prince_region0_iv_body2::PRINCE_REGION0_IV_BODY2_SPEC>)
        }
    }
    #[doc = "0x44 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code5(
        &self,
    ) -> &crate::Reg<prince_region0_iv_code5::PRINCE_REGION0_IV_CODE5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize)
                as *const crate::Reg<prince_region0_iv_code5::PRINCE_REGION0_IV_CODE5_SPEC>)
        }
    }
    #[doc = "0x44 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body3(
        &self,
    ) -> &crate::Reg<prince_region0_iv_body3::PRINCE_REGION0_IV_BODY3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize)
                as *const crate::Reg<prince_region0_iv_body3::PRINCE_REGION0_IV_BODY3_SPEC>)
        }
    }
    #[doc = "0x48 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code6(
        &self,
    ) -> &crate::Reg<prince_region0_iv_code6::PRINCE_REGION0_IV_CODE6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(72usize)
                as *const crate::Reg<prince_region0_iv_code6::PRINCE_REGION0_IV_CODE6_SPEC>)
        }
    }
    #[doc = "0x48 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body4(
        &self,
    ) -> &crate::Reg<prince_region0_iv_body4::PRINCE_REGION0_IV_BODY4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(72usize)
                as *const crate::Reg<prince_region0_iv_body4::PRINCE_REGION0_IV_BODY4_SPEC>)
        }
    }
    #[doc = "0x4c - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code7(
        &self,
    ) -> &crate::Reg<prince_region0_iv_code7::PRINCE_REGION0_IV_CODE7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(76usize)
                as *const crate::Reg<prince_region0_iv_code7::PRINCE_REGION0_IV_CODE7_SPEC>)
        }
    }
    #[doc = "0x4c - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body5(
        &self,
    ) -> &crate::Reg<prince_region0_iv_body5::PRINCE_REGION0_IV_BODY5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(76usize)
                as *const crate::Reg<prince_region0_iv_body5::PRINCE_REGION0_IV_BODY5_SPEC>)
        }
    }
    #[doc = "0x50 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code8(
        &self,
    ) -> &crate::Reg<prince_region0_iv_code8::PRINCE_REGION0_IV_CODE8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(80usize)
                as *const crate::Reg<prince_region0_iv_code8::PRINCE_REGION0_IV_CODE8_SPEC>)
        }
    }
    #[doc = "0x50 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body6(
        &self,
    ) -> &crate::Reg<prince_region0_iv_body6::PRINCE_REGION0_IV_BODY6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(80usize)
                as *const crate::Reg<prince_region0_iv_body6::PRINCE_REGION0_IV_BODY6_SPEC>)
        }
    }
    #[doc = "0x54 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code9(
        &self,
    ) -> &crate::Reg<prince_region0_iv_code9::PRINCE_REGION0_IV_CODE9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(84usize)
                as *const crate::Reg<prince_region0_iv_code9::PRINCE_REGION0_IV_CODE9_SPEC>)
        }
    }
    #[doc = "0x54 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body7(
        &self,
    ) -> &crate::Reg<prince_region0_iv_body7::PRINCE_REGION0_IV_BODY7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(84usize)
                as *const crate::Reg<prince_region0_iv_body7::PRINCE_REGION0_IV_BODY7_SPEC>)
        }
    }
    #[doc = "0x58 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code10(
        &self,
    ) -> &crate::Reg<prince_region0_iv_code10::PRINCE_REGION0_IV_CODE10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(88usize)
                as *const crate::Reg<prince_region0_iv_code10::PRINCE_REGION0_IV_CODE10_SPEC>)
        }
    }
    #[doc = "0x58 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body8(
        &self,
    ) -> &crate::Reg<prince_region0_iv_body8::PRINCE_REGION0_IV_BODY8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(88usize)
                as *const crate::Reg<prince_region0_iv_body8::PRINCE_REGION0_IV_BODY8_SPEC>)
        }
    }
    #[doc = "0x5c - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code11(
        &self,
    ) -> &crate::Reg<prince_region0_iv_code11::PRINCE_REGION0_IV_CODE11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(92usize)
                as *const crate::Reg<prince_region0_iv_code11::PRINCE_REGION0_IV_CODE11_SPEC>)
        }
    }
    #[doc = "0x5c - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body9(
        &self,
    ) -> &crate::Reg<prince_region0_iv_body9::PRINCE_REGION0_IV_BODY9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(92usize)
                as *const crate::Reg<prince_region0_iv_body9::PRINCE_REGION0_IV_BODY9_SPEC>)
        }
    }
    #[doc = "0x60 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code12(
        &self,
    ) -> &crate::Reg<prince_region0_iv_code12::PRINCE_REGION0_IV_CODE12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(96usize)
                as *const crate::Reg<prince_region0_iv_code12::PRINCE_REGION0_IV_CODE12_SPEC>)
        }
    }
    #[doc = "0x60 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body10(
        &self,
    ) -> &crate::Reg<prince_region0_iv_body10::PRINCE_REGION0_IV_BODY10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(96usize)
                as *const crate::Reg<prince_region0_iv_body10::PRINCE_REGION0_IV_BODY10_SPEC>)
        }
    }
    #[doc = "0x64 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code13(
        &self,
    ) -> &crate::Reg<prince_region0_iv_code13::PRINCE_REGION0_IV_CODE13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(100usize)
                as *const crate::Reg<prince_region0_iv_code13::PRINCE_REGION0_IV_CODE13_SPEC>)
        }
    }
    #[doc = "0x64 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body11(
        &self,
    ) -> &crate::Reg<prince_region0_iv_body11::PRINCE_REGION0_IV_BODY11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(100usize)
                as *const crate::Reg<prince_region0_iv_body11::PRINCE_REGION0_IV_BODY11_SPEC>)
        }
    }
    #[doc = "0x68 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_header0(
        &self,
    ) -> &crate::Reg<prince_region1_iv_header0::PRINCE_REGION1_IV_HEADER0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(104usize)
                as *const crate::Reg<prince_region1_iv_header0::PRINCE_REGION1_IV_HEADER0_SPEC>)
        }
    }
    #[doc = "0x68 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code0(
        &self,
    ) -> &crate::Reg<prince_region1_iv_code0::PRINCE_REGION1_IV_CODE0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(104usize)
                as *const crate::Reg<prince_region1_iv_code0::PRINCE_REGION1_IV_CODE0_SPEC>)
        }
    }
    #[doc = "0x6c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_header1(
        &self,
    ) -> &crate::Reg<prince_region1_iv_header1::PRINCE_REGION1_IV_HEADER1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(108usize)
                as *const crate::Reg<prince_region1_iv_header1::PRINCE_REGION1_IV_HEADER1_SPEC>)
        }
    }
    #[doc = "0x6c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code1(
        &self,
    ) -> &crate::Reg<prince_region1_iv_code1::PRINCE_REGION1_IV_CODE1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(108usize)
                as *const crate::Reg<prince_region1_iv_code1::PRINCE_REGION1_IV_CODE1_SPEC>)
        }
    }
    #[doc = "0x70 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code2(
        &self,
    ) -> &crate::Reg<prince_region1_iv_code2::PRINCE_REGION1_IV_CODE2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(112usize)
                as *const crate::Reg<prince_region1_iv_code2::PRINCE_REGION1_IV_CODE2_SPEC>)
        }
    }
    #[doc = "0x70 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body0(
        &self,
    ) -> &crate::Reg<prince_region1_iv_body0::PRINCE_REGION1_IV_BODY0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(112usize)
                as *const crate::Reg<prince_region1_iv_body0::PRINCE_REGION1_IV_BODY0_SPEC>)
        }
    }
    #[doc = "0x74 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code3(
        &self,
    ) -> &crate::Reg<prince_region1_iv_code3::PRINCE_REGION1_IV_CODE3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(116usize)
                as *const crate::Reg<prince_region1_iv_code3::PRINCE_REGION1_IV_CODE3_SPEC>)
        }
    }
    #[doc = "0x74 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body1(
        &self,
    ) -> &crate::Reg<prince_region1_iv_body1::PRINCE_REGION1_IV_BODY1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(116usize)
                as *const crate::Reg<prince_region1_iv_body1::PRINCE_REGION1_IV_BODY1_SPEC>)
        }
    }
    #[doc = "0x78 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code4(
        &self,
    ) -> &crate::Reg<prince_region1_iv_code4::PRINCE_REGION1_IV_CODE4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(120usize)
                as *const crate::Reg<prince_region1_iv_code4::PRINCE_REGION1_IV_CODE4_SPEC>)
        }
    }
    #[doc = "0x78 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body2(
        &self,
    ) -> &crate::Reg<prince_region1_iv_body2::PRINCE_REGION1_IV_BODY2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(120usize)
                as *const crate::Reg<prince_region1_iv_body2::PRINCE_REGION1_IV_BODY2_SPEC>)
        }
    }
    #[doc = "0x7c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code5(
        &self,
    ) -> &crate::Reg<prince_region1_iv_code5::PRINCE_REGION1_IV_CODE5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(124usize)
                as *const crate::Reg<prince_region1_iv_code5::PRINCE_REGION1_IV_CODE5_SPEC>)
        }
    }
    #[doc = "0x7c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body3(
        &self,
    ) -> &crate::Reg<prince_region1_iv_body3::PRINCE_REGION1_IV_BODY3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(124usize)
                as *const crate::Reg<prince_region1_iv_body3::PRINCE_REGION1_IV_BODY3_SPEC>)
        }
    }
    #[doc = "0x80 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code6(
        &self,
    ) -> &crate::Reg<prince_region1_iv_code6::PRINCE_REGION1_IV_CODE6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(128usize)
                as *const crate::Reg<prince_region1_iv_code6::PRINCE_REGION1_IV_CODE6_SPEC>)
        }
    }
    #[doc = "0x80 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body4(
        &self,
    ) -> &crate::Reg<prince_region1_iv_body4::PRINCE_REGION1_IV_BODY4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(128usize)
                as *const crate::Reg<prince_region1_iv_body4::PRINCE_REGION1_IV_BODY4_SPEC>)
        }
    }
    #[doc = "0x84 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code7(
        &self,
    ) -> &crate::Reg<prince_region1_iv_code7::PRINCE_REGION1_IV_CODE7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(132usize)
                as *const crate::Reg<prince_region1_iv_code7::PRINCE_REGION1_IV_CODE7_SPEC>)
        }
    }
    #[doc = "0x84 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body5(
        &self,
    ) -> &crate::Reg<prince_region1_iv_body5::PRINCE_REGION1_IV_BODY5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(132usize)
                as *const crate::Reg<prince_region1_iv_body5::PRINCE_REGION1_IV_BODY5_SPEC>)
        }
    }
    #[doc = "0x88 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code8(
        &self,
    ) -> &crate::Reg<prince_region1_iv_code8::PRINCE_REGION1_IV_CODE8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(136usize)
                as *const crate::Reg<prince_region1_iv_code8::PRINCE_REGION1_IV_CODE8_SPEC>)
        }
    }
    #[doc = "0x88 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body6(
        &self,
    ) -> &crate::Reg<prince_region1_iv_body6::PRINCE_REGION1_IV_BODY6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(136usize)
                as *const crate::Reg<prince_region1_iv_body6::PRINCE_REGION1_IV_BODY6_SPEC>)
        }
    }
    #[doc = "0x8c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code9(
        &self,
    ) -> &crate::Reg<prince_region1_iv_code9::PRINCE_REGION1_IV_CODE9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(140usize)
                as *const crate::Reg<prince_region1_iv_code9::PRINCE_REGION1_IV_CODE9_SPEC>)
        }
    }
    #[doc = "0x8c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body7(
        &self,
    ) -> &crate::Reg<prince_region1_iv_body7::PRINCE_REGION1_IV_BODY7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(140usize)
                as *const crate::Reg<prince_region1_iv_body7::PRINCE_REGION1_IV_BODY7_SPEC>)
        }
    }
    #[doc = "0x90 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code10(
        &self,
    ) -> &crate::Reg<prince_region1_iv_code10::PRINCE_REGION1_IV_CODE10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(144usize)
                as *const crate::Reg<prince_region1_iv_code10::PRINCE_REGION1_IV_CODE10_SPEC>)
        }
    }
    #[doc = "0x90 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body8(
        &self,
    ) -> &crate::Reg<prince_region1_iv_body8::PRINCE_REGION1_IV_BODY8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(144usize)
                as *const crate::Reg<prince_region1_iv_body8::PRINCE_REGION1_IV_BODY8_SPEC>)
        }
    }
    #[doc = "0x94 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code11(
        &self,
    ) -> &crate::Reg<prince_region1_iv_code11::PRINCE_REGION1_IV_CODE11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(148usize)
                as *const crate::Reg<prince_region1_iv_code11::PRINCE_REGION1_IV_CODE11_SPEC>)
        }
    }
    #[doc = "0x94 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body9(
        &self,
    ) -> &crate::Reg<prince_region1_iv_body9::PRINCE_REGION1_IV_BODY9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(148usize)
                as *const crate::Reg<prince_region1_iv_body9::PRINCE_REGION1_IV_BODY9_SPEC>)
        }
    }
    #[doc = "0x98 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code12(
        &self,
    ) -> &crate::Reg<prince_region1_iv_code12::PRINCE_REGION1_IV_CODE12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(152usize)
                as *const crate::Reg<prince_region1_iv_code12::PRINCE_REGION1_IV_CODE12_SPEC>)
        }
    }
    #[doc = "0x98 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body10(
        &self,
    ) -> &crate::Reg<prince_region1_iv_body10::PRINCE_REGION1_IV_BODY10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(152usize)
                as *const crate::Reg<prince_region1_iv_body10::PRINCE_REGION1_IV_BODY10_SPEC>)
        }
    }
    #[doc = "0x9c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code13(
        &self,
    ) -> &crate::Reg<prince_region1_iv_code13::PRINCE_REGION1_IV_CODE13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(156usize)
                as *const crate::Reg<prince_region1_iv_code13::PRINCE_REGION1_IV_CODE13_SPEC>)
        }
    }
    #[doc = "0x9c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body11(
        &self,
    ) -> &crate::Reg<prince_region1_iv_body11::PRINCE_REGION1_IV_BODY11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(156usize)
                as *const crate::Reg<prince_region1_iv_body11::PRINCE_REGION1_IV_BODY11_SPEC>)
        }
    }
    #[doc = "0xa0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_header0(
        &self,
    ) -> &crate::Reg<prince_region2_iv_header0::PRINCE_REGION2_IV_HEADER0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(160usize)
                as *const crate::Reg<prince_region2_iv_header0::PRINCE_REGION2_IV_HEADER0_SPEC>)
        }
    }
    #[doc = "0xa0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code0(
        &self,
    ) -> &crate::Reg<prince_region2_iv_code0::PRINCE_REGION2_IV_CODE0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(160usize)
                as *const crate::Reg<prince_region2_iv_code0::PRINCE_REGION2_IV_CODE0_SPEC>)
        }
    }
    #[doc = "0xa4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_header1(
        &self,
    ) -> &crate::Reg<prince_region2_iv_header1::PRINCE_REGION2_IV_HEADER1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(164usize)
                as *const crate::Reg<prince_region2_iv_header1::PRINCE_REGION2_IV_HEADER1_SPEC>)
        }
    }
    #[doc = "0xa4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code1(
        &self,
    ) -> &crate::Reg<prince_region2_iv_code1::PRINCE_REGION2_IV_CODE1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(164usize)
                as *const crate::Reg<prince_region2_iv_code1::PRINCE_REGION2_IV_CODE1_SPEC>)
        }
    }
    #[doc = "0xa8 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code2(
        &self,
    ) -> &crate::Reg<prince_region2_iv_code2::PRINCE_REGION2_IV_CODE2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(168usize)
                as *const crate::Reg<prince_region2_iv_code2::PRINCE_REGION2_IV_CODE2_SPEC>)
        }
    }
    #[doc = "0xa8 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body0(
        &self,
    ) -> &crate::Reg<prince_region2_iv_body0::PRINCE_REGION2_IV_BODY0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(168usize)
                as *const crate::Reg<prince_region2_iv_body0::PRINCE_REGION2_IV_BODY0_SPEC>)
        }
    }
    #[doc = "0xac - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code3(
        &self,
    ) -> &crate::Reg<prince_region2_iv_code3::PRINCE_REGION2_IV_CODE3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(172usize)
                as *const crate::Reg<prince_region2_iv_code3::PRINCE_REGION2_IV_CODE3_SPEC>)
        }
    }
    #[doc = "0xac - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body1(
        &self,
    ) -> &crate::Reg<prince_region2_iv_body1::PRINCE_REGION2_IV_BODY1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(172usize)
                as *const crate::Reg<prince_region2_iv_body1::PRINCE_REGION2_IV_BODY1_SPEC>)
        }
    }
    #[doc = "0xb0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code4(
        &self,
    ) -> &crate::Reg<prince_region2_iv_code4::PRINCE_REGION2_IV_CODE4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(176usize)
                as *const crate::Reg<prince_region2_iv_code4::PRINCE_REGION2_IV_CODE4_SPEC>)
        }
    }
    #[doc = "0xb0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body2(
        &self,
    ) -> &crate::Reg<prince_region2_iv_body2::PRINCE_REGION2_IV_BODY2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(176usize)
                as *const crate::Reg<prince_region2_iv_body2::PRINCE_REGION2_IV_BODY2_SPEC>)
        }
    }
    #[doc = "0xb4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code5(
        &self,
    ) -> &crate::Reg<prince_region2_iv_code5::PRINCE_REGION2_IV_CODE5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(180usize)
                as *const crate::Reg<prince_region2_iv_code5::PRINCE_REGION2_IV_CODE5_SPEC>)
        }
    }
    #[doc = "0xb4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body3(
        &self,
    ) -> &crate::Reg<prince_region2_iv_body3::PRINCE_REGION2_IV_BODY3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(180usize)
                as *const crate::Reg<prince_region2_iv_body3::PRINCE_REGION2_IV_BODY3_SPEC>)
        }
    }
    #[doc = "0xb8 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code6(
        &self,
    ) -> &crate::Reg<prince_region2_iv_code6::PRINCE_REGION2_IV_CODE6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(184usize)
                as *const crate::Reg<prince_region2_iv_code6::PRINCE_REGION2_IV_CODE6_SPEC>)
        }
    }
    #[doc = "0xb8 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body4(
        &self,
    ) -> &crate::Reg<prince_region2_iv_body4::PRINCE_REGION2_IV_BODY4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(184usize)
                as *const crate::Reg<prince_region2_iv_body4::PRINCE_REGION2_IV_BODY4_SPEC>)
        }
    }
    #[doc = "0xbc - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code7(
        &self,
    ) -> &crate::Reg<prince_region2_iv_code7::PRINCE_REGION2_IV_CODE7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(188usize)
                as *const crate::Reg<prince_region2_iv_code7::PRINCE_REGION2_IV_CODE7_SPEC>)
        }
    }
    #[doc = "0xbc - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body5(
        &self,
    ) -> &crate::Reg<prince_region2_iv_body5::PRINCE_REGION2_IV_BODY5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(188usize)
                as *const crate::Reg<prince_region2_iv_body5::PRINCE_REGION2_IV_BODY5_SPEC>)
        }
    }
    #[doc = "0xc0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code8(
        &self,
    ) -> &crate::Reg<prince_region2_iv_code8::PRINCE_REGION2_IV_CODE8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(192usize)
                as *const crate::Reg<prince_region2_iv_code8::PRINCE_REGION2_IV_CODE8_SPEC>)
        }
    }
    #[doc = "0xc0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body6(
        &self,
    ) -> &crate::Reg<prince_region2_iv_body6::PRINCE_REGION2_IV_BODY6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(192usize)
                as *const crate::Reg<prince_region2_iv_body6::PRINCE_REGION2_IV_BODY6_SPEC>)
        }
    }
    #[doc = "0xc4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code9(
        &self,
    ) -> &crate::Reg<prince_region2_iv_code9::PRINCE_REGION2_IV_CODE9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(196usize)
                as *const crate::Reg<prince_region2_iv_code9::PRINCE_REGION2_IV_CODE9_SPEC>)
        }
    }
    #[doc = "0xc4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body7(
        &self,
    ) -> &crate::Reg<prince_region2_iv_body7::PRINCE_REGION2_IV_BODY7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(196usize)
                as *const crate::Reg<prince_region2_iv_body7::PRINCE_REGION2_IV_BODY7_SPEC>)
        }
    }
    #[doc = "0xc8 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code10(
        &self,
    ) -> &crate::Reg<prince_region2_iv_code10::PRINCE_REGION2_IV_CODE10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(200usize)
                as *const crate::Reg<prince_region2_iv_code10::PRINCE_REGION2_IV_CODE10_SPEC>)
        }
    }
    #[doc = "0xc8 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body8(
        &self,
    ) -> &crate::Reg<prince_region2_iv_body8::PRINCE_REGION2_IV_BODY8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(200usize)
                as *const crate::Reg<prince_region2_iv_body8::PRINCE_REGION2_IV_BODY8_SPEC>)
        }
    }
    #[doc = "0xcc - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code11(
        &self,
    ) -> &crate::Reg<prince_region2_iv_code11::PRINCE_REGION2_IV_CODE11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(204usize)
                as *const crate::Reg<prince_region2_iv_code11::PRINCE_REGION2_IV_CODE11_SPEC>)
        }
    }
    #[doc = "0xcc - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body9(
        &self,
    ) -> &crate::Reg<prince_region2_iv_body9::PRINCE_REGION2_IV_BODY9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(204usize)
                as *const crate::Reg<prince_region2_iv_body9::PRINCE_REGION2_IV_BODY9_SPEC>)
        }
    }
    #[doc = "0xd0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code12(
        &self,
    ) -> &crate::Reg<prince_region2_iv_code12::PRINCE_REGION2_IV_CODE12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(208usize)
                as *const crate::Reg<prince_region2_iv_code12::PRINCE_REGION2_IV_CODE12_SPEC>)
        }
    }
    #[doc = "0xd0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body10(
        &self,
    ) -> &crate::Reg<prince_region2_iv_body10::PRINCE_REGION2_IV_BODY10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(208usize)
                as *const crate::Reg<prince_region2_iv_body10::PRINCE_REGION2_IV_BODY10_SPEC>)
        }
    }
    #[doc = "0xd4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code13(
        &self,
    ) -> &crate::Reg<prince_region2_iv_code13::PRINCE_REGION2_IV_CODE13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(212usize)
                as *const crate::Reg<prince_region2_iv_code13::PRINCE_REGION2_IV_CODE13_SPEC>)
        }
    }
    #[doc = "0xd4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body11(
        &self,
    ) -> &crate::Reg<prince_region2_iv_body11::PRINCE_REGION2_IV_BODY11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(212usize)
                as *const crate::Reg<prince_region2_iv_body11::PRINCE_REGION2_IV_BODY11_SPEC>)
        }
    }
}
#[doc = "HEADER register accessor: an alias for `Reg<HEADER_SPEC>`"]
pub type HEADER = crate::Reg<header::HEADER_SPEC>;
#[doc = "."]
pub mod header;
#[doc = "VERSION register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "."]
pub mod version;
#[doc = "S_FW_Version register accessor: an alias for `Reg<S_FW_VERSION_SPEC>`"]
pub type S_FW_VERSION = crate::Reg<s_fw_version::S_FW_VERSION_SPEC>;
#[doc = "Secure firmware version (Monotonic counter)"]
pub mod s_fw_version;
#[doc = "NS_FW_Version register accessor: an alias for `Reg<NS_FW_VERSION_SPEC>`"]
pub type NS_FW_VERSION = crate::Reg<ns_fw_version::NS_FW_VERSION_SPEC>;
#[doc = "Non-Secure firmware version (Monotonic counter)"]
pub mod ns_fw_version;
#[doc = "IMAGE_KEY_REVOKE register accessor: an alias for `Reg<IMAGE_KEY_REVOKE_SPEC>`"]
pub type IMAGE_KEY_REVOKE = crate::Reg<image_key_revoke::IMAGE_KEY_REVOKE_SPEC>;
#[doc = "Image key revocation ID (Monotonic counter)"]
pub mod image_key_revoke;
#[doc = "ROTKH_REVOKE register accessor: an alias for `Reg<ROTKH_REVOKE_SPEC>`"]
pub type ROTKH_REVOKE = crate::Reg<rotkh_revoke::ROTKH_REVOKE_SPEC>;
#[doc = "."]
pub mod rotkh_revoke;
#[doc = "VENDOR_USAGE register accessor: an alias for `Reg<VENDOR_USAGE_SPEC>`"]
pub type VENDOR_USAGE = crate::Reg<vendor_usage::VENDOR_USAGE_SPEC>;
#[doc = "."]
pub mod vendor_usage;
#[doc = "DCFG_CC_SOCU_PIN register accessor: an alias for `Reg<DCFG_CC_SOCU_PIN_SPEC>`"]
pub type DCFG_CC_SOCU_PIN = crate::Reg<dcfg_cc_socu_pin::DCFG_CC_SOCU_PIN_SPEC>;
#[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
pub mod dcfg_cc_socu_pin;
#[doc = "DCFG_CC_SOCU_DFLT register accessor: an alias for `Reg<DCFG_CC_SOCU_DFLT_SPEC>`"]
pub type DCFG_CC_SOCU_DFLT = crate::Reg<dcfg_cc_socu_dflt::DCFG_CC_SOCU_DFLT_SPEC>;
#[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
pub mod dcfg_cc_socu_dflt;
#[doc = "ENABLE_FA_MODE register accessor: an alias for `Reg<ENABLE_FA_MODE_SPEC>`"]
pub type ENABLE_FA_MODE = crate::Reg<enable_fa_mode::ENABLE_FA_MODE_SPEC>;
#[doc = "Enable FA mode. SET_FA_MODE Command should write 0xC33CA55A to this word to indicate boot ROM to enter FA mode."]
pub mod enable_fa_mode;
#[doc = "CMPA_PROG_IN_PROGRESS register accessor: an alias for `Reg<CMPA_PROG_IN_PROGRESS_SPEC>`"]
pub type CMPA_PROG_IN_PROGRESS = crate::Reg<cmpa_prog_in_progress::CMPA_PROG_IN_PROGRESS_SPEC>;
#[doc = "CMPA Page programming on going. This field shall be set to 0x5CC55AA5 in the active CFPA page each time CMPA page programming is going on. It shall always be set to 0x00000000 in the CFPA scratch area."]
pub mod cmpa_prog_in_progress;
#[doc = "PRINCE_REGION0_IV_CODE0 register accessor: an alias for `Reg<PRINCE_REGION0_IV_CODE0_SPEC>`"]
pub type PRINCE_REGION0_IV_CODE0 =
    crate::Reg<prince_region0_iv_code0::PRINCE_REGION0_IV_CODE0_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_code0;
#[doc = "PRINCE_REGION0_IV_HEADER0 register accessor: an alias for `Reg<PRINCE_REGION0_IV_HEADER0_SPEC>`"]
pub type PRINCE_REGION0_IV_HEADER0 =
    crate::Reg<prince_region0_iv_header0::PRINCE_REGION0_IV_HEADER0_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_header0;
#[doc = "PRINCE_REGION0_IV_CODE1 register accessor: an alias for `Reg<PRINCE_REGION0_IV_CODE1_SPEC>`"]
pub type PRINCE_REGION0_IV_CODE1 =
    crate::Reg<prince_region0_iv_code1::PRINCE_REGION0_IV_CODE1_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_code1;
#[doc = "PRINCE_REGION0_IV_HEADER1 register accessor: an alias for `Reg<PRINCE_REGION0_IV_HEADER1_SPEC>`"]
pub type PRINCE_REGION0_IV_HEADER1 =
    crate::Reg<prince_region0_iv_header1::PRINCE_REGION0_IV_HEADER1_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_header1;
#[doc = "PRINCE_REGION0_IV_BODY0 register accessor: an alias for `Reg<PRINCE_REGION0_IV_BODY0_SPEC>`"]
pub type PRINCE_REGION0_IV_BODY0 =
    crate::Reg<prince_region0_iv_body0::PRINCE_REGION0_IV_BODY0_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_body0;
#[doc = "PRINCE_REGION0_IV_CODE2 register accessor: an alias for `Reg<PRINCE_REGION0_IV_CODE2_SPEC>`"]
pub type PRINCE_REGION0_IV_CODE2 =
    crate::Reg<prince_region0_iv_code2::PRINCE_REGION0_IV_CODE2_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_code2;
#[doc = "PRINCE_REGION0_IV_BODY1 register accessor: an alias for `Reg<PRINCE_REGION0_IV_BODY1_SPEC>`"]
pub type PRINCE_REGION0_IV_BODY1 =
    crate::Reg<prince_region0_iv_body1::PRINCE_REGION0_IV_BODY1_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_body1;
#[doc = "PRINCE_REGION0_IV_CODE3 register accessor: an alias for `Reg<PRINCE_REGION0_IV_CODE3_SPEC>`"]
pub type PRINCE_REGION0_IV_CODE3 =
    crate::Reg<prince_region0_iv_code3::PRINCE_REGION0_IV_CODE3_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_code3;
#[doc = "PRINCE_REGION0_IV_BODY2 register accessor: an alias for `Reg<PRINCE_REGION0_IV_BODY2_SPEC>`"]
pub type PRINCE_REGION0_IV_BODY2 =
    crate::Reg<prince_region0_iv_body2::PRINCE_REGION0_IV_BODY2_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_body2;
#[doc = "PRINCE_REGION0_IV_CODE4 register accessor: an alias for `Reg<PRINCE_REGION0_IV_CODE4_SPEC>`"]
pub type PRINCE_REGION0_IV_CODE4 =
    crate::Reg<prince_region0_iv_code4::PRINCE_REGION0_IV_CODE4_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_code4;
#[doc = "PRINCE_REGION0_IV_BODY3 register accessor: an alias for `Reg<PRINCE_REGION0_IV_BODY3_SPEC>`"]
pub type PRINCE_REGION0_IV_BODY3 =
    crate::Reg<prince_region0_iv_body3::PRINCE_REGION0_IV_BODY3_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_body3;
#[doc = "PRINCE_REGION0_IV_CODE5 register accessor: an alias for `Reg<PRINCE_REGION0_IV_CODE5_SPEC>`"]
pub type PRINCE_REGION0_IV_CODE5 =
    crate::Reg<prince_region0_iv_code5::PRINCE_REGION0_IV_CODE5_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_code5;
#[doc = "PRINCE_REGION0_IV_BODY4 register accessor: an alias for `Reg<PRINCE_REGION0_IV_BODY4_SPEC>`"]
pub type PRINCE_REGION0_IV_BODY4 =
    crate::Reg<prince_region0_iv_body4::PRINCE_REGION0_IV_BODY4_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_body4;
#[doc = "PRINCE_REGION0_IV_CODE6 register accessor: an alias for `Reg<PRINCE_REGION0_IV_CODE6_SPEC>`"]
pub type PRINCE_REGION0_IV_CODE6 =
    crate::Reg<prince_region0_iv_code6::PRINCE_REGION0_IV_CODE6_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_code6;
#[doc = "PRINCE_REGION0_IV_BODY5 register accessor: an alias for `Reg<PRINCE_REGION0_IV_BODY5_SPEC>`"]
pub type PRINCE_REGION0_IV_BODY5 =
    crate::Reg<prince_region0_iv_body5::PRINCE_REGION0_IV_BODY5_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_body5;
#[doc = "PRINCE_REGION0_IV_CODE7 register accessor: an alias for `Reg<PRINCE_REGION0_IV_CODE7_SPEC>`"]
pub type PRINCE_REGION0_IV_CODE7 =
    crate::Reg<prince_region0_iv_code7::PRINCE_REGION0_IV_CODE7_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_code7;
#[doc = "PRINCE_REGION0_IV_BODY6 register accessor: an alias for `Reg<PRINCE_REGION0_IV_BODY6_SPEC>`"]
pub type PRINCE_REGION0_IV_BODY6 =
    crate::Reg<prince_region0_iv_body6::PRINCE_REGION0_IV_BODY6_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_body6;
#[doc = "PRINCE_REGION0_IV_CODE8 register accessor: an alias for `Reg<PRINCE_REGION0_IV_CODE8_SPEC>`"]
pub type PRINCE_REGION0_IV_CODE8 =
    crate::Reg<prince_region0_iv_code8::PRINCE_REGION0_IV_CODE8_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_code8;
#[doc = "PRINCE_REGION0_IV_BODY7 register accessor: an alias for `Reg<PRINCE_REGION0_IV_BODY7_SPEC>`"]
pub type PRINCE_REGION0_IV_BODY7 =
    crate::Reg<prince_region0_iv_body7::PRINCE_REGION0_IV_BODY7_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_body7;
#[doc = "PRINCE_REGION0_IV_CODE9 register accessor: an alias for `Reg<PRINCE_REGION0_IV_CODE9_SPEC>`"]
pub type PRINCE_REGION0_IV_CODE9 =
    crate::Reg<prince_region0_iv_code9::PRINCE_REGION0_IV_CODE9_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_code9;
#[doc = "PRINCE_REGION0_IV_BODY8 register accessor: an alias for `Reg<PRINCE_REGION0_IV_BODY8_SPEC>`"]
pub type PRINCE_REGION0_IV_BODY8 =
    crate::Reg<prince_region0_iv_body8::PRINCE_REGION0_IV_BODY8_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_body8;
#[doc = "PRINCE_REGION0_IV_CODE10 register accessor: an alias for `Reg<PRINCE_REGION0_IV_CODE10_SPEC>`"]
pub type PRINCE_REGION0_IV_CODE10 =
    crate::Reg<prince_region0_iv_code10::PRINCE_REGION0_IV_CODE10_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_code10;
#[doc = "PRINCE_REGION0_IV_BODY9 register accessor: an alias for `Reg<PRINCE_REGION0_IV_BODY9_SPEC>`"]
pub type PRINCE_REGION0_IV_BODY9 =
    crate::Reg<prince_region0_iv_body9::PRINCE_REGION0_IV_BODY9_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_body9;
#[doc = "PRINCE_REGION0_IV_CODE11 register accessor: an alias for `Reg<PRINCE_REGION0_IV_CODE11_SPEC>`"]
pub type PRINCE_REGION0_IV_CODE11 =
    crate::Reg<prince_region0_iv_code11::PRINCE_REGION0_IV_CODE11_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_code11;
#[doc = "PRINCE_REGION0_IV_BODY10 register accessor: an alias for `Reg<PRINCE_REGION0_IV_BODY10_SPEC>`"]
pub type PRINCE_REGION0_IV_BODY10 =
    crate::Reg<prince_region0_iv_body10::PRINCE_REGION0_IV_BODY10_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_body10;
#[doc = "PRINCE_REGION0_IV_CODE12 register accessor: an alias for `Reg<PRINCE_REGION0_IV_CODE12_SPEC>`"]
pub type PRINCE_REGION0_IV_CODE12 =
    crate::Reg<prince_region0_iv_code12::PRINCE_REGION0_IV_CODE12_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_code12;
#[doc = "PRINCE_REGION0_IV_BODY11 register accessor: an alias for `Reg<PRINCE_REGION0_IV_BODY11_SPEC>`"]
pub type PRINCE_REGION0_IV_BODY11 =
    crate::Reg<prince_region0_iv_body11::PRINCE_REGION0_IV_BODY11_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_body11;
#[doc = "PRINCE_REGION0_IV_CODE13 register accessor: an alias for `Reg<PRINCE_REGION0_IV_CODE13_SPEC>`"]
pub type PRINCE_REGION0_IV_CODE13 =
    crate::Reg<prince_region0_iv_code13::PRINCE_REGION0_IV_CODE13_SPEC>;
#[doc = "."]
pub mod prince_region0_iv_code13;
#[doc = "PRINCE_REGION1_IV_CODE0 register accessor: an alias for `Reg<PRINCE_REGION1_IV_CODE0_SPEC>`"]
pub type PRINCE_REGION1_IV_CODE0 =
    crate::Reg<prince_region1_iv_code0::PRINCE_REGION1_IV_CODE0_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_code0;
#[doc = "PRINCE_REGION1_IV_HEADER0 register accessor: an alias for `Reg<PRINCE_REGION1_IV_HEADER0_SPEC>`"]
pub type PRINCE_REGION1_IV_HEADER0 =
    crate::Reg<prince_region1_iv_header0::PRINCE_REGION1_IV_HEADER0_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_header0;
#[doc = "PRINCE_REGION1_IV_CODE1 register accessor: an alias for `Reg<PRINCE_REGION1_IV_CODE1_SPEC>`"]
pub type PRINCE_REGION1_IV_CODE1 =
    crate::Reg<prince_region1_iv_code1::PRINCE_REGION1_IV_CODE1_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_code1;
#[doc = "PRINCE_REGION1_IV_HEADER1 register accessor: an alias for `Reg<PRINCE_REGION1_IV_HEADER1_SPEC>`"]
pub type PRINCE_REGION1_IV_HEADER1 =
    crate::Reg<prince_region1_iv_header1::PRINCE_REGION1_IV_HEADER1_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_header1;
#[doc = "PRINCE_REGION1_IV_BODY0 register accessor: an alias for `Reg<PRINCE_REGION1_IV_BODY0_SPEC>`"]
pub type PRINCE_REGION1_IV_BODY0 =
    crate::Reg<prince_region1_iv_body0::PRINCE_REGION1_IV_BODY0_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_body0;
#[doc = "PRINCE_REGION1_IV_CODE2 register accessor: an alias for `Reg<PRINCE_REGION1_IV_CODE2_SPEC>`"]
pub type PRINCE_REGION1_IV_CODE2 =
    crate::Reg<prince_region1_iv_code2::PRINCE_REGION1_IV_CODE2_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_code2;
#[doc = "PRINCE_REGION1_IV_BODY1 register accessor: an alias for `Reg<PRINCE_REGION1_IV_BODY1_SPEC>`"]
pub type PRINCE_REGION1_IV_BODY1 =
    crate::Reg<prince_region1_iv_body1::PRINCE_REGION1_IV_BODY1_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_body1;
#[doc = "PRINCE_REGION1_IV_CODE3 register accessor: an alias for `Reg<PRINCE_REGION1_IV_CODE3_SPEC>`"]
pub type PRINCE_REGION1_IV_CODE3 =
    crate::Reg<prince_region1_iv_code3::PRINCE_REGION1_IV_CODE3_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_code3;
#[doc = "PRINCE_REGION1_IV_BODY2 register accessor: an alias for `Reg<PRINCE_REGION1_IV_BODY2_SPEC>`"]
pub type PRINCE_REGION1_IV_BODY2 =
    crate::Reg<prince_region1_iv_body2::PRINCE_REGION1_IV_BODY2_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_body2;
#[doc = "PRINCE_REGION1_IV_CODE4 register accessor: an alias for `Reg<PRINCE_REGION1_IV_CODE4_SPEC>`"]
pub type PRINCE_REGION1_IV_CODE4 =
    crate::Reg<prince_region1_iv_code4::PRINCE_REGION1_IV_CODE4_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_code4;
#[doc = "PRINCE_REGION1_IV_BODY3 register accessor: an alias for `Reg<PRINCE_REGION1_IV_BODY3_SPEC>`"]
pub type PRINCE_REGION1_IV_BODY3 =
    crate::Reg<prince_region1_iv_body3::PRINCE_REGION1_IV_BODY3_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_body3;
#[doc = "PRINCE_REGION1_IV_CODE5 register accessor: an alias for `Reg<PRINCE_REGION1_IV_CODE5_SPEC>`"]
pub type PRINCE_REGION1_IV_CODE5 =
    crate::Reg<prince_region1_iv_code5::PRINCE_REGION1_IV_CODE5_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_code5;
#[doc = "PRINCE_REGION1_IV_BODY4 register accessor: an alias for `Reg<PRINCE_REGION1_IV_BODY4_SPEC>`"]
pub type PRINCE_REGION1_IV_BODY4 =
    crate::Reg<prince_region1_iv_body4::PRINCE_REGION1_IV_BODY4_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_body4;
#[doc = "PRINCE_REGION1_IV_CODE6 register accessor: an alias for `Reg<PRINCE_REGION1_IV_CODE6_SPEC>`"]
pub type PRINCE_REGION1_IV_CODE6 =
    crate::Reg<prince_region1_iv_code6::PRINCE_REGION1_IV_CODE6_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_code6;
#[doc = "PRINCE_REGION1_IV_BODY5 register accessor: an alias for `Reg<PRINCE_REGION1_IV_BODY5_SPEC>`"]
pub type PRINCE_REGION1_IV_BODY5 =
    crate::Reg<prince_region1_iv_body5::PRINCE_REGION1_IV_BODY5_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_body5;
#[doc = "PRINCE_REGION1_IV_CODE7 register accessor: an alias for `Reg<PRINCE_REGION1_IV_CODE7_SPEC>`"]
pub type PRINCE_REGION1_IV_CODE7 =
    crate::Reg<prince_region1_iv_code7::PRINCE_REGION1_IV_CODE7_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_code7;
#[doc = "PRINCE_REGION1_IV_BODY6 register accessor: an alias for `Reg<PRINCE_REGION1_IV_BODY6_SPEC>`"]
pub type PRINCE_REGION1_IV_BODY6 =
    crate::Reg<prince_region1_iv_body6::PRINCE_REGION1_IV_BODY6_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_body6;
#[doc = "PRINCE_REGION1_IV_CODE8 register accessor: an alias for `Reg<PRINCE_REGION1_IV_CODE8_SPEC>`"]
pub type PRINCE_REGION1_IV_CODE8 =
    crate::Reg<prince_region1_iv_code8::PRINCE_REGION1_IV_CODE8_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_code8;
#[doc = "PRINCE_REGION1_IV_BODY7 register accessor: an alias for `Reg<PRINCE_REGION1_IV_BODY7_SPEC>`"]
pub type PRINCE_REGION1_IV_BODY7 =
    crate::Reg<prince_region1_iv_body7::PRINCE_REGION1_IV_BODY7_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_body7;
#[doc = "PRINCE_REGION1_IV_CODE9 register accessor: an alias for `Reg<PRINCE_REGION1_IV_CODE9_SPEC>`"]
pub type PRINCE_REGION1_IV_CODE9 =
    crate::Reg<prince_region1_iv_code9::PRINCE_REGION1_IV_CODE9_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_code9;
#[doc = "PRINCE_REGION1_IV_BODY8 register accessor: an alias for `Reg<PRINCE_REGION1_IV_BODY8_SPEC>`"]
pub type PRINCE_REGION1_IV_BODY8 =
    crate::Reg<prince_region1_iv_body8::PRINCE_REGION1_IV_BODY8_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_body8;
#[doc = "PRINCE_REGION1_IV_CODE10 register accessor: an alias for `Reg<PRINCE_REGION1_IV_CODE10_SPEC>`"]
pub type PRINCE_REGION1_IV_CODE10 =
    crate::Reg<prince_region1_iv_code10::PRINCE_REGION1_IV_CODE10_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_code10;
#[doc = "PRINCE_REGION1_IV_BODY9 register accessor: an alias for `Reg<PRINCE_REGION1_IV_BODY9_SPEC>`"]
pub type PRINCE_REGION1_IV_BODY9 =
    crate::Reg<prince_region1_iv_body9::PRINCE_REGION1_IV_BODY9_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_body9;
#[doc = "PRINCE_REGION1_IV_CODE11 register accessor: an alias for `Reg<PRINCE_REGION1_IV_CODE11_SPEC>`"]
pub type PRINCE_REGION1_IV_CODE11 =
    crate::Reg<prince_region1_iv_code11::PRINCE_REGION1_IV_CODE11_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_code11;
#[doc = "PRINCE_REGION1_IV_BODY10 register accessor: an alias for `Reg<PRINCE_REGION1_IV_BODY10_SPEC>`"]
pub type PRINCE_REGION1_IV_BODY10 =
    crate::Reg<prince_region1_iv_body10::PRINCE_REGION1_IV_BODY10_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_body10;
#[doc = "PRINCE_REGION1_IV_CODE12 register accessor: an alias for `Reg<PRINCE_REGION1_IV_CODE12_SPEC>`"]
pub type PRINCE_REGION1_IV_CODE12 =
    crate::Reg<prince_region1_iv_code12::PRINCE_REGION1_IV_CODE12_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_code12;
#[doc = "PRINCE_REGION1_IV_BODY11 register accessor: an alias for `Reg<PRINCE_REGION1_IV_BODY11_SPEC>`"]
pub type PRINCE_REGION1_IV_BODY11 =
    crate::Reg<prince_region1_iv_body11::PRINCE_REGION1_IV_BODY11_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_body11;
#[doc = "PRINCE_REGION1_IV_CODE13 register accessor: an alias for `Reg<PRINCE_REGION1_IV_CODE13_SPEC>`"]
pub type PRINCE_REGION1_IV_CODE13 =
    crate::Reg<prince_region1_iv_code13::PRINCE_REGION1_IV_CODE13_SPEC>;
#[doc = "."]
pub mod prince_region1_iv_code13;
#[doc = "PRINCE_REGION2_IV_CODE0 register accessor: an alias for `Reg<PRINCE_REGION2_IV_CODE0_SPEC>`"]
pub type PRINCE_REGION2_IV_CODE0 =
    crate::Reg<prince_region2_iv_code0::PRINCE_REGION2_IV_CODE0_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_code0;
#[doc = "PRINCE_REGION2_IV_HEADER0 register accessor: an alias for `Reg<PRINCE_REGION2_IV_HEADER0_SPEC>`"]
pub type PRINCE_REGION2_IV_HEADER0 =
    crate::Reg<prince_region2_iv_header0::PRINCE_REGION2_IV_HEADER0_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_header0;
#[doc = "PRINCE_REGION2_IV_CODE1 register accessor: an alias for `Reg<PRINCE_REGION2_IV_CODE1_SPEC>`"]
pub type PRINCE_REGION2_IV_CODE1 =
    crate::Reg<prince_region2_iv_code1::PRINCE_REGION2_IV_CODE1_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_code1;
#[doc = "PRINCE_REGION2_IV_HEADER1 register accessor: an alias for `Reg<PRINCE_REGION2_IV_HEADER1_SPEC>`"]
pub type PRINCE_REGION2_IV_HEADER1 =
    crate::Reg<prince_region2_iv_header1::PRINCE_REGION2_IV_HEADER1_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_header1;
#[doc = "PRINCE_REGION2_IV_BODY0 register accessor: an alias for `Reg<PRINCE_REGION2_IV_BODY0_SPEC>`"]
pub type PRINCE_REGION2_IV_BODY0 =
    crate::Reg<prince_region2_iv_body0::PRINCE_REGION2_IV_BODY0_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_body0;
#[doc = "PRINCE_REGION2_IV_CODE2 register accessor: an alias for `Reg<PRINCE_REGION2_IV_CODE2_SPEC>`"]
pub type PRINCE_REGION2_IV_CODE2 =
    crate::Reg<prince_region2_iv_code2::PRINCE_REGION2_IV_CODE2_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_code2;
#[doc = "PRINCE_REGION2_IV_BODY1 register accessor: an alias for `Reg<PRINCE_REGION2_IV_BODY1_SPEC>`"]
pub type PRINCE_REGION2_IV_BODY1 =
    crate::Reg<prince_region2_iv_body1::PRINCE_REGION2_IV_BODY1_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_body1;
#[doc = "PRINCE_REGION2_IV_CODE3 register accessor: an alias for `Reg<PRINCE_REGION2_IV_CODE3_SPEC>`"]
pub type PRINCE_REGION2_IV_CODE3 =
    crate::Reg<prince_region2_iv_code3::PRINCE_REGION2_IV_CODE3_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_code3;
#[doc = "PRINCE_REGION2_IV_BODY2 register accessor: an alias for `Reg<PRINCE_REGION2_IV_BODY2_SPEC>`"]
pub type PRINCE_REGION2_IV_BODY2 =
    crate::Reg<prince_region2_iv_body2::PRINCE_REGION2_IV_BODY2_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_body2;
#[doc = "PRINCE_REGION2_IV_CODE4 register accessor: an alias for `Reg<PRINCE_REGION2_IV_CODE4_SPEC>`"]
pub type PRINCE_REGION2_IV_CODE4 =
    crate::Reg<prince_region2_iv_code4::PRINCE_REGION2_IV_CODE4_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_code4;
#[doc = "PRINCE_REGION2_IV_BODY3 register accessor: an alias for `Reg<PRINCE_REGION2_IV_BODY3_SPEC>`"]
pub type PRINCE_REGION2_IV_BODY3 =
    crate::Reg<prince_region2_iv_body3::PRINCE_REGION2_IV_BODY3_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_body3;
#[doc = "PRINCE_REGION2_IV_CODE5 register accessor: an alias for `Reg<PRINCE_REGION2_IV_CODE5_SPEC>`"]
pub type PRINCE_REGION2_IV_CODE5 =
    crate::Reg<prince_region2_iv_code5::PRINCE_REGION2_IV_CODE5_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_code5;
#[doc = "PRINCE_REGION2_IV_BODY4 register accessor: an alias for `Reg<PRINCE_REGION2_IV_BODY4_SPEC>`"]
pub type PRINCE_REGION2_IV_BODY4 =
    crate::Reg<prince_region2_iv_body4::PRINCE_REGION2_IV_BODY4_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_body4;
#[doc = "PRINCE_REGION2_IV_CODE6 register accessor: an alias for `Reg<PRINCE_REGION2_IV_CODE6_SPEC>`"]
pub type PRINCE_REGION2_IV_CODE6 =
    crate::Reg<prince_region2_iv_code6::PRINCE_REGION2_IV_CODE6_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_code6;
#[doc = "PRINCE_REGION2_IV_BODY5 register accessor: an alias for `Reg<PRINCE_REGION2_IV_BODY5_SPEC>`"]
pub type PRINCE_REGION2_IV_BODY5 =
    crate::Reg<prince_region2_iv_body5::PRINCE_REGION2_IV_BODY5_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_body5;
#[doc = "PRINCE_REGION2_IV_CODE7 register accessor: an alias for `Reg<PRINCE_REGION2_IV_CODE7_SPEC>`"]
pub type PRINCE_REGION2_IV_CODE7 =
    crate::Reg<prince_region2_iv_code7::PRINCE_REGION2_IV_CODE7_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_code7;
#[doc = "PRINCE_REGION2_IV_BODY6 register accessor: an alias for `Reg<PRINCE_REGION2_IV_BODY6_SPEC>`"]
pub type PRINCE_REGION2_IV_BODY6 =
    crate::Reg<prince_region2_iv_body6::PRINCE_REGION2_IV_BODY6_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_body6;
#[doc = "PRINCE_REGION2_IV_CODE8 register accessor: an alias for `Reg<PRINCE_REGION2_IV_CODE8_SPEC>`"]
pub type PRINCE_REGION2_IV_CODE8 =
    crate::Reg<prince_region2_iv_code8::PRINCE_REGION2_IV_CODE8_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_code8;
#[doc = "PRINCE_REGION2_IV_BODY7 register accessor: an alias for `Reg<PRINCE_REGION2_IV_BODY7_SPEC>`"]
pub type PRINCE_REGION2_IV_BODY7 =
    crate::Reg<prince_region2_iv_body7::PRINCE_REGION2_IV_BODY7_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_body7;
#[doc = "PRINCE_REGION2_IV_CODE9 register accessor: an alias for `Reg<PRINCE_REGION2_IV_CODE9_SPEC>`"]
pub type PRINCE_REGION2_IV_CODE9 =
    crate::Reg<prince_region2_iv_code9::PRINCE_REGION2_IV_CODE9_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_code9;
#[doc = "PRINCE_REGION2_IV_BODY8 register accessor: an alias for `Reg<PRINCE_REGION2_IV_BODY8_SPEC>`"]
pub type PRINCE_REGION2_IV_BODY8 =
    crate::Reg<prince_region2_iv_body8::PRINCE_REGION2_IV_BODY8_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_body8;
#[doc = "PRINCE_REGION2_IV_CODE10 register accessor: an alias for `Reg<PRINCE_REGION2_IV_CODE10_SPEC>`"]
pub type PRINCE_REGION2_IV_CODE10 =
    crate::Reg<prince_region2_iv_code10::PRINCE_REGION2_IV_CODE10_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_code10;
#[doc = "PRINCE_REGION2_IV_BODY9 register accessor: an alias for `Reg<PRINCE_REGION2_IV_BODY9_SPEC>`"]
pub type PRINCE_REGION2_IV_BODY9 =
    crate::Reg<prince_region2_iv_body9::PRINCE_REGION2_IV_BODY9_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_body9;
#[doc = "PRINCE_REGION2_IV_CODE11 register accessor: an alias for `Reg<PRINCE_REGION2_IV_CODE11_SPEC>`"]
pub type PRINCE_REGION2_IV_CODE11 =
    crate::Reg<prince_region2_iv_code11::PRINCE_REGION2_IV_CODE11_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_code11;
#[doc = "PRINCE_REGION2_IV_BODY10 register accessor: an alias for `Reg<PRINCE_REGION2_IV_BODY10_SPEC>`"]
pub type PRINCE_REGION2_IV_BODY10 =
    crate::Reg<prince_region2_iv_body10::PRINCE_REGION2_IV_BODY10_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_body10;
#[doc = "PRINCE_REGION2_IV_CODE12 register accessor: an alias for `Reg<PRINCE_REGION2_IV_CODE12_SPEC>`"]
pub type PRINCE_REGION2_IV_CODE12 =
    crate::Reg<prince_region2_iv_code12::PRINCE_REGION2_IV_CODE12_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_code12;
#[doc = "PRINCE_REGION2_IV_BODY11 register accessor: an alias for `Reg<PRINCE_REGION2_IV_BODY11_SPEC>`"]
pub type PRINCE_REGION2_IV_BODY11 =
    crate::Reg<prince_region2_iv_body11::PRINCE_REGION2_IV_BODY11_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_body11;
#[doc = "PRINCE_REGION2_IV_CODE13 register accessor: an alias for `Reg<PRINCE_REGION2_IV_CODE13_SPEC>`"]
pub type PRINCE_REGION2_IV_CODE13 =
    crate::Reg<prince_region2_iv_code13::PRINCE_REGION2_IV_CODE13_SPEC>;
#[doc = "."]
pub mod prince_region2_iv_code13;
#[doc = "CUSTOMER_DEFINED register accessor: an alias for `Reg<CUSTOMER_DEFINED_SPEC>`"]
pub type CUSTOMER_DEFINED = crate::Reg<customer_defined::CUSTOMER_DEFINED_SPEC>;
#[doc = "Customer Defined (Programable through ROM API)"]
pub mod customer_defined;
#[doc = "SHA256_DIGEST register accessor: an alias for `Reg<SHA256_DIGEST_SPEC>`"]
pub type SHA256_DIGEST = crate::Reg<sha256_digest::SHA256_DIGEST_SPEC>;
#[doc = "SHA256_DIGEST0 for DIGEST\\[31:0\\]
SHA256_DIGEST1 for DIGEST\\[63:32\\]
SHA256_DIGEST2 for DIGEST\\[95:64\\]
SHA256_DIGEST3 for DIGEST\\[127:96\\]
SHA256_DIGEST4 for DIGEST\\[159:128\\]
SHA256_DIGEST5 for DIGEST\\[191:160\\]
SHA256_DIGEST6 for DIGEST\\[223:192\\]
SHA256_DIGEST7 for DIGEST\\[255:224\\]"]
pub mod sha256_digest;
