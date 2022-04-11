#[doc = "Register `GP_STATUS_REG` reader"]
pub struct R(crate::R<GP_STATUS_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_STATUS_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_STATUS_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_STATUS_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GP_STATUS_REG` writer"]
pub struct W(crate::W<GP_STATUS_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_STATUS_REG_SPEC>;
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
impl From<crate::W<GP_STATUS_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_STATUS_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL_PHASE` reader - If '1', it designates that the chip is in Calibration Phase i.e. the OTP has been initially programmed but no Calibration has occured."]
pub struct CAL_PHASE_R(crate::FieldReader<bool, bool>);
impl CAL_PHASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAL_PHASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL_PHASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAL_PHASE` writer - If '1', it designates that the chip is in Calibration Phase i.e. the OTP has been initially programmed but no Calibration has occured."]
pub struct CAL_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_PHASE_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u16 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - If '1', it designates that the chip is in Calibration Phase i.e. the OTP has been initially programmed but no Calibration has occured."]
    #[inline(always)]
    pub fn cal_phase(&self) -> CAL_PHASE_R {
        CAL_PHASE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If '1', it designates that the chip is in Calibration Phase i.e. the OTP has been initially programmed but no Calibration has occured."]
    #[inline(always)]
    pub fn cal_phase(&mut self) -> CAL_PHASE_W {
        CAL_PHASE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General purpose system status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_status_reg](index.html) module"]
pub struct GP_STATUS_REG_SPEC;
impl crate::RegisterSpec for GP_STATUS_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gp_status_reg::R](R) reader structure"]
impl crate::Readable for GP_STATUS_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_status_reg::W](W) writer structure"]
impl crate::Writable for GP_STATUS_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GP_STATUS_REG to value 0"]
impl crate::Resettable for GP_STATUS_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
