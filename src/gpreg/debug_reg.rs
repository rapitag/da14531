#[doc = "Register `DEBUG_REG` reader"]
pub struct R(crate::R<DEBUG_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG_REG` writer"]
pub struct W(crate::W<DEBUG_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_REG_SPEC>;
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
impl From<crate::W<DEBUG_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEBUGS_FREEZE_EN` reader - Default '1', freezing of the on-chip timers is enabled when the Cortex-M0Plus is halted in DEBUG State. If '0', freezing of the on-chip timers is depending on FREEZE_REG when the Cortex-M0Plus is halted in DEBUG State except the watchdog timer. The watchdog timer is always frozen when the Cortex-M0Plus is halted in DEBUG State."]
pub struct DEBUGS_FREEZE_EN_R(crate::FieldReader<bool, bool>);
impl DEBUGS_FREEZE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEBUGS_FREEZE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUGS_FREEZE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEBUGS_FREEZE_EN` writer - Default '1', freezing of the on-chip timers is enabled when the Cortex-M0Plus is halted in DEBUG State. If '0', freezing of the on-chip timers is depending on FREEZE_REG when the Cortex-M0Plus is halted in DEBUG State except the watchdog timer. The watchdog timer is always frozen when the Cortex-M0Plus is halted in DEBUG State."]
pub struct DEBUGS_FREEZE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUGS_FREEZE_EN_W<'a> {
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
    #[doc = "Bit 0 - Default '1', freezing of the on-chip timers is enabled when the Cortex-M0Plus is halted in DEBUG State. If '0', freezing of the on-chip timers is depending on FREEZE_REG when the Cortex-M0Plus is halted in DEBUG State except the watchdog timer. The watchdog timer is always frozen when the Cortex-M0Plus is halted in DEBUG State."]
    #[inline(always)]
    pub fn debugs_freeze_en(&self) -> DEBUGS_FREEZE_EN_R {
        DEBUGS_FREEZE_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Default '1', freezing of the on-chip timers is enabled when the Cortex-M0Plus is halted in DEBUG State. If '0', freezing of the on-chip timers is depending on FREEZE_REG when the Cortex-M0Plus is halted in DEBUG State except the watchdog timer. The watchdog timer is always frozen when the Cortex-M0Plus is halted in DEBUG State."]
    #[inline(always)]
    pub fn debugs_freeze_en(&mut self) -> DEBUGS_FREEZE_EN_W {
        DEBUGS_FREEZE_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Various debug information register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_reg](index.html) module"]
pub struct DEBUG_REG_SPEC;
impl crate::RegisterSpec for DEBUG_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [debug_reg::R](R) reader structure"]
impl crate::Readable for DEBUG_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug_reg::W](W) writer structure"]
impl crate::Writable for DEBUG_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEBUG_REG to value 0x01"]
impl crate::Resettable for DEBUG_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
