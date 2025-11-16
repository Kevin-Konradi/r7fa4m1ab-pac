#[doc = "Register `USBMC` reader"]
pub type R = crate::R<UsbmcSpec>;
#[doc = "Register `USBMC` writer"]
pub type W = crate::W<UsbmcSpec>;
#[doc = "USB Reference Power Supply Circuit On/Off Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vddusbe {
    #[doc = "0: USB reference power supply circuit off"]
    _0 = 0,
    #[doc = "1: USB reference power supply circuit on"]
    _1 = 1,
}
impl From<Vddusbe> for bool {
    #[inline(always)]
    fn from(variant: Vddusbe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDUSBE` reader - USB Reference Power Supply Circuit On/Off Control"]
pub type VddusbeR = crate::BitReader<Vddusbe>;
impl VddusbeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vddusbe {
        match self.bits {
            false => Vddusbe::_0,
            true => Vddusbe::_1,
        }
    }
    #[doc = "USB reference power supply circuit off"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vddusbe::_0
    }
    #[doc = "USB reference power supply circuit on"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vddusbe::_1
    }
}
#[doc = "Field `VDDUSBE` writer - USB Reference Power Supply Circuit On/Off Control"]
pub type VddusbeW<'a, REG> = crate::BitWriter<'a, REG, Vddusbe>;
impl<'a, REG> VddusbeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB reference power supply circuit off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vddusbe::_0)
    }
    #[doc = "USB reference power supply circuit on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vddusbe::_1)
    }
}
#[doc = "USB Regulator On/Off Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vdcen {
    #[doc = "0: USB regulator off"]
    _0 = 0,
    #[doc = "1: USB regulator on"]
    _1 = 1,
}
impl From<Vdcen> for bool {
    #[inline(always)]
    fn from(variant: Vdcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDCEN` reader - USB Regulator On/Off Control"]
pub type VdcenR = crate::BitReader<Vdcen>;
impl VdcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vdcen {
        match self.bits {
            false => Vdcen::_0,
            true => Vdcen::_1,
        }
    }
    #[doc = "USB regulator off"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vdcen::_0
    }
    #[doc = "USB regulator on"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vdcen::_1
    }
}
#[doc = "Field `VDCEN` writer - USB Regulator On/Off Control"]
pub type VdcenW<'a, REG> = crate::BitWriter<'a, REG, Vdcen>;
impl<'a, REG> VdcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB regulator off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vdcen::_0)
    }
    #[doc = "USB regulator on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vdcen::_1)
    }
}
impl R {
    #[doc = "Bit 0 - USB Reference Power Supply Circuit On/Off Control"]
    #[inline(always)]
    pub fn vddusbe(&self) -> VddusbeR {
        VddusbeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - USB Regulator On/Off Control"]
    #[inline(always)]
    pub fn vdcen(&self) -> VdcenR {
        VdcenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBMC")
            .field("vdcen", &self.vdcen())
            .field("vddusbe", &self.vddusbe())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - USB Reference Power Supply Circuit On/Off Control"]
    #[inline(always)]
    pub fn vddusbe(&mut self) -> VddusbeW<UsbmcSpec> {
        VddusbeW::new(self, 0)
    }
    #[doc = "Bit 7 - USB Regulator On/Off Control"]
    #[inline(always)]
    pub fn vdcen(&mut self) -> VdcenW<UsbmcSpec> {
        VdcenW::new(self, 7)
    }
}
#[doc = "USB Module Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbmcSpec;
impl crate::RegisterSpec for UsbmcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usbmc::R`](R) reader structure"]
impl crate::Readable for UsbmcSpec {}
#[doc = "`write(|w| ..)` method takes [`usbmc::W`](W) writer structure"]
impl crate::Writable for UsbmcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBMC to value 0x02"]
impl crate::Resettable for UsbmcSpec {
    const RESET_VALUE: u16 = 0x02;
}
