#[doc = "Register `CTSUSSC` reader"]
pub type R = crate::R<CtsusscSpec>;
#[doc = "Register `CTSUSSC` writer"]
pub type W = crate::W<CtsusscSpec>;
#[doc = "CTSU Spectrum Diffusion Frequency Division Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsussdiv {
    #[doc = "0: 4.00 <= fb"]
    _0000 = 0,
    #[doc = "1: 2.00 <= fb < 4.00"]
    _0001 = 1,
    #[doc = "2: 1.33 <= fb < 2.00"]
    _0010 = 2,
    #[doc = "3: 1.00 <= fb < 1.33"]
    _0011 = 3,
    #[doc = "4: 0.80 <= fb < 1.00"]
    _0100 = 4,
    #[doc = "5: 0.67 <= fb < 0.80"]
    _0101 = 5,
    #[doc = "6: 0.57 <= fb < 0.67"]
    _0110 = 6,
    #[doc = "7: 0.50 <= fb < 0.57"]
    _0111 = 7,
    #[doc = "8: 0.44 <= fb < 0.50"]
    _1000 = 8,
    #[doc = "9: 0.40 <= fb < 0.44"]
    _1001 = 9,
    #[doc = "10: 0.36 <= fb < 0.40"]
    _1010 = 10,
    #[doc = "11: 0.33 <= fb < 0.36"]
    _1011 = 11,
    #[doc = "12: 0.31 <= fb < 0.33"]
    _1100 = 12,
    #[doc = "13: 0.29 <= fb < 0.31"]
    _1101 = 13,
    #[doc = "14: 0.27 <= fb < 0.29"]
    _1110 = 14,
    #[doc = "15: fb < 0.27"]
    _1111 = 15,
}
impl From<Ctsussdiv> for u8 {
    #[inline(always)]
    fn from(variant: Ctsussdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsussdiv {
    type Ux = u8;
}
impl crate::IsEnum for Ctsussdiv {}
#[doc = "Field `CTSUSSDIV` reader - CTSU Spectrum Diffusion Frequency Division Setting"]
pub type CtsussdivR = crate::FieldReader<Ctsussdiv>;
impl CtsussdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsussdiv {
        match self.bits {
            0 => Ctsussdiv::_0000,
            1 => Ctsussdiv::_0001,
            2 => Ctsussdiv::_0010,
            3 => Ctsussdiv::_0011,
            4 => Ctsussdiv::_0100,
            5 => Ctsussdiv::_0101,
            6 => Ctsussdiv::_0110,
            7 => Ctsussdiv::_0111,
            8 => Ctsussdiv::_1000,
            9 => Ctsussdiv::_1001,
            10 => Ctsussdiv::_1010,
            11 => Ctsussdiv::_1011,
            12 => Ctsussdiv::_1100,
            13 => Ctsussdiv::_1101,
            14 => Ctsussdiv::_1110,
            15 => Ctsussdiv::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "4.00 <= fb"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Ctsussdiv::_0000
    }
    #[doc = "2.00 <= fb < 4.00"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == Ctsussdiv::_0001
    }
    #[doc = "1.33 <= fb < 2.00"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == Ctsussdiv::_0010
    }
    #[doc = "1.00 <= fb < 1.33"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == Ctsussdiv::_0011
    }
    #[doc = "0.80 <= fb < 1.00"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == Ctsussdiv::_0100
    }
    #[doc = "0.67 <= fb < 0.80"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == Ctsussdiv::_0101
    }
    #[doc = "0.57 <= fb < 0.67"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == Ctsussdiv::_0110
    }
    #[doc = "0.50 <= fb < 0.57"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == Ctsussdiv::_0111
    }
    #[doc = "0.44 <= fb < 0.50"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == Ctsussdiv::_1000
    }
    #[doc = "0.40 <= fb < 0.44"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == Ctsussdiv::_1001
    }
    #[doc = "0.36 <= fb < 0.40"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == Ctsussdiv::_1010
    }
    #[doc = "0.33 <= fb < 0.36"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == Ctsussdiv::_1011
    }
    #[doc = "0.31 <= fb < 0.33"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == Ctsussdiv::_1100
    }
    #[doc = "0.29 <= fb < 0.31"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == Ctsussdiv::_1101
    }
    #[doc = "0.27 <= fb < 0.29"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == Ctsussdiv::_1110
    }
    #[doc = "fb < 0.27"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == Ctsussdiv::_1111
    }
}
#[doc = "Field `CTSUSSDIV` writer - CTSU Spectrum Diffusion Frequency Division Setting"]
pub type CtsussdivW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ctsussdiv, crate::Safe>;
impl<'a, REG> CtsussdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4.00 <= fb"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsussdiv::_0000)
    }
    #[doc = "2.00 <= fb < 4.00"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsussdiv::_0001)
    }
    #[doc = "1.33 <= fb < 2.00"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsussdiv::_0010)
    }
    #[doc = "1.00 <= fb < 1.33"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsussdiv::_0011)
    }
    #[doc = "0.80 <= fb < 1.00"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsussdiv::_0100)
    }
    #[doc = "0.67 <= fb < 0.80"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsussdiv::_0101)
    }
    #[doc = "0.57 <= fb < 0.67"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsussdiv::_0110)
    }
    #[doc = "0.50 <= fb < 0.57"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsussdiv::_0111)
    }
    #[doc = "0.44 <= fb < 0.50"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsussdiv::_1000)
    }
    #[doc = "0.40 <= fb < 0.44"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsussdiv::_1001)
    }
    #[doc = "0.36 <= fb < 0.40"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsussdiv::_1010)
    }
    #[doc = "0.33 <= fb < 0.36"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsussdiv::_1011)
    }
    #[doc = "0.31 <= fb < 0.33"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsussdiv::_1100)
    }
    #[doc = "0.29 <= fb < 0.31"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsussdiv::_1101)
    }
    #[doc = "0.27 <= fb < 0.29"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsussdiv::_1110)
    }
    #[doc = "fb < 0.27"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsussdiv::_1111)
    }
}
impl R {
    #[doc = "Bits 8:11 - CTSU Spectrum Diffusion Frequency Division Setting"]
    #[inline(always)]
    pub fn ctsussdiv(&self) -> CtsussdivR {
        CtsussdivR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUSSC")
            .field("ctsussdiv", &self.ctsussdiv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 8:11 - CTSU Spectrum Diffusion Frequency Division Setting"]
    #[inline(always)]
    pub fn ctsussdiv(&mut self) -> CtsussdivW<CtsusscSpec> {
        CtsussdivW::new(self, 8)
    }
}
#[doc = "CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsussc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsussc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtsusscSpec;
impl crate::RegisterSpec for CtsusscSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctsussc::R`](R) reader structure"]
impl crate::Readable for CtsusscSpec {}
#[doc = "`write(|w| ..)` method takes [`ctsussc::W`](W) writer structure"]
impl crate::Writable for CtsusscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTSUSSC to value 0"]
impl crate::Resettable for CtsusscSpec {}
