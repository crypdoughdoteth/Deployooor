
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * Environment variables [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env`. Like [`$env/dynamic/private`](https://kit.svelte.dev/docs/modules#$env-dynamic-private), this module cannot be imported into client-side code. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://kit.svelte.dev/docs/configuration#env) (if configured).
 * 
 * _Unlike_ [`$env/dynamic/private`](https://kit.svelte.dev/docs/modules#$env-dynamic-private), the values exported from this module are statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * ```ts
 * import { API_KEY } from '$env/static/private';
 * ```
 * 
 * Note that all environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * 
 * ```
 * MY_FEATURE_FLAG=""
 * ```
 * 
 * You can override `.env` values from the command line like so:
 * 
 * ```bash
 * MY_FEATURE_FLAG="enabled" npm run dev
 * ```
 */
declare module '$env/static/private' {
	export const SHELL: string;
	export const npm_command: string;
	export const LSCOLORS: string;
	export const PIPENV_VENV_IN_PROJECT: string;
	export const npm_config_userconfig: string;
	export const npm_config_cache: string;
	export const LESS: string;
	export const HISTCONTROL: string;
	export const WSL2_GUI_APPS_ENABLED: string;
	export const WSL_DISTRO_NAME: string;
	export const WT_SESSION: string;
	export const HISTSIZE: string;
	export const HOSTNAME: string;
	export const NODE: string;
	export const DOTNET_ROOT: string;
	export const COLOR: string;
	export const npm_config_local_prefix: string;
	export const npm_config_globalconfig: string;
	export const EDITOR: string;
	export const MACOSX_DEPLOYMENT_TARGET: string;
	export const NAME: string;
	export const PWD: string;
	export const LOGNAME: string;
	export const TAURI_PLATFORM: string;
	export const npm_config_init_module: string;
	export const _: string;
	export const HOME: string;
	export const LANG: string;
	export const WSL_INTEROP: string;
	export const LS_COLORS: string;
	export const npm_package_version: string;
	export const TAURI_PLATFORM_VERSION: string;
	export const WAYLAND_DISPLAY: string;
	export const SSL_CERT_DIR: string;
	export const VIRTUAL_ENV_DISABLE_PROMPT: string;
	export const TAURI_TARGET_TRIPLE: string;
	export const VIRTUALENVWRAPPER_PYTHON: string;
	export const TAURI_ARCH: string;
	export const INIT_CWD: string;
	export const DOTNET_BUNDLE_EXTRACT_BASE_DIR: string;
	export const npm_lifecycle_script: string;
	export const npm_config_npm_version: string;
	export const TERM: string;
	export const npm_package_name: string;
	export const ZSH: string;
	export const npm_config_prefix: string;
	export const LESSOPEN: string;
	export const USER: string;
	export const DISPLAY: string;
	export const npm_lifecycle_event: string;
	export const SHLVL: string;
	export const PAGER: string;
	export const npm_config_user_agent: string;
	export const npm_execpath: string;
	export const XDG_RUNTIME_DIR: string;
	export const SSL_CERT_FILE: string;
	export const DEBUGINFOD_URLS: string;
	export const TAURI_FAMILY: string;
	export const npm_package_json: string;
	export const WSLENV: string;
	export const KDEDIRS: string;
	export const TAURI_DEBUG: string;
	export const XDG_DATA_DIRS: string;
	export const npm_config_noproxy: string;
	export const PATH: string;
	export const npm_config_node_gyp: string;
	export const TAURI_PLATFORM_TYPE: string;
	export const npm_config_python: string;
	export const CARGO: string;
	export const npm_config_global_prefix: string;
	export const MAIL: string;
	export const HOSTTYPE: string;
	export const PULSE_SERVER: string;
	export const WT_PROFILE_ID: string;
	export const npm_node_execpath: string;
	export const OLDPWD: string;
	export const NODE_ENV: string;
}

/**
 * Similar to [`$env/static/private`](https://kit.svelte.dev/docs/modules#$env-static-private), except that it only includes environment variables that begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Values are replaced statically at build time.
 * 
 * ```ts
 * import { PUBLIC_BASE_URL } from '$env/static/public';
 * ```
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to runtime environment variables, as defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://kit.svelte.dev/docs/cli)), this is equivalent to `process.env`. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://kit.svelte.dev/docs/configuration#env) (if configured).
 * 
 * This module cannot be imported into client-side code.
 * 
 * Dynamic environment variables cannot be used during prerendering.
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * console.log(env.DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 * 
 * > In `dev`, `$env/dynamic` always includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 */
declare module '$env/dynamic/private' {
	export const env: {
		SHELL: string;
		npm_command: string;
		LSCOLORS: string;
		PIPENV_VENV_IN_PROJECT: string;
		npm_config_userconfig: string;
		npm_config_cache: string;
		LESS: string;
		HISTCONTROL: string;
		WSL2_GUI_APPS_ENABLED: string;
		WSL_DISTRO_NAME: string;
		WT_SESSION: string;
		HISTSIZE: string;
		HOSTNAME: string;
		NODE: string;
		DOTNET_ROOT: string;
		COLOR: string;
		npm_config_local_prefix: string;
		npm_config_globalconfig: string;
		EDITOR: string;
		MACOSX_DEPLOYMENT_TARGET: string;
		NAME: string;
		PWD: string;
		LOGNAME: string;
		TAURI_PLATFORM: string;
		npm_config_init_module: string;
		_: string;
		HOME: string;
		LANG: string;
		WSL_INTEROP: string;
		LS_COLORS: string;
		npm_package_version: string;
		TAURI_PLATFORM_VERSION: string;
		WAYLAND_DISPLAY: string;
		SSL_CERT_DIR: string;
		VIRTUAL_ENV_DISABLE_PROMPT: string;
		TAURI_TARGET_TRIPLE: string;
		VIRTUALENVWRAPPER_PYTHON: string;
		TAURI_ARCH: string;
		INIT_CWD: string;
		DOTNET_BUNDLE_EXTRACT_BASE_DIR: string;
		npm_lifecycle_script: string;
		npm_config_npm_version: string;
		TERM: string;
		npm_package_name: string;
		ZSH: string;
		npm_config_prefix: string;
		LESSOPEN: string;
		USER: string;
		DISPLAY: string;
		npm_lifecycle_event: string;
		SHLVL: string;
		PAGER: string;
		npm_config_user_agent: string;
		npm_execpath: string;
		XDG_RUNTIME_DIR: string;
		SSL_CERT_FILE: string;
		DEBUGINFOD_URLS: string;
		TAURI_FAMILY: string;
		npm_package_json: string;
		WSLENV: string;
		KDEDIRS: string;
		TAURI_DEBUG: string;
		XDG_DATA_DIRS: string;
		npm_config_noproxy: string;
		PATH: string;
		npm_config_node_gyp: string;
		TAURI_PLATFORM_TYPE: string;
		npm_config_python: string;
		CARGO: string;
		npm_config_global_prefix: string;
		MAIL: string;
		HOSTTYPE: string;
		PULSE_SERVER: string;
		WT_PROFILE_ID: string;
		npm_node_execpath: string;
		OLDPWD: string;
		NODE_ENV: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * Similar to [`$env/dynamic/private`](https://kit.svelte.dev/docs/modules#$env-dynamic-private), but only includes variables that begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Note that public dynamic environment variables must all be sent from the server to the client, causing larger network requests — when possible, use `$env/static/public` instead.
 * 
 * Dynamic environment variables cannot be used during prerendering.
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.PUBLIC_DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}
