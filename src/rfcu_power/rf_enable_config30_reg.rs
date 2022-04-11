#[doc = "Register `RF_ENABLE_CONFIG30_REG` reader"]
pub struct R(crate::R<RF_ENABLE_CONFIG30_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_ENABLE_CONFIG30_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_ENABLE_CONFIG30_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_ENABLE_CONFIG30_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_ENABLE_CONFIG30_REG` writer"]
pub struct W(crate::W<RF_ENABLE_CONFIG30_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_ENABLE_CONFIG30_REG_SPEC>;
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
impl From<crate::W<RF_ENABLE_CONFIG30_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_ENABLE_CONFIG30_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADPLLDIG_RST_DCF_TX` reader - "]
pub struct ADPLLDIG_RST_DCF_TX_R(crate::FieldReader<u8, u8>);
impl ADPLLDIG_RST_DCF_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADPLLDIG_RST_DCF_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLLDIG_RST_DCF_TX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPLLDIG_RST_DCF_TX` writer - "]
pub struct ADPLLDIG_RST_DCF_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLLDIG_RST_DCF_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `ADPLLDIG_RST_DCF_RX` reader - "]
pub struct ADPLLDIG_RST_DCF_RX_R(crate::FieldReader<u8, u8>);
impl ADPLLDIG_RST_DCF_RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADPLLDIG_RST_DCF_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLLDIG_RST_DCF_RX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPLLDIG_RST_DCF_RX` writer - "]
pub struct ADPLLDIG_RST_DCF_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLLDIG_RST_DCF_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn adplldig_rst_dcf_tx(&self) -> ADPLLDIG_RST_DCF_TX_R {
        ADPLLDIG_RST_DCF_TX_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn adplldig_rst_dcf_rx(&self) -> ADPLLDIG_RST_DCF_RX_R {
        ADPLLDIG_RST_DCF_RX_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn adplldig_rst_dcf_tx(&mut self) -> ADPLLDIG_RST_DCF_TX_W {
        ADPLLDIG_RST_DCF_TX_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn adplldig_rst_dcf_rx(&mut self) -> ADPLLDIG_RST_DCF_RX_W {
        ADPLLDIG_RST_DCF_RX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_enable_config30_reg](index.html) module"]
pub struct RF_ENABLE_CONFIG30_REG_SPEC;
impl crate::RegisterSpec for RF_ENABLE_CONFIG30_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_enable_config30_reg::R](R) reader structure"]
impl crate::Readable for RF_ENABLE_CONFIG30_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_enable_config30_reg::W](W) writer structure"]
impl crate::Writable for RF_ENABLE_CONFIG30_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_ENABLE_CONFIG30_REG to value 0x0107"]
impl crate::Resettable for RF_ENABLE_CONFIG30_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0107
    }
}
