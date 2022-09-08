//! This is Top Level doc comments for module

const std = @import("std");

/// This is top doc comment
pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("Hello, {s}!\n", .{"world"});
    // This is normal comment
    std.debug.print("This is '{s}' !\n", .{"compile time word"});
}
