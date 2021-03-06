foreigner_class!(class Position {
    self_type GnssInfo;
    private constructor create_position() -> GnssInfo;
    fn Position::getLatitude(&self) -> f64;
});

foreigner_class!(class PosErr {
    self_type PosErr;
    constructor PosErr::new() -> PosErr;
});

foreigner_class!(class LocationService {
    self_type LocationService;

    constructor LocationService::new() -> LocationService;
    fn LocationService::f1() -> Result<GnssInfo, String>;
    fn LocationService::f2() -> Result<(), String>;

    fn LocationService::f3(&self) -> Result<GnssInfo, PosErr>;
    fn LocationService::f4() -> Result<(), PosErr>;
    fn LocationService::f5() -> Result<Vec<GnssInfo>, PosErr>;
    fn LocationService::create() -> Result<LocationService, String>;
});

foreigner_class!(class Foo {
    self_type Foo<'a>;
    constructor Foo::create() -> Foo<'a>;
    fn Foo::from_string<'a>(_: &str) -> Result<Foo<'a>, String>;
});
