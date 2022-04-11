#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub rfmon_ctrl_reg: crate::Reg<rfmon_ctrl_reg::RFMON_CTRL_REG_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - "]
    pub rfmon_addr_reg: crate::Reg<rfmon_addr_reg::RFMON_ADDR_REG_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - "]
    pub rfmon_len_reg: crate::Reg<rfmon_len_reg::RFMON_LEN_REG_SPEC>,
    _reserved3: [u8; 0x02],
    #[doc = "0x0c - "]
    pub rfmon_stat_reg: crate::Reg<rfmon_stat_reg::RFMON_STAT_REG_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - "]
    pub rfmon_crv_addr_reg: crate::Reg<rfmon_crv_addr_reg::RFMON_CRV_ADDR_REG_SPEC>,
    _reserved5: [u8; 0x02],
    #[doc = "0x14 - "]
    pub rfmon_crv_len_reg: crate::Reg<rfmon_crv_len_reg::RFMON_CRV_LEN_REG_SPEC>,
}
#[doc = "RFMON_ADDR_REG register accessor: an alias for `Reg<RFMON_ADDR_REG_SPEC>`"]
pub type RFMON_ADDR_REG = crate::Reg<rfmon_addr_reg::RFMON_ADDR_REG_SPEC>;
#[doc = ""]
pub mod rfmon_addr_reg;
#[doc = "RFMON_CRV_ADDR_REG register accessor: an alias for `Reg<RFMON_CRV_ADDR_REG_SPEC>`"]
pub type RFMON_CRV_ADDR_REG = crate::Reg<rfmon_crv_addr_reg::RFMON_CRV_ADDR_REG_SPEC>;
#[doc = ""]
pub mod rfmon_crv_addr_reg;
#[doc = "RFMON_CRV_LEN_REG register accessor: an alias for `Reg<RFMON_CRV_LEN_REG_SPEC>`"]
pub type RFMON_CRV_LEN_REG = crate::Reg<rfmon_crv_len_reg::RFMON_CRV_LEN_REG_SPEC>;
#[doc = ""]
pub mod rfmon_crv_len_reg;
#[doc = "RFMON_CTRL_REG register accessor: an alias for `Reg<RFMON_CTRL_REG_SPEC>`"]
pub type RFMON_CTRL_REG = crate::Reg<rfmon_ctrl_reg::RFMON_CTRL_REG_SPEC>;
#[doc = ""]
pub mod rfmon_ctrl_reg;
#[doc = "RFMON_LEN_REG register accessor: an alias for `Reg<RFMON_LEN_REG_SPEC>`"]
pub type RFMON_LEN_REG = crate::Reg<rfmon_len_reg::RFMON_LEN_REG_SPEC>;
#[doc = ""]
pub mod rfmon_len_reg;
#[doc = "RFMON_STAT_REG register accessor: an alias for `Reg<RFMON_STAT_REG_SPEC>`"]
pub type RFMON_STAT_REG = crate::Reg<rfmon_stat_reg::RFMON_STAT_REG_SPEC>;
#[doc = ""]
pub mod rfmon_stat_reg;
