/*
MIT License

Copyright (c) 2025 Ritchie Mwewa

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use crate::cli::flags::ShowIcons;
use crate::display::theme::colours::{Colour, RgbColours};
use phf::{Map, phf_map};
use std::sync::atomic::{AtomicBool, Ordering};

// Global atomic: are icons enabled?
static ICONS_ENABLED: AtomicBool = AtomicBool::new(true);

pub struct IconSettings;

impl IconSettings {
    pub(crate) fn enable() {
        ICONS_ENABLED.store(true, Ordering::SeqCst);
    }

    pub(crate) fn disable() {
        ICONS_ENABLED.store(false, Ordering::SeqCst);
    }

    pub(crate) fn enabled() -> bool {
        ICONS_ENABLED.load(Ordering::SeqCst)
    }

    pub fn setup(show_icons: ShowIcons) {
        match show_icons {
            ShowIcons::Always => Self::enable(),
            ShowIcons::Never => Self::disable(),
            ShowIcons::Auto => {
                if unsafe { libc::isatty(libc::STDOUT_FILENO) == 1 } {
                    Self::enable()
                } else {
                    Self::disable()
                }
            }
        }
    }
}

#[non_exhaustive]
pub(crate) struct Icons;

#[rustfmt::skip]
#[allow(dead_code)]
impl Icons {
    const AUDIO: char           = '\u{f001}';  // 
    const ANACONDA: char        = '\u{e715}';  // 
    const ATOM: char            = '\u{ee99}';  // 
    const BINARY: char          = '\u{eae8}';  // 
    const BOOK: char            = '\u{e28b}';  // 
    const CALENDAR: char        = '\u{eab0}';  // 
    const CACHE: char           = '\u{f49b}';  // 
    const CAD: char             = '\u{f0eec}'; // 󰻬
    const CLOCK: char           = '\u{f43a}';  // 
    const SPELLCHECK: char      = '\u{f04c6}'; // 󰓆
    const CODE_OF_CONDUCT: char = '\u{f4ae}';  // 
    const COMPRESSED: char      = '\u{f05c4}'; // 󰗄
    const CONFIG: char          = '\u{f0493}'; // 󰒓
    const COW: char             = '\u{eef1}';  // 
    const CSS3: char            = '\u{e749}';  // 
    const DATABASE: char        = '\u{f1c0}';  // 
    const DESKTOP: char         = '\u{f108}';  // 
    const DIFF: char            = '\u{f440}';  // 
    const DISK_IMAGE: char      = '\u{e271}';  // 
    const DOCKER: char          = '\u{e650}';  // 
    const DOCUMENT: char        = '\u{f022c}'; // 󰈬
    const DOWNLOAD: char        = '\u{f01da}'; // 󰇚
    const EDA_SCH: char         = '\u{f0b45}'; // 󰭅
    const EDA_PCB: char         = '\u{eabe}';  // 
    const EDITORCONFIG: char    = '\u{e652}';  // 
    const EMACS: char           = '\u{e632}';  // 
    const ESLINT: char          = '\u{e655}';  // 
    const FILE: char            = '\u{f15b}';  // 
    const FILE_3D: char         = '\u{f01a7}'; // 󰆧
    const FILE_SYMLINK: char    = '\u{f1177}'; // 󱅷
    const FOLDER: char          = '\u{f07b}';  // 
    const FOLDER_CONTACTS: char = '\u{f024c}'; // 󰉌
    const FOLDER_BUILD: char    = '\u{f19fc}'; // 󱧼
    const FOLDER_CONFIG: char   = '\u{e5fc}';  // 
    const FOLDER_DOCUMENTS: char = '\u{f0c82}'; // 󰲂
    const FOLDER_DOWNLOADS: char = '\u{f024d}'; // 󰉍
    const FOLDER_EXERCISM: char = '\u{ebe5}';  // 
    const FOLDER_FAVORITES: char = '\u{f10ea}'; // 󱃪
    const FOLDER_GIT: char      = '\u{e5fb}';  // 
    const FOLDER_GITHUB: char   = '\u{e5fd}';  // 
    const FOLDER_HIDDEN: char   = '\u{f179e}'; // 󱞞
    const FOLDER_HOME: char     = '\u{f10b5}'; // 󱂵
    const FOLDER_IMAGE: char    = '\u{f024f}'; // 󰉏
    const FOLDER_KEY: char      = '\u{f08ac}'; // 󰢬
    const FOLDER_MUSIC: char    = '\u{f1359}'; // 󱍙
    const FOLDER_NPM: char      = '\u{e5fa}';  // 
    const FOLDER_OCAML: char    = '\u{e67a}';  // 
    const FOLDER_OPEN: char     = '\u{f114}';  // 
    const FOLDER_SRC: char      = '\u{f107d}'; // 󱁽
    const FILE_UNKNOWN: char    = '\u{f086f}'; // 󰡯
    const FONT: char            = '\u{f031}';  // 
    const FREECAD: char         = '\u{f336}';  // 
    const GIMP: char            = '\u{f338}';  // 
    const GIST_SECRET: char     = '\u{f0221}'; // 󰈡
    const GIT: char             = '\u{f02a2}'; // 󰊢
    const GITLAB: char          = '\u{f296}';  // 
    const GLASS_MUG: char       = '\u{f02a6}'; // 󰊦
    const GODOT: char           = '\u{e65f}';  // 
    const GOOGLE_CLOUD: char    = '\u{f11f6}'; // 󱇶
    const GRADLE: char          = '\u{e660}';  // 
    const GRAPH: char           = '\u{f1049}'; // 󱁉
    const GRAPHQL: char         = '\u{e662}';  // 
    const GRUNT: char           = '\u{e611}';  // 
    const GTK: char             = '\u{f362}';  // 
    const GULP: char            = '\u{e610}';  // 
    const HOOK: char            = '\u{f06e2}'; // 󰛢
    const HTML5: char           = '\u{f13b}';  // 
    const HYPRLAND: char        = '\u{f359}';  // 
    const IMAGE: char           = '\u{f02e9}'; // 󰋩
    const INFO: char            = '\u{f129}';  // 
    const INTELLIJ: char        = '\u{e7b5}';  // 
    const JSON: char            = '\u{e60b}';  // 
    const JSONL: char           = '\u{f0626}'; // 󰘦
    const KEY: char             = '\u{eb11}';  // 
    const KDENLIVE: char        = '\u{f33c}';  // 
    const KEYPASS: char         = '\u{f23e}';  // 
    const KICAD: char           = '\u{f34c}';  // 
    const KRITA: char           = '\u{f33d}';  // 
    const LANG_ARDUINO: char    = '\u{f34b}';  // 
    const LANG_ASSEMBLY: char   = '\u{e637}';  // 
    const LANG_C: char          = '\u{e61e}';  // 
    const LANG_CPP: char        = '\u{e61d}';  // 
    const LANG_CSHARP: char     = '\u{f031b}'; // 󰌛
    const LANG_D: char          = '\u{e7af}';  // 
    const LANG_ELIXIR: char     = '\u{e62d}';  // 
    const LANG_FENNEL: char     = '\u{e6af}';  // 
    const LANG_FORTRAN: char    = '\u{f121a}'; // 󱈚
    const LANG_FSHARP: char     = '\u{e7a7}';  // 
    const LANG_GLEAM: char      = '\u{f09a5}'; // 󰦥
    const LANG_GO: char         = '\u{e627}';  // 
    const LANG_GROOVY: char     = '\u{e775}';  // 
    const LANG_HASKELL: char    = '\u{e777}';  // 
    const LANG_HDL: char        = '\u{f035b}'; // 󰍛
    const LANG_HOLYC: char      = '\u{f00a2}'; // 󰂢
    const LANG_JAVA: char       = '\u{e256}';  // 
    const LANG_JAVASCRIPT: char = '\u{e74e}';  // 
    const LANG_KOTLIN: char     = '\u{e634}';  // 
    const LANG_LUA: char        = '\u{e620}';  // 
    const LANG_NIM: char        = '\u{e677}';  // 
    const LANG_OCAML: char      = '\u{e67a}';  // 
    const LANG_PERL: char       = '\u{e67e}';  // 
    const LANG_PHP: char        = '\u{e608}';  // 
    const LANG_PYTHON: char     = '\u{e606}';  // 
    const LANG_R: char          = '\u{e68a}';  // 
    const LANG_RUBY: char       = '\u{e739}';  // 
    const LANG_RUBYRAILS: char  = '\u{e73b}';  // 
    const LANG_RUST: char       = '\u{e68b}';  // 
    const LANG_SASS: char       = '\u{f07ec}'; // 󰟬
    const LANG_SCHEME: char     = '\u{e6b1}';  // 
    const LANG_STYLUS: char     = '\u{e600}';  // 
    const LANG_TEX: char        = '\u{e69b}';  // 
    const LANG_TYPESCRIPT: char = '\u{e628}';  // 
    const LANG_V: char          = '\u{e6ac}';  // 
    const LIBRARY: char         = '\u{eb9c}';  // 
    const LICENSE: char         = '\u{f02d}';  // 
    const LOCK: char            = '\u{f023}';  // 
    const LOG: char             = '\u{f4ed}';  // 
    const MAIL: char            = '\u{f0e0}';  // 
    const MAKE: char            = '\u{e673}';  // 
    const MARKDOWN: char        = '\u{f48a}';  // 
    const MUSTACHE: char        = '\u{e60f}';  // 
    const MOVIE: char           = '\u{f0fce}'; // 󰿎
    const NANO: char            = '\u{e838}';  // 
    const NEWS: char            = '\u{f0395}'; // 󰎕
    const NODEJS: char          = '\u{e718}';  // 
    const NOTEBOOK: char        = '\u{e678}';  // 
    const NPM: char             = '\u{e71e}';  // 
    const NUXT: char            = '\u{f1106}'; // 󱄆
    const OS_ANDROID: char      = '\u{e70e}';  // 
    const OS_APPLE: char        = '\u{f179}';  // 
    const OS_LINUX: char        = '\u{f17c}';  // 
    const OS_WINDOWS: char      = '\u{f17a}';  // 
    const OS_WINDOWS_CMD: char  = '\u{ebc4}';  // 
    const PLAYLIST: char        = '\u{f0cb9}'; // 󰲹
    const POWERSHELL: char      = '\u{f0a0a}'; // 󰨊
    const PRIVATE_KEY: char     = '\u{f0dd6}'; // 󰷖
    const PUBLIC_KEY: char      = '\u{f0306}'; // 󰌆
    const PYTEST: char          = '\u{e87a}';  // 
    const QT: char              = '\u{f375}';  // 
    const RAZOR: char           = '\u{f1fa}';  // 
    const REACT: char           = '\u{e7ba}';  // 
    const README: char          = '\u{f00ba}'; // 󰂺
    const ROBOT: char           = '\u{f06a9}'; // 󰚩
    const SHEET: char           = '\u{f021b}'; // 󰈛
    const SHELL: char           = '\u{f1183}'; // 󱆃
    const SHELL_CMD: char       = '\u{e795}';  // 
    const SHELL_FILE: char      = '\u{ebca}';  // 
    const SHIELD_CHECK: char    = '\u{f0565}'; // 󰕥
    const SHIELD_KEY: char      = '\u{f0bc4}'; // 󰯄
    const SHIELD_LOCK: char     = '\u{f099d}'; // 󰦝
    const SIGNED_FILE: char     = '\u{ee3c}';  // 
    const SLIDE: char           = '\u{f0227}'; // 󰈧
    const SLIDERS: char         = '\u{f462}';  // 
    const SQLITE: char          = '\u{e7c4}';  // 
    const SUBLIME: char         = '\u{e7aa}';  // 
    const SUBTITLE: char        = '\u{f0a16}'; // 󰨖
    const SSH: char             = '\u{f08c0}'; // 󰣀
    const TCL: char             = '\u{f06d3}'; // 󰛓
    const TERRAFORM: char       = '\u{f1062}'; // 󱁢
    const TEST_TUBE: char       = '\u{f0668}'; // 󰙨
    const TEXT: char            = '\u{e64e}';  // 
    const TODO: char            = '\u{f0ae}';  // 
    const TRASH: char           = '\u{f1f8}';  // 
    const TYPST: char           = '\u{f37f}';  // 
    const TMUX: char            = '\u{ebc8}';  // 
    const TOML: char            = '\u{e6b2}';  // 
    const TRANSLATION: char     = '\u{f05ca}'; // 󰗊
    const USER_GROUP: char      = '\u{edca}';  // 
    const UNITY: char           = '\u{e721}';  // 
    const VECTOR: char          = '\u{f0559}'; // 󰕙
    const VIDEO: char           = '\u{f03d}';  // 
    const VIM: char             = '\u{e7c5}';  // 
    const WRENCH: char          = '\u{f0ad}';  // 
    const XML: char             = '\u{f05c0}'; // 󰗀
    const XORG:char             = '\u{f369}';  // 
    const YAML: char            = '\u{e6a8}';  // 
    const YARN: char            = '\u{e6a7}';  // 
}

/// PHF map for directory icon lookups (all keys must be lowercase for case-insensitive matching)
const DIRECTORY_ICONS: Map<&'static str, char> = phf_map! {
    ".cache"              => Icons::CACHE,
    ".cargo" | ".rustup"  => Icons::LANG_RUST,
    ".config" | "config" | "cron.d" | "cron.daily" | "cron.hourly" | "cron.minutely" | "cron.monthly" | "cron.weekly" | "etc" | "include" | "pacman.d" | "xbps.d" | "xorg.conf.d" => Icons::FOLDER_CONFIG,
    ".claude" | ".cursor" | ".codex" | ".aider" | ".autogpt" | ".devin" | ".copilot" | ".openai" | ".junie" => Icons::ROBOT,
    ".exercism"           => Icons::FOLDER_EXERCISM,
    ".git"                => Icons::FOLDER_GIT,
    ".github"             => Icons::FOLDER_GITHUB,
    ".npm" | "node_modules" | "npm_cache" => Icons::FOLDER_NPM,
    ".opam"               => Icons::FOLDER_OCAML,
    ".ssh" | "pam.d" | "ssh" | "sudoers.d" => Icons::FOLDER_KEY,
    ".trash"              => Icons::TRASH,
    "tests"               => Icons::TEST_TUBE,
    "build"               => Icons::FOLDER_BUILD,
    "cabal"               => Icons::LANG_HASKELL,
    "contacts"            => Icons::FOLDER_CONTACTS,
    "desktop"             => Icons::DESKTOP,
    "documents"           => Icons::FOLDER_DOCUMENTS,
    "downloads"           => Icons::FOLDER_DOWNLOADS,
    "favorites" | "favourites" => Icons::FOLDER_FAVORITES,
    "hidden"              => Icons::FOLDER_HIDDEN,
    "home"                => Icons::FOLDER_HOME,
    "mail"                => Icons::MAIL,
    "movies"              => Icons::MOVIE,
    "music"               => Icons::FOLDER_MUSIC,
    "pictures" | "img"    => Icons::FOLDER_IMAGE,
    "src"                 => Icons::FOLDER_SRC,
    "videos"              => Icons::VIDEO,
};

/// PHF map for directory colour lookups (non-themed directories only)
/// All directories use the default entry_directory theme colour
pub(crate) static DIRECTORY_COLOURS: Map<&'static str, Colour> = phf_map! {
    // All directories use themed default colour
};

/// PHF map for filename icon lookups (all keys must be lowercase for case-insensitive matching)
const FILENAME_ICONS: Map<&'static str, char> = phf_map! {
    // Shell configs
    ".aliases" | ".bashrc" | ".bash_aliases" | ".bash_history" | ".bash_logout" | ".bash_profile" | ".cshrc" | ".kshrc" | ".login" | ".logout" | ".profile" | ".tcshrc" | ".zlogin" | ".zlogout" | ".zprofile" | ".zshenv" | ".zshrc" | ".zsh_history" | ".zsh_sessions" | "bashrc" | "csh.cshrc" | "csh.login" | "csh.logout" | "profile" | "zlogin" | "zlogout" | "zprofile" | "zshenv" | "zshrc" => Icons::SHELL,
    // Git
    ".gitattributes" | ".git-blame-ignore-revs" | ".gitconfig" | ".gitignore" | ".gitignore_global" | ".gitmodules" | ".mailmap" | "commit_editmsg" => Icons::GIT,
    // ESLint
    ".eslintignore" | ".eslintrc.cjs" | ".eslintrc.js" | ".eslintrc.json" | ".eslintrc.yaml" | ".eslintrc.yml" => Icons::ESLINT,
    // Vim
    ".gvimrc" | ".ideavimrc" | ".viminfo" | ".vimrc" | "_gvimrc" | "_vimrc" => Icons::VIM,
    // License
    "copying" | "copyright" | "licence" | "licence.md" | "licence.txt" | "license" | "license-apache" | "license-mit" | "license.md" | "license.txt" => Icons::LICENSE,
    // NPM
    ".npmignore" | ".npmrc" | "npm-shrinkwrap.json" | "npmrc" | "package-lock.json" | "package.json" => Icons::NPM,
    // Lock files
    ".parentlock" | "group" | "gshadow" | "lock" | "passwd" | "shadow" | "sudoers" => Icons::LOCK,
    // Make
    "gnumakefile" | "makefile" | "makefile.ac" | "makefile.am" | "makefile.in" => Icons::MAKE,
    // News/Changelog
    "changelog" | "changelog.md" | "changes" | "changes.md" | "news" | "news.md" => Icons::NEWS,
    // Config
    ".clang-format" | ".clang-tidy" | ".htaccess" | ".htpasswd" | ".inputrc" | ".luacheckrc" | ".luaurc" | ".pylintrc" | "config" | "config.status" | "configure.ac" | "configure.in" | "crontab" | "crypttab" | "environment" | "hostname" | "inputrc" | "shells" | "sxhkdrc" => Icons::CONFIG,
    // Docker
    "docker-compose.yml" | "dockerfile" | "compose.yaml" | "compose.yml" | "docker-compose.yaml" => Icons::DOCKER,
    // Xorg
    ".xauthority" | ".xinitrc" | ".xresources" | ".xsession" | "xorg.conf" | "xsettingsd.conf" | "xmobarrc" | "xmobarrc.hs" | "xmonad.hs" => Icons::XORG,
    // Apple
    ".cfusertextencoding" | ".ds_store" | "._ds_store" | "localized" => Icons::OS_APPLE,
    // Ruby
    ".rvm" | ".rvmrc" | "config.ru" | "gemfile" | "gemfile.lock" | "rakefile" | "rvmrc" => Icons::LANG_RUBY,
    // Rust
    ".rustfmt.toml" | "cargo.lock" | "cargo.toml" | "release.toml" => Icons::LANG_RUST,
    // Go
    "go.mod" | "go.sum" | "go.work" => Icons::LANG_GO,
    // Gradle
    "build.gradle.kts" | "gradle" | "gradle.properties" | "gradlew" | "gradlew.bat" | "settings.gradle.kts" => Icons::GRADLE,
    // Python
    ".python_history" | "constraints.txt" | "manifest" | "manifest.in" | "pyvenv.cfg" | "pyproject.toml" | "requirements.txt" => Icons::LANG_PYTHON,
    // PHP
    "composer.json" | "composer.lock" | "php.ini" => Icons::LANG_PHP,
    // SSH Private Keys
    "id_dsa" | "id_ecdsa" | "id_ecdsa_sk" | "id_ed25519" | "id_ed25519_sk" | "id_rsa" => Icons::PRIVATE_KEY,
    // Task runners
    "gruntfile.coffee" | "gruntfile.js" | "gruntfile.ls" => Icons::GRUNT,
    "gulpfile.coffee" | "gulpfile.js" | "gulpfile.ls" => Icons::GULP,
    // KiCad
    "fp-info-cache" | "fp-lib-table" | "sym-lib-table" => Icons::KICAD,
    // KDE apps
    "kdenlive-layoutsrc" | "kdenliverc" => Icons::KDENLIVE,
    "kritadisplayrc" | "kritarc" => Icons::KRITA,
    // Misc grouped
    "log" => Icons::LOG,
    "localtime" | "timezone" => Icons::CLOCK,
    "configure" | "dune-project" | "justfile" => Icons::WRENCH,
    "todo" | "todo.md" => Icons::TODO,
    "readme" | "readme.md" => Icons::README,
    "tmux.conf" | "tmux.conf.local" => Icons::TMUX,
    "qt5ct.conf" | "qt6ct.conf" | "qtproject.conf" => Icons::QT,
    ".ocamlinit" | "dune" => Icons::LANG_OCAML,
    ".fennelrc" | "fennelrc" => Icons::LANG_FENNEL,
    "authorized_keys" | "known_hosts" => Icons::SSH,
    "authors" | "authors.txt" => Icons::USER_GROUP,
    "brewfile" | "brewfile.lock.json" => Icons::GLASS_MUG,
    "code_of_conduct" | "code_of_conduct.md" => Icons::CODE_OF_CONDUCT,
    "hypridle.conf" | "hyprland.conf" | "hyprlock.conf" | "hyprpaper.conf" => Icons::HYPRLAND,
    "i3blocks.conf" | "i3status.conf" => '\u{f35a}',
    "cantorrc" | "kalgebrarc" | "kdeglobals" => '\u{f373}',
    "security" | "security.md" => '\u{f0483}',
    "prusaslicer.ini" | "prusaslicergcodeviewer.ini" => '\u{f351}',
    "heroku.yml" | "procfile" => '\u{e77b}',
    ".srcinfo" | "pkgbuild" => '\u{f303}',
    ".prettierignore" | ".prettierrc" | ".prettierrc.json" | ".prettierrc.json5" | ".prettierrc.toml" | ".prettierrc.yaml" | ".prettierrc.yml" => '\u{e6b4}',
    // Unique entries
    ".atom"               => Icons::ATOM,
    ".codespellrc"        => Icons::SPELLCHECK,
    ".condarc"            => Icons::ANACONDA,
    ".editorconfig"       => Icons::EDITORCONFIG,
    ".emacs"              => Icons::EMACS,
    ".envrc"              => Icons::SLIDERS,
    ".gcloudignore"       => Icons::GOOGLE_CLOUD,
    ".gitlab-ci.yml"      => Icons::GITLAB,
    ".gtkrc-2.0"          => Icons::GTK,
    ".idea"               => Icons::INTELLIJ,
    ".nanorc"             => Icons::NANO,
    ".nuxtrc"             => Icons::NUXT,
    ".node_repl_history"  => Icons::NODEJS,
    ".pre-commit-config.yaml" => Icons::HOOK,
    ".stowrc"             => Icons::COW,
    ".yarnrc"             => Icons::YARN,
    "a.out"               => Icons::SHELL_CMD,
    "bspwmrc"             => '\u{f355}',
    "build.zig.zon"       => '\u{e6a9}',
    "bun.lockb"           => '\u{e76f}',
    "cmakelists.txt"      => '\u{e794}',
    "dropbox"             => '\u{e707}',
    "earthfile"           => '\u{f0ac}',
    "favicon.ico"         => '\u{e623}',
    "flake.lock"          => '\u{f313}',
    "fonts.conf"          => Icons::FONT,
    "freecad.conf"        => Icons::FREECAD,
    "gtkrc"               => Icons::GTK,
    "index.theme"         => '\u{ee72}',
    "jenkinsfile"         => '\u{e66e}',
    "jsconfig.json"       => Icons::LANG_JAVASCRIPT,
    "lxde-rc.xml"         => '\u{f363}',
    "lxqt.conf"           => '\u{f364}',
    "mix.lock"            => Icons::LANG_ELIXIR,
    "mpv.conf"            => '\u{f36e}',
    "platformio.ini"      => '\u{e682}',
    "pytest.ini"          => Icons::PYTEST, // 
    "pom.xml"             => '\u{e674}',
    "renovate.json"       => '\u{f027c}',
    "robots.txt"          => '\u{f06a9}',
    "rubydoc"             => Icons::LANG_RUBYRAILS,
    "tsconfig.json"       => Icons::LANG_TYPESCRIPT,
    "vagrantfile"         => '\u{2371}',
    "vlcrc"               => '\u{f057c}',
    "webpack.config.js"   => '\u{f072b}',
    "weston.ini"          => '\u{f367}',
    "yarn.lock"           => Icons::YARN,
};

/// PHF map for filename colour lookups (non-themed files only)
/// Themed filenames will use extension-based theming
pub(crate) static FILENAME_COLOURS: Map<&'static str, Colour> = phf_map! {
    "Makefile"       => Colour::DarkGray,
    "LICENSE"        => Colour::White,
};

/// PHF map for extension icon lookups
const EXTENSION_ICONS: Map<&'static str, char> = phf_map! {
    // Video
    "3g2" | "3gp" | "3gp2" | "3gpp" | "3gpp2" | "avi" | "cast" | "flv" | "h264" | "heics" | "m2ts" | "m2v" | "m4v" | "mkv" | "mov" | "mp4" | "mpeg" | "mpg" | "ogm" | "ogv" | "video" | "vob" | "webm" | "wmv" => Icons::VIDEO,
    // Audio
    "aac" | "aif" | "aifc" | "aiff" | "alac" | "ape" | "flac" | "m4a" | "mka" | "mp2" | "mp3" | "ogg" | "opus" | "pcm" | "swf" | "wav" | "wma" | "wv" => Icons::AUDIO,
    // Image
    "arw" | "avif" | "bmp" | "cbr" | "cbz" | "cr2" | "dvi" | "gif" | "heic" | "heif" | "ico" | "j2c" | "j2k" | "jfi" | "jfif" | "jif" | "jp2" | "jpe" | "jpeg" | "jpf" | "jpg" | "jpx" | "jxl" | "nef" | "orf" | "pbm" | "pgm" | "png" | "pnm" | "ppm" | "pxm" | "raw" | "tif" | "tiff" | "webp" | "xpm" => Icons::IMAGE,
    // Compressed
    "7z" | "ar" | "arj" | "br" | "bz" | "bz2" | "bz3" | "cpio" | "gz" | "lz" | "lz4" | "lzh" | "lzma" | "lzo" | "par" | "rar" | "tar" | "taz" | "tbz" | "tbz2" | "tgz" | "tlz" | "txz" | "tz" | "tzo" | "xz" | "z" | "zip" | "zst" => Icons::COMPRESSED,
    // Fonts
    "bdf" | "eot" | "flc" | "flf" | "fnt" | "fon" | "font" | "lff" | "otf" | "psf" | "ttc" | "ttf" | "woff" | "woff2" => Icons::FONT,
    // C++
    "c++" | "cc" | "cp" | "cpp" | "cxx" | "h++" | "hh" | "hpp" | "hxx" | "mm" => Icons::LANG_CPP,
    // Python
    "pxd" | "py" | "pyc" | "pyd" | "pyi" | "pyo" | "pyw" | "pyx" | "whl" => Icons::LANG_PYTHON,
    // Java
    "class" | "jad" | "jar" | "java" | "war" => Icons::LANG_JAVA,
    // Documents
    "djv" | "djvu" | "doc" | "docm" | "docx" | "gdoc" => Icons::DOCUMENT,
    // Database
    "db" | "dconf" | "dump" | "ldb" | "mdb" | "odb" | "prql" | "sql" => Icons::DATABASE,
    // SQLite
    "db3" | "s3db" | "sl3" | "sqlite" | "sqlite3" => Icons::SQLITE,
    // Ruby
    "gem" | "gemfile" | "gemspec" | "guardfile" | "procfile" | "rake" | "rakefile" | "rb" | "rspec" | "rspec_parallel" | "rspec_status" | "ru" => Icons::LANG_RUBY,
    // TeX/LaTeX
    "bib" | "bst" | "cls" | "latex" | "ltx" | "sty" | "tex" => Icons::LANG_TEX,
    // Fortran
    "f" | "f90" | "for" => Icons::LANG_FORTRAN,
    // F#
    "f#" | "fs" | "fsi" | "fsproj" | "fsscript" | "fsx" => Icons::LANG_FSHARP,
    // Elixir
    "eex" | "ex" | "exs" | "leex" => Icons::LANG_ELIXIR,
    // Config
    "cfg" | "conf" | "config" | "ini" | "tml" => Icons::CONFIG,
    // Shell commands
    "awk" | "bash" | "bats" | "csh" | "fish" | "ksh" | "nu" | "sh" | "shell" | "zsh" => Icons::SHELL_FILE,
    // Downloads
    "crdownload" | "fdmdownload" | "part" => Icons::DOWNLOAD,
    // Playlists
    "cue" | "m3u" | "m3u8" | "pls" => Icons::PLAYLIST,
    // Markdown
    "jmd" | "markdown" | "md" | "mdx" | "mkd" | "rdoc" | "rmd" => Icons::MARKDOWN,
    // 3D Files
    "3mf" | "fbx" | "obj" | "ply" | "stl" | "wrl" | "wrz" => Icons::FILE_3D,
    // CAD
    "123dx" | "3dm" | "brep" | "catpart" | "catproduct" | "dwg" | "dxf" | "f3d" | "f3z" | "iam" | "ifc" | "ige" | "iges" | "igs" | "ipt" | "psm" | "skp" | "sldasm" | "sldprt" | "slvs" | "ste" | "step" | "stp" | "x_b" | "x_t" => Icons::CAD,
    // EDA PCB
    "brd" | "gbl" | "gbo" | "gbp" | "gbr" | "gbs" | "gm1" | "gml" | "gtl" | "gto" | "gtp" | "gts" | "lpp" | "pcbdoc" | "prjpcb" => Icons::EDA_PCB,
    // Haskell
    "hs" | "lhs" => Icons::LANG_HASKELL,
    // Groovy
    "groovy" | "gvy" => Icons::LANG_GROOVY,
    // OCaml
    "ml" | "mli" | "mll" | "mly" => Icons::LANG_OCAML,
    // Scheme
    "rkt" | "scm" | "sld" | "ss" => Icons::LANG_SCHEME,
    // Nim
    "nim" | "nimble" | "nims" => Icons::LANG_NIM,
    // Lua
    "lua" | "luac" | "luau" => Icons::LANG_LUA,
    // Kotlin
    "kt" | "kts" => Icons::LANG_KOTLIN,
    // C#
    "cs" | "csproj" | "csx" => Icons::LANG_CSHARP,
    // D
    "d" | "di" => Icons::LANG_D,
    // HDL
    "sv" | "svh" | "vhdl" => Icons::LANG_HDL,
    // Slides
    "gslides" | "pps" | "ppsx" | "ppt" | "pptx" => Icons::SLIDE,
    // Subtitles
    "ass" | "lrc" | "sbt" | "srt" | "ssa" | "sub" => Icons::SUBTITLE,
    // Checksums
    "md5" | "sha1" | "sha224" | "sha256" | "sha384" | "sha512" => Icons::SHIELD_CHECK,
    // Encrypted
    "age" | "asc" | "gpg" => Icons::SHIELD_LOCK,
    // Translation
    "mo" | "po" | "pot" | "qm" => Icons::TRANSLATION,
    // FreeCAD
    "fcbak" | "fcmacro" | "fcmat" | "fcparam" | "fcscript" | "fcstd" | "fcstd1" | "fctb" | "fctl" => Icons::FREECAD,
    // KiCad
    "kicad_dru" | "kicad_mod" | "kicad_pcb" | "kicad_prl" | "kicad_pro" | "kicad_sch" | "kicad_sym" | "kicad_wks" => Icons::KICAD,
    // Godot
    "gd" | "godot" | "tres" | "tscn" => Icons::GODOT,
    // Terraform
    "tf" | "tfstate" | "tfvars" => Icons::TERRAFORM,
    // Kdenlive
    "kdenlive" | "kdenlivetitle" => Icons::KDENLIVE,
    // Krita
    "kpp" | "kra" | "krz" => Icons::KRITA,
    // Unity
    "unity" | "unity3d" => Icons::UNITY,
    // Qt
    "qml" | "qrc" | "qss" => Icons::QT,
    // Binary
    "app" | "bin" | "elf" | "hi" | "o" => Icons::BINARY,
    // JSON
    "avro" | "json" | "json5" | "jsonc" | "properties" | "webmanifest" => Icons::JSON,
    "jsonl" => Icons::JSONL,
    // HTML
    "htm" | "html" | "shtml" | "xhtml" => Icons::HTML5,
    // Graph
    "dot" | "gv" => Icons::GRAPH,
    // Calendar
    "ical" | "icalendar" | "ics" | "ifb" => Icons::CALENDAR,
    // Keys
    "key" | "p12" | "pem" | "pfx" => Icons::KEY,
    // Books
    "ebook" | "epub" | "mobi" => Icons::BOOK,
    // Diff
    "diff" | "patch" => Icons::DIFF,
    // Text
    "rst" | "rtf" | "txt" => Icons::TEXT,
    // Library
    "dll" | "lbr" | "lib" => Icons::LIBRARY,
    // XML
    "opml" | "xml" | "xul" => Icons::XML,
    // Perl
    "pl" | "plx" | "pm" | "pod" | "t" => Icons::LANG_PERL,
    // Apple
    "apple" | "applescript" | "bundle" | "dylib" | "localized" | "plist" => Icons::OS_APPLE,
    // Windows
    "cab" | "cmd" | "msi" | "windows" => Icons::OS_WINDOWS,
    // Linux
    "a" | "ko" | "so" => Icons::OS_LINUX,
    // Rust
    "rlib" | "rmeta" | "rs" => Icons::LANG_RUST,
    // JavaScript
    "cjs" | "js" | "mjs" => Icons::LANG_JAVASCRIPT,
    // TypeScript
    "cts" | "mts" | "ts" => Icons::LANG_TYPESCRIPT,
    // Emacs
    "el" | "elc" => Icons::EMACS,
    // Stylus
    "styl" | "stylus" => Icons::LANG_STYLUS,
    // Lock
    "lck" | "lock" => Icons::LOCK,
    // React
    "jsx" | "tsx" => Icons::REACT,
    // Vector
    "eps" | "ps" | "svg" => Icons::VECTOR,
    // C
    "c" | "h" | "inl" | "m" => Icons::LANG_C,
    // Sublime
    "sublime-build" | "sublime-keymap" | "sublime-menu" | "sublime-options" | "sublime-package" | "sublime-project" | "sublime-session" | "sublime-settings" | "sublime-snippet" | "sublime-theme" => Icons::SUBLIME,
    // YAML
    "yaml" | "yml" => Icons::YAML,
    // Sass
    "sass" | "scss" => Icons::LANG_SASS,
    // PowerShell
    "ps1" | "psd1" | "psm1" => Icons::POWERSHELL,
    // Docker
    "dockerfile" | "dockerignore" => Icons::DOCKER,
    // GraphQL
    "gql" | "graphql" => Icons::GRAPHQL,
    // EDA Schematic
    "sch" | "schdoc" => Icons::EDA_SCH,
    // Tcl
    "tbc" | "tcl" => Icons::TCL,
    // Info
    "info" | "nfo" => Icons::INFO,
    // Certificates
    "cert" | "crt" => Icons::GIST_SECRET,
    // Android
    "android" | "apk" => Icons::OS_ANDROID,
    // Mustache
    "hbs" | "mustache" => Icons::MUSTACHE,
    // KeePass
    "kdb" | "kdbx" => Icons::KEYPASS,
    // Razor
    "cshtml" | "razor" => Icons::RAZOR,
    // Windows executables
    "bat" | "exe" => Icons::OS_WINDOWS_CMD,
    // Rails
    "erb" | "rubydoc" | "slim" => Icons::LANG_RUBYRAILS,
    // R
    "r" | "rdata" | "rds" => Icons::LANG_R,
    // Signed files
    "sig" | "signature" => Icons::SIGNED_FILE,
    // Disk images (already consolidated)
    "dmg" | "image" | "img" | "iso" | "qcow" | "qcow2" | "tc" | "vdi" | "vhd" | "vmdk" => Icons::DISK_IMAGE,
    // Spreadsheets (already consolidated)
    "csv" | "gsheet" | "tsv" | "xlr" | "xls" | "xlsx" => Icons::SHEET,
    // Clojure
    "clj" | "cljc" => '\u{e768}',
    // Erlang
    "erl" | "hrl" => '\u{e7b1}',
    // Photoshop
    "psb" | "psd" => '\u{e7b8}',
    // Sound Font
    "sf2" | "sfz" => '\u{f0f70}',
    // Saleae Logic
    "sal" | "sr" => '\u{f147b}',
    // ROM files
    "gba" | "z64" => '\u{f1393}',
    // Nintendo Switch
    "nsp" | "xci" => '\u{F07E1}',
    // Swift
    "swift" | "xcplayground" => '\u{e755}',
    // Visual Studio
    "sln" | "suo" => '\u{e70c}',
    // LibreOffice Draw
    "fodg" | "odg" => '\u{f379}',
    // LibreOffice Impress
    "fodp" | "odp" => '\u{f37a}',
    // LibreOffice Calc
    "fods" | "ods" => '\u{f378}',
    // LibreOffice Writer
    "fodt" | "odt" => '\u{f37c}',
    // Unique entries
    "acf"            => '\u{f1b6}',
    "ai"             => '\u{e7b4}',
    "asm" | "s"      => Icons::LANG_ASSEMBLY,
    "asp"            => '\u{f121}',
    "blend"          => '\u{f00ab}',
    "cache"          => Icons::CACHE,
    "cljs"           => '\u{e76a}',
    "cmake"          => '\u{e794}',
    "coffee"         => '\u{f0f4}',
    "com"            => '\u{e629}',
    "conda"          => '\u{e715}',
    "cow"            => '\u{f019a}',
    "cr"             => '\u{e62f}',
    "css"            => Icons::CSS3,
    "cu"             => '\u{e64b}',
    "dart"           => '\u{e798}',
    "deb"            => '\u{e77d}',
    "desktop"        => '\u{ebd1}',
    "download"       => Icons::DOWNLOAD,
    "drawio"         => '\u{ebba}',
    "ebuild"         => '\u{f30d}',
    "edn"            => '\u{e76a}',
    "editorconfig"   => '\u{e652}',
    "ejs"            => '\u{e618}',
    "elm"            => '\u{e62c}',
    "eml"            => '\u{f003}',
    "env"            => '\u{f462}',
    "fnl"            => Icons::LANG_FENNEL,
    "gcode"          => '\u{f0af4}',
    "gform"          => '\u{f298}',
    "git"            => Icons::GIT,
    "gleam"          => Icons::LANG_GLEAM,
    "go"             => Icons::LANG_GO,
    "gradle"         => Icons::GRADLE,
    "gresource"      => Icons::GTK,
    "haml"           => '\u{e664}',
    "hc"             => Icons::LANG_HOLYC,
    "hex"            => '\u{f12a7}',
    "iml"            => Icons::INTELLIJ,
    "ino"            => Icons::LANG_ARDUINO,
    "ipynb"          => Icons::NOTEBOOK,
    "jl"             => '\u{e624}',
    "jwmrc"          => '\u{f35b}',
    "kbx"            => Icons::SHIELD_KEY,
    "less"           => '\u{e758}',
    "license"        => Icons::LICENSE,
    "lisp"           => '\u{f0172}',
    "log"            => Icons::LOG,
    "magnet"         => '\u{f076}',
    "mid"            => '\u{f08f2}',
    "mk"             => Icons::MAKE,
    "msf"            => '\u{f370}',
    "ninja"          => '\u{f0774}',
    "nix"            => '\u{f313}',
    "node"           => Icons::NODEJS,
    "norg"           => '\u{e847}',
    "odf"            => '\u{f37b}',
    "opam"           => '\u{f0627}',
    "org"            => '\u{e633}',
    "out"            => '\u{eb2c}',
    "pdf"            => '\u{f1c1}',
    "pkg"            => '\u{eb29}',
    "pp"             => '\u{e631}',
    "pub"            => Icons::PUBLIC_KEY,
    "purs"           => '\u{e630}',
    "rdb"            => '\u{e76d}',
    "readme"         => Icons::README,
    "rpm"            => '\u{e7bb}',
    "rss"            => '\u{f09e}',
    "scad"           => '\u{f34e}',
    "scala"          => '\u{e737}',
    "service"        => '\u{eba2}',
    "svelte"         => '\u{e697}',
    "tmux"           => Icons::TMUX,
    "toml"           => Icons::TOML,
    "torrent"        => '\u{e275}',
    "twig"           => '\u{e61c}',
    "typ"            => Icons::TYPST,
    "ui"             => '\u{f2d0}',
    "v"              => Icons::LANG_V,
    "vala"           => '\u{e8d1}',
    "vhs"            => '\u{F0A1B}',
    "vi"             => '\u{e81e}',
    "vim"            => Icons::VIM,
    "vsix"           => '\u{f0a1e}',
    "vue"            => '\u{f0844}',
    "xcf"            => Icons::GIMP,
    "xaml"           => '\u{f0673}',
    "xpi"            => '\u{eae6}',
    "zig"            => '\u{e6a9}',
    "zsh-theme"      => Icons::SHELL,
};

/// PHF map for extension colour lookups (non-themed extensions only)
/// Themed extensions are handled in the colour_for_entry function
pub(crate) static EXTENSION_COLOURS: Map<&'static str, Colour> = phf_map! {
    // Non-themed file types that use static colours
    "lock"  => Colour::LightGray,
    "log"   => Colour::White,
};

/// Default fallback values
pub(crate) const DEFAULT_FILE_ICON: char = Icons::FILE;
pub(crate) const DEFAULT_DIR_ICON: char = Icons::FOLDER;
pub(crate) const SYMLINK_ICON: char = Icons::FILE_SYMLINK;

/// Default file colour (from theme)
pub(crate) fn default_file_colour() -> Colour {
    RgbColours::theme().entry_file.colour
}

#[allow(dead_code)]
/// Default directory colour (from theme)
pub(crate) fn default_dir_colour() -> Colour {
    RgbColours::theme().entry_directory.colour
}

/// Symlink colour (from theme)
pub(crate) fn symlink_colour() -> Colour {
    RgbColours::theme().entry_symlink.colour
}

/// Lookup icon for an entry
pub(crate) fn icon_for_entry(
    name: &str,
    extension: &str,
    is_dir: bool,
    has_children: bool,
    is_symlink: bool,
) -> char {
    if is_symlink {
        return SYMLINK_ICON;
    }

    if is_dir {
        if !has_children {
            return Icons::FOLDER_OPEN;
        }
        let name_lower = name.to_lowercase();
        return *DIRECTORY_ICONS
            .get(name_lower.as_str())
            .unwrap_or(&DEFAULT_DIR_ICON);
    }

    // Check filename first (case-insensitive)
    let name_lower = name.to_lowercase();
    if let Some(icon) = FILENAME_ICONS.get(name_lower.as_str()) {
        return *icon;
    }

    // Then check extension
    if !extension.is_empty()
        && let Some(icon) = EXTENSION_ICONS.get(extension)
    {
        return *icon;
    }

    DEFAULT_FILE_ICON
}

/// Lookup colour for an entry
pub(crate) fn colour_for_entry(
    name: &str,
    extension: &str,
    is_dir: bool,
    is_symlink: bool,
) -> Colour {
    if is_symlink {
        return symlink_colour();
    }

    if is_dir {
        return *DIRECTORY_COLOURS
            .get(name)
            .unwrap_or(&RgbColours::cobalite());
    }

    // Check filename first
    if let Some(colour) = FILENAME_COLOURS.get(name) {
        return *colour;
    }

    // Then check themed extensions
    if !extension.is_empty() {
        let themed_colour = match extension {
            // Rust
            "rs" | "rlib" | "rmeta" => Some(RgbColours::almost_apricot()),

            // Python
            "py" | "pyi" | "pyc" | "pyd" | "pyo" | "pyw" | "pyx" | "pxd" | "whl" => {
                Some(RgbColours::mega_blue())
            }

            // JavaScript/TypeScript
            "js" | "mjs" | "cjs" | "ts" | "mts" | "cts" => {
                Some(RgbColours::theme().code_javascript.colour)
            }
            "jsx" | "tsx" => Some(RgbColours::theme().code_javascript.colour),

            // C/C++
            "c" | "h" | "inl" | "m" => Some(RgbColours::thors_thunder()),
            "cpp" | "cc" | "cxx" | "c++" | "hpp" | "hh" | "hxx" | "h++" | "mm" => {
                Some(RgbColours::thors_thunder())
            }

            // Go
            "go" => Some(RgbColours::malibu_blue()),

            // Java/Kotlin
            "java" | "jar" | "class" | "war" | "jad" => Some(RgbColours::princeton_orange()),
            "kt" | "kts" => Some(RgbColours::princeton_orange()),

            // Ruby
            "rb" | "rake" | "gemspec" | "erb" | "slim" => {
                Some(RgbColours::theme().code_ruby.colour)
            }

            // PHP
            "php" | "phar" => Some(RgbColours::theme().code_php.colour),

            // Lua
            "lua" | "luac" | "luau" => Some(RgbColours::theme().code_lua.colour),

            // Shell scripts
            "sh" | "bash" | "zsh" | "fish" | "ksh" | "csh" | "tcsh" | "nu" => {
                Some(RgbColours::thors_thunder())
            }

            // C#/F#
            "cs" | "csx" | "csproj" => Some(RgbColours::theme().code_php.colour),
            "fs" | "fsi" | "fsx" | "fsscript" | "fsproj" => {
                Some(RgbColours::theme().code_php.colour)
            }

            // Rust-like / Systems
            "zig" | "nim" | "nims" | "nimble" => Some(RgbColours::almost_apricot()),

            // Functional languages
            "hs" | "lhs" => Some(RgbColours::theme().code_php.colour),
            "ml" | "mli" | "mll" | "mly" => Some(RgbColours::princeton_orange()),
            "ex" | "exs" | "eex" | "leex" => Some(RgbColours::theme().code_php.colour),
            "erl" | "hrl" => Some(RgbColours::theme().code_ruby.colour),
            "clj" | "cljs" | "cljc" | "edn" => Some(RgbColours::malibu_blue()),
            "rkt" | "scm" | "ss" | "sld" => Some(RgbColours::theme().code_lua.colour),
            "lisp" | "el" | "elc" => Some(RgbColours::theme().code_php.colour),
            "fnl" => Some(RgbColours::theme().code_lua.colour),
            "gleam" => Some(RgbColours::theme().code_ruby.colour),

            // Web frameworks
            "vue" | "svelte" => Some(RgbColours::theme().code_javascript.colour),
            "elm" => Some(RgbColours::malibu_blue()),

            // Other languages
            "dart" => Some(RgbColours::malibu_blue()),
            "swift" => Some(RgbColours::almost_apricot()),
            "scala" => Some(RgbColours::theme().code_ruby.colour),
            "groovy" | "gvy" | "gradle" => Some(RgbColours::princeton_orange()),
            "r" | "rdata" | "rds" => Some(RgbColours::malibu_blue()),
            "jl" => Some(RgbColours::theme().code_javascript.colour),
            "pl" | "pm" | "pod" | "t" | "plx" => Some(RgbColours::theme().code_lua.colour),
            "d" | "di" => Some(RgbColours::theme().code_ruby.colour),
            "cr" => Some(RgbColours::thors_thunder()),
            "purs" => Some(RgbColours::theme().code_php.colour),
            "tcl" | "tbc" => Some(RgbColours::theme().code_lua.colour),
            "vala" => Some(RgbColours::thors_thunder()),
            "awk" => Some(RgbColours::thors_thunder()),
            "v" => Some(RgbColours::malibu_blue()),

            // Assembly / Low-level
            "asm" | "s" => Some(RgbColours::thors_thunder()),
            "hc" => Some(RgbColours::thors_thunder()),

            // HDL
            "sv" | "svh" | "vhdl" => Some(RgbColours::thors_thunder()),

            // Web markup/styles
            "html" | "htm" | "xhtml" | "shtml" => Some(RgbColours::scoville_high()),
            "css" => Some(RgbColours::cyber_grape()),
            "scss" | "sass" | "less" | "styl" | "stylus" => Some(RgbColours::cyber_grape()),

            // Data formats
            "json" | "json5" | "jsonc" | "avro" => Some(RgbColours::theme().web_json.colour),
            "xml" | "xul" | "opml" | "plist" => Some(RgbColours::theme().web_xml.colour),
            "yaml" | "yml" => Some(RgbColours::hawaii_morning()),
            "toml" | "tml" => Some(RgbColours::hawaii_morning()),
            "ini" | "cfg" | "conf" | "config" => Some(RgbColours::hawaii_morning()),
            "env" => Some(RgbColours::hawaii_morning()),

            // Document types
            "txt" | "text" | "rtf" => Some(RgbColours::theme().doc_text.colour),
            "md" | "markdown" | "mkd" | "mdx" | "rmd" | "rdoc" | "jmd" => {
                Some(RgbColours::extraordinary_abundance())
            }
            "pdf" => Some(RgbColours::theme().doc_pdf.colour),
            "rst" => Some(RgbColours::theme().doc_text.colour),
            "org" | "norg" => Some(RgbColours::theme().doc_text.colour),
            "tex" | "latex" | "ltx" | "sty" | "cls" | "bst" | "bib" => {
                Some(RgbColours::theme().doc_text.colour)
            }
            "typ" => Some(RgbColours::theme().doc_text.colour),
            "doc" | "docx" | "docm" => Some(RgbColours::theme().doc_text.colour),
            "odt" | "fodt" => Some(RgbColours::theme().doc_text.colour),
            "epub" | "mobi" | "ebook" => Some(RgbColours::theme().doc_pdf.colour),
            "djvu" | "djv" => Some(RgbColours::theme().doc_pdf.colour),

            // Spreadsheets
            "xls" | "xlsx" | "xlsm" | "xlr" | "csv" | "tsv" => {
                Some(RgbColours::theme().code_javascript.colour)
            }
            "ods" | "fods" => Some(RgbColours::theme().code_javascript.colour),
            "gsheet" => Some(RgbColours::theme().code_javascript.colour),

            // Presentations
            "ppt" | "pptx" | "pps" | "ppsx" => Some(RgbColours::almost_apricot()),
            "odp" | "fodp" | "gslides" => Some(RgbColours::almost_apricot()),

            // Image types
            "png" | "jpg" | "jpeg" | "jpe" | "jif" | "jfif" | "jfi" => {
                Some(RgbColours::sachet_pink())
            }
            "gif" | "webp" | "avif" | "jxl" => Some(RgbColours::sachet_pink()),
            "bmp" | "ico" | "tif" | "tiff" => Some(RgbColours::sachet_pink()),
            "svg" | "eps" | "ps" => Some(RgbColours::sachet_pink()),
            "psd" | "psb" | "xcf" | "kra" | "krz" => Some(RgbColours::sachet_pink()),
            "raw" | "cr2" | "nef" | "orf" | "arw" | "dng" => Some(RgbColours::sachet_pink()),
            "heic" | "heif" => Some(RgbColours::sachet_pink()),
            "jp2" | "j2k" | "j2c" | "jpf" | "jpx" => Some(RgbColours::sachet_pink()),
            "pbm" | "pgm" | "ppm" | "pnm" | "pxm" | "xpm" => Some(RgbColours::sachet_pink()),
            "ai" => Some(RgbColours::sachet_pink()),
            "cbr" | "cbz" => Some(RgbColours::sachet_pink()),

            // Video types
            "mp4" | "m4v" | "mkv" | "webm" => Some(RgbColours::mandarin_sorbet()),
            "avi" | "mov" | "wmv" | "flv" => Some(RgbColours::mandarin_sorbet()),
            "mpeg" | "mpg" | "m2v" | "m2ts" => Some(RgbColours::mandarin_sorbet()),
            "vob" | "ogv" | "ogm" => Some(RgbColours::mandarin_sorbet()),
            "3gp" | "3g2" | "3gpp" | "3gpp2" => Some(RgbColours::mandarin_sorbet()),
            "h264" | "heics" | "cast" => Some(RgbColours::mandarin_sorbet()),

            // Audio types
            "mp3" | "wav" | "flac" | "m4a" | "ogg" | "opus" => {
                Some(RgbColours::exhilarating_green())
            }
            "aac" | "wma" | "aif" | "aiff" | "aifc" | "alac" => {
                Some(RgbColours::exhilarating_green())
            }
            "ape" | "mka" | "wv" | "mp2" | "pcm" => Some(RgbColours::exhilarating_green()),
            "mid" | "sf2" | "sfz" => Some(RgbColours::exhilarating_green()),

            // Subtitles
            "srt" | "sub" | "ass" | "ssa" | "vtt" | "lrc" => {
                Some(RgbColours::theme().doc_text.colour)
            }

            // Playlists
            "m3u" | "m3u8" | "pls" | "cue" => Some(RgbColours::exhilarating_green()),

            // Archive types
            "zip" | "tar" | "gz" | "7z" | "rar" => Some(RgbColours::theme().archive.colour),
            "bz" | "bz2" | "bz3" | "xz" | "lz" | "lz4" | "lzma" | "lzo" | "lzh" => {
                Some(RgbColours::theme().archive.colour)
            }
            "tgz" | "tbz" | "tbz2" | "txz" | "tlz" | "taz" | "tz" | "tzo" => {
                Some(RgbColours::theme().archive.colour)
            }
            "zst" | "z" | "ar" | "arj" | "cpio" | "par" | "cab" => {
                Some(RgbColours::theme().archive.colour)
            }
            "deb" | "rpm" | "pkg" | "apk" => Some(RgbColours::theme().archive.colour),
            "dmg" | "iso" | "img" | "qcow" | "qcow2" | "vdi" | "vmdk" | "vhd" | "tc" => {
                Some(RgbColours::theme().archive.colour)
            }

            // Database
            "sql" | "sqlite" | "sqlite3" | "db" | "db3" | "s3db" | "sl3" => {
                Some(RgbColours::mega_blue())
            }
            "mdb" | "ldb" | "odb" | "dump" | "prql" => Some(RgbColours::mega_blue()),

            // Fonts
            "ttf" | "otf" | "woff" | "woff2" | "eot" => Some(RgbColours::theme().doc_text.colour),
            "fon" | "fnt" | "bdf" | "psf" | "flc" | "flf" | "lff" | "font" => {
                Some(RgbColours::theme().doc_text.colour)
            }

            // 3D/CAD
            "obj" | "fbx" | "stl" | "ply" | "3mf" | "blend" => Some(RgbColours::malibu_blue()),
            "dwg" | "dxf" | "step" | "stp" | "ste" | "iges" | "igs" | "ige" => {
                Some(RgbColours::malibu_blue())
            }
            "ifc" | "brep" | "f3d" | "f3z" | "skp" | "slvs" => Some(RgbColours::malibu_blue()),
            "fcstd" | "fcstd1" | "scad" => Some(RgbColours::malibu_blue()),

            // Security/Keys
            "pem" | "crt" | "cert" | "key" | "p12" | "pfx" | "pub" => {
                Some(RgbColours::theme().code_ruby.colour)
            }
            "gpg" | "asc" | "age" | "sig" | "signature" => {
                Some(RgbColours::theme().code_ruby.colour)
            }
            "kdb" | "kdbx" | "kbx" => Some(RgbColours::theme().code_ruby.colour),

            // Checksums
            "md5" | "sha1" | "sha224" | "sha256" | "sha384" | "sha512" => {
                Some(RgbColours::theme().checksum.colour)
            }

            // Build/Config
            "cmake" | "mk" | "ninja" => Some(RgbColours::thors_thunder()),
            "dockerfile" | "dockerignore" => Some(RgbColours::malibu_blue()),
            "tf" | "tfstate" | "tfvars" => Some(RgbColours::theme().code_php.colour),
            "nix" => Some(RgbColours::malibu_blue()),
            "ebuild" => Some(RgbColours::theme().code_php.colour),

            // Git
            "git" | "gitignore" | "gitattributes" | "gitmodules" => {
                Some(RgbColours::almost_apricot())
            }
            "diff" | "patch" => Some(RgbColours::theme().code_javascript.colour),

            // Notebooks
            "ipynb" => Some(RgbColours::mega_blue()),

            // Misc
            "graphql" | "gql" => Some(RgbColours::theme().code_ruby.colour),
            "dot" | "gv" => Some(RgbColours::thors_thunder()),
            "po" | "pot" | "mo" | "qm" => Some(RgbColours::theme().doc_text.colour),

            _ => None,
        };

        if let Some(colour) = themed_colour {
            return colour;
        }

        // Fall back to static extension map for non-themed extensions
        if let Some(colour) = EXTENSION_COLOURS.get(extension) {
            return *colour;
        }
    }

    default_file_colour()
}
