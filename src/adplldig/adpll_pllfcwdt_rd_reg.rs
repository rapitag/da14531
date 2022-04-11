#[doc = "Register `ADPLL_PLLFCWDT_RD_REG` reader"]
pub struct R(crate::R<ADPLL_PLLFCWDT_RD_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_PLLFCWDT_RD_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_PLLFCWDT_RD_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_PLLFCWDT_RD_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_PLLFCWDT_RD_REG` writer"]
pub struct W(crate::W<ADPLL_PLLFCWDT_RD_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_PLLFCWDT_RD_REG_SPEC>;
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
impl From<crate::W<ADPLL_PLLFCWDT_RD_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_PLLFCWDT_RD_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLFCWDT` reader - "]
pub struct PLLFCWDT_R(crate::FieldReader<u32, u32>);
impl PLLFCWDT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PLLFCWDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLFCWDT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:22"]
    #[inline(always)]
    pub fn pllfcwdt(&self) -> PLLFCWDT_R {
        PLLFCWDT_R::new((self.bits & 0x007f_ffff) as u32)
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_pllfcwdt_rd_reg](index.html) module"]
pub struct ADPLL_PLLFCWDT_RD_REG_SPEC;
impl crate::RegisterSpec for ADPLL_PLLFCWDT_RD_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_pllfcwdt_rd_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_PLLFCWDT_RD_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_pllfcwdt_rd_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_PLLFCWDT_RD_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_PLLFCWDT_RD_REG to value 0x0025_8800"]
impl crate::Resettable for ADPLL_PLLFCWDT_RD_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0025_8800
    }
}
