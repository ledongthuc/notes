const print = @import("std").debug.print;

pub fn main() !void {
    const integer: i32 = 1 + 1;
    print("1 + 1 = {}\n", .{integer});

    const ptrInteger: *const i32 = &integer;
    print("pointer of integer = {}\n", .{ptrInteger});
    // const ptrInteger2: anyopaque = undefined;
    // print("pointer of integer = {}\n", .{ptrInteger2});

    const floats: f32 = 7.0 / 3.0;
    print("7.0 / 3.0 = {}\n", .{floats});

    // const floats2: f32 = 7.0 / 0.0;
    // print("7.0 / 0 = {}\n", .{floats2});

    // const floats3: f32 = 7.0 / 3;
    // print("7.0 / 3 = {}\n", .{floats3});

    const floats3: f32 = 7.0 / @as(f32, 3);
    print("7.0 / 3 = {}\n", .{floats3});

    var optional_v: ?f32 = null;
    print("optional_v = {}, is null: {}\n", .{ optional_v, optional_v == null });
    optional_v = 5.0;
    print("optional_v = {}, is null: {}\n", .{ optional_v, optional_v == null });

    var v_or_err: anyerror!i32 = error.ArgNotFound;
    print("type: {s}, value: {}\n", .{ @typeName(@TypeOf(v_or_err)), v_or_err });
    v_or_err = 123;
    print("type: {s}, value: {}\n", .{ @typeName(@TypeOf(v_or_err)), v_or_err });

    var undefined_i: i32 = undefined;
    print("undefined_i = {}\n", .{undefined_i});
    var undefined_f: f32 = undefined;
    print("undefined_f = {}\n", .{undefined_f});
    var undefined_b: bool = undefined;
    print("undefined_b = {}\n", .{undefined_b});
    // var null_i: bool = null;
    // print("null_i = {}\n", .{null_i});

    const bytes = "hello";
    print("{s}\n", .{@typeName(@TypeOf(bytes))}); // *const [5:0]u8
    print("{d}\n", .{bytes.len}); // 5
    print("{c}\n", .{bytes[1]}); // 'e'
    print("{d}\n", .{bytes[5]}); // 0

    const multilines =
        \\ Line 1
        \\ Line 2
    ;
    print("multilines: {s}", .{multilines});
}
