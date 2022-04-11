#[doc = "Register `RF_ADPLLDIG_RFMON_CTRL_REG` reader"]
pub struct R(crate::R<RF_ADPLLDIG_RFMON_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_ADPLLDIG_RFMON_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_ADPLLDIG_RFMON_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_ADPLLDIG_RFMON_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_ADPLLDIG_RFMON_CTRL_REG` writer"]
pub struct W(crate::W<RF_ADPLLDIG_RFMON_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_ADPLLDIG_RFMON_CTRL_REG_SPEC>;
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
impl From<crate::W<RF_ADPLLDIG_RFMON_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_ADPLLDIG_RFMON_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADPLLDIG_RFMON_SPARE` reader - "]
pub struct ADPLLDIG_RFMON_SPARE_R(crate::FieldReader<u8, u8>);
impl ADPLLDIG_RFMON_SPARE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADPLLDIG_RFMON_SPARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLLDIG_RFMON_SPARE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPLLDIG_RFMON_SPARE` writer - "]
pub struct ADPLLDIG_RFMON_SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLLDIG_RFMON_SPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `ADPLLDIG_RFMON_MUX_SEL` reader - "]
pub struct ADPLLDIG_RFMON_MUX_SEL_R(crate::FieldReader<u8, u8>);
impl ADPLLDIG_RFMON_MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADPLLDIG_RFMON_MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLLDIG_RFMON_MUX_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPLLDIG_RFMON_MUX_SEL` writer - "]
pub struct ADPLLDIG_RFMON_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLLDIG_RFMON_MUX_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 1)) | ((value as u32 & 7) << 1);
        self.w
    }
}
#[doc = "Field `ADPLLDIG_SYNC_CLK_INV` reader - "]
pub struct ADPLLDIG_SYNC_CLK_INV_R(crate::FieldReader<bool, bool>);
impl ADPLLDIG_SYNC_CLK_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLLDIG_SYNC_CLK_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLLDIG_SYNC_CLK_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPLLDIG_SYNC_CLK_INV` writer - "]
pub struct ADPLLDIG_SYNC_CLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLLDIG_SYNC_CLK_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn adplldig_rfmon_spare(&self) -> ADPLLDIG_RFMON_SPARE_R {
        ADPLLDIG_RFMON_SPARE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn adplldig_rfmon_mux_sel(&self) -> ADPLLDIG_RFMON_MUX_SEL_R {
        ADPLLDIG_RFMON_MUX_SEL_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adplldig_sync_clk_inv(&self) -> ADPLLDIG_SYNC_CLK_INV_R {
        ADPLLDIG_SYNC_CLK_INV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn adplldig_rfmon_spare(&mut self) -> ADPLLDIG_RFMON_SPARE_W {
        ADPLLDIG_RFMON_SPARE_W { w: self }
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn adplldig_rfmon_mux_sel(&mut self) -> ADPLLDIG_RFMON_MUX_SEL_W {
        ADPLLDIG_RFMON_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adplldig_sync_clk_inv(&mut self) -> ADPLLDIG_SYNC_CLK_INV_W {
        ADPLLDIG_SYNC_CLK_INV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_adplldig_rfmon_ctrl_reg](index.html) module"]
pub struct RF_ADPLLDIG_RFMON_CTRL_REG_SPEC;
impl crate::RegisterSpec for RF_ADPLLDIG_RFMON_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_adplldig_rfmon_ctrl_reg::R](R) reader structure"]
impl crate::Readable for RF_ADPLLDIG_RFMON_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_adplldig_rfmon_ctrl_reg::W](W) writer structure"]
impl crate::Writable for RF_ADPLLDIG_RFMON_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_ADPLLDIG_RFMON_CTRL_REG to value 0x01"]
impl crate::Resettable for RF_ADPLLDIG_RFMON_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
