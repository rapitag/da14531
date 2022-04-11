#[doc = "Register `XTAL32M_CTRL0_REG` reader"]
pub struct R(crate::R<XTAL32M_CTRL0_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL32M_CTRL0_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL32M_CTRL0_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL32M_CTRL0_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL32M_CTRL0_REG` writer"]
pub struct W(crate::W<XTAL32M_CTRL0_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL32M_CTRL0_REG_SPEC>;
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
impl From<crate::W<XTAL32M_CTRL0_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL32M_CTRL0_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL32M_SPARE` reader - "]
pub struct XTAL32M_SPARE_R(crate::FieldReader<u8, u8>);
impl XTAL32M_SPARE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XTAL32M_SPARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32M_SPARE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32M_SPARE` writer - "]
pub struct XTAL32M_SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32M_SPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u16 & 3) << 8);
        self.w
    }
}
#[doc = "Field `CORE_AMPL_TRIM` reader - Core amplitude trimming"]
pub struct CORE_AMPL_TRIM_R(crate::FieldReader<u8, u8>);
impl CORE_AMPL_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_AMPL_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_AMPL_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_AMPL_TRIM` writer - Core amplitude trimming"]
pub struct CORE_AMPL_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_AMPL_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 5)) | ((value as u16 & 7) << 5);
        self.w
    }
}
#[doc = "Field `CORE_CUR_SET` reader - Core current trim setting"]
pub struct CORE_CUR_SET_R(crate::FieldReader<u8, u8>);
impl CORE_CUR_SET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_CUR_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_CUR_SET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_CUR_SET` writer - Core current trim setting"]
pub struct CORE_CUR_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_CUR_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 2)) | ((value as u16 & 7) << 2);
        self.w
    }
}
#[doc = "Field `CORE_AMPL_REG_NULLBIAS` reader - Keep bias in ampl detector alive, even when there is a large drive"]
pub struct CORE_AMPL_REG_NULLBIAS_R(crate::FieldReader<bool, bool>);
impl CORE_AMPL_REG_NULLBIAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_AMPL_REG_NULLBIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_AMPL_REG_NULLBIAS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_AMPL_REG_NULLBIAS` writer - Keep bias in ampl detector alive, even when there is a large drive"]
pub struct CORE_AMPL_REG_NULLBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_AMPL_REG_NULLBIAS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u16 & 1) << 1);
        self.w
    }
}
#[doc = "Field `DCBLOCK_ENABLE` reader - Enable dcblock/high pass filter circuit"]
pub struct DCBLOCK_ENABLE_R(crate::FieldReader<bool, bool>);
impl DCBLOCK_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCBLOCK_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCBLOCK_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCBLOCK_ENABLE` writer - Enable dcblock/high pass filter circuit"]
pub struct DCBLOCK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCBLOCK_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u16 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn xtal32m_spare(&self) -> XTAL32M_SPARE_R {
        XTAL32M_SPARE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Core amplitude trimming"]
    #[inline(always)]
    pub fn core_ampl_trim(&self) -> CORE_AMPL_TRIM_R {
        CORE_AMPL_TRIM_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 2:4 - Core current trim setting"]
    #[inline(always)]
    pub fn core_cur_set(&self) -> CORE_CUR_SET_R {
        CORE_CUR_SET_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 1 - Keep bias in ampl detector alive, even when there is a large drive"]
    #[inline(always)]
    pub fn core_ampl_reg_nullbias(&self) -> CORE_AMPL_REG_NULLBIAS_R {
        CORE_AMPL_REG_NULLBIAS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Enable dcblock/high pass filter circuit"]
    #[inline(always)]
    pub fn dcblock_enable(&self) -> DCBLOCK_ENABLE_R {
        DCBLOCK_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn xtal32m_spare(&mut self) -> XTAL32M_SPARE_W {
        XTAL32M_SPARE_W { w: self }
    }
    #[doc = "Bits 5:7 - Core amplitude trimming"]
    #[inline(always)]
    pub fn core_ampl_trim(&mut self) -> CORE_AMPL_TRIM_W {
        CORE_AMPL_TRIM_W { w: self }
    }
    #[doc = "Bits 2:4 - Core current trim setting"]
    #[inline(always)]
    pub fn core_cur_set(&mut self) -> CORE_CUR_SET_W {
        CORE_CUR_SET_W { w: self }
    }
    #[doc = "Bit 1 - Keep bias in ampl detector alive, even when there is a large drive"]
    #[inline(always)]
    pub fn core_ampl_reg_nullbias(&mut self) -> CORE_AMPL_REG_NULLBIAS_W {
        CORE_AMPL_REG_NULLBIAS_W { w: self }
    }
    #[doc = "Bit 0 - Enable dcblock/high pass filter circuit"]
    #[inline(always)]
    pub fn dcblock_enable(&mut self) -> DCBLOCK_ENABLE_W {
        DCBLOCK_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control bits for XTAL32M\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal32m_ctrl0_reg](index.html) module"]
pub struct XTAL32M_CTRL0_REG_SPEC;
impl crate::RegisterSpec for XTAL32M_CTRL0_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [xtal32m_ctrl0_reg::R](R) reader structure"]
impl crate::Readable for XTAL32M_CTRL0_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal32m_ctrl0_reg::W](W) writer structure"]
impl crate::Writable for XTAL32M_CTRL0_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTAL32M_CTRL0_REG to value 0x15"]
impl crate::Resettable for XTAL32M_CTRL0_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x15
    }
}
