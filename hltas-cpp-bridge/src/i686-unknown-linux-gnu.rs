/* automatically generated by rust-bindgen 0.69.4 */

extern "C" {
    pub fn hltas_input_set_property(
        input: *mut ::std::os::raw::c_void,
        property: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn hltas_input_get_property(
        input: *const ::std::os::raw::c_void,
        property: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn hltas_input_push_frame(input: *mut ::std::os::raw::c_void, frame: *const hltas_frame);
}
extern "C" {
    pub fn hltas_input_get_frame(
        input: *const ::std::os::raw::c_void,
        index: usize,
        frame: *mut hltas_frame,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn hltas_input_set_error_message(
        input: *mut ::std::os::raw::c_void,
        message: *const ::std::os::raw::c_char,
    );
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ErrorCode {
    OK = 0,
    FAILOPEN = 1,
    FAILVER = 2,
    NOTSUPPORTED = 3,
    FAILLINE = 4,
    NOSAVENAME = 5,
    FAILFRAME = 6,
    FAILWRITE = 7,
    NOSEED = 8,
    NOYAW = 9,
    NOBUTTONS = 10,
    BOTHAJDT = 11,
    NOLGAGSTACTION = 12,
    NOLGAGSTMINSPEED = 13,
    LGAGSTACTIONTIMES = 14,
    NORESETSEED = 15,
    INVALID_ALGORITHM = 16,
    MISSING_CONSTRAINTS = 17,
    NO_PM_IN_TOLERANCE = 18,
    MISSING_ALGORITHM_FROMTO_PARAMETERS = 19,
    NO_TO_IN_FROMTO_ALGORITHM = 20,
    NO_YAWSPEED = 21,
    UNSUPPORTED_YAWSPEED_DIR = 22,
    NEGATIVE_YAWSPEED_VALUE = 23,
    NO_ACCELERATION_YAWSPEED = 24,
    UNSUPPORTED_ACCEL_YAWSPEED_DIR = 25,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ErrorDescription {
    pub Code: ErrorCode,
    pub LineNumber: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_ErrorDescription() {
    const UNINIT: ::std::mem::MaybeUninit<ErrorDescription> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ErrorDescription>(),
        8usize,
        concat!("Size of: ", stringify!(ErrorDescription))
    );
    assert_eq!(
        ::std::mem::align_of::<ErrorDescription>(),
        4usize,
        concat!("Alignment of ", stringify!(ErrorDescription))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Code) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorDescription),
            "::",
            stringify!(Code)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).LineNumber) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorDescription),
            "::",
            stringify!(LineNumber)
        )
    );
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum StrafeType {
    MAXACCEL = 0,
    MAXANGLE = 1,
    MAXDECCEL = 2,
    CONSTSPEED = 3,
    CONSTYAWSPEED = 4,
    ACCELYAWSPEED = 5,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum StrafeDir {
    LEFT = 0,
    RIGHT = 1,
    BEST = 2,
    YAW = 3,
    POINT = 4,
    LINE = 5,
    LEFT_RIGHT = 6,
    RIGHT_LEFT = 7,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ButtonState {
    NOTHING = 0,
    SET = 1,
    CLEAR = 2,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Button {
    FORWARD = 0,
    FORWARD_LEFT = 1,
    LEFT = 2,
    BACK_LEFT = 3,
    BACK = 4,
    BACK_RIGHT = 5,
    RIGHT = 6,
    FORWARD_RIGHT = 7,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StrafeButtons {
    pub AirLeft: Button,
    pub AirRight: Button,
    pub GroundLeft: Button,
    pub GroundRight: Button,
}
#[test]
fn bindgen_test_layout_StrafeButtons() {
    const UNINIT: ::std::mem::MaybeUninit<StrafeButtons> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<StrafeButtons>(),
        4usize,
        concat!("Size of: ", stringify!(StrafeButtons))
    );
    assert_eq!(
        ::std::mem::align_of::<StrafeButtons>(),
        1usize,
        concat!("Alignment of ", stringify!(StrafeButtons))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).AirLeft) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(StrafeButtons),
            "::",
            stringify!(AirLeft)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).AirRight) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(StrafeButtons),
            "::",
            stringify!(AirRight)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).GroundLeft) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(StrafeButtons),
            "::",
            stringify!(GroundLeft)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).GroundRight) as usize - ptr as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(StrafeButtons),
            "::",
            stringify!(GroundRight)
        )
    );
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum StrafingAlgorithm {
    YAW = 0,
    VECTORIAL = 1,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ConstraintsType {
    VELOCITY = 0,
    VELOCITY_AVG = 1,
    VELOCITY_LOCK = 2,
    YAW = 3,
    YAW_RANGE = 4,
    LOOK_AT = 5,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AlgorithmParameters {
    pub Type: ConstraintsType,
    pub Parameters: AlgorithmParameters__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union AlgorithmParameters__bindgen_ty_1 {
    pub Velocity: AlgorithmParameters__bindgen_ty_1__bindgen_ty_1,
    pub VelocityAvg: AlgorithmParameters__bindgen_ty_1__bindgen_ty_2,
    pub VelocityLock: AlgorithmParameters__bindgen_ty_1__bindgen_ty_3,
    pub Yaw: AlgorithmParameters__bindgen_ty_1__bindgen_ty_4,
    pub YawRange: AlgorithmParameters__bindgen_ty_1__bindgen_ty_5,
    pub LookAt: AlgorithmParameters__bindgen_ty_1__bindgen_ty_6,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AlgorithmParameters__bindgen_ty_1__bindgen_ty_1 {
    pub Constraints: f64,
}
#[test]
fn bindgen_test_layout_AlgorithmParameters__bindgen_ty_1__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<AlgorithmParameters__bindgen_ty_1__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Constraints) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Constraints)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AlgorithmParameters__bindgen_ty_1__bindgen_ty_2 {
    pub Constraints: f64,
}
#[test]
fn bindgen_test_layout_AlgorithmParameters__bindgen_ty_1__bindgen_ty_2() {
    const UNINIT: ::std::mem::MaybeUninit<AlgorithmParameters__bindgen_ty_1__bindgen_ty_2> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_2>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_2>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Constraints) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(Constraints)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AlgorithmParameters__bindgen_ty_1__bindgen_ty_3 {
    pub Constraints: f64,
}
#[test]
fn bindgen_test_layout_AlgorithmParameters__bindgen_ty_1__bindgen_ty_3() {
    const UNINIT: ::std::mem::MaybeUninit<AlgorithmParameters__bindgen_ty_1__bindgen_ty_3> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_3>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_3>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Constraints) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(Constraints)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AlgorithmParameters__bindgen_ty_1__bindgen_ty_4 {
    pub Yaw: f64,
    pub Constraints: f64,
}
#[test]
fn bindgen_test_layout_AlgorithmParameters__bindgen_ty_1__bindgen_ty_4() {
    const UNINIT: ::std::mem::MaybeUninit<AlgorithmParameters__bindgen_ty_1__bindgen_ty_4> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_4>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_4>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Yaw) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(Yaw)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Constraints) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(Constraints)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AlgorithmParameters__bindgen_ty_1__bindgen_ty_5 {
    pub LowestYaw: f64,
    pub HighestYaw: f64,
}
#[test]
fn bindgen_test_layout_AlgorithmParameters__bindgen_ty_1__bindgen_ty_5() {
    const UNINIT: ::std::mem::MaybeUninit<AlgorithmParameters__bindgen_ty_1__bindgen_ty_5> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_5>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_5)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_5>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_5)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).LowestYaw) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_5),
            "::",
            stringify!(LowestYaw)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).HighestYaw) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_5),
            "::",
            stringify!(HighestYaw)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AlgorithmParameters__bindgen_ty_1__bindgen_ty_6 {
    pub Entity: ::std::os::raw::c_uint,
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}
#[test]
fn bindgen_test_layout_AlgorithmParameters__bindgen_ty_1__bindgen_ty_6() {
    const UNINIT: ::std::mem::MaybeUninit<AlgorithmParameters__bindgen_ty_1__bindgen_ty_6> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_6>(),
        28usize,
        concat!(
            "Size of: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_6)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_6>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_6)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Entity) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_6),
            "::",
            stringify!(Entity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).X) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_6),
            "::",
            stringify!(X)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Y) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_6),
            "::",
            stringify!(Y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Z) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_6),
            "::",
            stringify!(Z)
        )
    );
}
#[test]
fn bindgen_test_layout_AlgorithmParameters__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<AlgorithmParameters__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AlgorithmParameters__bindgen_ty_1>(),
        28usize,
        concat!("Size of: ", stringify!(AlgorithmParameters__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<AlgorithmParameters__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(AlgorithmParameters__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Velocity) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1),
            "::",
            stringify!(Velocity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).VelocityAvg) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1),
            "::",
            stringify!(VelocityAvg)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).VelocityLock) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1),
            "::",
            stringify!(VelocityLock)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Yaw) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1),
            "::",
            stringify!(Yaw)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).YawRange) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1),
            "::",
            stringify!(YawRange)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).LookAt) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1),
            "::",
            stringify!(LookAt)
        )
    );
}
#[test]
fn bindgen_test_layout_AlgorithmParameters() {
    const UNINIT: ::std::mem::MaybeUninit<AlgorithmParameters> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AlgorithmParameters>(),
        32usize,
        concat!("Size of: ", stringify!(AlgorithmParameters))
    );
    assert_eq!(
        ::std::mem::align_of::<AlgorithmParameters>(),
        4usize,
        concat!("Alignment of ", stringify!(AlgorithmParameters))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters),
            "::",
            stringify!(Type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Parameters) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters),
            "::",
            stringify!(Parameters)
        )
    );
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ChangeTarget {
    YAW = 0,
    PITCH = 1,
    TARGET_YAW = 2,
    TARGET_YAW_OFFSET = 3,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hltas_frame {
    pub Strafe: bool,
    pub Lgagst: bool,
    pub Autojump: bool,
    pub Ducktap: bool,
    pub Jumpbug: bool,
    pub Dbc: bool,
    pub Dbg: bool,
    pub Dwj: bool,
    pub Type: StrafeType,
    pub Dir: StrafeDir,
    pub LgagstFullMaxspeed: bool,
    pub LgagstTimes: u32,
    pub AutojumpTimes: u32,
    pub Ducktap0ms: bool,
    pub DucktapTimes: u32,
    pub JumpbugTimes: u32,
    pub DbcCeilings: bool,
    pub DbcTimes: u32,
    pub DbgTimes: u32,
    pub DwjTimes: u32,
    pub Forward: bool,
    pub Left: bool,
    pub Right: bool,
    pub Back: bool,
    pub Up: bool,
    pub Down: bool,
    pub Jump: bool,
    pub Duck: bool,
    pub Use: bool,
    pub Attack1: bool,
    pub Attack2: bool,
    pub Reload: bool,
    pub Frametime: *const ::std::os::raw::c_char,
    pub PitchPresent: bool,
    pub YawPresent: bool,
    pub Yaw: f64,
    pub X: f64,
    pub Y: f64,
    pub Count: ::std::os::raw::c_uint,
    pub Yawspeed: f64,
    pub TargetYawspeed: f64,
    pub Acceleration: f64,
    pub Pitch: f64,
    pub Repeats: u32,
    pub Commands: *const ::std::os::raw::c_char,
    pub Comments: *const ::std::os::raw::c_char,
    pub SaveName: *const ::std::os::raw::c_char,
    pub SeedPresent: bool,
    pub Seed: u32,
    pub BtnState: ButtonState,
    pub Buttons: StrafeButtons,
    pub LgagstMinSpeedPresent: bool,
    pub LgagstMinSpeed: f32,
    pub ResetFrame: bool,
    pub ResetNonSharedRNGSeed: i64,
    pub StrafingAlgorithmPresent: bool,
    pub Algorithm: StrafingAlgorithm,
    pub AlgorithmParametersPresent: bool,
    pub Parameters: AlgorithmParameters,
    pub ChangePresent: bool,
    pub Target: ChangeTarget,
    pub ChangeFinalValue: f32,
    pub ChangeOver: f32,
    pub TargetYawOverride: *const f32,
    pub TargetYawOverrideCount: usize,
    pub RenderYawOverride: *const f32,
    pub RenderYawOverrideCount: usize,
}
#[test]
fn bindgen_test_layout_hltas_frame() {
    const UNINIT: ::std::mem::MaybeUninit<hltas_frame> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<hltas_frame>(),
        248usize,
        concat!("Size of: ", stringify!(hltas_frame))
    );
    assert_eq!(
        ::std::mem::align_of::<hltas_frame>(),
        4usize,
        concat!("Alignment of ", stringify!(hltas_frame))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Strafe) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Strafe)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Lgagst) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Lgagst)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Autojump) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Autojump)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Ducktap) as usize - ptr as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Ducktap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Jumpbug) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Jumpbug)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Dbc) as usize - ptr as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Dbc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Dbg) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Dbg)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Dwj) as usize - ptr as usize },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Dwj)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Type) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Dir) as usize - ptr as usize },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Dir)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).LgagstFullMaxspeed) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(LgagstFullMaxspeed)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).LgagstTimes) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(LgagstTimes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).AutojumpTimes) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(AutojumpTimes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Ducktap0ms) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Ducktap0ms)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).DucktapTimes) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(DucktapTimes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).JumpbugTimes) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(JumpbugTimes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).DbcCeilings) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(DbcCeilings)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).DbcTimes) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(DbcTimes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).DbgTimes) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(DbgTimes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).DwjTimes) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(DwjTimes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Forward) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Forward)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Left) as usize - ptr as usize },
        49usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Left)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Right) as usize - ptr as usize },
        50usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Right)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Back) as usize - ptr as usize },
        51usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Back)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Up) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Up)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Down) as usize - ptr as usize },
        53usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Down)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Jump) as usize - ptr as usize },
        54usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Jump)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Duck) as usize - ptr as usize },
        55usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Duck)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Use) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Use)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Attack1) as usize - ptr as usize },
        57usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Attack1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Attack2) as usize - ptr as usize },
        58usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Attack2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Reload) as usize - ptr as usize },
        59usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Reload)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Frametime) as usize - ptr as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Frametime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).PitchPresent) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(PitchPresent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).YawPresent) as usize - ptr as usize },
        65usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(YawPresent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Yaw) as usize - ptr as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Yaw)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).X) as usize - ptr as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(X)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Y) as usize - ptr as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Count) as usize - ptr as usize },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Yawspeed) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Yawspeed)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).TargetYawspeed) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(TargetYawspeed)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Acceleration) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Acceleration)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Pitch) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Pitch)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Repeats) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Repeats)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Commands) as usize - ptr as usize },
        132usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Commands)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Comments) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Comments)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SaveName) as usize - ptr as usize },
        140usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(SaveName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SeedPresent) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(SeedPresent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Seed) as usize - ptr as usize },
        148usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Seed)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).BtnState) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(BtnState)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Buttons) as usize - ptr as usize },
        153usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Buttons)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).LgagstMinSpeedPresent) as usize - ptr as usize },
        157usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(LgagstMinSpeedPresent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).LgagstMinSpeed) as usize - ptr as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(LgagstMinSpeed)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ResetFrame) as usize - ptr as usize },
        164usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(ResetFrame)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ResetNonSharedRNGSeed) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(ResetNonSharedRNGSeed)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).StrafingAlgorithmPresent) as usize - ptr as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(StrafingAlgorithmPresent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Algorithm) as usize - ptr as usize },
        180usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Algorithm)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).AlgorithmParametersPresent) as usize - ptr as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(AlgorithmParametersPresent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Parameters) as usize - ptr as usize },
        188usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Parameters)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ChangePresent) as usize - ptr as usize },
        220usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(ChangePresent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Target) as usize - ptr as usize },
        221usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Target)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ChangeFinalValue) as usize - ptr as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(ChangeFinalValue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ChangeOver) as usize - ptr as usize },
        228usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(ChangeOver)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).TargetYawOverride) as usize - ptr as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(TargetYawOverride)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).TargetYawOverrideCount) as usize - ptr as usize },
        236usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(TargetYawOverrideCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).RenderYawOverride) as usize - ptr as usize },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(RenderYawOverride)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).RenderYawOverrideCount) as usize - ptr as usize },
        244usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(RenderYawOverrideCount)
        )
    );
}
