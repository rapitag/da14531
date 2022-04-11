#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Select clock for oscillator calibration"]
    pub clk_ref_sel_reg: crate::Reg<clk_ref_sel_reg::CLK_REF_SEL_REG_SPEC>,
    #[doc = "0x02 - Count value for oscillator calibration"]
    pub clk_ref_cnt_reg: crate::Reg<clk_ref_cnt_reg::CLK_REF_CNT_REG_SPEC>,
    #[doc = "0x04 - XTAL32M reference cycles, lower 16 bits"]
    pub clk_ref_val_l_reg: crate::Reg<clk_ref_val_l_reg::CLK_REF_VAL_L_REG_SPEC>,
    #[doc = "0x06 - XTAL32M reference cycles, higher 16 bits"]
    pub clk_ref_val_h_reg: crate::Reg<clk_ref_val_h_reg::CLK_REF_VAL_H_REG_SPEC>,
}
#[doc = "CLK_REF_CNT_REG register accessor: an alias for `Reg<CLK_REF_CNT_REG_SPEC>`"]
pub type CLK_REF_CNT_REG = crate::Reg<clk_ref_cnt_reg::CLK_REF_CNT_REG_SPEC>;
#[doc = "Count value for oscillator calibration"]
pub mod clk_ref_cnt_reg;
#[doc = "CLK_REF_SEL_REG register accessor: an alias for `Reg<CLK_REF_SEL_REG_SPEC>`"]
pub type CLK_REF_SEL_REG = crate::Reg<clk_ref_sel_reg::CLK_REF_SEL_REG_SPEC>;
#[doc = "Select clock for oscillator calibration"]
pub mod clk_ref_sel_reg;
#[doc = "CLK_REF_VAL_H_REG register accessor: an alias for `Reg<CLK_REF_VAL_H_REG_SPEC>`"]
pub type CLK_REF_VAL_H_REG = crate::Reg<clk_ref_val_h_reg::CLK_REF_VAL_H_REG_SPEC>;
#[doc = "XTAL32M reference cycles, higher 16 bits"]
pub mod clk_ref_val_h_reg;
#[doc = "CLK_REF_VAL_L_REG register accessor: an alias for `Reg<CLK_REF_VAL_L_REG_SPEC>`"]
pub type CLK_REF_VAL_L_REG = crate::Reg<clk_ref_val_l_reg::CLK_REF_VAL_L_REG_SPEC>;
#[doc = "XTAL32M reference cycles, lower 16 bits"]
pub mod clk_ref_val_l_reg;
