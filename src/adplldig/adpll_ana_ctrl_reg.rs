#[doc = "Register `ADPLL_ANA_CTRL_REG` reader"]
pub struct R(crate::R<ADPLL_ANA_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_ANA_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_ANA_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_ANA_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_ANA_CTRL_REG` writer"]
pub struct W(crate::W<ADPLL_ANA_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_ANA_CTRL_REG_SPEC>;
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
impl From<crate::W<ADPLL_ANA_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_ANA_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTC_LDO_DMY` reader - "]
pub struct DTC_LDO_DMY_R(crate::FieldReader<u8, u8>);
impl DTC_LDO_DMY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTC_LDO_DMY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTC_LDO_DMY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTC_LDO_DMY` writer - "]
pub struct DTC_LDO_DMY_W<'a> {
    w: &'a mut W,
}
impl<'a> DTC_LDO_DMY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 27)) | ((value as u32 & 3) << 27);
        self.w
    }
}
#[doc = "Field `VPASETTLE` reader - "]
pub struct VPASETTLE_R(crate::FieldReader<u8, u8>);
impl VPASETTLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VPASETTLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VPASETTLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VPASETTLE` writer - "]
pub struct VPASETTLE_W<'a> {
    w: &'a mut W,
}
impl<'a> VPASETTLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 24)) | ((value as u32 & 3) << 24);
        self.w
    }
}
#[doc = "Field `TDC_OFFSET` reader - "]
pub struct TDC_OFFSET_R(crate::FieldReader<u8, u8>);
impl TDC_OFFSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TDC_OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDC_OFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDC_OFFSET` writer - "]
pub struct TDC_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> TDC_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `DTC_EN` reader - "]
pub struct DTC_EN_R(crate::FieldReader<bool, bool>);
impl DTC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTC_EN` writer - "]
pub struct DTC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `DTCOFFSET` reader - "]
pub struct DTCOFFSET_R(crate::FieldReader<u8, u8>);
impl DTCOFFSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTCOFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTCOFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTCOFFSET` writer - "]
pub struct DTCOFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCOFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `TGLDETEN` reader - "]
pub struct TGLDETEN_R(crate::FieldReader<bool, bool>);
impl TGLDETEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TGLDETEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TGLDETEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TGLDETEN` writer - "]
pub struct TGLDETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TGLDETEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `EN_CKDCOMOD` reader - "]
pub struct EN_CKDCOMOD_R(crate::FieldReader<bool, bool>);
impl EN_CKDCOMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_CKDCOMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_CKDCOMOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_CKDCOMOD` writer - "]
pub struct EN_CKDCOMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_CKDCOMOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `INV_CKDCOMOD` reader - "]
pub struct INV_CKDCOMOD_R(crate::FieldReader<bool, bool>);
impl INV_CKDCOMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INV_CKDCOMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INV_CKDCOMOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV_CKDCOMOD` writer - "]
pub struct INV_CKDCOMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_CKDCOMOD_W<'a> {
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
#[doc = "Field `INV_CKPHV` reader - "]
pub struct INV_CKPHV_R(crate::FieldReader<bool, bool>);
impl INV_CKPHV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INV_CKPHV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INV_CKPHV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV_CKPHV` writer - "]
pub struct INV_CKPHV_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_CKPHV_W<'a> {
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
#[doc = "Field `INV_CKTDC` reader - "]
pub struct INV_CKTDC_R(crate::FieldReader<bool, bool>);
impl INV_CKTDC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INV_CKTDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INV_CKTDC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV_CKTDC` writer - "]
pub struct INV_CKTDC_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_CKTDC_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `TDC_INV` reader - "]
pub struct TDC_INV_R(crate::FieldReader<bool, bool>);
impl TDC_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TDC_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDC_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDC_INV` writer - "]
pub struct TDC_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> TDC_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `TDC_CKVIN_EN` reader - "]
pub struct TDC_CKVIN_EN_R(crate::FieldReader<bool, bool>);
impl TDC_CKVIN_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TDC_CKVIN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDC_CKVIN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDC_CKVIN_EN` writer - "]
pub struct TDC_CKVIN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TDC_CKVIN_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `TDC_DTCIN_EN` reader - "]
pub struct TDC_DTCIN_EN_R(crate::FieldReader<bool, bool>);
impl TDC_DTCIN_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TDC_DTCIN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDC_DTCIN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDC_DTCIN_EN` writer - "]
pub struct TDC_DTCIN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TDC_DTCIN_EN_W<'a> {
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
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn dtc_ldo_dmy(&self) -> DTC_LDO_DMY_R {
        DTC_LDO_DMY_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn vpasettle(&self) -> VPASETTLE_R {
        VPASETTLE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tdc_offset(&self) -> TDC_OFFSET_R {
        TDC_OFFSET_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dtc_en(&self) -> DTC_EN_R {
        DTC_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn dtcoffset(&self) -> DTCOFFSET_R {
        DTCOFFSET_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tgldeten(&self) -> TGLDETEN_R {
        TGLDETEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn en_ckdcomod(&self) -> EN_CKDCOMOD_R {
        EN_CKDCOMOD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn inv_ckdcomod(&self) -> INV_CKDCOMOD_R {
        INV_CKDCOMOD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn inv_ckphv(&self) -> INV_CKPHV_R {
        INV_CKPHV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn inv_cktdc(&self) -> INV_CKTDC_R {
        INV_CKTDC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tdc_inv(&self) -> TDC_INV_R {
        TDC_INV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tdc_ckvin_en(&self) -> TDC_CKVIN_EN_R {
        TDC_CKVIN_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tdc_dtcin_en(&self) -> TDC_DTCIN_EN_R {
        TDC_DTCIN_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn dtc_ldo_dmy(&mut self) -> DTC_LDO_DMY_W {
        DTC_LDO_DMY_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn vpasettle(&mut self) -> VPASETTLE_W {
        VPASETTLE_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tdc_offset(&mut self) -> TDC_OFFSET_W {
        TDC_OFFSET_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dtc_en(&mut self) -> DTC_EN_W {
        DTC_EN_W { w: self }
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn dtcoffset(&mut self) -> DTCOFFSET_W {
        DTCOFFSET_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tgldeten(&mut self) -> TGLDETEN_W {
        TGLDETEN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn en_ckdcomod(&mut self) -> EN_CKDCOMOD_W {
        EN_CKDCOMOD_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn inv_ckdcomod(&mut self) -> INV_CKDCOMOD_W {
        INV_CKDCOMOD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn inv_ckphv(&mut self) -> INV_CKPHV_W {
        INV_CKPHV_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn inv_cktdc(&mut self) -> INV_CKTDC_W {
        INV_CKTDC_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tdc_inv(&mut self) -> TDC_INV_W {
        TDC_INV_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tdc_ckvin_en(&mut self) -> TDC_CKVIN_EN_W {
        TDC_CKVIN_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tdc_dtcin_en(&mut self) -> TDC_DTCIN_EN_W {
        TDC_DTCIN_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_ana_ctrl_reg](index.html) module"]
pub struct ADPLL_ANA_CTRL_REG_SPEC;
impl crate::RegisterSpec for ADPLL_ANA_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_ana_ctrl_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_ANA_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_ana_ctrl_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_ANA_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_ANA_CTRL_REG to value 0x0215_807b"]
impl crate::Resettable for ADPLL_ANA_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0215_807b
    }
}
