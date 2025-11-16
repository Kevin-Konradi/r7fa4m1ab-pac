#[doc = "Register `VBTICTLR` reader"]
pub type R = crate::R<VbtictlrSpec>;
#[doc = "Register `VBTICTLR` writer"]
pub type W = crate::W<VbtictlrSpec>;
#[doc = "VBATT Wakeup I/O 0 Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vch0inen {
    #[doc = "0: VBATWIO0, RTCIC0 inputs disabled"]
    _0 = 0,
    #[doc = "1: VBATWIO0, RTCIC0 inputs enabled."]
    _1 = 1,
}
impl From<Vch0inen> for bool {
    #[inline(always)]
    fn from(variant: Vch0inen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCH0INEN` reader - VBATT Wakeup I/O 0 Input Enable"]
pub type Vch0inenR = crate::BitReader<Vch0inen>;
impl Vch0inenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vch0inen {
        match self.bits {
            false => Vch0inen::_0,
            true => Vch0inen::_1,
        }
    }
    #[doc = "VBATWIO0, RTCIC0 inputs disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vch0inen::_0
    }
    #[doc = "VBATWIO0, RTCIC0 inputs enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vch0inen::_1
    }
}
#[doc = "Field `VCH0INEN` writer - VBATT Wakeup I/O 0 Input Enable"]
pub type Vch0inenW<'a, REG> = crate::BitWriter<'a, REG, Vch0inen>;
impl<'a, REG> Vch0inenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATWIO0, RTCIC0 inputs disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vch0inen::_0)
    }
    #[doc = "VBATWIO0, RTCIC0 inputs enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vch0inen::_1)
    }
}
#[doc = "VBATT Wakeup I/O 1 Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vch1inen {
    #[doc = "0: VBATWIO1, RTCIC1 inputs disabled"]
    _0 = 0,
    #[doc = "1: VBATWIO1, RTCIC1 inputs enabled."]
    _1 = 1,
}
impl From<Vch1inen> for bool {
    #[inline(always)]
    fn from(variant: Vch1inen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCH1INEN` reader - VBATT Wakeup I/O 1 Input Enable"]
pub type Vch1inenR = crate::BitReader<Vch1inen>;
impl Vch1inenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vch1inen {
        match self.bits {
            false => Vch1inen::_0,
            true => Vch1inen::_1,
        }
    }
    #[doc = "VBATWIO1, RTCIC1 inputs disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vch1inen::_0
    }
    #[doc = "VBATWIO1, RTCIC1 inputs enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vch1inen::_1
    }
}
#[doc = "Field `VCH1INEN` writer - VBATT Wakeup I/O 1 Input Enable"]
pub type Vch1inenW<'a, REG> = crate::BitWriter<'a, REG, Vch1inen>;
impl<'a, REG> Vch1inenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATWIO1, RTCIC1 inputs disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vch1inen::_0)
    }
    #[doc = "VBATWIO1, RTCIC1 inputs enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vch1inen::_1)
    }
}
#[doc = "VBATT Wakeup I/O 2 Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vch2inen {
    #[doc = "0: VBATWIO2 and RTCIC2 inputs disabled"]
    _0 = 0,
    #[doc = "1: VBATWIO2 and RTCIC2 inputs enabled."]
    _1 = 1,
}
impl From<Vch2inen> for bool {
    #[inline(always)]
    fn from(variant: Vch2inen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCH2INEN` reader - VBATT Wakeup I/O 2 Input Enable"]
pub type Vch2inenR = crate::BitReader<Vch2inen>;
impl Vch2inenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vch2inen {
        match self.bits {
            false => Vch2inen::_0,
            true => Vch2inen::_1,
        }
    }
    #[doc = "VBATWIO2 and RTCIC2 inputs disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vch2inen::_0
    }
    #[doc = "VBATWIO2 and RTCIC2 inputs enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vch2inen::_1
    }
}
#[doc = "Field `VCH2INEN` writer - VBATT Wakeup I/O 2 Input Enable"]
pub type Vch2inenW<'a, REG> = crate::BitWriter<'a, REG, Vch2inen>;
impl<'a, REG> Vch2inenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATWIO2 and RTCIC2 inputs disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vch2inen::_0)
    }
    #[doc = "VBATWIO2 and RTCIC2 inputs enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vch2inen::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATT Wakeup I/O 0 Input Enable"]
    #[inline(always)]
    pub fn vch0inen(&self) -> Vch0inenR {
        Vch0inenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBATT Wakeup I/O 1 Input Enable"]
    #[inline(always)]
    pub fn vch1inen(&self) -> Vch1inenR {
        Vch1inenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBATT Wakeup I/O 2 Input Enable"]
    #[inline(always)]
    pub fn vch2inen(&self) -> Vch2inenR {
        Vch2inenR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VBTICTLR")
            .field("vch2inen", &self.vch2inen())
            .field("vch1inen", &self.vch1inen())
            .field("vch0inen", &self.vch0inen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - VBATT Wakeup I/O 0 Input Enable"]
    #[inline(always)]
    pub fn vch0inen(&mut self) -> Vch0inenW<VbtictlrSpec> {
        Vch0inenW::new(self, 0)
    }
    #[doc = "Bit 1 - VBATT Wakeup I/O 1 Input Enable"]
    #[inline(always)]
    pub fn vch1inen(&mut self) -> Vch1inenW<VbtictlrSpec> {
        Vch1inenW::new(self, 1)
    }
    #[doc = "Bit 2 - VBATT Wakeup I/O 2 Input Enable"]
    #[inline(always)]
    pub fn vch2inen(&mut self) -> Vch2inenW<VbtictlrSpec> {
        Vch2inenW::new(self, 2)
    }
}
#[doc = "VBATT Input Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtictlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtictlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbtictlrSpec;
impl crate::RegisterSpec for VbtictlrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtictlr::R`](R) reader structure"]
impl crate::Readable for VbtictlrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbtictlr::W`](W) writer structure"]
impl crate::Writable for VbtictlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VBTICTLR to value 0"]
impl crate::Resettable for VbtictlrSpec {}
