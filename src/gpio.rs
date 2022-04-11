#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - P0 Data input/output Register"]
    pub p0_data_reg: crate::Reg<p0_data_reg::P0_DATA_REG_SPEC>,
    #[doc = "0x02 - P0 Set port pins Register"]
    pub p0_set_data_reg: crate::Reg<p0_set_data_reg::P0_SET_DATA_REG_SPEC>,
    #[doc = "0x04 - P0 Reset port pins Register"]
    pub p0_reset_data_reg: crate::Reg<p0_reset_data_reg::P0_RESET_DATA_REG_SPEC>,
    #[doc = "0x06 - P00 Mode Register"]
    pub p00_mode_reg: crate::Reg<p00_mode_reg::P00_MODE_REG_SPEC>,
    #[doc = "0x08 - P01 Mode Register"]
    pub p01_mode_reg: crate::Reg<p01_mode_reg::P01_MODE_REG_SPEC>,
    #[doc = "0x0a - P02 Mode Register"]
    pub p02_mode_reg: crate::Reg<p02_mode_reg::P02_MODE_REG_SPEC>,
    #[doc = "0x0c - P03 Mode Register"]
    pub p03_mode_reg: crate::Reg<p03_mode_reg::P03_MODE_REG_SPEC>,
    #[doc = "0x0e - P04 Mode Register"]
    pub p04_mode_reg: crate::Reg<p04_mode_reg::P04_MODE_REG_SPEC>,
    #[doc = "0x10 - P05 Mode Register"]
    pub p05_mode_reg: crate::Reg<p05_mode_reg::P05_MODE_REG_SPEC>,
    #[doc = "0x12 - P06 Mode Register"]
    pub p06_mode_reg: crate::Reg<p06_mode_reg::P06_MODE_REG_SPEC>,
    #[doc = "0x14 - P07 Mode Register"]
    pub p07_mode_reg: crate::Reg<p07_mode_reg::P07_MODE_REG_SPEC>,
    #[doc = "0x16 - P08 Mode Register"]
    pub p08_mode_reg: crate::Reg<p08_mode_reg::P08_MODE_REG_SPEC>,
    #[doc = "0x18 - P09 Mode Register"]
    pub p09_mode_reg: crate::Reg<p09_mode_reg::P09_MODE_REG_SPEC>,
    #[doc = "0x1a - P010 Mode Register"]
    pub p010_mode_reg: crate::Reg<p010_mode_reg::P010_MODE_REG_SPEC>,
    #[doc = "0x1c - P011 Mode Register"]
    pub p011_mode_reg: crate::Reg<p011_mode_reg::P011_MODE_REG_SPEC>,
    #[doc = "0x1e - Pad driving strength control Register"]
    pub pad_weak_ctrl_reg: crate::Reg<pad_weak_ctrl_reg::PAD_WEAK_CTRL_REG_SPEC>,
    #[doc = "0x20 - "]
    pub scan_observe_reg: crate::Reg<scan_observe_reg::SCAN_OBSERVE_REG_SPEC>,
    _reserved17: [u8; 0x0e],
    #[doc = "0x30 - "]
    pub test_ctrl_reg: crate::Reg<test_ctrl_reg::TEST_CTRL_REG_SPEC>,
    #[doc = "0x32 - "]
    pub test_ctrl2_reg: crate::Reg<test_ctrl2_reg::TEST_CTRL2_REG_SPEC>,
    #[doc = "0x34 - "]
    pub test_ctrl3_reg: crate::Reg<test_ctrl3_reg::TEST_CTRL3_REG_SPEC>,
    #[doc = "0x36 - "]
    pub test_ctrl4_reg: crate::Reg<test_ctrl4_reg::TEST_CTRL4_REG_SPEC>,
    #[doc = "0x38 - "]
    pub xtal32m_testctrl0_reg: crate::Reg<xtal32m_testctrl0_reg::XTAL32M_TESTCTRL0_REG_SPEC>,
    #[doc = "0x3a - "]
    pub xtal32m_testctrl1_reg: crate::Reg<xtal32m_testctrl1_reg::XTAL32M_TESTCTRL1_REG_SPEC>,
    #[doc = "0x3c - "]
    pub bist_ctrl_reg: crate::Reg<bist_ctrl_reg::BIST_CTRL_REG_SPEC>,
    #[doc = "0x3e - "]
    pub rombist_resultl_reg: crate::Reg<rombist_resultl_reg::ROMBIST_RESULTL_REG_SPEC>,
    #[doc = "0x40 - "]
    pub rombist_resulth_reg: crate::Reg<rombist_resulth_reg::ROMBIST_RESULTH_REG_SPEC>,
}
#[doc = "BIST_CTRL_REG register accessor: an alias for `Reg<BIST_CTRL_REG_SPEC>`"]
pub type BIST_CTRL_REG = crate::Reg<bist_ctrl_reg::BIST_CTRL_REG_SPEC>;
#[doc = ""]
pub mod bist_ctrl_reg;
#[doc = "P00_MODE_REG register accessor: an alias for `Reg<P00_MODE_REG_SPEC>`"]
pub type P00_MODE_REG = crate::Reg<p00_mode_reg::P00_MODE_REG_SPEC>;
#[doc = "P00 Mode Register"]
pub mod p00_mode_reg;
#[doc = "P010_MODE_REG register accessor: an alias for `Reg<P010_MODE_REG_SPEC>`"]
pub type P010_MODE_REG = crate::Reg<p010_mode_reg::P010_MODE_REG_SPEC>;
#[doc = "P010 Mode Register"]
pub mod p010_mode_reg;
#[doc = "P011_MODE_REG register accessor: an alias for `Reg<P011_MODE_REG_SPEC>`"]
pub type P011_MODE_REG = crate::Reg<p011_mode_reg::P011_MODE_REG_SPEC>;
#[doc = "P011 Mode Register"]
pub mod p011_mode_reg;
#[doc = "P01_MODE_REG register accessor: an alias for `Reg<P01_MODE_REG_SPEC>`"]
pub type P01_MODE_REG = crate::Reg<p01_mode_reg::P01_MODE_REG_SPEC>;
#[doc = "P01 Mode Register"]
pub mod p01_mode_reg;
#[doc = "P02_MODE_REG register accessor: an alias for `Reg<P02_MODE_REG_SPEC>`"]
pub type P02_MODE_REG = crate::Reg<p02_mode_reg::P02_MODE_REG_SPEC>;
#[doc = "P02 Mode Register"]
pub mod p02_mode_reg;
#[doc = "P03_MODE_REG register accessor: an alias for `Reg<P03_MODE_REG_SPEC>`"]
pub type P03_MODE_REG = crate::Reg<p03_mode_reg::P03_MODE_REG_SPEC>;
#[doc = "P03 Mode Register"]
pub mod p03_mode_reg;
#[doc = "P04_MODE_REG register accessor: an alias for `Reg<P04_MODE_REG_SPEC>`"]
pub type P04_MODE_REG = crate::Reg<p04_mode_reg::P04_MODE_REG_SPEC>;
#[doc = "P04 Mode Register"]
pub mod p04_mode_reg;
#[doc = "P05_MODE_REG register accessor: an alias for `Reg<P05_MODE_REG_SPEC>`"]
pub type P05_MODE_REG = crate::Reg<p05_mode_reg::P05_MODE_REG_SPEC>;
#[doc = "P05 Mode Register"]
pub mod p05_mode_reg;
#[doc = "P06_MODE_REG register accessor: an alias for `Reg<P06_MODE_REG_SPEC>`"]
pub type P06_MODE_REG = crate::Reg<p06_mode_reg::P06_MODE_REG_SPEC>;
#[doc = "P06 Mode Register"]
pub mod p06_mode_reg;
#[doc = "P07_MODE_REG register accessor: an alias for `Reg<P07_MODE_REG_SPEC>`"]
pub type P07_MODE_REG = crate::Reg<p07_mode_reg::P07_MODE_REG_SPEC>;
#[doc = "P07 Mode Register"]
pub mod p07_mode_reg;
#[doc = "P08_MODE_REG register accessor: an alias for `Reg<P08_MODE_REG_SPEC>`"]
pub type P08_MODE_REG = crate::Reg<p08_mode_reg::P08_MODE_REG_SPEC>;
#[doc = "P08 Mode Register"]
pub mod p08_mode_reg;
#[doc = "P09_MODE_REG register accessor: an alias for `Reg<P09_MODE_REG_SPEC>`"]
pub type P09_MODE_REG = crate::Reg<p09_mode_reg::P09_MODE_REG_SPEC>;
#[doc = "P09 Mode Register"]
pub mod p09_mode_reg;
#[doc = "P0_DATA_REG register accessor: an alias for `Reg<P0_DATA_REG_SPEC>`"]
pub type P0_DATA_REG = crate::Reg<p0_data_reg::P0_DATA_REG_SPEC>;
#[doc = "P0 Data input/output Register"]
pub mod p0_data_reg;
#[doc = "P0_RESET_DATA_REG register accessor: an alias for `Reg<P0_RESET_DATA_REG_SPEC>`"]
pub type P0_RESET_DATA_REG = crate::Reg<p0_reset_data_reg::P0_RESET_DATA_REG_SPEC>;
#[doc = "P0 Reset port pins Register"]
pub mod p0_reset_data_reg;
#[doc = "P0_SET_DATA_REG register accessor: an alias for `Reg<P0_SET_DATA_REG_SPEC>`"]
pub type P0_SET_DATA_REG = crate::Reg<p0_set_data_reg::P0_SET_DATA_REG_SPEC>;
#[doc = "P0 Set port pins Register"]
pub mod p0_set_data_reg;
#[doc = "PAD_WEAK_CTRL_REG register accessor: an alias for `Reg<PAD_WEAK_CTRL_REG_SPEC>`"]
pub type PAD_WEAK_CTRL_REG = crate::Reg<pad_weak_ctrl_reg::PAD_WEAK_CTRL_REG_SPEC>;
#[doc = "Pad driving strength control Register"]
pub mod pad_weak_ctrl_reg;
#[doc = "ROMBIST_RESULTH_REG register accessor: an alias for `Reg<ROMBIST_RESULTH_REG_SPEC>`"]
pub type ROMBIST_RESULTH_REG = crate::Reg<rombist_resulth_reg::ROMBIST_RESULTH_REG_SPEC>;
#[doc = ""]
pub mod rombist_resulth_reg;
#[doc = "ROMBIST_RESULTL_REG register accessor: an alias for `Reg<ROMBIST_RESULTL_REG_SPEC>`"]
pub type ROMBIST_RESULTL_REG = crate::Reg<rombist_resultl_reg::ROMBIST_RESULTL_REG_SPEC>;
#[doc = ""]
pub mod rombist_resultl_reg;
#[doc = "SCAN_OBSERVE_REG register accessor: an alias for `Reg<SCAN_OBSERVE_REG_SPEC>`"]
pub type SCAN_OBSERVE_REG = crate::Reg<scan_observe_reg::SCAN_OBSERVE_REG_SPEC>;
#[doc = ""]
pub mod scan_observe_reg;
#[doc = "TEST_CTRL2_REG register accessor: an alias for `Reg<TEST_CTRL2_REG_SPEC>`"]
pub type TEST_CTRL2_REG = crate::Reg<test_ctrl2_reg::TEST_CTRL2_REG_SPEC>;
#[doc = ""]
pub mod test_ctrl2_reg;
#[doc = "TEST_CTRL3_REG register accessor: an alias for `Reg<TEST_CTRL3_REG_SPEC>`"]
pub type TEST_CTRL3_REG = crate::Reg<test_ctrl3_reg::TEST_CTRL3_REG_SPEC>;
#[doc = ""]
pub mod test_ctrl3_reg;
#[doc = "TEST_CTRL4_REG register accessor: an alias for `Reg<TEST_CTRL4_REG_SPEC>`"]
pub type TEST_CTRL4_REG = crate::Reg<test_ctrl4_reg::TEST_CTRL4_REG_SPEC>;
#[doc = ""]
pub mod test_ctrl4_reg;
#[doc = "TEST_CTRL_REG register accessor: an alias for `Reg<TEST_CTRL_REG_SPEC>`"]
pub type TEST_CTRL_REG = crate::Reg<test_ctrl_reg::TEST_CTRL_REG_SPEC>;
#[doc = ""]
pub mod test_ctrl_reg;
#[doc = "XTAL32M_TESTCTRL0_REG register accessor: an alias for `Reg<XTAL32M_TESTCTRL0_REG_SPEC>`"]
pub type XTAL32M_TESTCTRL0_REG = crate::Reg<xtal32m_testctrl0_reg::XTAL32M_TESTCTRL0_REG_SPEC>;
#[doc = ""]
pub mod xtal32m_testctrl0_reg;
#[doc = "XTAL32M_TESTCTRL1_REG register accessor: an alias for `Reg<XTAL32M_TESTCTRL1_REG_SPEC>`"]
pub type XTAL32M_TESTCTRL1_REG = crate::Reg<xtal32m_testctrl1_reg::XTAL32M_TESTCTRL1_REG_SPEC>;
#[doc = ""]
pub mod xtal32m_testctrl1_reg;
