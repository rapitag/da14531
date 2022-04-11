#[doc = "Register `POR_TIMER_REG` reader"]
pub struct R(crate::R<POR_TIMER_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POR_TIMER_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POR_TIMER_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POR_TIMER_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POR_TIMER_REG` writer"]
pub struct W(crate::W<POR_TIMER_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POR_TIMER_REG_SPEC>;
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
impl From<crate::W<POR_TIMER_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POR_TIMER_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POR_TIME` reader - Time for the POReset to happen. Formula: Time = POR_TIME x 4096 x RC32k clock period Default value: ~3 seconds When set to 0x00, the POR TIMER is disabled."]
pub struct POR_TIME_R(crate::FieldReader<u8, u8>);
impl POR_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        POR_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POR_TIME` writer - Time for the POReset to happen. Formula: Time = POR_TIME x 4096 x RC32k clock period Default value: ~3 seconds When set to 0x00, the POR TIMER is disabled."]
pub struct POR_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u16 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Time for the POReset to happen. Formula: Time = POR_TIME x 4096 x RC32k clock period Default value: ~3 seconds When set to 0x00, the POR TIMER is disabled."]
    #[inline(always)]
    pub fn por_time(&self) -> POR_TIME_R {
        POR_TIME_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Time for the POReset to happen. Formula: Time = POR_TIME x 4096 x RC32k clock period Default value: ~3 seconds When set to 0x00, the POR TIMER is disabled."]
    #[inline(always)]
    pub fn por_time(&mut self) -> POR_TIME_W {
        POR_TIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time for POR to happen\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [por_timer_reg](index.html) module"]
pub struct POR_TIMER_REG_SPEC;
impl crate::RegisterSpec for POR_TIMER_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [por_timer_reg::R](R) reader structure"]
impl crate::Readable for POR_TIMER_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [por_timer_reg::W](W) writer structure"]
impl crate::Writable for POR_TIMER_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POR_TIMER_REG to value 0x18"]
impl crate::Resettable for POR_TIMER_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x18
    }
}
