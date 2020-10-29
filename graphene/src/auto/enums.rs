// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use graphene_sys;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum EulerOrder {
    Default,
    Xyz,
    Yzx,
    Zxy,
    Xzy,
    Yxz,
    Zyx,
    Sxyz,
    Sxyx,
    Sxzy,
    Sxzx,
    Syzx,
    Syzy,
    Syxz,
    Syxy,
    Szxy,
    Szxz,
    Szyx,
    Szyz,
    Rzyx,
    Rxyx,
    Ryzx,
    Rxzx,
    Rxzy,
    Ryzy,
    Rzxy,
    Ryxy,
    Ryxz,
    Rzxz,
    Rxyz,
    Rzyz,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for EulerOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "EulerOrder::{}",
            match *self {
                EulerOrder::Default => "Default",
                EulerOrder::Xyz => "Xyz",
                EulerOrder::Yzx => "Yzx",
                EulerOrder::Zxy => "Zxy",
                EulerOrder::Xzy => "Xzy",
                EulerOrder::Yxz => "Yxz",
                EulerOrder::Zyx => "Zyx",
                EulerOrder::Sxyz => "Sxyz",
                EulerOrder::Sxyx => "Sxyx",
                EulerOrder::Sxzy => "Sxzy",
                EulerOrder::Sxzx => "Sxzx",
                EulerOrder::Syzx => "Syzx",
                EulerOrder::Syzy => "Syzy",
                EulerOrder::Syxz => "Syxz",
                EulerOrder::Syxy => "Syxy",
                EulerOrder::Szxy => "Szxy",
                EulerOrder::Szxz => "Szxz",
                EulerOrder::Szyx => "Szyx",
                EulerOrder::Szyz => "Szyz",
                EulerOrder::Rzyx => "Rzyx",
                EulerOrder::Rxyx => "Rxyx",
                EulerOrder::Ryzx => "Ryzx",
                EulerOrder::Rxzx => "Rxzx",
                EulerOrder::Rxzy => "Rxzy",
                EulerOrder::Ryzy => "Ryzy",
                EulerOrder::Rzxy => "Rzxy",
                EulerOrder::Ryxy => "Ryxy",
                EulerOrder::Ryxz => "Ryxz",
                EulerOrder::Rzxz => "Rzxz",
                EulerOrder::Rxyz => "Rxyz",
                EulerOrder::Rzyz => "Rzyz",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for EulerOrder {
    type GlibType = graphene_sys::graphene_euler_order_t;

    fn to_glib(&self) -> graphene_sys::graphene_euler_order_t {
        match *self {
            EulerOrder::Default => graphene_sys::GRAPHENE_EULER_ORDER_DEFAULT,
            EulerOrder::Xyz => graphene_sys::GRAPHENE_EULER_ORDER_XYZ,
            EulerOrder::Yzx => graphene_sys::GRAPHENE_EULER_ORDER_YZX,
            EulerOrder::Zxy => graphene_sys::GRAPHENE_EULER_ORDER_ZXY,
            EulerOrder::Xzy => graphene_sys::GRAPHENE_EULER_ORDER_XZY,
            EulerOrder::Yxz => graphene_sys::GRAPHENE_EULER_ORDER_YXZ,
            EulerOrder::Zyx => graphene_sys::GRAPHENE_EULER_ORDER_ZYX,
            EulerOrder::Sxyz => graphene_sys::GRAPHENE_EULER_ORDER_SXYZ,
            EulerOrder::Sxyx => graphene_sys::GRAPHENE_EULER_ORDER_SXYX,
            EulerOrder::Sxzy => graphene_sys::GRAPHENE_EULER_ORDER_SXZY,
            EulerOrder::Sxzx => graphene_sys::GRAPHENE_EULER_ORDER_SXZX,
            EulerOrder::Syzx => graphene_sys::GRAPHENE_EULER_ORDER_SYZX,
            EulerOrder::Syzy => graphene_sys::GRAPHENE_EULER_ORDER_SYZY,
            EulerOrder::Syxz => graphene_sys::GRAPHENE_EULER_ORDER_SYXZ,
            EulerOrder::Syxy => graphene_sys::GRAPHENE_EULER_ORDER_SYXY,
            EulerOrder::Szxy => graphene_sys::GRAPHENE_EULER_ORDER_SZXY,
            EulerOrder::Szxz => graphene_sys::GRAPHENE_EULER_ORDER_SZXZ,
            EulerOrder::Szyx => graphene_sys::GRAPHENE_EULER_ORDER_SZYX,
            EulerOrder::Szyz => graphene_sys::GRAPHENE_EULER_ORDER_SZYZ,
            EulerOrder::Rzyx => graphene_sys::GRAPHENE_EULER_ORDER_RZYX,
            EulerOrder::Rxyx => graphene_sys::GRAPHENE_EULER_ORDER_RXYX,
            EulerOrder::Ryzx => graphene_sys::GRAPHENE_EULER_ORDER_RYZX,
            EulerOrder::Rxzx => graphene_sys::GRAPHENE_EULER_ORDER_RXZX,
            EulerOrder::Rxzy => graphene_sys::GRAPHENE_EULER_ORDER_RXZY,
            EulerOrder::Ryzy => graphene_sys::GRAPHENE_EULER_ORDER_RYZY,
            EulerOrder::Rzxy => graphene_sys::GRAPHENE_EULER_ORDER_RZXY,
            EulerOrder::Ryxy => graphene_sys::GRAPHENE_EULER_ORDER_RYXY,
            EulerOrder::Ryxz => graphene_sys::GRAPHENE_EULER_ORDER_RYXZ,
            EulerOrder::Rzxz => graphene_sys::GRAPHENE_EULER_ORDER_RZXZ,
            EulerOrder::Rxyz => graphene_sys::GRAPHENE_EULER_ORDER_RXYZ,
            EulerOrder::Rzyz => graphene_sys::GRAPHENE_EULER_ORDER_RZYZ,
            EulerOrder::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<graphene_sys::graphene_euler_order_t> for EulerOrder {
    fn from_glib(value: graphene_sys::graphene_euler_order_t) -> Self {
        skip_assert_initialized!();
        match value {
            -1 => EulerOrder::Default,
            0 => EulerOrder::Xyz,
            1 => EulerOrder::Yzx,
            2 => EulerOrder::Zxy,
            3 => EulerOrder::Xzy,
            4 => EulerOrder::Yxz,
            5 => EulerOrder::Zyx,
            6 => EulerOrder::Sxyz,
            7 => EulerOrder::Sxyx,
            8 => EulerOrder::Sxzy,
            9 => EulerOrder::Sxzx,
            10 => EulerOrder::Syzx,
            11 => EulerOrder::Syzy,
            12 => EulerOrder::Syxz,
            13 => EulerOrder::Syxy,
            14 => EulerOrder::Szxy,
            15 => EulerOrder::Szxz,
            16 => EulerOrder::Szyx,
            17 => EulerOrder::Szyz,
            18 => EulerOrder::Rzyx,
            19 => EulerOrder::Rxyx,
            20 => EulerOrder::Ryzx,
            21 => EulerOrder::Rxzx,
            22 => EulerOrder::Rxzy,
            23 => EulerOrder::Ryzy,
            24 => EulerOrder::Rzxy,
            25 => EulerOrder::Ryxy,
            26 => EulerOrder::Ryxz,
            27 => EulerOrder::Rzxz,
            28 => EulerOrder::Rxyz,
            29 => EulerOrder::Rzyz,
            value => EulerOrder::__Unknown(value),
        }
    }
}
