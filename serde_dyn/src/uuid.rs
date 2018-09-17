use std::ffi::{CStr, CString, OsStr, OsString};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use TypeUuid;

macro_rules! uuid {
    ($($type:ty => $id:expr),*) => {
        $(
            impl TypeUuid for $type {
                const UUID: u128 = $id;
            }
        )*
    }
}

uuid!{
    bool => 303353233546724231503697821832179461032,
    isize => 178084411820446663345161593199032920798,
    i8 => 115357190310068448140767679954845677011,
    i16 => 107722480968332948077656692445078063509,
    i32 => 161451673476522446293992879402045754367,
    i64 => 145450927602215131338380207983972844202,
    usize => 295680611213336671245494363719319189294,
    u8 => 105058260925501056684439690539946676899,
    u16 => 312550751882319709168694876218818335511,
    u32 => 78938753682486420131643715391047518652,
    u64 => 96948407298216036451617836814752334661,
    i128 => 61923966083469738988201255667277167770,
    u128 => 193030263834657559374493247408796947892,
    f32 => 69443304145415659655719030374683640413,
    f64 => 247679804515240109457135926398957666274,
    char => 107511146275753433606411061620325627675,
    str => 260809803319724059460629304709789868664,
    String => 298860549233238441386912461835653940333,
    CStr => 289933002765888658565256572983888514494,
    CString => 268656644579896298292146466429404937324,
    NonZeroU8 => 204882233994250040264768371422204349097,
    NonZeroU16 => 217807892668594830953932848193358461752,
    NonZeroU32 => 260680670413769366247727678900147000055,
    NonZeroU64 => 257656966494485424609139822877298993309,
    Duration => 45451698239681999507049061493934694960,
    SystemTime => 191920302000181308617186230753142127145,
    IpAddr => 36255310673157719272816199808287244814,
    Ipv4Addr => 217767414863944109076628226089448354293,
    Ipv6Addr => 183994496209701802524404769843133255174,
    SocketAddr => 254299716502850251377305503002586130068,
    SocketAddrV4 => 125579292062698970903542399175220727094,
    SocketAddrV6 => 184009016166986372316840604914436849128,
    Path => 272571052580499152112459658538880459164,
    PathBuf => 153267939792972678252642457586541721161,
    OsStr => 8086496532309862183846913619050606917,
    OsString => 143211202153189072093200836318928513145,
    () => 23818894022279401834075037072386988352
}