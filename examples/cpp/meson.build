project(
  'example-fracgm-cpp-project', 'cpp',
  version : '0.1',
  default_options : ['warning_level=3', 'cpp_std=c++17']
)

cpp = meson.get_compiler('cpp')

eigen3 = dependency('eigen3', fallback: ['eigen', 'eigen_dep'])

fracgm_include = include_directories('../../fracgm-cxx/include')
fracgm_library = cpp.find_library('libfracgm_cxx', dirs: join_paths(meson.source_root(), '../../target/release'))

exe = executable(
  'example-fracgm-cpp-project',
  sources: ['example_fracgm_cpp_project.cpp'],
  include_directories: [fracgm_include],
  dependencies: [eigen3, fracgm_library],
)

test('basic', exe)
