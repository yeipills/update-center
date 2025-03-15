project('gnome-update-center', 'rust',
  version: '0.1.0',
  license: 'GPL-3.0-or-later',
  meson_version: '>= 0.59.0',
)

i18n = import('i18n')
gnome = import('gnome')

base_id = 'org.gnome.UpdateCenter'
profile = get_option('profile')
if profile == 'development'
  app_id = '@0@.Devel'.format(base_id)
else
  app_id = base_id
endif

# Install data files
install_data(
  'data' / '@0@.desktop'.format(base_id),
  rename: '@0@.desktop'.format(app_id),
  install_dir: get_option('datadir') / 'applications',
)

install_data(
  'data' / '@0@.metainfo.xml'.format(base_id),
  rename: '@0@.metainfo.xml'.format(app_id),
  install_dir: get_option('datadir') / 'metainfo',
)

# Install icons
icon_sizes = ['scalable']
foreach size : icon_sizes
  install_data(
    'data' / 'icons' / size / '@0@.svg'.format(base_id),
    rename: '@0@.svg'.format(app_id),
    install_dir: get_option('datadir') / 'icons' / 'hicolor' / size / 'apps',
  )
endforeach

# Compile resources
resources = gnome.compile_resources(
  'resources',
  'data' / 'resources.gresource.xml',
  source_dir: 'data',
  c_name: 'resources',
)

# Configure Cargo build
cargo = find_program('cargo', required: true)

cargo_options = ['--manifest-path', meson.project_source_root() / 'Cargo.toml']
cargo_options += ['--target-dir', meson.project_build_root() / 'target']

if get_option('buildtype') == 'release'
  cargo_options += ['--release']
  rust_target = 'release'
else
  rust_target = 'debug'
endif

# Build the Rust application
cargo_build = custom_target(
  'cargo-build',
  output: 'gnome-update-center',
  command: [
    cargo, 'build',
    cargo_options,
  ],
  build_by_default: true,
)

# Install binary
install_data(
  meson.project_build_root() / 'target' / rust_target / 'gnome-update-center',
  install_dir: get_option('bindir'),
  install_mode: 'rwxr-xr-x',
)

# Generate and install systemd user service
configure_file(
  input: 'data' / 'systemd' / 'org.gnome.UpdateCenter.service.in',
  output: '@0@.service'.format(app_id),
  configuration: {
    'bindir': get_option('prefix') / get_option('bindir'),
    'app_id': app_id,
  },
  install_dir: get_option('prefix') / 'lib' / 'systemd' / 'user',
)