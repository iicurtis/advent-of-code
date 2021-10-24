project(
    'aoc2016', 'cpp',
    version: '0.1.0',
    license: 'GPLv3',
    default_options : [
        'cpp_std=c++17',
        'buildtype=release',
        'default_library=static'
    ],
)

cpp_args = ['-Wall', '-Wextra', '-Wno-deprecated-declarations', '-pedantic', '-Wno-gnu-zero-variadic-macro-arguments', '-ffast-math']
cpp_link_args = ['-lstdc++fs']

compiler = meson.get_compiler('cpp')
git = find_program('git', required: false)

if not git.found()
    add_project_arguments('-DVERSION="@0@"'.format(meson.project_version()), language: 'cpp')
else
    git_commit_hash = run_command([git.path(), 'describe', '--always', '--tags']).stdout().strip()
    git_branch = run_command([git.path(), 'rev-parse', '--abbrev-ref', 'HEAD']).stdout().strip()
    version = '"@0@ (" __DATE__ ", branch \'@1@\')"'.format(git_commit_hash, git_branch)
    add_project_arguments('-DVERSION=@0@'.format(version), language: 'cpp')
endif

if not compiler.has_header('filesystem')
    add_project_arguments('-DFILESYSTEM_EXPERIMENTAL', language: 'cpp')
endif

add_global_arguments(cpp_args, language : 'cpp')
add_global_link_arguments(cpp_link_args, language : 'cpp')

thread_dep = dependency('threads')
fmt = subproject('fmt').get_variable('fmt_dep')

src_files = files(
  'src/Advent.cpp',
  'src/day01.cpp',
  'src/day02.cpp',
  'src/day03.cpp'
)

executable(
    'aoc',
    src_files,
    dependencies: [thread_dep, fmt],
    include_directories: [include_directories('include')]
)

# Testing
# doctest = subproject('doctest').get_variable('dep')
# tests = executable('tests', 'test/tests.cpp', dependencies: [doctest])
# test('Tests', tests, args: ['--force-colors=true'])
