#[doc = "Register `RF_CNTRL_TIMER_24_REG` reader"]
pub struct R(crate::R<RF_CNTRL_TIMER_24_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_CNTRL_TIMER_24_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_CNTRL_TIMER_24_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_CNTRL_TIMER_24_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_CNTRL_TIMER_24_REG` writer"]
pub struct W(crate::W<RF_CNTRL_TIMER_24_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_CNTRL_TIMER_24_REG_SPEC>;
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
impl From<crate::W<RF_CNTRL_TIMER_24_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_CNTRL_TIMER_24_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET_OFFSET` reader - "]
pub struct RESET_OFFSET_R(crate::FieldReader<u8, u8>);
impl RESET_OFFSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RESET_OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_OFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_OFFSET` writer - "]
pub struct RESET_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SET_OFFSET` reader - "]
pub struct SET_OFFSET_R(crate::FieldReader<u8, u8>);
impl SET_OFFSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SET_OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SET_OFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SET_OFFSET` writer - "]
pub struct SET_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reset_offset(&self) -> RESET_OFFSET_R {
        RESET_OFFSET_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn set_offset(&self) -> SET_OFFSET_R {
        SET_OFFSET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reset_offset(&mut self) -> RESET_OFFSET_W {
        RESET_OFFSET_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn set_offset(&mut self) -> SET_OFFSET_W {
        SET_OFFSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_cntrl_timer_24_reg](index.html) module"]
pub struct RF_CNTRL_TIMER_24_REG_SPEC;
impl crate::RegisterSpec for RF_CNTRL_TIMER_24_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_cntrl_timer_24_reg::R](R) reader structure"]
impl crate::Readable for RF_CNTRL_TIMER_24_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_cntrl_timer_24_reg::W](W) writer structure"]
impl crate::Writable for RF_CNTRL_TIMER_24_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_CNTRL_TIMER_24_REG to value 0"]
impl crate::Resettable for RF_CNTRL_TIMER_24_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
