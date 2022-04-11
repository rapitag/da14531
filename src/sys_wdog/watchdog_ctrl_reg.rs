#[doc = "Register `WATCHDOG_CTRL_REG` reader"]
pub struct R(crate::R<WATCHDOG_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WATCHDOG_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WATCHDOG_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WATCHDOG_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WATCHDOG_CTRL_REG` writer"]
pub struct W(crate::W<WATCHDOG_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WATCHDOG_CTRL_REG_SPEC>;
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
impl From<crate::W<WATCHDOG_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WATCHDOG_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NMI_RST` reader - 0 = Watchdog timer generates NMI at value 0, and WDOG (SYS) reset at <=-16. Timer can be frozen /resumed using SET_FREEZE_REG\\[FRZ_WDOG\\]/ RESET_FREEZE_REG\\[FRZ_WDOG\\]. 1 = Watchdog timer generates a WDOG (SYS) reset at value 0 and can not be frozen by Software. Note that this bit can only be set to 1 by SW and only be reset with a WDOG (SYS) reset or SW reset. The watchdog is always frozen when the Cortex-M0 is halted in DEBUG State."]
pub struct NMI_RST_R(crate::FieldReader<bool, bool>);
impl NMI_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NMI_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NMI_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMI_RST` writer - 0 = Watchdog timer generates NMI at value 0, and WDOG (SYS) reset at <=-16. Timer can be frozen /resumed using SET_FREEZE_REG\\[FRZ_WDOG\\]/ RESET_FREEZE_REG\\[FRZ_WDOG\\]. 1 = Watchdog timer generates a WDOG (SYS) reset at value 0 and can not be frozen by Software. Note that this bit can only be set to 1 by SW and only be reset with a WDOG (SYS) reset or SW reset. The watchdog is always frozen when the Cortex-M0 is halted in DEBUG State."]
pub struct NMI_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> NMI_RST_W<'a> {
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
    #[doc = "Bit 0 - 0 = Watchdog timer generates NMI at value 0, and WDOG (SYS) reset at <=-16. Timer can be frozen /resumed using SET_FREEZE_REG\\[FRZ_WDOG\\]/ RESET_FREEZE_REG\\[FRZ_WDOG\\]. 1 = Watchdog timer generates a WDOG (SYS) reset at value 0 and can not be frozen by Software. Note that this bit can only be set to 1 by SW and only be reset with a WDOG (SYS) reset or SW reset. The watchdog is always frozen when the Cortex-M0 is halted in DEBUG State."]
    #[inline(always)]
    pub fn nmi_rst(&self) -> NMI_RST_R {
        NMI_RST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0 = Watchdog timer generates NMI at value 0, and WDOG (SYS) reset at <=-16. Timer can be frozen /resumed using SET_FREEZE_REG\\[FRZ_WDOG\\]/ RESET_FREEZE_REG\\[FRZ_WDOG\\]. 1 = Watchdog timer generates a WDOG (SYS) reset at value 0 and can not be frozen by Software. Note that this bit can only be set to 1 by SW and only be reset with a WDOG (SYS) reset or SW reset. The watchdog is always frozen when the Cortex-M0 is halted in DEBUG State."]
    #[inline(always)]
    pub fn nmi_rst(&mut self) -> NMI_RST_W {
        NMI_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [watchdog_ctrl_reg](index.html) module"]
pub struct WATCHDOG_CTRL_REG_SPEC;
impl crate::RegisterSpec for WATCHDOG_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [watchdog_ctrl_reg::R](R) reader structure"]
impl crate::Readable for WATCHDOG_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [watchdog_ctrl_reg::W](W) writer structure"]
impl crate::Writable for WATCHDOG_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WATCHDOG_CTRL_REG to value 0"]
impl crate::Resettable for WATCHDOG_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
