#[doc = "Register `ICSR2` reader"]
pub type R = crate::R<Icsr2Spec>;
#[doc = "Register `ICSR2` writer"]
pub type W = crate::W<Icsr2Spec>;
#[doc = "Timeout Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmof {
    #[doc = "0: Timeout is not detected."]
    _0 = 0,
    #[doc = "1: Timeout is detected."]
    _1 = 1,
}
impl From<Tmof> for bool {
    #[inline(always)]
    fn from(variant: Tmof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMOF` reader - Timeout Detection Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type TmofR = crate::BitReader<Tmof>;
impl TmofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmof {
        match self.bits {
            false => Tmof::_0,
            true => Tmof::_1,
        }
    }
    #[doc = "Timeout is not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmof::_0
    }
    #[doc = "Timeout is detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmof::_1
    }
}
#[doc = "Field `TMOF` writer - Timeout Detection Flag"]
pub type TmofW<'a, REG> = crate::BitWriter0C<'a, REG, Tmof>;
impl<'a, REG> TmofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timeout is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmof::_0)
    }
    #[doc = "Timeout is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmof::_1)
    }
}
#[doc = "Arbitration-Lost Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Al {
    #[doc = "0: Arbitration is not lost."]
    _0 = 0,
    #[doc = "1: Arbitration is lost."]
    _1 = 1,
}
impl From<Al> for bool {
    #[inline(always)]
    fn from(variant: Al) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AL` reader - Arbitration-Lost Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type AlR = crate::BitReader<Al>;
impl AlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Al {
        match self.bits {
            false => Al::_0,
            true => Al::_1,
        }
    }
    #[doc = "Arbitration is not lost."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Al::_0
    }
    #[doc = "Arbitration is lost."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Al::_1
    }
}
#[doc = "Field `AL` writer - Arbitration-Lost Flag"]
pub type AlW<'a, REG> = crate::BitWriter0C<'a, REG, Al>;
impl<'a, REG> AlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Arbitration is not lost."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Al::_0)
    }
    #[doc = "Arbitration is lost."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Al::_1)
    }
}
#[doc = "Start Condition Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "0: Start condition is not detected."]
    _0 = 0,
    #[doc = "1: Start condition is detected."]
    _1 = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Start Condition Detection Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type StartR = crate::BitReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start {
        match self.bits {
            false => Start::_0,
            true => Start::_1,
        }
    }
    #[doc = "Start condition is not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Start::_0
    }
    #[doc = "Start condition is detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Start::_1
    }
}
#[doc = "Field `START` writer - Start Condition Detection Flag"]
pub type StartW<'a, REG> = crate::BitWriter0C<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start condition is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Start::_0)
    }
    #[doc = "Start condition is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Start::_1)
    }
}
#[doc = "Stop Condition Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    #[doc = "0: Stop condition is not detected."]
    _0 = 0,
    #[doc = "1: Stop condition is detected."]
    _1 = 1,
}
impl From<Stop> for bool {
    #[inline(always)]
    fn from(variant: Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - Stop Condition Detection Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type StopR = crate::BitReader<Stop>;
impl StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop {
        match self.bits {
            false => Stop::_0,
            true => Stop::_1,
        }
    }
    #[doc = "Stop condition is not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Stop::_0
    }
    #[doc = "Stop condition is detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Stop::_1
    }
}
#[doc = "Field `STOP` writer - Stop Condition Detection Flag"]
pub type StopW<'a, REG> = crate::BitWriter0C<'a, REG, Stop>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop condition is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::_0)
    }
    #[doc = "Stop condition is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::_1)
    }
}
#[doc = "NACK Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nackf {
    #[doc = "0: NACK is not detected."]
    _0 = 0,
    #[doc = "1: NACK is detected."]
    _1 = 1,
}
impl From<Nackf> for bool {
    #[inline(always)]
    fn from(variant: Nackf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKF` reader - NACK Detection Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type NackfR = crate::BitReader<Nackf>;
impl NackfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nackf {
        match self.bits {
            false => Nackf::_0,
            true => Nackf::_1,
        }
    }
    #[doc = "NACK is not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nackf::_0
    }
    #[doc = "NACK is detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nackf::_1
    }
}
#[doc = "Field `NACKF` writer - NACK Detection Flag"]
pub type NackfW<'a, REG> = crate::BitWriter0C<'a, REG, Nackf>;
impl<'a, REG> NackfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NACK is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nackf::_0)
    }
    #[doc = "NACK is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nackf::_1)
    }
}
#[doc = "Receive Data Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdrf {
    #[doc = "0: ICDRR contains no receive data."]
    _0 = 0,
    #[doc = "1: ICDRR contains receive data."]
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
    #[doc = "ICDRR contains no receive data."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rdrf::_0
    }
    #[doc = "ICDRR contains receive data."]
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
    #[doc = "ICDRR contains no receive data."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdrf::_0)
    }
    #[doc = "ICDRR contains receive data."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdrf::_1)
    }
}
#[doc = "Transmit End Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tend {
    #[doc = "0: Data is being transmitted."]
    _0 = 0,
    #[doc = "1: Data has been transmitted."]
    _1 = 1,
}
impl From<Tend> for bool {
    #[inline(always)]
    fn from(variant: Tend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEND` reader - Transmit End Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
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
    #[doc = "Data is being transmitted."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tend::_0
    }
    #[doc = "Data has been transmitted."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tend::_1
    }
}
#[doc = "Field `TEND` writer - Transmit End Flag"]
pub type TendW<'a, REG> = crate::BitWriter0C<'a, REG, Tend>;
impl<'a, REG> TendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is being transmitted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tend::_0)
    }
    #[doc = "Data has been transmitted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tend::_1)
    }
}
#[doc = "Transmit Data Empty Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdre {
    #[doc = "0: ICDRT contains transmit data."]
    _0 = 0,
    #[doc = "1: ICDRT contains no transmit data."]
    _1 = 1,
}
impl From<Tdre> for bool {
    #[inline(always)]
    fn from(variant: Tdre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDRE` reader - Transmit Data Empty Flag"]
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
    #[doc = "ICDRT contains transmit data."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tdre::_0
    }
    #[doc = "ICDRT contains no transmit data."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tdre::_1
    }
}
impl R {
    #[doc = "Bit 0 - Timeout Detection Flag"]
    #[inline(always)]
    pub fn tmof(&self) -> TmofR {
        TmofR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Arbitration-Lost Flag"]
    #[inline(always)]
    pub fn al(&self) -> AlR {
        AlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start Condition Detection Flag"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stop Condition Detection Flag"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NACK Detection Flag"]
    #[inline(always)]
    pub fn nackf(&self) -> NackfR {
        NackfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Data Full Flag"]
    #[inline(always)]
    pub fn rdrf(&self) -> RdrfR {
        RdrfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit End Flag"]
    #[inline(always)]
    pub fn tend(&self) -> TendR {
        TendR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Data Empty Flag"]
    #[inline(always)]
    pub fn tdre(&self) -> TdreR {
        TdreR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICSR2").field("tdre", &self.tdre()).finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timeout Detection Flag"]
    #[inline(always)]
    pub fn tmof(&mut self) -> TmofW<Icsr2Spec> {
        TmofW::new(self, 0)
    }
    #[doc = "Bit 1 - Arbitration-Lost Flag"]
    #[inline(always)]
    pub fn al(&mut self) -> AlW<Icsr2Spec> {
        AlW::new(self, 1)
    }
    #[doc = "Bit 2 - Start Condition Detection Flag"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<Icsr2Spec> {
        StartW::new(self, 2)
    }
    #[doc = "Bit 3 - Stop Condition Detection Flag"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<Icsr2Spec> {
        StopW::new(self, 3)
    }
    #[doc = "Bit 4 - NACK Detection Flag"]
    #[inline(always)]
    pub fn nackf(&mut self) -> NackfW<Icsr2Spec> {
        NackfW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive Data Full Flag"]
    #[inline(always)]
    pub fn rdrf(&mut self) -> RdrfW<Icsr2Spec> {
        RdrfW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit End Flag"]
    #[inline(always)]
    pub fn tend(&mut self) -> TendW<Icsr2Spec> {
        TendW::new(self, 6)
    }
}
#[doc = "I2C Bus Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`icsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icsr2Spec;
impl crate::RegisterSpec for Icsr2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icsr2::R`](R) reader structure"]
impl crate::Readable for Icsr2Spec {}
#[doc = "`write(|w| ..)` method takes [`icsr2::W`](W) writer structure"]
impl crate::Writable for Icsr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x7f;
}
#[doc = "`reset()` method sets ICSR2 to value 0"]
impl crate::Resettable for Icsr2Spec {}
