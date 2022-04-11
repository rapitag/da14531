#[doc = "Register `ANA_STATUS_REG` reader"]
pub struct R(crate::R<ANA_STATUS_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANA_STATUS_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANA_STATUS_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANA_STATUS_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANA_STATUS_REG` writer"]
pub struct W(crate::W<ANA_STATUS_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANA_STATUS_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ANA_STATUS_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANA_STATUS_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKLESS_WAKEUP_STAT` reader - Indicates the output of the Clockless wakeup XOR tree. If this signal is '0', the chip will wake up. Use the HIBERN_WKUP_POLARITY bit to set the value to '1' before going into hibernation mode."]
pub struct CLKLESS_WAKEUP_STAT_R(crate::FieldReader<bool, bool>);
impl CLKLESS_WAKEUP_STAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKLESS_WAKEUP_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKLESS_WAKEUP_STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_RUNNING` reader - "]
pub struct FORCE_RUNNING_R(crate::FieldReader<bool, bool>);
impl FORCE_RUNNING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_RUNNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_RUNNING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_GPADC_OK` reader - Indicates that LDO_GPADC output is OK"]
pub struct LDO_GPADC_OK_R(crate::FieldReader<bool, bool>);
impl LDO_GPADC_OK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_GPADC_OK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_GPADC_OK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_XTAL_OK` reader - Indicates that LDO_XTAL output is OK"]
pub struct LDO_XTAL_OK_R(crate::FieldReader<bool, bool>);
impl LDO_XTAL_OK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_XTAL_OK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_XTAL_OK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOST_SELECTED` reader - 0: Buck mode detected 1: Boost mode detected"]
pub struct BOOST_SELECTED_R(crate::FieldReader<bool, bool>);
impl BOOST_SELECTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOOST_SELECTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOST_SELECTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POR_VBAT_HIGH` reader - Output of VBAT_HIGH supply rail voltage monitoring circuit. 0: Voltage level on VBAT_HIGH is lower than POR VBAT_HIGH threshold VTH_L (rail not ok, will result in reset if not masked) 1: Voltage level on VBAT_HIGH is higher than POR VBAT_HIGH threshold VTH_H (rail ok, reset released)"]
pub struct POR_VBAT_HIGH_R(crate::FieldReader<bool, bool>);
impl POR_VBAT_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POR_VBAT_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR_VBAT_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POR_VBAT_LOW` reader - Output of VBAT_LOW supply rail voltage monitoring circuit. 0: Voltage level on VBAT_LOW is lower than POR VBAT_LOW threshold VTH_L (rail not ok, will result in reset if not masked) 1: Voltage level on VBAT_LOW is higher than POR VBAT_LOW threshold VTH_H (rail ok, reset released)"]
pub struct POR_VBAT_LOW_R(crate::FieldReader<bool, bool>);
impl POR_VBAT_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POR_VBAT_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR_VBAT_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BANDGAP_OK` reader - Indicates that BANDGAP is OK"]
pub struct BANDGAP_OK_R(crate::FieldReader<bool, bool>);
impl BANDGAP_OK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BANDGAP_OK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BANDGAP_OK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP_VBAT_HIGH_NOK` reader - Indicates that VBAT_HIGH < VBAT_LOW -50 mV"]
pub struct COMP_VBAT_HIGH_NOK_R(crate::FieldReader<bool, bool>);
impl COMP_VBAT_HIGH_NOK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP_VBAT_HIGH_NOK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP_VBAT_HIGH_NOK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP_VBAT_HIGH_OK` reader - Indicates that VBAT_HIGH > VBAT_LOW +50 mV"]
pub struct COMP_VBAT_HIGH_OK_R(crate::FieldReader<bool, bool>);
impl COMP_VBAT_HIGH_OK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP_VBAT_HIGH_OK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP_VBAT_HIGH_OK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDC_OK` reader - Indicates that VBAT_LOW (buck mode) or VBAT_HIGH (boost mode) is OK"]
pub struct DCDC_OK_R(crate::FieldReader<bool, bool>);
impl DCDC_OK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCDC_OK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC_OK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_LOW_OK` reader - Indicates that LDO_LOW output is OK (only valid for high current mode)"]
pub struct LDO_LOW_OK_R(crate::FieldReader<bool, bool>);
impl LDO_LOW_OK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_LOW_OK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_LOW_OK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_CORE_OK` reader - Indicates that LDO_CORE output is OK"]
pub struct LDO_CORE_OK_R(crate::FieldReader<bool, bool>);
impl LDO_CORE_OK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_CORE_OK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_CORE_OK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 12 - Indicates the output of the Clockless wakeup XOR tree. If this signal is '0', the chip will wake up. Use the HIBERN_WKUP_POLARITY bit to set the value to '1' before going into hibernation mode."]
    #[inline(always)]
    pub fn clkless_wakeup_stat(&self) -> CLKLESS_WAKEUP_STAT_R {
        CLKLESS_WAKEUP_STAT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn force_running(&self) -> FORCE_RUNNING_R {
        FORCE_RUNNING_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates that LDO_GPADC output is OK"]
    #[inline(always)]
    pub fn ldo_gpadc_ok(&self) -> LDO_GPADC_OK_R {
        LDO_GPADC_OK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates that LDO_XTAL output is OK"]
    #[inline(always)]
    pub fn ldo_xtal_ok(&self) -> LDO_XTAL_OK_R {
        LDO_XTAL_OK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - 0: Buck mode detected 1: Boost mode detected"]
    #[inline(always)]
    pub fn boost_selected(&self) -> BOOST_SELECTED_R {
        BOOST_SELECTED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Output of VBAT_HIGH supply rail voltage monitoring circuit. 0: Voltage level on VBAT_HIGH is lower than POR VBAT_HIGH threshold VTH_L (rail not ok, will result in reset if not masked) 1: Voltage level on VBAT_HIGH is higher than POR VBAT_HIGH threshold VTH_H (rail ok, reset released)"]
    #[inline(always)]
    pub fn por_vbat_high(&self) -> POR_VBAT_HIGH_R {
        POR_VBAT_HIGH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Output of VBAT_LOW supply rail voltage monitoring circuit. 0: Voltage level on VBAT_LOW is lower than POR VBAT_LOW threshold VTH_L (rail not ok, will result in reset if not masked) 1: Voltage level on VBAT_LOW is higher than POR VBAT_LOW threshold VTH_H (rail ok, reset released)"]
    #[inline(always)]
    pub fn por_vbat_low(&self) -> POR_VBAT_LOW_R {
        POR_VBAT_LOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates that BANDGAP is OK"]
    #[inline(always)]
    pub fn bandgap_ok(&self) -> BANDGAP_OK_R {
        BANDGAP_OK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates that VBAT_HIGH < VBAT_LOW -50 mV"]
    #[inline(always)]
    pub fn comp_vbat_high_nok(&self) -> COMP_VBAT_HIGH_NOK_R {
        COMP_VBAT_HIGH_NOK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates that VBAT_HIGH > VBAT_LOW +50 mV"]
    #[inline(always)]
    pub fn comp_vbat_high_ok(&self) -> COMP_VBAT_HIGH_OK_R {
        COMP_VBAT_HIGH_OK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates that VBAT_LOW (buck mode) or VBAT_HIGH (boost mode) is OK"]
    #[inline(always)]
    pub fn dcdc_ok(&self) -> DCDC_OK_R {
        DCDC_OK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that LDO_LOW output is OK (only valid for high current mode)"]
    #[inline(always)]
    pub fn ldo_low_ok(&self) -> LDO_LOW_OK_R {
        LDO_LOW_OK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Indicates that LDO_CORE output is OK"]
    #[inline(always)]
    pub fn ldo_core_ok(&self) -> LDO_CORE_OK_R {
        LDO_CORE_OK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status bit of analog (power management) circuits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_status_reg](index.html) module"]
pub struct ANA_STATUS_REG_SPEC;
impl crate::RegisterSpec for ANA_STATUS_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ana_status_reg::R](R) reader structure"]
impl crate::Readable for ANA_STATUS_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ana_status_reg::W](W) writer structure"]
impl crate::Writable for ANA_STATUS_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANA_STATUS_REG to value 0"]
impl crate::Resettable for ANA_STATUS_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
