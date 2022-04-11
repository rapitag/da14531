#[doc = "Register `CLK_REF_SEL_REG` reader"]
pub struct R(crate::R<CLK_REF_SEL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_REF_SEL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_REF_SEL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_REF_SEL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_REF_SEL_REG` writer"]
pub struct W(crate::W<CLK_REF_SEL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_REF_SEL_REG_SPEC>;
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
impl From<crate::W<CLK_REF_SEL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_REF_SEL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXT_CNT_EN_SEL` reader - 0 : Enable XTAL_CNT counter by the REF_CLK selected by REF_CLK_SEL. 1 : Enable XTAL_CNT counter from an external input."]
pub struct EXT_CNT_EN_SEL_R(crate::FieldReader<bool, bool>);
impl EXT_CNT_EN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXT_CNT_EN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_CNT_EN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT_CNT_EN_SEL` writer - 0 : Enable XTAL_CNT counter by the REF_CLK selected by REF_CLK_SEL. 1 : Enable XTAL_CNT counter from an external input."]
pub struct EXT_CNT_EN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_CNT_EN_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u16 & 1) << 3);
        self.w
    }
}
#[doc = "Field `REF_CAL_START` reader - Writing a '1' starts a calibration of the clock selected by CLK_REF_SEL_REG\\[REF_CLK_SEL\\]. This bit is cleared when calibration is finished, and CLK_REF_VAL is ready."]
pub struct REF_CAL_START_R(crate::FieldReader<bool, bool>);
impl REF_CAL_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REF_CAL_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REF_CAL_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REF_CAL_START` writer - Writing a '1' starts a calibration of the clock selected by CLK_REF_SEL_REG\\[REF_CLK_SEL\\]. This bit is cleared when calibration is finished, and CLK_REF_VAL is ready."]
pub struct REF_CAL_START_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_CAL_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u16 & 1) << 2);
        self.w
    }
}
#[doc = "Field `REF_CLK_SEL` reader - Select clock input for calibration: 0x0 : RC32K 0x1 : RC32M 0x2 : XTAL32K 0x3 : RCX"]
pub struct REF_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl REF_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REF_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REF_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REF_CLK_SEL` writer - Select clock input for calibration: 0x0 : RC32K 0x1 : RC32M 0x2 : XTAL32K 0x3 : RCX"]
pub struct REF_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u16 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - 0 : Enable XTAL_CNT counter by the REF_CLK selected by REF_CLK_SEL. 1 : Enable XTAL_CNT counter from an external input."]
    #[inline(always)]
    pub fn ext_cnt_en_sel(&self) -> EXT_CNT_EN_SEL_R {
        EXT_CNT_EN_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Writing a '1' starts a calibration of the clock selected by CLK_REF_SEL_REG\\[REF_CLK_SEL\\]. This bit is cleared when calibration is finished, and CLK_REF_VAL is ready."]
    #[inline(always)]
    pub fn ref_cal_start(&self) -> REF_CAL_START_R {
        REF_CAL_START_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 0:1 - Select clock input for calibration: 0x0 : RC32K 0x1 : RC32M 0x2 : XTAL32K 0x3 : RCX"]
    #[inline(always)]
    pub fn ref_clk_sel(&self) -> REF_CLK_SEL_R {
        REF_CLK_SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - 0 : Enable XTAL_CNT counter by the REF_CLK selected by REF_CLK_SEL. 1 : Enable XTAL_CNT counter from an external input."]
    #[inline(always)]
    pub fn ext_cnt_en_sel(&mut self) -> EXT_CNT_EN_SEL_W {
        EXT_CNT_EN_SEL_W { w: self }
    }
    #[doc = "Bit 2 - Writing a '1' starts a calibration of the clock selected by CLK_REF_SEL_REG\\[REF_CLK_SEL\\]. This bit is cleared when calibration is finished, and CLK_REF_VAL is ready."]
    #[inline(always)]
    pub fn ref_cal_start(&mut self) -> REF_CAL_START_W {
        REF_CAL_START_W { w: self }
    }
    #[doc = "Bits 0:1 - Select clock input for calibration: 0x0 : RC32K 0x1 : RC32M 0x2 : XTAL32K 0x3 : RCX"]
    #[inline(always)]
    pub fn ref_clk_sel(&mut self) -> REF_CLK_SEL_W {
        REF_CLK_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Select clock for oscillator calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ref_sel_reg](index.html) module"]
pub struct CLK_REF_SEL_REG_SPEC;
impl crate::RegisterSpec for CLK_REF_SEL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [clk_ref_sel_reg::R](R) reader structure"]
impl crate::Readable for CLK_REF_SEL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ref_sel_reg::W](W) writer structure"]
impl crate::Writable for CLK_REF_SEL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_REF_SEL_REG to value 0"]
impl crate::Resettable for CLK_REF_SEL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
