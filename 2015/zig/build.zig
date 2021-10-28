// shamelessly taken from travv0/AdventOfCode2016
const std = @import("std");
const builtin = @import("builtin");

pub fn build(b: *std.build.Builder) !void {
    const mode = b.standardReleaseOptions();
    const target = b.standardTargetOptions(.{});

    var days_list = std.ArrayList([]const u8).init(b.allocator);

    const cwd = try std.fs.cwd().openDir("src", .{ .iterate = true });
    var iter = cwd.iterate();
    while (try iter.next()) |entry| {
        const name = try b.allocator.dupe(u8, entry.name);
        if (name.len == 9 and std.mem.startsWith(u8, name, "day")) {
            try days_list.append(name[0..5]);
        }
    }

    const days = days_list.toOwnedSlice();
    std.sort.sort([]const u8, days, {}, strLessThan);

    var build_zig_path = [_][]const u8{ "build.zig", "src" };
    const fmt_paths = try std.mem.concat(b.allocator, []const u8, &[_][][]const u8{
        &build_zig_path,
    });
    const fmt = b.addFmt(fmt_paths);

    const build_all_step = b.step("install", "Build executables for all days.");
    const test_all_step = b.step("test", "Run all tests.");
    const run_all_step = b.step("run", "Run all days.");
    for (days) |day| {
        const src = try std.fmt.allocPrint(b.allocator, "src/{s}.zig", .{day});
        const exe = b.addExecutable(day, src);
        const tests = b.addTest(src);

        exe.setTarget(target);
        tests.setTarget(target);

        if (std.mem.eql(u8, day, "day04")) {
            for (&[_]*std.build.LibExeObjStep{ exe, tests }) |step| {
                step.addIncludeDir("day04" ++ std.fs.path.sep_str ++ "include");
                step.addLibPath("day04" ++ std.fs.path.sep_str ++ "lib");
                step.linkSystemLibrary("c");
                step.linkSystemLibrary("pcre");
                if (builtin.os.tag == .windows) {
                    step.setTarget(.{
                        .cpu_arch = .i386,
                        .os_tag = .windows,
                        .abi = .gnu,
                    });
                }
            }
        }

        if (std.mem.eql(u8, day, "day13") or std.mem.eql(u8, day, "day17")) {
            for (&[_]*std.build.LibExeObjStep{ exe, tests }) |step| {
                step.addCSourceFile("AStar/AStar.c", &[_][]const u8{"-std=c99"});
                step.addIncludeDir("AStar");
                step.linkSystemLibrary("c");
            }
        }

        for (&[_]*std.build.LibExeObjStep{ exe, tests }) |step| {
            //step.addPackagePath("ziter", "ziter/ziter.zig");
            //step.addPackagePath("util", "util.zig");
            //step.addPackagePath("hashset", "hash_set.zig");
            step.setBuildMode(mode);
        }

        exe.install();

        tests.setNamePrefix(try std.fmt.allocPrint(b.allocator, "{s} ", .{day}));

        const build_step = b.step(
            try std.fmt.allocPrint(b.allocator, "build-{s}", .{day}),
            try std.fmt.allocPrint(b.allocator, "Build executable for {s}.", .{day}),
        );
        build_step.dependOn(&exe.step);
        build_all_step.dependOn(&exe.step);

        const test_log_step = b.addLog("\nTest results for {s}:\n", .{day});

        const test_step = b.step(
            try std.fmt.allocPrint(b.allocator, "test-{s}", .{day}),
            try std.fmt.allocPrint(b.allocator, "Run all tests for {s}.", .{day}),
        );
        test_step.dependOn(&test_log_step.step);
        test_step.dependOn(&tests.step);
        test_all_step.dependOn(test_step);

        const log_step = b.addLog("\nResults for {s}:\n", .{day});

        const run_cmd = exe.run();
        run_cmd.addArg(try std.fs.path.join(b.allocator, &[_][]const u8{ day, "input.txt" }));

        const run_step = b.step(
            try std.fmt.allocPrint(b.allocator, "run-{s}", .{day}),
            try std.fmt.allocPrint(b.allocator, "Run the executable for {s}.", .{day}),
        );
        run_step.dependOn(&log_step.step);
        run_step.dependOn(&run_cmd.step);
        run_all_step.dependOn(run_step);
    }

    const fmt_step = b.step("fmt", "Format source files.");
    fmt_step.dependOn(&fmt.step);

    const all_step = b.step("all", "Run all test and all days.");
    all_step.dependOn(fmt_step);
    all_step.dependOn(b.getInstallStep());
    all_step.dependOn(build_all_step);
    all_step.dependOn(test_all_step);
    all_step.dependOn(run_all_step);

    b.default_step.dependOn(fmt_step);
    b.default_step = run_all_step;
}

fn strLessThan(context: void, a: []const u8, b: []const u8) bool {
    var i: usize = 0;
    while (true) : (i += 1) {
        if (a.len <= i and b.len <= i) {
            return false;
        } else if (a.len <= i) {
            return true;
        } else if (b.len <= i) {
            return false;
        }

        if (a[i] < b[i]) {
            return true;
        } else if (a[i] > b[i]) {
            return false;
        }
    }
}
