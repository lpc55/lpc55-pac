#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Security access rules for Flash and ROM slaves."]
    pub sec_ctrl_flash_rom_slave_rule:
        crate::Reg<sec_ctrl_flash_rom_slave_rule::SEC_CTRL_FLASH_ROM_SLAVE_RULE_SPEC>,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
    pub sec_ctrl_flash_mem_rule0:
        crate::Reg<sec_ctrl_flash_mem_rule0::SEC_CTRL_FLASH_MEM_RULE0_SPEC>,
    #[doc = "0x14 - Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
    pub sec_ctrl_flash_mem_rule1:
        crate::Reg<sec_ctrl_flash_mem_rule1::SEC_CTRL_FLASH_MEM_RULE1_SPEC>,
    #[doc = "0x18 - Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
    pub sec_ctrl_flash_mem_rule2:
        crate::Reg<sec_ctrl_flash_mem_rule2::SEC_CTRL_FLASH_MEM_RULE2_SPEC>,
    _reserved4: [u8; 4usize],
    #[doc = "0x20 - Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    pub sec_ctrl_rom_mem_rule0: crate::Reg<sec_ctrl_rom_mem_rule0::SEC_CTRL_ROM_MEM_RULE0_SPEC>,
    #[doc = "0x24 - Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    pub sec_ctrl_rom_mem_rule1: crate::Reg<sec_ctrl_rom_mem_rule1::SEC_CTRL_ROM_MEM_RULE1_SPEC>,
    #[doc = "0x28 - Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    pub sec_ctrl_rom_mem_rule2: crate::Reg<sec_ctrl_rom_mem_rule2::SEC_CTRL_ROM_MEM_RULE2_SPEC>,
    #[doc = "0x2c - Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    pub sec_ctrl_rom_mem_rule3: crate::Reg<sec_ctrl_rom_mem_rule3::SEC_CTRL_ROM_MEM_RULE3_SPEC>,
    #[doc = "0x30 - Security access rules for RAMX slaves."]
    pub sec_ctrl_ramx_slave_rule:
        crate::Reg<sec_ctrl_ramx_slave_rule::SEC_CTRL_RAMX_SLAVE_RULE_SPEC>,
    _reserved9: [u8; 12usize],
    #[doc = "0x40 - Security access rules for RAMX slaves."]
    pub sec_ctrl_ramx_mem_rule0: crate::Reg<sec_ctrl_ramx_mem_rule0::SEC_CTRL_RAMX_MEM_RULE0_SPEC>,
    _reserved10: [u8; 12usize],
    #[doc = "0x50 - Security access rules for RAM0 slaves."]
    pub sec_ctrl_ram0_slave_rule:
        crate::Reg<sec_ctrl_ram0_slave_rule::SEC_CTRL_RAM0_SLAVE_RULE_SPEC>,
    _reserved11: [u8; 12usize],
    #[doc = "0x60 - Security access rules for RAM0 slaves."]
    pub sec_ctrl_ram0_mem_rule0: crate::Reg<sec_ctrl_ram0_mem_rule0::SEC_CTRL_RAM0_MEM_RULE0_SPEC>,
    #[doc = "0x64 - Security access rules for RAM0 slaves."]
    pub sec_ctrl_ram0_mem_rule1: crate::Reg<sec_ctrl_ram0_mem_rule1::SEC_CTRL_RAM0_MEM_RULE1_SPEC>,
    _reserved13: [u8; 8usize],
    #[doc = "0x70 - Security access rules for RAM1 slaves."]
    pub sec_ctrl_ram1_slave_rule:
        crate::Reg<sec_ctrl_ram1_slave_rule::SEC_CTRL_RAM1_SLAVE_RULE_SPEC>,
    _reserved14: [u8; 12usize],
    #[doc = "0x80 - Security access rules for RAM1 slaves."]
    pub sec_ctrl_ram1_mem_rule0: crate::Reg<sec_ctrl_ram1_mem_rule0::SEC_CTRL_RAM1_MEM_RULE0_SPEC>,
    #[doc = "0x84 - Security access rules for RAM1 slaves."]
    pub sec_ctrl_ram1_mem_rule1: crate::Reg<sec_ctrl_ram1_mem_rule1::SEC_CTRL_RAM1_MEM_RULE1_SPEC>,
    _reserved16: [u8; 8usize],
    #[doc = "0x90 - Security access rules for RAM2 slaves."]
    pub sec_ctrl_ram2_slave_rule:
        crate::Reg<sec_ctrl_ram2_slave_rule::SEC_CTRL_RAM2_SLAVE_RULE_SPEC>,
    _reserved17: [u8; 12usize],
    #[doc = "0xa0 - Security access rules for RAM2 slaves."]
    pub sec_ctrl_ram2_mem_rule0: crate::Reg<sec_ctrl_ram2_mem_rule0::SEC_CTRL_RAM2_MEM_RULE0_SPEC>,
    #[doc = "0xa4 - Security access rules for RAM2 slaves."]
    pub sec_ctrl_ram2_mem_rule1: crate::Reg<sec_ctrl_ram2_mem_rule1::SEC_CTRL_RAM2_MEM_RULE1_SPEC>,
    _reserved19: [u8; 8usize],
    #[doc = "0xb0 - Security access rules for RAM3 slaves."]
    pub sec_ctrl_ram3_slave_rule:
        crate::Reg<sec_ctrl_ram3_slave_rule::SEC_CTRL_RAM3_SLAVE_RULE_SPEC>,
    _reserved20: [u8; 12usize],
    #[doc = "0xc0 - Security access rules for RAM3 slaves."]
    pub sec_ctrl_ram3_mem_rule0: crate::Reg<sec_ctrl_ram3_mem_rule0::SEC_CTRL_RAM3_MEM_RULE0_SPEC>,
    #[doc = "0xc4 - Security access rules for RAM3 slaves."]
    pub sec_ctrl_ram3_mem_rule1: crate::Reg<sec_ctrl_ram3_mem_rule1::SEC_CTRL_RAM3_MEM_RULE1_SPEC>,
    _reserved22: [u8; 8usize],
    #[doc = "0xd0 - Security access rules for RAM4 slaves."]
    pub sec_ctrl_ram4_slave_rule:
        crate::Reg<sec_ctrl_ram4_slave_rule::SEC_CTRL_RAM4_SLAVE_RULE_SPEC>,
    _reserved23: [u8; 12usize],
    #[doc = "0xe0 - Security access rules for RAM4 slaves."]
    pub sec_ctrl_ram4_mem_rule0: crate::Reg<sec_ctrl_ram4_mem_rule0::SEC_CTRL_RAM4_MEM_RULE0_SPEC>,
    _reserved24: [u8; 12usize],
    #[doc = "0xf0 - Security access rules for both APB Bridges slaves."]
    pub sec_ctrl_apb_bridge_slave_rule:
        crate::Reg<sec_ctrl_apb_bridge_slave_rule::SEC_CTRL_APB_BRIDGE_SLAVE_RULE_SPEC>,
    _reserved25: [u8; 12usize],
    #[doc = "0x100 - Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
    pub sec_ctrl_apb_bridge0_mem_ctrl0:
        crate::Reg<sec_ctrl_apb_bridge0_mem_ctrl0::SEC_CTRL_APB_BRIDGE0_MEM_CTRL0_SPEC>,
    #[doc = "0x104 - Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
    pub sec_ctrl_apb_bridge0_mem_ctrl1:
        crate::Reg<sec_ctrl_apb_bridge0_mem_ctrl1::SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC>,
    #[doc = "0x108 - Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
    pub sec_ctrl_apb_bridge0_mem_ctrl2:
        crate::Reg<sec_ctrl_apb_bridge0_mem_ctrl2::SEC_CTRL_APB_BRIDGE0_MEM_CTRL2_SPEC>,
    _reserved28: [u8; 4usize],
    #[doc = "0x110 - Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    pub sec_ctrl_apb_bridge1_mem_ctrl0:
        crate::Reg<sec_ctrl_apb_bridge1_mem_ctrl0::SEC_CTRL_APB_BRIDGE1_MEM_CTRL0_SPEC>,
    #[doc = "0x114 - Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    pub sec_ctrl_apb_bridge1_mem_ctrl1:
        crate::Reg<sec_ctrl_apb_bridge1_mem_ctrl1::SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC>,
    #[doc = "0x118 - Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    pub sec_ctrl_apb_bridge1_mem_ctrl2:
        crate::Reg<sec_ctrl_apb_bridge1_mem_ctrl2::SEC_CTRL_APB_BRIDGE1_MEM_CTRL2_SPEC>,
    #[doc = "0x11c - Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    pub sec_ctrl_apb_bridge1_mem_ctrl3:
        crate::Reg<sec_ctrl_apb_bridge1_mem_ctrl3::SEC_CTRL_APB_BRIDGE1_MEM_CTRL3_SPEC>,
    #[doc = "0x120 - Security access rules for AHB peripherals."]
    pub sec_ctrl_ahb_port8_slave0_rule:
        crate::Reg<sec_ctrl_ahb_port8_slave0_rule::SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>,
    #[doc = "0x124 - Security access rules for AHB peripherals."]
    pub sec_ctrl_ahb_port8_slave1_rule:
        crate::Reg<sec_ctrl_ahb_port8_slave1_rule::SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC>,
    _reserved34: [u8; 8usize],
    #[doc = "0x130 - Security access rules for AHB peripherals."]
    pub sec_ctrl_ahb_port9_slave0_rule:
        crate::Reg<sec_ctrl_ahb_port9_slave0_rule::SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC>,
    #[doc = "0x134 - Security access rules for AHB peripherals."]
    pub sec_ctrl_ahb_port9_slave1_rule:
        crate::Reg<sec_ctrl_ahb_port9_slave1_rule::SEC_CTRL_AHB_PORT9_SLAVE1_RULE_SPEC>,
    _reserved36: [u8; 8usize],
    #[doc = "0x140 - Security access rules for AHB peripherals."]
    pub sec_ctrl_ahb_port10_slave0_rule:
        crate::Reg<sec_ctrl_ahb_port10_slave0_rule::SEC_CTRL_AHB_PORT10_SLAVE0_RULE_SPEC>,
    #[doc = "0x144 - Security access rules for AHB peripherals."]
    pub sec_ctrl_ahb_port10_slave1_rule:
        crate::Reg<sec_ctrl_ahb_port10_slave1_rule::SEC_CTRL_AHB_PORT10_SLAVE1_RULE_SPEC>,
    _reserved38: [u8; 8usize],
    #[doc = "0x150 - Security access rules for AHB_SEC_CTRL_AHB."]
    pub sec_ctrl_ahb_sec_ctrl_mem_rule:
        crate::Reg<sec_ctrl_ahb_sec_ctrl_mem_rule::SEC_CTRL_AHB_SEC_CTRL_MEM_RULE_SPEC>,
    _reserved39: [u8; 12usize],
    #[doc = "0x160 - Security access rules for USB High speed RAM slaves."]
    pub sec_ctrl_usb_hs_slave_rule:
        crate::Reg<sec_ctrl_usb_hs_slave_rule::SEC_CTRL_USB_HS_SLAVE_RULE_SPEC>,
    _reserved40: [u8; 12usize],
    #[doc = "0x170 - Security access rules for RAM_USB_HS."]
    pub sec_ctrl_usb_hs_mem_rule:
        crate::Reg<sec_ctrl_usb_hs_mem_rule::SEC_CTRL_USB_HS_MEM_RULE_SPEC>,
    _reserved41: [u8; 3212usize],
    #[doc = "0xe00 - most recent security violation address for AHB port n"]
    pub sec_vio_addr: [crate::Reg<sec_vio_addr::SEC_VIO_ADDR_SPEC>; 12],
    _reserved42: [u8; 80usize],
    #[doc = "0xe80 - most recent security violation miscellaneous information for AHB port n"]
    pub sec_vio_misc_info: [crate::Reg<sec_vio_misc_info::SEC_VIO_MISC_INFO_SPEC>; 12],
    _reserved43: [u8; 80usize],
    #[doc = "0xf00 - security violation address/information registers valid flags"]
    pub sec_vio_info_valid: crate::Reg<sec_vio_info_valid::SEC_VIO_INFO_VALID_SPEC>,
    _reserved44: [u8; 124usize],
    #[doc = "0xf80 - Secure GPIO mask for port 0 pins."]
    pub sec_gpio_mask0: crate::Reg<sec_gpio_mask0::SEC_GPIO_MASK0_SPEC>,
    #[doc = "0xf84 - Secure GPIO mask for port 1 pins."]
    pub sec_gpio_mask1: crate::Reg<sec_gpio_mask1::SEC_GPIO_MASK1_SPEC>,
    _reserved46: [u8; 8usize],
    #[doc = "0xf90 - Secure Interrupt mask for CPU1"]
    pub sec_cpu_int_mask0: crate::Reg<sec_cpu_int_mask0::SEC_CPU_INT_MASK0_SPEC>,
    #[doc = "0xf94 - Secure Interrupt mask for CPU1"]
    pub sec_cpu_int_mask1: crate::Reg<sec_cpu_int_mask1::SEC_CPU_INT_MASK1_SPEC>,
    _reserved48: [u8; 36usize],
    #[doc = "0xfbc - Security General Purpose register access control."]
    pub sec_mask_lock: crate::Reg<sec_mask_lock::SEC_MASK_LOCK_SPEC>,
    _reserved49: [u8; 16usize],
    #[doc = "0xfd0 - master secure level register"]
    pub master_sec_level: crate::Reg<master_sec_level::MASTER_SEC_LEVEL_SPEC>,
    #[doc = "0xfd4 - master secure level anti-pole register"]
    pub master_sec_anti_pol_reg: crate::Reg<master_sec_anti_pol_reg::MASTER_SEC_ANTI_POL_REG_SPEC>,
    _reserved51: [u8; 20usize],
    #[doc = "0xfec - Miscalleneous control signals for in Cortex M33 (CPU0)"]
    pub cpu0_lock_reg: crate::Reg<cpu0_lock_reg::CPU0_LOCK_REG_SPEC>,
    #[doc = "0xff0 - Miscalleneous control signals for in micro-Cortex M33 (CPU1)"]
    pub cpu1_lock_reg: crate::Reg<cpu1_lock_reg::CPU1_LOCK_REG_SPEC>,
    _reserved53: [u8; 4usize],
    #[doc = "0xff8 - secure control duplicate register"]
    pub misc_ctrl_dp_reg: crate::Reg<misc_ctrl_dp_reg::MISC_CTRL_DP_REG_SPEC>,
    #[doc = "0xffc - secure control register"]
    pub misc_ctrl_reg: crate::Reg<misc_ctrl_reg::MISC_CTRL_REG_SPEC>,
}
#[doc = "SEC_CTRL_FLASH_ROM_SLAVE_RULE register accessor: an alias for `Reg<SEC_CTRL_FLASH_ROM_SLAVE_RULE_SPEC>`"]
pub type SEC_CTRL_FLASH_ROM_SLAVE_RULE =
    crate::Reg<sec_ctrl_flash_rom_slave_rule::SEC_CTRL_FLASH_ROM_SLAVE_RULE_SPEC>;
#[doc = "Security access rules for Flash and ROM slaves."]
pub mod sec_ctrl_flash_rom_slave_rule;
#[doc = "SEC_CTRL_FLASH_MEM_RULE0 register accessor: an alias for `Reg<SEC_CTRL_FLASH_MEM_RULE0_SPEC>`"]
pub type SEC_CTRL_FLASH_MEM_RULE0 =
    crate::Reg<sec_ctrl_flash_mem_rule0::SEC_CTRL_FLASH_MEM_RULE0_SPEC>;
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
pub mod sec_ctrl_flash_mem_rule0;
#[doc = "SEC_CTRL_FLASH_MEM_RULE1 register accessor: an alias for `Reg<SEC_CTRL_FLASH_MEM_RULE1_SPEC>`"]
pub type SEC_CTRL_FLASH_MEM_RULE1 =
    crate::Reg<sec_ctrl_flash_mem_rule1::SEC_CTRL_FLASH_MEM_RULE1_SPEC>;
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
pub mod sec_ctrl_flash_mem_rule1;
#[doc = "SEC_CTRL_FLASH_MEM_RULE2 register accessor: an alias for `Reg<SEC_CTRL_FLASH_MEM_RULE2_SPEC>`"]
pub type SEC_CTRL_FLASH_MEM_RULE2 =
    crate::Reg<sec_ctrl_flash_mem_rule2::SEC_CTRL_FLASH_MEM_RULE2_SPEC>;
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
pub mod sec_ctrl_flash_mem_rule2;
#[doc = "SEC_CTRL_ROM_MEM_RULE0 register accessor: an alias for `Reg<SEC_CTRL_ROM_MEM_RULE0_SPEC>`"]
pub type SEC_CTRL_ROM_MEM_RULE0 = crate::Reg<sec_ctrl_rom_mem_rule0::SEC_CTRL_ROM_MEM_RULE0_SPEC>;
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
pub mod sec_ctrl_rom_mem_rule0;
#[doc = "SEC_CTRL_ROM_MEM_RULE1 register accessor: an alias for `Reg<SEC_CTRL_ROM_MEM_RULE1_SPEC>`"]
pub type SEC_CTRL_ROM_MEM_RULE1 = crate::Reg<sec_ctrl_rom_mem_rule1::SEC_CTRL_ROM_MEM_RULE1_SPEC>;
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
pub mod sec_ctrl_rom_mem_rule1;
#[doc = "SEC_CTRL_ROM_MEM_RULE2 register accessor: an alias for `Reg<SEC_CTRL_ROM_MEM_RULE2_SPEC>`"]
pub type SEC_CTRL_ROM_MEM_RULE2 = crate::Reg<sec_ctrl_rom_mem_rule2::SEC_CTRL_ROM_MEM_RULE2_SPEC>;
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
pub mod sec_ctrl_rom_mem_rule2;
#[doc = "SEC_CTRL_ROM_MEM_RULE3 register accessor: an alias for `Reg<SEC_CTRL_ROM_MEM_RULE3_SPEC>`"]
pub type SEC_CTRL_ROM_MEM_RULE3 = crate::Reg<sec_ctrl_rom_mem_rule3::SEC_CTRL_ROM_MEM_RULE3_SPEC>;
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
pub mod sec_ctrl_rom_mem_rule3;
#[doc = "SEC_CTRL_RAMX_SLAVE_RULE register accessor: an alias for `Reg<SEC_CTRL_RAMX_SLAVE_RULE_SPEC>`"]
pub type SEC_CTRL_RAMX_SLAVE_RULE =
    crate::Reg<sec_ctrl_ramx_slave_rule::SEC_CTRL_RAMX_SLAVE_RULE_SPEC>;
#[doc = "Security access rules for RAMX slaves."]
pub mod sec_ctrl_ramx_slave_rule;
#[doc = "SEC_CTRL_RAMX_MEM_RULE0 register accessor: an alias for `Reg<SEC_CTRL_RAMX_MEM_RULE0_SPEC>`"]
pub type SEC_CTRL_RAMX_MEM_RULE0 =
    crate::Reg<sec_ctrl_ramx_mem_rule0::SEC_CTRL_RAMX_MEM_RULE0_SPEC>;
#[doc = "Security access rules for RAMX slaves."]
pub mod sec_ctrl_ramx_mem_rule0;
#[doc = "SEC_CTRL_RAM0_SLAVE_RULE register accessor: an alias for `Reg<SEC_CTRL_RAM0_SLAVE_RULE_SPEC>`"]
pub type SEC_CTRL_RAM0_SLAVE_RULE =
    crate::Reg<sec_ctrl_ram0_slave_rule::SEC_CTRL_RAM0_SLAVE_RULE_SPEC>;
#[doc = "Security access rules for RAM0 slaves."]
pub mod sec_ctrl_ram0_slave_rule;
#[doc = "SEC_CTRL_RAM0_MEM_RULE0 register accessor: an alias for `Reg<SEC_CTRL_RAM0_MEM_RULE0_SPEC>`"]
pub type SEC_CTRL_RAM0_MEM_RULE0 =
    crate::Reg<sec_ctrl_ram0_mem_rule0::SEC_CTRL_RAM0_MEM_RULE0_SPEC>;
#[doc = "Security access rules for RAM0 slaves."]
pub mod sec_ctrl_ram0_mem_rule0;
#[doc = "SEC_CTRL_RAM0_MEM_RULE1 register accessor: an alias for `Reg<SEC_CTRL_RAM0_MEM_RULE1_SPEC>`"]
pub type SEC_CTRL_RAM0_MEM_RULE1 =
    crate::Reg<sec_ctrl_ram0_mem_rule1::SEC_CTRL_RAM0_MEM_RULE1_SPEC>;
#[doc = "Security access rules for RAM0 slaves."]
pub mod sec_ctrl_ram0_mem_rule1;
#[doc = "SEC_CTRL_RAM1_SLAVE_RULE register accessor: an alias for `Reg<SEC_CTRL_RAM1_SLAVE_RULE_SPEC>`"]
pub type SEC_CTRL_RAM1_SLAVE_RULE =
    crate::Reg<sec_ctrl_ram1_slave_rule::SEC_CTRL_RAM1_SLAVE_RULE_SPEC>;
#[doc = "Security access rules for RAM1 slaves."]
pub mod sec_ctrl_ram1_slave_rule;
#[doc = "SEC_CTRL_RAM1_MEM_RULE0 register accessor: an alias for `Reg<SEC_CTRL_RAM1_MEM_RULE0_SPEC>`"]
pub type SEC_CTRL_RAM1_MEM_RULE0 =
    crate::Reg<sec_ctrl_ram1_mem_rule0::SEC_CTRL_RAM1_MEM_RULE0_SPEC>;
#[doc = "Security access rules for RAM1 slaves."]
pub mod sec_ctrl_ram1_mem_rule0;
#[doc = "SEC_CTRL_RAM1_MEM_RULE1 register accessor: an alias for `Reg<SEC_CTRL_RAM1_MEM_RULE1_SPEC>`"]
pub type SEC_CTRL_RAM1_MEM_RULE1 =
    crate::Reg<sec_ctrl_ram1_mem_rule1::SEC_CTRL_RAM1_MEM_RULE1_SPEC>;
#[doc = "Security access rules for RAM1 slaves."]
pub mod sec_ctrl_ram1_mem_rule1;
#[doc = "SEC_CTRL_RAM2_SLAVE_RULE register accessor: an alias for `Reg<SEC_CTRL_RAM2_SLAVE_RULE_SPEC>`"]
pub type SEC_CTRL_RAM2_SLAVE_RULE =
    crate::Reg<sec_ctrl_ram2_slave_rule::SEC_CTRL_RAM2_SLAVE_RULE_SPEC>;
#[doc = "Security access rules for RAM2 slaves."]
pub mod sec_ctrl_ram2_slave_rule;
#[doc = "SEC_CTRL_RAM2_MEM_RULE0 register accessor: an alias for `Reg<SEC_CTRL_RAM2_MEM_RULE0_SPEC>`"]
pub type SEC_CTRL_RAM2_MEM_RULE0 =
    crate::Reg<sec_ctrl_ram2_mem_rule0::SEC_CTRL_RAM2_MEM_RULE0_SPEC>;
#[doc = "Security access rules for RAM2 slaves."]
pub mod sec_ctrl_ram2_mem_rule0;
#[doc = "SEC_CTRL_RAM2_MEM_RULE1 register accessor: an alias for `Reg<SEC_CTRL_RAM2_MEM_RULE1_SPEC>`"]
pub type SEC_CTRL_RAM2_MEM_RULE1 =
    crate::Reg<sec_ctrl_ram2_mem_rule1::SEC_CTRL_RAM2_MEM_RULE1_SPEC>;
#[doc = "Security access rules for RAM2 slaves."]
pub mod sec_ctrl_ram2_mem_rule1;
#[doc = "SEC_CTRL_RAM3_SLAVE_RULE register accessor: an alias for `Reg<SEC_CTRL_RAM3_SLAVE_RULE_SPEC>`"]
pub type SEC_CTRL_RAM3_SLAVE_RULE =
    crate::Reg<sec_ctrl_ram3_slave_rule::SEC_CTRL_RAM3_SLAVE_RULE_SPEC>;
#[doc = "Security access rules for RAM3 slaves."]
pub mod sec_ctrl_ram3_slave_rule;
#[doc = "SEC_CTRL_RAM3_MEM_RULE0 register accessor: an alias for `Reg<SEC_CTRL_RAM3_MEM_RULE0_SPEC>`"]
pub type SEC_CTRL_RAM3_MEM_RULE0 =
    crate::Reg<sec_ctrl_ram3_mem_rule0::SEC_CTRL_RAM3_MEM_RULE0_SPEC>;
#[doc = "Security access rules for RAM3 slaves."]
pub mod sec_ctrl_ram3_mem_rule0;
#[doc = "SEC_CTRL_RAM3_MEM_RULE1 register accessor: an alias for `Reg<SEC_CTRL_RAM3_MEM_RULE1_SPEC>`"]
pub type SEC_CTRL_RAM3_MEM_RULE1 =
    crate::Reg<sec_ctrl_ram3_mem_rule1::SEC_CTRL_RAM3_MEM_RULE1_SPEC>;
#[doc = "Security access rules for RAM3 slaves."]
pub mod sec_ctrl_ram3_mem_rule1;
#[doc = "SEC_CTRL_RAM4_SLAVE_RULE register accessor: an alias for `Reg<SEC_CTRL_RAM4_SLAVE_RULE_SPEC>`"]
pub type SEC_CTRL_RAM4_SLAVE_RULE =
    crate::Reg<sec_ctrl_ram4_slave_rule::SEC_CTRL_RAM4_SLAVE_RULE_SPEC>;
#[doc = "Security access rules for RAM4 slaves."]
pub mod sec_ctrl_ram4_slave_rule;
#[doc = "SEC_CTRL_RAM4_MEM_RULE0 register accessor: an alias for `Reg<SEC_CTRL_RAM4_MEM_RULE0_SPEC>`"]
pub type SEC_CTRL_RAM4_MEM_RULE0 =
    crate::Reg<sec_ctrl_ram4_mem_rule0::SEC_CTRL_RAM4_MEM_RULE0_SPEC>;
#[doc = "Security access rules for RAM4 slaves."]
pub mod sec_ctrl_ram4_mem_rule0;
#[doc = "SEC_CTRL_APB_BRIDGE_SLAVE_RULE register accessor: an alias for `Reg<SEC_CTRL_APB_BRIDGE_SLAVE_RULE_SPEC>`"]
pub type SEC_CTRL_APB_BRIDGE_SLAVE_RULE =
    crate::Reg<sec_ctrl_apb_bridge_slave_rule::SEC_CTRL_APB_BRIDGE_SLAVE_RULE_SPEC>;
#[doc = "Security access rules for both APB Bridges slaves."]
pub mod sec_ctrl_apb_bridge_slave_rule;
#[doc = "SEC_CTRL_APB_BRIDGE0_MEM_CTRL0 register accessor: an alias for `Reg<SEC_CTRL_APB_BRIDGE0_MEM_CTRL0_SPEC>`"]
pub type SEC_CTRL_APB_BRIDGE0_MEM_CTRL0 =
    crate::Reg<sec_ctrl_apb_bridge0_mem_ctrl0::SEC_CTRL_APB_BRIDGE0_MEM_CTRL0_SPEC>;
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
pub mod sec_ctrl_apb_bridge0_mem_ctrl0;
#[doc = "SEC_CTRL_APB_BRIDGE0_MEM_CTRL1 register accessor: an alias for `Reg<SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC>`"]
pub type SEC_CTRL_APB_BRIDGE0_MEM_CTRL1 =
    crate::Reg<sec_ctrl_apb_bridge0_mem_ctrl1::SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC>;
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
pub mod sec_ctrl_apb_bridge0_mem_ctrl1;
#[doc = "SEC_CTRL_APB_BRIDGE0_MEM_CTRL2 register accessor: an alias for `Reg<SEC_CTRL_APB_BRIDGE0_MEM_CTRL2_SPEC>`"]
pub type SEC_CTRL_APB_BRIDGE0_MEM_CTRL2 =
    crate::Reg<sec_ctrl_apb_bridge0_mem_ctrl2::SEC_CTRL_APB_BRIDGE0_MEM_CTRL2_SPEC>;
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
pub mod sec_ctrl_apb_bridge0_mem_ctrl2;
#[doc = "SEC_CTRL_APB_BRIDGE1_MEM_CTRL0 register accessor: an alias for `Reg<SEC_CTRL_APB_BRIDGE1_MEM_CTRL0_SPEC>`"]
pub type SEC_CTRL_APB_BRIDGE1_MEM_CTRL0 =
    crate::Reg<sec_ctrl_apb_bridge1_mem_ctrl0::SEC_CTRL_APB_BRIDGE1_MEM_CTRL0_SPEC>;
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
pub mod sec_ctrl_apb_bridge1_mem_ctrl0;
#[doc = "SEC_CTRL_APB_BRIDGE1_MEM_CTRL1 register accessor: an alias for `Reg<SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC>`"]
pub type SEC_CTRL_APB_BRIDGE1_MEM_CTRL1 =
    crate::Reg<sec_ctrl_apb_bridge1_mem_ctrl1::SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC>;
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
pub mod sec_ctrl_apb_bridge1_mem_ctrl1;
#[doc = "SEC_CTRL_APB_BRIDGE1_MEM_CTRL2 register accessor: an alias for `Reg<SEC_CTRL_APB_BRIDGE1_MEM_CTRL2_SPEC>`"]
pub type SEC_CTRL_APB_BRIDGE1_MEM_CTRL2 =
    crate::Reg<sec_ctrl_apb_bridge1_mem_ctrl2::SEC_CTRL_APB_BRIDGE1_MEM_CTRL2_SPEC>;
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
pub mod sec_ctrl_apb_bridge1_mem_ctrl2;
#[doc = "SEC_CTRL_APB_BRIDGE1_MEM_CTRL3 register accessor: an alias for `Reg<SEC_CTRL_APB_BRIDGE1_MEM_CTRL3_SPEC>`"]
pub type SEC_CTRL_APB_BRIDGE1_MEM_CTRL3 =
    crate::Reg<sec_ctrl_apb_bridge1_mem_ctrl3::SEC_CTRL_APB_BRIDGE1_MEM_CTRL3_SPEC>;
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
pub mod sec_ctrl_apb_bridge1_mem_ctrl3;
#[doc = "SEC_CTRL_AHB_PORT8_SLAVE0_RULE register accessor: an alias for `Reg<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>`"]
pub type SEC_CTRL_AHB_PORT8_SLAVE0_RULE =
    crate::Reg<sec_ctrl_ahb_port8_slave0_rule::SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>;
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb_port8_slave0_rule;
#[doc = "SEC_CTRL_AHB_PORT8_SLAVE1_RULE register accessor: an alias for `Reg<SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC>`"]
pub type SEC_CTRL_AHB_PORT8_SLAVE1_RULE =
    crate::Reg<sec_ctrl_ahb_port8_slave1_rule::SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC>;
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb_port8_slave1_rule;
#[doc = "SEC_CTRL_AHB_PORT9_SLAVE0_RULE register accessor: an alias for `Reg<SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC>`"]
pub type SEC_CTRL_AHB_PORT9_SLAVE0_RULE =
    crate::Reg<sec_ctrl_ahb_port9_slave0_rule::SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC>;
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb_port9_slave0_rule;
#[doc = "SEC_CTRL_AHB_PORT9_SLAVE1_RULE register accessor: an alias for `Reg<SEC_CTRL_AHB_PORT9_SLAVE1_RULE_SPEC>`"]
pub type SEC_CTRL_AHB_PORT9_SLAVE1_RULE =
    crate::Reg<sec_ctrl_ahb_port9_slave1_rule::SEC_CTRL_AHB_PORT9_SLAVE1_RULE_SPEC>;
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb_port9_slave1_rule;
#[doc = "SEC_CTRL_AHB_PORT10_SLAVE0_RULE register accessor: an alias for `Reg<SEC_CTRL_AHB_PORT10_SLAVE0_RULE_SPEC>`"]
pub type SEC_CTRL_AHB_PORT10_SLAVE0_RULE =
    crate::Reg<sec_ctrl_ahb_port10_slave0_rule::SEC_CTRL_AHB_PORT10_SLAVE0_RULE_SPEC>;
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb_port10_slave0_rule;
#[doc = "SEC_CTRL_AHB_PORT10_SLAVE1_RULE register accessor: an alias for `Reg<SEC_CTRL_AHB_PORT10_SLAVE1_RULE_SPEC>`"]
pub type SEC_CTRL_AHB_PORT10_SLAVE1_RULE =
    crate::Reg<sec_ctrl_ahb_port10_slave1_rule::SEC_CTRL_AHB_PORT10_SLAVE1_RULE_SPEC>;
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb_port10_slave1_rule;
#[doc = "SEC_CTRL_AHB_SEC_CTRL_MEM_RULE register accessor: an alias for `Reg<SEC_CTRL_AHB_SEC_CTRL_MEM_RULE_SPEC>`"]
pub type SEC_CTRL_AHB_SEC_CTRL_MEM_RULE =
    crate::Reg<sec_ctrl_ahb_sec_ctrl_mem_rule::SEC_CTRL_AHB_SEC_CTRL_MEM_RULE_SPEC>;
#[doc = "Security access rules for AHB_SEC_CTRL_AHB."]
pub mod sec_ctrl_ahb_sec_ctrl_mem_rule;
#[doc = "SEC_CTRL_USB_HS_SLAVE_RULE register accessor: an alias for `Reg<SEC_CTRL_USB_HS_SLAVE_RULE_SPEC>`"]
pub type SEC_CTRL_USB_HS_SLAVE_RULE =
    crate::Reg<sec_ctrl_usb_hs_slave_rule::SEC_CTRL_USB_HS_SLAVE_RULE_SPEC>;
#[doc = "Security access rules for USB High speed RAM slaves."]
pub mod sec_ctrl_usb_hs_slave_rule;
#[doc = "SEC_CTRL_USB_HS_MEM_RULE register accessor: an alias for `Reg<SEC_CTRL_USB_HS_MEM_RULE_SPEC>`"]
pub type SEC_CTRL_USB_HS_MEM_RULE =
    crate::Reg<sec_ctrl_usb_hs_mem_rule::SEC_CTRL_USB_HS_MEM_RULE_SPEC>;
#[doc = "Security access rules for RAM_USB_HS."]
pub mod sec_ctrl_usb_hs_mem_rule;
#[doc = "sec_vio_addr register accessor: an alias for `Reg<SEC_VIO_ADDR_SPEC>`"]
pub type SEC_VIO_ADDR = crate::Reg<sec_vio_addr::SEC_VIO_ADDR_SPEC>;
#[doc = "most recent security violation address for AHB port n"]
pub mod sec_vio_addr;
#[doc = "sec_vio_misc_info register accessor: an alias for `Reg<SEC_VIO_MISC_INFO_SPEC>`"]
pub type SEC_VIO_MISC_INFO = crate::Reg<sec_vio_misc_info::SEC_VIO_MISC_INFO_SPEC>;
#[doc = "most recent security violation miscellaneous information for AHB port n"]
pub mod sec_vio_misc_info;
#[doc = "SEC_VIO_INFO_VALID register accessor: an alias for `Reg<SEC_VIO_INFO_VALID_SPEC>`"]
pub type SEC_VIO_INFO_VALID = crate::Reg<sec_vio_info_valid::SEC_VIO_INFO_VALID_SPEC>;
#[doc = "security violation address/information registers valid flags"]
pub mod sec_vio_info_valid;
#[doc = "SEC_GPIO_MASK0 register accessor: an alias for `Reg<SEC_GPIO_MASK0_SPEC>`"]
pub type SEC_GPIO_MASK0 = crate::Reg<sec_gpio_mask0::SEC_GPIO_MASK0_SPEC>;
#[doc = "Secure GPIO mask for port 0 pins."]
pub mod sec_gpio_mask0;
#[doc = "SEC_GPIO_MASK1 register accessor: an alias for `Reg<SEC_GPIO_MASK1_SPEC>`"]
pub type SEC_GPIO_MASK1 = crate::Reg<sec_gpio_mask1::SEC_GPIO_MASK1_SPEC>;
#[doc = "Secure GPIO mask for port 1 pins."]
pub mod sec_gpio_mask1;
#[doc = "SEC_CPU_INT_MASK0 register accessor: an alias for `Reg<SEC_CPU_INT_MASK0_SPEC>`"]
pub type SEC_CPU_INT_MASK0 = crate::Reg<sec_cpu_int_mask0::SEC_CPU_INT_MASK0_SPEC>;
#[doc = "Secure Interrupt mask for CPU1"]
pub mod sec_cpu_int_mask0;
#[doc = "SEC_CPU_INT_MASK1 register accessor: an alias for `Reg<SEC_CPU_INT_MASK1_SPEC>`"]
pub type SEC_CPU_INT_MASK1 = crate::Reg<sec_cpu_int_mask1::SEC_CPU_INT_MASK1_SPEC>;
#[doc = "Secure Interrupt mask for CPU1"]
pub mod sec_cpu_int_mask1;
#[doc = "SEC_MASK_LOCK register accessor: an alias for `Reg<SEC_MASK_LOCK_SPEC>`"]
pub type SEC_MASK_LOCK = crate::Reg<sec_mask_lock::SEC_MASK_LOCK_SPEC>;
#[doc = "Security General Purpose register access control."]
pub mod sec_mask_lock;
#[doc = "MASTER_SEC_LEVEL register accessor: an alias for `Reg<MASTER_SEC_LEVEL_SPEC>`"]
pub type MASTER_SEC_LEVEL = crate::Reg<master_sec_level::MASTER_SEC_LEVEL_SPEC>;
#[doc = "master secure level register"]
pub mod master_sec_level;
#[doc = "MASTER_SEC_ANTI_POL_REG register accessor: an alias for `Reg<MASTER_SEC_ANTI_POL_REG_SPEC>`"]
pub type MASTER_SEC_ANTI_POL_REG =
    crate::Reg<master_sec_anti_pol_reg::MASTER_SEC_ANTI_POL_REG_SPEC>;
#[doc = "master secure level anti-pole register"]
pub mod master_sec_anti_pol_reg;
#[doc = "CPU0_LOCK_REG register accessor: an alias for `Reg<CPU0_LOCK_REG_SPEC>`"]
pub type CPU0_LOCK_REG = crate::Reg<cpu0_lock_reg::CPU0_LOCK_REG_SPEC>;
#[doc = "Miscalleneous control signals for in Cortex M33 (CPU0)"]
pub mod cpu0_lock_reg;
#[doc = "CPU1_LOCK_REG register accessor: an alias for `Reg<CPU1_LOCK_REG_SPEC>`"]
pub type CPU1_LOCK_REG = crate::Reg<cpu1_lock_reg::CPU1_LOCK_REG_SPEC>;
#[doc = "Miscalleneous control signals for in micro-Cortex M33 (CPU1)"]
pub mod cpu1_lock_reg;
#[doc = "MISC_CTRL_DP_REG register accessor: an alias for `Reg<MISC_CTRL_DP_REG_SPEC>`"]
pub type MISC_CTRL_DP_REG = crate::Reg<misc_ctrl_dp_reg::MISC_CTRL_DP_REG_SPEC>;
#[doc = "secure control duplicate register"]
pub mod misc_ctrl_dp_reg;
#[doc = "MISC_CTRL_REG register accessor: an alias for `Reg<MISC_CTRL_REG_SPEC>`"]
pub type MISC_CTRL_REG = crate::Reg<misc_ctrl_reg::MISC_CTRL_REG_SPEC>;
#[doc = "secure control register"]
pub mod misc_ctrl_reg;
