#[doc = "Register `VBTOCTLR` reader"]
pub type R = crate::R<VbtoctlrSpec>;
#[doc = "Register `VBTOCTLR` writer"]
pub type W = crate::W<VbtoctlrSpec>;
#[doc = "VBATT Wakeup I/O 0 Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vch0oen {
    #[doc = "0: VBATWIO0 output disabled"]
    _0 = 0,
    #[doc = "1: VBATWIO0 output enabled"]
    _1 = 1,
}
impl From<Vch0oen> for bool {
    #[inline(always)]
    fn from(variant: Vch0oen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCH0OEN` reader - VBATT Wakeup I/O 0 Output Enable"]
pub type Vch0oenR = crate::BitReader<Vch0oen>;
impl Vch0oenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vch0oen {
        match self.bits {
            false => Vch0oen::_0,
            true => Vch0oen::_1,
        }
    }
    #[doc = "VBATWIO0 output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vch0oen::_0
    }
    #[doc = "VBATWIO0 output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vch0oen::_1
    }
}
#[doc = "Field `VCH0OEN` writer - VBATT Wakeup I/O 0 Output Enable"]
pub type Vch0oenW<'a, REG> = crate::BitWriter<'a, REG, Vch0oen>;
impl<'a, REG> Vch0oenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATWIO0 output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vch0oen::_0)
    }
    #[doc = "VBATWIO0 output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vch0oen::_1)
    }
}
#[doc = "VBATT Wakeup I/O 1 Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vch1oen {
    #[doc = "0: VBATWIO1 output disabled"]
    _0 = 0,
    #[doc = "1: VBATWIO1 output enabled"]
    _1 = 1,
}
impl From<Vch1oen> for bool {
    #[inline(always)]
    fn from(variant: Vch1oen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCH1OEN` reader - VBATT Wakeup I/O 1 Output Enable"]
pub type Vch1oenR = crate::BitReader<Vch1oen>;
impl Vch1oenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vch1oen {
        match self.bits {
            false => Vch1oen::_0,
            true => Vch1oen::_1,
        }
    }
    #[doc = "VBATWIO1 output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vch1oen::_0
    }
    #[doc = "VBATWIO1 output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vch1oen::_1
    }
}
#[doc = "Field `VCH1OEN` writer - VBATT Wakeup I/O 1 Output Enable"]
pub type Vch1oenW<'a, REG> = crate::BitWriter<'a, REG, Vch1oen>;
impl<'a, REG> Vch1oenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATWIO1 output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vch1oen::_0)
    }
    #[doc = "VBATWIO1 output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vch1oen::_1)
    }
}
#[doc = "VBATT Wakeup I/O 2 Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vch2oen {
    #[doc = "0: VBATWIO2 output disabled"]
    _0 = 0,
    #[doc = "1: VBATWIO2 output enabled"]
    _1 = 1,
}
impl From<Vch2oen> for bool {
    #[inline(always)]
    fn from(variant: Vch2oen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCH2OEN` reader - VBATT Wakeup I/O 2 Output Enable"]
pub type Vch2oenR = crate::BitReader<Vch2oen>;
impl Vch2oenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vch2oen {
        match self.bits {
            false => Vch2oen::_0,
            true => Vch2oen::_1,
        }
    }
    #[doc = "VBATWIO2 output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vch2oen::_0
    }
    #[doc = "VBATWIO2 output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vch2oen::_1
    }
}
#[doc = "Field `VCH2OEN` writer - VBATT Wakeup I/O 2 Output Enable"]
pub type Vch2oenW<'a, REG> = crate::BitWriter<'a, REG, Vch2oen>;
impl<'a, REG> Vch2oenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATWIO2 output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vch2oen::_0)
    }
    #[doc = "VBATWIO2 output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vch2oen::_1)
    }
}
#[doc = "VBATT Wakeup I/O 0 Output Level Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vout0lsel {
    #[doc = "0: Output L before VBATT wakeup trigger"]
    _0 = 0,
    #[doc = "1: Output H before VBATT wakeup trigger"]
    _1 = 1,
}
impl From<Vout0lsel> for bool {
    #[inline(always)]
    fn from(variant: Vout0lsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOUT0LSEL` reader - VBATT Wakeup I/O 0 Output Level Selection"]
pub type Vout0lselR = crate::BitReader<Vout0lsel>;
impl Vout0lselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vout0lsel {
        match self.bits {
            false => Vout0lsel::_0,
            true => Vout0lsel::_1,
        }
    }
    #[doc = "Output L before VBATT wakeup trigger"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vout0lsel::_0
    }
    #[doc = "Output H before VBATT wakeup trigger"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vout0lsel::_1
    }
}
#[doc = "Field `VOUT0LSEL` writer - VBATT Wakeup I/O 0 Output Level Selection"]
pub type Vout0lselW<'a, REG> = crate::BitWriter<'a, REG, Vout0lsel>;
impl<'a, REG> Vout0lselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output L before VBATT wakeup trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vout0lsel::_0)
    }
    #[doc = "Output H before VBATT wakeup trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vout0lsel::_1)
    }
}
#[doc = "VBATT Wakeup I/O 1 Output Level Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vout1lsel {
    #[doc = "0: Output L before VBATT wake up trigger"]
    _0 = 0,
    #[doc = "1: Output H before VBATT wake up trigger"]
    _1 = 1,
}
impl From<Vout1lsel> for bool {
    #[inline(always)]
    fn from(variant: Vout1lsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOUT1LSEL` reader - VBATT Wakeup I/O 1 Output Level Selection"]
pub type Vout1lselR = crate::BitReader<Vout1lsel>;
impl Vout1lselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vout1lsel {
        match self.bits {
            false => Vout1lsel::_0,
            true => Vout1lsel::_1,
        }
    }
    #[doc = "Output L before VBATT wake up trigger"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vout1lsel::_0
    }
    #[doc = "Output H before VBATT wake up trigger"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vout1lsel::_1
    }
}
#[doc = "Field `VOUT1LSEL` writer - VBATT Wakeup I/O 1 Output Level Selection"]
pub type Vout1lselW<'a, REG> = crate::BitWriter<'a, REG, Vout1lsel>;
impl<'a, REG> Vout1lselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output L before VBATT wake up trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vout1lsel::_0)
    }
    #[doc = "Output H before VBATT wake up trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vout1lsel::_1)
    }
}
#[doc = "VBATT Wakeup I/O 2 Output Level Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vout2lsel {
    #[doc = "0: Output L before VBATT wake up trigger"]
    _0 = 0,
    #[doc = "1: Output H before VBATT wake up trigger"]
    _1 = 1,
}
impl From<Vout2lsel> for bool {
    #[inline(always)]
    fn from(variant: Vout2lsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOUT2LSEL` reader - VBATT Wakeup I/O 2 Output Level Selection"]
pub type Vout2lselR = crate::BitReader<Vout2lsel>;
impl Vout2lselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vout2lsel {
        match self.bits {
            false => Vout2lsel::_0,
            true => Vout2lsel::_1,
        }
    }
    #[doc = "Output L before VBATT wake up trigger"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vout2lsel::_0
    }
    #[doc = "Output H before VBATT wake up trigger"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vout2lsel::_1
    }
}
#[doc = "Field `VOUT2LSEL` writer - VBATT Wakeup I/O 2 Output Level Selection"]
pub type Vout2lselW<'a, REG> = crate::BitWriter<'a, REG, Vout2lsel>;
impl<'a, REG> Vout2lselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output L before VBATT wake up trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vout2lsel::_0)
    }
    #[doc = "Output H before VBATT wake up trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vout2lsel::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATT Wakeup I/O 0 Output Enable"]
    #[inline(always)]
    pub fn vch0oen(&self) -> Vch0oenR {
        Vch0oenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBATT Wakeup I/O 1 Output Enable"]
    #[inline(always)]
    pub fn vch1oen(&self) -> Vch1oenR {
        Vch1oenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBATT Wakeup I/O 2 Output Enable"]
    #[inline(always)]
    pub fn vch2oen(&self) -> Vch2oenR {
        Vch2oenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBATT Wakeup I/O 0 Output Level Selection"]
    #[inline(always)]
    pub fn vout0lsel(&self) -> Vout0lselR {
        Vout0lselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VBATT Wakeup I/O 1 Output Level Selection"]
    #[inline(always)]
    pub fn vout1lsel(&self) -> Vout1lselR {
        Vout1lselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VBATT Wakeup I/O 2 Output Level Selection"]
    #[inline(always)]
    pub fn vout2lsel(&self) -> Vout2lselR {
        Vout2lselR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VBTOCTLR")
            .field("vout2lsel", &self.vout2lsel())
            .field("vout1lsel", &self.vout1lsel())
            .field("vout0lsel", &self.vout0lsel())
            .field("vch2oen", &self.vch2oen())
            .field("vch1oen", &self.vch1oen())
            .field("vch0oen", &self.vch0oen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - VBATT Wakeup I/O 0 Output Enable"]
    #[inline(always)]
    pub fn vch0oen(&mut self) -> Vch0oenW<VbtoctlrSpec> {
        Vch0oenW::new(self, 0)
    }
    #[doc = "Bit 1 - VBATT Wakeup I/O 1 Output Enable"]
    #[inline(always)]
    pub fn vch1oen(&mut self) -> Vch1oenW<VbtoctlrSpec> {
        Vch1oenW::new(self, 1)
    }
    #[doc = "Bit 2 - VBATT Wakeup I/O 2 Output Enable"]
    #[inline(always)]
    pub fn vch2oen(&mut self) -> Vch2oenW<VbtoctlrSpec> {
        Vch2oenW::new(self, 2)
    }
    #[doc = "Bit 3 - VBATT Wakeup I/O 0 Output Level Selection"]
    #[inline(always)]
    pub fn vout0lsel(&mut self) -> Vout0lselW<VbtoctlrSpec> {
        Vout0lselW::new(self, 3)
    }
    #[doc = "Bit 4 - VBATT Wakeup I/O 1 Output Level Selection"]
    #[inline(always)]
    pub fn vout1lsel(&mut self) -> Vout1lselW<VbtoctlrSpec> {
        Vout1lselW::new(self, 4)
    }
    #[doc = "Bit 5 - VBATT Wakeup I/O 2 Output Level Selection"]
    #[inline(always)]
    pub fn vout2lsel(&mut self) -> Vout2lselW<VbtoctlrSpec> {
        Vout2lselW::new(self, 5)
    }
}
#[doc = "VBATT Output Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtoctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtoctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbtoctlrSpec;
impl crate::RegisterSpec for VbtoctlrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtoctlr::R`](R) reader structure"]
impl crate::Readable for VbtoctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbtoctlr::W`](W) writer structure"]
impl crate::Writable for VbtoctlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VBTOCTLR to value 0"]
impl crate::Resettable for VbtoctlrSpec {}
