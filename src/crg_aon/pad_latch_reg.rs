#[doc = "Register `PAD_LATCH_REG` reader"]
pub struct R(crate::R<PAD_LATCH_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_LATCH_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_LATCH_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_LATCH_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_LATCH_REG` writer"]
pub struct W(crate::W<PAD_LATCH_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_LATCH_REG_SPEC>;
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
impl From<crate::W<PAD_LATCH_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_LATCH_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_LATCH_EN` reader - Controls the state retention of the pads. 0: latches are closed, pads retain their state. 1: latches are open, new control values have immediate effect"]
pub struct PAD_LATCH_EN_R(crate::FieldReader<bool, bool>);
impl PAD_LATCH_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_LATCH_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_LATCH_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_LATCH_EN` writer - Controls the state retention of the pads. 0: latches are closed, pads retain their state. 1: latches are open, new control values have immediate effect"]
pub struct PAD_LATCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_LATCH_EN_W<'a> {
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
    #[doc = "Bit 0 - Controls the state retention of the pads. 0: latches are closed, pads retain their state. 1: latches are open, new control values have immediate effect"]
    #[inline(always)]
    pub fn pad_latch_en(&self) -> PAD_LATCH_EN_R {
        PAD_LATCH_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls the state retention of the pads. 0: latches are closed, pads retain their state. 1: latches are open, new control values have immediate effect"]
    #[inline(always)]
    pub fn pad_latch_en(&mut self) -> PAD_LATCH_EN_W {
        PAD_LATCH_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control the state retention of the GPIO ports\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_latch_reg](index.html) module"]
pub struct PAD_LATCH_REG_SPEC;
impl crate::RegisterSpec for PAD_LATCH_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pad_latch_reg::R](R) reader structure"]
impl crate::Readable for PAD_LATCH_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_latch_reg::W](W) writer structure"]
impl crate::Writable for PAD_LATCH_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_LATCH_REG to value 0x01"]
impl crate::Resettable for PAD_LATCH_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
