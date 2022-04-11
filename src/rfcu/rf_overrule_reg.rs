#[doc = "Register `RF_OVERRULE_REG` reader"]
pub struct R(crate::R<RF_OVERRULE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_OVERRULE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_OVERRULE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_OVERRULE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_OVERRULE_REG` writer"]
pub struct W(crate::W<RF_OVERRULE_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_OVERRULE_REG_SPEC>;
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
impl From<crate::W<RF_OVERRULE_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_OVERRULE_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_EN_OVR` reader - "]
pub struct RX_EN_OVR_R(crate::FieldReader<u8, u8>);
impl RX_EN_OVR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_EN_OVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_EN_OVR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_EN_OVR` writer - "]
pub struct RX_EN_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EN_OVR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `TX_EN_OVR` reader - "]
pub struct TX_EN_OVR_R(crate::FieldReader<u8, u8>);
impl TX_EN_OVR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_EN_OVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_EN_OVR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_EN_OVR` writer - "]
pub struct TX_EN_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_EN_OVR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn rx_en_ovr(&self) -> RX_EN_OVR_R {
        RX_EN_OVR_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tx_en_ovr(&self) -> TX_EN_OVR_R {
        TX_EN_OVR_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn rx_en_ovr(&mut self) -> RX_EN_OVR_W {
        RX_EN_OVR_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tx_en_ovr(&mut self) -> TX_EN_OVR_W {
        TX_EN_OVR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_overrule_reg](index.html) module"]
pub struct RF_OVERRULE_REG_SPEC;
impl crate::RegisterSpec for RF_OVERRULE_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_overrule_reg::R](R) reader structure"]
impl crate::Readable for RF_OVERRULE_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_overrule_reg::W](W) writer structure"]
impl crate::Writable for RF_OVERRULE_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_OVERRULE_REG to value 0"]
impl crate::Resettable for RF_OVERRULE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
