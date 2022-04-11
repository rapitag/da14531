#[doc = "Register `RF_ADPLLDIG_CTRL_REG` reader"]
pub struct R(crate::R<RF_ADPLLDIG_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_ADPLLDIG_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_ADPLLDIG_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_ADPLLDIG_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_ADPLLDIG_CTRL_REG` writer"]
pub struct W(crate::W<RF_ADPLLDIG_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_ADPLLDIG_CTRL_REG_SPEC>;
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
impl From<crate::W<RF_ADPLLDIG_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_ADPLLDIG_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWR_SW_TIM_CTRL` reader - "]
pub struct PWR_SW_TIM_CTRL_R(crate::FieldReader<u8, u8>);
impl PWR_SW_TIM_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PWR_SW_TIM_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_SW_TIM_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_SW_TIM_CTRL` writer - "]
pub struct PWR_SW_TIM_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_SW_TIM_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 4)) | ((value as u32 & 7) << 4);
        self.w
    }
}
#[doc = "Field `OPENLOOP_RDY_WR` reader - "]
pub struct OPENLOOP_RDY_WR_R(crate::FieldReader<bool, bool>);
impl OPENLOOP_RDY_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OPENLOOP_RDY_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPENLOOP_RDY_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPENLOOP_RDY_WR` writer - "]
pub struct OPENLOOP_RDY_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPENLOOP_RDY_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `OPENLOOP_RDY_SEL` reader - "]
pub struct OPENLOOP_RDY_SEL_R(crate::FieldReader<bool, bool>);
impl OPENLOOP_RDY_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OPENLOOP_RDY_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPENLOOP_RDY_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPENLOOP_RDY_SEL` writer - "]
pub struct OPENLOOP_RDY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OPENLOOP_RDY_SEL_W<'a> {
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
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn pwr_sw_tim_ctrl(&self) -> PWR_SW_TIM_CTRL_R {
        PWR_SW_TIM_CTRL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn openloop_rdy_wr(&self) -> OPENLOOP_RDY_WR_R {
        OPENLOOP_RDY_WR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn openloop_rdy_sel(&self) -> OPENLOOP_RDY_SEL_R {
        OPENLOOP_RDY_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn pwr_sw_tim_ctrl(&mut self) -> PWR_SW_TIM_CTRL_W {
        PWR_SW_TIM_CTRL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn openloop_rdy_wr(&mut self) -> OPENLOOP_RDY_WR_W {
        OPENLOOP_RDY_WR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn openloop_rdy_sel(&mut self) -> OPENLOOP_RDY_SEL_W {
        OPENLOOP_RDY_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_adplldig_ctrl_reg](index.html) module"]
pub struct RF_ADPLLDIG_CTRL_REG_SPEC;
impl crate::RegisterSpec for RF_ADPLLDIG_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_adplldig_ctrl_reg::R](R) reader structure"]
impl crate::Readable for RF_ADPLLDIG_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_adplldig_ctrl_reg::W](W) writer structure"]
impl crate::Writable for RF_ADPLLDIG_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_ADPLLDIG_CTRL_REG to value 0x10"]
impl crate::Resettable for RF_ADPLLDIG_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
