#[doc = "Register `BLE_DEEPSLCNTL_REG` reader"]
pub struct R(crate::R<BLE_DEEPSLCNTL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_DEEPSLCNTL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_DEEPSLCNTL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_DEEPSLCNTL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_DEEPSLCNTL_REG` writer"]
pub struct W(crate::W<BLE_DEEPSLCNTL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_DEEPSLCNTL_REG_SPEC>;
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
impl From<crate::W<BLE_DEEPSLCNTL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_DEEPSLCNTL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTWKUPDSB` reader - External Wake-Up disable 0: RW-BLE Core can be woken by external wake-up 1: RW-BLE Core cannot be woken up by external wake-up"]
pub struct EXTWKUPDSB_R(crate::FieldReader<bool, bool>);
impl EXTWKUPDSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTWKUPDSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTWKUPDSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTWKUPDSB` writer - External Wake-Up disable 0: RW-BLE Core can be woken by external wake-up 1: RW-BLE Core cannot be woken up by external wake-up"]
pub struct EXTWKUPDSB_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTWKUPDSB_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
#[doc = "Field `DEEP_SLEEP_STAT` reader - Indicator of current Deep Sleep clock mux status: 0: RW-BLE Core is not yet in Deep Sleep Mode 1: RW-BLE Core is in Deep Sleep Mode (only low_power_clk is running)"]
pub struct DEEP_SLEEP_STAT_R(crate::FieldReader<bool, bool>);
impl DEEP_SLEEP_STAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEEP_SLEEP_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEEP_SLEEP_STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFT_WAKEUP_REQ` reader - Wake Up Request from BLE Software. Applies when system is in Deep Sleep Mode. It wakes up the BLE Core when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0."]
pub struct SOFT_WAKEUP_REQ_R(crate::FieldReader<bool, bool>);
impl SOFT_WAKEUP_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOFT_WAKEUP_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFT_WAKEUP_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFT_WAKEUP_REQ` writer - Wake Up Request from BLE Software. Applies when system is in Deep Sleep Mode. It wakes up the BLE Core when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0."]
pub struct SOFT_WAKEUP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_WAKEUP_REQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `DEEP_SLEEP_CORR_EN` writer - 625us base time reference integer and fractional part correction. Applies when system has been woken-up from Deep Sleep Mode. It enables Fine Counter and Base Time counter when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0."]
pub struct DEEP_SLEEP_CORR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEP_SLEEP_CORR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `DEEP_SLEEP_ON` writer - 0: BLE Core in normal active mode 1: Request RW-BLE Core to switch in deep sleep mode. This bit is reset on DEEP_SLEEP_STAT falling edge."]
pub struct DEEP_SLEEP_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEP_SLEEP_ON_W<'a> {
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
#[doc = "Field `DEEP_SLEEP_IRQ_EN` reader - Always set to \"3\" when DEEP_SLEEP_ON is set to \"1\". It controls the generation of BLE_WAKEUP_LP_IRQ."]
pub struct DEEP_SLEEP_IRQ_EN_R(crate::FieldReader<u8, u8>);
impl DEEP_SLEEP_IRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEEP_SLEEP_IRQ_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEEP_SLEEP_IRQ_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEEP_SLEEP_IRQ_EN` writer - Always set to \"3\" when DEEP_SLEEP_ON is set to \"1\". It controls the generation of BLE_WAKEUP_LP_IRQ."]
pub struct DEEP_SLEEP_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEP_SLEEP_IRQ_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - External Wake-Up disable 0: RW-BLE Core can be woken by external wake-up 1: RW-BLE Core cannot be woken up by external wake-up"]
    #[inline(always)]
    pub fn extwkupdsb(&self) -> EXTWKUPDSB_R {
        EXTWKUPDSB_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicator of current Deep Sleep clock mux status: 0: RW-BLE Core is not yet in Deep Sleep Mode 1: RW-BLE Core is in Deep Sleep Mode (only low_power_clk is running)"]
    #[inline(always)]
    pub fn deep_sleep_stat(&self) -> DEEP_SLEEP_STAT_R {
        DEEP_SLEEP_STAT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake Up Request from BLE Software. Applies when system is in Deep Sleep Mode. It wakes up the BLE Core when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0."]
    #[inline(always)]
    pub fn soft_wakeup_req(&self) -> SOFT_WAKEUP_REQ_R {
        SOFT_WAKEUP_REQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 0:1 - Always set to \"3\" when DEEP_SLEEP_ON is set to \"1\". It controls the generation of BLE_WAKEUP_LP_IRQ."]
    #[inline(always)]
    pub fn deep_sleep_irq_en(&self) -> DEEP_SLEEP_IRQ_EN_R {
        DEEP_SLEEP_IRQ_EN_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - External Wake-Up disable 0: RW-BLE Core can be woken by external wake-up 1: RW-BLE Core cannot be woken up by external wake-up"]
    #[inline(always)]
    pub fn extwkupdsb(&mut self) -> EXTWKUPDSB_W {
        EXTWKUPDSB_W { w: self }
    }
    #[doc = "Bit 4 - Wake Up Request from BLE Software. Applies when system is in Deep Sleep Mode. It wakes up the BLE Core when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0."]
    #[inline(always)]
    pub fn soft_wakeup_req(&mut self) -> SOFT_WAKEUP_REQ_W {
        SOFT_WAKEUP_REQ_W { w: self }
    }
    #[doc = "Bit 3 - 625us base time reference integer and fractional part correction. Applies when system has been woken-up from Deep Sleep Mode. It enables Fine Counter and Base Time counter when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0."]
    #[inline(always)]
    pub fn deep_sleep_corr_en(&mut self) -> DEEP_SLEEP_CORR_EN_W {
        DEEP_SLEEP_CORR_EN_W { w: self }
    }
    #[doc = "Bit 2 - 0: BLE Core in normal active mode 1: Request RW-BLE Core to switch in deep sleep mode. This bit is reset on DEEP_SLEEP_STAT falling edge."]
    #[inline(always)]
    pub fn deep_sleep_on(&mut self) -> DEEP_SLEEP_ON_W {
        DEEP_SLEEP_ON_W { w: self }
    }
    #[doc = "Bits 0:1 - Always set to \"3\" when DEEP_SLEEP_ON is set to \"1\". It controls the generation of BLE_WAKEUP_LP_IRQ."]
    #[inline(always)]
    pub fn deep_sleep_irq_en(&mut self) -> DEEP_SLEEP_IRQ_EN_W {
        DEEP_SLEEP_IRQ_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep-Sleep control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_deepslcntl_reg](index.html) module"]
pub struct BLE_DEEPSLCNTL_REG_SPEC;
impl crate::RegisterSpec for BLE_DEEPSLCNTL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_deepslcntl_reg::R](R) reader structure"]
impl crate::Readable for BLE_DEEPSLCNTL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_deepslcntl_reg::W](W) writer structure"]
impl crate::Writable for BLE_DEEPSLCNTL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_DEEPSLCNTL_REG to value 0"]
impl crate::Resettable for BLE_DEEPSLCNTL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
