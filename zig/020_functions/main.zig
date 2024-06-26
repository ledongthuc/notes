const expect = @import("std").testing.expect;
const builtin = @import("builtin");
const native_arch = builtin.cpu.arch;
const std = @import("std");

// normal function
fn add(a: i8, b: i8) i8 {
    if (a == 0) {
        return b;
    }

    return a + b;
}

// exported function that can use by C-ABI
export fn sub(a: i8, b: i8) i8 {
    return a - b;
}

// defines function that will be resolved by linker. linking statically, or at runtime, when linking dynamically
const WINAPI: std.builtin.CallingConvention = if (native_arch == .i386) .Stdcall else .C;
extern "kernel32" fn ExitProcess(exit_code: u32) callconv(WINAPI) noreturn;
extern "c" fn atan2(a: f64, b: f64) f64;

fn abort() noreturn {
    @setCold(true); // function rarely use
    while (true) {}
}

// Naked function. No prolog and epilog code
// https://www.ibm.com/docs/en/zos/2.3.0?topic=c-prolog-epilog-code
fn _start() callconv(.Naked) noreturn {
    abort();
}

inline fn shiftLeftOne(a: u32) u32 {
    return a << 1;
}

// public from another import
pub fn sub2(a: i8, b: i8) i8 {
    return a - b;
}

// function pointer is runtime-known signature
const call2_op = *const fn (a: i8, b: i8) i8;
fn do_op(fn_call: call2_op, op1: i8, op2: i8) i8 {
    return fn_call(op1, op2);
}

test "call function" {
    try expect(do_op(add, 5, 6) == 11);
    try expect(do_op(sub2, 5, 6) == -1);
}

const Point = struct {
    x: i32,
    y: i32,
};

fn foo(point: Point) i32 {
    // Struct, array, union can select pass by value or reference by compiler.
    return point.x + point.y;
}

test "pass struct to function" {
    try expect(foo(Point{ .x = 1, .y = 2 }) == 3);
}

fn addFortyTwo(x: anytype) @TypeOf(x) {
    return x + 42;
}

test "fn type inference" {
    try expect(addFortyTwo(1) == 43);
    try expect(@TypeOf(addFortyTwo(1)) == comptime_int);
    var y: i64 = 2;
    try expect(addFortyTwo(y) == 44);
    try expect(@TypeOf(addFortyTwo(y)) == i64);
}

test "fn reflection" {
    try expect(@typeInfo(@TypeOf(expect)).Fn.args[0].arg_type.? == bool);
    try expect(@typeInfo(@TypeOf(expect)).Fn.is_var_args == false);
}
