#[doc = "Register `HIBERN_CTRL_REG` reader"]
pub struct R(crate::R<HIBERN_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIBERN_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIBERN_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIBERN_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HIBERN_CTRL_REG` writer"]
pub struct W(crate::W<HIBERN_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HIBERN_CTRL_REG_SPEC>;
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
impl From<crate::W<HIBERN_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HIBERN_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIBERN_WKUP_MASK` reader - Selects which pin to wakeup from"]
pub struct HIBERN_WKUP_MASK_R(crate::FieldReader<u8, u8>);
impl HIBERN_WKUP_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HIBERN_WKUP_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIBERN_WKUP_MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIBERN_WKUP_MASK` writer - Selects which pin to wakeup from"]
pub struct HIBERN_WKUP_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> HIBERN_WKUP_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | ((value as u16 & 0x1f) << 2);
        self.w
    }
}
#[doc = "Field `HIBERN_WKUP_POLARITY` reader - Selects the polarity of the wakeup source. The polarity must be chosen such that the ANA_STATUS_REG\\[CLKLESS_WAKEUP_STAT\\]
is '1'. Any change on the selected GPIOs will make the CLKLESS_WAKEUP_STAT go to '0', and wakeup the system from hibernation."]
pub struct HIBERN_WKUP_POLARITY_R(crate::FieldReader<bool, bool>);
impl HIBERN_WKUP_POLARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HIBERN_WKUP_POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIBERN_WKUP_POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIBERN_WKUP_POLARITY` writer - Selects the polarity of the wakeup source. The polarity must be chosen such that the ANA_STATUS_REG\\[CLKLESS_WAKEUP_STAT\\]
is '1'. Any change on the selected GPIOs will make the CLKLESS_WAKEUP_STAT go to '0', and wakeup the system from hibernation."]
pub struct HIBERN_WKUP_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> HIBERN_WKUP_POLARITY_W<'a> {
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
#[doc = "Field `HIBERNATION_ENABLE` reader - Enables the hibernation mode when sleeping 0: deep sleep mode, PD_SLP remains on 1: hibernation mode, PD_SLP goes off. REMAP_ADR0 needs to be set to the correct source to boot from before going to sleep."]
pub struct HIBERNATION_ENABLE_R(crate::FieldReader<bool, bool>);
impl HIBERNATION_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HIBERNATION_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIBERNATION_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIBERNATION_ENABLE` writer - Enables the hibernation mode when sleeping 0: deep sleep mode, PD_SLP remains on 1: hibernation mode, PD_SLP goes off. REMAP_ADR0 needs to be set to the correct source to boot from before going to sleep."]
pub struct HIBERNATION_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> HIBERNATION_ENABLE_W<'a> {
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
    #[doc = "Bits 2:6 - Selects which pin to wakeup from"]
    #[inline(always)]
    pub fn hibern_wkup_mask(&self) -> HIBERN_WKUP_MASK_R {
        HIBERN_WKUP_MASK_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 1 - Selects the polarity of the wakeup source. The polarity must be chosen such that the ANA_STATUS_REG\\[CLKLESS_WAKEUP_STAT\\]
is '1'. Any change on the selected GPIOs will make the CLKLESS_WAKEUP_STAT go to '0', and wakeup the system from hibernation."]
    #[inline(always)]
    pub fn hibern_wkup_polarity(&self) -> HIBERN_WKUP_POLARITY_R {
        HIBERN_WKUP_POLARITY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Enables the hibernation mode when sleeping 0: deep sleep mode, PD_SLP remains on 1: hibernation mode, PD_SLP goes off. REMAP_ADR0 needs to be set to the correct source to boot from before going to sleep."]
    #[inline(always)]
    pub fn hibernation_enable(&self) -> HIBERNATION_ENABLE_R {
        HIBERNATION_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:6 - Selects which pin to wakeup from"]
    #[inline(always)]
    pub fn hibern_wkup_mask(&mut self) -> HIBERN_WKUP_MASK_W {
        HIBERN_WKUP_MASK_W { w: self }
    }
    #[doc = "Bit 1 - Selects the polarity of the wakeup source. The polarity must be chosen such that the ANA_STATUS_REG\\[CLKLESS_WAKEUP_STAT\\]
is '1'. Any change on the selected GPIOs will make the CLKLESS_WAKEUP_STAT go to '0', and wakeup the system from hibernation."]
    #[inline(always)]
    pub fn hibern_wkup_polarity(&mut self) -> HIBERN_WKUP_POLARITY_W {
        HIBERN_WKUP_POLARITY_W { w: self }
    }
    #[doc = "Bit 0 - Enables the hibernation mode when sleeping 0: deep sleep mode, PD_SLP remains on 1: hibernation mode, PD_SLP goes off. REMAP_ADR0 needs to be set to the correct source to boot from before going to sleep."]
    #[inline(always)]
    pub fn hibernation_enable(&mut self) -> HIBERNATION_ENABLE_W {
        HIBERNATION_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hibern_ctrl_reg](index.html) module"]
pub struct HIBERN_CTRL_REG_SPEC;
impl crate::RegisterSpec for HIBERN_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [hibern_ctrl_reg::R](R) reader structure"]
impl crate::Readable for HIBERN_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hibern_ctrl_reg::W](W) writer structure"]
impl crate::Writable for HIBERN_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HIBERN_CTRL_REG to value 0"]
impl crate::Resettable for HIBERN_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
