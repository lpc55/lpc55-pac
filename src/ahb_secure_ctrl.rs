#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Security access rules for Flash and ROM slaves."]
    pub sec_ctrl_flash_rom_slave_rule: SEC_CTRL_FLASH_ROM_SLAVE_RULE,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
    pub sec_ctrl_flash_mem_rule0: SEC_CTRL_FLASH_MEM_RULE0,
    #[doc = "0x14 - Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
    pub sec_ctrl_flash_mem_rule1: SEC_CTRL_FLASH_MEM_RULE1,
    #[doc = "0x18 - Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
    pub sec_ctrl_flash_mem_rule2: SEC_CTRL_FLASH_MEM_RULE2,
    _reserved4: [u8; 4usize],
    #[doc = "0x20 - Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    pub sec_ctrl_rom_mem_rule0: SEC_CTRL_ROM_MEM_RULE0,
    #[doc = "0x24 - Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    pub sec_ctrl_rom_mem_rule1: SEC_CTRL_ROM_MEM_RULE1,
    #[doc = "0x28 - Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    pub sec_ctrl_rom_mem_rule2: SEC_CTRL_ROM_MEM_RULE2,
    #[doc = "0x2c - Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    pub sec_ctrl_rom_mem_rule3: SEC_CTRL_ROM_MEM_RULE3,
    #[doc = "0x30 - Security access rules for RAMX slaves."]
    pub sec_ctrl_ramx_slave_rule: SEC_CTRL_RAMX_SLAVE_RULE,
    _reserved9: [u8; 12usize],
    #[doc = "0x40 - Security access rules for RAMX slaves."]
    pub sec_ctrl_ramx_mem_rule0: SEC_CTRL_RAMX_MEM_RULE0,
    _reserved10: [u8; 12usize],
    #[doc = "0x50 - Security access rules for RAM0 slaves."]
    pub sec_ctrl_ram0_slave_rule: SEC_CTRL_RAM0_SLAVE_RULE,
    _reserved11: [u8; 12usize],
    #[doc = "0x60 - Security access rules for RAM0 slaves."]
    pub sec_ctrl_ram0_mem_rule0: SEC_CTRL_RAM0_MEM_RULE0,
    #[doc = "0x64 - Security access rules for RAM0 slaves."]
    pub sec_ctrl_ram0_mem_rule1: SEC_CTRL_RAM0_MEM_RULE1,
    _reserved13: [u8; 8usize],
    #[doc = "0x70 - Security access rules for RAM1 slaves."]
    pub sec_ctrl_ram1_slave_rule: SEC_CTRL_RAM1_SLAVE_RULE,
    _reserved14: [u8; 12usize],
    #[doc = "0x80 - Security access rules for RAM1 slaves."]
    pub sec_ctrl_ram1_mem_rule0: SEC_CTRL_RAM1_MEM_RULE0,
    #[doc = "0x84 - Security access rules for RAM1 slaves."]
    pub sec_ctrl_ram1_mem_rule1: SEC_CTRL_RAM1_MEM_RULE1,
    _reserved16: [u8; 8usize],
    #[doc = "0x90 - Security access rules for RAM2 slaves."]
    pub sec_ctrl_ram2_slave_rule: SEC_CTRL_RAM2_SLAVE_RULE,
    _reserved17: [u8; 12usize],
    #[doc = "0xa0 - Security access rules for RAM2 slaves."]
    pub sec_ctrl_ram2_mem_rule0: SEC_CTRL_RAM2_MEM_RULE0,
    #[doc = "0xa4 - Security access rules for RAM2 slaves."]
    pub sec_ctrl_ram2_mem_rule1: SEC_CTRL_RAM2_MEM_RULE1,
    _reserved19: [u8; 8usize],
    #[doc = "0xb0 - Security access rules for RAM3 slaves."]
    pub sec_ctrl_ram3_slave_rule: SEC_CTRL_RAM3_SLAVE_RULE,
    _reserved20: [u8; 12usize],
    #[doc = "0xc0 - Security access rules for RAM3 slaves."]
    pub sec_ctrl_ram3_mem_rule0: SEC_CTRL_RAM3_MEM_RULE0,
    #[doc = "0xc4 - Security access rules for RAM3 slaves."]
    pub sec_ctrl_ram3_mem_rule1: SEC_CTRL_RAM3_MEM_RULE1,
    _reserved22: [u8; 8usize],
    #[doc = "0xd0 - Security access rules for RAM4 slaves."]
    pub sec_ctrl_ram4_slave_rule: SEC_CTRL_RAM4_SLAVE_RULE,
    _reserved23: [u8; 12usize],
    #[doc = "0xe0 - Security access rules for RAM4 slaves."]
    pub sec_ctrl_ram4_mem_rule0: SEC_CTRL_RAM4_MEM_RULE0,
    _reserved24: [u8; 12usize],
    #[doc = "0xf0 - Security access rules for both APB Bridges slaves."]
    pub sec_ctrl_apb_bridge_slave_rule: SEC_CTRL_APB_BRIDGE_SLAVE_RULE,
    _reserved25: [u8; 12usize],
    #[doc = "0x100 - Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
    pub sec_ctrl_apb_bridge0_mem_ctrl0: SEC_CTRL_APB_BRIDGE0_MEM_CTRL0,
    #[doc = "0x104 - Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
    pub sec_ctrl_apb_bridge0_mem_ctrl1: SEC_CTRL_APB_BRIDGE0_MEM_CTRL1,
    #[doc = "0x108 - Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
    pub sec_ctrl_apb_bridge0_mem_ctrl2: SEC_CTRL_APB_BRIDGE0_MEM_CTRL2,
    _reserved28: [u8; 4usize],
    #[doc = "0x110 - Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    pub sec_ctrl_apb_bridge1_mem_ctrl0: SEC_CTRL_APB_BRIDGE1_MEM_CTRL0,
    #[doc = "0x114 - Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    pub sec_ctrl_apb_bridge1_mem_ctrl1: SEC_CTRL_APB_BRIDGE1_MEM_CTRL1,
    #[doc = "0x118 - Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    pub sec_ctrl_apb_bridge1_mem_ctrl2: SEC_CTRL_APB_BRIDGE1_MEM_CTRL2,
    #[doc = "0x11c - Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    pub sec_ctrl_apb_bridge1_mem_ctrl3: SEC_CTRL_APB_BRIDGE1_MEM_CTRL3,
    #[doc = "0x120 - Security access rules for AHB peripherals."]
    pub sec_ctrl_ahb_port8_slave0_rule: SEC_CTRL_AHB_PORT8_SLAVE0_RULE,
    #[doc = "0x124 - Security access rules for AHB peripherals."]
    pub sec_ctrl_ahb_port8_slave1_rule: SEC_CTRL_AHB_PORT8_SLAVE1_RULE,
    _reserved34: [u8; 8usize],
    #[doc = "0x130 - Security access rules for AHB peripherals."]
    pub sec_ctrl_ahb_port9_slave0_rule: SEC_CTRL_AHB_PORT9_SLAVE0_RULE,
    #[doc = "0x134 - Security access rules for AHB peripherals."]
    pub sec_ctrl_ahb_port9_slave1_rule: SEC_CTRL_AHB_PORT9_SLAVE1_RULE,
    _reserved36: [u8; 8usize],
    #[doc = "0x140 - Security access rules for AHB peripherals."]
    pub sec_ctrl_ahb_port10_slave0_rule: SEC_CTRL_AHB_PORT10_SLAVE0_RULE,
    #[doc = "0x144 - Security access rules for AHB peripherals."]
    pub sec_ctrl_ahb_port10_slave1_rule: SEC_CTRL_AHB_PORT10_SLAVE1_RULE,
    _reserved38: [u8; 8usize],
    #[doc = "0x150 - Security access rules for AHB_SEC_CTRL_AHB."]
    pub sec_ctrl_ahb_sec_ctrl_mem_rule: SEC_CTRL_AHB_SEC_CTRL_MEM_RULE,
    _reserved39: [u8; 12usize],
    #[doc = "0x160 - Security access rules for USB High speed RAM slaves."]
    pub sec_ctrl_usb_hs_slave_rule: SEC_CTRL_USB_HS_SLAVE_RULE,
    _reserved40: [u8; 12usize],
    #[doc = "0x170 - Security access rules for RAM_USB_HS."]
    pub sec_ctrl_usb_hs_mem_rule: SEC_CTRL_USB_HS_MEM_RULE,
    _reserved41: [u8; 3212usize],
    #[doc = "0xe00 - most recent security violation address for AHB port n"]
    pub sec_vio_addr: [SEC_VIO_ADDR; 12],
    _reserved42: [u8; 80usize],
    #[doc = "0xe80 - most recent security violation miscellaneous information for AHB port n"]
    pub sec_vio_misc_info: [SEC_VIO_MISC_INFO; 12],
    _reserved43: [u8; 80usize],
    #[doc = "0xf00 - security violation address/information registers valid flags"]
    pub sec_vio_info_valid: SEC_VIO_INFO_VALID,
    _reserved44: [u8; 124usize],
    #[doc = "0xf80 - Secure GPIO mask for port 0 pins."]
    pub sec_gpio_mask0: SEC_GPIO_MASK0,
    #[doc = "0xf84 - Secure GPIO mask for port 1 pins."]
    pub sec_gpio_mask1: SEC_GPIO_MASK1,
    _reserved46: [u8; 8usize],
    #[doc = "0xf90 - Secure Interrupt mask for CPU1"]
    pub sec_cpu_int_mask0: SEC_CPU_INT_MASK0,
    #[doc = "0xf94 - Secure Interrupt mask for CPU1"]
    pub sec_cpu_int_mask1: SEC_CPU_INT_MASK1,
    _reserved48: [u8; 36usize],
    #[doc = "0xfbc - Security General Purpose register access control."]
    pub sec_mask_lock: SEC_MASK_LOCK,
    _reserved49: [u8; 16usize],
    #[doc = "0xfd0 - master secure level register"]
    pub master_sec_level: MASTER_SEC_LEVEL,
    #[doc = "0xfd4 - master secure level anti-pole register"]
    pub master_sec_anti_pol_reg: MASTER_SEC_ANTI_POL_REG,
    _reserved51: [u8; 20usize],
    #[doc = "0xfec - Miscalleneous control signals for in Cortex M33 (CPU0)"]
    pub cpu0_lock_reg: CPU0_LOCK_REG,
    #[doc = "0xff0 - Miscalleneous control signals for in micro-Cortex M33 (CPU1)"]
    pub cpu1_lock_reg: CPU1_LOCK_REG,
    _reserved53: [u8; 4usize],
    #[doc = "0xff8 - secure control duplicate register"]
    pub misc_ctrl_dp_reg: MISC_CTRL_DP_REG,
    #[doc = "0xffc - secure control register"]
    pub misc_ctrl_reg: MISC_CTRL_REG,
}
#[doc = "Security access rules for Flash and ROM slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_flash_rom_slave_rule](sec_ctrl_flash_rom_slave_rule) module"]
pub type SEC_CTRL_FLASH_ROM_SLAVE_RULE = crate::Reg<u32, _SEC_CTRL_FLASH_ROM_SLAVE_RULE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_FLASH_ROM_SLAVE_RULE;
#[doc = "`read()` method returns [sec_ctrl_flash_rom_slave_rule::R](sec_ctrl_flash_rom_slave_rule::R) reader structure"]
impl crate::Readable for SEC_CTRL_FLASH_ROM_SLAVE_RULE {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_flash_rom_slave_rule::W](sec_ctrl_flash_rom_slave_rule::W) writer structure"]
impl crate::Writable for SEC_CTRL_FLASH_ROM_SLAVE_RULE {}
#[doc = "Security access rules for Flash and ROM slaves."]
pub mod sec_ctrl_flash_rom_slave_rule;
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_flash_mem_rule0](sec_ctrl_flash_mem_rule0) module"]
pub type SEC_CTRL_FLASH_MEM_RULE0 = crate::Reg<u32, _SEC_CTRL_FLASH_MEM_RULE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_FLASH_MEM_RULE0;
#[doc = "`read()` method returns [sec_ctrl_flash_mem_rule0::R](sec_ctrl_flash_mem_rule0::R) reader structure"]
impl crate::Readable for SEC_CTRL_FLASH_MEM_RULE0 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_flash_mem_rule0::W](sec_ctrl_flash_mem_rule0::W) writer structure"]
impl crate::Writable for SEC_CTRL_FLASH_MEM_RULE0 {}
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
pub mod sec_ctrl_flash_mem_rule0;
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_flash_mem_rule1](sec_ctrl_flash_mem_rule1) module"]
pub type SEC_CTRL_FLASH_MEM_RULE1 = crate::Reg<u32, _SEC_CTRL_FLASH_MEM_RULE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_FLASH_MEM_RULE1;
#[doc = "`read()` method returns [sec_ctrl_flash_mem_rule1::R](sec_ctrl_flash_mem_rule1::R) reader structure"]
impl crate::Readable for SEC_CTRL_FLASH_MEM_RULE1 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_flash_mem_rule1::W](sec_ctrl_flash_mem_rule1::W) writer structure"]
impl crate::Writable for SEC_CTRL_FLASH_MEM_RULE1 {}
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
pub mod sec_ctrl_flash_mem_rule1;
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_flash_mem_rule2](sec_ctrl_flash_mem_rule2) module"]
pub type SEC_CTRL_FLASH_MEM_RULE2 = crate::Reg<u32, _SEC_CTRL_FLASH_MEM_RULE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_FLASH_MEM_RULE2;
#[doc = "`read()` method returns [sec_ctrl_flash_mem_rule2::R](sec_ctrl_flash_mem_rule2::R) reader structure"]
impl crate::Readable for SEC_CTRL_FLASH_MEM_RULE2 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_flash_mem_rule2::W](sec_ctrl_flash_mem_rule2::W) writer structure"]
impl crate::Writable for SEC_CTRL_FLASH_MEM_RULE2 {}
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
pub mod sec_ctrl_flash_mem_rule2;
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_rom_mem_rule0](sec_ctrl_rom_mem_rule0) module"]
pub type SEC_CTRL_ROM_MEM_RULE0 = crate::Reg<u32, _SEC_CTRL_ROM_MEM_RULE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_ROM_MEM_RULE0;
#[doc = "`read()` method returns [sec_ctrl_rom_mem_rule0::R](sec_ctrl_rom_mem_rule0::R) reader structure"]
impl crate::Readable for SEC_CTRL_ROM_MEM_RULE0 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_rom_mem_rule0::W](sec_ctrl_rom_mem_rule0::W) writer structure"]
impl crate::Writable for SEC_CTRL_ROM_MEM_RULE0 {}
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
pub mod sec_ctrl_rom_mem_rule0;
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_rom_mem_rule1](sec_ctrl_rom_mem_rule1) module"]
pub type SEC_CTRL_ROM_MEM_RULE1 = crate::Reg<u32, _SEC_CTRL_ROM_MEM_RULE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_ROM_MEM_RULE1;
#[doc = "`read()` method returns [sec_ctrl_rom_mem_rule1::R](sec_ctrl_rom_mem_rule1::R) reader structure"]
impl crate::Readable for SEC_CTRL_ROM_MEM_RULE1 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_rom_mem_rule1::W](sec_ctrl_rom_mem_rule1::W) writer structure"]
impl crate::Writable for SEC_CTRL_ROM_MEM_RULE1 {}
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
pub mod sec_ctrl_rom_mem_rule1;
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_rom_mem_rule2](sec_ctrl_rom_mem_rule2) module"]
pub type SEC_CTRL_ROM_MEM_RULE2 = crate::Reg<u32, _SEC_CTRL_ROM_MEM_RULE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_ROM_MEM_RULE2;
#[doc = "`read()` method returns [sec_ctrl_rom_mem_rule2::R](sec_ctrl_rom_mem_rule2::R) reader structure"]
impl crate::Readable for SEC_CTRL_ROM_MEM_RULE2 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_rom_mem_rule2::W](sec_ctrl_rom_mem_rule2::W) writer structure"]
impl crate::Writable for SEC_CTRL_ROM_MEM_RULE2 {}
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
pub mod sec_ctrl_rom_mem_rule2;
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_rom_mem_rule3](sec_ctrl_rom_mem_rule3) module"]
pub type SEC_CTRL_ROM_MEM_RULE3 = crate::Reg<u32, _SEC_CTRL_ROM_MEM_RULE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_ROM_MEM_RULE3;
#[doc = "`read()` method returns [sec_ctrl_rom_mem_rule3::R](sec_ctrl_rom_mem_rule3::R) reader structure"]
impl crate::Readable for SEC_CTRL_ROM_MEM_RULE3 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_rom_mem_rule3::W](sec_ctrl_rom_mem_rule3::W) writer structure"]
impl crate::Writable for SEC_CTRL_ROM_MEM_RULE3 {}
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
pub mod sec_ctrl_rom_mem_rule3;
#[doc = "Security access rules for RAMX slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ramx_slave_rule](sec_ctrl_ramx_slave_rule) module"]
pub type SEC_CTRL_RAMX_SLAVE_RULE = crate::Reg<u32, _SEC_CTRL_RAMX_SLAVE_RULE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_RAMX_SLAVE_RULE;
#[doc = "`read()` method returns [sec_ctrl_ramx_slave_rule::R](sec_ctrl_ramx_slave_rule::R) reader structure"]
impl crate::Readable for SEC_CTRL_RAMX_SLAVE_RULE {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ramx_slave_rule::W](sec_ctrl_ramx_slave_rule::W) writer structure"]
impl crate::Writable for SEC_CTRL_RAMX_SLAVE_RULE {}
#[doc = "Security access rules for RAMX slaves."]
pub mod sec_ctrl_ramx_slave_rule;
#[doc = "Security access rules for RAMX slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ramx_mem_rule0](sec_ctrl_ramx_mem_rule0) module"]
pub type SEC_CTRL_RAMX_MEM_RULE0 = crate::Reg<u32, _SEC_CTRL_RAMX_MEM_RULE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_RAMX_MEM_RULE0;
#[doc = "`read()` method returns [sec_ctrl_ramx_mem_rule0::R](sec_ctrl_ramx_mem_rule0::R) reader structure"]
impl crate::Readable for SEC_CTRL_RAMX_MEM_RULE0 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ramx_mem_rule0::W](sec_ctrl_ramx_mem_rule0::W) writer structure"]
impl crate::Writable for SEC_CTRL_RAMX_MEM_RULE0 {}
#[doc = "Security access rules for RAMX slaves."]
pub mod sec_ctrl_ramx_mem_rule0;
#[doc = "Security access rules for RAM0 slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ram0_slave_rule](sec_ctrl_ram0_slave_rule) module"]
pub type SEC_CTRL_RAM0_SLAVE_RULE = crate::Reg<u32, _SEC_CTRL_RAM0_SLAVE_RULE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_RAM0_SLAVE_RULE;
#[doc = "`read()` method returns [sec_ctrl_ram0_slave_rule::R](sec_ctrl_ram0_slave_rule::R) reader structure"]
impl crate::Readable for SEC_CTRL_RAM0_SLAVE_RULE {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ram0_slave_rule::W](sec_ctrl_ram0_slave_rule::W) writer structure"]
impl crate::Writable for SEC_CTRL_RAM0_SLAVE_RULE {}
#[doc = "Security access rules for RAM0 slaves."]
pub mod sec_ctrl_ram0_slave_rule;
#[doc = "Security access rules for RAM0 slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ram0_mem_rule0](sec_ctrl_ram0_mem_rule0) module"]
pub type SEC_CTRL_RAM0_MEM_RULE0 = crate::Reg<u32, _SEC_CTRL_RAM0_MEM_RULE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_RAM0_MEM_RULE0;
#[doc = "`read()` method returns [sec_ctrl_ram0_mem_rule0::R](sec_ctrl_ram0_mem_rule0::R) reader structure"]
impl crate::Readable for SEC_CTRL_RAM0_MEM_RULE0 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ram0_mem_rule0::W](sec_ctrl_ram0_mem_rule0::W) writer structure"]
impl crate::Writable for SEC_CTRL_RAM0_MEM_RULE0 {}
#[doc = "Security access rules for RAM0 slaves."]
pub mod sec_ctrl_ram0_mem_rule0;
#[doc = "Security access rules for RAM0 slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ram0_mem_rule1](sec_ctrl_ram0_mem_rule1) module"]
pub type SEC_CTRL_RAM0_MEM_RULE1 = crate::Reg<u32, _SEC_CTRL_RAM0_MEM_RULE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_RAM0_MEM_RULE1;
#[doc = "`read()` method returns [sec_ctrl_ram0_mem_rule1::R](sec_ctrl_ram0_mem_rule1::R) reader structure"]
impl crate::Readable for SEC_CTRL_RAM0_MEM_RULE1 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ram0_mem_rule1::W](sec_ctrl_ram0_mem_rule1::W) writer structure"]
impl crate::Writable for SEC_CTRL_RAM0_MEM_RULE1 {}
#[doc = "Security access rules for RAM0 slaves."]
pub mod sec_ctrl_ram0_mem_rule1;
#[doc = "Security access rules for RAM1 slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ram1_slave_rule](sec_ctrl_ram1_slave_rule) module"]
pub type SEC_CTRL_RAM1_SLAVE_RULE = crate::Reg<u32, _SEC_CTRL_RAM1_SLAVE_RULE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_RAM1_SLAVE_RULE;
#[doc = "`read()` method returns [sec_ctrl_ram1_slave_rule::R](sec_ctrl_ram1_slave_rule::R) reader structure"]
impl crate::Readable for SEC_CTRL_RAM1_SLAVE_RULE {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ram1_slave_rule::W](sec_ctrl_ram1_slave_rule::W) writer structure"]
impl crate::Writable for SEC_CTRL_RAM1_SLAVE_RULE {}
#[doc = "Security access rules for RAM1 slaves."]
pub mod sec_ctrl_ram1_slave_rule;
#[doc = "Security access rules for RAM1 slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ram1_mem_rule0](sec_ctrl_ram1_mem_rule0) module"]
pub type SEC_CTRL_RAM1_MEM_RULE0 = crate::Reg<u32, _SEC_CTRL_RAM1_MEM_RULE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_RAM1_MEM_RULE0;
#[doc = "`read()` method returns [sec_ctrl_ram1_mem_rule0::R](sec_ctrl_ram1_mem_rule0::R) reader structure"]
impl crate::Readable for SEC_CTRL_RAM1_MEM_RULE0 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ram1_mem_rule0::W](sec_ctrl_ram1_mem_rule0::W) writer structure"]
impl crate::Writable for SEC_CTRL_RAM1_MEM_RULE0 {}
#[doc = "Security access rules for RAM1 slaves."]
pub mod sec_ctrl_ram1_mem_rule0;
#[doc = "Security access rules for RAM1 slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ram1_mem_rule1](sec_ctrl_ram1_mem_rule1) module"]
pub type SEC_CTRL_RAM1_MEM_RULE1 = crate::Reg<u32, _SEC_CTRL_RAM1_MEM_RULE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_RAM1_MEM_RULE1;
#[doc = "`read()` method returns [sec_ctrl_ram1_mem_rule1::R](sec_ctrl_ram1_mem_rule1::R) reader structure"]
impl crate::Readable for SEC_CTRL_RAM1_MEM_RULE1 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ram1_mem_rule1::W](sec_ctrl_ram1_mem_rule1::W) writer structure"]
impl crate::Writable for SEC_CTRL_RAM1_MEM_RULE1 {}
#[doc = "Security access rules for RAM1 slaves."]
pub mod sec_ctrl_ram1_mem_rule1;
#[doc = "Security access rules for RAM2 slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ram2_slave_rule](sec_ctrl_ram2_slave_rule) module"]
pub type SEC_CTRL_RAM2_SLAVE_RULE = crate::Reg<u32, _SEC_CTRL_RAM2_SLAVE_RULE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_RAM2_SLAVE_RULE;
#[doc = "`read()` method returns [sec_ctrl_ram2_slave_rule::R](sec_ctrl_ram2_slave_rule::R) reader structure"]
impl crate::Readable for SEC_CTRL_RAM2_SLAVE_RULE {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ram2_slave_rule::W](sec_ctrl_ram2_slave_rule::W) writer structure"]
impl crate::Writable for SEC_CTRL_RAM2_SLAVE_RULE {}
#[doc = "Security access rules for RAM2 slaves."]
pub mod sec_ctrl_ram2_slave_rule;
#[doc = "Security access rules for RAM2 slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ram2_mem_rule0](sec_ctrl_ram2_mem_rule0) module"]
pub type SEC_CTRL_RAM2_MEM_RULE0 = crate::Reg<u32, _SEC_CTRL_RAM2_MEM_RULE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_RAM2_MEM_RULE0;
#[doc = "`read()` method returns [sec_ctrl_ram2_mem_rule0::R](sec_ctrl_ram2_mem_rule0::R) reader structure"]
impl crate::Readable for SEC_CTRL_RAM2_MEM_RULE0 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ram2_mem_rule0::W](sec_ctrl_ram2_mem_rule0::W) writer structure"]
impl crate::Writable for SEC_CTRL_RAM2_MEM_RULE0 {}
#[doc = "Security access rules for RAM2 slaves."]
pub mod sec_ctrl_ram2_mem_rule0;
#[doc = "Security access rules for RAM2 slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ram2_mem_rule1](sec_ctrl_ram2_mem_rule1) module"]
pub type SEC_CTRL_RAM2_MEM_RULE1 = crate::Reg<u32, _SEC_CTRL_RAM2_MEM_RULE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_RAM2_MEM_RULE1;
#[doc = "`read()` method returns [sec_ctrl_ram2_mem_rule1::R](sec_ctrl_ram2_mem_rule1::R) reader structure"]
impl crate::Readable for SEC_CTRL_RAM2_MEM_RULE1 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ram2_mem_rule1::W](sec_ctrl_ram2_mem_rule1::W) writer structure"]
impl crate::Writable for SEC_CTRL_RAM2_MEM_RULE1 {}
#[doc = "Security access rules for RAM2 slaves."]
pub mod sec_ctrl_ram2_mem_rule1;
#[doc = "Security access rules for RAM3 slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ram3_slave_rule](sec_ctrl_ram3_slave_rule) module"]
pub type SEC_CTRL_RAM3_SLAVE_RULE = crate::Reg<u32, _SEC_CTRL_RAM3_SLAVE_RULE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_RAM3_SLAVE_RULE;
#[doc = "`read()` method returns [sec_ctrl_ram3_slave_rule::R](sec_ctrl_ram3_slave_rule::R) reader structure"]
impl crate::Readable for SEC_CTRL_RAM3_SLAVE_RULE {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ram3_slave_rule::W](sec_ctrl_ram3_slave_rule::W) writer structure"]
impl crate::Writable for SEC_CTRL_RAM3_SLAVE_RULE {}
#[doc = "Security access rules for RAM3 slaves."]
pub mod sec_ctrl_ram3_slave_rule;
#[doc = "Security access rules for RAM3 slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ram3_mem_rule0](sec_ctrl_ram3_mem_rule0) module"]
pub type SEC_CTRL_RAM3_MEM_RULE0 = crate::Reg<u32, _SEC_CTRL_RAM3_MEM_RULE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_RAM3_MEM_RULE0;
#[doc = "`read()` method returns [sec_ctrl_ram3_mem_rule0::R](sec_ctrl_ram3_mem_rule0::R) reader structure"]
impl crate::Readable for SEC_CTRL_RAM3_MEM_RULE0 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ram3_mem_rule0::W](sec_ctrl_ram3_mem_rule0::W) writer structure"]
impl crate::Writable for SEC_CTRL_RAM3_MEM_RULE0 {}
#[doc = "Security access rules for RAM3 slaves."]
pub mod sec_ctrl_ram3_mem_rule0;
#[doc = "Security access rules for RAM3 slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ram3_mem_rule1](sec_ctrl_ram3_mem_rule1) module"]
pub type SEC_CTRL_RAM3_MEM_RULE1 = crate::Reg<u32, _SEC_CTRL_RAM3_MEM_RULE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_RAM3_MEM_RULE1;
#[doc = "`read()` method returns [sec_ctrl_ram3_mem_rule1::R](sec_ctrl_ram3_mem_rule1::R) reader structure"]
impl crate::Readable for SEC_CTRL_RAM3_MEM_RULE1 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ram3_mem_rule1::W](sec_ctrl_ram3_mem_rule1::W) writer structure"]
impl crate::Writable for SEC_CTRL_RAM3_MEM_RULE1 {}
#[doc = "Security access rules for RAM3 slaves."]
pub mod sec_ctrl_ram3_mem_rule1;
#[doc = "Security access rules for RAM4 slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ram4_slave_rule](sec_ctrl_ram4_slave_rule) module"]
pub type SEC_CTRL_RAM4_SLAVE_RULE = crate::Reg<u32, _SEC_CTRL_RAM4_SLAVE_RULE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_RAM4_SLAVE_RULE;
#[doc = "`read()` method returns [sec_ctrl_ram4_slave_rule::R](sec_ctrl_ram4_slave_rule::R) reader structure"]
impl crate::Readable for SEC_CTRL_RAM4_SLAVE_RULE {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ram4_slave_rule::W](sec_ctrl_ram4_slave_rule::W) writer structure"]
impl crate::Writable for SEC_CTRL_RAM4_SLAVE_RULE {}
#[doc = "Security access rules for RAM4 slaves."]
pub mod sec_ctrl_ram4_slave_rule;
#[doc = "Security access rules for RAM4 slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ram4_mem_rule0](sec_ctrl_ram4_mem_rule0) module"]
pub type SEC_CTRL_RAM4_MEM_RULE0 = crate::Reg<u32, _SEC_CTRL_RAM4_MEM_RULE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_RAM4_MEM_RULE0;
#[doc = "`read()` method returns [sec_ctrl_ram4_mem_rule0::R](sec_ctrl_ram4_mem_rule0::R) reader structure"]
impl crate::Readable for SEC_CTRL_RAM4_MEM_RULE0 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ram4_mem_rule0::W](sec_ctrl_ram4_mem_rule0::W) writer structure"]
impl crate::Writable for SEC_CTRL_RAM4_MEM_RULE0 {}
#[doc = "Security access rules for RAM4 slaves."]
pub mod sec_ctrl_ram4_mem_rule0;
#[doc = "Security access rules for both APB Bridges slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_apb_bridge_slave_rule](sec_ctrl_apb_bridge_slave_rule) module"]
pub type SEC_CTRL_APB_BRIDGE_SLAVE_RULE = crate::Reg<u32, _SEC_CTRL_APB_BRIDGE_SLAVE_RULE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_APB_BRIDGE_SLAVE_RULE;
#[doc = "`read()` method returns [sec_ctrl_apb_bridge_slave_rule::R](sec_ctrl_apb_bridge_slave_rule::R) reader structure"]
impl crate::Readable for SEC_CTRL_APB_BRIDGE_SLAVE_RULE {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_apb_bridge_slave_rule::W](sec_ctrl_apb_bridge_slave_rule::W) writer structure"]
impl crate::Writable for SEC_CTRL_APB_BRIDGE_SLAVE_RULE {}
#[doc = "Security access rules for both APB Bridges slaves."]
pub mod sec_ctrl_apb_bridge_slave_rule;
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_apb_bridge0_mem_ctrl0](sec_ctrl_apb_bridge0_mem_ctrl0) module"]
pub type SEC_CTRL_APB_BRIDGE0_MEM_CTRL0 = crate::Reg<u32, _SEC_CTRL_APB_BRIDGE0_MEM_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_APB_BRIDGE0_MEM_CTRL0;
#[doc = "`read()` method returns [sec_ctrl_apb_bridge0_mem_ctrl0::R](sec_ctrl_apb_bridge0_mem_ctrl0::R) reader structure"]
impl crate::Readable for SEC_CTRL_APB_BRIDGE0_MEM_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_apb_bridge0_mem_ctrl0::W](sec_ctrl_apb_bridge0_mem_ctrl0::W) writer structure"]
impl crate::Writable for SEC_CTRL_APB_BRIDGE0_MEM_CTRL0 {}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
pub mod sec_ctrl_apb_bridge0_mem_ctrl0;
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_apb_bridge0_mem_ctrl1](sec_ctrl_apb_bridge0_mem_ctrl1) module"]
pub type SEC_CTRL_APB_BRIDGE0_MEM_CTRL1 = crate::Reg<u32, _SEC_CTRL_APB_BRIDGE0_MEM_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_APB_BRIDGE0_MEM_CTRL1;
#[doc = "`read()` method returns [sec_ctrl_apb_bridge0_mem_ctrl1::R](sec_ctrl_apb_bridge0_mem_ctrl1::R) reader structure"]
impl crate::Readable for SEC_CTRL_APB_BRIDGE0_MEM_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_apb_bridge0_mem_ctrl1::W](sec_ctrl_apb_bridge0_mem_ctrl1::W) writer structure"]
impl crate::Writable for SEC_CTRL_APB_BRIDGE0_MEM_CTRL1 {}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
pub mod sec_ctrl_apb_bridge0_mem_ctrl1;
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_apb_bridge0_mem_ctrl2](sec_ctrl_apb_bridge0_mem_ctrl2) module"]
pub type SEC_CTRL_APB_BRIDGE0_MEM_CTRL2 = crate::Reg<u32, _SEC_CTRL_APB_BRIDGE0_MEM_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_APB_BRIDGE0_MEM_CTRL2;
#[doc = "`read()` method returns [sec_ctrl_apb_bridge0_mem_ctrl2::R](sec_ctrl_apb_bridge0_mem_ctrl2::R) reader structure"]
impl crate::Readable for SEC_CTRL_APB_BRIDGE0_MEM_CTRL2 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_apb_bridge0_mem_ctrl2::W](sec_ctrl_apb_bridge0_mem_ctrl2::W) writer structure"]
impl crate::Writable for SEC_CTRL_APB_BRIDGE0_MEM_CTRL2 {}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
pub mod sec_ctrl_apb_bridge0_mem_ctrl2;
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_apb_bridge1_mem_ctrl0](sec_ctrl_apb_bridge1_mem_ctrl0) module"]
pub type SEC_CTRL_APB_BRIDGE1_MEM_CTRL0 = crate::Reg<u32, _SEC_CTRL_APB_BRIDGE1_MEM_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_APB_BRIDGE1_MEM_CTRL0;
#[doc = "`read()` method returns [sec_ctrl_apb_bridge1_mem_ctrl0::R](sec_ctrl_apb_bridge1_mem_ctrl0::R) reader structure"]
impl crate::Readable for SEC_CTRL_APB_BRIDGE1_MEM_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_apb_bridge1_mem_ctrl0::W](sec_ctrl_apb_bridge1_mem_ctrl0::W) writer structure"]
impl crate::Writable for SEC_CTRL_APB_BRIDGE1_MEM_CTRL0 {}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
pub mod sec_ctrl_apb_bridge1_mem_ctrl0;
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_apb_bridge1_mem_ctrl1](sec_ctrl_apb_bridge1_mem_ctrl1) module"]
pub type SEC_CTRL_APB_BRIDGE1_MEM_CTRL1 = crate::Reg<u32, _SEC_CTRL_APB_BRIDGE1_MEM_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_APB_BRIDGE1_MEM_CTRL1;
#[doc = "`read()` method returns [sec_ctrl_apb_bridge1_mem_ctrl1::R](sec_ctrl_apb_bridge1_mem_ctrl1::R) reader structure"]
impl crate::Readable for SEC_CTRL_APB_BRIDGE1_MEM_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_apb_bridge1_mem_ctrl1::W](sec_ctrl_apb_bridge1_mem_ctrl1::W) writer structure"]
impl crate::Writable for SEC_CTRL_APB_BRIDGE1_MEM_CTRL1 {}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
pub mod sec_ctrl_apb_bridge1_mem_ctrl1;
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_apb_bridge1_mem_ctrl2](sec_ctrl_apb_bridge1_mem_ctrl2) module"]
pub type SEC_CTRL_APB_BRIDGE1_MEM_CTRL2 = crate::Reg<u32, _SEC_CTRL_APB_BRIDGE1_MEM_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_APB_BRIDGE1_MEM_CTRL2;
#[doc = "`read()` method returns [sec_ctrl_apb_bridge1_mem_ctrl2::R](sec_ctrl_apb_bridge1_mem_ctrl2::R) reader structure"]
impl crate::Readable for SEC_CTRL_APB_BRIDGE1_MEM_CTRL2 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_apb_bridge1_mem_ctrl2::W](sec_ctrl_apb_bridge1_mem_ctrl2::W) writer structure"]
impl crate::Writable for SEC_CTRL_APB_BRIDGE1_MEM_CTRL2 {}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
pub mod sec_ctrl_apb_bridge1_mem_ctrl2;
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_apb_bridge1_mem_ctrl3](sec_ctrl_apb_bridge1_mem_ctrl3) module"]
pub type SEC_CTRL_APB_BRIDGE1_MEM_CTRL3 = crate::Reg<u32, _SEC_CTRL_APB_BRIDGE1_MEM_CTRL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_APB_BRIDGE1_MEM_CTRL3;
#[doc = "`read()` method returns [sec_ctrl_apb_bridge1_mem_ctrl3::R](sec_ctrl_apb_bridge1_mem_ctrl3::R) reader structure"]
impl crate::Readable for SEC_CTRL_APB_BRIDGE1_MEM_CTRL3 {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_apb_bridge1_mem_ctrl3::W](sec_ctrl_apb_bridge1_mem_ctrl3::W) writer structure"]
impl crate::Writable for SEC_CTRL_APB_BRIDGE1_MEM_CTRL3 {}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
pub mod sec_ctrl_apb_bridge1_mem_ctrl3;
#[doc = "Security access rules for AHB peripherals.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ahb_port8_slave0_rule](sec_ctrl_ahb_port8_slave0_rule) module"]
pub type SEC_CTRL_AHB_PORT8_SLAVE0_RULE = crate::Reg<u32, _SEC_CTRL_AHB_PORT8_SLAVE0_RULE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_AHB_PORT8_SLAVE0_RULE;
#[doc = "`read()` method returns [sec_ctrl_ahb_port8_slave0_rule::R](sec_ctrl_ahb_port8_slave0_rule::R) reader structure"]
impl crate::Readable for SEC_CTRL_AHB_PORT8_SLAVE0_RULE {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ahb_port8_slave0_rule::W](sec_ctrl_ahb_port8_slave0_rule::W) writer structure"]
impl crate::Writable for SEC_CTRL_AHB_PORT8_SLAVE0_RULE {}
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb_port8_slave0_rule;
#[doc = "Security access rules for AHB peripherals.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ahb_port8_slave1_rule](sec_ctrl_ahb_port8_slave1_rule) module"]
pub type SEC_CTRL_AHB_PORT8_SLAVE1_RULE = crate::Reg<u32, _SEC_CTRL_AHB_PORT8_SLAVE1_RULE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_AHB_PORT8_SLAVE1_RULE;
#[doc = "`read()` method returns [sec_ctrl_ahb_port8_slave1_rule::R](sec_ctrl_ahb_port8_slave1_rule::R) reader structure"]
impl crate::Readable for SEC_CTRL_AHB_PORT8_SLAVE1_RULE {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ahb_port8_slave1_rule::W](sec_ctrl_ahb_port8_slave1_rule::W) writer structure"]
impl crate::Writable for SEC_CTRL_AHB_PORT8_SLAVE1_RULE {}
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb_port8_slave1_rule;
#[doc = "Security access rules for AHB peripherals.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ahb_port9_slave0_rule](sec_ctrl_ahb_port9_slave0_rule) module"]
pub type SEC_CTRL_AHB_PORT9_SLAVE0_RULE = crate::Reg<u32, _SEC_CTRL_AHB_PORT9_SLAVE0_RULE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_AHB_PORT9_SLAVE0_RULE;
#[doc = "`read()` method returns [sec_ctrl_ahb_port9_slave0_rule::R](sec_ctrl_ahb_port9_slave0_rule::R) reader structure"]
impl crate::Readable for SEC_CTRL_AHB_PORT9_SLAVE0_RULE {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ahb_port9_slave0_rule::W](sec_ctrl_ahb_port9_slave0_rule::W) writer structure"]
impl crate::Writable for SEC_CTRL_AHB_PORT9_SLAVE0_RULE {}
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb_port9_slave0_rule;
#[doc = "Security access rules for AHB peripherals.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ahb_port9_slave1_rule](sec_ctrl_ahb_port9_slave1_rule) module"]
pub type SEC_CTRL_AHB_PORT9_SLAVE1_RULE = crate::Reg<u32, _SEC_CTRL_AHB_PORT9_SLAVE1_RULE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_AHB_PORT9_SLAVE1_RULE;
#[doc = "`read()` method returns [sec_ctrl_ahb_port9_slave1_rule::R](sec_ctrl_ahb_port9_slave1_rule::R) reader structure"]
impl crate::Readable for SEC_CTRL_AHB_PORT9_SLAVE1_RULE {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ahb_port9_slave1_rule::W](sec_ctrl_ahb_port9_slave1_rule::W) writer structure"]
impl crate::Writable for SEC_CTRL_AHB_PORT9_SLAVE1_RULE {}
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb_port9_slave1_rule;
#[doc = "Security access rules for AHB peripherals.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ahb_port10_slave0_rule](sec_ctrl_ahb_port10_slave0_rule) module"]
pub type SEC_CTRL_AHB_PORT10_SLAVE0_RULE = crate::Reg<u32, _SEC_CTRL_AHB_PORT10_SLAVE0_RULE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_AHB_PORT10_SLAVE0_RULE;
#[doc = "`read()` method returns [sec_ctrl_ahb_port10_slave0_rule::R](sec_ctrl_ahb_port10_slave0_rule::R) reader structure"]
impl crate::Readable for SEC_CTRL_AHB_PORT10_SLAVE0_RULE {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ahb_port10_slave0_rule::W](sec_ctrl_ahb_port10_slave0_rule::W) writer structure"]
impl crate::Writable for SEC_CTRL_AHB_PORT10_SLAVE0_RULE {}
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb_port10_slave0_rule;
#[doc = "Security access rules for AHB peripherals.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ahb_port10_slave1_rule](sec_ctrl_ahb_port10_slave1_rule) module"]
pub type SEC_CTRL_AHB_PORT10_SLAVE1_RULE = crate::Reg<u32, _SEC_CTRL_AHB_PORT10_SLAVE1_RULE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_AHB_PORT10_SLAVE1_RULE;
#[doc = "`read()` method returns [sec_ctrl_ahb_port10_slave1_rule::R](sec_ctrl_ahb_port10_slave1_rule::R) reader structure"]
impl crate::Readable for SEC_CTRL_AHB_PORT10_SLAVE1_RULE {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ahb_port10_slave1_rule::W](sec_ctrl_ahb_port10_slave1_rule::W) writer structure"]
impl crate::Writable for SEC_CTRL_AHB_PORT10_SLAVE1_RULE {}
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb_port10_slave1_rule;
#[doc = "Security access rules for AHB_SEC_CTRL_AHB.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ahb_sec_ctrl_mem_rule](sec_ctrl_ahb_sec_ctrl_mem_rule) module"]
pub type SEC_CTRL_AHB_SEC_CTRL_MEM_RULE = crate::Reg<u32, _SEC_CTRL_AHB_SEC_CTRL_MEM_RULE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_AHB_SEC_CTRL_MEM_RULE;
#[doc = "`read()` method returns [sec_ctrl_ahb_sec_ctrl_mem_rule::R](sec_ctrl_ahb_sec_ctrl_mem_rule::R) reader structure"]
impl crate::Readable for SEC_CTRL_AHB_SEC_CTRL_MEM_RULE {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ahb_sec_ctrl_mem_rule::W](sec_ctrl_ahb_sec_ctrl_mem_rule::W) writer structure"]
impl crate::Writable for SEC_CTRL_AHB_SEC_CTRL_MEM_RULE {}
#[doc = "Security access rules for AHB_SEC_CTRL_AHB."]
pub mod sec_ctrl_ahb_sec_ctrl_mem_rule;
#[doc = "Security access rules for USB High speed RAM slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_usb_hs_slave_rule](sec_ctrl_usb_hs_slave_rule) module"]
pub type SEC_CTRL_USB_HS_SLAVE_RULE = crate::Reg<u32, _SEC_CTRL_USB_HS_SLAVE_RULE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_USB_HS_SLAVE_RULE;
#[doc = "`read()` method returns [sec_ctrl_usb_hs_slave_rule::R](sec_ctrl_usb_hs_slave_rule::R) reader structure"]
impl crate::Readable for SEC_CTRL_USB_HS_SLAVE_RULE {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_usb_hs_slave_rule::W](sec_ctrl_usb_hs_slave_rule::W) writer structure"]
impl crate::Writable for SEC_CTRL_USB_HS_SLAVE_RULE {}
#[doc = "Security access rules for USB High speed RAM slaves."]
pub mod sec_ctrl_usb_hs_slave_rule;
#[doc = "Security access rules for RAM_USB_HS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_usb_hs_mem_rule](sec_ctrl_usb_hs_mem_rule) module"]
pub type SEC_CTRL_USB_HS_MEM_RULE = crate::Reg<u32, _SEC_CTRL_USB_HS_MEM_RULE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CTRL_USB_HS_MEM_RULE;
#[doc = "`read()` method returns [sec_ctrl_usb_hs_mem_rule::R](sec_ctrl_usb_hs_mem_rule::R) reader structure"]
impl crate::Readable for SEC_CTRL_USB_HS_MEM_RULE {}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_usb_hs_mem_rule::W](sec_ctrl_usb_hs_mem_rule::W) writer structure"]
impl crate::Writable for SEC_CTRL_USB_HS_MEM_RULE {}
#[doc = "Security access rules for RAM_USB_HS."]
pub mod sec_ctrl_usb_hs_mem_rule;
#[doc = "most recent security violation address for AHB port n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_vio_addr](sec_vio_addr) module"]
pub type SEC_VIO_ADDR = crate::Reg<u32, _SEC_VIO_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_VIO_ADDR;
#[doc = "`read()` method returns [sec_vio_addr::R](sec_vio_addr::R) reader structure"]
impl crate::Readable for SEC_VIO_ADDR {}
#[doc = "most recent security violation address for AHB port n"]
pub mod sec_vio_addr;
#[doc = "most recent security violation miscellaneous information for AHB port n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_vio_misc_info](sec_vio_misc_info) module"]
pub type SEC_VIO_MISC_INFO = crate::Reg<u32, _SEC_VIO_MISC_INFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_VIO_MISC_INFO;
#[doc = "`read()` method returns [sec_vio_misc_info::R](sec_vio_misc_info::R) reader structure"]
impl crate::Readable for SEC_VIO_MISC_INFO {}
#[doc = "most recent security violation miscellaneous information for AHB port n"]
pub mod sec_vio_misc_info;
#[doc = "security violation address/information registers valid flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_vio_info_valid](sec_vio_info_valid) module"]
pub type SEC_VIO_INFO_VALID = crate::Reg<u32, _SEC_VIO_INFO_VALID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_VIO_INFO_VALID;
#[doc = "`read()` method returns [sec_vio_info_valid::R](sec_vio_info_valid::R) reader structure"]
impl crate::Readable for SEC_VIO_INFO_VALID {}
#[doc = "`write(|w| ..)` method takes [sec_vio_info_valid::W](sec_vio_info_valid::W) writer structure"]
impl crate::Writable for SEC_VIO_INFO_VALID {}
#[doc = "security violation address/information registers valid flags"]
pub mod sec_vio_info_valid;
#[doc = "Secure GPIO mask for port 0 pins.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_gpio_mask0](sec_gpio_mask0) module"]
pub type SEC_GPIO_MASK0 = crate::Reg<u32, _SEC_GPIO_MASK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_GPIO_MASK0;
#[doc = "`read()` method returns [sec_gpio_mask0::R](sec_gpio_mask0::R) reader structure"]
impl crate::Readable for SEC_GPIO_MASK0 {}
#[doc = "`write(|w| ..)` method takes [sec_gpio_mask0::W](sec_gpio_mask0::W) writer structure"]
impl crate::Writable for SEC_GPIO_MASK0 {}
#[doc = "Secure GPIO mask for port 0 pins."]
pub mod sec_gpio_mask0;
#[doc = "Secure GPIO mask for port 1 pins.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_gpio_mask1](sec_gpio_mask1) module"]
pub type SEC_GPIO_MASK1 = crate::Reg<u32, _SEC_GPIO_MASK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_GPIO_MASK1;
#[doc = "`read()` method returns [sec_gpio_mask1::R](sec_gpio_mask1::R) reader structure"]
impl crate::Readable for SEC_GPIO_MASK1 {}
#[doc = "`write(|w| ..)` method takes [sec_gpio_mask1::W](sec_gpio_mask1::W) writer structure"]
impl crate::Writable for SEC_GPIO_MASK1 {}
#[doc = "Secure GPIO mask for port 1 pins."]
pub mod sec_gpio_mask1;
#[doc = "Secure Interrupt mask for CPU1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_cpu_int_mask0](sec_cpu_int_mask0) module"]
pub type SEC_CPU_INT_MASK0 = crate::Reg<u32, _SEC_CPU_INT_MASK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CPU_INT_MASK0;
#[doc = "`read()` method returns [sec_cpu_int_mask0::R](sec_cpu_int_mask0::R) reader structure"]
impl crate::Readable for SEC_CPU_INT_MASK0 {}
#[doc = "`write(|w| ..)` method takes [sec_cpu_int_mask0::W](sec_cpu_int_mask0::W) writer structure"]
impl crate::Writable for SEC_CPU_INT_MASK0 {}
#[doc = "Secure Interrupt mask for CPU1"]
pub mod sec_cpu_int_mask0;
#[doc = "Secure Interrupt mask for CPU1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_cpu_int_mask1](sec_cpu_int_mask1) module"]
pub type SEC_CPU_INT_MASK1 = crate::Reg<u32, _SEC_CPU_INT_MASK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CPU_INT_MASK1;
#[doc = "`read()` method returns [sec_cpu_int_mask1::R](sec_cpu_int_mask1::R) reader structure"]
impl crate::Readable for SEC_CPU_INT_MASK1 {}
#[doc = "`write(|w| ..)` method takes [sec_cpu_int_mask1::W](sec_cpu_int_mask1::W) writer structure"]
impl crate::Writable for SEC_CPU_INT_MASK1 {}
#[doc = "Secure Interrupt mask for CPU1"]
pub mod sec_cpu_int_mask1;
#[doc = "Security General Purpose register access control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_mask_lock](sec_mask_lock) module"]
pub type SEC_MASK_LOCK = crate::Reg<u32, _SEC_MASK_LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_MASK_LOCK;
#[doc = "`read()` method returns [sec_mask_lock::R](sec_mask_lock::R) reader structure"]
impl crate::Readable for SEC_MASK_LOCK {}
#[doc = "`write(|w| ..)` method takes [sec_mask_lock::W](sec_mask_lock::W) writer structure"]
impl crate::Writable for SEC_MASK_LOCK {}
#[doc = "Security General Purpose register access control."]
pub mod sec_mask_lock;
#[doc = "master secure level register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [master_sec_level](master_sec_level) module"]
pub type MASTER_SEC_LEVEL = crate::Reg<u32, _MASTER_SEC_LEVEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASTER_SEC_LEVEL;
#[doc = "`read()` method returns [master_sec_level::R](master_sec_level::R) reader structure"]
impl crate::Readable for MASTER_SEC_LEVEL {}
#[doc = "`write(|w| ..)` method takes [master_sec_level::W](master_sec_level::W) writer structure"]
impl crate::Writable for MASTER_SEC_LEVEL {}
#[doc = "master secure level register"]
pub mod master_sec_level;
#[doc = "master secure level anti-pole register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [master_sec_anti_pol_reg](master_sec_anti_pol_reg) module"]
pub type MASTER_SEC_ANTI_POL_REG = crate::Reg<u32, _MASTER_SEC_ANTI_POL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASTER_SEC_ANTI_POL_REG;
#[doc = "`read()` method returns [master_sec_anti_pol_reg::R](master_sec_anti_pol_reg::R) reader structure"]
impl crate::Readable for MASTER_SEC_ANTI_POL_REG {}
#[doc = "`write(|w| ..)` method takes [master_sec_anti_pol_reg::W](master_sec_anti_pol_reg::W) writer structure"]
impl crate::Writable for MASTER_SEC_ANTI_POL_REG {}
#[doc = "master secure level anti-pole register"]
pub mod master_sec_anti_pol_reg;
#[doc = "Miscalleneous control signals for in Cortex M33 (CPU0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu0_lock_reg](cpu0_lock_reg) module"]
pub type CPU0_LOCK_REG = crate::Reg<u32, _CPU0_LOCK_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU0_LOCK_REG;
#[doc = "`read()` method returns [cpu0_lock_reg::R](cpu0_lock_reg::R) reader structure"]
impl crate::Readable for CPU0_LOCK_REG {}
#[doc = "`write(|w| ..)` method takes [cpu0_lock_reg::W](cpu0_lock_reg::W) writer structure"]
impl crate::Writable for CPU0_LOCK_REG {}
#[doc = "Miscalleneous control signals for in Cortex M33 (CPU0)"]
pub mod cpu0_lock_reg;
#[doc = "Miscalleneous control signals for in micro-Cortex M33 (CPU1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu1_lock_reg](cpu1_lock_reg) module"]
pub type CPU1_LOCK_REG = crate::Reg<u32, _CPU1_LOCK_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU1_LOCK_REG;
#[doc = "`read()` method returns [cpu1_lock_reg::R](cpu1_lock_reg::R) reader structure"]
impl crate::Readable for CPU1_LOCK_REG {}
#[doc = "`write(|w| ..)` method takes [cpu1_lock_reg::W](cpu1_lock_reg::W) writer structure"]
impl crate::Writable for CPU1_LOCK_REG {}
#[doc = "Miscalleneous control signals for in micro-Cortex M33 (CPU1)"]
pub mod cpu1_lock_reg;
#[doc = "secure control duplicate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_ctrl_dp_reg](misc_ctrl_dp_reg) module"]
pub type MISC_CTRL_DP_REG = crate::Reg<u32, _MISC_CTRL_DP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC_CTRL_DP_REG;
#[doc = "`read()` method returns [misc_ctrl_dp_reg::R](misc_ctrl_dp_reg::R) reader structure"]
impl crate::Readable for MISC_CTRL_DP_REG {}
#[doc = "`write(|w| ..)` method takes [misc_ctrl_dp_reg::W](misc_ctrl_dp_reg::W) writer structure"]
impl crate::Writable for MISC_CTRL_DP_REG {}
#[doc = "secure control duplicate register"]
pub mod misc_ctrl_dp_reg;
#[doc = "secure control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_ctrl_reg](misc_ctrl_reg) module"]
pub type MISC_CTRL_REG = crate::Reg<u32, _MISC_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC_CTRL_REG;
#[doc = "`read()` method returns [misc_ctrl_reg::R](misc_ctrl_reg::R) reader structure"]
impl crate::Readable for MISC_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [misc_ctrl_reg::W](misc_ctrl_reg::W) writer structure"]
impl crate::Writable for MISC_CTRL_REG {}
#[doc = "secure control register"]
pub mod misc_ctrl_reg;
