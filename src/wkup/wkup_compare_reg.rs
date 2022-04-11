#[doc = "Register `WKUP_COMPARE_REG` reader"]
pub struct R(crate::R<WKUP_COMPARE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKUP_COMPARE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKUP_COMPARE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKUP_COMPARE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKUP_COMPARE_REG` writer"]
pub struct W(crate::W<WKUP_COMPARE_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKUP_COMPARE_REG_SPEC>;
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
impl From<crate::W<WKUP_COMPARE_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKUP_COMPARE_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUP_COMPARE` reader - Defines the number of events -1 that have to be counted before the wakeup interrupt will be given. value 0 means one event."]
pub struct WKUP_COMPARE_R(crate::FieldReader<u8, u8>);
impl WKUP_COMPARE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WKUP_COMPARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUP_COMPARE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUP_COMPARE` writer - Defines the number of events -1 that have to be counted before the wakeup interrupt will be given. value 0 means one event."]
pub struct WKUP_COMPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUP_COMPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Defines the number of events -1 that have to be counted before the wakeup interrupt will be given. value 0 means one event."]
    #[inline(always)]
    pub fn wkup_compare(&self) -> WKUP_COMPARE_R {
        WKUP_COMPARE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Defines the number of events -1 that have to be counted before the wakeup interrupt will be given. value 0 means one event."]
    #[inline(always)]
    pub fn wkup_compare(&mut self) -> WKUP_COMPARE_W {
        WKUP_COMPARE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Number of events before wakeup interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkup_compare_reg](index.html) module"]
pub struct WKUP_COMPARE_REG_SPEC;
impl crate::RegisterSpec for WKUP_COMPARE_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wkup_compare_reg::R](R) reader structure"]
impl crate::Readable for WKUP_COMPARE_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wkup_compare_reg::W](W) writer structure"]
impl crate::Writable for WKUP_COMPARE_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WKUP_COMPARE_REG to value 0"]
impl crate::Resettable for WKUP_COMPARE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
