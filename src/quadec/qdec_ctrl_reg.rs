#[doc = "Register `QDEC_CTRL_REG` reader"]
pub struct R(crate::R<QDEC_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QDEC_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QDEC_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QDEC_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QDEC_CTRL_REG` writer"]
pub struct W(crate::W<QDEC_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QDEC_CTRL_REG_SPEC>;
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
impl From<crate::W<QDEC_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QDEC_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QDEC_IRQ_THRES` reader - Defines the number of events on either counter (X or Y or Z) that need to be reached before an interrupt is generated. Events are equal to QDEC_IRQ_THRES+1."]
pub struct QDEC_IRQ_THRES_R(crate::FieldReader<u8, u8>);
impl QDEC_IRQ_THRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        QDEC_IRQ_THRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QDEC_IRQ_THRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QDEC_IRQ_THRES` writer - Defines the number of events on either counter (X or Y or Z) that need to be reached before an interrupt is generated. Events are equal to QDEC_IRQ_THRES+1."]
pub struct QDEC_IRQ_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> QDEC_IRQ_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 3)) | ((value as u16 & 0xff) << 3);
        self.w
    }
}
#[doc = "Field `QDEC_IRQ_STATUS` reader - 1 = Interrupt is occured. 0 = No interrupt pending Write 1 will clear the pending interrupt"]
pub struct QDEC_IRQ_STATUS_R(crate::FieldReader<bool, bool>);
impl QDEC_IRQ_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QDEC_IRQ_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QDEC_IRQ_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QDEC_IRQ_STATUS` writer - 1 = Interrupt is occured. 0 = No interrupt pending Write 1 will clear the pending interrupt"]
pub struct QDEC_IRQ_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> QDEC_IRQ_STATUS_W<'a> {
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
#[doc = "Field `QDEC_EVENT_CNT_CLR` reader - Writing 1 QDEC_EVENT_CNT_REG is cleared"]
pub struct QDEC_EVENT_CNT_CLR_R(crate::FieldReader<bool, bool>);
impl QDEC_EVENT_CNT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QDEC_EVENT_CNT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QDEC_EVENT_CNT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QDEC_EVENT_CNT_CLR` writer - Writing 1 QDEC_EVENT_CNT_REG is cleared"]
pub struct QDEC_EVENT_CNT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> QDEC_EVENT_CNT_CLR_W<'a> {
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
#[doc = "Field `QDEC_IRQ_ENABLE` reader - 0 = interrupt is masked 1 = interrupt is enabled"]
pub struct QDEC_IRQ_ENABLE_R(crate::FieldReader<bool, bool>);
impl QDEC_IRQ_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QDEC_IRQ_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QDEC_IRQ_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QDEC_IRQ_ENABLE` writer - 0 = interrupt is masked 1 = interrupt is enabled"]
pub struct QDEC_IRQ_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> QDEC_IRQ_ENABLE_W<'a> {
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
    #[doc = "Bits 3:10 - Defines the number of events on either counter (X or Y or Z) that need to be reached before an interrupt is generated. Events are equal to QDEC_IRQ_THRES+1."]
    #[inline(always)]
    pub fn qdec_irq_thres(&self) -> QDEC_IRQ_THRES_R {
        QDEC_IRQ_THRES_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 2 - 1 = Interrupt is occured. 0 = No interrupt pending Write 1 will clear the pending interrupt"]
    #[inline(always)]
    pub fn qdec_irq_status(&self) -> QDEC_IRQ_STATUS_R {
        QDEC_IRQ_STATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Writing 1 QDEC_EVENT_CNT_REG is cleared"]
    #[inline(always)]
    pub fn qdec_event_cnt_clr(&self) -> QDEC_EVENT_CNT_CLR_R {
        QDEC_EVENT_CNT_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - 0 = interrupt is masked 1 = interrupt is enabled"]
    #[inline(always)]
    pub fn qdec_irq_enable(&self) -> QDEC_IRQ_ENABLE_R {
        QDEC_IRQ_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 3:10 - Defines the number of events on either counter (X or Y or Z) that need to be reached before an interrupt is generated. Events are equal to QDEC_IRQ_THRES+1."]
    #[inline(always)]
    pub fn qdec_irq_thres(&mut self) -> QDEC_IRQ_THRES_W {
        QDEC_IRQ_THRES_W { w: self }
    }
    #[doc = "Bit 2 - 1 = Interrupt is occured. 0 = No interrupt pending Write 1 will clear the pending interrupt"]
    #[inline(always)]
    pub fn qdec_irq_status(&mut self) -> QDEC_IRQ_STATUS_W {
        QDEC_IRQ_STATUS_W { w: self }
    }
    #[doc = "Bit 1 - Writing 1 QDEC_EVENT_CNT_REG is cleared"]
    #[inline(always)]
    pub fn qdec_event_cnt_clr(&mut self) -> QDEC_EVENT_CNT_CLR_W {
        QDEC_EVENT_CNT_CLR_W { w: self }
    }
    #[doc = "Bit 0 - 0 = interrupt is masked 1 = interrupt is enabled"]
    #[inline(always)]
    pub fn qdec_irq_enable(&mut self) -> QDEC_IRQ_ENABLE_W {
        QDEC_IRQ_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Quad Decoder control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qdec_ctrl_reg](index.html) module"]
pub struct QDEC_CTRL_REG_SPEC;
impl crate::RegisterSpec for QDEC_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [qdec_ctrl_reg::R](R) reader structure"]
impl crate::Readable for QDEC_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qdec_ctrl_reg::W](W) writer structure"]
impl crate::Writable for QDEC_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QDEC_CTRL_REG to value 0x11"]
impl crate::Resettable for QDEC_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x11
    }
}
