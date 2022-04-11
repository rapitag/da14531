#[doc = "Register `ADPLL_FREQMEAS_RD_REG` reader"]
pub struct R(crate::R<ADPLL_FREQMEAS_RD_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_FREQMEAS_RD_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_FREQMEAS_RD_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_FREQMEAS_RD_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_FREQMEAS_RD_REG` writer"]
pub struct W(crate::W<ADPLL_FREQMEAS_RD_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_FREQMEAS_RD_REG_SPEC>;
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
impl From<crate::W<ADPLL_FREQMEAS_RD_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_FREQMEAS_RD_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEASDONE_OUT` reader - "]
pub struct MEASDONE_OUT_R(crate::FieldReader<bool, bool>);
impl MEASDONE_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEASDONE_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEASDONE_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QUALMONDET` reader - "]
pub struct QUALMONDET_R(crate::FieldReader<bool, bool>);
impl QUALMONDET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QUALMONDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QUALMONDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDCBUB` reader - "]
pub struct TDCBUB_R(crate::FieldReader<bool, bool>);
impl TDCBUB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TDCBUB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDCBUB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHVSA0` reader - "]
pub struct PHVSA0_R(crate::FieldReader<bool, bool>);
impl PHVSA0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PHVSA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHVSA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHVSA1` reader - "]
pub struct PHVSA1_R(crate::FieldReader<bool, bool>);
impl PHVSA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PHVSA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHVSA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQDIFF` reader - "]
pub struct FREQDIFF_R(crate::FieldReader<u32, u32>);
impl FREQDIFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        FREQDIFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREQDIFF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn measdone_out(&self) -> MEASDONE_OUT_R {
        MEASDONE_OUT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn qualmondet(&self) -> QUALMONDET_R {
        QUALMONDET_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tdcbub(&self) -> TDCBUB_R {
        TDCBUB_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn phvsa0(&self) -> PHVSA0_R {
        PHVSA0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn phvsa1(&self) -> PHVSA1_R {
        PHVSA1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 0:22"]
    #[inline(always)]
    pub fn freqdiff(&self) -> FREQDIFF_R {
        FREQDIFF_R::new((self.bits & 0x007f_ffff) as u32)
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_freqmeas_rd_reg](index.html) module"]
pub struct ADPLL_FREQMEAS_RD_REG_SPEC;
impl crate::RegisterSpec for ADPLL_FREQMEAS_RD_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_freqmeas_rd_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_FREQMEAS_RD_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_freqmeas_rd_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_FREQMEAS_RD_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_FREQMEAS_RD_REG to value 0x2100_0000"]
impl crate::Resettable for ADPLL_FREQMEAS_RD_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2100_0000
    }
}
