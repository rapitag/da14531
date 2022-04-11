#[doc = "Register `QDEC_CTRL2_REG` reader"]
pub struct R(crate::R<QDEC_CTRL2_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QDEC_CTRL2_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QDEC_CTRL2_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QDEC_CTRL2_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QDEC_CTRL2_REG` writer"]
pub struct W(crate::W<QDEC_CTRL2_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QDEC_CTRL2_REG_SPEC>;
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
impl From<crate::W<QDEC_CTRL2_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QDEC_CTRL2_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QDEC_CHZ_EVENT_MODE` reader - 0 = Normal quadrature counting 1 = Counts rising and falling edge of both ports (if both ports change at the same time, counter increases by 1)"]
pub struct QDEC_CHZ_EVENT_MODE_R(crate::FieldReader<bool, bool>);
impl QDEC_CHZ_EVENT_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QDEC_CHZ_EVENT_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QDEC_CHZ_EVENT_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QDEC_CHZ_EVENT_MODE` writer - 0 = Normal quadrature counting 1 = Counts rising and falling edge of both ports (if both ports change at the same time, counter increases by 1)"]
pub struct QDEC_CHZ_EVENT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> QDEC_CHZ_EVENT_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u16 & 1) << 11);
        self.w
    }
}
#[doc = "Field `QDEC_CHY_EVENT_MODE` reader - 0 = Normal quadrature counting 1 = Counts rising and falling edge of both ports (if both ports change at the same time, counter increases by 1)"]
pub struct QDEC_CHY_EVENT_MODE_R(crate::FieldReader<bool, bool>);
impl QDEC_CHY_EVENT_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QDEC_CHY_EVENT_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QDEC_CHY_EVENT_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QDEC_CHY_EVENT_MODE` writer - 0 = Normal quadrature counting 1 = Counts rising and falling edge of both ports (if both ports change at the same time, counter increases by 1)"]
pub struct QDEC_CHY_EVENT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> QDEC_CHY_EVENT_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u16 & 1) << 10);
        self.w
    }
}
#[doc = "Field `QDEC_CHX_EVENT_MODE` reader - 0 = Normal quadrature counting 1 = Counts rising and falling edge of both ports (if both ports change at the same time, counter increases by 1)"]
pub struct QDEC_CHX_EVENT_MODE_R(crate::FieldReader<bool, bool>);
impl QDEC_CHX_EVENT_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QDEC_CHX_EVENT_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QDEC_CHX_EVENT_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QDEC_CHX_EVENT_MODE` writer - 0 = Normal quadrature counting 1 = Counts rising and falling edge of both ports (if both ports change at the same time, counter increases by 1)"]
pub struct QDEC_CHX_EVENT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> QDEC_CHX_EVENT_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u16 & 1) << 9);
        self.w
    }
}
#[doc = "Field `QDEC_CHZ_PORT_SEL` reader - Defines which GPIOs are mapped on Channel Z 0: none 1: P0\\[2\\]
-> CHZ_A, P0\\[5\\]
-> CHZ_B 2: P0\\[1\\]
-> CHZ_A, P0\\[4\\]
-> CHZ_B 3: P0\\[3\\]
-> CHZ_A, P0\\[10\\]
-> CHZ_B 4: P0\\[6\\]
-> CHZ_A, P0\\[7\\]
-> CHZ_B 5: P0\\[8\\]
-> CHZ_A, P0\\[9\\]
-> CHZ_B 6: P0\\[0\\]
-> CHZ_A, P0\\[11\\]
-> CHZ_B 7: none"]
pub struct QDEC_CHZ_PORT_SEL_R(crate::FieldReader<u8, u8>);
impl QDEC_CHZ_PORT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        QDEC_CHZ_PORT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QDEC_CHZ_PORT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QDEC_CHZ_PORT_SEL` writer - Defines which GPIOs are mapped on Channel Z 0: none 1: P0\\[2\\]
-> CHZ_A, P0\\[5\\]
-> CHZ_B 2: P0\\[1\\]
-> CHZ_A, P0\\[4\\]
-> CHZ_B 3: P0\\[3\\]
-> CHZ_A, P0\\[10\\]
-> CHZ_B 4: P0\\[6\\]
-> CHZ_A, P0\\[7\\]
-> CHZ_B 5: P0\\[8\\]
-> CHZ_A, P0\\[9\\]
-> CHZ_B 6: P0\\[0\\]
-> CHZ_A, P0\\[11\\]
-> CHZ_B 7: none"]
pub struct QDEC_CHZ_PORT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QDEC_CHZ_PORT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 6)) | ((value as u16 & 7) << 6);
        self.w
    }
}
#[doc = "Field `QDEC_CHY_PORT_SEL` reader - Defines which GPIOs are mapped on Channel Y 0: none 1: P0\\[2\\]
-> CHY_A, P0\\[5\\]
-> CHY_B 2: P0\\[1\\]
-> CHY_A, P0\\[4\\]
-> CHY_B 3: P0\\[3\\]
-> CHY_A, P0\\[10\\]
-> CHY_B 4: P0\\[6\\]
-> CHY_A, P0\\[7\\]
-> CHY_B 5: P0\\[8\\]
-> CHY_A, P0\\[9\\]
-> CHY_B 6: P0\\[0\\]
-> CHY_A, P0\\[11\\]
-> CHY_B 7: none"]
pub struct QDEC_CHY_PORT_SEL_R(crate::FieldReader<u8, u8>);
impl QDEC_CHY_PORT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        QDEC_CHY_PORT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QDEC_CHY_PORT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QDEC_CHY_PORT_SEL` writer - Defines which GPIOs are mapped on Channel Y 0: none 1: P0\\[2\\]
-> CHY_A, P0\\[5\\]
-> CHY_B 2: P0\\[1\\]
-> CHY_A, P0\\[4\\]
-> CHY_B 3: P0\\[3\\]
-> CHY_A, P0\\[10\\]
-> CHY_B 4: P0\\[6\\]
-> CHY_A, P0\\[7\\]
-> CHY_B 5: P0\\[8\\]
-> CHY_A, P0\\[9\\]
-> CHY_B 6: P0\\[0\\]
-> CHY_A, P0\\[11\\]
-> CHY_B 7: none"]
pub struct QDEC_CHY_PORT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QDEC_CHY_PORT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 3)) | ((value as u16 & 7) << 3);
        self.w
    }
}
#[doc = "Field `QDEC_CHX_PORT_SEL` reader - Defines which GPIOs are mapped on Channel X 0: none 1: P0\\[2\\]
-> CHX_A, P0\\[5\\]
-> CHX_B 2: P0\\[1\\]
-> CHX_A, P0\\[4\\]
-> CHX_B 3: P0\\[3\\]
-> CHX_A, P0\\[10\\]
-> CHX_B 4: P0\\[6\\]
-> CHX_A, P0\\[7\\]
-> CHX_B 5: P0\\[8\\]
-> CHX_A, P0\\[9\\]
-> CHX_B 6: P0\\[0\\]
-> CHX_A, P0\\[11\\]
-> CHX_B 7: none"]
pub struct QDEC_CHX_PORT_SEL_R(crate::FieldReader<u8, u8>);
impl QDEC_CHX_PORT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        QDEC_CHX_PORT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QDEC_CHX_PORT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QDEC_CHX_PORT_SEL` writer - Defines which GPIOs are mapped on Channel X 0: none 1: P0\\[2\\]
-> CHX_A, P0\\[5\\]
-> CHX_B 2: P0\\[1\\]
-> CHX_A, P0\\[4\\]
-> CHX_B 3: P0\\[3\\]
-> CHX_A, P0\\[10\\]
-> CHX_B 4: P0\\[6\\]
-> CHX_A, P0\\[7\\]
-> CHX_B 5: P0\\[8\\]
-> CHX_A, P0\\[9\\]
-> CHX_B 6: P0\\[0\\]
-> CHX_A, P0\\[11\\]
-> CHX_B 7: none"]
pub struct QDEC_CHX_PORT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QDEC_CHX_PORT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u16 & 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - 0 = Normal quadrature counting 1 = Counts rising and falling edge of both ports (if both ports change at the same time, counter increases by 1)"]
    #[inline(always)]
    pub fn qdec_chz_event_mode(&self) -> QDEC_CHZ_EVENT_MODE_R {
        QDEC_CHZ_EVENT_MODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - 0 = Normal quadrature counting 1 = Counts rising and falling edge of both ports (if both ports change at the same time, counter increases by 1)"]
    #[inline(always)]
    pub fn qdec_chy_event_mode(&self) -> QDEC_CHY_EVENT_MODE_R {
        QDEC_CHY_EVENT_MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - 0 = Normal quadrature counting 1 = Counts rising and falling edge of both ports (if both ports change at the same time, counter increases by 1)"]
    #[inline(always)]
    pub fn qdec_chx_event_mode(&self) -> QDEC_CHX_EVENT_MODE_R {
        QDEC_CHX_EVENT_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Defines which GPIOs are mapped on Channel Z 0: none 1: P0\\[2\\]
-> CHZ_A, P0\\[5\\]
-> CHZ_B 2: P0\\[1\\]
-> CHZ_A, P0\\[4\\]
-> CHZ_B 3: P0\\[3\\]
-> CHZ_A, P0\\[10\\]
-> CHZ_B 4: P0\\[6\\]
-> CHZ_A, P0\\[7\\]
-> CHZ_B 5: P0\\[8\\]
-> CHZ_A, P0\\[9\\]
-> CHZ_B 6: P0\\[0\\]
-> CHZ_A, P0\\[11\\]
-> CHZ_B 7: none"]
    #[inline(always)]
    pub fn qdec_chz_port_sel(&self) -> QDEC_CHZ_PORT_SEL_R {
        QDEC_CHZ_PORT_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 3:5 - Defines which GPIOs are mapped on Channel Y 0: none 1: P0\\[2\\]
-> CHY_A, P0\\[5\\]
-> CHY_B 2: P0\\[1\\]
-> CHY_A, P0\\[4\\]
-> CHY_B 3: P0\\[3\\]
-> CHY_A, P0\\[10\\]
-> CHY_B 4: P0\\[6\\]
-> CHY_A, P0\\[7\\]
-> CHY_B 5: P0\\[8\\]
-> CHY_A, P0\\[9\\]
-> CHY_B 6: P0\\[0\\]
-> CHY_A, P0\\[11\\]
-> CHY_B 7: none"]
    #[inline(always)]
    pub fn qdec_chy_port_sel(&self) -> QDEC_CHY_PORT_SEL_R {
        QDEC_CHY_PORT_SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 0:2 - Defines which GPIOs are mapped on Channel X 0: none 1: P0\\[2\\]
-> CHX_A, P0\\[5\\]
-> CHX_B 2: P0\\[1\\]
-> CHX_A, P0\\[4\\]
-> CHX_B 3: P0\\[3\\]
-> CHX_A, P0\\[10\\]
-> CHX_B 4: P0\\[6\\]
-> CHX_A, P0\\[7\\]
-> CHX_B 5: P0\\[8\\]
-> CHX_A, P0\\[9\\]
-> CHX_B 6: P0\\[0\\]
-> CHX_A, P0\\[11\\]
-> CHX_B 7: none"]
    #[inline(always)]
    pub fn qdec_chx_port_sel(&self) -> QDEC_CHX_PORT_SEL_R {
        QDEC_CHX_PORT_SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 11 - 0 = Normal quadrature counting 1 = Counts rising and falling edge of both ports (if both ports change at the same time, counter increases by 1)"]
    #[inline(always)]
    pub fn qdec_chz_event_mode(&mut self) -> QDEC_CHZ_EVENT_MODE_W {
        QDEC_CHZ_EVENT_MODE_W { w: self }
    }
    #[doc = "Bit 10 - 0 = Normal quadrature counting 1 = Counts rising and falling edge of both ports (if both ports change at the same time, counter increases by 1)"]
    #[inline(always)]
    pub fn qdec_chy_event_mode(&mut self) -> QDEC_CHY_EVENT_MODE_W {
        QDEC_CHY_EVENT_MODE_W { w: self }
    }
    #[doc = "Bit 9 - 0 = Normal quadrature counting 1 = Counts rising and falling edge of both ports (if both ports change at the same time, counter increases by 1)"]
    #[inline(always)]
    pub fn qdec_chx_event_mode(&mut self) -> QDEC_CHX_EVENT_MODE_W {
        QDEC_CHX_EVENT_MODE_W { w: self }
    }
    #[doc = "Bits 6:8 - Defines which GPIOs are mapped on Channel Z 0: none 1: P0\\[2\\]
-> CHZ_A, P0\\[5\\]
-> CHZ_B 2: P0\\[1\\]
-> CHZ_A, P0\\[4\\]
-> CHZ_B 3: P0\\[3\\]
-> CHZ_A, P0\\[10\\]
-> CHZ_B 4: P0\\[6\\]
-> CHZ_A, P0\\[7\\]
-> CHZ_B 5: P0\\[8\\]
-> CHZ_A, P0\\[9\\]
-> CHZ_B 6: P0\\[0\\]
-> CHZ_A, P0\\[11\\]
-> CHZ_B 7: none"]
    #[inline(always)]
    pub fn qdec_chz_port_sel(&mut self) -> QDEC_CHZ_PORT_SEL_W {
        QDEC_CHZ_PORT_SEL_W { w: self }
    }
    #[doc = "Bits 3:5 - Defines which GPIOs are mapped on Channel Y 0: none 1: P0\\[2\\]
-> CHY_A, P0\\[5\\]
-> CHY_B 2: P0\\[1\\]
-> CHY_A, P0\\[4\\]
-> CHY_B 3: P0\\[3\\]
-> CHY_A, P0\\[10\\]
-> CHY_B 4: P0\\[6\\]
-> CHY_A, P0\\[7\\]
-> CHY_B 5: P0\\[8\\]
-> CHY_A, P0\\[9\\]
-> CHY_B 6: P0\\[0\\]
-> CHY_A, P0\\[11\\]
-> CHY_B 7: none"]
    #[inline(always)]
    pub fn qdec_chy_port_sel(&mut self) -> QDEC_CHY_PORT_SEL_W {
        QDEC_CHY_PORT_SEL_W { w: self }
    }
    #[doc = "Bits 0:2 - Defines which GPIOs are mapped on Channel X 0: none 1: P0\\[2\\]
-> CHX_A, P0\\[5\\]
-> CHX_B 2: P0\\[1\\]
-> CHX_A, P0\\[4\\]
-> CHX_B 3: P0\\[3\\]
-> CHX_A, P0\\[10\\]
-> CHX_B 4: P0\\[6\\]
-> CHX_A, P0\\[7\\]
-> CHX_B 5: P0\\[8\\]
-> CHX_A, P0\\[9\\]
-> CHX_B 6: P0\\[0\\]
-> CHX_A, P0\\[11\\]
-> CHX_B 7: none"]
    #[inline(always)]
    pub fn qdec_chx_port_sel(&mut self) -> QDEC_CHX_PORT_SEL_W {
        QDEC_CHX_PORT_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Quad Decoder port selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qdec_ctrl2_reg](index.html) module"]
pub struct QDEC_CTRL2_REG_SPEC;
impl crate::RegisterSpec for QDEC_CTRL2_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [qdec_ctrl2_reg::R](R) reader structure"]
impl crate::Readable for QDEC_CTRL2_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qdec_ctrl2_reg::W](W) writer structure"]
impl crate::Writable for QDEC_CTRL2_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QDEC_CTRL2_REG to value 0x0ed1"]
impl crate::Resettable for QDEC_CTRL2_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0ed1
    }
}
