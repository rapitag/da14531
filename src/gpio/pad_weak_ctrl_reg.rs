#[doc = "Register `PAD_WEAK_CTRL_REG` reader"]
pub struct R(crate::R<PAD_WEAK_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_WEAK_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_WEAK_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_WEAK_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_WEAK_CTRL_REG` writer"]
pub struct W(crate::W<PAD_WEAK_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_WEAK_CTRL_REG_SPEC>;
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
impl From<crate::W<PAD_WEAK_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_WEAK_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_LOW_DRV` reader - 0 = Normal operation 1 = Reduces the driving strength of P0_x pad. Bit x controls the driving strength of P0_x, x=0, 1,..., 11."]
pub struct PAD_LOW_DRV_R(crate::FieldReader<u16, u16>);
impl PAD_LOW_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PAD_LOW_DRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_LOW_DRV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_LOW_DRV` writer - 0 = Normal operation 1 = Reduces the driving strength of P0_x pad. Bit x controls the driving strength of P0_x, x=0, 1,..., 11."]
pub struct PAD_LOW_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_LOW_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u16 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - 0 = Normal operation 1 = Reduces the driving strength of P0_x pad. Bit x controls the driving strength of P0_x, x=0, 1,..., 11."]
    #[inline(always)]
    pub fn pad_low_drv(&self) -> PAD_LOW_DRV_R {
        PAD_LOW_DRV_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 0 = Normal operation 1 = Reduces the driving strength of P0_x pad. Bit x controls the driving strength of P0_x, x=0, 1,..., 11."]
    #[inline(always)]
    pub fn pad_low_drv(&mut self) -> PAD_LOW_DRV_W {
        PAD_LOW_DRV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pad driving strength control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_weak_ctrl_reg](index.html) module"]
pub struct PAD_WEAK_CTRL_REG_SPEC;
impl crate::RegisterSpec for PAD_WEAK_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pad_weak_ctrl_reg::R](R) reader structure"]
impl crate::Readable for PAD_WEAK_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_weak_ctrl_reg::W](W) writer structure"]
impl crate::Writable for PAD_WEAK_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_WEAK_CTRL_REG to value 0"]
impl crate::Resettable for PAD_WEAK_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
