const std = @import("std");

pub fn main() !void {
    const allocator = std.heap.page_allocator;

    // JSON data
    const json_data =
        \\{
        \\    "name": "Mani",
        \\    "age": 25,
        \\    "city": "Zigland"
        \\}
    ;

    // Open file for writing
    var file = try std.fs.cwd().createFile("mani.json", .{ .read = true });
    defer file.close();

    // Write JSON data
    try file.writeAll(json_data);
    std.debug.print("JSON data written to mani.json\n", .{});

    // Open file for reading
    var file_read = try std.fs.cwd().openFile("mani.json", .{});
    defer file_read.close();

    // Read file contents
    var buffer: [256]u8 = undefined;
    const bytes_read = try file_read.readAll(&buffer);
    const content = buffer[0..bytes_read];

    // Print file contents
    std.debug.print("Read from file: {s}\n", .{content});
}