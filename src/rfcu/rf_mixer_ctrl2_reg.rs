#[doc = "Register `RF_MIXER_CTRL2_REG` reader"]
pub struct R(crate::R<RF_MIXER_CTRL2_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_MIXER_CTRL2_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_MIXER_CTRL2_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_MIXER_CTRL2_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_MIXER_CTRL2_REG` writer"]
pub struct W(crate::W<RF_MIXER_CTRL2_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_MIXER_CTRL2_REG_SPEC>;
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
impl From<crate::W<RF_MIXER_CTRL2_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_MIXER_CTRL2_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIX_CAL_SELECT` reader - "]
pub struct MIX_CAL_SELECT_R(crate::FieldReader<bool, bool>);
impl MIX_CAL_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MIX_CAL_SELECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIX_CAL_SELECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIX_CAL_SELECT` writer - "]
pub struct MIX_CAL_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> MIX_CAL_SELECT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `MIX_CAL_CAP_WR_2M` reader - "]
pub struct MIX_CAL_CAP_WR_2M_R(crate::FieldReader<u8, u8>);
impl MIX_CAL_CAP_WR_2M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MIX_CAL_CAP_WR_2M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIX_CAL_CAP_WR_2M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIX_CAL_CAP_WR_2M` writer - "]
pub struct MIX_CAL_CAP_WR_2M_W<'a> {
    w: &'a mut W,
}
impl<'a> MIX_CAL_CAP_WR_2M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `MIX_CAL_CAP_WR_1M` reader - "]
pub struct MIX_CAL_CAP_WR_1M_R(crate::FieldReader<u8, u8>);
impl MIX_CAL_CAP_WR_1M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MIX_CAL_CAP_WR_1M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIX_CAL_CAP_WR_1M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIX_CAL_CAP_WR_1M` writer - "]
pub struct MIX_CAL_CAP_WR_1M_W<'a> {
    w: &'a mut W,
}
impl<'a> MIX_CAL_CAP_WR_1M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mix_cal_select(&self) -> MIX_CAL_SELECT_R {
        MIX_CAL_SELECT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn mix_cal_cap_wr_2m(&self) -> MIX_CAL_CAP_WR_2M_R {
        MIX_CAL_CAP_WR_2M_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn mix_cal_cap_wr_1m(&self) -> MIX_CAL_CAP_WR_1M_R {
        MIX_CAL_CAP_WR_1M_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mix_cal_select(&mut self) -> MIX_CAL_SELECT_W {
        MIX_CAL_SELECT_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn mix_cal_cap_wr_2m(&mut self) -> MIX_CAL_CAP_WR_2M_W {
        MIX_CAL_CAP_WR_2M_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn mix_cal_cap_wr_1m(&mut self) -> MIX_CAL_CAP_WR_1M_W {
        MIX_CAL_CAP_WR_1M_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_mixer_ctrl2_reg](index.html) module"]
pub struct RF_MIXER_CTRL2_REG_SPEC;
impl crate::RegisterSpec for RF_MIXER_CTRL2_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_mixer_ctrl2_reg::R](R) reader structure"]
impl crate::Readable for RF_MIXER_CTRL2_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_mixer_ctrl2_reg::W](W) writer structure"]
impl crate::Writable for RF_MIXER_CTRL2_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_MIXER_CTRL2_REG to value 0"]
impl crate::Resettable for RF_MIXER_CTRL2_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
