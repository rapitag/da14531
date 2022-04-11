#[doc = "Register `WKUP_CTRL_REG` reader"]
pub struct R(crate::R<WKUP_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKUP_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKUP_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKUP_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKUP_CTRL_REG` writer"]
pub struct W(crate::W<WKUP_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKUP_CTRL_REG_SPEC>;
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
impl From<crate::W<WKUP_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKUP_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUP2_ENABLE_IRQ` reader - 0 = no interrupt will be generated 1 = if the event counter2 reaches the value set by WKUP_COMPARE_REG an IRQ will be generated"]
pub struct WKUP2_ENABLE_IRQ_R(crate::FieldReader<bool, bool>);
impl WKUP2_ENABLE_IRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUP2_ENABLE_IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUP2_ENABLE_IRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUP2_ENABLE_IRQ` writer - 0 = no interrupt will be generated 1 = if the event counter2 reaches the value set by WKUP_COMPARE_REG an IRQ will be generated"]
pub struct WKUP2_ENABLE_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUP2_ENABLE_IRQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u16 & 1) << 8);
        self.w
    }
}
#[doc = "Field `WKUP_ENABLE_IRQ` reader - 0 = no interrupt will be generated 1 = if the event counter reaches the value set by WKUP_COMPARE_REG an IRQ will be generated"]
pub struct WKUP_ENABLE_IRQ_R(crate::FieldReader<bool, bool>);
impl WKUP_ENABLE_IRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUP_ENABLE_IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUP_ENABLE_IRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUP_ENABLE_IRQ` writer - 0 = no interrupt will be generated 1 = if the event counter reaches the value set by WKUP_COMPARE_REG an IRQ will be generated"]
pub struct WKUP_ENABLE_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUP_ENABLE_IRQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u16 & 1) << 7);
        self.w
    }
}
#[doc = "Field `WKUP_SFT_KEYHIT` reader - 0 = no effect 1 = emulate key hit. The event counter and counter2 will increment by 1 (after debouncing if enabled). First make this bit 0 before any new key hit can be sensed."]
pub struct WKUP_SFT_KEYHIT_R(crate::FieldReader<bool, bool>);
impl WKUP_SFT_KEYHIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUP_SFT_KEYHIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUP_SFT_KEYHIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUP_SFT_KEYHIT` writer - 0 = no effect 1 = emulate key hit. The event counter and counter2 will increment by 1 (after debouncing if enabled). First make this bit 0 before any new key hit can be sensed."]
pub struct WKUP_SFT_KEYHIT_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUP_SFT_KEYHIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u16 & 1) << 6);
        self.w
    }
}
#[doc = "Field `WKUP_DEB_VALUE` reader - Keyboard debounce time (N*1 ms with N = 1 to 63). 0x0: no debouncing 0x1 to 0x3F: 1 ms to 63 ms debounce time"]
pub struct WKUP_DEB_VALUE_R(crate::FieldReader<u8, u8>);
impl WKUP_DEB_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WKUP_DEB_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUP_DEB_VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUP_DEB_VALUE` writer - Keyboard debounce time (N*1 ms with N = 1 to 63). 0x0: no debouncing 0x1 to 0x3F: 1 ms to 63 ms debounce time"]
pub struct WKUP_DEB_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUP_DEB_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u16 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - 0 = no interrupt will be generated 1 = if the event counter2 reaches the value set by WKUP_COMPARE_REG an IRQ will be generated"]
    #[inline(always)]
    pub fn wkup2_enable_irq(&self) -> WKUP2_ENABLE_IRQ_R {
        WKUP2_ENABLE_IRQ_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - 0 = no interrupt will be generated 1 = if the event counter reaches the value set by WKUP_COMPARE_REG an IRQ will be generated"]
    #[inline(always)]
    pub fn wkup_enable_irq(&self) -> WKUP_ENABLE_IRQ_R {
        WKUP_ENABLE_IRQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - 0 = no effect 1 = emulate key hit. The event counter and counter2 will increment by 1 (after debouncing if enabled). First make this bit 0 before any new key hit can be sensed."]
    #[inline(always)]
    pub fn wkup_sft_keyhit(&self) -> WKUP_SFT_KEYHIT_R {
        WKUP_SFT_KEYHIT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 0:5 - Keyboard debounce time (N*1 ms with N = 1 to 63). 0x0: no debouncing 0x1 to 0x3F: 1 ms to 63 ms debounce time"]
    #[inline(always)]
    pub fn wkup_deb_value(&self) -> WKUP_DEB_VALUE_R {
        WKUP_DEB_VALUE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - 0 = no interrupt will be generated 1 = if the event counter2 reaches the value set by WKUP_COMPARE_REG an IRQ will be generated"]
    #[inline(always)]
    pub fn wkup2_enable_irq(&mut self) -> WKUP2_ENABLE_IRQ_W {
        WKUP2_ENABLE_IRQ_W { w: self }
    }
    #[doc = "Bit 7 - 0 = no interrupt will be generated 1 = if the event counter reaches the value set by WKUP_COMPARE_REG an IRQ will be generated"]
    #[inline(always)]
    pub fn wkup_enable_irq(&mut self) -> WKUP_ENABLE_IRQ_W {
        WKUP_ENABLE_IRQ_W { w: self }
    }
    #[doc = "Bit 6 - 0 = no effect 1 = emulate key hit. The event counter and counter2 will increment by 1 (after debouncing if enabled). First make this bit 0 before any new key hit can be sensed."]
    #[inline(always)]
    pub fn wkup_sft_keyhit(&mut self) -> WKUP_SFT_KEYHIT_W {
        WKUP_SFT_KEYHIT_W { w: self }
    }
    #[doc = "Bits 0:5 - Keyboard debounce time (N*1 ms with N = 1 to 63). 0x0: no debouncing 0x1 to 0x3F: 1 ms to 63 ms debounce time"]
    #[inline(always)]
    pub fn wkup_deb_value(&mut self) -> WKUP_DEB_VALUE_W {
        WKUP_DEB_VALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register for the wakeup counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkup_ctrl_reg](index.html) module"]
pub struct WKUP_CTRL_REG_SPEC;
impl crate::RegisterSpec for WKUP_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wkup_ctrl_reg::R](R) reader structure"]
impl crate::Readable for WKUP_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wkup_ctrl_reg::W](W) writer structure"]
impl crate::Writable for WKUP_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WKUP_CTRL_REG to value 0"]
impl crate::Resettable for WKUP_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
