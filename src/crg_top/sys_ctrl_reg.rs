#[doc = "Register `SYS_CTRL_REG` reader"]
pub struct R(crate::R<SYS_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_CTRL_REG` writer"]
pub struct W(crate::W<SYS_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_CTRL_REG_SPEC>;
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
impl From<crate::W<SYS_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_RESET` writer - Writing a '1' to this bit will reset the device, except for: SYS_CTRL_REG CLK_FREQ_TRIM_REG ..."]
pub struct SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u16 & 1) << 15);
        self.w
    }
}
#[doc = "Field `TIMEOUT_DISABLE` reader - Disables timeout in Power statemachine. By default, the statemachine continues if after 2 ms the blocks are not started up. This can be read back from ANA_STATUS_REG."]
pub struct TIMEOUT_DISABLE_R(crate::FieldReader<bool, bool>);
impl TIMEOUT_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMEOUT_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMEOUT_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMEOUT_DISABLE` writer - Disables timeout in Power statemachine. By default, the statemachine continues if after 2 ms the blocks are not started up. This can be read back from ANA_STATUS_REG."]
pub struct TIMEOUT_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_DISABLE_W<'a> {
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
#[doc = "Field `DEBUGGER_ENABLE` reader - Enable the debugger. This bit is set by the booter according to the OTP header. If not set, the SWDIO and SW_CLK can be used as gpio ports. 0x0: no debugger enabled. 0x1: SW_CLK = P0\\[2\\], SW_DIO=P0\\[5\\]
0x2: SW_CLK = P0\\[2\\], SW_DIO=P0\\[1\\]
0x3: SW_CLK = P0\\[2\\], SW_DIO=P0\\[10\\]"]
pub struct DEBUGGER_ENABLE_R(crate::FieldReader<u8, u8>);
impl DEBUGGER_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEBUGGER_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUGGER_ENABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEBUGGER_ENABLE` writer - Enable the debugger. This bit is set by the booter according to the OTP header. If not set, the SWDIO and SW_CLK can be used as gpio ports. 0x0: no debugger enabled. 0x1: SW_CLK = P0\\[2\\], SW_DIO=P0\\[5\\]
0x2: SW_CLK = P0\\[2\\], SW_DIO=P0\\[1\\]
0x3: SW_CLK = P0\\[2\\], SW_DIO=P0\\[10\\]"]
pub struct DEBUGGER_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUGGER_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 7)) | ((value as u16 & 3) << 7);
        self.w
    }
}
#[doc = "Field `OTPC_RESET_REQ` reader - Reset request for the OTP controller."]
pub struct OTPC_RESET_REQ_R(crate::FieldReader<bool, bool>);
impl OTPC_RESET_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OTPC_RESET_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_RESET_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_RESET_REQ` writer - Reset request for the OTP controller."]
pub struct OTPC_RESET_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_RESET_REQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u16 & 1) << 6);
        self.w
    }
}
#[doc = "Field `OTP_COPY` reader - Enables OTP to SysRAM copy action after waking up PD_SYS"]
pub struct OTP_COPY_R(crate::FieldReader<bool, bool>);
impl OTP_COPY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OTP_COPY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTP_COPY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTP_COPY` writer - Enables OTP to SysRAM copy action after waking up PD_SYS"]
pub struct OTP_COPY_W<'a> {
    w: &'a mut W,
}
impl<'a> OTP_COPY_W<'a> {
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
#[doc = "Field `DEV_PHASE` reader - Sets the development phase mode. If this bit is set, in combination with the OTP_COPY bit, the OTP DMA will emulate the OTP mirroring to System RAM. No actual writing to RAM is done, but the exact same amount of time is spend as if the mirroring would take place. This is to mimic the behavior as if the System Code is already in OTP, and the mirroring takes place after waking up, but the (development) code still resides in an external source. If this bit is set to '0' and OTP_COPY='1', then the OTP DMA will actually do the OTP mirroring at wakeup."]
pub struct DEV_PHASE_R(crate::FieldReader<bool, bool>);
impl DEV_PHASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEV_PHASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEV_PHASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEV_PHASE` writer - Sets the development phase mode. If this bit is set, in combination with the OTP_COPY bit, the OTP DMA will emulate the OTP mirroring to System RAM. No actual writing to RAM is done, but the exact same amount of time is spend as if the mirroring would take place. This is to mimic the behavior as if the System Code is already in OTP, and the mirroring takes place after waking up, but the (development) code still resides in an external source. If this bit is set to '0' and OTP_COPY='1', then the OTP DMA will actually do the OTP mirroring at wakeup."]
pub struct DEV_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_PHASE_W<'a> {
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
#[doc = "Field `REMAP_ADR0` reader - Controls which memory is located at address 0x0000 for execution. 0x0: ROM 0x1: OTP 0x2: RAM (SysRAM1) 0x3: RAM (SysRAM3, 28 kBytes offset) This bitfield only takes affect after a Software Reset."]
pub struct REMAP_ADR0_R(crate::FieldReader<u8, u8>);
impl REMAP_ADR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REMAP_ADR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REMAP_ADR0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REMAP_ADR0` writer - Controls which memory is located at address 0x0000 for execution. 0x0: ROM 0x1: OTP 0x2: RAM (SysRAM1) 0x3: RAM (SysRAM3, 28 kBytes offset) This bitfield only takes affect after a Software Reset."]
pub struct REMAP_ADR0_W<'a> {
    w: &'a mut W,
}
impl<'a> REMAP_ADR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u16 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 10 - Disables timeout in Power statemachine. By default, the statemachine continues if after 2 ms the blocks are not started up. This can be read back from ANA_STATUS_REG."]
    #[inline(always)]
    pub fn timeout_disable(&self) -> TIMEOUT_DISABLE_R {
        TIMEOUT_DISABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Enable the debugger. This bit is set by the booter according to the OTP header. If not set, the SWDIO and SW_CLK can be used as gpio ports. 0x0: no debugger enabled. 0x1: SW_CLK = P0\\[2\\], SW_DIO=P0\\[5\\]
0x2: SW_CLK = P0\\[2\\], SW_DIO=P0\\[1\\]
0x3: SW_CLK = P0\\[2\\], SW_DIO=P0\\[10\\]"]
    #[inline(always)]
    pub fn debugger_enable(&self) -> DEBUGGER_ENABLE_R {
        DEBUGGER_ENABLE_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 6 - Reset request for the OTP controller."]
    #[inline(always)]
    pub fn otpc_reset_req(&self) -> OTPC_RESET_REQ_R {
        OTPC_RESET_REQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables OTP to SysRAM copy action after waking up PD_SYS"]
    #[inline(always)]
    pub fn otp_copy(&self) -> OTP_COPY_R {
        OTP_COPY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 2 - Sets the development phase mode. If this bit is set, in combination with the OTP_COPY bit, the OTP DMA will emulate the OTP mirroring to System RAM. No actual writing to RAM is done, but the exact same amount of time is spend as if the mirroring would take place. This is to mimic the behavior as if the System Code is already in OTP, and the mirroring takes place after waking up, but the (development) code still resides in an external source. If this bit is set to '0' and OTP_COPY='1', then the OTP DMA will actually do the OTP mirroring at wakeup."]
    #[inline(always)]
    pub fn dev_phase(&self) -> DEV_PHASE_R {
        DEV_PHASE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 0:1 - Controls which memory is located at address 0x0000 for execution. 0x0: ROM 0x1: OTP 0x2: RAM (SysRAM1) 0x3: RAM (SysRAM3, 28 kBytes offset) This bitfield only takes affect after a Software Reset."]
    #[inline(always)]
    pub fn remap_adr0(&self) -> REMAP_ADR0_R {
        REMAP_ADR0_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - Writing a '1' to this bit will reset the device, except for: SYS_CTRL_REG CLK_FREQ_TRIM_REG ..."]
    #[inline(always)]
    pub fn sw_reset(&mut self) -> SW_RESET_W {
        SW_RESET_W { w: self }
    }
    #[doc = "Bit 10 - Disables timeout in Power statemachine. By default, the statemachine continues if after 2 ms the blocks are not started up. This can be read back from ANA_STATUS_REG."]
    #[inline(always)]
    pub fn timeout_disable(&mut self) -> TIMEOUT_DISABLE_W {
        TIMEOUT_DISABLE_W { w: self }
    }
    #[doc = "Bits 7:8 - Enable the debugger. This bit is set by the booter according to the OTP header. If not set, the SWDIO and SW_CLK can be used as gpio ports. 0x0: no debugger enabled. 0x1: SW_CLK = P0\\[2\\], SW_DIO=P0\\[5\\]
0x2: SW_CLK = P0\\[2\\], SW_DIO=P0\\[1\\]
0x3: SW_CLK = P0\\[2\\], SW_DIO=P0\\[10\\]"]
    #[inline(always)]
    pub fn debugger_enable(&mut self) -> DEBUGGER_ENABLE_W {
        DEBUGGER_ENABLE_W { w: self }
    }
    #[doc = "Bit 6 - Reset request for the OTP controller."]
    #[inline(always)]
    pub fn otpc_reset_req(&mut self) -> OTPC_RESET_REQ_W {
        OTPC_RESET_REQ_W { w: self }
    }
    #[doc = "Bit 4 - Enables OTP to SysRAM copy action after waking up PD_SYS"]
    #[inline(always)]
    pub fn otp_copy(&mut self) -> OTP_COPY_W {
        OTP_COPY_W { w: self }
    }
    #[doc = "Bit 2 - Sets the development phase mode. If this bit is set, in combination with the OTP_COPY bit, the OTP DMA will emulate the OTP mirroring to System RAM. No actual writing to RAM is done, but the exact same amount of time is spend as if the mirroring would take place. This is to mimic the behavior as if the System Code is already in OTP, and the mirroring takes place after waking up, but the (development) code still resides in an external source. If this bit is set to '0' and OTP_COPY='1', then the OTP DMA will actually do the OTP mirroring at wakeup."]
    #[inline(always)]
    pub fn dev_phase(&mut self) -> DEV_PHASE_W {
        DEV_PHASE_W { w: self }
    }
    #[doc = "Bits 0:1 - Controls which memory is located at address 0x0000 for execution. 0x0: ROM 0x1: OTP 0x2: RAM (SysRAM1) 0x3: RAM (SysRAM3, 28 kBytes offset) This bitfield only takes affect after a Software Reset."]
    #[inline(always)]
    pub fn remap_adr0(&mut self) -> REMAP_ADR0_W {
        REMAP_ADR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_ctrl_reg](index.html) module"]
pub struct SYS_CTRL_REG_SPEC;
impl crate::RegisterSpec for SYS_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sys_ctrl_reg::R](R) reader structure"]
impl crate::Readable for SYS_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_ctrl_reg::W](W) writer structure"]
impl crate::Writable for SYS_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_CTRL_REG to value 0x20"]
impl crate::Resettable for SYS_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
