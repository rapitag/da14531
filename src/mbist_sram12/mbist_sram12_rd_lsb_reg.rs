#[doc = "Register `MBIST_SRAM12_RD_LSB_REG` reader"]
pub struct R(crate::R<MBIST_SRAM12_RD_LSB_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MBIST_SRAM12_RD_LSB_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MBIST_SRAM12_RD_LSB_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MBIST_SRAM12_RD_LSB_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MBIST_SRAM12_RD_LSB_REG` writer"]
pub struct W(crate::W<MBIST_SRAM12_RD_LSB_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MBIST_SRAM12_RD_LSB_REG_SPEC>;
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
impl From<crate::W<MBIST_SRAM12_RD_LSB_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MBIST_SRAM12_RD_LSB_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MBIST_LSB_DATA` reader - Returns the actual LSB read data in case of a mismatch."]
pub struct MBIST_LSB_DATA_R(crate::FieldReader<bool, bool>);
impl MBIST_LSB_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MBIST_LSB_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MBIST_LSB_DATA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Returns the actual LSB read data in case of a mismatch."]
    #[inline(always)]
    pub fn mbist_lsb_data(&self) -> MBIST_LSB_DATA_R {
        MBIST_LSB_DATA_R::new((self.bits & 1) != 0)
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbist_sram12_rd_lsb_reg](index.html) module"]
pub struct MBIST_SRAM12_RD_LSB_REG_SPEC;
impl crate::RegisterSpec for MBIST_SRAM12_RD_LSB_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mbist_sram12_rd_lsb_reg::R](R) reader structure"]
impl crate::Readable for MBIST_SRAM12_RD_LSB_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mbist_sram12_rd_lsb_reg::W](W) writer structure"]
impl crate::Writable for MBIST_SRAM12_RD_LSB_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MBIST_SRAM12_RD_LSB_REG to value 0"]
impl crate::Resettable for MBIST_SRAM12_RD_LSB_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
