#[doc = "Register `GTCLR` writer"]
pub type W = crate::W<GtclrSpec>;
#[doc = "Channel 0 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cclr0 {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT320.GTCNT counter clears"]
    _1 = 1,
}
impl From<Cclr0> for bool {
    #[inline(always)]
    fn from(variant: Cclr0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR0` writer - Channel 0 GTCNT Count Clear"]
pub type Cclr0W<'a, REG> = crate::BitWriter<'a, REG, Cclr0>;
impl<'a, REG> Cclr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr0::_0)
    }
    #[doc = "GPT320.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr0::_1)
    }
}
#[doc = "Channel 1 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cclr1 {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT321.GTCNT counter clears"]
    _1 = 1,
}
impl From<Cclr1> for bool {
    #[inline(always)]
    fn from(variant: Cclr1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR1` writer - Channel 1 GTCNT Count Clear"]
pub type Cclr1W<'a, REG> = crate::BitWriter<'a, REG, Cclr1>;
impl<'a, REG> Cclr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr1::_0)
    }
    #[doc = "GPT321.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr1::_1)
    }
}
#[doc = "Channel 2 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cclr2 {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT322.GTCNT counter clears"]
    _1 = 1,
}
impl From<Cclr2> for bool {
    #[inline(always)]
    fn from(variant: Cclr2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR2` writer - Channel 2 GTCNT Count Clear"]
pub type Cclr2W<'a, REG> = crate::BitWriter<'a, REG, Cclr2>;
impl<'a, REG> Cclr2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr2::_0)
    }
    #[doc = "GPT322.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr2::_1)
    }
}
#[doc = "Channel 3 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cclr3 {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT323.GTCNT counter clears"]
    _1 = 1,
}
impl From<Cclr3> for bool {
    #[inline(always)]
    fn from(variant: Cclr3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR3` writer - Channel 3 GTCNT Count Clear"]
pub type Cclr3W<'a, REG> = crate::BitWriter<'a, REG, Cclr3>;
impl<'a, REG> Cclr3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr3::_0)
    }
    #[doc = "GPT323.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr3::_1)
    }
}
#[doc = "Channel 4 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cclr4 {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT164.GTCNT counter clears"]
    _1 = 1,
}
impl From<Cclr4> for bool {
    #[inline(always)]
    fn from(variant: Cclr4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR4` writer - Channel 4 GTCNT Count Clear"]
pub type Cclr4W<'a, REG> = crate::BitWriter<'a, REG, Cclr4>;
impl<'a, REG> Cclr4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr4::_0)
    }
    #[doc = "GPT164.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr4::_1)
    }
}
#[doc = "Channel 5 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cclr5 {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT165.GTCNT counter clears"]
    _1 = 1,
}
impl From<Cclr5> for bool {
    #[inline(always)]
    fn from(variant: Cclr5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR5` writer - Channel 5 GTCNT Count Clear"]
pub type Cclr5W<'a, REG> = crate::BitWriter<'a, REG, Cclr5>;
impl<'a, REG> Cclr5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr5::_0)
    }
    #[doc = "GPT165.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr5::_1)
    }
}
#[doc = "Channel 6 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cclr6 {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT166.GTCNT counter clears"]
    _1 = 1,
}
impl From<Cclr6> for bool {
    #[inline(always)]
    fn from(variant: Cclr6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR6` writer - Channel 6 GTCNT Count Clear"]
pub type Cclr6W<'a, REG> = crate::BitWriter<'a, REG, Cclr6>;
impl<'a, REG> Cclr6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr6::_0)
    }
    #[doc = "GPT166.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr6::_1)
    }
}
#[doc = "Channel 7 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cclr7 {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT167.GTCNT counter clears"]
    _1 = 1,
}
impl From<Cclr7> for bool {
    #[inline(always)]
    fn from(variant: Cclr7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR7` writer - Channel 7 GTCNT Count Clear"]
pub type Cclr7W<'a, REG> = crate::BitWriter<'a, REG, Cclr7>;
impl<'a, REG> Cclr7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr7::_0)
    }
    #[doc = "GPT167.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr7::_1)
    }
}
impl core::fmt::Debug for crate::generic::Reg<GtclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr0(&mut self) -> Cclr0W<GtclrSpec> {
        Cclr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr1(&mut self) -> Cclr1W<GtclrSpec> {
        Cclr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr2(&mut self) -> Cclr2W<GtclrSpec> {
        Cclr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr3(&mut self) -> Cclr3W<GtclrSpec> {
        Cclr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr4(&mut self) -> Cclr4W<GtclrSpec> {
        Cclr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr5(&mut self) -> Cclr5W<GtclrSpec> {
        Cclr5W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr6(&mut self) -> Cclr6W<GtclrSpec> {
        Cclr6W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr7(&mut self) -> Cclr7W<GtclrSpec> {
        Cclr7W::new(self, 7)
    }
}
#[doc = "General PWM Timer Software Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtclrSpec;
impl crate::RegisterSpec for GtclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gtclr::W`](W) writer structure"]
impl crate::Writable for GtclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTCLR to value 0"]
impl crate::Resettable for GtclrSpec {}
