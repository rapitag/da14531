#[doc = "Register `TIMER1_CLR_EVENT_REG` reader"]
pub struct R(crate::R<TIMER1_CLR_EVENT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_CLR_EVENT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1_CLR_EVENT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1_CLR_EVENT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER1_CLR_EVENT_REG` writer"]
pub struct W(crate::W<TIMER1_CLR_EVENT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1_CLR_EVENT_REG_SPEC>;
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
impl From<crate::W<TIMER1_CLR_EVENT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1_CLR_EVENT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER1_CLR_IN2_EVENT` reader - Write 1 to clear the TIMER1_IN2_EVENT and TIMER1_IN2_OVRFLW"]
pub struct TIMER1_CLR_IN2_EVENT_R(crate::FieldReader<bool, bool>);
impl TIMER1_CLR_IN2_EVENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_CLR_IN2_EVENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_CLR_IN2_EVENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_CLR_IN2_EVENT` writer - Write 1 to clear the TIMER1_IN2_EVENT and TIMER1_IN2_OVRFLW"]
pub struct TIMER1_CLR_IN2_EVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_CLR_IN2_EVENT_W<'a> {
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
#[doc = "Field `TIMER1_CLR_IN1_EVENT` reader - Write 1 to clear the TIMER1_IN1_EVENT and TIMER1_IN1_OVRFLW"]
pub struct TIMER1_CLR_IN1_EVENT_R(crate::FieldReader<bool, bool>);
impl TIMER1_CLR_IN1_EVENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_CLR_IN1_EVENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_CLR_IN1_EVENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_CLR_IN1_EVENT` writer - Write 1 to clear the TIMER1_IN1_EVENT and TIMER1_IN1_OVRFLW"]
pub struct TIMER1_CLR_IN1_EVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_CLR_IN1_EVENT_W<'a> {
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
#[doc = "Field `TIMER1_CLR_TIMER_EVENT` reader - Write 1 to clear the TIMER1_TIMER_EVENT"]
pub struct TIMER1_CLR_TIMER_EVENT_R(crate::FieldReader<bool, bool>);
impl TIMER1_CLR_TIMER_EVENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_CLR_TIMER_EVENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_CLR_TIMER_EVENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_CLR_TIMER_EVENT` writer - Write 1 to clear the TIMER1_TIMER_EVENT"]
pub struct TIMER1_CLR_TIMER_EVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_CLR_TIMER_EVENT_W<'a> {
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
    #[doc = "Bit 2 - Write 1 to clear the TIMER1_IN2_EVENT and TIMER1_IN2_OVRFLW"]
    #[inline(always)]
    pub fn timer1_clr_in2_event(&self) -> TIMER1_CLR_IN2_EVENT_R {
        TIMER1_CLR_IN2_EVENT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to clear the TIMER1_IN1_EVENT and TIMER1_IN1_OVRFLW"]
    #[inline(always)]
    pub fn timer1_clr_in1_event(&self) -> TIMER1_CLR_IN1_EVENT_R {
        TIMER1_CLR_IN1_EVENT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Write 1 to clear the TIMER1_TIMER_EVENT"]
    #[inline(always)]
    pub fn timer1_clr_timer_event(&self) -> TIMER1_CLR_TIMER_EVENT_R {
        TIMER1_CLR_TIMER_EVENT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Write 1 to clear the TIMER1_IN2_EVENT and TIMER1_IN2_OVRFLW"]
    #[inline(always)]
    pub fn timer1_clr_in2_event(&mut self) -> TIMER1_CLR_IN2_EVENT_W {
        TIMER1_CLR_IN2_EVENT_W { w: self }
    }
    #[doc = "Bit 1 - Write 1 to clear the TIMER1_IN1_EVENT and TIMER1_IN1_OVRFLW"]
    #[inline(always)]
    pub fn timer1_clr_in1_event(&mut self) -> TIMER1_CLR_IN1_EVENT_W {
        TIMER1_CLR_IN1_EVENT_W { w: self }
    }
    #[doc = "Bit 0 - Write 1 to clear the TIMER1_TIMER_EVENT"]
    #[inline(always)]
    pub fn timer1_clr_timer_event(&mut self) -> TIMER1_CLR_TIMER_EVENT_W {
        TIMER1_CLR_TIMER_EVENT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1_clr_event_reg](index.html) module"]
pub struct TIMER1_CLR_EVENT_REG_SPEC;
impl crate::RegisterSpec for TIMER1_CLR_EVENT_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1_clr_event_reg::R](R) reader structure"]
impl crate::Readable for TIMER1_CLR_EVENT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1_clr_event_reg::W](W) writer structure"]
impl crate::Writable for TIMER1_CLR_EVENT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER1_CLR_EVENT_REG to value 0"]
impl crate::Resettable for TIMER1_CLR_EVENT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
