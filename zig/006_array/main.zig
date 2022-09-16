const std = @import("std");
const assert = @import("std").debug.assert;

const Point = struct {
    x: i32,
    y: i32,
};

fn makePoint(x: i32) Point {
    return Point{
        .x = x,
        .y = x * 2,
    };
}

pub fn main() !void {
    const arr_str = [_]u8{ 'h', 'e', 'l', 'l', 'o' };
    comptime {
        assert(arr_str.len == 5);
    }

    for (arr_str) |byte, i| {
        std.debug.print("byte[{}]: {}\n", .{ i, byte });
    }

    var fancy_array = init: {
        var initial_value: [10]Point = undefined;
        for (initial_value) |*pt, i| {
            pt.* = Point{
                .x = @intCast(i32, i),
                .y = @intCast(i32, i) * 2,
            };
        }
        break :init initial_value;
    };
    std.debug.print("fancy_array: {array}\n", .{fancy_array});

    var fancy_array2 = [_]Point{makePoint(3)} ** 10;
    std.debug.print("fancy_array2: {array}\n", .{fancy_array2});

    var array: [4]u8 = .{ 11, 22, 33, 44 };
    std.debug.print("array: {}\n", .{array[0]});

    var struct_obj = .{ 11, 22, 33, 44 };
    std.debug.print("struct_obj.@\"0\": {}\n", .{struct_obj.@"0"});

    var sentinelTerminated = [_:9]u8{ 1, 2, 3, 4 };
    for (sentinelTerminated) |item, index| {
        std.debug.print("sentinelTerminated[{}]: {}\n", .{ index, item });
    }
    std.debug.print("sentinelTerminated[4]: {}\n", .{sentinelTerminated[4]});
}
