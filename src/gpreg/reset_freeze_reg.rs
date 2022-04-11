#[doc = "Register `RESET_FREEZE_REG` reader"]
pub struct R(crate::R<RESET_FREEZE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_FREEZE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_FREEZE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_FREEZE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET_FREEZE_REG` writer"]
pub struct W(crate::W<RESET_FREEZE_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_FREEZE_REG_SPEC>;
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
impl From<crate::W<RESET_FREEZE_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_FREEZE_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRZ_DMA` reader - If '1', the DMA continues, '0' is discarded."]
pub struct FRZ_DMA_R(crate::FieldReader<bool, bool>);
impl FRZ_DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRZ_DMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRZ_DMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRZ_DMA` writer - If '1', the DMA continues, '0' is discarded."]
pub struct FRZ_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZ_DMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u16 & 1) << 4);
        self.w
    }
}
#[doc = "Field `FRZ_WDOG` reader - If '1', the watchdog timer continues, '0' is discarded."]
pub struct FRZ_WDOG_R(crate::FieldReader<bool, bool>);
impl FRZ_WDOG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRZ_WDOG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRZ_WDOG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRZ_WDOG` writer - If '1', the watchdog timer continues, '0' is discarded."]
pub struct FRZ_WDOG_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZ_WDOG_W<'a> {
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
#[doc = "Field `FRZ_BLETIM` reader - If '1', the the BLE master clock continues, '0' is discarded."]
pub struct FRZ_BLETIM_R(crate::FieldReader<bool, bool>);
impl FRZ_BLETIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRZ_BLETIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRZ_BLETIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRZ_BLETIM` writer - If '1', the the BLE master clock continues, '0' is discarded."]
pub struct FRZ_BLETIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZ_BLETIM_W<'a> {
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
#[doc = "Field `FRZ_SWTIM` reader - If '1', the SW Timer (TIMER0) continues, '0' is discarded."]
pub struct FRZ_SWTIM_R(crate::FieldReader<bool, bool>);
impl FRZ_SWTIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRZ_SWTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRZ_SWTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRZ_SWTIM` writer - If '1', the SW Timer (TIMER0) continues, '0' is discarded."]
pub struct FRZ_SWTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZ_SWTIM_W<'a> {
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
#[doc = "Field `FRZ_WKUPTIM` reader - If '1', the Wake Up Timer continues, '0' is discarded."]
pub struct FRZ_WKUPTIM_R(crate::FieldReader<bool, bool>);
impl FRZ_WKUPTIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRZ_WKUPTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRZ_WKUPTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRZ_WKUPTIM` writer - If '1', the Wake Up Timer continues, '0' is discarded."]
pub struct FRZ_WKUPTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZ_WKUPTIM_W<'a> {
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
    #[doc = "Bit 4 - If '1', the DMA continues, '0' is discarded."]
    #[inline(always)]
    pub fn frz_dma(&self) -> FRZ_DMA_R {
        FRZ_DMA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - If '1', the watchdog timer continues, '0' is discarded."]
    #[inline(always)]
    pub fn frz_wdog(&self) -> FRZ_WDOG_R {
        FRZ_WDOG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - If '1', the the BLE master clock continues, '0' is discarded."]
    #[inline(always)]
    pub fn frz_bletim(&self) -> FRZ_BLETIM_R {
        FRZ_BLETIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - If '1', the SW Timer (TIMER0) continues, '0' is discarded."]
    #[inline(always)]
    pub fn frz_swtim(&self) -> FRZ_SWTIM_R {
        FRZ_SWTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - If '1', the Wake Up Timer continues, '0' is discarded."]
    #[inline(always)]
    pub fn frz_wkuptim(&self) -> FRZ_WKUPTIM_R {
        FRZ_WKUPTIM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - If '1', the DMA continues, '0' is discarded."]
    #[inline(always)]
    pub fn frz_dma(&mut self) -> FRZ_DMA_W {
        FRZ_DMA_W { w: self }
    }
    #[doc = "Bit 3 - If '1', the watchdog timer continues, '0' is discarded."]
    #[inline(always)]
    pub fn frz_wdog(&mut self) -> FRZ_WDOG_W {
        FRZ_WDOG_W { w: self }
    }
    #[doc = "Bit 2 - If '1', the the BLE master clock continues, '0' is discarded."]
    #[inline(always)]
    pub fn frz_bletim(&mut self) -> FRZ_BLETIM_W {
        FRZ_BLETIM_W { w: self }
    }
    #[doc = "Bit 1 - If '1', the SW Timer (TIMER0) continues, '0' is discarded."]
    #[inline(always)]
    pub fn frz_swtim(&mut self) -> FRZ_SWTIM_W {
        FRZ_SWTIM_W { w: self }
    }
    #[doc = "Bit 0 - If '1', the Wake Up Timer continues, '0' is discarded."]
    #[inline(always)]
    pub fn frz_wkuptim(&mut self) -> FRZ_WKUPTIM_W {
        FRZ_WKUPTIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls unfreezing of various timers/counters.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_freeze_reg](index.html) module"]
pub struct RESET_FREEZE_REG_SPEC;
impl crate::RegisterSpec for RESET_FREEZE_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [reset_freeze_reg::R](R) reader structure"]
impl crate::Readable for RESET_FREEZE_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset_freeze_reg::W](W) writer structure"]
impl crate::Writable for RESET_FREEZE_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESET_FREEZE_REG to value 0"]
impl crate::Resettable for RESET_FREEZE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
