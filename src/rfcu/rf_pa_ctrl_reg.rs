#[doc = "Register `RF_PA_CTRL_REG` reader"]
pub struct R(crate::R<RF_PA_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_PA_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_PA_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_PA_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_PA_CTRL_REG` writer"]
pub struct W(crate::W<RF_PA_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_PA_CTRL_REG_SPEC>;
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
impl From<crate::W<RF_PA_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_PA_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA_RAMP_STEP_SPEED` reader - "]
pub struct PA_RAMP_STEP_SPEED_R(crate::FieldReader<u8, u8>);
impl PA_RAMP_STEP_SPEED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PA_RAMP_STEP_SPEED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_RAMP_STEP_SPEED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA_RAMP_STEP_SPEED` writer - "]
pub struct PA_RAMP_STEP_SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_RAMP_STEP_SPEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `TRIM_DUTY_NEG` reader - "]
pub struct TRIM_DUTY_NEG_R(crate::FieldReader<u8, u8>);
impl TRIM_DUTY_NEG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRIM_DUTY_NEG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_DUTY_NEG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIM_DUTY_NEG` writer - "]
pub struct TRIM_DUTY_NEG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_DUTY_NEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 3)) | ((value as u32 & 7) << 3);
        self.w
    }
}
#[doc = "Field `TRIM_DUTY_POS` reader - "]
pub struct TRIM_DUTY_POS_R(crate::FieldReader<u8, u8>);
impl TRIM_DUTY_POS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRIM_DUTY_POS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_DUTY_POS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIM_DUTY_POS` writer - "]
pub struct TRIM_DUTY_POS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_DUTY_POS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pa_ramp_step_speed(&self) -> PA_RAMP_STEP_SPEED_R {
        PA_RAMP_STEP_SPEED_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn trim_duty_neg(&self) -> TRIM_DUTY_NEG_R {
        TRIM_DUTY_NEG_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn trim_duty_pos(&self) -> TRIM_DUTY_POS_R {
        TRIM_DUTY_POS_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pa_ramp_step_speed(&mut self) -> PA_RAMP_STEP_SPEED_W {
        PA_RAMP_STEP_SPEED_W { w: self }
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn trim_duty_neg(&mut self) -> TRIM_DUTY_NEG_W {
        TRIM_DUTY_NEG_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn trim_duty_pos(&mut self) -> TRIM_DUTY_POS_W {
        TRIM_DUTY_POS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_pa_ctrl_reg](index.html) module"]
pub struct RF_PA_CTRL_REG_SPEC;
impl crate::RegisterSpec for RF_PA_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_pa_ctrl_reg::R](R) reader structure"]
impl crate::Readable for RF_PA_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_pa_ctrl_reg::W](W) writer structure"]
impl crate::Writable for RF_PA_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_PA_CTRL_REG to value 0x0300"]
impl crate::Resettable for RF_PA_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0300
    }
}
