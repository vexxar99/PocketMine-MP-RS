<p align="center">
	<a href="https://axolotl-pm.org">
		<!--[if IE]>
			<img src="https://github.com/axolotl-pm/PocketMine-MP/blob/stable/.github/readme/pocketmine.png" alt="The PocketMine-MP logo" title="PocketMine" loading="eager" />
		<![endif]-->
		<img src="https://raw.githubusercontent.com/axolotl-pm/PocketMine-MP/stable/.github/readme/pocketmine-axolotl-wordmark.svg" alt="PocketMine-MP" loading="eager" />
	</a><br>
	<b>A highly customisable, open source server software for Minecraft: Bedrock Edition written in PHP</b>
</p>

<p align="center">
	<a href="https://github.com/axolotl-pm/PocketMine-MP/actions/workflows/main.yml"><img src="https://github.com/axolotl-pm/PocketMine-MP/actions/workflows/main.yml/badge.svg" alt="CI" /></a>
	<a href="https://github.com/axolotl-pm/PocketMine-MP/releases/latest"><img alt="GitHub release (latest SemVer)" src="https://img.shields.io/github/v/release/axolotl-pm/PocketMine-MP?label=release&sort=semver"></a>
	<a href="https://discord.axolotl-pm.org/"><img src="https://img.shields.io/discord/1526008653482692768?label=discord&color=7289DA&logo=discord" alt="Discord" /></a>
	<br>
	<a href="https://github.com/axolotl-pm/PocketMine-MP/releases"><img alt="GitHub all releases" src="https://img.shields.io/github/downloads/axolotl-pm/PocketMine-MP/total?label=downloads%40total"></a>
	<a href="https://github.com/axolotl-pm/PocketMine-MP/releases/latest"><img alt="GitHub release (latest by SemVer)" src="https://img.shields.io/github/downloads/axolotl-pm/PocketMine-MP/latest/total?sort=semver"></a>
</p>

## What is this?
PocketMine-MP is a highly customisable server software for Minecraft: Bedrock Edition, built in PHP.
A fork of the legacy [pmmp/PocketMine-MP](https://github.com/pmmp/PocketMine-MP) project, it is maintained by the
[Axolotl-PM](https://github.com/axolotl-pm) Team, which develops and publishes its releases.

Axolotl-PM is made up of contributors who have been part of the PocketMine-MP ecosystem for over 5 years,
contributing to the original repository. The project builds on the original's 10+ years of history.

If you're looking to create a Minecraft: Bedrock server with **custom functionality**, look no further.

- 🧩 **Powerful plugin API** - extend and customise gameplay as you see fit
- 🗺️ **Rich ecosystem** and **large developer community** - find plugins easily and learn to develop your own
- 🌐 **Multi-world support** - offer a more varied game experience to players without transferring them to other server nodes
- 🏎️ **Performance** - get 100+ players onto one server (depending on hardware and plugins)
- ⤴️ **Continuously updated** - new Minecraft versions are usually supported within days

## :x: PocketMine-MP is NOT a vanilla Minecraft server software.
**It is poorly suited to hosting vanilla survival servers.**
It doesn't have many features from the vanilla game, such as vanilla world generation, redstone, mob AI, and various other things.

If you just want to play **vanilla survival multiplayer**, consider using the [official Minecraft: Bedrock server software](https://minecraft.net/download/server/bedrock) instead of PocketMine-MP.

If that's not an option for you, you may be able to add some of PocketMine-MP's missing features using plugins from [Poggit](https://poggit.pmmp.io/plugins), or write plugins to implement them yourself.

## Native world generation (Rust)
This fork adds `pmmpnative`, an optional PHP extension written in Rust (see the [`rust`](rust) directory), which replaces the slowest parts of world generation with native code:

- Overworld chunk generation: biome selection, elevation smoothing, terrain shaping and ground cover
- Nether terrain shaping
- `Simplex::getFastNoise3D()`, which plugin generators can also benefit from

| Per chunk | PHP only | With `pmmpnative` |
|---|---|---|
| Overworld generation | 4.8 ms | 0.30 ms |
| Nether generation | 19.8 ms | 0.30 ms |

Measured on a prebuilt PHP 8.3 binary on Linux x86_64. Chunk population (ores, trees, tall grass) still runs in PHP and is not included.

The extension is optional. Without it the server runs the PHP code exactly as before, and the generated terrain is identical either way. Plugins which subclass `Simplex` or override the noise methods of `BiomeSelector` automatically use the PHP code.

The extension is not included in the prebuilt PHP binaries; see [BUILDING.md](BUILDING.md#building-the-optional-pmmpnative-extension) for how to build and load it. It has only been built and tested on Linux x86_64 with PHP 8.3 (ZTS) so far.

## Getting Started
- [Documentation](https://axolotl-pm.org/docs/)
- [Installation instructions](https://axolotl-pm.org/docs/installation)
- [Plugin repository](https://poggit.pmmp.io/plugins) (Poggit, the legacy PocketMine-MP plugin repository; we don't host one yet)

## Community & Support
Join our [Discord](https://discord.axolotl-pm.org/) server to chat with other users and developers.

You can also post questions on [StackOverflow](https://stackoverflow.com/tags/pocketmine) under the tag `pocketmine`.

## Developing Plugins
If you want to write your own plugins, the following resources may be useful.
Don't forget you can always ask our community if you need help.

 * [Developer documentation](https://axolotl-pm.org/docs/developers/plugin-docs-index) - General documentation for PocketMine-MP plugin developers
 * [Latest release API documentation](https://apidoc.pmmp.io) - Doxygen API documentation generated for each release of the original project
 * [DevTools](https://github.com/pmmp/DevTools/) - Development tools plugin for creating plugins
 * [ExamplePlugin](https://github.com/pmmp/ExamplePlugin/) - Example plugin demonstrating some basic API features

## Contributing to PocketMine-MP
PocketMine-MP accepts community contributions! The following resources will be useful if you want to contribute to PocketMine-MP.
 * [Building and running PocketMine-MP from source](BUILDING.md)
 * [Contributing Guidelines](CONTRIBUTING.md)

New here? Check out [issues with the "Easy task" label](https://github.com/axolotl-pm/PocketMine-MP/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22Easy%20task%22) for things you could work to familiarise yourself with the codebase.

## Licensing information
This project is licensed under LGPL-3.0. Please see the [LICENSE](/LICENSE) file for details.

Axolotl-PM is not affiliated with Mojang. All brands and trademarks belong to their respective owners. PocketMine-MP is not a Mojang-approved software, nor is it associated with Mojang.
