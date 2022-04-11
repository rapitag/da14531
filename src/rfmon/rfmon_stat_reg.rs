#[doc = "Register `RFMON_STAT_REG` reader"]
pub struct R(crate::R<RFMON_STAT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFMON_STAT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFMON_STAT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFMON_STAT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFMON_STAT_REG` writer"]
pub struct W(crate::W<RFMON_STAT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFMON_STAT_REG_SPEC>;
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
impl From<crate::W<RFMON_STAT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFMON_STAT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFMON_OFLOW_STK` reader - "]
pub struct RFMON_OFLOW_STK_R(crate::FieldReader<bool, bool>);
impl RFMON_OFLOW_STK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFMON_OFLOW_STK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFMON_OFLOW_STK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFMON_OFLOW_STK` writer - "]
pub struct RFMON_OFLOW_STK_W<'a> {
    w: &'a mut W,
}
impl<'a> RFMON_OFLOW_STK_W<'a> {
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
#[doc = "Field `RFMON_ACTIVE` reader - "]
pub struct RFMON_ACTIVE_R(crate::FieldReader<bool, bool>);
impl RFMON_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFMON_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFMON_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rfmon_oflow_stk(&self) -> RFMON_OFLOW_STK_R {
        RFMON_OFLOW_STK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rfmon_active(&self) -> RFMON_ACTIVE_R {
        RFMON_ACTIVE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rfmon_oflow_stk(&mut self) -> RFMON_OFLOW_STK_W {
        RFMON_OFLOW_STK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfmon_stat_reg](index.html) module"]
pub struct RFMON_STAT_REG_SPEC;
impl crate::RegisterSpec for RFMON_STAT_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rfmon_stat_reg::R](R) reader structure"]
impl crate::Readable for RFMON_STAT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfmon_stat_reg::W](W) writer structure"]
impl crate::Writable for RFMON_STAT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFMON_STAT_REG to value 0"]
impl crate::Resettable for RFMON_STAT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
