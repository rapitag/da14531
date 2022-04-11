#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub adpll_attr_ctrl_reg: crate::Reg<adpll_attr_ctrl_reg::ADPLL_ATTR_CTRL_REG_SPEC>,
    #[doc = "0x04 - "]
    pub adpll_cn_ctrl_reg: crate::Reg<adpll_cn_ctrl_reg::ADPLL_CN_CTRL_REG_SPEC>,
    #[doc = "0x08 - "]
    pub adpll_fif_ctrl1_reg: crate::Reg<adpll_fif_ctrl1_reg::ADPLL_FIF_CTRL1_REG_SPEC>,
    #[doc = "0x0c - "]
    pub adpll_fif_ctrl2_reg: crate::Reg<adpll_fif_ctrl2_reg::ADPLL_FIF_CTRL2_REG_SPEC>,
    #[doc = "0x10 - "]
    pub adpll_kdco_cal_ctrl1_reg:
        crate::Reg<adpll_kdco_cal_ctrl1_reg::ADPLL_KDCO_CAL_CTRL1_REG_SPEC>,
    #[doc = "0x14 - "]
    pub adpll_kdco_cal_ctrl2_reg:
        crate::Reg<adpll_kdco_cal_ctrl2_reg::ADPLL_KDCO_CAL_CTRL2_REG_SPEC>,
    #[doc = "0x18 - "]
    pub adpll_kdtctdc_cal_ctrl1_reg:
        crate::Reg<adpll_kdtctdc_cal_ctrl1_reg::ADPLL_KDTCTDC_CAL_CTRL1_REG_SPEC>,
    #[doc = "0x1c - "]
    pub adpll_kdtctdc_cal_ctrl2_reg:
        crate::Reg<adpll_kdtctdc_cal_ctrl2_reg::ADPLL_KDTCTDC_CAL_CTRL2_REG_SPEC>,
    #[doc = "0x20 - "]
    pub adpll_dcoamp_cal_ctrl_reg:
        crate::Reg<adpll_dcoamp_cal_ctrl_reg::ADPLL_DCOAMP_CAL_CTRL_REG_SPEC>,
    #[doc = "0x24 - "]
    pub adpll_txmod_ctrl_reg: crate::Reg<adpll_txmod_ctrl_reg::ADPLL_TXMOD_CTRL_REG_SPEC>,
    #[doc = "0x28 - "]
    pub adpll_sdmod_ctrl_reg: crate::Reg<adpll_sdmod_ctrl_reg::ADPLL_SDMOD_CTRL_REG_SPEC>,
    #[doc = "0x2c - "]
    pub adpll_lf_ctrl1_reg: crate::Reg<adpll_lf_ctrl1_reg::ADPLL_LF_CTRL1_REG_SPEC>,
    #[doc = "0x30 - "]
    pub adpll_lf_ctrl2_reg: crate::Reg<adpll_lf_ctrl2_reg::ADPLL_LF_CTRL2_REG_SPEC>,
    #[doc = "0x34 - "]
    pub adpll_ana_ctrl_reg: crate::Reg<adpll_ana_ctrl_reg::ADPLL_ANA_CTRL_REG_SPEC>,
    #[doc = "0x38 - "]
    pub adpll_div_ctrl_reg: crate::Reg<adpll_div_ctrl_reg::ADPLL_DIV_CTRL_REG_SPEC>,
    #[doc = "0x3c - "]
    pub adpll_init_ctrl_reg: crate::Reg<adpll_init_ctrl_reg::ADPLL_INIT_CTRL_REG_SPEC>,
    #[doc = "0x40 - "]
    pub adpll_fsm_ctrl_reg: crate::Reg<adpll_fsm_ctrl_reg::ADPLL_FSM_CTRL_REG_SPEC>,
    #[doc = "0x44 - "]
    pub adpll_mon_ctrl_reg: crate::Reg<adpll_mon_ctrl_reg::ADPLL_MON_CTRL_REG_SPEC>,
    #[doc = "0x48 - "]
    pub adpll_misc_ctrl_reg: crate::Reg<adpll_misc_ctrl_reg::ADPLL_MISC_CTRL_REG_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0x50 - "]
    pub adpll_overrule_ctrl1_reg:
        crate::Reg<adpll_overrule_ctrl1_reg::ADPLL_OVERRULE_CTRL1_REG_SPEC>,
    #[doc = "0x54 - "]
    pub adpll_overrule_ctrl2_reg:
        crate::Reg<adpll_overrule_ctrl2_reg::ADPLL_OVERRULE_CTRL2_REG_SPEC>,
    #[doc = "0x58 - "]
    pub adpll_overrule_ctrl3_reg:
        crate::Reg<adpll_overrule_ctrl3_reg::ADPLL_OVERRULE_CTRL3_REG_SPEC>,
    #[doc = "0x5c - "]
    pub adpll_rfpt_ctrl_reg: crate::Reg<adpll_rfpt_ctrl_reg::ADPLL_RFPT_CTRL_REG_SPEC>,
    #[doc = "0x60 - "]
    pub adpll_anatst_ctrl_reg: crate::Reg<adpll_anatst_ctrl_reg::ADPLL_ANATST_CTRL_REG_SPEC>,
    #[doc = "0x64 - "]
    pub adpll_acc_ctrl_reg: crate::Reg<adpll_acc_ctrl_reg::ADPLL_ACC_CTRL_REG_SPEC>,
    _reserved25: [u8; 0x14],
    #[doc = "0x7c - "]
    pub adpll_freqmeas_rd_reg: crate::Reg<adpll_freqmeas_rd_reg::ADPLL_FREQMEAS_RD_REG_SPEC>,
    #[doc = "0x80 - "]
    pub adpll_dco_rd_reg: crate::Reg<adpll_dco_rd_reg::ADPLL_DCO_RD_REG_SPEC>,
    #[doc = "0x84 - "]
    pub adpll_kdco_rd_reg: crate::Reg<adpll_kdco_rd_reg::ADPLL_KDCO_RD_REG_SPEC>,
    #[doc = "0x88 - "]
    pub adpll_kdtc_rd_reg: crate::Reg<adpll_kdtc_rd_reg::ADPLL_KDTC_RD_REG_SPEC>,
    #[doc = "0x8c - "]
    pub adpll_tunestate_rd_reg: crate::Reg<adpll_tunestate_rd_reg::ADPLL_TUNESTATE_RD_REG_SPEC>,
    #[doc = "0x90 - "]
    pub adpll_pllfcwdt_rd_reg: crate::Reg<adpll_pllfcwdt_rd_reg::ADPLL_PLLFCWDT_RD_REG_SPEC>,
    #[doc = "0x94 - "]
    pub adpll_anatst_rd_reg: crate::Reg<adpll_anatst_rd_reg::ADPLL_ANATST_RD_REG_SPEC>,
}
#[doc = "ADPLL_ACC_CTRL_REG register accessor: an alias for `Reg<ADPLL_ACC_CTRL_REG_SPEC>`"]
pub type ADPLL_ACC_CTRL_REG = crate::Reg<adpll_acc_ctrl_reg::ADPLL_ACC_CTRL_REG_SPEC>;
#[doc = ""]
pub mod adpll_acc_ctrl_reg;
#[doc = "ADPLL_ANATST_CTRL_REG register accessor: an alias for `Reg<ADPLL_ANATST_CTRL_REG_SPEC>`"]
pub type ADPLL_ANATST_CTRL_REG = crate::Reg<adpll_anatst_ctrl_reg::ADPLL_ANATST_CTRL_REG_SPEC>;
#[doc = ""]
pub mod adpll_anatst_ctrl_reg;
#[doc = "ADPLL_ANATST_RD_REG register accessor: an alias for `Reg<ADPLL_ANATST_RD_REG_SPEC>`"]
pub type ADPLL_ANATST_RD_REG = crate::Reg<adpll_anatst_rd_reg::ADPLL_ANATST_RD_REG_SPEC>;
#[doc = ""]
pub mod adpll_anatst_rd_reg;
#[doc = "ADPLL_ANA_CTRL_REG register accessor: an alias for `Reg<ADPLL_ANA_CTRL_REG_SPEC>`"]
pub type ADPLL_ANA_CTRL_REG = crate::Reg<adpll_ana_ctrl_reg::ADPLL_ANA_CTRL_REG_SPEC>;
#[doc = ""]
pub mod adpll_ana_ctrl_reg;
#[doc = "ADPLL_ATTR_CTRL_REG register accessor: an alias for `Reg<ADPLL_ATTR_CTRL_REG_SPEC>`"]
pub type ADPLL_ATTR_CTRL_REG = crate::Reg<adpll_attr_ctrl_reg::ADPLL_ATTR_CTRL_REG_SPEC>;
#[doc = ""]
pub mod adpll_attr_ctrl_reg;
#[doc = "ADPLL_CN_CTRL_REG register accessor: an alias for `Reg<ADPLL_CN_CTRL_REG_SPEC>`"]
pub type ADPLL_CN_CTRL_REG = crate::Reg<adpll_cn_ctrl_reg::ADPLL_CN_CTRL_REG_SPEC>;
#[doc = ""]
pub mod adpll_cn_ctrl_reg;
#[doc = "ADPLL_DCOAMP_CAL_CTRL_REG register accessor: an alias for `Reg<ADPLL_DCOAMP_CAL_CTRL_REG_SPEC>`"]
pub type ADPLL_DCOAMP_CAL_CTRL_REG =
    crate::Reg<adpll_dcoamp_cal_ctrl_reg::ADPLL_DCOAMP_CAL_CTRL_REG_SPEC>;
#[doc = ""]
pub mod adpll_dcoamp_cal_ctrl_reg;
#[doc = "ADPLL_DCO_RD_REG register accessor: an alias for `Reg<ADPLL_DCO_RD_REG_SPEC>`"]
pub type ADPLL_DCO_RD_REG = crate::Reg<adpll_dco_rd_reg::ADPLL_DCO_RD_REG_SPEC>;
#[doc = ""]
pub mod adpll_dco_rd_reg;
#[doc = "ADPLL_DIV_CTRL_REG register accessor: an alias for `Reg<ADPLL_DIV_CTRL_REG_SPEC>`"]
pub type ADPLL_DIV_CTRL_REG = crate::Reg<adpll_div_ctrl_reg::ADPLL_DIV_CTRL_REG_SPEC>;
#[doc = ""]
pub mod adpll_div_ctrl_reg;
#[doc = "ADPLL_FIF_CTRL1_REG register accessor: an alias for `Reg<ADPLL_FIF_CTRL1_REG_SPEC>`"]
pub type ADPLL_FIF_CTRL1_REG = crate::Reg<adpll_fif_ctrl1_reg::ADPLL_FIF_CTRL1_REG_SPEC>;
#[doc = ""]
pub mod adpll_fif_ctrl1_reg;
#[doc = "ADPLL_FIF_CTRL2_REG register accessor: an alias for `Reg<ADPLL_FIF_CTRL2_REG_SPEC>`"]
pub type ADPLL_FIF_CTRL2_REG = crate::Reg<adpll_fif_ctrl2_reg::ADPLL_FIF_CTRL2_REG_SPEC>;
#[doc = ""]
pub mod adpll_fif_ctrl2_reg;
#[doc = "ADPLL_FREQMEAS_RD_REG register accessor: an alias for `Reg<ADPLL_FREQMEAS_RD_REG_SPEC>`"]
pub type ADPLL_FREQMEAS_RD_REG = crate::Reg<adpll_freqmeas_rd_reg::ADPLL_FREQMEAS_RD_REG_SPEC>;
#[doc = ""]
pub mod adpll_freqmeas_rd_reg;
#[doc = "ADPLL_FSM_CTRL_REG register accessor: an alias for `Reg<ADPLL_FSM_CTRL_REG_SPEC>`"]
pub type ADPLL_FSM_CTRL_REG = crate::Reg<adpll_fsm_ctrl_reg::ADPLL_FSM_CTRL_REG_SPEC>;
#[doc = ""]
pub mod adpll_fsm_ctrl_reg;
#[doc = "ADPLL_INIT_CTRL_REG register accessor: an alias for `Reg<ADPLL_INIT_CTRL_REG_SPEC>`"]
pub type ADPLL_INIT_CTRL_REG = crate::Reg<adpll_init_ctrl_reg::ADPLL_INIT_CTRL_REG_SPEC>;
#[doc = ""]
pub mod adpll_init_ctrl_reg;
#[doc = "ADPLL_KDCO_CAL_CTRL1_REG register accessor: an alias for `Reg<ADPLL_KDCO_CAL_CTRL1_REG_SPEC>`"]
pub type ADPLL_KDCO_CAL_CTRL1_REG =
    crate::Reg<adpll_kdco_cal_ctrl1_reg::ADPLL_KDCO_CAL_CTRL1_REG_SPEC>;
#[doc = ""]
pub mod adpll_kdco_cal_ctrl1_reg;
#[doc = "ADPLL_KDCO_CAL_CTRL2_REG register accessor: an alias for `Reg<ADPLL_KDCO_CAL_CTRL2_REG_SPEC>`"]
pub type ADPLL_KDCO_CAL_CTRL2_REG =
    crate::Reg<adpll_kdco_cal_ctrl2_reg::ADPLL_KDCO_CAL_CTRL2_REG_SPEC>;
#[doc = ""]
pub mod adpll_kdco_cal_ctrl2_reg;
#[doc = "ADPLL_KDCO_RD_REG register accessor: an alias for `Reg<ADPLL_KDCO_RD_REG_SPEC>`"]
pub type ADPLL_KDCO_RD_REG = crate::Reg<adpll_kdco_rd_reg::ADPLL_KDCO_RD_REG_SPEC>;
#[doc = ""]
pub mod adpll_kdco_rd_reg;
#[doc = "ADPLL_KDTCTDC_CAL_CTRL1_REG register accessor: an alias for `Reg<ADPLL_KDTCTDC_CAL_CTRL1_REG_SPEC>`"]
pub type ADPLL_KDTCTDC_CAL_CTRL1_REG =
    crate::Reg<adpll_kdtctdc_cal_ctrl1_reg::ADPLL_KDTCTDC_CAL_CTRL1_REG_SPEC>;
#[doc = ""]
pub mod adpll_kdtctdc_cal_ctrl1_reg;
#[doc = "ADPLL_KDTCTDC_CAL_CTRL2_REG register accessor: an alias for `Reg<ADPLL_KDTCTDC_CAL_CTRL2_REG_SPEC>`"]
pub type ADPLL_KDTCTDC_CAL_CTRL2_REG =
    crate::Reg<adpll_kdtctdc_cal_ctrl2_reg::ADPLL_KDTCTDC_CAL_CTRL2_REG_SPEC>;
#[doc = ""]
pub mod adpll_kdtctdc_cal_ctrl2_reg;
#[doc = "ADPLL_KDTC_RD_REG register accessor: an alias for `Reg<ADPLL_KDTC_RD_REG_SPEC>`"]
pub type ADPLL_KDTC_RD_REG = crate::Reg<adpll_kdtc_rd_reg::ADPLL_KDTC_RD_REG_SPEC>;
#[doc = ""]
pub mod adpll_kdtc_rd_reg;
#[doc = "ADPLL_LF_CTRL1_REG register accessor: an alias for `Reg<ADPLL_LF_CTRL1_REG_SPEC>`"]
pub type ADPLL_LF_CTRL1_REG = crate::Reg<adpll_lf_ctrl1_reg::ADPLL_LF_CTRL1_REG_SPEC>;
#[doc = ""]
pub mod adpll_lf_ctrl1_reg;
#[doc = "ADPLL_LF_CTRL2_REG register accessor: an alias for `Reg<ADPLL_LF_CTRL2_REG_SPEC>`"]
pub type ADPLL_LF_CTRL2_REG = crate::Reg<adpll_lf_ctrl2_reg::ADPLL_LF_CTRL2_REG_SPEC>;
#[doc = ""]
pub mod adpll_lf_ctrl2_reg;
#[doc = "ADPLL_MISC_CTRL_REG register accessor: an alias for `Reg<ADPLL_MISC_CTRL_REG_SPEC>`"]
pub type ADPLL_MISC_CTRL_REG = crate::Reg<adpll_misc_ctrl_reg::ADPLL_MISC_CTRL_REG_SPEC>;
#[doc = ""]
pub mod adpll_misc_ctrl_reg;
#[doc = "ADPLL_MON_CTRL_REG register accessor: an alias for `Reg<ADPLL_MON_CTRL_REG_SPEC>`"]
pub type ADPLL_MON_CTRL_REG = crate::Reg<adpll_mon_ctrl_reg::ADPLL_MON_CTRL_REG_SPEC>;
#[doc = ""]
pub mod adpll_mon_ctrl_reg;
#[doc = "ADPLL_OVERRULE_CTRL1_REG register accessor: an alias for `Reg<ADPLL_OVERRULE_CTRL1_REG_SPEC>`"]
pub type ADPLL_OVERRULE_CTRL1_REG =
    crate::Reg<adpll_overrule_ctrl1_reg::ADPLL_OVERRULE_CTRL1_REG_SPEC>;
#[doc = ""]
pub mod adpll_overrule_ctrl1_reg;
#[doc = "ADPLL_OVERRULE_CTRL2_REG register accessor: an alias for `Reg<ADPLL_OVERRULE_CTRL2_REG_SPEC>`"]
pub type ADPLL_OVERRULE_CTRL2_REG =
    crate::Reg<adpll_overrule_ctrl2_reg::ADPLL_OVERRULE_CTRL2_REG_SPEC>;
#[doc = ""]
pub mod adpll_overrule_ctrl2_reg;
#[doc = "ADPLL_OVERRULE_CTRL3_REG register accessor: an alias for `Reg<ADPLL_OVERRULE_CTRL3_REG_SPEC>`"]
pub type ADPLL_OVERRULE_CTRL3_REG =
    crate::Reg<adpll_overrule_ctrl3_reg::ADPLL_OVERRULE_CTRL3_REG_SPEC>;
#[doc = ""]
pub mod adpll_overrule_ctrl3_reg;
#[doc = "ADPLL_PLLFCWDT_RD_REG register accessor: an alias for `Reg<ADPLL_PLLFCWDT_RD_REG_SPEC>`"]
pub type ADPLL_PLLFCWDT_RD_REG = crate::Reg<adpll_pllfcwdt_rd_reg::ADPLL_PLLFCWDT_RD_REG_SPEC>;
#[doc = ""]
pub mod adpll_pllfcwdt_rd_reg;
#[doc = "ADPLL_RFPT_CTRL_REG register accessor: an alias for `Reg<ADPLL_RFPT_CTRL_REG_SPEC>`"]
pub type ADPLL_RFPT_CTRL_REG = crate::Reg<adpll_rfpt_ctrl_reg::ADPLL_RFPT_CTRL_REG_SPEC>;
#[doc = ""]
pub mod adpll_rfpt_ctrl_reg;
#[doc = "ADPLL_SDMOD_CTRL_REG register accessor: an alias for `Reg<ADPLL_SDMOD_CTRL_REG_SPEC>`"]
pub type ADPLL_SDMOD_CTRL_REG = crate::Reg<adpll_sdmod_ctrl_reg::ADPLL_SDMOD_CTRL_REG_SPEC>;
#[doc = ""]
pub mod adpll_sdmod_ctrl_reg;
#[doc = "ADPLL_TUNESTATE_RD_REG register accessor: an alias for `Reg<ADPLL_TUNESTATE_RD_REG_SPEC>`"]
pub type ADPLL_TUNESTATE_RD_REG = crate::Reg<adpll_tunestate_rd_reg::ADPLL_TUNESTATE_RD_REG_SPEC>;
#[doc = ""]
pub mod adpll_tunestate_rd_reg;
#[doc = "ADPLL_TXMOD_CTRL_REG register accessor: an alias for `Reg<ADPLL_TXMOD_CTRL_REG_SPEC>`"]
pub type ADPLL_TXMOD_CTRL_REG = crate::Reg<adpll_txmod_ctrl_reg::ADPLL_TXMOD_CTRL_REG_SPEC>;
#[doc = ""]
pub mod adpll_txmod_ctrl_reg;
