#[doc = "Register `CLK_REF_CNT_REG` reader"]
pub struct R(crate::R<CLK_REF_CNT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_REF_CNT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_REF_CNT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_REF_CNT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_REF_CNT_REG` writer"]
pub struct W(crate::W<CLK_REF_CNT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_REF_CNT_REG_SPEC>;
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
impl From<crate::W<CLK_REF_CNT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_REF_CNT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REF_CNT_VAL` reader - Indicates the calibration time, with a decrement counter to 1."]
pub struct REF_CNT_VAL_R(crate::FieldReader<u16, u16>);
impl REF_CNT_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        REF_CNT_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REF_CNT_VAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REF_CNT_VAL` writer - Indicates the calibration time, with a decrement counter to 1."]
pub struct REF_CNT_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_CNT_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Indicates the calibration time, with a decrement counter to 1."]
    #[inline(always)]
    pub fn ref_cnt_val(&self) -> REF_CNT_VAL_R {
        REF_CNT_VAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicates the calibration time, with a decrement counter to 1."]
    #[inline(always)]
    pub fn ref_cnt_val(&mut self) -> REF_CNT_VAL_W {
        REF_CNT_VAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Count value for oscillator calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ref_cnt_reg](index.html) module"]
pub struct CLK_REF_CNT_REG_SPEC;
impl crate::RegisterSpec for CLK_REF_CNT_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [clk_ref_cnt_reg::R](R) reader structure"]
impl crate::Readable for CLK_REF_CNT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ref_cnt_reg::W](W) writer structure"]
impl crate::Writable for CLK_REF_CNT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_REF_CNT_REG to value 0"]
impl crate::Resettable for CLK_REF_CNT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
