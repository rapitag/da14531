#[doc = "Register `ADPLL_KDTCTDC_CAL_CTRL2_REG` reader"]
pub struct R(crate::R<ADPLL_KDTCTDC_CAL_CTRL2_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_KDTCTDC_CAL_CTRL2_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_KDTCTDC_CAL_CTRL2_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_KDTCTDC_CAL_CTRL2_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_KDTCTDC_CAL_CTRL2_REG` writer"]
pub struct W(crate::W<ADPLL_KDTCTDC_CAL_CTRL2_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_KDTCTDC_CAL_CTRL2_REG_SPEC>;
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
impl From<crate::W<ADPLL_KDTCTDC_CAL_CTRL2_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_KDTCTDC_CAL_CTRL2_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHRDLY_EXTRA` reader - "]
pub struct PHRDLY_EXTRA_R(crate::FieldReader<bool, bool>);
impl PHRDLY_EXTRA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PHRDLY_EXTRA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHRDLY_EXTRA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHRDLY_EXTRA` writer - "]
pub struct PHRDLY_EXTRA_W<'a> {
    w: &'a mut W,
}
impl<'a> PHRDLY_EXTRA_W<'a> {
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
#[doc = "Field `TKDTCCAL` reader - "]
pub struct TKDTCCAL_R(crate::FieldReader<u8, u8>);
impl TKDTCCAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TKDTCCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TKDTCCAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TKDTCCAL` writer - "]
pub struct TKDTCCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TKDTCCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | ((value as u32 & 0x0f) << 11);
        self.w
    }
}
#[doc = "Field `PHRDLY` reader - "]
pub struct PHRDLY_R(crate::FieldReader<u8, u8>);
impl PHRDLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PHRDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHRDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHRDLY` writer - "]
pub struct PHRDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> PHRDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 9)) | ((value as u32 & 3) << 9);
        self.w
    }
}
#[doc = "Field `KTDCCALEN` reader - "]
pub struct KTDCCALEN_R(crate::FieldReader<bool, bool>);
impl KTDCCALEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KTDCCALEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KTDCCALEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KTDCCALEN` writer - "]
pub struct KTDCCALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> KTDCCALEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `KDTCCALLG` reader - "]
pub struct KDTCCALLG_R(crate::FieldReader<u8, u8>);
impl KDTCCALLG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KDTCCALLG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDTCCALLG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDTCCALLG` writer - "]
pub struct KDTCCALLG_W<'a> {
    w: &'a mut W,
}
impl<'a> KDTCCALLG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 4)) | ((value as u32 & 7) << 4);
        self.w
    }
}
#[doc = "Field `KDTCCAL_INV` reader - "]
pub struct KDTCCAL_INV_R(crate::FieldReader<bool, bool>);
impl KDTCCAL_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KDTCCAL_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDTCCAL_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDTCCAL_INV` writer - "]
pub struct KDTCCAL_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> KDTCCAL_INV_W<'a> {
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
#[doc = "Field `KDTCCALMOD1P` reader - "]
pub struct KDTCCALMOD1P_R(crate::FieldReader<bool, bool>);
impl KDTCCALMOD1P_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KDTCCALMOD1P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDTCCALMOD1P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDTCCALMOD1P` writer - "]
pub struct KDTCCALMOD1P_W<'a> {
    w: &'a mut W,
}
impl<'a> KDTCCALMOD1P_W<'a> {
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
#[doc = "Field `KDTCCALMOD` reader - "]
pub struct KDTCCALMOD_R(crate::FieldReader<bool, bool>);
impl KDTCCALMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KDTCCALMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDTCCALMOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDTCCALMOD` writer - "]
pub struct KDTCCALMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> KDTCCALMOD_W<'a> {
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
#[doc = "Field `KDTCCALEN` reader - "]
pub struct KDTCCALEN_R(crate::FieldReader<bool, bool>);
impl KDTCCALEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KDTCCALEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDTCCALEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDTCCALEN` writer - "]
pub struct KDTCCALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> KDTCCALEN_W<'a> {
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
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn phrdly_extra(&self) -> PHRDLY_EXTRA_R {
        PHRDLY_EXTRA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn tkdtccal(&self) -> TKDTCCAL_R {
        TKDTCCAL_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn phrdly(&self) -> PHRDLY_R {
        PHRDLY_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ktdccalen(&self) -> KTDCCALEN_R {
        KTDCCALEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn kdtccallg(&self) -> KDTCCALLG_R {
        KDTCCALLG_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn kdtccal_inv(&self) -> KDTCCAL_INV_R {
        KDTCCAL_INV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn kdtccalmod1p(&self) -> KDTCCALMOD1P_R {
        KDTCCALMOD1P_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn kdtccalmod(&self) -> KDTCCALMOD_R {
        KDTCCALMOD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn kdtccalen(&self) -> KDTCCALEN_R {
        KDTCCALEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn phrdly_extra(&mut self) -> PHRDLY_EXTRA_W {
        PHRDLY_EXTRA_W { w: self }
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn tkdtccal(&mut self) -> TKDTCCAL_W {
        TKDTCCAL_W { w: self }
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn phrdly(&mut self) -> PHRDLY_W {
        PHRDLY_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ktdccalen(&mut self) -> KTDCCALEN_W {
        KTDCCALEN_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn kdtccallg(&mut self) -> KDTCCALLG_W {
        KDTCCALLG_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn kdtccal_inv(&mut self) -> KDTCCAL_INV_W {
        KDTCCAL_INV_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn kdtccalmod1p(&mut self) -> KDTCCALMOD1P_W {
        KDTCCALMOD1P_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn kdtccalmod(&mut self) -> KDTCCALMOD_W {
        KDTCCALMOD_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn kdtccalen(&mut self) -> KDTCCALEN_W {
        KDTCCALEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_kdtctdc_cal_ctrl2_reg](index.html) module"]
pub struct ADPLL_KDTCTDC_CAL_CTRL2_REG_SPEC;
impl crate::RegisterSpec for ADPLL_KDTCTDC_CAL_CTRL2_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_kdtctdc_cal_ctrl2_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_KDTCTDC_CAL_CTRL2_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_kdtctdc_cal_ctrl2_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_KDTCTDC_CAL_CTRL2_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_KDTCTDC_CAL_CTRL2_REG to value 0x7f50"]
impl crate::Resettable for ADPLL_KDTCTDC_CAL_CTRL2_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f50
    }
}
