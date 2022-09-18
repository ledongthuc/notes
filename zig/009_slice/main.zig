const expectEqual = @import("std").testing.expectEqual;

test "basic slice" {
    var array = [_]i32{ 1, 2, 3, 4 };
    var known_at_runtime_start: usize = 1;
    var known_at_runtime_end: usize = 3;
    const slice = array[known_at_runtime_start..known_at_runtime_end];
    // slice[10] += 1; //outbound index - runtime

    try expectEqual([]i32, @TypeOf(slice));
    try expectEqual(&array[1], &slice[0]);
    try expectEqual(known_at_runtime_end - known_at_runtime_start, slice.len);
    try expectEqual([*]i32, @TypeOf(slice.ptr));
    try expectEqual(i32, @TypeOf(slice[0]));

    const pointer_to_slice = array[1..3];
    try expectEqual(*[2]i32, @TypeOf(pointer_to_slice));
}

test "null terminated slice" {
    const slice: [:0]const u8 = "hello";
    try expectEqual(5, slice.len);
    try expectEqual(0, slice[5]);

    const slice2: [:0]const u8 = "hello";
    try expectEqual(5, slice2.len);
    try expectEqual(0, slice2[5]);

    var array = [_]u8{ 3, 2, 1, 99, 3, 2, 1, 0 };
    var runtime_length: usize = 3;
    const slice3 = array[0..runtime_length :99];
    // const slice4 = array[0 .. runtime_length + 1 :99]; // Runtime failed because of invalid terminated 99

    try expectEqual([:99]u8, @TypeOf(slice3));
    try expectEqual(@intCast(usize, 3), slice3.len);
}
