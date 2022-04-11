#[doc = "Register `GP_DATA_REG` reader"]
pub struct R(crate::R<GP_DATA_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_DATA_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_DATA_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_DATA_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GP_DATA_REG` writer"]
pub struct W(crate::W<GP_DATA_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_DATA_REG_SPEC>;
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
impl From<crate::W<GP_DATA_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_DATA_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANA_SPARE` reader - "]
pub struct ANA_SPARE_R(crate::FieldReader<u8, u8>);
impl ANA_SPARE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ANA_SPARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANA_SPARE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANA_SPARE` writer - "]
pub struct ANA_SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> ANA_SPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 5)) | ((value as u16 & 7) << 5);
        self.w
    }
}
#[doc = "Field `DISABLE_CLAMP_OVERRULE` reader - "]
pub struct DISABLE_CLAMP_OVERRULE_R(crate::FieldReader<bool, bool>);
impl DISABLE_CLAMP_OVERRULE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISABLE_CLAMP_OVERRULE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISABLE_CLAMP_OVERRULE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISABLE_CLAMP_OVERRULE` writer - "]
pub struct DISABLE_CLAMP_OVERRULE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_CLAMP_OVERRULE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u16 & 1) << 4);
        self.w
    }
}
#[doc = "Field `SW_GP_DATA` reader - "]
pub struct SW_GP_DATA_R(crate::FieldReader<u8, u8>);
impl SW_GP_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SW_GP_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_GP_DATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_GP_DATA` writer - "]
pub struct SW_GP_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_GP_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u16 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn ana_spare(&self) -> ANA_SPARE_R {
        ANA_SPARE_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn disable_clamp_overrule(&self) -> DISABLE_CLAMP_OVERRULE_R {
        DISABLE_CLAMP_OVERRULE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sw_gp_data(&self) -> SW_GP_DATA_R {
        SW_GP_DATA_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn ana_spare(&mut self) -> ANA_SPARE_W {
        ANA_SPARE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn disable_clamp_overrule(&mut self) -> DISABLE_CLAMP_OVERRULE_W {
        DISABLE_CLAMP_OVERRULE_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sw_gp_data(&mut self) -> SW_GP_DATA_W {
        SW_GP_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_data_reg](index.html) module"]
pub struct GP_DATA_REG_SPEC;
impl crate::RegisterSpec for GP_DATA_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gp_data_reg::R](R) reader structure"]
impl crate::Readable for GP_DATA_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_data_reg::W](W) writer structure"]
impl crate::Writable for GP_DATA_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GP_DATA_REG to value 0"]
impl crate::Resettable for GP_DATA_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
