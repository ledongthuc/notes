const std = @import("std");

pub fn main() !void {
    // Integer
    var integer: i32 = std.math.maxInt(i32);
    std.debug.print("Integer: {}\n", .{integer});
    // std.debug.print("Integer + 1: {}\n", .{integer + 1}); // integer overflow
    std.debug.print("Integer +% 1: {}\n", .{integer +% 1}); // wrapping to min value
    std.debug.print("Integer +| 1: {}\n", .{integer +| 1}); // sarturating at max value
    var integer3Bits: i3 = std.math.maxInt(i3);
    std.debug.print("Integer 3 bits: {}\n", .{integer3Bits}); // 1 bit sign, 2 bits value
    std.debug.print("-%@as(i32, std.math.minInt(i32)): {}\n", .{-%@as(i32, std.math.minInt(i32))});
    // std.debug.print("-@as(i32, std.math.minInt(i32)): {}\n", .{-@as(i32, std.math.minInt(i32))}); //overflow

    // Float
    var integerTestFloat: i32 = 123.0; // coerce
    std.debug.print("integerTestFloat: {}\n", .{integerTestFloat}); // 1 bit sign, 2 bits value
    const float = 123.0;
    // var float = 123.0; // Requires const to compiler-known value. Only accept no fractional float coerce
    var integerTestFloat2: i32 = float;
    std.debug.print("integerTestFloat2: {}\n", .{integerTestFloat2});
    std.debug.print("integerTestFloat2: {}\n", .{integerTestFloat2});

    const value1: ?u32 = null;
    const value2: u32 = 222;
    std.debug.print("value1 orelse value2: {}\n", .{value1 orelse value2});
    // std.debug.print("value1.? : {}\n", .{value1.?}); // value1 is compiler known value that is null
}
