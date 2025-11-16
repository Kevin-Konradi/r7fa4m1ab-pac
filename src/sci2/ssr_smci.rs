#[doc = "Register `SSR_SMCI` reader"]
pub type R = crate::R<SsrSmciSpec>;
#[doc = "Register `SSR_SMCI` writer"]
pub type W = crate::W<SsrSmciSpec>;
#[doc = "Field `MPBT` reader - Multi-Processor Bit Transfer This bit should be 0 in smart card interface mode."]
pub type MpbtR = crate::BitReader;
#[doc = "Field `MPBT` writer - Multi-Processor Bit Transfer This bit should be 0 in smart card interface mode."]
pub type MpbtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPB` reader - Multi-Processor This bit should be 0 in smart card interface mode."]
pub type MpbR = crate::BitReader;
#[doc = "Transmit End Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tend {
    #[doc = "0: A character is being transmitted."]
    _0 = 0,
    #[doc = "1: Character transfer has been completed."]
    _1 = 1,
}
impl From<Tend> for bool {
    #[inline(always)]
    fn from(variant: Tend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEND` reader - Transmit End Flag"]
pub type TendR = crate::BitReader<Tend>;
impl TendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tend {
        match self.bits {
            false => Tend::_0,
            true => Tend::_1,
        }
    }
    #[doc = "A character is being transmitted."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tend::_0
    }
    #[doc = "Character transfer has been completed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tend::_1
    }
}
#[doc = "Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Per {
    #[doc = "0: No parity error occurred"]
    _0 = 0,
    #[doc = "1: A parity error has occurred"]
    _1 = 1,
}
impl From<Per> for bool {
    #[inline(always)]
    fn from(variant: Per) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PER` reader - Parity Error Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type PerR = crate::BitReader<Per>;
impl PerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Per {
        match self.bits {
            false => Per::_0,
            true => Per::_1,
        }
    }
    #[doc = "No parity error occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Per::_0
    }
    #[doc = "A parity error has occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Per::_1
    }
}
#[doc = "Field `PER` writer - Parity Error Flag"]
pub type PerW<'a, REG> = crate::BitWriter0C<'a, REG, Per>;
impl<'a, REG> PerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Per::_0)
    }
    #[doc = "A parity error has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Per::_1)
    }
}
#[doc = "Error Signal Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ers {
    #[doc = "0: Low error signal not responded"]
    _0 = 0,
    #[doc = "1: Low error signal responded"]
    _1 = 1,
}
impl From<Ers> for bool {
    #[inline(always)]
    fn from(variant: Ers) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERS` reader - Error Signal Status Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type ErsR = crate::BitReader<Ers>;
impl ErsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ers {
        match self.bits {
            false => Ers::_0,
            true => Ers::_1,
        }
    }
    #[doc = "Low error signal not responded"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ers::_0
    }
    #[doc = "Low error signal responded"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ers::_1
    }
}
#[doc = "Field `ERS` writer - Error Signal Status Flag"]
pub type ErsW<'a, REG> = crate::BitWriter0C<'a, REG, Ers>;
impl<'a, REG> ErsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low error signal not responded"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ers::_0)
    }
    #[doc = "Low error signal responded"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ers::_1)
    }
}
#[doc = "Overrun Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orer {
    #[doc = "0: No overrun error occurred"]
    _0 = 0,
    #[doc = "1: An overrun error has occurred"]
    _1 = 1,
}
impl From<Orer> for bool {
    #[inline(always)]
    fn from(variant: Orer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ORER` reader - Overrun Error Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type OrerR = crate::BitReader<Orer>;
impl OrerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Orer {
        match self.bits {
            false => Orer::_0,
            true => Orer::_1,
        }
    }
    #[doc = "No overrun error occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Orer::_0
    }
    #[doc = "An overrun error has occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Orer::_1
    }
}
#[doc = "Field `ORER` writer - Overrun Error Flag"]
pub type OrerW<'a, REG> = crate::BitWriter0C<'a, REG, Orer>;
impl<'a, REG> OrerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overrun error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Orer::_0)
    }
    #[doc = "An overrun error has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Orer::_1)
    }
}
#[doc = "Receive Data Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdrf {
    #[doc = "0: No received data is in RDR register"]
    _0 = 0,
    #[doc = "1: Received data is in RDR register"]
    _1 = 1,
}
impl From<Rdrf> for bool {
    #[inline(always)]
    fn from(variant: Rdrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDRF` reader - Receive Data Full Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type RdrfR = crate::BitReader<Rdrf>;
impl RdrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdrf {
        match self.bits {
            false => Rdrf::_0,
            true => Rdrf::_1,
        }
    }
    #[doc = "No received data is in RDR register"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rdrf::_0
    }
    #[doc = "Received data is in RDR register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rdrf::_1
    }
}
#[doc = "Field `RDRF` writer - Receive Data Full Flag"]
pub type RdrfW<'a, REG> = crate::BitWriter0C<'a, REG, Rdrf>;
impl<'a, REG> RdrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No received data is in RDR register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdrf::_0)
    }
    #[doc = "Received data is in RDR register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdrf::_1)
    }
}
#[doc = "Transmit Data Empty Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdre {
    #[doc = "0: Transmit data is in TDR register"]
    _0 = 0,
    #[doc = "1: No transmit data is in TDR register"]
    _1 = 1,
}
impl From<Tdre> for bool {
    #[inline(always)]
    fn from(variant: Tdre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDRE` reader - Transmit Data Empty Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type TdreR = crate::BitReader<Tdre>;
impl TdreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdre {
        match self.bits {
            false => Tdre::_0,
            true => Tdre::_1,
        }
    }
    #[doc = "Transmit data is in TDR register"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tdre::_0
    }
    #[doc = "No transmit data is in TDR register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tdre::_1
    }
}
#[doc = "Field `TDRE` writer - Transmit Data Empty Flag"]
pub type TdreW<'a, REG> = crate::BitWriter0C<'a, REG, Tdre>;
impl<'a, REG> TdreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit data is in TDR register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tdre::_0)
    }
    #[doc = "No transmit data is in TDR register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tdre::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Multi-Processor Bit Transfer This bit should be 0 in smart card interface mode."]
    #[inline(always)]
    pub fn mpbt(&self) -> MpbtR {
        MpbtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Multi-Processor This bit should be 0 in smart card interface mode."]
    #[inline(always)]
    pub fn mpb(&self) -> MpbR {
        MpbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit End Flag"]
    #[inline(always)]
    pub fn tend(&self) -> TendR {
        TendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Error Flag"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Error Signal Status Flag"]
    #[inline(always)]
    pub fn ers(&self) -> ErsR {
        ErsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error Flag"]
    #[inline(always)]
    pub fn orer(&self) -> OrerR {
        OrerR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Data Full Flag"]
    #[inline(always)]
    pub fn rdrf(&self) -> RdrfR {
        RdrfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Data Empty Flag"]
    #[inline(always)]
    pub fn tdre(&self) -> TdreR {
        TdreR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSR_SMCI")
            .field("tend", &self.tend())
            .field("mpb", &self.mpb())
            .field("mpbt", &self.mpbt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Multi-Processor Bit Transfer This bit should be 0 in smart card interface mode."]
    #[inline(always)]
    pub fn mpbt(&mut self) -> MpbtW<SsrSmciSpec> {
        MpbtW::new(self, 0)
    }
    #[doc = "Bit 3 - Parity Error Flag"]
    #[inline(always)]
    pub fn per(&mut self) -> PerW<SsrSmciSpec> {
        PerW::new(self, 3)
    }
    #[doc = "Bit 4 - Error Signal Status Flag"]
    #[inline(always)]
    pub fn ers(&mut self) -> ErsW<SsrSmciSpec> {
        ErsW::new(self, 4)
    }
    #[doc = "Bit 5 - Overrun Error Flag"]
    #[inline(always)]
    pub fn orer(&mut self) -> OrerW<SsrSmciSpec> {
        OrerW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Data Full Flag"]
    #[inline(always)]
    pub fn rdrf(&mut self) -> RdrfW<SsrSmciSpec> {
        RdrfW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Data Empty Flag"]
    #[inline(always)]
    pub fn tdre(&mut self) -> TdreW<SsrSmciSpec> {
        TdreW::new(self, 7)
    }
}
#[doc = "Serial Status Register(SCMR.SMIF = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ssr_smci::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssr_smci::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsrSmciSpec;
impl crate::RegisterSpec for SsrSmciSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ssr_smci::R`](R) reader structure"]
impl crate::Readable for SsrSmciSpec {}
#[doc = "`write(|w| ..)` method takes [`ssr_smci::W`](W) writer structure"]
impl crate::Writable for SsrSmciSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0xf8;
}
#[doc = "`reset()` method sets SSR_SMCI to value 0x84"]
impl crate::Resettable for SsrSmciSpec {
    const RESET_VALUE: u8 = 0x84;
}
