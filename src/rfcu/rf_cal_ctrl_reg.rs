#[doc = "Register `RF_CAL_CTRL_REG` reader"]
pub struct R(crate::R<RF_CAL_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_CAL_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_CAL_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_CAL_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_CAL_CTRL_REG` writer"]
pub struct W(crate::W<RF_CAL_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_CAL_CTRL_REG_SPEC>;
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
impl From<crate::W<RF_CAL_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_CAL_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DC_OFFSET_CAL_DIS` reader - "]
pub struct DC_OFFSET_CAL_DIS_R(crate::FieldReader<bool, bool>);
impl DC_OFFSET_CAL_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DC_OFFSET_CAL_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_OFFSET_CAL_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC_OFFSET_CAL_DIS` writer - "]
pub struct DC_OFFSET_CAL_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_OFFSET_CAL_DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `RF_CAL_CTRL_SPARE` reader - "]
pub struct RF_CAL_CTRL_SPARE_R(crate::FieldReader<bool, bool>);
impl RF_CAL_CTRL_SPARE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_CAL_CTRL_SPARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_CAL_CTRL_SPARE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF_CAL_CTRL_SPARE` writer - "]
pub struct RF_CAL_CTRL_SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_CAL_CTRL_SPARE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `EO_CAL` reader - "]
pub struct EO_CAL_R(crate::FieldReader<bool, bool>);
impl EO_CAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EO_CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EO_CAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SO_CAL` writer - "]
pub struct SO_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SO_CAL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dc_offset_cal_dis(&self) -> DC_OFFSET_CAL_DIS_R {
        DC_OFFSET_CAL_DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rf_cal_ctrl_spare(&self) -> RF_CAL_CTRL_SPARE_R {
        RF_CAL_CTRL_SPARE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn eo_cal(&self) -> EO_CAL_R {
        EO_CAL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dc_offset_cal_dis(&mut self) -> DC_OFFSET_CAL_DIS_W {
        DC_OFFSET_CAL_DIS_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rf_cal_ctrl_spare(&mut self) -> RF_CAL_CTRL_SPARE_W {
        RF_CAL_CTRL_SPARE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn so_cal(&mut self) -> SO_CAL_W {
        SO_CAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_cal_ctrl_reg](index.html) module"]
pub struct RF_CAL_CTRL_REG_SPEC;
impl crate::RegisterSpec for RF_CAL_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_cal_ctrl_reg::R](R) reader structure"]
impl crate::Readable for RF_CAL_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_cal_ctrl_reg::W](W) writer structure"]
impl crate::Writable for RF_CAL_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_CAL_CTRL_REG to value 0"]
impl crate::Resettable for RF_CAL_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
