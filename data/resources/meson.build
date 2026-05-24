# Compile Blueprint resources from src/
src_dir = meson.project_source_root() / 'src'

blp_files = []
ui_files = []
ui_files_xml = ''

foreach file : fs.read(src_dir / 'ui-blueprint-resources.in').splitlines()
    file = file.strip()
    if file.startswith('#') or file == ''
        continue
    endif

    blp_files += src_dir / file
    ui_alias = file.replace('.blp', '.ui')
    ui_file = ui_alias.replace('/', '-')
    ui_files += ui_file
    ui_files_xml += '\n    <file compressed="true" preprocess="xml-stripblanks" alias="' + ui_alias + '">' + ui_file + '</file>'
endforeach

blueprints = custom_target(
    'blueprints',
    input: blp_files,
    output: ui_files,
    command: [
        meson.project_source_root() / 'build-aux' / 'compile-blueprints.sh',
        blueprint_compiler.full_path(),
        '@OUTDIR@',
        meson.project_source_root() / 'src',
        '@INPUT@',
    ],
)

scalable_dir = 'icons' / 'hicolor' / '512x512' / 'apps'
install_data(
  [
    scalable_dir / ('@0@.png').format(base_id),
    scalable_dir / ('@0@.png').format(application_id)
  ],
  install_dir: datadir /scalable_dir
)

symbolic_dir = 'icons' / 'hicolor' / 'symbolic' / 'apps'
install_data(
  symbolic_dir / ('@0@-symbolic.svg').format(base_id),
  install_dir: datadir / symbolic_dir
)

# Resources
resources = gnome.compile_resources(
    'resources',
    'resources.gresource.xml',
    gresource_bundle: true,
    source_dir: meson.current_build_dir(),
    install: true,
    install_dir: pkgdatadir,
    dependencies: [appdata_file, blueprints],
)