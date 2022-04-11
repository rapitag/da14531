#[doc = "Register `MEM_CTRL_REG` reader"]
pub struct R(crate::R<MEM_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_CTRL_REG` writer"]
pub struct W(crate::W<MEM_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_CTRL_REG_SPEC>;
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
impl From<crate::W<MEM_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARB2_AHB2_WR_BUFF` reader - "]
pub struct ARB2_AHB2_WR_BUFF_R(crate::FieldReader<bool, bool>);
impl ARB2_AHB2_WR_BUFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARB2_AHB2_WR_BUFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARB2_AHB2_WR_BUFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARB2_AHB_WR_BUFF` reader - "]
pub struct ARB2_AHB_WR_BUFF_R(crate::FieldReader<bool, bool>);
impl ARB2_AHB_WR_BUFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARB2_AHB_WR_BUFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARB2_AHB_WR_BUFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARB1_AHB2_WR_BUFF` reader - "]
pub struct ARB1_AHB2_WR_BUFF_R(crate::FieldReader<bool, bool>);
impl ARB1_AHB2_WR_BUFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARB1_AHB2_WR_BUFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARB1_AHB2_WR_BUFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARB1_AHB_WR_BUFF` reader - "]
pub struct ARB1_AHB_WR_BUFF_R(crate::FieldReader<bool, bool>);
impl ARB1_AHB_WR_BUFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARB1_AHB_WR_BUFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARB1_AHB_WR_BUFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_MARGIN` reader - "]
pub struct RAM_MARGIN_R(crate::FieldReader<u8, u8>);
impl RAM_MARGIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAM_MARGIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_MARGIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_MARGIN` writer - "]
pub struct RAM_MARGIN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_MARGIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u16 & 3) << 6);
        self.w
    }
}
#[doc = "Field `RAM_DST` reader - "]
pub struct RAM_DST_R(crate::FieldReader<bool, bool>);
impl RAM_DST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAM_DST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_DST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_DST` writer - "]
pub struct RAM_DST_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_DST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u16 & 1) << 5);
        self.w
    }
}
#[doc = "Field `ROM_MARGIN_EN` reader - "]
pub struct ROM_MARGIN_EN_R(crate::FieldReader<bool, bool>);
impl ROM_MARGIN_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROM_MARGIN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_MARGIN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_MARGIN_EN` writer - "]
pub struct ROM_MARGIN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_MARGIN_EN_W<'a> {
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
#[doc = "Field `ROM_MARGIN_CTRL` reader - "]
pub struct ROM_MARGIN_CTRL_R(crate::FieldReader<u8, u8>);
impl ROM_MARGIN_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ROM_MARGIN_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_MARGIN_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_MARGIN_CTRL` writer - "]
pub struct ROM_MARGIN_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_MARGIN_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u16 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn arb2_ahb2_wr_buff(&self) -> ARB2_AHB2_WR_BUFF_R {
        ARB2_AHB2_WR_BUFF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn arb2_ahb_wr_buff(&self) -> ARB2_AHB_WR_BUFF_R {
        ARB2_AHB_WR_BUFF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn arb1_ahb2_wr_buff(&self) -> ARB1_AHB2_WR_BUFF_R {
        ARB1_AHB2_WR_BUFF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn arb1_ahb_wr_buff(&self) -> ARB1_AHB_WR_BUFF_R {
        ARB1_AHB_WR_BUFF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn ram_margin(&self) -> RAM_MARGIN_R {
        RAM_MARGIN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ram_dst(&self) -> RAM_DST_R {
        RAM_DST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rom_margin_en(&self) -> ROM_MARGIN_EN_R {
        ROM_MARGIN_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rom_margin_ctrl(&self) -> ROM_MARGIN_CTRL_R {
        ROM_MARGIN_CTRL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn ram_margin(&mut self) -> RAM_MARGIN_W {
        RAM_MARGIN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ram_dst(&mut self) -> RAM_DST_W {
        RAM_DST_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rom_margin_en(&mut self) -> ROM_MARGIN_EN_W {
        ROM_MARGIN_EN_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rom_margin_ctrl(&mut self) -> ROM_MARGIN_CTRL_W {
        ROM_MARGIN_CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_ctrl_reg](index.html) module"]
pub struct MEM_CTRL_REG_SPEC;
impl crate::RegisterSpec for MEM_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mem_ctrl_reg::R](R) reader structure"]
impl crate::Readable for MEM_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_ctrl_reg::W](W) writer structure"]
impl crate::Writable for MEM_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEM_CTRL_REG to value 0x80"]
impl crate::Resettable for MEM_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
