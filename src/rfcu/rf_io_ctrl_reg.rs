#[doc = "Register `RF_IO_CTRL_REG` reader"]
pub struct R(crate::R<RF_IO_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_IO_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_IO_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_IO_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_IO_CTRL_REG` writer"]
pub struct W(crate::W<RF_IO_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_IO_CTRL_REG_SPEC>;
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
impl From<crate::W<RF_IO_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_IO_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFIO_TUNE_CAP_TRIM_TX` reader - "]
pub struct RFIO_TUNE_CAP_TRIM_TX_R(crate::FieldReader<u8, u8>);
impl RFIO_TUNE_CAP_TRIM_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RFIO_TUNE_CAP_TRIM_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFIO_TUNE_CAP_TRIM_TX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFIO_TUNE_CAP_TRIM_TX` writer - "]
pub struct RFIO_TUNE_CAP_TRIM_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> RFIO_TUNE_CAP_TRIM_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RFIO_TUNE_CAP_TRIM_RX` reader - "]
pub struct RFIO_TUNE_CAP_TRIM_RX_R(crate::FieldReader<u8, u8>);
impl RFIO_TUNE_CAP_TRIM_RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RFIO_TUNE_CAP_TRIM_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFIO_TUNE_CAP_TRIM_RX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFIO_TUNE_CAP_TRIM_RX` writer - "]
pub struct RFIO_TUNE_CAP_TRIM_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> RFIO_TUNE_CAP_TRIM_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn rfio_tune_cap_trim_tx(&self) -> RFIO_TUNE_CAP_TRIM_TX_R {
        RFIO_TUNE_CAP_TRIM_TX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rfio_tune_cap_trim_rx(&self) -> RFIO_TUNE_CAP_TRIM_RX_R {
        RFIO_TUNE_CAP_TRIM_RX_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn rfio_tune_cap_trim_tx(&mut self) -> RFIO_TUNE_CAP_TRIM_TX_W {
        RFIO_TUNE_CAP_TRIM_TX_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rfio_tune_cap_trim_rx(&mut self) -> RFIO_TUNE_CAP_TRIM_RX_W {
        RFIO_TUNE_CAP_TRIM_RX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_io_ctrl_reg](index.html) module"]
pub struct RF_IO_CTRL_REG_SPEC;
impl crate::RegisterSpec for RF_IO_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_io_ctrl_reg::R](R) reader structure"]
impl crate::Readable for RF_IO_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_io_ctrl_reg::W](W) writer structure"]
impl crate::Writable for RF_IO_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_IO_CTRL_REG to value 0x0100"]
impl crate::Resettable for RF_IO_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
