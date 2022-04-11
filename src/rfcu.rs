#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub rf_attr_reg: crate::Reg<rf_attr_reg::RF_ATTR_REG_SPEC>,
    #[doc = "0x04 - "]
    pub rf_radio_init_reg: crate::Reg<rf_radio_init_reg::RF_RADIO_INIT_REG_SPEC>,
    #[doc = "0x08 - "]
    pub rf_ldo_status_reg: crate::Reg<rf_ldo_status_reg::RF_LDO_STATUS_REG_SPEC>,
    #[doc = "0x0c - "]
    pub rf_adplldig_ctrl_reg: crate::Reg<rf_adplldig_ctrl_reg::RF_ADPLLDIG_CTRL_REG_SPEC>,
    #[doc = "0x10 - "]
    pub rf_agc_ext_lut_reg: crate::Reg<rf_agc_ext_lut_reg::RF_AGC_EXT_LUT_REG_SPEC>,
    #[doc = "0x14 - "]
    pub rf_calstate_reg: crate::Reg<rf_calstate_reg::RF_CALSTATE_REG_SPEC>,
    #[doc = "0x18 - "]
    pub rf_scan_feedback_reg: crate::Reg<rf_scan_feedback_reg::RF_SCAN_FEEDBACK_REG_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - "]
    pub rf_cal_ctrl_reg: crate::Reg<rf_cal_ctrl_reg::RF_CAL_CTRL_REG_SPEC>,
    #[doc = "0x24 - "]
    pub rf_irq_ctrl_reg: crate::Reg<rf_irq_ctrl_reg::RF_IRQ_CTRL_REG_SPEC>,
    #[doc = "0x28 - "]
    pub rf_adci_dc_offset_reg: crate::Reg<rf_adci_dc_offset_reg::RF_ADCI_DC_OFFSET_REG_SPEC>,
    #[doc = "0x2c - "]
    pub rf_adcq_dc_offset_reg: crate::Reg<rf_adcq_dc_offset_reg::RF_ADCQ_DC_OFFSET_REG_SPEC>,
    #[doc = "0x30 - "]
    pub rf_spare_reg: crate::Reg<rf_spare_reg::RF_SPARE_REG_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x3c - "]
    pub rf_iff_ctrl_reg: crate::Reg<rf_iff_ctrl_reg::RF_IFF_CTRL_REG_SPEC>,
    #[doc = "0x40 - "]
    pub rf_adc_ctrl1_reg: crate::Reg<rf_adc_ctrl1_reg::RF_ADC_CTRL1_REG_SPEC>,
    #[doc = "0x44 - "]
    pub rf_adc_ctrl2_reg: crate::Reg<rf_adc_ctrl2_reg::RF_ADC_CTRL2_REG_SPEC>,
    #[doc = "0x48 - "]
    pub rf_adc_ctrl3_reg: crate::Reg<rf_adc_ctrl3_reg::RF_ADC_CTRL3_REG_SPEC>,
    #[doc = "0x4c - "]
    pub rf_pa_ctrl_reg: crate::Reg<rf_pa_ctrl_reg::RF_PA_CTRL_REG_SPEC>,
    _reserved17: [u8; 0x08],
    #[doc = "0x58 - "]
    pub rf_ldo_vref_sel_reg: crate::Reg<rf_ldo_vref_sel_reg::RF_LDO_VREF_SEL_REG_SPEC>,
    _reserved18: [u8; 0x08],
    #[doc = "0x64 - "]
    pub rf_mixer_ctrl1_reg: crate::Reg<rf_mixer_ctrl1_reg::RF_MIXER_CTRL1_REG_SPEC>,
    #[doc = "0x68 - "]
    pub rf_mixer_ctrl2_reg: crate::Reg<rf_mixer_ctrl2_reg::RF_MIXER_CTRL2_REG_SPEC>,
    _reserved20: [u8; 0x08],
    #[doc = "0x74 - "]
    pub rf_io_ctrl_reg: crate::Reg<rf_io_ctrl_reg::RF_IO_CTRL_REG_SPEC>,
    #[doc = "0x78 - "]
    pub rf_lna_ctrl1_reg: crate::Reg<rf_lna_ctrl1_reg::RF_LNA_CTRL1_REG_SPEC>,
    #[doc = "0x7c - "]
    pub rf_lna_ctrl2_reg: crate::Reg<rf_lna_ctrl2_reg::RF_LNA_CTRL2_REG_SPEC>,
    #[doc = "0x80 - "]
    pub rf_lna_ctrl3_reg: crate::Reg<rf_lna_ctrl3_reg::RF_LNA_CTRL3_REG_SPEC>,
    _reserved24: [u8; 0x1c],
    #[doc = "0xa0 - "]
    pub rf_adplldig_rfmon_ctrl_reg:
        crate::Reg<rf_adplldig_rfmon_ctrl_reg::RF_ADPLLDIG_RFMON_CTRL_REG_SPEC>,
    _reserved25: [u8; 0x04],
    #[doc = "0xa8 - "]
    pub rf_rfcu_ctrl_reg: crate::Reg<rf_rfcu_ctrl_reg::RF_RFCU_CTRL_REG_SPEC>,
    #[doc = "0xac - "]
    pub rf_overrule_reg: crate::Reg<rf_overrule_reg::RF_OVERRULE_REG_SPEC>,
    #[doc = "0xb0 - "]
    pub rf_diagirq_ctrl_reg: crate::Reg<rf_diagirq_ctrl_reg::RF_DIAGIRQ_CTRL_REG_SPEC>,
    #[doc = "0xb4 - "]
    pub rf_diagirq_stat_reg: crate::Reg<rf_diagirq_stat_reg::RF_DIAGIRQ_STAT_REG_SPEC>,
    #[doc = "0xb8 - "]
    pub rf_ldo_ctrl_reg: crate::Reg<rf_ldo_ctrl_reg::RF_LDO_CTRL_REG_SPEC>,
}
#[doc = "RF_ADCI_DC_OFFSET_REG register accessor: an alias for `Reg<RF_ADCI_DC_OFFSET_REG_SPEC>`"]
pub type RF_ADCI_DC_OFFSET_REG = crate::Reg<rf_adci_dc_offset_reg::RF_ADCI_DC_OFFSET_REG_SPEC>;
#[doc = ""]
pub mod rf_adci_dc_offset_reg;
#[doc = "RF_ADCQ_DC_OFFSET_REG register accessor: an alias for `Reg<RF_ADCQ_DC_OFFSET_REG_SPEC>`"]
pub type RF_ADCQ_DC_OFFSET_REG = crate::Reg<rf_adcq_dc_offset_reg::RF_ADCQ_DC_OFFSET_REG_SPEC>;
#[doc = ""]
pub mod rf_adcq_dc_offset_reg;
#[doc = "RF_ADC_CTRL1_REG register accessor: an alias for `Reg<RF_ADC_CTRL1_REG_SPEC>`"]
pub type RF_ADC_CTRL1_REG = crate::Reg<rf_adc_ctrl1_reg::RF_ADC_CTRL1_REG_SPEC>;
#[doc = ""]
pub mod rf_adc_ctrl1_reg;
#[doc = "RF_ADC_CTRL2_REG register accessor: an alias for `Reg<RF_ADC_CTRL2_REG_SPEC>`"]
pub type RF_ADC_CTRL2_REG = crate::Reg<rf_adc_ctrl2_reg::RF_ADC_CTRL2_REG_SPEC>;
#[doc = ""]
pub mod rf_adc_ctrl2_reg;
#[doc = "RF_ADC_CTRL3_REG register accessor: an alias for `Reg<RF_ADC_CTRL3_REG_SPEC>`"]
pub type RF_ADC_CTRL3_REG = crate::Reg<rf_adc_ctrl3_reg::RF_ADC_CTRL3_REG_SPEC>;
#[doc = ""]
pub mod rf_adc_ctrl3_reg;
#[doc = "RF_ADPLLDIG_CTRL_REG register accessor: an alias for `Reg<RF_ADPLLDIG_CTRL_REG_SPEC>`"]
pub type RF_ADPLLDIG_CTRL_REG = crate::Reg<rf_adplldig_ctrl_reg::RF_ADPLLDIG_CTRL_REG_SPEC>;
#[doc = ""]
pub mod rf_adplldig_ctrl_reg;
#[doc = "RF_ADPLLDIG_RFMON_CTRL_REG register accessor: an alias for `Reg<RF_ADPLLDIG_RFMON_CTRL_REG_SPEC>`"]
pub type RF_ADPLLDIG_RFMON_CTRL_REG =
    crate::Reg<rf_adplldig_rfmon_ctrl_reg::RF_ADPLLDIG_RFMON_CTRL_REG_SPEC>;
#[doc = ""]
pub mod rf_adplldig_rfmon_ctrl_reg;
#[doc = "RF_AGC_EXT_LUT_REG register accessor: an alias for `Reg<RF_AGC_EXT_LUT_REG_SPEC>`"]
pub type RF_AGC_EXT_LUT_REG = crate::Reg<rf_agc_ext_lut_reg::RF_AGC_EXT_LUT_REG_SPEC>;
#[doc = ""]
pub mod rf_agc_ext_lut_reg;
#[doc = "RF_ATTR_REG register accessor: an alias for `Reg<RF_ATTR_REG_SPEC>`"]
pub type RF_ATTR_REG = crate::Reg<rf_attr_reg::RF_ATTR_REG_SPEC>;
#[doc = ""]
pub mod rf_attr_reg;
#[doc = "RF_CALSTATE_REG register accessor: an alias for `Reg<RF_CALSTATE_REG_SPEC>`"]
pub type RF_CALSTATE_REG = crate::Reg<rf_calstate_reg::RF_CALSTATE_REG_SPEC>;
#[doc = ""]
pub mod rf_calstate_reg;
#[doc = "RF_CAL_CTRL_REG register accessor: an alias for `Reg<RF_CAL_CTRL_REG_SPEC>`"]
pub type RF_CAL_CTRL_REG = crate::Reg<rf_cal_ctrl_reg::RF_CAL_CTRL_REG_SPEC>;
#[doc = ""]
pub mod rf_cal_ctrl_reg;
#[doc = "RF_DIAGIRQ_CTRL_REG register accessor: an alias for `Reg<RF_DIAGIRQ_CTRL_REG_SPEC>`"]
pub type RF_DIAGIRQ_CTRL_REG = crate::Reg<rf_diagirq_ctrl_reg::RF_DIAGIRQ_CTRL_REG_SPEC>;
#[doc = ""]
pub mod rf_diagirq_ctrl_reg;
#[doc = "RF_DIAGIRQ_STAT_REG register accessor: an alias for `Reg<RF_DIAGIRQ_STAT_REG_SPEC>`"]
pub type RF_DIAGIRQ_STAT_REG = crate::Reg<rf_diagirq_stat_reg::RF_DIAGIRQ_STAT_REG_SPEC>;
#[doc = ""]
pub mod rf_diagirq_stat_reg;
#[doc = "RF_IFF_CTRL_REG register accessor: an alias for `Reg<RF_IFF_CTRL_REG_SPEC>`"]
pub type RF_IFF_CTRL_REG = crate::Reg<rf_iff_ctrl_reg::RF_IFF_CTRL_REG_SPEC>;
#[doc = ""]
pub mod rf_iff_ctrl_reg;
#[doc = "RF_IO_CTRL_REG register accessor: an alias for `Reg<RF_IO_CTRL_REG_SPEC>`"]
pub type RF_IO_CTRL_REG = crate::Reg<rf_io_ctrl_reg::RF_IO_CTRL_REG_SPEC>;
#[doc = ""]
pub mod rf_io_ctrl_reg;
#[doc = "RF_IRQ_CTRL_REG register accessor: an alias for `Reg<RF_IRQ_CTRL_REG_SPEC>`"]
pub type RF_IRQ_CTRL_REG = crate::Reg<rf_irq_ctrl_reg::RF_IRQ_CTRL_REG_SPEC>;
#[doc = ""]
pub mod rf_irq_ctrl_reg;
#[doc = "RF_LDO_CTRL_REG register accessor: an alias for `Reg<RF_LDO_CTRL_REG_SPEC>`"]
pub type RF_LDO_CTRL_REG = crate::Reg<rf_ldo_ctrl_reg::RF_LDO_CTRL_REG_SPEC>;
#[doc = ""]
pub mod rf_ldo_ctrl_reg;
#[doc = "RF_LDO_STATUS_REG register accessor: an alias for `Reg<RF_LDO_STATUS_REG_SPEC>`"]
pub type RF_LDO_STATUS_REG = crate::Reg<rf_ldo_status_reg::RF_LDO_STATUS_REG_SPEC>;
#[doc = ""]
pub mod rf_ldo_status_reg;
#[doc = "RF_LDO_VREF_SEL_REG register accessor: an alias for `Reg<RF_LDO_VREF_SEL_REG_SPEC>`"]
pub type RF_LDO_VREF_SEL_REG = crate::Reg<rf_ldo_vref_sel_reg::RF_LDO_VREF_SEL_REG_SPEC>;
#[doc = ""]
pub mod rf_ldo_vref_sel_reg;
#[doc = "RF_LNA_CTRL1_REG register accessor: an alias for `Reg<RF_LNA_CTRL1_REG_SPEC>`"]
pub type RF_LNA_CTRL1_REG = crate::Reg<rf_lna_ctrl1_reg::RF_LNA_CTRL1_REG_SPEC>;
#[doc = ""]
pub mod rf_lna_ctrl1_reg;
#[doc = "RF_LNA_CTRL2_REG register accessor: an alias for `Reg<RF_LNA_CTRL2_REG_SPEC>`"]
pub type RF_LNA_CTRL2_REG = crate::Reg<rf_lna_ctrl2_reg::RF_LNA_CTRL2_REG_SPEC>;
#[doc = ""]
pub mod rf_lna_ctrl2_reg;
#[doc = "RF_LNA_CTRL3_REG register accessor: an alias for `Reg<RF_LNA_CTRL3_REG_SPEC>`"]
pub type RF_LNA_CTRL3_REG = crate::Reg<rf_lna_ctrl3_reg::RF_LNA_CTRL3_REG_SPEC>;
#[doc = ""]
pub mod rf_lna_ctrl3_reg;
#[doc = "RF_MIXER_CTRL1_REG register accessor: an alias for `Reg<RF_MIXER_CTRL1_REG_SPEC>`"]
pub type RF_MIXER_CTRL1_REG = crate::Reg<rf_mixer_ctrl1_reg::RF_MIXER_CTRL1_REG_SPEC>;
#[doc = ""]
pub mod rf_mixer_ctrl1_reg;
#[doc = "RF_MIXER_CTRL2_REG register accessor: an alias for `Reg<RF_MIXER_CTRL2_REG_SPEC>`"]
pub type RF_MIXER_CTRL2_REG = crate::Reg<rf_mixer_ctrl2_reg::RF_MIXER_CTRL2_REG_SPEC>;
#[doc = ""]
pub mod rf_mixer_ctrl2_reg;
#[doc = "RF_OVERRULE_REG register accessor: an alias for `Reg<RF_OVERRULE_REG_SPEC>`"]
pub type RF_OVERRULE_REG = crate::Reg<rf_overrule_reg::RF_OVERRULE_REG_SPEC>;
#[doc = ""]
pub mod rf_overrule_reg;
#[doc = "RF_PA_CTRL_REG register accessor: an alias for `Reg<RF_PA_CTRL_REG_SPEC>`"]
pub type RF_PA_CTRL_REG = crate::Reg<rf_pa_ctrl_reg::RF_PA_CTRL_REG_SPEC>;
#[doc = ""]
pub mod rf_pa_ctrl_reg;
#[doc = "RF_RADIO_INIT_REG register accessor: an alias for `Reg<RF_RADIO_INIT_REG_SPEC>`"]
pub type RF_RADIO_INIT_REG = crate::Reg<rf_radio_init_reg::RF_RADIO_INIT_REG_SPEC>;
#[doc = ""]
pub mod rf_radio_init_reg;
#[doc = "RF_RFCU_CTRL_REG register accessor: an alias for `Reg<RF_RFCU_CTRL_REG_SPEC>`"]
pub type RF_RFCU_CTRL_REG = crate::Reg<rf_rfcu_ctrl_reg::RF_RFCU_CTRL_REG_SPEC>;
#[doc = ""]
pub mod rf_rfcu_ctrl_reg;
#[doc = "RF_SCAN_FEEDBACK_REG register accessor: an alias for `Reg<RF_SCAN_FEEDBACK_REG_SPEC>`"]
pub type RF_SCAN_FEEDBACK_REG = crate::Reg<rf_scan_feedback_reg::RF_SCAN_FEEDBACK_REG_SPEC>;
#[doc = ""]
pub mod rf_scan_feedback_reg;
#[doc = "RF_SPARE_REG register accessor: an alias for `Reg<RF_SPARE_REG_SPEC>`"]
pub type RF_SPARE_REG = crate::Reg<rf_spare_reg::RF_SPARE_REG_SPEC>;
#[doc = ""]
pub mod rf_spare_reg;
