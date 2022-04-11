#[doc = "Register `ADPLL_KDCO_CAL_CTRL1_REG` reader"]
pub struct R(crate::R<ADPLL_KDCO_CAL_CTRL1_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_KDCO_CAL_CTRL1_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_KDCO_CAL_CTRL1_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_KDCO_CAL_CTRL1_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_KDCO_CAL_CTRL1_REG` writer"]
pub struct W(crate::W<ADPLL_KDCO_CAL_CTRL1_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_KDCO_CAL_CTRL1_REG_SPEC>;
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
impl From<crate::W<ADPLL_KDCO_CAL_CTRL1_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_KDCO_CAL_CTRL1_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KDCOLF_IN_1M` reader - "]
pub struct KDCOLF_IN_1M_R(crate::FieldReader<u8, u8>);
impl KDCOLF_IN_1M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KDCOLF_IN_1M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDCOLF_IN_1M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDCOLF_IN_1M` writer - "]
pub struct KDCOLF_IN_1M_W<'a> {
    w: &'a mut W,
}
impl<'a> KDCOLF_IN_1M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `KDCOHFIC_1M` reader - "]
pub struct KDCOHFIC_1M_R(crate::FieldReader<u8, u8>);
impl KDCOHFIC_1M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KDCOHFIC_1M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDCOHFIC_1M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDCOHFIC_1M` writer - "]
pub struct KDCOHFIC_1M_W<'a> {
    w: &'a mut W,
}
impl<'a> KDCOHFIC_1M_W<'a> {
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
    pub fn kdcolf_in_1m(&self) -> KDCOLF_IN_1M_R {
        KDCOLF_IN_1M_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn kdcohfic_1m(&self) -> KDCOHFIC_1M_R {
        KDCOHFIC_1M_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn kdcolf_in_1m(&mut self) -> KDCOLF_IN_1M_W {
        KDCOLF_IN_1M_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn kdcohfic_1m(&mut self) -> KDCOHFIC_1M_W {
        KDCOHFIC_1M_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_kdco_cal_ctrl1_reg](index.html) module"]
pub struct ADPLL_KDCO_CAL_CTRL1_REG_SPEC;
impl crate::RegisterSpec for ADPLL_KDCO_CAL_CTRL1_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_kdco_cal_ctrl1_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_KDCO_CAL_CTRL1_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_kdco_cal_ctrl1_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_KDCO_CAL_CTRL1_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_KDCO_CAL_CTRL1_REG to value 0x9b9b"]
impl crate::Resettable for ADPLL_KDCO_CAL_CTRL1_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x9b9b
    }
}
