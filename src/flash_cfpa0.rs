#[doc = r" Register block"]
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
    _reserved0: [u8; 4usize],
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
    #[doc = "0x30 - ."]
    pub prince_region0_iv_code0: PRINCE_REGION0_IV_CODE0,
    #[doc = "0x34 - ."]
    pub prince_region0_iv_code1: PRINCE_REGION0_IV_CODE1,
    #[doc = "0x38 - ."]
    pub prince_region0_iv_body0: PRINCE_REGION0_IV_BODY0,
    #[doc = "0x3c - ."]
    pub prince_region0_iv_body1: PRINCE_REGION0_IV_BODY1,
    #[doc = "0x40 - ."]
    pub prince_region0_iv_body2: PRINCE_REGION0_IV_BODY2,
    #[doc = "0x44 - ."]
    pub prince_region0_iv_body3: PRINCE_REGION0_IV_BODY3,
    #[doc = "0x48 - ."]
    pub prince_region0_iv_body4: PRINCE_REGION0_IV_BODY4,
    #[doc = "0x4c - ."]
    pub prince_region0_iv_body5: PRINCE_REGION0_IV_BODY5,
    #[doc = "0x50 - ."]
    pub prince_region0_iv_body6: PRINCE_REGION0_IV_BODY6,
    #[doc = "0x54 - ."]
    pub prince_region0_iv_body7: PRINCE_REGION0_IV_BODY7,
    #[doc = "0x58 - ."]
    pub prince_region0_iv_body8: PRINCE_REGION0_IV_BODY8,
    #[doc = "0x5c - ."]
    pub prince_region0_iv_body9: PRINCE_REGION0_IV_BODY9,
    #[doc = "0x60 - ."]
    pub prince_region0_iv_body10: PRINCE_REGION0_IV_BODY10,
    #[doc = "0x64 - ."]
    pub prince_region0_iv_body11: PRINCE_REGION0_IV_BODY11,
    #[doc = "0x68 - ."]
    pub prince_region1_iv_code0: PRINCE_REGION1_IV_CODE0,
    #[doc = "0x6c - ."]
    pub prince_region1_iv_code1: PRINCE_REGION1_IV_CODE1,
    #[doc = "0x70 - ."]
    pub prince_region1_iv_body0: PRINCE_REGION1_IV_BODY0,
    #[doc = "0x74 - ."]
    pub prince_region1_iv_body1: PRINCE_REGION1_IV_BODY1,
    #[doc = "0x78 - ."]
    pub prince_region1_iv_body2: PRINCE_REGION1_IV_BODY2,
    #[doc = "0x7c - ."]
    pub prince_region1_iv_body3: PRINCE_REGION1_IV_BODY3,
    #[doc = "0x80 - ."]
    pub prince_region1_iv_body4: PRINCE_REGION1_IV_BODY4,
    #[doc = "0x84 - ."]
    pub prince_region1_iv_body5: PRINCE_REGION1_IV_BODY5,
    #[doc = "0x88 - ."]
    pub prince_region1_iv_body6: PRINCE_REGION1_IV_BODY6,
    #[doc = "0x8c - ."]
    pub prince_region1_iv_body7: PRINCE_REGION1_IV_BODY7,
    #[doc = "0x90 - ."]
    pub prince_region1_iv_body8: PRINCE_REGION1_IV_BODY8,
    #[doc = "0x94 - ."]
    pub prince_region1_iv_body9: PRINCE_REGION1_IV_BODY9,
    #[doc = "0x98 - ."]
    pub prince_region1_iv_body10: PRINCE_REGION1_IV_BODY10,
    #[doc = "0x9c - ."]
    pub prince_region1_iv_body11: PRINCE_REGION1_IV_BODY11,
    #[doc = "0xa0 - ."]
    pub prince_region2_iv_code0: PRINCE_REGION2_IV_CODE0,
    #[doc = "0xa4 - ."]
    pub prince_region2_iv_code1: PRINCE_REGION2_IV_CODE1,
    #[doc = "0xa8 - ."]
    pub prince_region2_iv_body0: PRINCE_REGION2_IV_BODY0,
    #[doc = "0xac - ."]
    pub prince_region2_iv_body1: PRINCE_REGION2_IV_BODY1,
    #[doc = "0xb0 - ."]
    pub prince_region2_iv_body2: PRINCE_REGION2_IV_BODY2,
    #[doc = "0xb4 - ."]
    pub prince_region2_iv_body3: PRINCE_REGION2_IV_BODY3,
    #[doc = "0xb8 - ."]
    pub prince_region2_iv_body4: PRINCE_REGION2_IV_BODY4,
    #[doc = "0xbc - ."]
    pub prince_region2_iv_body5: PRINCE_REGION2_IV_BODY5,
    #[doc = "0xc0 - ."]
    pub prince_region2_iv_body6: PRINCE_REGION2_IV_BODY6,
    #[doc = "0xc4 - ."]
    pub prince_region2_iv_body7: PRINCE_REGION2_IV_BODY7,
    #[doc = "0xc8 - ."]
    pub prince_region2_iv_body8: PRINCE_REGION2_IV_BODY8,
    #[doc = "0xcc - ."]
    pub prince_region2_iv_body9: PRINCE_REGION2_IV_BODY9,
    #[doc = "0xd0 - ."]
    pub prince_region2_iv_body10: PRINCE_REGION2_IV_BODY10,
    #[doc = "0xd4 - ."]
    pub prince_region2_iv_body11: PRINCE_REGION2_IV_BODY11,
    _reserved1: [u8; 40usize],
    #[doc = "0x100 - Customer Defined (Programable through ROM API)"]
    pub customer_defined: [CUSTOMER_DEFINED; 56],
    #[doc = "0x1e0 - SHA256_DIGEST0 for DIGEST\\[31:0\\] SHA256_DIGEST1 for DIGEST\\[63:32\\] SHA256_DIGEST2 for DIGEST\\[95:64\\] SHA256_DIGEST3 for DIGEST\\[127:96\\] SHA256_DIGEST4 for DIGEST\\[159:128\\] SHA256_DIGEST5 for DIGEST\\[191:160\\] SHA256_DIGEST6 for DIGEST\\[223:192\\] SHA256_DIGEST7 for DIGEST\\[255:224\\]"]
    pub sha256_digest: [SHA256_DIGEST; 8],
}
#[doc = "."]
pub struct HEADER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod header;
#[doc = "."]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod version;
#[doc = "Secure firmware version (Monotonic counter)"]
pub struct S_FW_VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Secure firmware version (Monotonic counter)"]
pub mod s_fw_version;
#[doc = "Non-Secure firmware version (Monotonic counter)"]
pub struct NS_FW_VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Non-Secure firmware version (Monotonic counter)"]
pub mod ns_fw_version;
#[doc = "Image key revocation ID (Monotonic counter)"]
pub struct IMAGE_KEY_REVOKE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Image key revocation ID (Monotonic counter)"]
pub mod image_key_revoke;
#[doc = "."]
pub struct ROTKH_REVOKE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod rotkh_revoke;
#[doc = "."]
pub struct VENDOR_USAGE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod vendor_usage;
#[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
pub struct DCFG_CC_SOCU_PIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
pub mod dcfg_cc_socu_pin;
#[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
pub struct DCFG_CC_SOCU_DFLT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
pub mod dcfg_cc_socu_dflt;
#[doc = "Enable FA mode. SET_FA_MODE Command should write 0xC33CA55A to this word to indicate boot ROM to enter FA mode."]
pub struct ENABLE_FA_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable FA mode. SET_FA_MODE Command should write 0xC33CA55A to this word to indicate boot ROM to enter FA mode."]
pub mod enable_fa_mode;
#[doc = "CMPA Page programming on going. This field shall be set to 0x5CC55AA5 in the active CFPA page each time CMPA page programming is going on. It shall always be set to 0x00000000 in the CFPA scratch area."]
pub struct CMPA_PROG_IN_PROGRESS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CMPA Page programming on going. This field shall be set to 0x5CC55AA5 in the active CFPA page each time CMPA page programming is going on. It shall always be set to 0x00000000 in the CFPA scratch area."]
pub mod cmpa_prog_in_progress;
#[doc = "."]
pub struct PRINCE_REGION0_IV_CODE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_code0;
#[doc = "."]
pub struct PRINCE_REGION0_IV_HEADER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_header0;
#[doc = "."]
pub struct PRINCE_REGION0_IV_CODE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_code1;
#[doc = "."]
pub struct PRINCE_REGION0_IV_HEADER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_header1;
#[doc = "."]
pub struct PRINCE_REGION0_IV_BODY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_body0;
#[doc = "."]
pub struct PRINCE_REGION0_IV_CODE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_code2;
#[doc = "."]
pub struct PRINCE_REGION0_IV_BODY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_body1;
#[doc = "."]
pub struct PRINCE_REGION0_IV_CODE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_code3;
#[doc = "."]
pub struct PRINCE_REGION0_IV_BODY2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_body2;
#[doc = "."]
pub struct PRINCE_REGION0_IV_CODE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_code4;
#[doc = "."]
pub struct PRINCE_REGION0_IV_BODY3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_body3;
#[doc = "."]
pub struct PRINCE_REGION0_IV_CODE5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_code5;
#[doc = "."]
pub struct PRINCE_REGION0_IV_BODY4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_body4;
#[doc = "."]
pub struct PRINCE_REGION0_IV_CODE6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_code6;
#[doc = "."]
pub struct PRINCE_REGION0_IV_BODY5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_body5;
#[doc = "."]
pub struct PRINCE_REGION0_IV_CODE7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_code7;
#[doc = "."]
pub struct PRINCE_REGION0_IV_BODY6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_body6;
#[doc = "."]
pub struct PRINCE_REGION0_IV_CODE8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_code8;
#[doc = "."]
pub struct PRINCE_REGION0_IV_BODY7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_body7;
#[doc = "."]
pub struct PRINCE_REGION0_IV_CODE9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_code9;
#[doc = "."]
pub struct PRINCE_REGION0_IV_BODY8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_body8;
#[doc = "."]
pub struct PRINCE_REGION0_IV_CODE10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_code10;
#[doc = "."]
pub struct PRINCE_REGION0_IV_BODY9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_body9;
#[doc = "."]
pub struct PRINCE_REGION0_IV_CODE11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_code11;
#[doc = "."]
pub struct PRINCE_REGION0_IV_BODY10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_body10;
#[doc = "."]
pub struct PRINCE_REGION0_IV_CODE12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_code12;
#[doc = "."]
pub struct PRINCE_REGION0_IV_BODY11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_body11;
#[doc = "."]
pub struct PRINCE_REGION0_IV_CODE13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_iv_code13;
#[doc = "."]
pub struct PRINCE_REGION1_IV_CODE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_code0;
#[doc = "."]
pub struct PRINCE_REGION1_IV_HEADER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_header0;
#[doc = "."]
pub struct PRINCE_REGION1_IV_CODE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_code1;
#[doc = "."]
pub struct PRINCE_REGION1_IV_HEADER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_header1;
#[doc = "."]
pub struct PRINCE_REGION1_IV_BODY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_body0;
#[doc = "."]
pub struct PRINCE_REGION1_IV_CODE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_code2;
#[doc = "."]
pub struct PRINCE_REGION1_IV_BODY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_body1;
#[doc = "."]
pub struct PRINCE_REGION1_IV_CODE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_code3;
#[doc = "."]
pub struct PRINCE_REGION1_IV_BODY2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_body2;
#[doc = "."]
pub struct PRINCE_REGION1_IV_CODE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_code4;
#[doc = "."]
pub struct PRINCE_REGION1_IV_BODY3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_body3;
#[doc = "."]
pub struct PRINCE_REGION1_IV_CODE5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_code5;
#[doc = "."]
pub struct PRINCE_REGION1_IV_BODY4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_body4;
#[doc = "."]
pub struct PRINCE_REGION1_IV_CODE6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_code6;
#[doc = "."]
pub struct PRINCE_REGION1_IV_BODY5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_body5;
#[doc = "."]
pub struct PRINCE_REGION1_IV_CODE7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_code7;
#[doc = "."]
pub struct PRINCE_REGION1_IV_BODY6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_body6;
#[doc = "."]
pub struct PRINCE_REGION1_IV_CODE8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_code8;
#[doc = "."]
pub struct PRINCE_REGION1_IV_BODY7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_body7;
#[doc = "."]
pub struct PRINCE_REGION1_IV_CODE9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_code9;
#[doc = "."]
pub struct PRINCE_REGION1_IV_BODY8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_body8;
#[doc = "."]
pub struct PRINCE_REGION1_IV_CODE10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_code10;
#[doc = "."]
pub struct PRINCE_REGION1_IV_BODY9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_body9;
#[doc = "."]
pub struct PRINCE_REGION1_IV_CODE11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_code11;
#[doc = "."]
pub struct PRINCE_REGION1_IV_BODY10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_body10;
#[doc = "."]
pub struct PRINCE_REGION1_IV_CODE12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_code12;
#[doc = "."]
pub struct PRINCE_REGION1_IV_BODY11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_body11;
#[doc = "."]
pub struct PRINCE_REGION1_IV_CODE13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_iv_code13;
#[doc = "."]
pub struct PRINCE_REGION2_IV_CODE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_code0;
#[doc = "."]
pub struct PRINCE_REGION2_IV_HEADER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_header0;
#[doc = "."]
pub struct PRINCE_REGION2_IV_CODE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_code1;
#[doc = "."]
pub struct PRINCE_REGION2_IV_HEADER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_header1;
#[doc = "."]
pub struct PRINCE_REGION2_IV_BODY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_body0;
#[doc = "."]
pub struct PRINCE_REGION2_IV_CODE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_code2;
#[doc = "."]
pub struct PRINCE_REGION2_IV_BODY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_body1;
#[doc = "."]
pub struct PRINCE_REGION2_IV_CODE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_code3;
#[doc = "."]
pub struct PRINCE_REGION2_IV_BODY2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_body2;
#[doc = "."]
pub struct PRINCE_REGION2_IV_CODE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_code4;
#[doc = "."]
pub struct PRINCE_REGION2_IV_BODY3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_body3;
#[doc = "."]
pub struct PRINCE_REGION2_IV_CODE5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_code5;
#[doc = "."]
pub struct PRINCE_REGION2_IV_BODY4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_body4;
#[doc = "."]
pub struct PRINCE_REGION2_IV_CODE6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_code6;
#[doc = "."]
pub struct PRINCE_REGION2_IV_BODY5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_body5;
#[doc = "."]
pub struct PRINCE_REGION2_IV_CODE7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_code7;
#[doc = "."]
pub struct PRINCE_REGION2_IV_BODY6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_body6;
#[doc = "."]
pub struct PRINCE_REGION2_IV_CODE8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_code8;
#[doc = "."]
pub struct PRINCE_REGION2_IV_BODY7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_body7;
#[doc = "."]
pub struct PRINCE_REGION2_IV_CODE9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_code9;
#[doc = "."]
pub struct PRINCE_REGION2_IV_BODY8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_body8;
#[doc = "."]
pub struct PRINCE_REGION2_IV_CODE10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_code10;
#[doc = "."]
pub struct PRINCE_REGION2_IV_BODY9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_body9;
#[doc = "."]
pub struct PRINCE_REGION2_IV_CODE11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_code11;
#[doc = "."]
pub struct PRINCE_REGION2_IV_BODY10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_body10;
#[doc = "."]
pub struct PRINCE_REGION2_IV_CODE12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_code12;
#[doc = "."]
pub struct PRINCE_REGION2_IV_BODY11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_body11;
#[doc = "."]
pub struct PRINCE_REGION2_IV_CODE13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_iv_code13;
#[doc = "Customer Defined (Programable through ROM API)"]
pub struct CUSTOMER_DEFINED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Customer Defined (Programable through ROM API)"]
pub mod customer_defined;
#[doc = "SHA256_DIGEST0 for DIGEST\\[31:0\\] SHA256_DIGEST1 for DIGEST\\[63:32\\] SHA256_DIGEST2 for DIGEST\\[95:64\\] SHA256_DIGEST3 for DIGEST\\[127:96\\] SHA256_DIGEST4 for DIGEST\\[159:128\\] SHA256_DIGEST5 for DIGEST\\[191:160\\] SHA256_DIGEST6 for DIGEST\\[223:192\\] SHA256_DIGEST7 for DIGEST\\[255:224\\]"]
pub struct SHA256_DIGEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SHA256_DIGEST0 for DIGEST\\[31:0\\] SHA256_DIGEST1 for DIGEST\\[63:32\\] SHA256_DIGEST2 for DIGEST\\[95:64\\] SHA256_DIGEST3 for DIGEST\\[127:96\\] SHA256_DIGEST4 for DIGEST\\[159:128\\] SHA256_DIGEST5 for DIGEST\\[191:160\\] SHA256_DIGEST6 for DIGEST\\[223:192\\] SHA256_DIGEST7 for DIGEST\\[255:224\\]"]
pub mod sha256_digest;
