#[doc = "Register `ADPLL_KDTC_RD_REG` reader"]
pub struct R(crate::R<ADPLL_KDTC_RD_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_KDTC_RD_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_KDTC_RD_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_KDTC_RD_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_KDTC_RD_REG` writer"]
pub struct W(crate::W<ADPLL_KDTC_RD_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_KDTC_RD_REG_SPEC>;
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
impl From<crate::W<ADPLL_KDTC_RD_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_KDTC_RD_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL_KDTCCAL` reader - "]
pub struct CAL_KDTCCAL_R(crate::FieldReader<bool, bool>);
impl CAL_KDTCCAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAL_KDTCCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL_KDTCCAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDTC_ALPHA_COMP` reader - "]
pub struct KDTC_ALPHA_COMP_R(crate::FieldReader<u16, u16>);
impl KDTC_ALPHA_COMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        KDTC_ALPHA_COMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDTC_ALPHA_COMP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDTCCN` reader - "]
pub struct KDTCCN_R(crate::FieldReader<u8, u8>);
impl KDTCCN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KDTCCN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDTCCN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDTC_OUT` reader - "]
pub struct KDTC_OUT_R(crate::FieldReader<u16, u16>);
impl KDTC_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        KDTC_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDTC_OUT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cal_kdtccal(&self) -> CAL_KDTCCAL_R {
        CAL_KDTCCAL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn kdtc_alpha_comp(&self) -> KDTC_ALPHA_COMP_R {
        KDTC_ALPHA_COMP_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn kdtccn(&self) -> KDTCCN_R {
        KDTCCN_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn kdtc_out(&self) -> KDTC_OUT_R {
        KDTC_OUT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_kdtc_rd_reg](index.html) module"]
pub struct ADPLL_KDTC_RD_REG_SPEC;
impl crate::RegisterSpec for ADPLL_KDTC_RD_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_kdtc_rd_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_KDTC_RD_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_kdtc_rd_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_KDTC_RD_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_KDTC_RD_REG to value 0"]
impl crate::Resettable for ADPLL_KDTC_RD_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
