const std = @import("std");
const print = std.debug.print;

const Point2 = struct {
    x: i32,
    y: i32,
    z: i32,
};

pub fn main() !void {
    var another_i1: i32 = 1;
    var another_i2: i32 = 4;
    var another_i3: i32 = 7;
    var another_i4: i32 = 12;

    print("Address of another_i1: {}\n", .{@ptrToInt(&another_i1)});
    print("Address of another_i2: {}\n", .{@ptrToInt(&another_i2)});
    print("Address of another_i3: {}\n", .{@ptrToInt(&another_i3)});
    print("Address of another_i4: {}\n", .{@ptrToInt(&another_i4)});

    var wrong_pointer_of_struct = @fieldParentPtr(Point2, "y", &another_i3);
    print("Address of wrong_pointer_of_struct: {}\n", .{@ptrToInt(wrong_pointer_of_struct)});
    print("Address of wrong_pointer_of_struct.x: {}\n", .{@ptrToInt(&wrong_pointer_of_struct.x)});
    print("Address of wrong_pointer_of_struct.y: {}\n", .{@ptrToInt(&wrong_pointer_of_struct.y)});
    print("Address of wrong_pointer_of_struct.z: {}\n", .{@ptrToInt(&wrong_pointer_of_struct.z)});

    wrong_pointer_of_struct.x = 100;
    wrong_pointer_of_struct.y = 101;
    wrong_pointer_of_struct.z = 102;
    print("wrong_pointer_of_struct.x: {}\n", .{wrong_pointer_of_struct.x});
    print("wrong_pointer_of_struct.y: {}\n", .{wrong_pointer_of_struct.y});
    print("wrong_pointer_of_struct.z: {}\n", .{wrong_pointer_of_struct.z});
    print("another_i1: {}\n", .{another_i1});
    print("another_i2: {}\n", .{another_i2});
    print("another_i3: {}\n", .{another_i3});
    print("another_i4: {}\n", .{another_i4});
}

// Output:

// DEBUG mode:
// zig run hello.zig
// Address of another_i1: 140702020582356
// Address of another_i2: 140702020582352
// Address of another_i3: 140702020582348
// Address of another_i4: 140702020582344
// Address of wrong_pointer_of_struct: 140702020582344
// Address of wrong_pointer_of_struct.x: 140702020582344
// Address of wrong_pointer_of_struct.y: 140702020582348
// Address of wrong_pointer_of_struct.z: 140702020582352
// wrong_pointer_of_struct.x: 100
// wrong_pointer_of_struct.y: 101
// wrong_pointer_of_struct.z: 102
// another_i1: 1
// another_i2: 102
// another_i3: 101
// another_i4: 100
//
//
//
// ReleaseFast mode:
// zig run hello.zig -O ReleaseFast
// Address of another_i1: 140701985938824
// Address of another_i2: 140701985938820
// Address of another_i3: 140701985938812
// Address of another_i4: 140701985938816
// Address of wrong_pointer_of_struct: 140701985938808
// Address of wrong_pointer_of_struct.x: 140701985938808
// Address of wrong_pointer_of_struct.y: 140701985938812
// Address of wrong_pointer_of_struct.z: 140701985938816
// wrong_pointer_of_struct.x: 100
// wrong_pointer_of_struct.y: 101
// wrong_pointer_of_struct.z: 102
// another_i1: 1
// another_i2: 4
// another_i3: 101
// another_i4: 102
//
//
//
// ReleaseSafe mode:
// zig run hello.zig -O ReleaseSafe
// Address of another_i1: 140702026869868
// Address of another_i2: 140702026869864
// Address of another_i3: 140702026869860
// Address of another_i4: 140702026869856
// Address of wrong_pointer_of_struct: 140702026869856
// Address of wrong_pointer_of_struct.x: 140702026869856
// Address of wrong_pointer_of_struct.y: 140702026869860
// Address of wrong_pointer_of_struct.z: 140702026869864
// wrong_pointer_of_struct.x: 100
// wrong_pointer_of_struct.y: 101
// wrong_pointer_of_struct.z: 102
// another_i1: 1
// another_i2: 102
// another_i3: 101
// another_i4: 100
//
//
//
// ReleaseSmall mode:
// zig run hello.zig -O ReleaseSmall
// Address of another_i1: 140702020697548
// Address of another_i2: 140702020697544
// Address of another_i3: 140702020697536
// Address of another_i4: 140702020697540
// Address of wrong_pointer_of_struct: 140702020697532
// Address of wrong_pointer_of_struct.x: 140702020697532
// Address of wrong_pointer_of_struct.y: 140702020697536
// Address of wrong_pointer_of_struct.z: 140702020697540
// wrong_pointer_of_struct.x: 100
// wrong_pointer_of_struct.y: 101
// wrong_pointer_of_struct.z: 102
// another_i1: 1
// another_i2: 4
// another_i3: 101
// another_i4: 102
