#[doc = "Register `WKUP_COUNTER_REG` reader"]
pub struct R(crate::R<WKUP_COUNTER_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKUP_COUNTER_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKUP_COUNTER_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKUP_COUNTER_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKUP_COUNTER_REG` writer"]
pub struct W(crate::W<WKUP_COUNTER_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKUP_COUNTER_REG_SPEC>;
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
impl From<crate::W<WKUP_COUNTER_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKUP_COUNTER_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENT2_VALUE` reader - This value represents the number of events that have been counted so far. It will be reset by resetting the interrupt."]
pub struct EVENT2_VALUE_R(crate::FieldReader<u8, u8>);
impl EVENT2_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EVENT2_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENT2_VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVENT_VALUE` reader - This value represents the number of events that have been counted so far. It will be reset by resetting the interrupt."]
pub struct EVENT_VALUE_R(crate::FieldReader<u8, u8>);
impl EVENT_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EVENT_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENT_VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 8:15 - This value represents the number of events that have been counted so far. It will be reset by resetting the interrupt."]
    #[inline(always)]
    pub fn event2_value(&self) -> EVENT2_VALUE_R {
        EVENT2_VALUE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - This value represents the number of events that have been counted so far. It will be reset by resetting the interrupt."]
    #[inline(always)]
    pub fn event_value(&self) -> EVENT_VALUE_R {
        EVENT_VALUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Actual number of events of the wakeup counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkup_counter_reg](index.html) module"]
pub struct WKUP_COUNTER_REG_SPEC;
impl crate::RegisterSpec for WKUP_COUNTER_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wkup_counter_reg::R](R) reader structure"]
impl crate::Readable for WKUP_COUNTER_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wkup_counter_reg::W](W) writer structure"]
impl crate::Writable for WKUP_COUNTER_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WKUP_COUNTER_REG to value 0"]
impl crate::Resettable for WKUP_COUNTER_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
