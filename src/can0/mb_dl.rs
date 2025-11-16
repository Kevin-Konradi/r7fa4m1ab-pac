#[doc = "Register `MB%s_DL` reader"]
pub type R = crate::R<MbDlSpec>;
#[doc = "Register `MB%s_DL` writer"]
pub type W = crate::W<MbDlSpec>;
#[doc = "Data Length Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dlc {
    #[doc = "0: Data length = 0 byte"]
    _0000 = 0,
    #[doc = "1: Data length = 1 byte"]
    _0001 = 1,
    #[doc = "2: Data length = 2 bytes"]
    _0010 = 2,
    #[doc = "3: Data length = 3 bytes"]
    _0011 = 3,
    #[doc = "4: Data length = 4 bytes"]
    _0100 = 4,
    #[doc = "5: Data length = 5 bytes"]
    _0101 = 5,
    #[doc = "6: Data length = 6 bytes"]
    _0110 = 6,
    #[doc = "7: Data length = 7 bytes"]
    _0111 = 7,
    #[doc = "8: Data length = 8 bytes"]
    Others = 8,
}
impl From<Dlc> for u8 {
    #[inline(always)]
    fn from(variant: Dlc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dlc {
    type Ux = u8;
}
impl crate::IsEnum for Dlc {}
#[doc = "Field `DLC` reader - Data Length Code"]
pub type DlcR = crate::FieldReader<Dlc>;
impl DlcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dlc {
        match self.bits {
            0 => Dlc::_0000,
            1 => Dlc::_0001,
            2 => Dlc::_0010,
            3 => Dlc::_0011,
            4 => Dlc::_0100,
            5 => Dlc::_0101,
            6 => Dlc::_0110,
            7 => Dlc::_0111,
            _ => Dlc::Others,
        }
    }
    #[doc = "Data length = 0 byte"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Dlc::_0000
    }
    #[doc = "Data length = 1 byte"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == Dlc::_0001
    }
    #[doc = "Data length = 2 bytes"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == Dlc::_0010
    }
    #[doc = "Data length = 3 bytes"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == Dlc::_0011
    }
    #[doc = "Data length = 4 bytes"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == Dlc::_0100
    }
    #[doc = "Data length = 5 bytes"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == Dlc::_0101
    }
    #[doc = "Data length = 6 bytes"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == Dlc::_0110
    }
    #[doc = "Data length = 7 bytes"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == Dlc::_0111
    }
    #[doc = "Data length = 8 bytes"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Dlc::Others)
    }
}
#[doc = "Field `DLC` writer - Data Length Code"]
pub type DlcW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dlc, crate::Safe>;
impl<'a, REG> DlcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data length = 0 byte"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Dlc::_0000)
    }
    #[doc = "Data length = 1 byte"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(Dlc::_0001)
    }
    #[doc = "Data length = 2 bytes"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(Dlc::_0010)
    }
    #[doc = "Data length = 3 bytes"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(Dlc::_0011)
    }
    #[doc = "Data length = 4 bytes"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(Dlc::_0100)
    }
    #[doc = "Data length = 5 bytes"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(Dlc::_0101)
    }
    #[doc = "Data length = 6 bytes"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(Dlc::_0110)
    }
    #[doc = "Data length = 7 bytes"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(Dlc::_0111)
    }
    #[doc = "Data length = 8 bytes"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Dlc::Others)
    }
}
impl R {
    #[doc = "Bits 0:3 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&self) -> DlcR {
        DlcR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MB_DL").field("dlc", &self.dlc()).finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&mut self) -> DlcW<MbDlSpec> {
        DlcW::new(self, 0)
    }
}
#[doc = "Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_dl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_dl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbDlSpec;
impl crate::RegisterSpec for MbDlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mb_dl::R`](R) reader structure"]
impl crate::Readable for MbDlSpec {}
#[doc = "`write(|w| ..)` method takes [`mb_dl::W`](W) writer structure"]
impl crate::Writable for MbDlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MB%s_DL to value 0"]
impl crate::Resettable for MbDlSpec {}
