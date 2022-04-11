#[doc = "Register `GPIO_IRQ4_IN_SEL_REG` reader"]
pub struct R(crate::R<GPIO_IRQ4_IN_SEL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_IRQ4_IN_SEL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_IRQ4_IN_SEL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_IRQ4_IN_SEL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_IRQ4_IN_SEL_REG` writer"]
pub struct W(crate::W<GPIO_IRQ4_IN_SEL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_IRQ4_IN_SEL_REG_SPEC>;
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
impl From<crate::W<GPIO_IRQ4_IN_SEL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_IRQ4_IN_SEL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KBRD_IRQ4_SEL` reader - see KBRD_IRQ0_SEL"]
pub struct KBRD_IRQ4_SEL_R(crate::FieldReader<u8, u8>);
impl KBRD_IRQ4_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KBRD_IRQ4_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KBRD_IRQ4_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBRD_IRQ4_SEL` writer - see KBRD_IRQ0_SEL"]
pub struct KBRD_IRQ4_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> KBRD_IRQ4_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u16 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - see KBRD_IRQ0_SEL"]
    #[inline(always)]
    pub fn kbrd_irq4_sel(&self) -> KBRD_IRQ4_SEL_R {
        KBRD_IRQ4_SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - see KBRD_IRQ0_SEL"]
    #[inline(always)]
    pub fn kbrd_irq4_sel(&mut self) -> KBRD_IRQ4_SEL_W {
        KBRD_IRQ4_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO interrupt selection for GPIO_IRQ4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_irq4_in_sel_reg](index.html) module"]
pub struct GPIO_IRQ4_IN_SEL_REG_SPEC;
impl crate::RegisterSpec for GPIO_IRQ4_IN_SEL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gpio_irq4_in_sel_reg::R](R) reader structure"]
impl crate::Readable for GPIO_IRQ4_IN_SEL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_irq4_in_sel_reg::W](W) writer structure"]
impl crate::Writable for GPIO_IRQ4_IN_SEL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_IRQ4_IN_SEL_REG to value 0"]
impl crate::Resettable for GPIO_IRQ4_IN_SEL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
