const std = @import("std");
const print = std.debug.print;
const expect = @import("std").testing.expect;

pub fn main() !void {
    var y: i32 = 123;
    const x = blk: {
        y += 1;
        break :blk y;
    };
    try expect(x == 124);
    try expect(y == 124);

    {
        const pi = 3.14;
        _ = pi;
    }
    {
        var pi: bool = true;
        _ = pi;
    }
}
