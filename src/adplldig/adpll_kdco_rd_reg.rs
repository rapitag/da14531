#[doc = "Register `ADPLL_KDCO_RD_REG` reader"]
pub struct R(crate::R<ADPLL_KDCO_RD_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_KDCO_RD_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_KDCO_RD_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_KDCO_RD_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_KDCO_RD_REG` writer"]
pub struct W(crate::W<ADPLL_KDCO_RD_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_KDCO_RD_REG_SPEC>;
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
impl From<crate::W<ADPLL_KDCO_RD_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_KDCO_RD_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL_KDCOCAL` reader - "]
pub struct CAL_KDCOCAL_R(crate::FieldReader<bool, bool>);
impl CAL_KDCOCAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAL_KDCOCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL_KDCOCAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDCOCN` reader - "]
pub struct KDCOCN_R(crate::FieldReader<u8, u8>);
impl KDCOCN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KDCOCN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDCOCN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDCO_HF_OUT` reader - "]
pub struct KDCO_HF_OUT_R(crate::FieldReader<u8, u8>);
impl KDCO_HF_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KDCO_HF_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDCO_HF_OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDCO_HF_INT` reader - "]
pub struct KDCO_HF_INT_R(crate::FieldReader<u8, u8>);
impl KDCO_HF_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KDCO_HF_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDCO_HF_INT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cal_kdcocal(&self) -> CAL_KDCOCAL_R {
        CAL_KDCOCAL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn kdcocn(&self) -> KDCOCN_R {
        KDCOCN_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn kdco_hf_out(&self) -> KDCO_HF_OUT_R {
        KDCO_HF_OUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn kdco_hf_int(&self) -> KDCO_HF_INT_R {
        KDCO_HF_INT_R::new((self.bits & 0xff) as u8)
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_kdco_rd_reg](index.html) module"]
pub struct ADPLL_KDCO_RD_REG_SPEC;
impl crate::RegisterSpec for ADPLL_KDCO_RD_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_kdco_rd_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_KDCO_RD_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_kdco_rd_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_KDCO_RD_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_KDCO_RD_REG to value 0"]
impl crate::Resettable for ADPLL_KDCO_RD_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
