#[doc = "Register `I2C_DMA_CR_REG` reader"]
pub struct R(crate::R<I2C_DMA_CR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_DMA_CR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_DMA_CR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_DMA_CR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_DMA_CR_REG` writer"]
pub struct W(crate::W<I2C_DMA_CR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_DMA_CR_REG_SPEC>;
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
impl From<crate::W<I2C_DMA_CR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_DMA_CR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDMAE` reader - Transmit DMA Enable. //This bit enables/disables the transmit FIFO DMA channel. 0 = Transmit DMA disabled 1 = Transmit DMA enabled"]
pub struct TDMAE_R(crate::FieldReader<bool, bool>);
impl TDMAE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TDMAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDMAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDMAE` writer - Transmit DMA Enable. //This bit enables/disables the transmit FIFO DMA channel. 0 = Transmit DMA disabled 1 = Transmit DMA enabled"]
pub struct TDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDMAE_W<'a> {
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
#[doc = "Field `RDMAE` reader - Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. 0 = Receive DMA disabled 1 = Receive DMA enabled"]
pub struct RDMAE_R(crate::FieldReader<bool, bool>);
impl RDMAE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDMAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDMAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDMAE` writer - Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. 0 = Receive DMA disabled 1 = Receive DMA enabled"]
pub struct RDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDMAE_W<'a> {
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
    #[doc = "Bit 1 - Transmit DMA Enable. //This bit enables/disables the transmit FIFO DMA channel. 0 = Transmit DMA disabled 1 = Transmit DMA enabled"]
    #[inline(always)]
    pub fn tdmae(&self) -> TDMAE_R {
        TDMAE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. 0 = Receive DMA disabled 1 = Receive DMA enabled"]
    #[inline(always)]
    pub fn rdmae(&self) -> RDMAE_R {
        RDMAE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Transmit DMA Enable. //This bit enables/disables the transmit FIFO DMA channel. 0 = Transmit DMA disabled 1 = Transmit DMA enabled"]
    #[inline(always)]
    pub fn tdmae(&mut self) -> TDMAE_W {
        TDMAE_W { w: self }
    }
    #[doc = "Bit 0 - Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. 0 = Receive DMA disabled 1 = Receive DMA enabled"]
    #[inline(always)]
    pub fn rdmae(&mut self) -> RDMAE_W {
        RDMAE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_dma_cr_reg](index.html) module"]
pub struct I2C_DMA_CR_REG_SPEC;
impl crate::RegisterSpec for I2C_DMA_CR_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [i2c_dma_cr_reg::R](R) reader structure"]
impl crate::Readable for I2C_DMA_CR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_dma_cr_reg::W](W) writer structure"]
impl crate::Writable for I2C_DMA_CR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_DMA_CR_REG to value 0"]
impl crate::Resettable for I2C_DMA_CR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
