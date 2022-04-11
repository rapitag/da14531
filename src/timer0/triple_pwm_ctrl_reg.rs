#[doc = "Register `TRIPLE_PWM_CTRL_REG` reader"]
pub struct R(crate::R<TRIPLE_PWM_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIPLE_PWM_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIPLE_PWM_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIPLE_PWM_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIPLE_PWM_CTRL_REG` writer"]
pub struct W(crate::W<TRIPLE_PWM_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIPLE_PWM_CTRL_REG_SPEC>;
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
impl From<crate::W<TRIPLE_PWM_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIPLE_PWM_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIPLE_PWM_CLK_SEL` reader - 1 = Timer2 uses 16, 8, 4 or 2 MHz (fast) clock frequency. 0 = Timer2 uses LP clock"]
pub struct TRIPLE_PWM_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl TRIPLE_PWM_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIPLE_PWM_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIPLE_PWM_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIPLE_PWM_CLK_SEL` writer - 1 = Timer2 uses 16, 8, 4 or 2 MHz (fast) clock frequency. 0 = Timer2 uses LP clock"]
pub struct TRIPLE_PWM_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIPLE_PWM_CLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u16 & 1) << 3);
        self.w
    }
}
#[doc = "Field `HW_PAUSE_EN` reader - '1' = HW can pause PWM 2,3,4,5,6,7"]
pub struct HW_PAUSE_EN_R(crate::FieldReader<bool, bool>);
impl HW_PAUSE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HW_PAUSE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HW_PAUSE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HW_PAUSE_EN` writer - '1' = HW can pause PWM 2,3,4,5,6,7"]
pub struct HW_PAUSE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_PAUSE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u16 & 1) << 2);
        self.w
    }
}
#[doc = "Field `SW_PAUSE_EN` reader - '1' = PWM 2 3 4 5 6 7 are paused"]
pub struct SW_PAUSE_EN_R(crate::FieldReader<bool, bool>);
impl SW_PAUSE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW_PAUSE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_PAUSE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_PAUSE_EN` writer - '1' = PWM 2 3 4 5 6 7 are paused"]
pub struct SW_PAUSE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_PAUSE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u16 & 1) << 1);
        self.w
    }
}
#[doc = "Field `TRIPLE_PWM_ENABLE` reader - '1' = enable PWM 2 3 4 5 6 7"]
pub struct TRIPLE_PWM_ENABLE_R(crate::FieldReader<bool, bool>);
impl TRIPLE_PWM_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIPLE_PWM_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIPLE_PWM_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIPLE_PWM_ENABLE` writer - '1' = enable PWM 2 3 4 5 6 7"]
pub struct TRIPLE_PWM_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIPLE_PWM_ENABLE_W<'a> {
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
    #[doc = "Bit 3 - 1 = Timer2 uses 16, 8, 4 or 2 MHz (fast) clock frequency. 0 = Timer2 uses LP clock"]
    #[inline(always)]
    pub fn triple_pwm_clk_sel(&self) -> TRIPLE_PWM_CLK_SEL_R {
        TRIPLE_PWM_CLK_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - '1' = HW can pause PWM 2,3,4,5,6,7"]
    #[inline(always)]
    pub fn hw_pause_en(&self) -> HW_PAUSE_EN_R {
        HW_PAUSE_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - '1' = PWM 2 3 4 5 6 7 are paused"]
    #[inline(always)]
    pub fn sw_pause_en(&self) -> SW_PAUSE_EN_R {
        SW_PAUSE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - '1' = enable PWM 2 3 4 5 6 7"]
    #[inline(always)]
    pub fn triple_pwm_enable(&self) -> TRIPLE_PWM_ENABLE_R {
        TRIPLE_PWM_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 1 = Timer2 uses 16, 8, 4 or 2 MHz (fast) clock frequency. 0 = Timer2 uses LP clock"]
    #[inline(always)]
    pub fn triple_pwm_clk_sel(&mut self) -> TRIPLE_PWM_CLK_SEL_W {
        TRIPLE_PWM_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 2 - '1' = HW can pause PWM 2,3,4,5,6,7"]
    #[inline(always)]
    pub fn hw_pause_en(&mut self) -> HW_PAUSE_EN_W {
        HW_PAUSE_EN_W { w: self }
    }
    #[doc = "Bit 1 - '1' = PWM 2 3 4 5 6 7 are paused"]
    #[inline(always)]
    pub fn sw_pause_en(&mut self) -> SW_PAUSE_EN_W {
        SW_PAUSE_EN_W { w: self }
    }
    #[doc = "Bit 0 - '1' = enable PWM 2 3 4 5 6 7"]
    #[inline(always)]
    pub fn triple_pwm_enable(&mut self) -> TRIPLE_PWM_ENABLE_W {
        TRIPLE_PWM_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM 2,3,4,5,6,7 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [triple_pwm_ctrl_reg](index.html) module"]
pub struct TRIPLE_PWM_CTRL_REG_SPEC;
impl crate::RegisterSpec for TRIPLE_PWM_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [triple_pwm_ctrl_reg::R](R) reader structure"]
impl crate::Readable for TRIPLE_PWM_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [triple_pwm_ctrl_reg::W](W) writer structure"]
impl crate::Writable for TRIPLE_PWM_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIPLE_PWM_CTRL_REG to value 0x04"]
impl crate::Resettable for TRIPLE_PWM_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
