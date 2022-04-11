#[doc = "Register `RF_IFF_CTRL_REG` reader"]
pub struct R(crate::R<RF_IFF_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_IFF_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_IFF_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_IFF_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_IFF_CTRL_REG` writer"]
pub struct W(crate::W<RF_IFF_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_IFF_CTRL_REG_SPEC>;
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
impl From<crate::W<RF_IFF_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_IFF_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IFF_DCOC_DAC_REFCUR_CTRL` reader - "]
pub struct IFF_DCOC_DAC_REFCUR_CTRL_R(crate::FieldReader<u8, u8>);
impl IFF_DCOC_DAC_REFCUR_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IFF_DCOC_DAC_REFCUR_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFF_DCOC_DAC_REFCUR_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFF_DCOC_DAC_REFCUR_CTRL` writer - "]
pub struct IFF_DCOC_DAC_REFCUR_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> IFF_DCOC_DAC_REFCUR_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 13)) | ((value as u32 & 3) << 13);
        self.w
    }
}
#[doc = "Field `IFF_COMPLEX_DIS` reader - "]
pub struct IFF_COMPLEX_DIS_R(crate::FieldReader<bool, bool>);
impl IFF_COMPLEX_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IFF_COMPLEX_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFF_COMPLEX_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFF_COMPLEX_DIS` writer - "]
pub struct IFF_COMPLEX_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> IFF_COMPLEX_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `RF_IFF_CTRL_SPARE` reader - "]
pub struct RF_IFF_CTRL_SPARE_R(crate::FieldReader<u8, u8>);
impl RF_IFF_CTRL_SPARE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RF_IFF_CTRL_SPARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_IFF_CTRL_SPARE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF_IFF_CTRL_SPARE` writer - "]
pub struct RF_IFF_CTRL_SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_IFF_CTRL_SPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | ((value as u32 & 0x3f) << 6);
        self.w
    }
}
#[doc = "Field `IFF_DCOC_DAC_DIS` reader - "]
pub struct IFF_DCOC_DAC_DIS_R(crate::FieldReader<bool, bool>);
impl IFF_DCOC_DAC_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IFF_DCOC_DAC_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFF_DCOC_DAC_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFF_DCOC_DAC_DIS` writer - "]
pub struct IFF_DCOC_DAC_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> IFF_DCOC_DAC_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `IF_MUTE` reader - "]
pub struct IF_MUTE_R(crate::FieldReader<bool, bool>);
impl IF_MUTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IF_MUTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IF_MUTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IF_MUTE` writer - "]
pub struct IF_MUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_MUTE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `IF_CAL_TRIM` reader - "]
pub struct IF_CAL_TRIM_R(crate::FieldReader<u8, u8>);
impl IF_CAL_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IF_CAL_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IF_CAL_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IF_CAL_TRIM` writer - "]
pub struct IF_CAL_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_CAL_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn iff_dcoc_dac_refcur_ctrl(&self) -> IFF_DCOC_DAC_REFCUR_CTRL_R {
        IFF_DCOC_DAC_REFCUR_CTRL_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn iff_complex_dis(&self) -> IFF_COMPLEX_DIS_R {
        IFF_COMPLEX_DIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn rf_iff_ctrl_spare(&self) -> RF_IFF_CTRL_SPARE_R {
        RF_IFF_CTRL_SPARE_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn iff_dcoc_dac_dis(&self) -> IFF_DCOC_DAC_DIS_R {
        IFF_DCOC_DAC_DIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn if_mute(&self) -> IF_MUTE_R {
        IF_MUTE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn if_cal_trim(&self) -> IF_CAL_TRIM_R {
        IF_CAL_TRIM_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn iff_dcoc_dac_refcur_ctrl(&mut self) -> IFF_DCOC_DAC_REFCUR_CTRL_W {
        IFF_DCOC_DAC_REFCUR_CTRL_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn iff_complex_dis(&mut self) -> IFF_COMPLEX_DIS_W {
        IFF_COMPLEX_DIS_W { w: self }
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn rf_iff_ctrl_spare(&mut self) -> RF_IFF_CTRL_SPARE_W {
        RF_IFF_CTRL_SPARE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn iff_dcoc_dac_dis(&mut self) -> IFF_DCOC_DAC_DIS_W {
        IFF_DCOC_DAC_DIS_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn if_mute(&mut self) -> IF_MUTE_W {
        IF_MUTE_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn if_cal_trim(&mut self) -> IF_CAL_TRIM_W {
        IF_CAL_TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_iff_ctrl_reg](index.html) module"]
pub struct RF_IFF_CTRL_REG_SPEC;
impl crate::RegisterSpec for RF_IFF_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_iff_ctrl_reg::R](R) reader structure"]
impl crate::Readable for RF_IFF_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_iff_ctrl_reg::W](W) writer structure"]
impl crate::Writable for RF_IFF_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_IFF_CTRL_REG to value 0x01"]
impl crate::Resettable for RF_IFF_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
