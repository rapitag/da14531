#[doc = "Register `ADPLL_ANATST_CTRL_REG` reader"]
pub struct R(crate::R<ADPLL_ANATST_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_ANATST_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_ANATST_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_ANATST_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_ANATST_CTRL_REG` writer"]
pub struct W(crate::W<ADPLL_ANATST_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_ANATST_CTRL_REG_SPEC>;
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
impl From<crate::W<ADPLL_ANATST_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_ANATST_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANATSTSPARE` reader - "]
pub struct ANATSTSPARE_R(crate::FieldReader<u16, u16>);
impl ANATSTSPARE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ANATSTSPARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANATSTSPARE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANATSTSPARE` writer - "]
pub struct ANATSTSPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> ANATSTSPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `ANATSTEN` reader - "]
pub struct ANATSTEN_R(crate::FieldReader<u16, u16>);
impl ANATSTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ANATSTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANATSTEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANATSTEN` writer - "]
pub struct ANATSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ANATSTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn anatstspare(&self) -> ANATSTSPARE_R {
        ANATSTSPARE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn anatsten(&self) -> ANATSTEN_R {
        ANATSTEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn anatstspare(&mut self) -> ANATSTSPARE_W {
        ANATSTSPARE_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn anatsten(&mut self) -> ANATSTEN_W {
        ANATSTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_anatst_ctrl_reg](index.html) module"]
pub struct ADPLL_ANATST_CTRL_REG_SPEC;
impl crate::RegisterSpec for ADPLL_ANATST_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_anatst_ctrl_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_ANATST_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_anatst_ctrl_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_ANATST_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_ANATST_CTRL_REG to value 0"]
impl crate::Resettable for ADPLL_ANATST_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
