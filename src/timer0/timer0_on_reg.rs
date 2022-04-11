#[doc = "Register `TIMER0_ON_REG` reader"]
pub struct R(crate::R<TIMER0_ON_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER0_ON_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER0_ON_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER0_ON_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER0_ON_REG` writer"]
pub struct W(crate::W<TIMER0_ON_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER0_ON_REG_SPEC>;
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
impl From<crate::W<TIMER0_ON_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER0_ON_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM0_ON` reader - Timer0 On reload value: If read the actual ON-counter value is returned"]
pub struct TIM0_ON_R(crate::FieldReader<u16, u16>);
impl TIM0_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TIM0_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM0_ON_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM0_ON` writer - Timer0 On reload value: If read the actual ON-counter value is returned"]
pub struct TIM0_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM0_ON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer0 On reload value: If read the actual ON-counter value is returned"]
    #[inline(always)]
    pub fn tim0_on(&self) -> TIM0_ON_R {
        TIM0_ON_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer0 On reload value: If read the actual ON-counter value is returned"]
    #[inline(always)]
    pub fn tim0_on(&mut self) -> TIM0_ON_W {
        TIM0_ON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer0 on control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0_on_reg](index.html) module"]
pub struct TIMER0_ON_REG_SPEC;
impl crate::RegisterSpec for TIMER0_ON_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [timer0_on_reg::R](R) reader structure"]
impl crate::Readable for TIMER0_ON_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer0_on_reg::W](W) writer structure"]
impl crate::Writable for TIMER0_ON_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER0_ON_REG to value 0"]
impl crate::Resettable for TIMER0_ON_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
