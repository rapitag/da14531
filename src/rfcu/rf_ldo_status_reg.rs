#[doc = "Register `RF_LDO_STATUS_REG` reader"]
pub struct R(crate::R<RF_LDO_STATUS_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_LDO_STATUS_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_LDO_STATUS_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_LDO_STATUS_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_LDO_STATUS_REG` writer"]
pub struct W(crate::W<RF_LDO_STATUS_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_LDO_STATUS_REG_SPEC>;
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
impl From<crate::W<RF_LDO_STATUS_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_LDO_STATUS_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ldo_dtc_vref_hold_rd` reader - "]
pub struct LDO_DTC_VREF_HOLD_RD_R(crate::FieldReader<bool, bool>);
impl LDO_DTC_VREF_HOLD_RD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_DTC_VREF_HOLD_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_DTC_VREF_HOLD_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo_dco_vref_hold_rd` reader - "]
pub struct LDO_DCO_VREF_HOLD_RD_R(crate::FieldReader<bool, bool>);
impl LDO_DCO_VREF_HOLD_RD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_DCO_VREF_HOLD_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_DCO_VREF_HOLD_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo_radio_vref_hold_rd` reader - "]
pub struct LDO_RADIO_VREF_HOLD_RD_R(crate::FieldReader<bool, bool>);
impl LDO_RADIO_VREF_HOLD_RD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_RADIO_VREF_HOLD_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_RADIO_VREF_HOLD_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo_dtc_en_rd` reader - "]
pub struct LDO_DTC_EN_RD_R(crate::FieldReader<bool, bool>);
impl LDO_DTC_EN_RD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_DTC_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_DTC_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo_dco_en_rd` reader - "]
pub struct LDO_DCO_EN_RD_R(crate::FieldReader<bool, bool>);
impl LDO_DCO_EN_RD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_DCO_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_DCO_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPLLDIG_LDO_ZERO_EN_RD` reader - "]
pub struct ADPLLDIG_LDO_ZERO_EN_RD_R(crate::FieldReader<bool, bool>);
impl ADPLLDIG_LDO_ZERO_EN_RD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLLDIG_LDO_ZERO_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLLDIG_LDO_ZERO_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPLLDIG_LDO_EN_RD` reader - "]
pub struct ADPLLDIG_LDO_EN_RD_R(crate::FieldReader<bool, bool>);
impl ADPLLDIG_LDO_EN_RD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLLDIG_LDO_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLLDIG_LDO_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RADIO_LDO_ZERO_EN_RD` reader - "]
pub struct RADIO_LDO_ZERO_EN_RD_R(crate::FieldReader<bool, bool>);
impl RADIO_LDO_ZERO_EN_RD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RADIO_LDO_ZERO_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RADIO_LDO_ZERO_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RADIO_LDO_EN_RD` reader - "]
pub struct RADIO_LDO_EN_RD_R(crate::FieldReader<bool, bool>);
impl RADIO_LDO_EN_RD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RADIO_LDO_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RADIO_LDO_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ldo_dtc_vref_hold_rd(&self) -> LDO_DTC_VREF_HOLD_RD_R {
        LDO_DTC_VREF_HOLD_RD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ldo_dco_vref_hold_rd(&self) -> LDO_DCO_VREF_HOLD_RD_R {
        LDO_DCO_VREF_HOLD_RD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ldo_radio_vref_hold_rd(&self) -> LDO_RADIO_VREF_HOLD_RD_R {
        LDO_RADIO_VREF_HOLD_RD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ldo_dtc_en_rd(&self) -> LDO_DTC_EN_RD_R {
        LDO_DTC_EN_RD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ldo_dco_en_rd(&self) -> LDO_DCO_EN_RD_R {
        LDO_DCO_EN_RD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn adplldig_ldo_zero_en_rd(&self) -> ADPLLDIG_LDO_ZERO_EN_RD_R {
        ADPLLDIG_LDO_ZERO_EN_RD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adplldig_ldo_en_rd(&self) -> ADPLLDIG_LDO_EN_RD_R {
        ADPLLDIG_LDO_EN_RD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn radio_ldo_zero_en_rd(&self) -> RADIO_LDO_ZERO_EN_RD_R {
        RADIO_LDO_ZERO_EN_RD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn radio_ldo_en_rd(&self) -> RADIO_LDO_EN_RD_R {
        RADIO_LDO_EN_RD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_ldo_status_reg](index.html) module"]
pub struct RF_LDO_STATUS_REG_SPEC;
impl crate::RegisterSpec for RF_LDO_STATUS_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_ldo_status_reg::R](R) reader structure"]
impl crate::Readable for RF_LDO_STATUS_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_ldo_status_reg::W](W) writer structure"]
impl crate::Writable for RF_LDO_STATUS_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_LDO_STATUS_REG to value 0"]
impl crate::Resettable for RF_LDO_STATUS_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
