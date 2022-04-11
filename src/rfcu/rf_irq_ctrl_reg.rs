#[doc = "Register `RF_IRQ_CTRL_REG` reader"]
pub struct R(crate::R<RF_IRQ_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_IRQ_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_IRQ_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_IRQ_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_IRQ_CTRL_REG` writer"]
pub struct W(crate::W<RF_IRQ_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_IRQ_CTRL_REG_SPEC>;
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
impl From<crate::W<RF_IRQ_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_IRQ_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EO_CAL_CLEAR` writer - "]
pub struct EO_CAL_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> EO_CAL_CLEAR_W<'a> {
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
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn eo_cal_clear(&mut self) -> EO_CAL_CLEAR_W {
        EO_CAL_CLEAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_irq_ctrl_reg](index.html) module"]
pub struct RF_IRQ_CTRL_REG_SPEC;
impl crate::RegisterSpec for RF_IRQ_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_irq_ctrl_reg::R](R) reader structure"]
impl crate::Readable for RF_IRQ_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_irq_ctrl_reg::W](W) writer structure"]
impl crate::Writable for RF_IRQ_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_IRQ_CTRL_REG to value 0x01"]
impl crate::Resettable for RF_IRQ_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
