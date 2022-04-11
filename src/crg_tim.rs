#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x4c],
    #[doc = "0x4c - Divisor for RTC 100Hz clock"]
    pub clk_rtcdiv_reg: crate::Reg<clk_rtcdiv_reg::CLK_RTCDIV_REG_SPEC>,
}
#[doc = "CLK_RTCDIV_REG register accessor: an alias for `Reg<CLK_RTCDIV_REG_SPEC>`"]
pub type CLK_RTCDIV_REG = crate::Reg<clk_rtcdiv_reg::CLK_RTCDIV_REG_SPEC>;
#[doc = "Divisor for RTC 100Hz clock"]
pub mod clk_rtcdiv_reg;
