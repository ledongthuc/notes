const expect = @import("std").testing.expect;
const std = @import("std");

test "basic type coercion" {
    var a: u8 = 1;
    var b: u16 = a;
    // unsigned 8-bit int cannot represent all possible unsigned 16-bit values
    // var c: u8 = b;
    var d: u16 = @as(u16, a);
    // unsigned 8-bit int cannot represent all possible unsigned 16-bit values
    // var e: u8 = @as(u16, a);

    try expect(a == 1);
    try expect(b == 1);
    // expect(c == 1);
    try expect(d == 1);
    // try expect(e == 1);
}

test "stricter qualification type coercion" {
    const c: u8 = 1;
    var v1: u8 = c;
    try expect(v1 == 1);
    try expect(@TypeOf(v1) == u8);

    var v2: *volatile u8 = &v1;
    try expect(@TypeOf(v2) == *volatile u8);
    try expect(v2.* == 1);

    const father1Error = error{
        kid1Error,
        kid2Error,
    };
    const father2Error = error{
        kid1Error,
    };
    var e = father1Error.kid1Error;
    e = father1Error.kid1Error;
    e = father2Error.kid1Error;
    try expect(e == father1Error.kid1Error);
}

test "float type coercion" {
    var a: f16 = 12.34;
    var b: f32 = a;
    var c: f64 = b;
    var d: f128 = c;
    try expect(d == a);

    // ambigous: float value 54.000000 cannot be coerced to type 'comptime_int'
    // var f: f32 = 54.0 / 5;

    var f1: f32 = 54.0 / @as(f32, 5);
    try expect(f1 == 10.8);

    var f2: f32 = @as(i32, 54.0) / 5;
    _ = f2;
}

// Constant pointer of array => slice
test "*const [N]T to []const T" {
    var x1: []const u8 = "hello";
    var x2: []const u8 = &[5]u8{ 'h', 'e', 'l', 'l', 111 };
    try expect(std.mem.eql(u8, x1, x2));

    var y: []const f32 = &[2]f32{ 1.2, 3.4 };
    try expect(y[0] == 1.2);
}

// Constant pointer of array => Error Union of slice
test "*const [N]T to E![]const T" {
    var x1: anyerror![]const u8 = "hello";
    var x2: anyerror![]const u8 = &[5]u8{ 'h', 'e', 'l', 'l', 111 };
    try expect(std.mem.eql(u8, try x1, try x2));

    var y: anyerror![]const f32 = &[2]f32{ 1.2, 3.4 };
    try expect((try y)[0] == 1.2);
}

// Constant pointer of array => optional slice
test "*const [N]T to ?[]const T" {
    var x1: ?[]const u8 = "hello";
    var x2: ?[]const u8 = &[5]u8{ 'h', 'e', 'l', 'l', 111 };
    try expect(std.mem.eql(u8, x1.?, x2.?));

    var y: ?[]const f32 = &[2]f32{ 1.2, 3.4 };
    try expect(y.?[0] == 1.2);
}

// pointer of array to slice
test "*[N]T to []T" {
    var buf: [5]u8 = "hello".*;
    const x: []u8 = &buf;
    try expect(std.mem.eql(u8, x, "hello"));

    const buf2 = [2]f32{ 1.2, 3.4 };
    const x2: []const f32 = &buf2;
    try expect(std.mem.eql(f32, x2, &[2]f32{ 1.2, 3.4 }));
}

// single pointer to array => many-item pointers
test "*[N]T to [*]T" {
    var buf: [5]u8 = "hello".*;
    const x: [*]u8 = &buf;
    try expect(x[4] == 'o');
    _ = x[5]; // would be an uncaught out of bounds pointer dereference!
}

test "error and optional" {
    var i: anyerror!?i32 = 1234;
    try expect((try i).? == 1234);
    i = null;
    try expect((try i) == null);
    i = error.FileNotFound;
}

test "comptime big number -> small number if fit" {
    const n1: i32 = 100;
    const n2: i8 = n1;
    // integer value 100 cannot be coerced to type 'i2'
    // const n3: i2 = n1;
    try expect(n2 == 100);
    // try expect(n3 == 100);
}

const E = enum {
    one,
    two,
    three,
};

const U = union(E) {
    one: i32,
    two: f32,
    three,
};

test "coercion between unions and enums" {
    var u = U{ .two = 12.34 };
    var e: E = u;
    try expect(e == E.two);

    const three = E.three;
    var another_u: U = three;
    try expect(another_u == E.three);
}
