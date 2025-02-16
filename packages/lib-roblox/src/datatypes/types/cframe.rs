use core::fmt;
use std::ops;

use glam::{EulerRot, Mat4, Quat, Vec3};
use mlua::prelude::*;
use rbx_dom_weak::types::{CFrame as DomCFrame, Matrix3 as DomMatrix3, Vector3 as DomVector3};

use super::{super::*, Vector3};

/**
    An implementation of the [CFrame](https://create.roblox.com/docs/reference/engine/datatypes/CFrame)
    Roblox datatype, backed by [`glam::Mat4`].

    This implements all documented properties, methods &
    constructors of the CFrame class as of March 2023.
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CFrame(pub Mat4);

impl CFrame {
    pub const IDENTITY: Self = Self(Mat4::IDENTITY);

    fn position(&self) -> Vec3 {
        self.0.w_axis.truncate()
    }

    fn orientation(&self) -> (Vec3, Vec3, Vec3) {
        (
            self.0.x_axis.truncate(),
            self.0.y_axis.truncate(),
            self.0.z_axis.truncate(),
        )
    }

    fn inverse(&self) -> Self {
        Self(self.0.inverse())
    }

    pub(crate) fn make_table(lua: &Lua, datatype_table: &LuaTable) -> LuaResult<()> {
        // Constants
        datatype_table.set("identity", CFrame(Mat4::IDENTITY))?;
        // Strict args constructors
        datatype_table.set(
            "lookAt",
            lua.create_function(|_, (from, to, up): (Vector3, Vector3, Option<Vector3>)| {
                Ok(CFrame(look_at(
                    from.0,
                    to.0,
                    up.unwrap_or(Vector3(Vec3::Y)).0,
                )))
            })?,
        )?;
        datatype_table.set(
            "fromEulerAnglesXYZ",
            lua.create_function(|_, (rx, ry, rz): (f32, f32, f32)| {
                Ok(CFrame(Mat4::from_euler(EulerRot::XYZ, rx, ry, rz)))
            })?,
        )?;
        datatype_table.set(
            "fromEulerAnglesYXZ",
            lua.create_function(|_, (rx, ry, rz): (f32, f32, f32)| {
                Ok(CFrame(Mat4::from_euler(EulerRot::YXZ, ry, rx, rz)))
            })?,
        )?;
        datatype_table.set(
            "Angles",
            lua.create_function(|_, (rx, ry, rz): (f32, f32, f32)| {
                Ok(CFrame(Mat4::from_euler(EulerRot::XYZ, rx, ry, rz)))
            })?,
        )?;
        datatype_table.set(
            "fromOrientation",
            lua.create_function(|_, (rx, ry, rz): (f32, f32, f32)| {
                Ok(CFrame(Mat4::from_euler(EulerRot::YXZ, ry, rx, rz)))
            })?,
        )?;
        datatype_table.set(
            "fromAxisAngle",
            lua.create_function(|_, (v, r): (Vector3, f32)| {
                Ok(CFrame(Mat4::from_axis_angle(v.0, r)))
            })?,
        )?;
        datatype_table.set(
            "fromMatrix",
            lua.create_function(
                |_, (pos, rx, ry, rz): (Vector3, Vector3, Vector3, Option<Vector3>)| {
                    Ok(CFrame(Mat4::from_cols(
                        rx.0.extend(0.0),
                        ry.0.extend(0.0),
                        rz.map(|r| r.0)
                            .unwrap_or_else(|| rx.0.cross(ry.0).normalize())
                            .extend(0.0),
                        pos.0.extend(1.0),
                    )))
                },
            )?,
        )?;
        // Dynamic args constructor
        type ArgsPos = Vector3;
        type ArgsLook = (Vector3, Vector3, Option<Vector3>);
        type ArgsPosXYZ = (f32, f32, f32);
        type ArgsPosXYZQuat = (f32, f32, f32, f32, f32, f32, f32);
        type ArgsMatrix = (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32);
        datatype_table.set(
            "new",
            lua.create_function(|lua, args: LuaMultiValue| {
                if args.clone().into_vec().is_empty() {
                    Ok(CFrame(Mat4::IDENTITY))
                } else if let Ok(pos) = ArgsPos::from_lua_multi(args.clone(), lua) {
                    Ok(CFrame(Mat4::from_translation(pos.0)))
                } else if let Ok((from, to, up)) = ArgsLook::from_lua_multi(args.clone(), lua) {
                    Ok(CFrame(look_at(
                        from.0,
                        to.0,
                        up.unwrap_or(Vector3(Vec3::Y)).0,
                    )))
                } else if let Ok((x, y, z)) = ArgsPosXYZ::from_lua_multi(args.clone(), lua) {
                    Ok(CFrame(Mat4::from_translation(Vec3::new(x, y, z))))
                } else if let Ok((x, y, z, qx, qy, qz, qw)) =
                    ArgsPosXYZQuat::from_lua_multi(args.clone(), lua)
                {
                    Ok(CFrame(Mat4::from_rotation_translation(
                        Quat::from_array([qx, qy, qz, qw]),
                        Vec3::new(x, y, z),
                    )))
                } else if let Ok((x, y, z, r00, r01, r02, r10, r11, r12, r20, r21, r22)) =
                    ArgsMatrix::from_lua_multi(args, lua)
                {
                    Ok(CFrame(Mat4::from_cols_array_2d(&[
                        [r00, r01, r02, 0.0],
                        [r10, r11, r12, 0.0],
                        [r20, r21, r22, 0.0],
                        [x, y, z, 1.0],
                    ])))
                } else {
                    // FUTURE: Better error message here using given arg types
                    Err(LuaError::RuntimeError(
                        "Invalid arguments to constructor".to_string(),
                    ))
                }
            })?,
        )
    }
}

impl LuaUserData for CFrame {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("Position", |_, this| Ok(Vector3(this.position())));
        fields.add_field_method_get("Rotation", |_, this| {
            Ok(CFrame(Mat4::from_cols(
                this.0.x_axis,
                this.0.y_axis,
                this.0.z_axis,
                Vec3::ZERO.extend(1.0),
            )))
        });
        fields.add_field_method_get("X", |_, this| Ok(this.position().x));
        fields.add_field_method_get("Y", |_, this| Ok(this.position().y));
        fields.add_field_method_get("Z", |_, this| Ok(this.position().z));
        fields.add_field_method_get("XVector", |_, this| Ok(Vector3(this.orientation().0)));
        fields.add_field_method_get("YVector", |_, this| Ok(Vector3(this.orientation().1)));
        fields.add_field_method_get("ZVector", |_, this| Ok(Vector3(this.orientation().2)));
        fields.add_field_method_get("RightVector", |_, this| Ok(Vector3(this.orientation().0)));
        fields.add_field_method_get("UpVector", |_, this| Ok(Vector3(this.orientation().1)));
        fields.add_field_method_get("LookVector", |_, this| Ok(Vector3(-this.orientation().2)));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        // Methods
        methods.add_method("Inverse", |_, this, ()| Ok(this.inverse()));
        methods.add_method("Lerp", |_, this, (goal, alpha): (CFrame, f32)| {
            let quat_this = Quat::from_mat4(&this.0);
            let quat_goal = Quat::from_mat4(&goal.0);
            let translation = this
                .0
                .w_axis
                .truncate()
                .lerp(goal.0.w_axis.truncate(), alpha);
            let rotation = quat_this.slerp(quat_goal, alpha);
            Ok(CFrame(Mat4::from_rotation_translation(
                rotation,
                translation,
            )))
        });
        methods.add_method("Orthonormalize", |_, this, ()| {
            let rotation = Quat::from_mat4(&this.0);
            let translation = this.0.w_axis.truncate();
            Ok(CFrame(Mat4::from_rotation_translation(
                rotation.normalize(),
                translation,
            )))
        });
        methods.add_method("ToWorldSpace", |_, this, rhs: CFrame| Ok(*this * rhs));
        methods.add_method("ToObjectSpace", |_, this, rhs: CFrame| {
            Ok(this.inverse() * rhs)
        });
        methods.add_method("PointToWorldSpace", |_, this, rhs: Vector3| Ok(*this * rhs));
        methods.add_method("PointToObjectSpace", |_, this, rhs: Vector3| {
            Ok(this.inverse() * rhs)
        });
        methods.add_method("VectorToWorldSpace", |_, this, rhs: Vector3| {
            Ok((*this - Vector3(this.position())) * rhs)
        });
        methods.add_method("VectorToObjectSpace", |_, this, rhs: Vector3| {
            let inv = this.inverse();
            Ok((inv - Vector3(inv.position())) * rhs)
        });
        #[rustfmt::skip]
        methods.add_method("GetComponents", |_, this, ()| {
            let pos = this.position();
            let (rx, ry, rz) = this.orientation();
            Ok((
                pos.x, pos.y, -pos.z,
				 rx.x,  rx.y,   rx.z,
				 ry.x,  ry.y,   ry.z,
				 rz.x,  rz.y,   rz.z,
            ))
        });
        methods.add_method("ToEulerAnglesXYZ", |_, this, ()| {
            Ok(Quat::from_mat4(&this.0).to_euler(EulerRot::XYZ))
        });
        methods.add_method("ToEulerAnglesYXZ", |_, this, ()| {
            let (ry, rx, rz) = Quat::from_mat4(&this.0).to_euler(EulerRot::YXZ);
            Ok((rx, ry, rz))
        });
        methods.add_method("ToOrientation", |_, this, ()| {
            let (ry, rx, rz) = Quat::from_mat4(&this.0).to_euler(EulerRot::YXZ);
            Ok((rx, ry, rz))
        });
        methods.add_method("ToAxisAngle", |_, this, ()| {
            let (axis, angle) = Quat::from_mat4(&this.0).to_axis_angle();
            Ok((Vector3(axis), angle))
        });
        // Metamethods
        methods.add_meta_method(LuaMetaMethod::Eq, userdata_impl_eq);
        methods.add_meta_method(LuaMetaMethod::ToString, userdata_impl_to_string);
        methods.add_meta_method(LuaMetaMethod::Mul, |lua, this, rhs: LuaValue| {
            if let LuaValue::UserData(ud) = &rhs {
                if let Ok(cf) = ud.borrow::<CFrame>() {
                    return lua.create_userdata(*this * *cf);
                } else if let Ok(vec) = ud.borrow::<Vector3>() {
                    return lua.create_userdata(*this * *vec);
                }
            };
            Err(LuaError::FromLuaConversionError {
                from: rhs.type_name(),
                to: "userdata",
                message: Some(format!(
                    "Expected CFrame or Vector3, got {}",
                    rhs.type_name()
                )),
            })
        });
        methods.add_meta_method(LuaMetaMethod::Add, |_, this, vec: Vector3| Ok(*this + vec));
        methods.add_meta_method(LuaMetaMethod::Sub, |_, this, vec: Vector3| Ok(*this - vec));
    }
}

impl fmt::Display for CFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pos = self.position();
        let (rx, ry, rz) = self.orientation();
        write!(
            f,
            "{}, {}, {}, {}",
            Vector3(pos),
            Vector3(rx),
            Vector3(ry),
            Vector3(rz)
        )
    }
}

impl ops::Mul for CFrame {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        CFrame(self.0 * rhs.0)
    }
}

impl ops::Mul<Vector3> for CFrame {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3(self.0.project_point3(rhs.0))
    }
}

impl ops::Add<Vector3> for CFrame {
    type Output = Self;
    fn add(self, rhs: Vector3) -> Self::Output {
        CFrame(Mat4::from_cols(
            self.0.x_axis,
            self.0.y_axis,
            self.0.z_axis,
            self.0.w_axis + rhs.0.extend(0.0),
        ))
    }
}

impl ops::Sub<Vector3> for CFrame {
    type Output = Self;
    fn sub(self, rhs: Vector3) -> Self::Output {
        CFrame(Mat4::from_cols(
            self.0.x_axis,
            self.0.y_axis,
            self.0.z_axis,
            self.0.w_axis - rhs.0.extend(0.0),
        ))
    }
}

impl From<DomCFrame> for CFrame {
    fn from(v: DomCFrame) -> Self {
        CFrame(Mat4::from_cols(
            Vector3::from(v.orientation.x).0.extend(0.0),
            Vector3::from(v.orientation.y).0.extend(0.0),
            Vector3::from(v.orientation.z).0.extend(0.0),
            Vector3::from(v.position).0.extend(1.0),
        ))
    }
}

impl From<CFrame> for DomCFrame {
    fn from(v: CFrame) -> Self {
        let (rx, ry, rz) = v.orientation();
        DomCFrame {
            position: DomVector3::from(Vector3(v.position())),
            orientation: DomMatrix3::new(
                DomVector3::from(Vector3(rx)),
                DomVector3::from(Vector3(ry)),
                DomVector3::from(Vector3(rz)),
            ),
        }
    }
}

/**
    Creates a matrix at the position `from`, looking towards `to`.

    [`glam`] does provide functions such as [`look_at_lh`], [`look_at_rh`] and more but
    they all create view matrices for camera transforms which is not what we want here.
*/
fn look_at(from: Vec3, to: Vec3, up: Vec3) -> Mat4 {
    let dir = (to - from).normalize();
    let xaxis = up.cross(dir).normalize();
    let yaxis = dir.cross(xaxis).normalize();

    Mat4::from_cols(
        Vec3::new(xaxis.x, yaxis.x, dir.x).extend(0.0),
        Vec3::new(xaxis.y, yaxis.y, dir.y).extend(0.0),
        Vec3::new(xaxis.z, yaxis.z, dir.z).extend(0.0),
        from.extend(1.0),
    )
}
