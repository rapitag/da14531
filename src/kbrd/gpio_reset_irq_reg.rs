#[doc = "Register `GPIO_RESET_IRQ_REG` reader"]
pub struct R(crate::R<GPIO_RESET_IRQ_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_RESET_IRQ_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_RESET_IRQ_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_RESET_IRQ_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_RESET_IRQ_REG` writer"]
pub struct W(crate::W<GPIO_RESET_IRQ_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_RESET_IRQ_REG_SPEC>;
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
impl From<crate::W<GPIO_RESET_IRQ_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_RESET_IRQ_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET_KBRD_IRQ` writer - writing a 1 to this bit will reset the KBRD IRQ. Reading returns 0."]
pub struct RESET_KBRD_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_KBRD_IRQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u16 & 1) << 5);
        self.w
    }
}
#[doc = "Field `RESET_GPIO4_IRQ` writer - writing a 1 to this bit will reset the GPIO4 IRQ. Reading returns 0."]
pub struct RESET_GPIO4_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_GPIO4_IRQ_W<'a> {
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
#[doc = "Field `RESET_GPIO3_IRQ` writer - writing a 1 to this bit will reset the GPIO3 IRQ. Reading returns 0."]
pub struct RESET_GPIO3_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_GPIO3_IRQ_W<'a> {
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
#[doc = "Field `RESET_GPIO2_IRQ` writer - writing a 1 to this bit will reset the GPIO2 IRQ. Reading returns 0."]
pub struct RESET_GPIO2_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_GPIO2_IRQ_W<'a> {
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
#[doc = "Field `RESET_GPIO1_IRQ` writer - writing a 1 to this bit will reset the GPIO1 IRQ. Reading returns 0."]
pub struct RESET_GPIO1_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_GPIO1_IRQ_W<'a> {
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
#[doc = "Field `RESET_GPIO0_IRQ` writer - writing a 1 to this bit will reset the GPIO0 IRQ. Reading returns 0."]
pub struct RESET_GPIO0_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_GPIO0_IRQ_W<'a> {
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
impl W {
    #[doc = "Bit 5 - writing a 1 to this bit will reset the KBRD IRQ. Reading returns 0."]
    #[inline(always)]
    pub fn reset_kbrd_irq(&mut self) -> RESET_KBRD_IRQ_W {
        RESET_KBRD_IRQ_W { w: self }
    }
    #[doc = "Bit 4 - writing a 1 to this bit will reset the GPIO4 IRQ. Reading returns 0."]
    #[inline(always)]
    pub fn reset_gpio4_irq(&mut self) -> RESET_GPIO4_IRQ_W {
        RESET_GPIO4_IRQ_W { w: self }
    }
    #[doc = "Bit 3 - writing a 1 to this bit will reset the GPIO3 IRQ. Reading returns 0."]
    #[inline(always)]
    pub fn reset_gpio3_irq(&mut self) -> RESET_GPIO3_IRQ_W {
        RESET_GPIO3_IRQ_W { w: self }
    }
    #[doc = "Bit 2 - writing a 1 to this bit will reset the GPIO2 IRQ. Reading returns 0."]
    #[inline(always)]
    pub fn reset_gpio2_irq(&mut self) -> RESET_GPIO2_IRQ_W {
        RESET_GPIO2_IRQ_W { w: self }
    }
    #[doc = "Bit 1 - writing a 1 to this bit will reset the GPIO1 IRQ. Reading returns 0."]
    #[inline(always)]
    pub fn reset_gpio1_irq(&mut self) -> RESET_GPIO1_IRQ_W {
        RESET_GPIO1_IRQ_W { w: self }
    }
    #[doc = "Bit 0 - writing a 1 to this bit will reset the GPIO0 IRQ. Reading returns 0."]
    #[inline(always)]
    pub fn reset_gpio0_irq(&mut self) -> RESET_GPIO0_IRQ_W {
        RESET_GPIO0_IRQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO interrupt reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_reset_irq_reg](index.html) module"]
pub struct GPIO_RESET_IRQ_REG_SPEC;
impl crate::RegisterSpec for GPIO_RESET_IRQ_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gpio_reset_irq_reg::R](R) reader structure"]
impl crate::Readable for GPIO_RESET_IRQ_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_reset_irq_reg::W](W) writer structure"]
impl crate::Writable for GPIO_RESET_IRQ_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_RESET_IRQ_REG to value 0"]
impl crate::Resettable for GPIO_RESET_IRQ_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
