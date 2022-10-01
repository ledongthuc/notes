const expect = @import("std").testing.expect;
const std = @import("std");
const maxInt = std.math.maxInt;

const FileOpenError = error{
    AccessDenied,
    OutOfMemory,
    FileNotFound,
};

const AllocationError = error{
    OutOfMemory,
};

const TestError = error.OutOfMemory;
const TestError2 = error{OutOfMemory}.OutOfMemory;

test "basic error" {
    try expect(@TypeOf(FileOpenError.AccessDenied) == FileOpenError);
}

pub fn mustContent(buf: []const u8) !u64 {
    errdefer {
        std.debug.print("Error with zero length", .{});
    }
    if (buf.len == 0) {
        return error.ZeroError;
    }
    return buf.len;
}

test "parse u64" {
    const result = try mustContent("1234");
    try expect(result == 4);

    const result2 = mustContent("") catch 0;
    try expect(result2 == 0);

    if (mustContent("")) |_| {} else |err| switch (err) {
        error.ZeroError => {
            std.debug.print("Zero error", .{});
        },
    }
}

const A = error{
    NotDir,

    /// A doc comment
    PathNotFound,
};
const B = error{
    OutOfMemory,

    /// B doc comment
    PathNotFound,
};

const C = A || B;

fn foo() C!void {
    return error.NotDir;
}

test "merge error sets" {
    if (foo()) {
        @panic("unexpected");
    } else |err| switch (err) {
        error.OutOfMemory => @panic("unexpected"),
        error.PathNotFound => @panic("unexpected"),
        error.NotDir => {},
    }
}
