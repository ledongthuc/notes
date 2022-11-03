const std = @import("std");
const expect = std.testing.expect;

pub fn main() !void {
    var general_purpose_allocator = std.heap.GeneralPurposeAllocator(.{}){};
    const gpa = general_purpose_allocator.allocator();
    const u32_ptr = try gpa.create(u32);
    _ = u32_ptr;
}
