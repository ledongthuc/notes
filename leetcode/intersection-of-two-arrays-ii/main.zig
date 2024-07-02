const std = @import("std");

pub fn main() void {
    _ = intersect(&[3]i32{ 4, 9, 5 }, &[5]i32{ 9, 4, 9, 8, 4 });
}

fn intersect(nums1: []const i32, nums2: []const i32) []i32 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer gpa.deinit();

    const allocator = gpa.allocator();

    var frequency_arr = try allocator.alloc(i32, 1001);
    defer allocator.free(frequency_arr);
    std.mem.set(frequency_arr[0..], 0);

    for (nums1) |item| {
        frequency_arr[item] += 1;
    }

    var result = try allocator.alloc(i32, 0);
    defer allocator.free(result);

    for (nums2) |item| {
        if (frequency_arr[item] > 0) {
            try result.append(item);
        }
        frequency_arr[item] -= 1;
    }

    return result;
}
