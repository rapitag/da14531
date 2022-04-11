#[doc = "Register `WKUP_IRQ_STATUS_REG` reader"]
pub struct R(crate::R<WKUP_IRQ_STATUS_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKUP_IRQ_STATUS_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKUP_IRQ_STATUS_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKUP_IRQ_STATUS_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKUP_IRQ_STATUS_REG` writer"]
pub struct W(crate::W<WKUP_IRQ_STATUS_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKUP_IRQ_STATUS_REG_SPEC>;
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
impl From<crate::W<WKUP_IRQ_STATUS_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKUP_IRQ_STATUS_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUP2_CNTR_RST` writer - writing 1 will reset the event2 counter"]
pub struct WKUP2_CNTR_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUP2_CNTR_RST_W<'a> {
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
#[doc = "Field `WKUP_CNTR_RST` writer - writing 1 will reset the event counter"]
pub struct WKUP_CNTR_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUP_CNTR_RST_W<'a> {
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
#[doc = "Field `WKUP2_IRQ_STATUS` reader - Gives 1 when there is a wkup2 pending IRQ. Writing 1 will reset the interrupt."]
pub struct WKUP2_IRQ_STATUS_R(crate::FieldReader<bool, bool>);
impl WKUP2_IRQ_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUP2_IRQ_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUP2_IRQ_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUP2_IRQ_STATUS` writer - Gives 1 when there is a wkup2 pending IRQ. Writing 1 will reset the interrupt."]
pub struct WKUP2_IRQ_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUP2_IRQ_STATUS_W<'a> {
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
#[doc = "Field `WKUP_IRQ_STATUS` reader - Gives 1 when there is a wkup pending IRQ. Writing 1 will reset the interrupt."]
pub struct WKUP_IRQ_STATUS_R(crate::FieldReader<bool, bool>);
impl WKUP_IRQ_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUP_IRQ_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUP_IRQ_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUP_IRQ_STATUS` writer - Gives 1 when there is a wkup pending IRQ. Writing 1 will reset the interrupt."]
pub struct WKUP_IRQ_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUP_IRQ_STATUS_W<'a> {
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
    #[doc = "Bit 1 - Gives 1 when there is a wkup2 pending IRQ. Writing 1 will reset the interrupt."]
    #[inline(always)]
    pub fn wkup2_irq_status(&self) -> WKUP2_IRQ_STATUS_R {
        WKUP2_IRQ_STATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Gives 1 when there is a wkup pending IRQ. Writing 1 will reset the interrupt."]
    #[inline(always)]
    pub fn wkup_irq_status(&self) -> WKUP_IRQ_STATUS_R {
        WKUP_IRQ_STATUS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - writing 1 will reset the event2 counter"]
    #[inline(always)]
    pub fn wkup2_cntr_rst(&mut self) -> WKUP2_CNTR_RST_W {
        WKUP2_CNTR_RST_W { w: self }
    }
    #[doc = "Bit 2 - writing 1 will reset the event counter"]
    #[inline(always)]
    pub fn wkup_cntr_rst(&mut self) -> WKUP_CNTR_RST_W {
        WKUP_CNTR_RST_W { w: self }
    }
    #[doc = "Bit 1 - Gives 1 when there is a wkup2 pending IRQ. Writing 1 will reset the interrupt."]
    #[inline(always)]
    pub fn wkup2_irq_status(&mut self) -> WKUP2_IRQ_STATUS_W {
        WKUP2_IRQ_STATUS_W { w: self }
    }
    #[doc = "Bit 0 - Gives 1 when there is a wkup pending IRQ. Writing 1 will reset the interrupt."]
    #[inline(always)]
    pub fn wkup_irq_status(&mut self) -> WKUP_IRQ_STATUS_W {
        WKUP_IRQ_STATUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset wakeup interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkup_irq_status_reg](index.html) module"]
pub struct WKUP_IRQ_STATUS_REG_SPEC;
impl crate::RegisterSpec for WKUP_IRQ_STATUS_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wkup_irq_status_reg::R](R) reader structure"]
impl crate::Readable for WKUP_IRQ_STATUS_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wkup_irq_status_reg::W](W) writer structure"]
impl crate::Writable for WKUP_IRQ_STATUS_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WKUP_IRQ_STATUS_REG to value 0"]
impl crate::Resettable for WKUP_IRQ_STATUS_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
