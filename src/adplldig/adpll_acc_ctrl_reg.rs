#[doc = "Register `ADPLL_ACC_CTRL_REG` reader"]
pub struct R(crate::R<ADPLL_ACC_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_ACC_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_ACC_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_ACC_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_ACC_CTRL_REG` writer"]
pub struct W(crate::W<ADPLL_ACC_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_ACC_CTRL_REG_SPEC>;
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
impl From<crate::W<ADPLL_ACC_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_ACC_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN_CMF_AVG` reader - "]
pub struct EN_CMF_AVG_R(crate::FieldReader<bool, bool>);
impl EN_CMF_AVG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_CMF_AVG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_CMF_AVG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_CMF_AVG` writer - "]
pub struct EN_CMF_AVG_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_CMF_AVG_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
#[doc = "Field `CLIP_MOD_TUNE_0_TX` reader - "]
pub struct CLIP_MOD_TUNE_0_TX_R(crate::FieldReader<u16, u16>);
impl CLIP_MOD_TUNE_0_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CLIP_MOD_TUNE_0_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLIP_MOD_TUNE_0_TX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLIP_MOD_TUNE_0_TX` writer - "]
pub struct CLIP_MOD_TUNE_0_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> CLIP_MOD_TUNE_0_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 16)) | ((value as u32 & 0x1fff) << 16);
        self.w
    }
}
#[doc = "Field `CLIP_MOD_TUNE_0_RX` reader - "]
pub struct CLIP_MOD_TUNE_0_RX_R(crate::FieldReader<u16, u16>);
impl CLIP_MOD_TUNE_0_RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CLIP_MOD_TUNE_0_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLIP_MOD_TUNE_0_RX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLIP_MOD_TUNE_0_RX` writer - "]
pub struct CLIP_MOD_TUNE_0_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> CLIP_MOD_TUNE_0_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | (value as u32 & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn en_cmf_avg(&self) -> EN_CMF_AVG_R {
        EN_CMF_AVG_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 16:28"]
    #[inline(always)]
    pub fn clip_mod_tune_0_tx(&self) -> CLIP_MOD_TUNE_0_TX_R {
        CLIP_MOD_TUNE_0_TX_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn clip_mod_tune_0_rx(&self) -> CLIP_MOD_TUNE_0_RX_R {
        CLIP_MOD_TUNE_0_RX_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn en_cmf_avg(&mut self) -> EN_CMF_AVG_W {
        EN_CMF_AVG_W { w: self }
    }
    #[doc = "Bits 16:28"]
    #[inline(always)]
    pub fn clip_mod_tune_0_tx(&mut self) -> CLIP_MOD_TUNE_0_TX_W {
        CLIP_MOD_TUNE_0_TX_W { w: self }
    }
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn clip_mod_tune_0_rx(&mut self) -> CLIP_MOD_TUNE_0_RX_W {
        CLIP_MOD_TUNE_0_RX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_acc_ctrl_reg](index.html) module"]
pub struct ADPLL_ACC_CTRL_REG_SPEC;
impl crate::RegisterSpec for ADPLL_ACC_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_acc_ctrl_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_ACC_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_acc_ctrl_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_ACC_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_ACC_CTRL_REG to value 0x7fff_ffff"]
impl crate::Resettable for ADPLL_ACC_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7fff_ffff
    }
}
