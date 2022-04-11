#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General Purpose ADC Control Register"]
    pub gp_adc_ctrl_reg: crate::Reg<gp_adc_ctrl_reg::GP_ADC_CTRL_REG_SPEC>,
    #[doc = "0x02 - General Purpose ADC Second Control Register"]
    pub gp_adc_ctrl2_reg: crate::Reg<gp_adc_ctrl2_reg::GP_ADC_CTRL2_REG_SPEC>,
    #[doc = "0x04 - General Purpose ADC Third Control Register"]
    pub gp_adc_ctrl3_reg: crate::Reg<gp_adc_ctrl3_reg::GP_ADC_CTRL3_REG_SPEC>,
    _reserved3: [u8; 0x02],
    #[doc = "0x08 - General Purpose ADC Positive Offset Register"]
    pub gp_adc_offp_reg: crate::Reg<gp_adc_offp_reg::GP_ADC_OFFP_REG_SPEC>,
    #[doc = "0x0a - General Purpose ADC Negative Offset Register"]
    pub gp_adc_offn_reg: crate::Reg<gp_adc_offn_reg::GP_ADC_OFFN_REG_SPEC>,
    #[doc = "0x0c - General Purpose ADC Trim Register"]
    pub gp_adc_trim_reg: crate::Reg<gp_adc_trim_reg::GP_ADC_TRIM_REG_SPEC>,
    #[doc = "0x0e - General Purpose ADC Clear Interrupt Register"]
    pub gp_adc_clear_int_reg: crate::Reg<gp_adc_clear_int_reg::GP_ADC_CLEAR_INT_REG_SPEC>,
    #[doc = "0x10 - General Purpose ADC Result Register"]
    pub gp_adc_result_reg: crate::Reg<gp_adc_result_reg::GP_ADC_RESULT_REG_SPEC>,
    _reserved8: [u8; 0x0a],
    #[doc = "0x1c - "]
    pub gp_adc_param_dif_reg: crate::Reg<gp_adc_param_dif_reg::GP_ADC_PARAM_DIF_REG_SPEC>,
    #[doc = "0x1e - "]
    pub gp_adc_param_se_reg: crate::Reg<gp_adc_param_se_reg::GP_ADC_PARAM_SE_REG_SPEC>,
}
#[doc = "GP_ADC_CLEAR_INT_REG register accessor: an alias for `Reg<GP_ADC_CLEAR_INT_REG_SPEC>`"]
pub type GP_ADC_CLEAR_INT_REG = crate::Reg<gp_adc_clear_int_reg::GP_ADC_CLEAR_INT_REG_SPEC>;
#[doc = "General Purpose ADC Clear Interrupt Register"]
pub mod gp_adc_clear_int_reg;
#[doc = "GP_ADC_CTRL2_REG register accessor: an alias for `Reg<GP_ADC_CTRL2_REG_SPEC>`"]
pub type GP_ADC_CTRL2_REG = crate::Reg<gp_adc_ctrl2_reg::GP_ADC_CTRL2_REG_SPEC>;
#[doc = "General Purpose ADC Second Control Register"]
pub mod gp_adc_ctrl2_reg;
#[doc = "GP_ADC_CTRL3_REG register accessor: an alias for `Reg<GP_ADC_CTRL3_REG_SPEC>`"]
pub type GP_ADC_CTRL3_REG = crate::Reg<gp_adc_ctrl3_reg::GP_ADC_CTRL3_REG_SPEC>;
#[doc = "General Purpose ADC Third Control Register"]
pub mod gp_adc_ctrl3_reg;
#[doc = "GP_ADC_CTRL_REG register accessor: an alias for `Reg<GP_ADC_CTRL_REG_SPEC>`"]
pub type GP_ADC_CTRL_REG = crate::Reg<gp_adc_ctrl_reg::GP_ADC_CTRL_REG_SPEC>;
#[doc = "General Purpose ADC Control Register"]
pub mod gp_adc_ctrl_reg;
#[doc = "GP_ADC_OFFN_REG register accessor: an alias for `Reg<GP_ADC_OFFN_REG_SPEC>`"]
pub type GP_ADC_OFFN_REG = crate::Reg<gp_adc_offn_reg::GP_ADC_OFFN_REG_SPEC>;
#[doc = "General Purpose ADC Negative Offset Register"]
pub mod gp_adc_offn_reg;
#[doc = "GP_ADC_OFFP_REG register accessor: an alias for `Reg<GP_ADC_OFFP_REG_SPEC>`"]
pub type GP_ADC_OFFP_REG = crate::Reg<gp_adc_offp_reg::GP_ADC_OFFP_REG_SPEC>;
#[doc = "General Purpose ADC Positive Offset Register"]
pub mod gp_adc_offp_reg;
#[doc = "GP_ADC_PARAM_DIF_REG register accessor: an alias for `Reg<GP_ADC_PARAM_DIF_REG_SPEC>`"]
pub type GP_ADC_PARAM_DIF_REG = crate::Reg<gp_adc_param_dif_reg::GP_ADC_PARAM_DIF_REG_SPEC>;
#[doc = ""]
pub mod gp_adc_param_dif_reg;
#[doc = "GP_ADC_PARAM_SE_REG register accessor: an alias for `Reg<GP_ADC_PARAM_SE_REG_SPEC>`"]
pub type GP_ADC_PARAM_SE_REG = crate::Reg<gp_adc_param_se_reg::GP_ADC_PARAM_SE_REG_SPEC>;
#[doc = ""]
pub mod gp_adc_param_se_reg;
#[doc = "GP_ADC_RESULT_REG register accessor: an alias for `Reg<GP_ADC_RESULT_REG_SPEC>`"]
pub type GP_ADC_RESULT_REG = crate::Reg<gp_adc_result_reg::GP_ADC_RESULT_REG_SPEC>;
#[doc = "General Purpose ADC Result Register"]
pub mod gp_adc_result_reg;
#[doc = "GP_ADC_TRIM_REG register accessor: an alias for `Reg<GP_ADC_TRIM_REG_SPEC>`"]
pub type GP_ADC_TRIM_REG = crate::Reg<gp_adc_trim_reg::GP_ADC_TRIM_REG_SPEC>;
#[doc = "General Purpose ADC Trim Register"]
pub mod gp_adc_trim_reg;
