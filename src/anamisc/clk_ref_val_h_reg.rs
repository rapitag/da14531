#[doc = "Register `CLK_REF_VAL_H_REG` reader"]
pub struct R(crate::R<CLK_REF_VAL_H_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_REF_VAL_H_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_REF_VAL_H_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_REF_VAL_H_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_REF_VAL_H_REG` writer"]
pub struct W(crate::W<CLK_REF_VAL_H_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_REF_VAL_H_REG_SPEC>;
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
impl From<crate::W<CLK_REF_VAL_H_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_REF_VAL_H_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL_CNT_VAL` reader - Returns the number of DIVN clock cycles counted during the calibration time, defined with REF_CNT_VAL"]
pub struct XTAL_CNT_VAL_R(crate::FieldReader<u16, u16>);
impl XTAL_CNT_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        XTAL_CNT_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL_CNT_VAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Returns the number of DIVN clock cycles counted during the calibration time, defined with REF_CNT_VAL"]
    #[inline(always)]
    pub fn xtal_cnt_val(&self) -> XTAL_CNT_VAL_R {
        XTAL_CNT_VAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XTAL32M reference cycles, higher 16 bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ref_val_h_reg](index.html) module"]
pub struct CLK_REF_VAL_H_REG_SPEC;
impl crate::RegisterSpec for CLK_REF_VAL_H_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [clk_ref_val_h_reg::R](R) reader structure"]
impl crate::Readable for CLK_REF_VAL_H_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ref_val_h_reg::W](W) writer structure"]
impl crate::Writable for CLK_REF_VAL_H_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_REF_VAL_H_REG to value 0"]
impl crate::Resettable for CLK_REF_VAL_H_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
