const std = @import("std");

fn paper_needed(dimensions: []const usize) anyerror!usize {
    return dimensions[0];
}

pub fn main() anyerror!void {
    var timer = try std.time.Timer.start();
    const t0 = timer.lap();
    const os = std.io.getStdOut().writer();

    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();

    const input_file = try std.fs.cwd().openFile(
        "../inputs/day02.txt",
        .{ .read = true },
    );
    defer input_file.close();

    const input = try input_file.reader().readAllAlloc(&arena.allocator, 1024 * 8);
    var n: usize = 0;
    var ribbon: usize = 0;
    var lines = std.mem.tokenize(input, "\n");
    while (lines.next()) |line| {
        if (line.len == 0) continue;
    }
    const t1 = timer.lap();
    try os.print("Part 1: {}\n", .{count});
    try os.print("Part 2: {}\n", .{part2});
    try os.print("Time:   {}μs\n", .{(t1 - t0) / std.time.ns_per_us});
}
