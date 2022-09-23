const expectEqual = @import("std").testing.expectEqual;
const expectError = @import("std").testing.expectError;

test "defer" {
    var i: i32 = 1;
    {
        defer i = 2;
        defer i = 5;
        i = 3;
    }

    try expectEqual(@intCast(i32, 2), i);
}

var x: i32 = 100;

test "errdefer" {
    try expectError(error.Overflow, errdeferHelper(error.Overflow));
    try expectEqual(@intCast(i32, 101), x);
}

fn errdeferHelper(err: anyerror) !void {
    errdefer {
        x += 1;
    }
    return err;
}
