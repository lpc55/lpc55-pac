#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ."]
    pub header: HEADER,
    #[doc = "0x04 - ."]
    pub version: VERSION,
    #[doc = "0x08 - Secure firmware version (Monotonic counter)"]
    pub s_fw_version: S_FW_VERSION,
    #[doc = "0x0c - Non-Secure firmware version (Monotonic counter)"]
    pub ns_fw_version: NS_FW_VERSION,
    #[doc = "0x10 - Image key revocation ID (Monotonic counter)"]
    pub image_key_revoke: IMAGE_KEY_REVOKE,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - ."]
    pub rotkh_revoke: ROTKH_REVOKE,
    #[doc = "0x1c - ."]
    pub vendor_usage: VENDOR_USAGE,
    #[doc = "0x20 - With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
    pub dcfg_cc_socu_pin: DCFG_CC_SOCU_PIN,
    #[doc = "0x24 - With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
    pub dcfg_cc_socu_dflt: DCFG_CC_SOCU_DFLT,
    #[doc = "0x28 - Enable FA mode. SET_FA_MODE Command should write 0xC33CA55A to this word to indicate boot ROM to enter FA mode."]
    pub enable_fa_mode: ENABLE_FA_MODE,
    #[doc = "0x2c - CMPA Page programming on going. This field shall be set to 0x5CC55AA5 in the active CFPA page each time CMPA page programming is going on. It shall always be set to 0x00000000 in the CFPA scratch area."]
    pub cmpa_prog_in_progress: CMPA_PROG_IN_PROGRESS,
    _reserved_11_prince_region0_iv: [u8; 4usize],
    _reserved_12_prince_region0_iv: [u8; 4usize],
    _reserved_13_prince_region0_iv: [u8; 4usize],
    _reserved_14_prince_region0_iv: [u8; 4usize],
    _reserved_15_prince_region0_iv: [u8; 4usize],
    _reserved_16_prince_region0_iv: [u8; 4usize],
    _reserved_17_prince_region0_iv: [u8; 4usize],
    _reserved_18_prince_region0_iv: [u8; 4usize],
    _reserved_19_prince_region0_iv: [u8; 4usize],
    _reserved_20_prince_region0_iv: [u8; 4usize],
    _reserved_21_prince_region0_iv: [u8; 4usize],
    _reserved_22_prince_region0_iv: [u8; 4usize],
    _reserved_23_prince_region0_iv: [u8; 4usize],
    _reserved_24_prince_region0_iv: [u8; 4usize],
    _reserved_25_prince_region1_iv: [u8; 4usize],
    _reserved_26_prince_region1_iv: [u8; 4usize],
    _reserved_27_prince_region1_iv: [u8; 4usize],
    _reserved_28_prince_region1_iv: [u8; 4usize],
    _reserved_29_prince_region1_iv: [u8; 4usize],
    _reserved_30_prince_region1_iv: [u8; 4usize],
    _reserved_31_prince_region1_iv: [u8; 4usize],
    _reserved_32_prince_region1_iv: [u8; 4usize],
    _reserved_33_prince_region1_iv: [u8; 4usize],
    _reserved_34_prince_region1_iv: [u8; 4usize],
    _reserved_35_prince_region1_iv: [u8; 4usize],
    _reserved_36_prince_region1_iv: [u8; 4usize],
    _reserved_37_prince_region1_iv: [u8; 4usize],
    _reserved_38_prince_region1_iv: [u8; 4usize],
    _reserved_39_prince_region2_iv: [u8; 4usize],
    _reserved_40_prince_region2_iv: [u8; 4usize],
    _reserved_41_prince_region2_iv: [u8; 4usize],
    _reserved_42_prince_region2_iv: [u8; 4usize],
    _reserved_43_prince_region2_iv: [u8; 4usize],
    _reserved_44_prince_region2_iv: [u8; 4usize],
    _reserved_45_prince_region2_iv: [u8; 4usize],
    _reserved_46_prince_region2_iv: [u8; 4usize],
    _reserved_47_prince_region2_iv: [u8; 4usize],
    _reserved_48_prince_region2_iv: [u8; 4usize],
    _reserved_49_prince_region2_iv: [u8; 4usize],
    _reserved_50_prince_region2_iv: [u8; 4usize],
    _reserved_51_prince_region2_iv: [u8; 4usize],
    _reserved_52_prince_region2_iv: [u8; 4usize],
    _reserved53: [u8; 40usize],
    #[doc = "0x100 - Customer Defined (Programable through ROM API)"]
    pub customer_defined: [CUSTOMER_DEFINED; 56],
    #[doc = "0x1e0 - SHA256_DIGEST0 for DIGEST\\[31:0\\]
SHA256_DIGEST1 for DIGEST\\[63:32\\]
SHA256_DIGEST2 for DIGEST\\[95:64\\]
SHA256_DIGEST3 for DIGEST\\[127:96\\]
SHA256_DIGEST4 for DIGEST\\[159:128\\]
SHA256_DIGEST5 for DIGEST\\[191:160\\]
SHA256_DIGEST6 for DIGEST\\[223:192\\]
SHA256_DIGEST7 for DIGEST\\[255:224\\]"]
    pub sha256_digest: [SHA256_DIGEST; 8],
}
impl RegisterBlock {
    #[doc = "0x30 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_header0(&self) -> &PRINCE_REGION0_IV_HEADER0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(48usize)
                as *const PRINCE_REGION0_IV_HEADER0)
        }
    }
    #[doc = "0x30 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_header0_mut(&self) -> &mut PRINCE_REGION0_IV_HEADER0 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(48usize)
                as *mut PRINCE_REGION0_IV_HEADER0)
        }
    }
    #[doc = "0x30 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code0(&self) -> &PRINCE_REGION0_IV_CODE0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(48usize) as *const PRINCE_REGION0_IV_CODE0)
        }
    }
    #[doc = "0x30 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code0_mut(&self) -> &mut PRINCE_REGION0_IV_CODE0 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(48usize) as *mut PRINCE_REGION0_IV_CODE0)
        }
    }
    #[doc = "0x34 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_header1(&self) -> &PRINCE_REGION0_IV_HEADER1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(52usize)
                as *const PRINCE_REGION0_IV_HEADER1)
        }
    }
    #[doc = "0x34 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_header1_mut(&self) -> &mut PRINCE_REGION0_IV_HEADER1 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(52usize)
                as *mut PRINCE_REGION0_IV_HEADER1)
        }
    }
    #[doc = "0x34 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code1(&self) -> &PRINCE_REGION0_IV_CODE1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(52usize) as *const PRINCE_REGION0_IV_CODE1)
        }
    }
    #[doc = "0x34 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code1_mut(&self) -> &mut PRINCE_REGION0_IV_CODE1 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut PRINCE_REGION0_IV_CODE1)
        }
    }
    #[doc = "0x38 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code2(&self) -> &PRINCE_REGION0_IV_CODE2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(56usize) as *const PRINCE_REGION0_IV_CODE2)
        }
    }
    #[doc = "0x38 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code2_mut(&self) -> &mut PRINCE_REGION0_IV_CODE2 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(56usize) as *mut PRINCE_REGION0_IV_CODE2)
        }
    }
    #[doc = "0x38 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body0(&self) -> &PRINCE_REGION0_IV_BODY0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(56usize) as *const PRINCE_REGION0_IV_BODY0)
        }
    }
    #[doc = "0x38 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body0_mut(&self) -> &mut PRINCE_REGION0_IV_BODY0 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(56usize) as *mut PRINCE_REGION0_IV_BODY0)
        }
    }
    #[doc = "0x3c - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code3(&self) -> &PRINCE_REGION0_IV_CODE3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(60usize) as *const PRINCE_REGION0_IV_CODE3)
        }
    }
    #[doc = "0x3c - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code3_mut(&self) -> &mut PRINCE_REGION0_IV_CODE3 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(60usize) as *mut PRINCE_REGION0_IV_CODE3)
        }
    }
    #[doc = "0x3c - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body1(&self) -> &PRINCE_REGION0_IV_BODY1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(60usize) as *const PRINCE_REGION0_IV_BODY1)
        }
    }
    #[doc = "0x3c - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body1_mut(&self) -> &mut PRINCE_REGION0_IV_BODY1 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(60usize) as *mut PRINCE_REGION0_IV_BODY1)
        }
    }
    #[doc = "0x40 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code4(&self) -> &PRINCE_REGION0_IV_CODE4 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(64usize) as *const PRINCE_REGION0_IV_CODE4)
        }
    }
    #[doc = "0x40 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code4_mut(&self) -> &mut PRINCE_REGION0_IV_CODE4 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut PRINCE_REGION0_IV_CODE4)
        }
    }
    #[doc = "0x40 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body2(&self) -> &PRINCE_REGION0_IV_BODY2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(64usize) as *const PRINCE_REGION0_IV_BODY2)
        }
    }
    #[doc = "0x40 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body2_mut(&self) -> &mut PRINCE_REGION0_IV_BODY2 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut PRINCE_REGION0_IV_BODY2)
        }
    }
    #[doc = "0x44 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code5(&self) -> &PRINCE_REGION0_IV_CODE5 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize) as *const PRINCE_REGION0_IV_CODE5)
        }
    }
    #[doc = "0x44 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code5_mut(&self) -> &mut PRINCE_REGION0_IV_CODE5 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut PRINCE_REGION0_IV_CODE5)
        }
    }
    #[doc = "0x44 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body3(&self) -> &PRINCE_REGION0_IV_BODY3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize) as *const PRINCE_REGION0_IV_BODY3)
        }
    }
    #[doc = "0x44 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body3_mut(&self) -> &mut PRINCE_REGION0_IV_BODY3 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut PRINCE_REGION0_IV_BODY3)
        }
    }
    #[doc = "0x48 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code6(&self) -> &PRINCE_REGION0_IV_CODE6 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(72usize) as *const PRINCE_REGION0_IV_CODE6)
        }
    }
    #[doc = "0x48 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code6_mut(&self) -> &mut PRINCE_REGION0_IV_CODE6 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(72usize) as *mut PRINCE_REGION0_IV_CODE6)
        }
    }
    #[doc = "0x48 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body4(&self) -> &PRINCE_REGION0_IV_BODY4 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(72usize) as *const PRINCE_REGION0_IV_BODY4)
        }
    }
    #[doc = "0x48 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body4_mut(&self) -> &mut PRINCE_REGION0_IV_BODY4 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(72usize) as *mut PRINCE_REGION0_IV_BODY4)
        }
    }
    #[doc = "0x4c - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code7(&self) -> &PRINCE_REGION0_IV_CODE7 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(76usize) as *const PRINCE_REGION0_IV_CODE7)
        }
    }
    #[doc = "0x4c - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code7_mut(&self) -> &mut PRINCE_REGION0_IV_CODE7 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(76usize) as *mut PRINCE_REGION0_IV_CODE7)
        }
    }
    #[doc = "0x4c - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body5(&self) -> &PRINCE_REGION0_IV_BODY5 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(76usize) as *const PRINCE_REGION0_IV_BODY5)
        }
    }
    #[doc = "0x4c - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body5_mut(&self) -> &mut PRINCE_REGION0_IV_BODY5 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(76usize) as *mut PRINCE_REGION0_IV_BODY5)
        }
    }
    #[doc = "0x50 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code8(&self) -> &PRINCE_REGION0_IV_CODE8 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(80usize) as *const PRINCE_REGION0_IV_CODE8)
        }
    }
    #[doc = "0x50 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code8_mut(&self) -> &mut PRINCE_REGION0_IV_CODE8 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(80usize) as *mut PRINCE_REGION0_IV_CODE8)
        }
    }
    #[doc = "0x50 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body6(&self) -> &PRINCE_REGION0_IV_BODY6 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(80usize) as *const PRINCE_REGION0_IV_BODY6)
        }
    }
    #[doc = "0x50 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body6_mut(&self) -> &mut PRINCE_REGION0_IV_BODY6 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(80usize) as *mut PRINCE_REGION0_IV_BODY6)
        }
    }
    #[doc = "0x54 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code9(&self) -> &PRINCE_REGION0_IV_CODE9 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(84usize) as *const PRINCE_REGION0_IV_CODE9)
        }
    }
    #[doc = "0x54 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code9_mut(&self) -> &mut PRINCE_REGION0_IV_CODE9 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(84usize) as *mut PRINCE_REGION0_IV_CODE9)
        }
    }
    #[doc = "0x54 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body7(&self) -> &PRINCE_REGION0_IV_BODY7 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(84usize) as *const PRINCE_REGION0_IV_BODY7)
        }
    }
    #[doc = "0x54 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body7_mut(&self) -> &mut PRINCE_REGION0_IV_BODY7 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(84usize) as *mut PRINCE_REGION0_IV_BODY7)
        }
    }
    #[doc = "0x58 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code10(&self) -> &PRINCE_REGION0_IV_CODE10 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(88usize) as *const PRINCE_REGION0_IV_CODE10)
        }
    }
    #[doc = "0x58 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code10_mut(&self) -> &mut PRINCE_REGION0_IV_CODE10 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(88usize) as *mut PRINCE_REGION0_IV_CODE10)
        }
    }
    #[doc = "0x58 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body8(&self) -> &PRINCE_REGION0_IV_BODY8 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(88usize) as *const PRINCE_REGION0_IV_BODY8)
        }
    }
    #[doc = "0x58 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body8_mut(&self) -> &mut PRINCE_REGION0_IV_BODY8 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(88usize) as *mut PRINCE_REGION0_IV_BODY8)
        }
    }
    #[doc = "0x5c - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code11(&self) -> &PRINCE_REGION0_IV_CODE11 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(92usize) as *const PRINCE_REGION0_IV_CODE11)
        }
    }
    #[doc = "0x5c - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code11_mut(&self) -> &mut PRINCE_REGION0_IV_CODE11 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(92usize) as *mut PRINCE_REGION0_IV_CODE11)
        }
    }
    #[doc = "0x5c - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body9(&self) -> &PRINCE_REGION0_IV_BODY9 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(92usize) as *const PRINCE_REGION0_IV_BODY9)
        }
    }
    #[doc = "0x5c - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body9_mut(&self) -> &mut PRINCE_REGION0_IV_BODY9 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(92usize) as *mut PRINCE_REGION0_IV_BODY9)
        }
    }
    #[doc = "0x60 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code12(&self) -> &PRINCE_REGION0_IV_CODE12 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(96usize) as *const PRINCE_REGION0_IV_CODE12)
        }
    }
    #[doc = "0x60 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code12_mut(&self) -> &mut PRINCE_REGION0_IV_CODE12 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(96usize) as *mut PRINCE_REGION0_IV_CODE12)
        }
    }
    #[doc = "0x60 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body10(&self) -> &PRINCE_REGION0_IV_BODY10 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(96usize) as *const PRINCE_REGION0_IV_BODY10)
        }
    }
    #[doc = "0x60 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body10_mut(&self) -> &mut PRINCE_REGION0_IV_BODY10 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(96usize) as *mut PRINCE_REGION0_IV_BODY10)
        }
    }
    #[doc = "0x64 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code13(&self) -> &PRINCE_REGION0_IV_CODE13 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(100usize)
                as *const PRINCE_REGION0_IV_CODE13)
        }
    }
    #[doc = "0x64 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_code13_mut(&self) -> &mut PRINCE_REGION0_IV_CODE13 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(100usize)
                as *mut PRINCE_REGION0_IV_CODE13)
        }
    }
    #[doc = "0x64 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body11(&self) -> &PRINCE_REGION0_IV_BODY11 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(100usize)
                as *const PRINCE_REGION0_IV_BODY11)
        }
    }
    #[doc = "0x64 - ."]
    #[inline(always)]
    pub fn prince_region0_iv_body11_mut(&self) -> &mut PRINCE_REGION0_IV_BODY11 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(100usize)
                as *mut PRINCE_REGION0_IV_BODY11)
        }
    }
    #[doc = "0x68 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_header0(&self) -> &PRINCE_REGION1_IV_HEADER0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(104usize)
                as *const PRINCE_REGION1_IV_HEADER0)
        }
    }
    #[doc = "0x68 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_header0_mut(&self) -> &mut PRINCE_REGION1_IV_HEADER0 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(104usize)
                as *mut PRINCE_REGION1_IV_HEADER0)
        }
    }
    #[doc = "0x68 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code0(&self) -> &PRINCE_REGION1_IV_CODE0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(104usize) as *const PRINCE_REGION1_IV_CODE0)
        }
    }
    #[doc = "0x68 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code0_mut(&self) -> &mut PRINCE_REGION1_IV_CODE0 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(104usize) as *mut PRINCE_REGION1_IV_CODE0)
        }
    }
    #[doc = "0x6c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_header1(&self) -> &PRINCE_REGION1_IV_HEADER1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(108usize)
                as *const PRINCE_REGION1_IV_HEADER1)
        }
    }
    #[doc = "0x6c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_header1_mut(&self) -> &mut PRINCE_REGION1_IV_HEADER1 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(108usize)
                as *mut PRINCE_REGION1_IV_HEADER1)
        }
    }
    #[doc = "0x6c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code1(&self) -> &PRINCE_REGION1_IV_CODE1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(108usize) as *const PRINCE_REGION1_IV_CODE1)
        }
    }
    #[doc = "0x6c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code1_mut(&self) -> &mut PRINCE_REGION1_IV_CODE1 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(108usize) as *mut PRINCE_REGION1_IV_CODE1)
        }
    }
    #[doc = "0x70 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code2(&self) -> &PRINCE_REGION1_IV_CODE2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(112usize) as *const PRINCE_REGION1_IV_CODE2)
        }
    }
    #[doc = "0x70 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code2_mut(&self) -> &mut PRINCE_REGION1_IV_CODE2 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(112usize) as *mut PRINCE_REGION1_IV_CODE2)
        }
    }
    #[doc = "0x70 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body0(&self) -> &PRINCE_REGION1_IV_BODY0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(112usize) as *const PRINCE_REGION1_IV_BODY0)
        }
    }
    #[doc = "0x70 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body0_mut(&self) -> &mut PRINCE_REGION1_IV_BODY0 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(112usize) as *mut PRINCE_REGION1_IV_BODY0)
        }
    }
    #[doc = "0x74 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code3(&self) -> &PRINCE_REGION1_IV_CODE3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(116usize) as *const PRINCE_REGION1_IV_CODE3)
        }
    }
    #[doc = "0x74 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code3_mut(&self) -> &mut PRINCE_REGION1_IV_CODE3 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(116usize) as *mut PRINCE_REGION1_IV_CODE3)
        }
    }
    #[doc = "0x74 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body1(&self) -> &PRINCE_REGION1_IV_BODY1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(116usize) as *const PRINCE_REGION1_IV_BODY1)
        }
    }
    #[doc = "0x74 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body1_mut(&self) -> &mut PRINCE_REGION1_IV_BODY1 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(116usize) as *mut PRINCE_REGION1_IV_BODY1)
        }
    }
    #[doc = "0x78 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code4(&self) -> &PRINCE_REGION1_IV_CODE4 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(120usize) as *const PRINCE_REGION1_IV_CODE4)
        }
    }
    #[doc = "0x78 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code4_mut(&self) -> &mut PRINCE_REGION1_IV_CODE4 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(120usize) as *mut PRINCE_REGION1_IV_CODE4)
        }
    }
    #[doc = "0x78 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body2(&self) -> &PRINCE_REGION1_IV_BODY2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(120usize) as *const PRINCE_REGION1_IV_BODY2)
        }
    }
    #[doc = "0x78 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body2_mut(&self) -> &mut PRINCE_REGION1_IV_BODY2 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(120usize) as *mut PRINCE_REGION1_IV_BODY2)
        }
    }
    #[doc = "0x7c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code5(&self) -> &PRINCE_REGION1_IV_CODE5 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(124usize) as *const PRINCE_REGION1_IV_CODE5)
        }
    }
    #[doc = "0x7c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code5_mut(&self) -> &mut PRINCE_REGION1_IV_CODE5 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(124usize) as *mut PRINCE_REGION1_IV_CODE5)
        }
    }
    #[doc = "0x7c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body3(&self) -> &PRINCE_REGION1_IV_BODY3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(124usize) as *const PRINCE_REGION1_IV_BODY3)
        }
    }
    #[doc = "0x7c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body3_mut(&self) -> &mut PRINCE_REGION1_IV_BODY3 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(124usize) as *mut PRINCE_REGION1_IV_BODY3)
        }
    }
    #[doc = "0x80 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code6(&self) -> &PRINCE_REGION1_IV_CODE6 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(128usize) as *const PRINCE_REGION1_IV_CODE6)
        }
    }
    #[doc = "0x80 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code6_mut(&self) -> &mut PRINCE_REGION1_IV_CODE6 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(128usize) as *mut PRINCE_REGION1_IV_CODE6)
        }
    }
    #[doc = "0x80 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body4(&self) -> &PRINCE_REGION1_IV_BODY4 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(128usize) as *const PRINCE_REGION1_IV_BODY4)
        }
    }
    #[doc = "0x80 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body4_mut(&self) -> &mut PRINCE_REGION1_IV_BODY4 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(128usize) as *mut PRINCE_REGION1_IV_BODY4)
        }
    }
    #[doc = "0x84 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code7(&self) -> &PRINCE_REGION1_IV_CODE7 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(132usize) as *const PRINCE_REGION1_IV_CODE7)
        }
    }
    #[doc = "0x84 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code7_mut(&self) -> &mut PRINCE_REGION1_IV_CODE7 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(132usize) as *mut PRINCE_REGION1_IV_CODE7)
        }
    }
    #[doc = "0x84 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body5(&self) -> &PRINCE_REGION1_IV_BODY5 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(132usize) as *const PRINCE_REGION1_IV_BODY5)
        }
    }
    #[doc = "0x84 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body5_mut(&self) -> &mut PRINCE_REGION1_IV_BODY5 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(132usize) as *mut PRINCE_REGION1_IV_BODY5)
        }
    }
    #[doc = "0x88 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code8(&self) -> &PRINCE_REGION1_IV_CODE8 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(136usize) as *const PRINCE_REGION1_IV_CODE8)
        }
    }
    #[doc = "0x88 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code8_mut(&self) -> &mut PRINCE_REGION1_IV_CODE8 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(136usize) as *mut PRINCE_REGION1_IV_CODE8)
        }
    }
    #[doc = "0x88 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body6(&self) -> &PRINCE_REGION1_IV_BODY6 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(136usize) as *const PRINCE_REGION1_IV_BODY6)
        }
    }
    #[doc = "0x88 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body6_mut(&self) -> &mut PRINCE_REGION1_IV_BODY6 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(136usize) as *mut PRINCE_REGION1_IV_BODY6)
        }
    }
    #[doc = "0x8c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code9(&self) -> &PRINCE_REGION1_IV_CODE9 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(140usize) as *const PRINCE_REGION1_IV_CODE9)
        }
    }
    #[doc = "0x8c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code9_mut(&self) -> &mut PRINCE_REGION1_IV_CODE9 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(140usize) as *mut PRINCE_REGION1_IV_CODE9)
        }
    }
    #[doc = "0x8c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body7(&self) -> &PRINCE_REGION1_IV_BODY7 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(140usize) as *const PRINCE_REGION1_IV_BODY7)
        }
    }
    #[doc = "0x8c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body7_mut(&self) -> &mut PRINCE_REGION1_IV_BODY7 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(140usize) as *mut PRINCE_REGION1_IV_BODY7)
        }
    }
    #[doc = "0x90 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code10(&self) -> &PRINCE_REGION1_IV_CODE10 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(144usize)
                as *const PRINCE_REGION1_IV_CODE10)
        }
    }
    #[doc = "0x90 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code10_mut(&self) -> &mut PRINCE_REGION1_IV_CODE10 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(144usize)
                as *mut PRINCE_REGION1_IV_CODE10)
        }
    }
    #[doc = "0x90 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body8(&self) -> &PRINCE_REGION1_IV_BODY8 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(144usize) as *const PRINCE_REGION1_IV_BODY8)
        }
    }
    #[doc = "0x90 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body8_mut(&self) -> &mut PRINCE_REGION1_IV_BODY8 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(144usize) as *mut PRINCE_REGION1_IV_BODY8)
        }
    }
    #[doc = "0x94 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code11(&self) -> &PRINCE_REGION1_IV_CODE11 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(148usize)
                as *const PRINCE_REGION1_IV_CODE11)
        }
    }
    #[doc = "0x94 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code11_mut(&self) -> &mut PRINCE_REGION1_IV_CODE11 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(148usize)
                as *mut PRINCE_REGION1_IV_CODE11)
        }
    }
    #[doc = "0x94 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body9(&self) -> &PRINCE_REGION1_IV_BODY9 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(148usize) as *const PRINCE_REGION1_IV_BODY9)
        }
    }
    #[doc = "0x94 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body9_mut(&self) -> &mut PRINCE_REGION1_IV_BODY9 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(148usize) as *mut PRINCE_REGION1_IV_BODY9)
        }
    }
    #[doc = "0x98 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code12(&self) -> &PRINCE_REGION1_IV_CODE12 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(152usize)
                as *const PRINCE_REGION1_IV_CODE12)
        }
    }
    #[doc = "0x98 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code12_mut(&self) -> &mut PRINCE_REGION1_IV_CODE12 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(152usize)
                as *mut PRINCE_REGION1_IV_CODE12)
        }
    }
    #[doc = "0x98 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body10(&self) -> &PRINCE_REGION1_IV_BODY10 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(152usize)
                as *const PRINCE_REGION1_IV_BODY10)
        }
    }
    #[doc = "0x98 - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body10_mut(&self) -> &mut PRINCE_REGION1_IV_BODY10 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(152usize)
                as *mut PRINCE_REGION1_IV_BODY10)
        }
    }
    #[doc = "0x9c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code13(&self) -> &PRINCE_REGION1_IV_CODE13 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(156usize)
                as *const PRINCE_REGION1_IV_CODE13)
        }
    }
    #[doc = "0x9c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_code13_mut(&self) -> &mut PRINCE_REGION1_IV_CODE13 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(156usize)
                as *mut PRINCE_REGION1_IV_CODE13)
        }
    }
    #[doc = "0x9c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body11(&self) -> &PRINCE_REGION1_IV_BODY11 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(156usize)
                as *const PRINCE_REGION1_IV_BODY11)
        }
    }
    #[doc = "0x9c - ."]
    #[inline(always)]
    pub fn prince_region1_iv_body11_mut(&self) -> &mut PRINCE_REGION1_IV_BODY11 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(156usize)
                as *mut PRINCE_REGION1_IV_BODY11)
        }
    }
    #[doc = "0xa0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_header0(&self) -> &PRINCE_REGION2_IV_HEADER0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(160usize)
                as *const PRINCE_REGION2_IV_HEADER0)
        }
    }
    #[doc = "0xa0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_header0_mut(&self) -> &mut PRINCE_REGION2_IV_HEADER0 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(160usize)
                as *mut PRINCE_REGION2_IV_HEADER0)
        }
    }
    #[doc = "0xa0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code0(&self) -> &PRINCE_REGION2_IV_CODE0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(160usize) as *const PRINCE_REGION2_IV_CODE0)
        }
    }
    #[doc = "0xa0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code0_mut(&self) -> &mut PRINCE_REGION2_IV_CODE0 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(160usize) as *mut PRINCE_REGION2_IV_CODE0)
        }
    }
    #[doc = "0xa4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_header1(&self) -> &PRINCE_REGION2_IV_HEADER1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(164usize)
                as *const PRINCE_REGION2_IV_HEADER1)
        }
    }
    #[doc = "0xa4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_header1_mut(&self) -> &mut PRINCE_REGION2_IV_HEADER1 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(164usize)
                as *mut PRINCE_REGION2_IV_HEADER1)
        }
    }
    #[doc = "0xa4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code1(&self) -> &PRINCE_REGION2_IV_CODE1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(164usize) as *const PRINCE_REGION2_IV_CODE1)
        }
    }
    #[doc = "0xa4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code1_mut(&self) -> &mut PRINCE_REGION2_IV_CODE1 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(164usize) as *mut PRINCE_REGION2_IV_CODE1)
        }
    }
    #[doc = "0xa8 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code2(&self) -> &PRINCE_REGION2_IV_CODE2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(168usize) as *const PRINCE_REGION2_IV_CODE2)
        }
    }
    #[doc = "0xa8 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code2_mut(&self) -> &mut PRINCE_REGION2_IV_CODE2 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(168usize) as *mut PRINCE_REGION2_IV_CODE2)
        }
    }
    #[doc = "0xa8 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body0(&self) -> &PRINCE_REGION2_IV_BODY0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(168usize) as *const PRINCE_REGION2_IV_BODY0)
        }
    }
    #[doc = "0xa8 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body0_mut(&self) -> &mut PRINCE_REGION2_IV_BODY0 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(168usize) as *mut PRINCE_REGION2_IV_BODY0)
        }
    }
    #[doc = "0xac - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code3(&self) -> &PRINCE_REGION2_IV_CODE3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(172usize) as *const PRINCE_REGION2_IV_CODE3)
        }
    }
    #[doc = "0xac - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code3_mut(&self) -> &mut PRINCE_REGION2_IV_CODE3 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(172usize) as *mut PRINCE_REGION2_IV_CODE3)
        }
    }
    #[doc = "0xac - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body1(&self) -> &PRINCE_REGION2_IV_BODY1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(172usize) as *const PRINCE_REGION2_IV_BODY1)
        }
    }
    #[doc = "0xac - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body1_mut(&self) -> &mut PRINCE_REGION2_IV_BODY1 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(172usize) as *mut PRINCE_REGION2_IV_BODY1)
        }
    }
    #[doc = "0xb0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code4(&self) -> &PRINCE_REGION2_IV_CODE4 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(176usize) as *const PRINCE_REGION2_IV_CODE4)
        }
    }
    #[doc = "0xb0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code4_mut(&self) -> &mut PRINCE_REGION2_IV_CODE4 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(176usize) as *mut PRINCE_REGION2_IV_CODE4)
        }
    }
    #[doc = "0xb0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body2(&self) -> &PRINCE_REGION2_IV_BODY2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(176usize) as *const PRINCE_REGION2_IV_BODY2)
        }
    }
    #[doc = "0xb0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body2_mut(&self) -> &mut PRINCE_REGION2_IV_BODY2 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(176usize) as *mut PRINCE_REGION2_IV_BODY2)
        }
    }
    #[doc = "0xb4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code5(&self) -> &PRINCE_REGION2_IV_CODE5 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(180usize) as *const PRINCE_REGION2_IV_CODE5)
        }
    }
    #[doc = "0xb4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code5_mut(&self) -> &mut PRINCE_REGION2_IV_CODE5 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(180usize) as *mut PRINCE_REGION2_IV_CODE5)
        }
    }
    #[doc = "0xb4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body3(&self) -> &PRINCE_REGION2_IV_BODY3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(180usize) as *const PRINCE_REGION2_IV_BODY3)
        }
    }
    #[doc = "0xb4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body3_mut(&self) -> &mut PRINCE_REGION2_IV_BODY3 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(180usize) as *mut PRINCE_REGION2_IV_BODY3)
        }
    }
    #[doc = "0xb8 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code6(&self) -> &PRINCE_REGION2_IV_CODE6 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(184usize) as *const PRINCE_REGION2_IV_CODE6)
        }
    }
    #[doc = "0xb8 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code6_mut(&self) -> &mut PRINCE_REGION2_IV_CODE6 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(184usize) as *mut PRINCE_REGION2_IV_CODE6)
        }
    }
    #[doc = "0xb8 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body4(&self) -> &PRINCE_REGION2_IV_BODY4 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(184usize) as *const PRINCE_REGION2_IV_BODY4)
        }
    }
    #[doc = "0xb8 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body4_mut(&self) -> &mut PRINCE_REGION2_IV_BODY4 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(184usize) as *mut PRINCE_REGION2_IV_BODY4)
        }
    }
    #[doc = "0xbc - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code7(&self) -> &PRINCE_REGION2_IV_CODE7 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(188usize) as *const PRINCE_REGION2_IV_CODE7)
        }
    }
    #[doc = "0xbc - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code7_mut(&self) -> &mut PRINCE_REGION2_IV_CODE7 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(188usize) as *mut PRINCE_REGION2_IV_CODE7)
        }
    }
    #[doc = "0xbc - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body5(&self) -> &PRINCE_REGION2_IV_BODY5 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(188usize) as *const PRINCE_REGION2_IV_BODY5)
        }
    }
    #[doc = "0xbc - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body5_mut(&self) -> &mut PRINCE_REGION2_IV_BODY5 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(188usize) as *mut PRINCE_REGION2_IV_BODY5)
        }
    }
    #[doc = "0xc0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code8(&self) -> &PRINCE_REGION2_IV_CODE8 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(192usize) as *const PRINCE_REGION2_IV_CODE8)
        }
    }
    #[doc = "0xc0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code8_mut(&self) -> &mut PRINCE_REGION2_IV_CODE8 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(192usize) as *mut PRINCE_REGION2_IV_CODE8)
        }
    }
    #[doc = "0xc0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body6(&self) -> &PRINCE_REGION2_IV_BODY6 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(192usize) as *const PRINCE_REGION2_IV_BODY6)
        }
    }
    #[doc = "0xc0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body6_mut(&self) -> &mut PRINCE_REGION2_IV_BODY6 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(192usize) as *mut PRINCE_REGION2_IV_BODY6)
        }
    }
    #[doc = "0xc4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code9(&self) -> &PRINCE_REGION2_IV_CODE9 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(196usize) as *const PRINCE_REGION2_IV_CODE9)
        }
    }
    #[doc = "0xc4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code9_mut(&self) -> &mut PRINCE_REGION2_IV_CODE9 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(196usize) as *mut PRINCE_REGION2_IV_CODE9)
        }
    }
    #[doc = "0xc4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body7(&self) -> &PRINCE_REGION2_IV_BODY7 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(196usize) as *const PRINCE_REGION2_IV_BODY7)
        }
    }
    #[doc = "0xc4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body7_mut(&self) -> &mut PRINCE_REGION2_IV_BODY7 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(196usize) as *mut PRINCE_REGION2_IV_BODY7)
        }
    }
    #[doc = "0xc8 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code10(&self) -> &PRINCE_REGION2_IV_CODE10 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(200usize)
                as *const PRINCE_REGION2_IV_CODE10)
        }
    }
    #[doc = "0xc8 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code10_mut(&self) -> &mut PRINCE_REGION2_IV_CODE10 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(200usize)
                as *mut PRINCE_REGION2_IV_CODE10)
        }
    }
    #[doc = "0xc8 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body8(&self) -> &PRINCE_REGION2_IV_BODY8 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(200usize) as *const PRINCE_REGION2_IV_BODY8)
        }
    }
    #[doc = "0xc8 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body8_mut(&self) -> &mut PRINCE_REGION2_IV_BODY8 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(200usize) as *mut PRINCE_REGION2_IV_BODY8)
        }
    }
    #[doc = "0xcc - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code11(&self) -> &PRINCE_REGION2_IV_CODE11 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(204usize)
                as *const PRINCE_REGION2_IV_CODE11)
        }
    }
    #[doc = "0xcc - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code11_mut(&self) -> &mut PRINCE_REGION2_IV_CODE11 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(204usize)
                as *mut PRINCE_REGION2_IV_CODE11)
        }
    }
    #[doc = "0xcc - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body9(&self) -> &PRINCE_REGION2_IV_BODY9 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(204usize) as *const PRINCE_REGION2_IV_BODY9)
        }
    }
    #[doc = "0xcc - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body9_mut(&self) -> &mut PRINCE_REGION2_IV_BODY9 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(204usize) as *mut PRINCE_REGION2_IV_BODY9)
        }
    }
    #[doc = "0xd0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code12(&self) -> &PRINCE_REGION2_IV_CODE12 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(208usize)
                as *const PRINCE_REGION2_IV_CODE12)
        }
    }
    #[doc = "0xd0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code12_mut(&self) -> &mut PRINCE_REGION2_IV_CODE12 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(208usize)
                as *mut PRINCE_REGION2_IV_CODE12)
        }
    }
    #[doc = "0xd0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body10(&self) -> &PRINCE_REGION2_IV_BODY10 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(208usize)
                as *const PRINCE_REGION2_IV_BODY10)
        }
    }
    #[doc = "0xd0 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body10_mut(&self) -> &mut PRINCE_REGION2_IV_BODY10 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(208usize)
                as *mut PRINCE_REGION2_IV_BODY10)
        }
    }
    #[doc = "0xd4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code13(&self) -> &PRINCE_REGION2_IV_CODE13 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(212usize)
                as *const PRINCE_REGION2_IV_CODE13)
        }
    }
    #[doc = "0xd4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_code13_mut(&self) -> &mut PRINCE_REGION2_IV_CODE13 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(212usize)
                as *mut PRINCE_REGION2_IV_CODE13)
        }
    }
    #[doc = "0xd4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body11(&self) -> &PRINCE_REGION2_IV_BODY11 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(212usize)
                as *const PRINCE_REGION2_IV_BODY11)
        }
    }
    #[doc = "0xd4 - ."]
    #[inline(always)]
    pub fn prince_region2_iv_body11_mut(&self) -> &mut PRINCE_REGION2_IV_BODY11 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(212usize)
                as *mut PRINCE_REGION2_IV_BODY11)
        }
    }
}
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [header](header) module"]
pub type HEADER = crate::Reg<u32, _HEADER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HEADER;
#[doc = "`read()` method returns [header::R](header::R) reader structure"]
impl crate::Readable for HEADER {}
#[doc = "`write(|w| ..)` method takes [header::W](header::W) writer structure"]
impl crate::Writable for HEADER {}
#[doc = "."]
pub mod header;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "`write(|w| ..)` method takes [version::W](version::W) writer structure"]
impl crate::Writable for VERSION {}
#[doc = "."]
pub mod version;
#[doc = "Secure firmware version (Monotonic counter)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_fw_version](s_fw_version) module"]
pub type S_FW_VERSION = crate::Reg<u32, _S_FW_VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S_FW_VERSION;
#[doc = "`read()` method returns [s_fw_version::R](s_fw_version::R) reader structure"]
impl crate::Readable for S_FW_VERSION {}
#[doc = "`write(|w| ..)` method takes [s_fw_version::W](s_fw_version::W) writer structure"]
impl crate::Writable for S_FW_VERSION {}
#[doc = "Secure firmware version (Monotonic counter)"]
pub mod s_fw_version;
#[doc = "Non-Secure firmware version (Monotonic counter)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ns_fw_version](ns_fw_version) module"]
pub type NS_FW_VERSION = crate::Reg<u32, _NS_FW_VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NS_FW_VERSION;
#[doc = "`read()` method returns [ns_fw_version::R](ns_fw_version::R) reader structure"]
impl crate::Readable for NS_FW_VERSION {}
#[doc = "`write(|w| ..)` method takes [ns_fw_version::W](ns_fw_version::W) writer structure"]
impl crate::Writable for NS_FW_VERSION {}
#[doc = "Non-Secure firmware version (Monotonic counter)"]
pub mod ns_fw_version;
#[doc = "Image key revocation ID (Monotonic counter)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [image_key_revoke](image_key_revoke) module"]
pub type IMAGE_KEY_REVOKE = crate::Reg<u32, _IMAGE_KEY_REVOKE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMAGE_KEY_REVOKE;
#[doc = "`read()` method returns [image_key_revoke::R](image_key_revoke::R) reader structure"]
impl crate::Readable for IMAGE_KEY_REVOKE {}
#[doc = "`write(|w| ..)` method takes [image_key_revoke::W](image_key_revoke::W) writer structure"]
impl crate::Writable for IMAGE_KEY_REVOKE {}
#[doc = "Image key revocation ID (Monotonic counter)"]
pub mod image_key_revoke;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rotkh_revoke](rotkh_revoke) module"]
pub type ROTKH_REVOKE = crate::Reg<u32, _ROTKH_REVOKE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROTKH_REVOKE;
#[doc = "`read()` method returns [rotkh_revoke::R](rotkh_revoke::R) reader structure"]
impl crate::Readable for ROTKH_REVOKE {}
#[doc = "`write(|w| ..)` method takes [rotkh_revoke::W](rotkh_revoke::W) writer structure"]
impl crate::Writable for ROTKH_REVOKE {}
#[doc = "."]
pub mod rotkh_revoke;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vendor_usage](vendor_usage) module"]
pub type VENDOR_USAGE = crate::Reg<u32, _VENDOR_USAGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VENDOR_USAGE;
#[doc = "`read()` method returns [vendor_usage::R](vendor_usage::R) reader structure"]
impl crate::Readable for VENDOR_USAGE {}
#[doc = "`write(|w| ..)` method takes [vendor_usage::W](vendor_usage::W) writer structure"]
impl crate::Writable for VENDOR_USAGE {}
#[doc = "."]
pub mod vendor_usage;
#[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcfg_cc_socu_pin](dcfg_cc_socu_pin) module"]
pub type DCFG_CC_SOCU_PIN = crate::Reg<u32, _DCFG_CC_SOCU_PIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCFG_CC_SOCU_PIN;
#[doc = "`read()` method returns [dcfg_cc_socu_pin::R](dcfg_cc_socu_pin::R) reader structure"]
impl crate::Readable for DCFG_CC_SOCU_PIN {}
#[doc = "`write(|w| ..)` method takes [dcfg_cc_socu_pin::W](dcfg_cc_socu_pin::W) writer structure"]
impl crate::Writable for DCFG_CC_SOCU_PIN {}
#[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
pub mod dcfg_cc_socu_pin;
#[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcfg_cc_socu_dflt](dcfg_cc_socu_dflt) module"]
pub type DCFG_CC_SOCU_DFLT = crate::Reg<u32, _DCFG_CC_SOCU_DFLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCFG_CC_SOCU_DFLT;
#[doc = "`read()` method returns [dcfg_cc_socu_dflt::R](dcfg_cc_socu_dflt::R) reader structure"]
impl crate::Readable for DCFG_CC_SOCU_DFLT {}
#[doc = "`write(|w| ..)` method takes [dcfg_cc_socu_dflt::W](dcfg_cc_socu_dflt::W) writer structure"]
impl crate::Writable for DCFG_CC_SOCU_DFLT {}
#[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
pub mod dcfg_cc_socu_dflt;
#[doc = "Enable FA mode. SET_FA_MODE Command should write 0xC33CA55A to this word to indicate boot ROM to enter FA mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable_fa_mode](enable_fa_mode) module"]
pub type ENABLE_FA_MODE = crate::Reg<u32, _ENABLE_FA_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE_FA_MODE;
#[doc = "`read()` method returns [enable_fa_mode::R](enable_fa_mode::R) reader structure"]
impl crate::Readable for ENABLE_FA_MODE {}
#[doc = "`write(|w| ..)` method takes [enable_fa_mode::W](enable_fa_mode::W) writer structure"]
impl crate::Writable for ENABLE_FA_MODE {}
#[doc = "Enable FA mode. SET_FA_MODE Command should write 0xC33CA55A to this word to indicate boot ROM to enter FA mode."]
pub mod enable_fa_mode;
#[doc = "CMPA Page programming on going. This field shall be set to 0x5CC55AA5 in the active CFPA page each time CMPA page programming is going on. It shall always be set to 0x00000000 in the CFPA scratch area.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpa_prog_in_progress](cmpa_prog_in_progress) module"]
pub type CMPA_PROG_IN_PROGRESS = crate::Reg<u32, _CMPA_PROG_IN_PROGRESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPA_PROG_IN_PROGRESS;
#[doc = "`read()` method returns [cmpa_prog_in_progress::R](cmpa_prog_in_progress::R) reader structure"]
impl crate::Readable for CMPA_PROG_IN_PROGRESS {}
#[doc = "`write(|w| ..)` method takes [cmpa_prog_in_progress::W](cmpa_prog_in_progress::W) writer structure"]
impl crate::Writable for CMPA_PROG_IN_PROGRESS {}
#[doc = "CMPA Page programming on going. This field shall be set to 0x5CC55AA5 in the active CFPA page each time CMPA page programming is going on. It shall always be set to 0x00000000 in the CFPA scratch area."]
pub mod cmpa_prog_in_progress;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_code0](prince_region0_iv_code0) module"]
pub type PRINCE_REGION0_IV_CODE0 = crate::Reg<u32, _PRINCE_REGION0_IV_CODE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_CODE0;
#[doc = "`read()` method returns [prince_region0_iv_code0::R](prince_region0_iv_code0::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_CODE0 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_code0::W](prince_region0_iv_code0::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_CODE0 {}
#[doc = "."]
pub mod prince_region0_iv_code0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_header0](prince_region0_iv_header0) module"]
pub type PRINCE_REGION0_IV_HEADER0 = crate::Reg<u32, _PRINCE_REGION0_IV_HEADER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_HEADER0;
#[doc = "`read()` method returns [prince_region0_iv_header0::R](prince_region0_iv_header0::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_HEADER0 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_header0::W](prince_region0_iv_header0::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_HEADER0 {}
#[doc = "."]
pub mod prince_region0_iv_header0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_code1](prince_region0_iv_code1) module"]
pub type PRINCE_REGION0_IV_CODE1 = crate::Reg<u32, _PRINCE_REGION0_IV_CODE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_CODE1;
#[doc = "`read()` method returns [prince_region0_iv_code1::R](prince_region0_iv_code1::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_CODE1 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_code1::W](prince_region0_iv_code1::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_CODE1 {}
#[doc = "."]
pub mod prince_region0_iv_code1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_header1](prince_region0_iv_header1) module"]
pub type PRINCE_REGION0_IV_HEADER1 = crate::Reg<u32, _PRINCE_REGION0_IV_HEADER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_HEADER1;
#[doc = "`read()` method returns [prince_region0_iv_header1::R](prince_region0_iv_header1::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_HEADER1 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_header1::W](prince_region0_iv_header1::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_HEADER1 {}
#[doc = "."]
pub mod prince_region0_iv_header1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_body0](prince_region0_iv_body0) module"]
pub type PRINCE_REGION0_IV_BODY0 = crate::Reg<u32, _PRINCE_REGION0_IV_BODY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_BODY0;
#[doc = "`read()` method returns [prince_region0_iv_body0::R](prince_region0_iv_body0::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_BODY0 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_body0::W](prince_region0_iv_body0::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_BODY0 {}
#[doc = "."]
pub mod prince_region0_iv_body0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_code2](prince_region0_iv_code2) module"]
pub type PRINCE_REGION0_IV_CODE2 = crate::Reg<u32, _PRINCE_REGION0_IV_CODE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_CODE2;
#[doc = "`read()` method returns [prince_region0_iv_code2::R](prince_region0_iv_code2::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_CODE2 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_code2::W](prince_region0_iv_code2::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_CODE2 {}
#[doc = "."]
pub mod prince_region0_iv_code2;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_body1](prince_region0_iv_body1) module"]
pub type PRINCE_REGION0_IV_BODY1 = crate::Reg<u32, _PRINCE_REGION0_IV_BODY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_BODY1;
#[doc = "`read()` method returns [prince_region0_iv_body1::R](prince_region0_iv_body1::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_BODY1 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_body1::W](prince_region0_iv_body1::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_BODY1 {}
#[doc = "."]
pub mod prince_region0_iv_body1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_code3](prince_region0_iv_code3) module"]
pub type PRINCE_REGION0_IV_CODE3 = crate::Reg<u32, _PRINCE_REGION0_IV_CODE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_CODE3;
#[doc = "`read()` method returns [prince_region0_iv_code3::R](prince_region0_iv_code3::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_CODE3 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_code3::W](prince_region0_iv_code3::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_CODE3 {}
#[doc = "."]
pub mod prince_region0_iv_code3;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_body2](prince_region0_iv_body2) module"]
pub type PRINCE_REGION0_IV_BODY2 = crate::Reg<u32, _PRINCE_REGION0_IV_BODY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_BODY2;
#[doc = "`read()` method returns [prince_region0_iv_body2::R](prince_region0_iv_body2::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_BODY2 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_body2::W](prince_region0_iv_body2::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_BODY2 {}
#[doc = "."]
pub mod prince_region0_iv_body2;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_code4](prince_region0_iv_code4) module"]
pub type PRINCE_REGION0_IV_CODE4 = crate::Reg<u32, _PRINCE_REGION0_IV_CODE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_CODE4;
#[doc = "`read()` method returns [prince_region0_iv_code4::R](prince_region0_iv_code4::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_CODE4 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_code4::W](prince_region0_iv_code4::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_CODE4 {}
#[doc = "."]
pub mod prince_region0_iv_code4;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_body3](prince_region0_iv_body3) module"]
pub type PRINCE_REGION0_IV_BODY3 = crate::Reg<u32, _PRINCE_REGION0_IV_BODY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_BODY3;
#[doc = "`read()` method returns [prince_region0_iv_body3::R](prince_region0_iv_body3::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_BODY3 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_body3::W](prince_region0_iv_body3::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_BODY3 {}
#[doc = "."]
pub mod prince_region0_iv_body3;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_code5](prince_region0_iv_code5) module"]
pub type PRINCE_REGION0_IV_CODE5 = crate::Reg<u32, _PRINCE_REGION0_IV_CODE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_CODE5;
#[doc = "`read()` method returns [prince_region0_iv_code5::R](prince_region0_iv_code5::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_CODE5 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_code5::W](prince_region0_iv_code5::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_CODE5 {}
#[doc = "."]
pub mod prince_region0_iv_code5;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_body4](prince_region0_iv_body4) module"]
pub type PRINCE_REGION0_IV_BODY4 = crate::Reg<u32, _PRINCE_REGION0_IV_BODY4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_BODY4;
#[doc = "`read()` method returns [prince_region0_iv_body4::R](prince_region0_iv_body4::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_BODY4 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_body4::W](prince_region0_iv_body4::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_BODY4 {}
#[doc = "."]
pub mod prince_region0_iv_body4;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_code6](prince_region0_iv_code6) module"]
pub type PRINCE_REGION0_IV_CODE6 = crate::Reg<u32, _PRINCE_REGION0_IV_CODE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_CODE6;
#[doc = "`read()` method returns [prince_region0_iv_code6::R](prince_region0_iv_code6::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_CODE6 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_code6::W](prince_region0_iv_code6::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_CODE6 {}
#[doc = "."]
pub mod prince_region0_iv_code6;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_body5](prince_region0_iv_body5) module"]
pub type PRINCE_REGION0_IV_BODY5 = crate::Reg<u32, _PRINCE_REGION0_IV_BODY5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_BODY5;
#[doc = "`read()` method returns [prince_region0_iv_body5::R](prince_region0_iv_body5::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_BODY5 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_body5::W](prince_region0_iv_body5::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_BODY5 {}
#[doc = "."]
pub mod prince_region0_iv_body5;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_code7](prince_region0_iv_code7) module"]
pub type PRINCE_REGION0_IV_CODE7 = crate::Reg<u32, _PRINCE_REGION0_IV_CODE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_CODE7;
#[doc = "`read()` method returns [prince_region0_iv_code7::R](prince_region0_iv_code7::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_CODE7 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_code7::W](prince_region0_iv_code7::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_CODE7 {}
#[doc = "."]
pub mod prince_region0_iv_code7;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_body6](prince_region0_iv_body6) module"]
pub type PRINCE_REGION0_IV_BODY6 = crate::Reg<u32, _PRINCE_REGION0_IV_BODY6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_BODY6;
#[doc = "`read()` method returns [prince_region0_iv_body6::R](prince_region0_iv_body6::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_BODY6 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_body6::W](prince_region0_iv_body6::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_BODY6 {}
#[doc = "."]
pub mod prince_region0_iv_body6;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_code8](prince_region0_iv_code8) module"]
pub type PRINCE_REGION0_IV_CODE8 = crate::Reg<u32, _PRINCE_REGION0_IV_CODE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_CODE8;
#[doc = "`read()` method returns [prince_region0_iv_code8::R](prince_region0_iv_code8::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_CODE8 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_code8::W](prince_region0_iv_code8::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_CODE8 {}
#[doc = "."]
pub mod prince_region0_iv_code8;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_body7](prince_region0_iv_body7) module"]
pub type PRINCE_REGION0_IV_BODY7 = crate::Reg<u32, _PRINCE_REGION0_IV_BODY7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_BODY7;
#[doc = "`read()` method returns [prince_region0_iv_body7::R](prince_region0_iv_body7::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_BODY7 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_body7::W](prince_region0_iv_body7::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_BODY7 {}
#[doc = "."]
pub mod prince_region0_iv_body7;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_code9](prince_region0_iv_code9) module"]
pub type PRINCE_REGION0_IV_CODE9 = crate::Reg<u32, _PRINCE_REGION0_IV_CODE9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_CODE9;
#[doc = "`read()` method returns [prince_region0_iv_code9::R](prince_region0_iv_code9::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_CODE9 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_code9::W](prince_region0_iv_code9::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_CODE9 {}
#[doc = "."]
pub mod prince_region0_iv_code9;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_body8](prince_region0_iv_body8) module"]
pub type PRINCE_REGION0_IV_BODY8 = crate::Reg<u32, _PRINCE_REGION0_IV_BODY8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_BODY8;
#[doc = "`read()` method returns [prince_region0_iv_body8::R](prince_region0_iv_body8::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_BODY8 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_body8::W](prince_region0_iv_body8::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_BODY8 {}
#[doc = "."]
pub mod prince_region0_iv_body8;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_code10](prince_region0_iv_code10) module"]
pub type PRINCE_REGION0_IV_CODE10 = crate::Reg<u32, _PRINCE_REGION0_IV_CODE10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_CODE10;
#[doc = "`read()` method returns [prince_region0_iv_code10::R](prince_region0_iv_code10::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_CODE10 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_code10::W](prince_region0_iv_code10::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_CODE10 {}
#[doc = "."]
pub mod prince_region0_iv_code10;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_body9](prince_region0_iv_body9) module"]
pub type PRINCE_REGION0_IV_BODY9 = crate::Reg<u32, _PRINCE_REGION0_IV_BODY9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_BODY9;
#[doc = "`read()` method returns [prince_region0_iv_body9::R](prince_region0_iv_body9::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_BODY9 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_body9::W](prince_region0_iv_body9::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_BODY9 {}
#[doc = "."]
pub mod prince_region0_iv_body9;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_code11](prince_region0_iv_code11) module"]
pub type PRINCE_REGION0_IV_CODE11 = crate::Reg<u32, _PRINCE_REGION0_IV_CODE11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_CODE11;
#[doc = "`read()` method returns [prince_region0_iv_code11::R](prince_region0_iv_code11::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_CODE11 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_code11::W](prince_region0_iv_code11::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_CODE11 {}
#[doc = "."]
pub mod prince_region0_iv_code11;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_body10](prince_region0_iv_body10) module"]
pub type PRINCE_REGION0_IV_BODY10 = crate::Reg<u32, _PRINCE_REGION0_IV_BODY10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_BODY10;
#[doc = "`read()` method returns [prince_region0_iv_body10::R](prince_region0_iv_body10::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_BODY10 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_body10::W](prince_region0_iv_body10::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_BODY10 {}
#[doc = "."]
pub mod prince_region0_iv_body10;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_code12](prince_region0_iv_code12) module"]
pub type PRINCE_REGION0_IV_CODE12 = crate::Reg<u32, _PRINCE_REGION0_IV_CODE12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_CODE12;
#[doc = "`read()` method returns [prince_region0_iv_code12::R](prince_region0_iv_code12::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_CODE12 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_code12::W](prince_region0_iv_code12::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_CODE12 {}
#[doc = "."]
pub mod prince_region0_iv_code12;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_body11](prince_region0_iv_body11) module"]
pub type PRINCE_REGION0_IV_BODY11 = crate::Reg<u32, _PRINCE_REGION0_IV_BODY11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_BODY11;
#[doc = "`read()` method returns [prince_region0_iv_body11::R](prince_region0_iv_body11::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_BODY11 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_body11::W](prince_region0_iv_body11::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_BODY11 {}
#[doc = "."]
pub mod prince_region0_iv_body11;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region0_iv_code13](prince_region0_iv_code13) module"]
pub type PRINCE_REGION0_IV_CODE13 = crate::Reg<u32, _PRINCE_REGION0_IV_CODE13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_IV_CODE13;
#[doc = "`read()` method returns [prince_region0_iv_code13::R](prince_region0_iv_code13::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_IV_CODE13 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_iv_code13::W](prince_region0_iv_code13::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_IV_CODE13 {}
#[doc = "."]
pub mod prince_region0_iv_code13;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_code0](prince_region1_iv_code0) module"]
pub type PRINCE_REGION1_IV_CODE0 = crate::Reg<u32, _PRINCE_REGION1_IV_CODE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_CODE0;
#[doc = "`read()` method returns [prince_region1_iv_code0::R](prince_region1_iv_code0::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_CODE0 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_code0::W](prince_region1_iv_code0::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_CODE0 {}
#[doc = "."]
pub mod prince_region1_iv_code0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_header0](prince_region1_iv_header0) module"]
pub type PRINCE_REGION1_IV_HEADER0 = crate::Reg<u32, _PRINCE_REGION1_IV_HEADER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_HEADER0;
#[doc = "`read()` method returns [prince_region1_iv_header0::R](prince_region1_iv_header0::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_HEADER0 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_header0::W](prince_region1_iv_header0::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_HEADER0 {}
#[doc = "."]
pub mod prince_region1_iv_header0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_code1](prince_region1_iv_code1) module"]
pub type PRINCE_REGION1_IV_CODE1 = crate::Reg<u32, _PRINCE_REGION1_IV_CODE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_CODE1;
#[doc = "`read()` method returns [prince_region1_iv_code1::R](prince_region1_iv_code1::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_CODE1 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_code1::W](prince_region1_iv_code1::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_CODE1 {}
#[doc = "."]
pub mod prince_region1_iv_code1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_header1](prince_region1_iv_header1) module"]
pub type PRINCE_REGION1_IV_HEADER1 = crate::Reg<u32, _PRINCE_REGION1_IV_HEADER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_HEADER1;
#[doc = "`read()` method returns [prince_region1_iv_header1::R](prince_region1_iv_header1::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_HEADER1 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_header1::W](prince_region1_iv_header1::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_HEADER1 {}
#[doc = "."]
pub mod prince_region1_iv_header1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_body0](prince_region1_iv_body0) module"]
pub type PRINCE_REGION1_IV_BODY0 = crate::Reg<u32, _PRINCE_REGION1_IV_BODY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_BODY0;
#[doc = "`read()` method returns [prince_region1_iv_body0::R](prince_region1_iv_body0::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_BODY0 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_body0::W](prince_region1_iv_body0::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_BODY0 {}
#[doc = "."]
pub mod prince_region1_iv_body0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_code2](prince_region1_iv_code2) module"]
pub type PRINCE_REGION1_IV_CODE2 = crate::Reg<u32, _PRINCE_REGION1_IV_CODE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_CODE2;
#[doc = "`read()` method returns [prince_region1_iv_code2::R](prince_region1_iv_code2::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_CODE2 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_code2::W](prince_region1_iv_code2::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_CODE2 {}
#[doc = "."]
pub mod prince_region1_iv_code2;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_body1](prince_region1_iv_body1) module"]
pub type PRINCE_REGION1_IV_BODY1 = crate::Reg<u32, _PRINCE_REGION1_IV_BODY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_BODY1;
#[doc = "`read()` method returns [prince_region1_iv_body1::R](prince_region1_iv_body1::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_BODY1 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_body1::W](prince_region1_iv_body1::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_BODY1 {}
#[doc = "."]
pub mod prince_region1_iv_body1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_code3](prince_region1_iv_code3) module"]
pub type PRINCE_REGION1_IV_CODE3 = crate::Reg<u32, _PRINCE_REGION1_IV_CODE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_CODE3;
#[doc = "`read()` method returns [prince_region1_iv_code3::R](prince_region1_iv_code3::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_CODE3 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_code3::W](prince_region1_iv_code3::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_CODE3 {}
#[doc = "."]
pub mod prince_region1_iv_code3;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_body2](prince_region1_iv_body2) module"]
pub type PRINCE_REGION1_IV_BODY2 = crate::Reg<u32, _PRINCE_REGION1_IV_BODY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_BODY2;
#[doc = "`read()` method returns [prince_region1_iv_body2::R](prince_region1_iv_body2::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_BODY2 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_body2::W](prince_region1_iv_body2::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_BODY2 {}
#[doc = "."]
pub mod prince_region1_iv_body2;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_code4](prince_region1_iv_code4) module"]
pub type PRINCE_REGION1_IV_CODE4 = crate::Reg<u32, _PRINCE_REGION1_IV_CODE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_CODE4;
#[doc = "`read()` method returns [prince_region1_iv_code4::R](prince_region1_iv_code4::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_CODE4 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_code4::W](prince_region1_iv_code4::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_CODE4 {}
#[doc = "."]
pub mod prince_region1_iv_code4;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_body3](prince_region1_iv_body3) module"]
pub type PRINCE_REGION1_IV_BODY3 = crate::Reg<u32, _PRINCE_REGION1_IV_BODY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_BODY3;
#[doc = "`read()` method returns [prince_region1_iv_body3::R](prince_region1_iv_body3::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_BODY3 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_body3::W](prince_region1_iv_body3::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_BODY3 {}
#[doc = "."]
pub mod prince_region1_iv_body3;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_code5](prince_region1_iv_code5) module"]
pub type PRINCE_REGION1_IV_CODE5 = crate::Reg<u32, _PRINCE_REGION1_IV_CODE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_CODE5;
#[doc = "`read()` method returns [prince_region1_iv_code5::R](prince_region1_iv_code5::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_CODE5 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_code5::W](prince_region1_iv_code5::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_CODE5 {}
#[doc = "."]
pub mod prince_region1_iv_code5;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_body4](prince_region1_iv_body4) module"]
pub type PRINCE_REGION1_IV_BODY4 = crate::Reg<u32, _PRINCE_REGION1_IV_BODY4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_BODY4;
#[doc = "`read()` method returns [prince_region1_iv_body4::R](prince_region1_iv_body4::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_BODY4 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_body4::W](prince_region1_iv_body4::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_BODY4 {}
#[doc = "."]
pub mod prince_region1_iv_body4;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_code6](prince_region1_iv_code6) module"]
pub type PRINCE_REGION1_IV_CODE6 = crate::Reg<u32, _PRINCE_REGION1_IV_CODE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_CODE6;
#[doc = "`read()` method returns [prince_region1_iv_code6::R](prince_region1_iv_code6::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_CODE6 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_code6::W](prince_region1_iv_code6::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_CODE6 {}
#[doc = "."]
pub mod prince_region1_iv_code6;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_body5](prince_region1_iv_body5) module"]
pub type PRINCE_REGION1_IV_BODY5 = crate::Reg<u32, _PRINCE_REGION1_IV_BODY5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_BODY5;
#[doc = "`read()` method returns [prince_region1_iv_body5::R](prince_region1_iv_body5::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_BODY5 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_body5::W](prince_region1_iv_body5::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_BODY5 {}
#[doc = "."]
pub mod prince_region1_iv_body5;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_code7](prince_region1_iv_code7) module"]
pub type PRINCE_REGION1_IV_CODE7 = crate::Reg<u32, _PRINCE_REGION1_IV_CODE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_CODE7;
#[doc = "`read()` method returns [prince_region1_iv_code7::R](prince_region1_iv_code7::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_CODE7 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_code7::W](prince_region1_iv_code7::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_CODE7 {}
#[doc = "."]
pub mod prince_region1_iv_code7;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_body6](prince_region1_iv_body6) module"]
pub type PRINCE_REGION1_IV_BODY6 = crate::Reg<u32, _PRINCE_REGION1_IV_BODY6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_BODY6;
#[doc = "`read()` method returns [prince_region1_iv_body6::R](prince_region1_iv_body6::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_BODY6 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_body6::W](prince_region1_iv_body6::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_BODY6 {}
#[doc = "."]
pub mod prince_region1_iv_body6;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_code8](prince_region1_iv_code8) module"]
pub type PRINCE_REGION1_IV_CODE8 = crate::Reg<u32, _PRINCE_REGION1_IV_CODE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_CODE8;
#[doc = "`read()` method returns [prince_region1_iv_code8::R](prince_region1_iv_code8::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_CODE8 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_code8::W](prince_region1_iv_code8::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_CODE8 {}
#[doc = "."]
pub mod prince_region1_iv_code8;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_body7](prince_region1_iv_body7) module"]
pub type PRINCE_REGION1_IV_BODY7 = crate::Reg<u32, _PRINCE_REGION1_IV_BODY7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_BODY7;
#[doc = "`read()` method returns [prince_region1_iv_body7::R](prince_region1_iv_body7::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_BODY7 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_body7::W](prince_region1_iv_body7::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_BODY7 {}
#[doc = "."]
pub mod prince_region1_iv_body7;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_code9](prince_region1_iv_code9) module"]
pub type PRINCE_REGION1_IV_CODE9 = crate::Reg<u32, _PRINCE_REGION1_IV_CODE9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_CODE9;
#[doc = "`read()` method returns [prince_region1_iv_code9::R](prince_region1_iv_code9::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_CODE9 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_code9::W](prince_region1_iv_code9::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_CODE9 {}
#[doc = "."]
pub mod prince_region1_iv_code9;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_body8](prince_region1_iv_body8) module"]
pub type PRINCE_REGION1_IV_BODY8 = crate::Reg<u32, _PRINCE_REGION1_IV_BODY8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_BODY8;
#[doc = "`read()` method returns [prince_region1_iv_body8::R](prince_region1_iv_body8::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_BODY8 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_body8::W](prince_region1_iv_body8::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_BODY8 {}
#[doc = "."]
pub mod prince_region1_iv_body8;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_code10](prince_region1_iv_code10) module"]
pub type PRINCE_REGION1_IV_CODE10 = crate::Reg<u32, _PRINCE_REGION1_IV_CODE10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_CODE10;
#[doc = "`read()` method returns [prince_region1_iv_code10::R](prince_region1_iv_code10::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_CODE10 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_code10::W](prince_region1_iv_code10::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_CODE10 {}
#[doc = "."]
pub mod prince_region1_iv_code10;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_body9](prince_region1_iv_body9) module"]
pub type PRINCE_REGION1_IV_BODY9 = crate::Reg<u32, _PRINCE_REGION1_IV_BODY9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_BODY9;
#[doc = "`read()` method returns [prince_region1_iv_body9::R](prince_region1_iv_body9::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_BODY9 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_body9::W](prince_region1_iv_body9::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_BODY9 {}
#[doc = "."]
pub mod prince_region1_iv_body9;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_code11](prince_region1_iv_code11) module"]
pub type PRINCE_REGION1_IV_CODE11 = crate::Reg<u32, _PRINCE_REGION1_IV_CODE11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_CODE11;
#[doc = "`read()` method returns [prince_region1_iv_code11::R](prince_region1_iv_code11::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_CODE11 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_code11::W](prince_region1_iv_code11::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_CODE11 {}
#[doc = "."]
pub mod prince_region1_iv_code11;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_body10](prince_region1_iv_body10) module"]
pub type PRINCE_REGION1_IV_BODY10 = crate::Reg<u32, _PRINCE_REGION1_IV_BODY10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_BODY10;
#[doc = "`read()` method returns [prince_region1_iv_body10::R](prince_region1_iv_body10::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_BODY10 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_body10::W](prince_region1_iv_body10::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_BODY10 {}
#[doc = "."]
pub mod prince_region1_iv_body10;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_code12](prince_region1_iv_code12) module"]
pub type PRINCE_REGION1_IV_CODE12 = crate::Reg<u32, _PRINCE_REGION1_IV_CODE12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_CODE12;
#[doc = "`read()` method returns [prince_region1_iv_code12::R](prince_region1_iv_code12::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_CODE12 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_code12::W](prince_region1_iv_code12::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_CODE12 {}
#[doc = "."]
pub mod prince_region1_iv_code12;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_body11](prince_region1_iv_body11) module"]
pub type PRINCE_REGION1_IV_BODY11 = crate::Reg<u32, _PRINCE_REGION1_IV_BODY11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_BODY11;
#[doc = "`read()` method returns [prince_region1_iv_body11::R](prince_region1_iv_body11::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_BODY11 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_body11::W](prince_region1_iv_body11::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_BODY11 {}
#[doc = "."]
pub mod prince_region1_iv_body11;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region1_iv_code13](prince_region1_iv_code13) module"]
pub type PRINCE_REGION1_IV_CODE13 = crate::Reg<u32, _PRINCE_REGION1_IV_CODE13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_IV_CODE13;
#[doc = "`read()` method returns [prince_region1_iv_code13::R](prince_region1_iv_code13::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_IV_CODE13 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_iv_code13::W](prince_region1_iv_code13::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_IV_CODE13 {}
#[doc = "."]
pub mod prince_region1_iv_code13;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_code0](prince_region2_iv_code0) module"]
pub type PRINCE_REGION2_IV_CODE0 = crate::Reg<u32, _PRINCE_REGION2_IV_CODE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_CODE0;
#[doc = "`read()` method returns [prince_region2_iv_code0::R](prince_region2_iv_code0::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_CODE0 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_code0::W](prince_region2_iv_code0::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_CODE0 {}
#[doc = "."]
pub mod prince_region2_iv_code0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_header0](prince_region2_iv_header0) module"]
pub type PRINCE_REGION2_IV_HEADER0 = crate::Reg<u32, _PRINCE_REGION2_IV_HEADER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_HEADER0;
#[doc = "`read()` method returns [prince_region2_iv_header0::R](prince_region2_iv_header0::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_HEADER0 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_header0::W](prince_region2_iv_header0::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_HEADER0 {}
#[doc = "."]
pub mod prince_region2_iv_header0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_code1](prince_region2_iv_code1) module"]
pub type PRINCE_REGION2_IV_CODE1 = crate::Reg<u32, _PRINCE_REGION2_IV_CODE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_CODE1;
#[doc = "`read()` method returns [prince_region2_iv_code1::R](prince_region2_iv_code1::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_CODE1 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_code1::W](prince_region2_iv_code1::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_CODE1 {}
#[doc = "."]
pub mod prince_region2_iv_code1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_header1](prince_region2_iv_header1) module"]
pub type PRINCE_REGION2_IV_HEADER1 = crate::Reg<u32, _PRINCE_REGION2_IV_HEADER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_HEADER1;
#[doc = "`read()` method returns [prince_region2_iv_header1::R](prince_region2_iv_header1::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_HEADER1 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_header1::W](prince_region2_iv_header1::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_HEADER1 {}
#[doc = "."]
pub mod prince_region2_iv_header1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_body0](prince_region2_iv_body0) module"]
pub type PRINCE_REGION2_IV_BODY0 = crate::Reg<u32, _PRINCE_REGION2_IV_BODY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_BODY0;
#[doc = "`read()` method returns [prince_region2_iv_body0::R](prince_region2_iv_body0::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_BODY0 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_body0::W](prince_region2_iv_body0::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_BODY0 {}
#[doc = "."]
pub mod prince_region2_iv_body0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_code2](prince_region2_iv_code2) module"]
pub type PRINCE_REGION2_IV_CODE2 = crate::Reg<u32, _PRINCE_REGION2_IV_CODE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_CODE2;
#[doc = "`read()` method returns [prince_region2_iv_code2::R](prince_region2_iv_code2::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_CODE2 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_code2::W](prince_region2_iv_code2::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_CODE2 {}
#[doc = "."]
pub mod prince_region2_iv_code2;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_body1](prince_region2_iv_body1) module"]
pub type PRINCE_REGION2_IV_BODY1 = crate::Reg<u32, _PRINCE_REGION2_IV_BODY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_BODY1;
#[doc = "`read()` method returns [prince_region2_iv_body1::R](prince_region2_iv_body1::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_BODY1 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_body1::W](prince_region2_iv_body1::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_BODY1 {}
#[doc = "."]
pub mod prince_region2_iv_body1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_code3](prince_region2_iv_code3) module"]
pub type PRINCE_REGION2_IV_CODE3 = crate::Reg<u32, _PRINCE_REGION2_IV_CODE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_CODE3;
#[doc = "`read()` method returns [prince_region2_iv_code3::R](prince_region2_iv_code3::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_CODE3 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_code3::W](prince_region2_iv_code3::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_CODE3 {}
#[doc = "."]
pub mod prince_region2_iv_code3;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_body2](prince_region2_iv_body2) module"]
pub type PRINCE_REGION2_IV_BODY2 = crate::Reg<u32, _PRINCE_REGION2_IV_BODY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_BODY2;
#[doc = "`read()` method returns [prince_region2_iv_body2::R](prince_region2_iv_body2::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_BODY2 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_body2::W](prince_region2_iv_body2::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_BODY2 {}
#[doc = "."]
pub mod prince_region2_iv_body2;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_code4](prince_region2_iv_code4) module"]
pub type PRINCE_REGION2_IV_CODE4 = crate::Reg<u32, _PRINCE_REGION2_IV_CODE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_CODE4;
#[doc = "`read()` method returns [prince_region2_iv_code4::R](prince_region2_iv_code4::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_CODE4 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_code4::W](prince_region2_iv_code4::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_CODE4 {}
#[doc = "."]
pub mod prince_region2_iv_code4;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_body3](prince_region2_iv_body3) module"]
pub type PRINCE_REGION2_IV_BODY3 = crate::Reg<u32, _PRINCE_REGION2_IV_BODY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_BODY3;
#[doc = "`read()` method returns [prince_region2_iv_body3::R](prince_region2_iv_body3::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_BODY3 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_body3::W](prince_region2_iv_body3::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_BODY3 {}
#[doc = "."]
pub mod prince_region2_iv_body3;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_code5](prince_region2_iv_code5) module"]
pub type PRINCE_REGION2_IV_CODE5 = crate::Reg<u32, _PRINCE_REGION2_IV_CODE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_CODE5;
#[doc = "`read()` method returns [prince_region2_iv_code5::R](prince_region2_iv_code5::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_CODE5 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_code5::W](prince_region2_iv_code5::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_CODE5 {}
#[doc = "."]
pub mod prince_region2_iv_code5;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_body4](prince_region2_iv_body4) module"]
pub type PRINCE_REGION2_IV_BODY4 = crate::Reg<u32, _PRINCE_REGION2_IV_BODY4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_BODY4;
#[doc = "`read()` method returns [prince_region2_iv_body4::R](prince_region2_iv_body4::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_BODY4 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_body4::W](prince_region2_iv_body4::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_BODY4 {}
#[doc = "."]
pub mod prince_region2_iv_body4;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_code6](prince_region2_iv_code6) module"]
pub type PRINCE_REGION2_IV_CODE6 = crate::Reg<u32, _PRINCE_REGION2_IV_CODE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_CODE6;
#[doc = "`read()` method returns [prince_region2_iv_code6::R](prince_region2_iv_code6::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_CODE6 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_code6::W](prince_region2_iv_code6::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_CODE6 {}
#[doc = "."]
pub mod prince_region2_iv_code6;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_body5](prince_region2_iv_body5) module"]
pub type PRINCE_REGION2_IV_BODY5 = crate::Reg<u32, _PRINCE_REGION2_IV_BODY5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_BODY5;
#[doc = "`read()` method returns [prince_region2_iv_body5::R](prince_region2_iv_body5::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_BODY5 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_body5::W](prince_region2_iv_body5::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_BODY5 {}
#[doc = "."]
pub mod prince_region2_iv_body5;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_code7](prince_region2_iv_code7) module"]
pub type PRINCE_REGION2_IV_CODE7 = crate::Reg<u32, _PRINCE_REGION2_IV_CODE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_CODE7;
#[doc = "`read()` method returns [prince_region2_iv_code7::R](prince_region2_iv_code7::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_CODE7 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_code7::W](prince_region2_iv_code7::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_CODE7 {}
#[doc = "."]
pub mod prince_region2_iv_code7;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_body6](prince_region2_iv_body6) module"]
pub type PRINCE_REGION2_IV_BODY6 = crate::Reg<u32, _PRINCE_REGION2_IV_BODY6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_BODY6;
#[doc = "`read()` method returns [prince_region2_iv_body6::R](prince_region2_iv_body6::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_BODY6 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_body6::W](prince_region2_iv_body6::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_BODY6 {}
#[doc = "."]
pub mod prince_region2_iv_body6;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_code8](prince_region2_iv_code8) module"]
pub type PRINCE_REGION2_IV_CODE8 = crate::Reg<u32, _PRINCE_REGION2_IV_CODE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_CODE8;
#[doc = "`read()` method returns [prince_region2_iv_code8::R](prince_region2_iv_code8::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_CODE8 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_code8::W](prince_region2_iv_code8::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_CODE8 {}
#[doc = "."]
pub mod prince_region2_iv_code8;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_body7](prince_region2_iv_body7) module"]
pub type PRINCE_REGION2_IV_BODY7 = crate::Reg<u32, _PRINCE_REGION2_IV_BODY7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_BODY7;
#[doc = "`read()` method returns [prince_region2_iv_body7::R](prince_region2_iv_body7::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_BODY7 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_body7::W](prince_region2_iv_body7::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_BODY7 {}
#[doc = "."]
pub mod prince_region2_iv_body7;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_code9](prince_region2_iv_code9) module"]
pub type PRINCE_REGION2_IV_CODE9 = crate::Reg<u32, _PRINCE_REGION2_IV_CODE9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_CODE9;
#[doc = "`read()` method returns [prince_region2_iv_code9::R](prince_region2_iv_code9::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_CODE9 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_code9::W](prince_region2_iv_code9::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_CODE9 {}
#[doc = "."]
pub mod prince_region2_iv_code9;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_body8](prince_region2_iv_body8) module"]
pub type PRINCE_REGION2_IV_BODY8 = crate::Reg<u32, _PRINCE_REGION2_IV_BODY8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_BODY8;
#[doc = "`read()` method returns [prince_region2_iv_body8::R](prince_region2_iv_body8::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_BODY8 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_body8::W](prince_region2_iv_body8::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_BODY8 {}
#[doc = "."]
pub mod prince_region2_iv_body8;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_code10](prince_region2_iv_code10) module"]
pub type PRINCE_REGION2_IV_CODE10 = crate::Reg<u32, _PRINCE_REGION2_IV_CODE10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_CODE10;
#[doc = "`read()` method returns [prince_region2_iv_code10::R](prince_region2_iv_code10::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_CODE10 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_code10::W](prince_region2_iv_code10::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_CODE10 {}
#[doc = "."]
pub mod prince_region2_iv_code10;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_body9](prince_region2_iv_body9) module"]
pub type PRINCE_REGION2_IV_BODY9 = crate::Reg<u32, _PRINCE_REGION2_IV_BODY9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_BODY9;
#[doc = "`read()` method returns [prince_region2_iv_body9::R](prince_region2_iv_body9::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_BODY9 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_body9::W](prince_region2_iv_body9::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_BODY9 {}
#[doc = "."]
pub mod prince_region2_iv_body9;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_code11](prince_region2_iv_code11) module"]
pub type PRINCE_REGION2_IV_CODE11 = crate::Reg<u32, _PRINCE_REGION2_IV_CODE11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_CODE11;
#[doc = "`read()` method returns [prince_region2_iv_code11::R](prince_region2_iv_code11::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_CODE11 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_code11::W](prince_region2_iv_code11::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_CODE11 {}
#[doc = "."]
pub mod prince_region2_iv_code11;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_body10](prince_region2_iv_body10) module"]
pub type PRINCE_REGION2_IV_BODY10 = crate::Reg<u32, _PRINCE_REGION2_IV_BODY10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_BODY10;
#[doc = "`read()` method returns [prince_region2_iv_body10::R](prince_region2_iv_body10::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_BODY10 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_body10::W](prince_region2_iv_body10::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_BODY10 {}
#[doc = "."]
pub mod prince_region2_iv_body10;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_code12](prince_region2_iv_code12) module"]
pub type PRINCE_REGION2_IV_CODE12 = crate::Reg<u32, _PRINCE_REGION2_IV_CODE12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_CODE12;
#[doc = "`read()` method returns [prince_region2_iv_code12::R](prince_region2_iv_code12::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_CODE12 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_code12::W](prince_region2_iv_code12::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_CODE12 {}
#[doc = "."]
pub mod prince_region2_iv_code12;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_body11](prince_region2_iv_body11) module"]
pub type PRINCE_REGION2_IV_BODY11 = crate::Reg<u32, _PRINCE_REGION2_IV_BODY11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_BODY11;
#[doc = "`read()` method returns [prince_region2_iv_body11::R](prince_region2_iv_body11::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_BODY11 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_body11::W](prince_region2_iv_body11::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_BODY11 {}
#[doc = "."]
pub mod prince_region2_iv_body11;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_region2_iv_code13](prince_region2_iv_code13) module"]
pub type PRINCE_REGION2_IV_CODE13 = crate::Reg<u32, _PRINCE_REGION2_IV_CODE13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_IV_CODE13;
#[doc = "`read()` method returns [prince_region2_iv_code13::R](prince_region2_iv_code13::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_IV_CODE13 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_iv_code13::W](prince_region2_iv_code13::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_IV_CODE13 {}
#[doc = "."]
pub mod prince_region2_iv_code13;
#[doc = "Customer Defined (Programable through ROM API)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [customer_defined](customer_defined) module"]
pub type CUSTOMER_DEFINED = crate::Reg<u32, _CUSTOMER_DEFINED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUSTOMER_DEFINED;
#[doc = "`read()` method returns [customer_defined::R](customer_defined::R) reader structure"]
impl crate::Readable for CUSTOMER_DEFINED {}
#[doc = "`write(|w| ..)` method takes [customer_defined::W](customer_defined::W) writer structure"]
impl crate::Writable for CUSTOMER_DEFINED {}
#[doc = "Customer Defined (Programable through ROM API)"]
pub mod customer_defined;
#[doc = "SHA256_DIGEST0 for DIGEST\\[31:0\\]
SHA256_DIGEST1 for DIGEST\\[63:32\\]
SHA256_DIGEST2 for DIGEST\\[95:64\\]
SHA256_DIGEST3 for DIGEST\\[127:96\\]
SHA256_DIGEST4 for DIGEST\\[159:128\\]
SHA256_DIGEST5 for DIGEST\\[191:160\\]
SHA256_DIGEST6 for DIGEST\\[223:192\\]
SHA256_DIGEST7 for DIGEST\\[255:224\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sha256_digest](sha256_digest) module"]
pub type SHA256_DIGEST = crate::Reg<u32, _SHA256_DIGEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHA256_DIGEST;
#[doc = "`read()` method returns [sha256_digest::R](sha256_digest::R) reader structure"]
impl crate::Readable for SHA256_DIGEST {}
#[doc = "`write(|w| ..)` method takes [sha256_digest::W](sha256_digest::W) writer structure"]
impl crate::Writable for SHA256_DIGEST {}
#[doc = "SHA256_DIGEST0 for DIGEST\\[31:0\\]
SHA256_DIGEST1 for DIGEST\\[63:32\\]
SHA256_DIGEST2 for DIGEST\\[95:64\\]
SHA256_DIGEST3 for DIGEST\\[127:96\\]
SHA256_DIGEST4 for DIGEST\\[159:128\\]
SHA256_DIGEST5 for DIGEST\\[191:160\\]
SHA256_DIGEST6 for DIGEST\\[223:192\\]
SHA256_DIGEST7 for DIGEST\\[255:224\\]"]
pub mod sha256_digest;
