#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register for the wakeup counter"]
    pub wkup_ctrl_reg: crate::Reg<wkup_ctrl_reg::WKUP_CTRL_REG_SPEC>,
    #[doc = "0x02 - Number of events before wakeup interrupt"]
    pub wkup_compare_reg: crate::Reg<wkup_compare_reg::WKUP_COMPARE_REG_SPEC>,
    #[doc = "0x04 - Reset wakeup interrupt"]
    pub wkup_irq_status_reg: crate::Reg<wkup_irq_status_reg::WKUP_IRQ_STATUS_REG_SPEC>,
    #[doc = "0x06 - Actual number of events of the wakeup counter"]
    pub wkup_counter_reg: crate::Reg<wkup_counter_reg::WKUP_COUNTER_REG_SPEC>,
    #[doc = "0x08 - Select which inputs from P0 port can trigger wkup counter"]
    pub wkup_select_gpio_reg: crate::Reg<wkup_select_gpio_reg::WKUP_SELECT_GPIO_REG_SPEC>,
    #[doc = "0x0a - Select which inputs from P1 port can trigger wkup counter"]
    pub wkup2_select_gpio_reg: crate::Reg<wkup2_select_gpio_reg::WKUP2_SELECT_GPIO_REG_SPEC>,
    #[doc = "0x0c - Select the sensitivity polarity for each P0 input"]
    pub wkup_pol_gpio_reg: crate::Reg<wkup_pol_gpio_reg::WKUP_POL_GPIO_REG_SPEC>,
    #[doc = "0x0e - Select the sensitivity polarity for each P1 input"]
    pub wkup2_pol_gpio_reg: crate::Reg<wkup2_pol_gpio_reg::WKUP2_POL_GPIO_REG_SPEC>,
}
#[doc = "WKUP2_POL_GPIO_REG register accessor: an alias for `Reg<WKUP2_POL_GPIO_REG_SPEC>`"]
pub type WKUP2_POL_GPIO_REG = crate::Reg<wkup2_pol_gpio_reg::WKUP2_POL_GPIO_REG_SPEC>;
#[doc = "Select the sensitivity polarity for each P1 input"]
pub mod wkup2_pol_gpio_reg;
#[doc = "WKUP2_SELECT_GPIO_REG register accessor: an alias for `Reg<WKUP2_SELECT_GPIO_REG_SPEC>`"]
pub type WKUP2_SELECT_GPIO_REG = crate::Reg<wkup2_select_gpio_reg::WKUP2_SELECT_GPIO_REG_SPEC>;
#[doc = "Select which inputs from P1 port can trigger wkup counter"]
pub mod wkup2_select_gpio_reg;
#[doc = "WKUP_COMPARE_REG register accessor: an alias for `Reg<WKUP_COMPARE_REG_SPEC>`"]
pub type WKUP_COMPARE_REG = crate::Reg<wkup_compare_reg::WKUP_COMPARE_REG_SPEC>;
#[doc = "Number of events before wakeup interrupt"]
pub mod wkup_compare_reg;
#[doc = "WKUP_COUNTER_REG register accessor: an alias for `Reg<WKUP_COUNTER_REG_SPEC>`"]
pub type WKUP_COUNTER_REG = crate::Reg<wkup_counter_reg::WKUP_COUNTER_REG_SPEC>;
#[doc = "Actual number of events of the wakeup counter"]
pub mod wkup_counter_reg;
#[doc = "WKUP_CTRL_REG register accessor: an alias for `Reg<WKUP_CTRL_REG_SPEC>`"]
pub type WKUP_CTRL_REG = crate::Reg<wkup_ctrl_reg::WKUP_CTRL_REG_SPEC>;
#[doc = "Control register for the wakeup counter"]
pub mod wkup_ctrl_reg;
#[doc = "WKUP_IRQ_STATUS_REG register accessor: an alias for `Reg<WKUP_IRQ_STATUS_REG_SPEC>`"]
pub type WKUP_IRQ_STATUS_REG = crate::Reg<wkup_irq_status_reg::WKUP_IRQ_STATUS_REG_SPEC>;
#[doc = "Reset wakeup interrupt"]
pub mod wkup_irq_status_reg;
#[doc = "WKUP_POL_GPIO_REG register accessor: an alias for `Reg<WKUP_POL_GPIO_REG_SPEC>`"]
pub type WKUP_POL_GPIO_REG = crate::Reg<wkup_pol_gpio_reg::WKUP_POL_GPIO_REG_SPEC>;
#[doc = "Select the sensitivity polarity for each P0 input"]
pub mod wkup_pol_gpio_reg;
#[doc = "WKUP_SELECT_GPIO_REG register accessor: an alias for `Reg<WKUP_SELECT_GPIO_REG_SPEC>`"]
pub type WKUP_SELECT_GPIO_REG = crate::Reg<wkup_select_gpio_reg::WKUP_SELECT_GPIO_REG_SPEC>;
#[doc = "Select which inputs from P0 port can trigger wkup counter"]
pub mod wkup_select_gpio_reg;
