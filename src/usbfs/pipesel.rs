#[doc = "Register `PIPESEL` reader"]
pub type R = crate::R<PipeselSpec>;
#[doc = "Register `PIPESEL` writer"]
pub type W = crate::W<PipeselSpec>;
#[doc = "Pipe Window Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pipesel {
    #[doc = "0: No pipe selected"]
    _0000 = 0,
    #[doc = "1: PIPE1"]
    _0001 = 1,
    #[doc = "2: PIPE2"]
    _0010 = 2,
    #[doc = "3: PIPE3"]
    _0011 = 3,
    #[doc = "4: PIPE4"]
    _0100 = 4,
    #[doc = "5: PIPE5"]
    _0101 = 5,
    #[doc = "6: PIPE6"]
    _0110 = 6,
    #[doc = "7: PIPE7"]
    _0111 = 7,
    #[doc = "8: PIPE8"]
    _1000 = 8,
    #[doc = "9: PIPE9"]
    _1001 = 9,
    #[doc = "10: Settings prohibited."]
    Others = 10,
}
impl From<Pipesel> for u8 {
    #[inline(always)]
    fn from(variant: Pipesel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pipesel {
    type Ux = u8;
}
impl crate::IsEnum for Pipesel {}
#[doc = "Field `PIPESEL` reader - Pipe Window Select"]
pub type PipeselR = crate::FieldReader<Pipesel>;
impl PipeselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipesel {
        match self.bits {
            0 => Pipesel::_0000,
            1 => Pipesel::_0001,
            2 => Pipesel::_0010,
            3 => Pipesel::_0011,
            4 => Pipesel::_0100,
            5 => Pipesel::_0101,
            6 => Pipesel::_0110,
            7 => Pipesel::_0111,
            8 => Pipesel::_1000,
            9 => Pipesel::_1001,
            _ => Pipesel::Others,
        }
    }
    #[doc = "No pipe selected"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Pipesel::_0000
    }
    #[doc = "PIPE1"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == Pipesel::_0001
    }
    #[doc = "PIPE2"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == Pipesel::_0010
    }
    #[doc = "PIPE3"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == Pipesel::_0011
    }
    #[doc = "PIPE4"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == Pipesel::_0100
    }
    #[doc = "PIPE5"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == Pipesel::_0101
    }
    #[doc = "PIPE6"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == Pipesel::_0110
    }
    #[doc = "PIPE7"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == Pipesel::_0111
    }
    #[doc = "PIPE8"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == Pipesel::_1000
    }
    #[doc = "PIPE9"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == Pipesel::_1001
    }
    #[doc = "Settings prohibited."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Pipesel::Others)
    }
}
#[doc = "Field `PIPESEL` writer - Pipe Window Select"]
pub type PipeselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Pipesel, crate::Safe>;
impl<'a, REG> PipeselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pipe selected"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::_0000)
    }
    #[doc = "PIPE1"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::_0001)
    }
    #[doc = "PIPE2"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::_0010)
    }
    #[doc = "PIPE3"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::_0011)
    }
    #[doc = "PIPE4"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::_0100)
    }
    #[doc = "PIPE5"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::_0101)
    }
    #[doc = "PIPE6"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::_0110)
    }
    #[doc = "PIPE7"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::_0111)
    }
    #[doc = "PIPE8"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::_1000)
    }
    #[doc = "PIPE9"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::_1001)
    }
    #[doc = "Settings prohibited."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::Others)
    }
}
impl R {
    #[doc = "Bits 0:3 - Pipe Window Select"]
    #[inline(always)]
    pub fn pipesel(&self) -> PipeselR {
        PipeselR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIPESEL")
            .field("pipesel", &self.pipesel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Pipe Window Select"]
    #[inline(always)]
    pub fn pipesel(&mut self) -> PipeselW<PipeselSpec> {
        PipeselW::new(self, 0)
    }
}
#[doc = "Pipe Window Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pipesel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipesel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PipeselSpec;
impl crate::RegisterSpec for PipeselSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pipesel::R`](R) reader structure"]
impl crate::Readable for PipeselSpec {}
#[doc = "`write(|w| ..)` method takes [`pipesel::W`](W) writer structure"]
impl crate::Writable for PipeselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIPESEL to value 0"]
impl crate::Resettable for PipeselSpec {}
