# Building
## Pre-requisites
- A bash shell (git bash is sufficient for Windows)
- [`git`](https://git-scm.com) available in your shell
- PHP 8.2 or newer available in your shell
- [`composer`](https://getcomposer.org) available in your shell

## Custom PHP binaries
Because PocketMine-MP requires several non-standard PHP extensions and configuration, PMMP provides scripts to build custom binaries for running PocketMine-MP, as well as prebuilt binaries.

- [Prebuilt binaries](https://github.com/pmmp/PHP-Binaries/releases)
- [Compile scripts](https://github.com/pmmp/php-build-scripts) are provided as a submodule in the path `build/php`

If you use a custom binary, you'll need to replace `composer` usages in this guide with `path/to/your/php path/to/your/composer.phar`.

## Setting up environment
1. `git clone https://github.com/pmmp/PocketMine-MP.git`
2. `composer install`

## Checking out a different branch to build
1. `git checkout <branch to checkout>`
2. Re-run `composer install` to synchronize dependencies.

## Optimizing for release builds
1. Add the flags `--no-dev --classmap-authoritative` to your `composer install` command. This will reduce build size and improve autoloading speed.

## Building `PocketMine-MP.phar`
Run `composer make-server` using your preferred PHP binary. It'll drop a `PocketMine-MP.phar` into the current working directory.

You can also use the `--out` option to change the output filename.

## Running PocketMine-MP from source code
Run `src/PocketMine.php` using your preferred PHP binary.

## Building the optional `pmmpnative` extension
Some CPU-heavy parts of world generation have a native implementation written in Rust, in the `rust` directory. It is built as a PHP extension. PocketMine-MP works without it and generates exactly the same terrain either way; the extension only makes it faster.

Pre-requisites: a Rust toolchain (`cargo`), `clang`/`libclang`, and the headers and `php-config` of the PHP build the server runs on. The extension must be built for the same PHP minor version and thread-safety mode (the PMMP binaries are ZTS).

1. `cd rust`
2. `PHP=/path/to/php PHP_CONFIG=/path/to/php-config cargo build --release`
3. Add `extension=/path/to/rust/target/release/libpmmpnative.so` to your `php.ini`.

Run `cargo test` in the `rust` directory for the Rust unit tests. `tests/phpunit/world/generator/NativeTerrainTest.php` compares the native results against the PHP implementation, and is skipped unless the extension is loaded.

The PHP code remains the reference implementation. If you change anything the native code replaces, make the same change in `rust/src`: `Simplex`, `Noise::noise2D()`, `Noise::getFastNoise3D()`, `BiomeSelector::pickBiome()`, `Normal::generateChunk()` (including biome selection and elevation smoothing), the `GroundCover` populator, and the terrain shaping in `Nether::generateChunk()`.
