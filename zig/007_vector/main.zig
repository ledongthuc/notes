const std = @import("std");

pub fn main() !void {
    const a = @Vector(4, i32){ 1, 2, 3, 4 };
    const b = @Vector(4, i32){ 5, 6, 7, 8 };
    std.debug.print("a+b: {}\n", .{a + b});

    var arr: [4]i32 = [_]i32{ 1, 2, 3, 4 };
    var vec: @Vector(4, i32) = arr; //array => vector
    std.debug.print("vec: {}\n", .{vec});

    var vec2: @Vector(2, i32) = arr[1..3].*; //slice => vector
    std.debug.print("vec2: {}\n", .{vec2});

    var slice: []const i32 = &arr;
    var offset: u32 = 1;
    const vec3: @Vector(2, i32) = slice[offset..][0..2].*;
    std.debug.print("vec3: {}\n", .{vec3});
}
