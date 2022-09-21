const expectEqual = @import("std").testing.expectEqual;
const expect = @import("std").testing.expect;

test "simple while" {
    var i: usize = 0;
    while (i < 10) {
        i += 1;
        if (i == 7) {
            break;
        }
    }
    try expectEqual(@intCast(usize, 7), i);

    i = 0;
    while (i < 10) : (i += 1) {}
    try expectEqual(@intCast(usize, 10), i);

    i = 0;
    var x: i32 = while (i < 10) : (i += 1) {
        if (i == 4) {
            break 1;
        }
    } else 0;
    try expectEqual(@intCast(i32, 1), x);

    outer: while (true) {
        while (true) {
            break :outer;
        }
    }
}

fn isBreakNumber(i: u32) ?u32 {
    if (i == 3) {
        return null;
    }
    return i + 1;
}

test "while null capture" {
    var i: u32 = 0;
    while (isBreakNumber(i)) |value| {
        i = value;
    }
    try expectEqual(@intCast(u32, 3), i);
}

fn isErrorNumber(i: u32) anyerror!u32 {
    if (i == 0) {
        return error.ReachedZero;
    }
    return i - 1;
}

test "while error capture" {
    var i: u32 = 5;
    while (isErrorNumber(i)) |value| {
        i = value;
    } else |err| {
        try expect(err == error.ReachedZero);
    }
    try expectEqual(@intCast(u32, 0), i);
}
