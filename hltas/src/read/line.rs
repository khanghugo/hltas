use std::{num::NonZeroU32, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{anychar, char, digit0, not_line_ending, one_of, space1},
    combinator::{cut, map, map_res, not, opt, peek, recognize, verify},
    number::complete::recognize_float,
    sequence::{pair, preceded, separated_pair, tuple},
};

use crate::{
    read::{
        context,
        properties::{non_shared_seed, property, shared_seed},
        Context, IResult,
    },
    types::*,
};

fn strafe_type(i: &str) -> IResult<StrafeType> {
    alt((
        map(char('0'), |_| StrafeType::MaxAccel),
        map(char('1'), |_| StrafeType::MaxAngle),
        map(char('2'), |_| StrafeType::MaxDeccel),
        map(char('3'), |_| StrafeType::ConstantSpeed),
    ))(i)
}

fn strafe_dir(i: &str) -> IResult<StrafeDir> {
    // The actual values for Yaw, Point and Line are filled in later, while parsing the yaw field.
    alt((
        map(char('0'), |_| StrafeDir::Left),
        map(char('1'), |_| StrafeDir::Right),
        map(char('2'), |_| StrafeDir::Best),
        map(char('3'), |_| StrafeDir::Yaw(0.)),
        map(char('4'), |_| StrafeDir::Point { x: 0., y: 0. }),
        map(char('5'), |_| StrafeDir::Line { yaw: 0. }),
    ))(i)
}

fn strafe_settings(i: &str) -> IResult<StrafeSettings> {
    map(tuple((strafe_type, strafe_dir)), |(type_, dir)| {
        StrafeSettings { type_, dir }
    })(i)
}

fn strafe(i: &str) -> IResult<Option<StrafeSettings>> {
    alt((
        map(tag("---"), |_| None),
        map(preceded(char('s'), strafe_settings), Some),
    ))(i)
}

fn non_zero_u32(i: &str) -> IResult<NonZeroU32> {
    map_res(
        recognize(pair(one_of("123456789"), digit0)),
        NonZeroU32::from_str,
    )(i)
}

fn times(i: &str) -> IResult<u32> {
    let (i, times) = opt(non_zero_u32)(i)?;
    Ok((i, times.map(NonZeroU32::get).unwrap_or(0)))
}

fn lgagst_action_speed(i: &str) -> IResult<LeaveGroundActionSpeed> {
    alt((
        map(char('l'), |_| LeaveGroundActionSpeed::Optimal),
        map(char('L'), |_| {
            LeaveGroundActionSpeed::OptimalWithFullMaxspeed
        }),
    ))(i)
}

fn lgagst_action(i: &str) -> IResult<LeaveGroundAction> {
    let (i, speed) = lgagst_action_speed(i)?;
    let (i, times) = times(i)?;

    // Check for the both autojump and ducktap error.
    cut(context(
        Context::BothAutoJumpAndDuckTap,
        not(peek(tuple((char('j'), alt((char('d'), char('D'))))))),
    ))(i)?;

    cut(alt((
        map(tag("j-"), move |_| LeaveGroundAction {
            speed,
            times,
            type_: LeaveGroundActionType::Jump,
        }),
        map(tag("-d"), move |_| LeaveGroundAction {
            speed,
            times,
            type_: LeaveGroundActionType::DuckTap { zero_ms: false },
        }),
        map(tag("-D"), move |_| LeaveGroundAction {
            speed,
            times,
            type_: LeaveGroundActionType::DuckTap { zero_ms: true },
        }),
    )))(i)
}

fn non_lgagst_action(i: &str) -> IResult<LeaveGroundAction> {
    let (i, _) = char('-')(i)?;

    // Check for the both autojump and ducktap error.
    cut(context(
        Context::BothAutoJumpAndDuckTap,
        not(peek(tuple((char('j'), times, alt((char('d'), char('D'))))))),
    ))(i)?;

    alt((
        map(tuple((char('j'), times, char('-'))), |(_, times, _)| {
            LeaveGroundAction {
                speed: LeaveGroundActionSpeed::Any,
                times,
                type_: LeaveGroundActionType::Jump,
            }
        }),
        map(pair(tag("-d"), times), |(_, times)| LeaveGroundAction {
            speed: LeaveGroundActionSpeed::Any,
            times,
            type_: LeaveGroundActionType::DuckTap { zero_ms: false },
        }),
        map(pair(tag("-D"), times), |(_, times)| LeaveGroundAction {
            speed: LeaveGroundActionSpeed::Any,
            times,
            type_: LeaveGroundActionType::DuckTap { zero_ms: true },
        }),
    ))(i)
}

fn leave_ground_action(i: &str) -> IResult<Option<LeaveGroundAction>> {
    alt((
        map(lgagst_action, Some),
        map(non_lgagst_action, Some),
        map(tag("---"), |_| None),
    ))(i)
}

fn jump_bug(i: &str) -> IResult<Option<JumpBug>> {
    alt((
        map(char('-'), |_| None),
        map(preceded(char('b'), times), |times| Some(JumpBug { times })),
    ))(i)
}

fn duck_before_collision(i: &str) -> IResult<Option<DuckBeforeCollision>> {
    alt((
        map(char('-'), |_| None),
        map(preceded(char('c'), times), |times| {
            Some(DuckBeforeCollision {
                times,
                including_ceilings: false,
            })
        }),
        map(preceded(char('C'), times), |times| {
            Some(DuckBeforeCollision {
                times,
                including_ceilings: true,
            })
        }),
    ))(i)
}

fn duck_before_ground(i: &str) -> IResult<Option<DuckBeforeGround>> {
    alt((
        map(char('-'), |_| None),
        map(preceded(char('g'), times), |times| {
            Some(DuckBeforeGround { times })
        }),
    ))(i)
}

fn duck_when_jump(i: &str) -> IResult<Option<DuckWhenJump>> {
    alt((
        map(char('-'), |_| None),
        map(preceded(char('w'), times), |times| {
            Some(DuckWhenJump { times })
        }),
    ))(i)
}

fn auto_actions(i: &str) -> IResult<AutoActions> {
    let (i, strafe) = strafe(i)?;
    let (i, leave_ground_action) = cut(leave_ground_action)(i)?;
    let (i, jump_bug) = cut(jump_bug)(i)?;
    let (i, duck_before_collision) = cut(duck_before_collision)(i)?;
    let (i, duck_before_ground) = cut(duck_before_ground)(i)?;
    let (i, duck_when_jump) = cut(duck_when_jump)(i)?;
    Ok((
        i,
        AutoActions {
            yaw_adjustment: strafe.map(YawAdjustment::Strafe),
            leave_ground_action,
            jump_bug,
            duck_before_collision,
            duck_before_ground,
            duck_when_jump,
        },
    ))
}

fn key<'a>(symbol: char) -> impl Fn(&'a str) -> IResult<bool> {
    alt((map(char(symbol), |_| true), map(char('-'), |_| false)))
}

fn movement_keys(i: &str) -> IResult<MovementKeys> {
    let (i, forward) = key('f')(i)?;
    let (i, left) = key('l')(i)?;
    let (i, right) = key('r')(i)?;
    let (i, back) = key('b')(i)?;
    let (i, up) = key('u')(i)?;
    let (i, down) = key('d')(i)?;
    Ok((
        i,
        MovementKeys {
            forward,
            left,
            right,
            back,
            up,
            down,
        },
    ))
}

fn action_keys(i: &str) -> IResult<ActionKeys> {
    let (i, jump) = key('j')(i)?;
    let (i, duck) = key('d')(i)?;
    let (i, use_) = key('u')(i)?;
    let (i, attack_1) = key('1')(i)?;
    let (i, attack_2) = key('2')(i)?;
    let (i, reload) = key('r')(i)?;
    Ok((
        i,
        ActionKeys {
            jump,
            duck,
            use_,
            attack_1,
            attack_2,
            reload,
        },
    ))
}

fn float(i: &str) -> IResult<f32> {
    verify(map_res(recognize_float, f32::from_str), |x| x.is_finite())(i)
}

/// Returns a parser for the yaw field given a `YawAdjustment`.
///
/// The yaw field contents depend on the strafing:
/// - If strafing is disabled, the yaw field can be either empty or contain one float (the yaw
///   angle).
/// - If strafing is enabled with Yaw or Line dir, then the yaw field should contain one float (the
///   yaw angle).
/// - If strafing is enabled with Point dir, the yaw field should contain two floats (X and Y
///   coordinates).
/// - If strafing is enabled with other dirs, the yaw field should be empty.
fn yaw_field<'a>(
    yaw_adjustment: Option<YawAdjustment>,
) -> impl Fn(&'a str) -> IResult<Option<YawAdjustment>> {
    move |i: &str| match yaw_adjustment {
        None => {
            let (i, yaw) = alt((map(float, Some), map(char('-'), |_| None)))(i)?;
            Ok((i, yaw.map(YawAdjustment::Set)))
        }
        Some(YawAdjustment::Strafe(StrafeSettings { dir, type_ })) => match dir {
            StrafeDir::Yaw(_) => {
                let (i, yaw) = float(i)?;
                Ok((
                    i,
                    Some(YawAdjustment::Strafe(StrafeSettings {
                        type_,
                        dir: StrafeDir::Yaw(yaw),
                    })),
                ))
            }
            StrafeDir::Line { .. } => {
                let (i, yaw) = float(i)?;
                Ok((
                    i,
                    Some(YawAdjustment::Strafe(StrafeSettings {
                        type_,
                        dir: StrafeDir::Line { yaw },
                    })),
                ))
            }
            StrafeDir::Point { .. } => {
                let (i, (x, y)) = separated_pair(float, space1, float)(i)?;
                Ok((
                    i,
                    Some(YawAdjustment::Strafe(StrafeSettings {
                        type_,
                        dir: StrafeDir::Point { x, y },
                    })),
                ))
            }
            dir => {
                let (i, _) = char('-')(i)?;
                Ok((
                    i,
                    Some(YawAdjustment::Strafe(StrafeSettings { type_, dir })),
                ))
            }
        },
        _ => unreachable!(),
    }
}

fn pitch(i: &str) -> IResult<Option<f32>> {
    alt((map(float, Some), map(char('-'), |_| None)))(i)
}

fn frame_count(i: &str) -> IResult<NonZeroU32> {
    alt((
        map(char('-'), |_| NonZeroU32::new(1).unwrap()), // Backwards compatibility.
        map(char('0'), |_| NonZeroU32::new(1).unwrap()), // Backwards compatibility.
        non_zero_u32,
    ))(i)
}

fn line_frame_bulk(i: &str) -> IResult<FrameBulk> {
    // Mutable because the yaw_adjustment parameter will be filled in later.
    let (i, mut auto_actions) = auto_actions(i)?;
    // Backwards compatibility: HLTAS didn't check the first field length, so extra characters were
    // permitted.
    let (i, _) = opt(is_not("|"))(i)?;

    let (i, movement_keys) = cut(preceded(char('|'), movement_keys))(i)?;
    let (i, action_keys) = cut(preceded(char('|'), action_keys))(i)?;
    let (i, frame_time) = cut(preceded(char('|'), recognize_float))(i)?;

    // Parse the yaw field and get the updated yaw_adjustment.
    let (i, new_yaw_adjustment) =
        cut(preceded(char('|'), yaw_field(auto_actions.yaw_adjustment)))(i)?;
    auto_actions.yaw_adjustment = new_yaw_adjustment;

    let (i, pitch) = cut(preceded(char('|'), pitch))(i)?;
    let (i, frame_count) = cut(preceded(char('|'), frame_count))(i)?;

    // The console command field is optional.
    let (i, console_command) = opt(preceded(char('|'), not_line_ending))(i)?;

    Ok((
        i,
        FrameBulk {
            auto_actions,
            movement_keys,
            action_keys,
            frame_time,
            pitch,
            frame_count,
            console_command,
        },
    ))
}

fn line_save(i: &str) -> IResult<&str> {
    let (i, (name, value)) = property(i)?;
    tag("save")(name)?;
    cut(context(Context::NoSaveName, anychar))(value)?;
    Ok((i, value))
}

fn line_seed(i: &str) -> IResult<u32> {
    let (i, (name, value)) = property(i)?;
    tag("seed")(name)?;
    cut(context(Context::NoSeed, anychar))(value)?;
    let (_, seed) = cut(shared_seed)(value)?;
    Ok((i, seed))
}

fn button(i: &str) -> IResult<Button> {
    alt((
        map(char('0'), |_| Button::Forward),
        map(char('1'), |_| Button::ForwardLeft),
        map(char('2'), |_| Button::Left),
        map(char('3'), |_| Button::BackLeft),
        map(char('4'), |_| Button::Back),
        map(char('5'), |_| Button::BackRight),
        map(char('6'), |_| Button::Right),
        map(char('7'), |_| Button::ForwardRight),
    ))(i)
}

fn buttons(i: &str) -> IResult<Buttons> {
    cut(context(Context::NoButtons, preceded(space1, anychar)))(i)?;
    let (i, air_left) = preceded(space1, button)(i)?;
    cut(context(Context::NoButtons, preceded(space1, anychar)))(i)?;
    let (i, air_right) = preceded(space1, button)(i)?;
    cut(context(Context::NoButtons, preceded(space1, anychar)))(i)?;
    let (i, ground_left) = preceded(space1, button)(i)?;
    cut(context(Context::NoButtons, preceded(space1, anychar)))(i)?;
    let (i, ground_right) = preceded(space1, button)(i)?;
    Ok((
        i,
        Buttons::Set {
            air_left,
            air_right,
            ground_left,
            ground_right,
        },
    ))
}

fn line_buttons(i: &str) -> IResult<Buttons> {
    let (i, _) = tag("buttons")(i)?;

    if preceded(space1::<&str, ()>, not_line_ending)(i).is_ok() {
        cut(buttons)(i)
    } else {
        Ok((i, Buttons::Reset))
    }
}

fn line_lgagst_min_speed(i: &str) -> IResult<f32> {
    let (i, (name, value)) = property(i)?;
    tag("lgagstminspeed")(name)?;
    cut(context(Context::NoLGAGSTMinSpeed, anychar))(value)?;
    let (_, lgagst_min_speed) = cut(float)(value)?;
    Ok((i, lgagst_min_speed))
}

fn line_reset(i: &str) -> IResult<i64> {
    let (i, (name, value)) = property(i)?;
    tag("reset")(name)?;
    cut(context(Context::NoResetSeed, anychar))(value)?;
    let (_, seed) = cut(non_shared_seed)(value)?;
    Ok((i, seed))
}

fn line_comment(i: &str) -> IResult<&str> {
    preceded(tag("//"), not_line_ending)(i)
}

pub(crate) fn line(i: &str) -> IResult<Line> {
    alt((
        map(line_frame_bulk, Line::FrameBulk),
        map(line_save, Line::Save),
        map(line_seed, Line::SharedSeed),
        map(line_buttons, Line::Buttons),
        map(line_lgagst_min_speed, Line::LGAGSTMinSpeed),
        map(line_reset, |non_shared_seed| Line::Reset {
            non_shared_seed,
        }),
        map(line_comment, Line::Comment),
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_save_name() {
        let input = "save ";
        let err = line_save(input).unwrap_err();
        if let nom::Err::Failure(err) = err {
            assert_eq!(err.context, Some(Context::NoSaveName));
        } else {
            unreachable!()
        }
    }

    #[test]
    fn no_seed() {
        let input = "seed ";
        let err = line_seed(input).unwrap_err();
        if let nom::Err::Failure(err) = err {
            assert_eq!(err.context, Some(Context::NoSeed));
        } else {
            unreachable!()
        }
    }

    #[test]
    fn no_buttons() {
        let input = "buttons ";
        let err = line_buttons(input).unwrap_err();
        if let nom::Err::Failure(err) = err {
            assert_eq!(err.context, Some(Context::NoButtons));
        } else {
            unreachable!()
        }
    }

    #[test]
    fn no_buttons_1() {
        let input = "buttons 1";
        let err = line_buttons(input).unwrap_err();
        if let nom::Err::Failure(err) = err {
            assert_eq!(err.context, Some(Context::NoButtons));
        } else {
            unreachable!()
        }
    }

    #[test]
    fn buttons_reset() {
        let input = "buttons";
        assert_eq!(line_buttons(input), Ok(("", Buttons::Reset)));
    }

    #[test]
    fn no_lgagst_min_speed() {
        let input = "lgagstminspeed ";
        let err = line_lgagst_min_speed(input).unwrap_err();
        if let nom::Err::Failure(err) = err {
            assert_eq!(err.context, Some(Context::NoLGAGSTMinSpeed));
        } else {
            unreachable!()
        }
    }

    #[test]
    fn no_reset_seed() {
        let input = "reset ";
        let err = line_reset(input).unwrap_err();
        if let nom::Err::Failure(err) = err {
            assert_eq!(err.context, Some(Context::NoResetSeed));
        } else {
            unreachable!()
        }
    }

    #[test]
    fn both_autojump_ducktap() {
        let input = "----jd----";
        let err = auto_actions(input).unwrap_err();
        if let nom::Err::Failure(err) = err {
            assert_eq!(err.context, Some(Context::BothAutoJumpAndDuckTap));
        } else {
            unreachable!()
        }
    }

    #[test]
    fn both_autojump_ducktap_lgagst() {
        let input = "---ljd----";
        let err = auto_actions(input).unwrap_err();
        if let nom::Err::Failure(err) = err {
            assert_eq!(err.context, Some(Context::BothAutoJumpAndDuckTap));
        } else {
            unreachable!()
        }
    }
}
