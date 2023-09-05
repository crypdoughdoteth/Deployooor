<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { fade } from 'svelte/transition';

	let pw: string;
	let name: string;
	let path: string;
	let keystorePending = false;
	let success = false;
	async function createKeystore(path: string, pass: string, name: string): Promise<void> {
		keystorePending = true;
		await invoke('generate_keystore', { path: path, password: pass, name: name })
			.then((_) => {
				success = true;
				setTimeout(() => (success = false), 6000);
			})
			.catch((error) => console.error(error));
		keystorePending = false;
	}
</script>

<div class="navbar rounded-xl place-content-center mt-5">
	<a href="/settings" class="btn btn-ghost normal-case text-xl">Settings</a>
</div>
{#if success === true}
	<div transition:fade class="alert alert-success">
		<svg
			xmlns="http://www.w3.org/2000/svg"
			class="stroke-current shrink-0 h-6 w-6"
			fill="none"
			viewBox="0 0 24 24"
			><path
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
			/></svg
		>
		<span>Settings Saved Successfully!</span>
	</div>
{/if}
<div class="flex flex-col justify-center items-center">
	<h1 class="text-3xl font-bold text-emerald-700 mt-10">Create Your Keystore!</h1>
</div>

<div class="flex flex-col justify-center items-center h-screen min-h-screen">
	<form on:submit={() => createKeystore(path, pw, name)}>
		<div class="form-control w-full max-w-xs">
			<!-- svelte-ignore a11y-label-has-associated-control -->
			<label class="label">
				<span class="label-text">Keystore Name</span>
			</label>
			<input
				bind:value={name}
				type="text"
				placeholder="|> name"
				class="input input-bordered w-full max-w-xs"
				required
			/>
		</div>
		<div class="form-control w-full max-w-xs">
			<!-- svelte-ignore a11y-label-has-associated-control -->
			<label class="label">
				<span class="label-text">Keystore Path</span>
			</label>
			<input
				bind:value={path}
				type="text"
				placeholder="|> path"
				class="input input-bordered w-full max-w-xs"
				required
			/>
		</div>
		<div class="form-control w-full max-w-xs">
			<!-- svelte-ignore a11y-label-has-associated-control -->
			<label class="label">
				<span class="label-text">Encrypt With Password</span>
			</label>
			<input
				bind:value={pw}
				type="text"
				placeholder="|> password"
				class="input input-bordered w-full max-w-xs"
				required
			/>
		</div>
		<div class="flex flex-col justify-center items-center">
			<button type="submit" class="btn btn-primary rounded-xl border-8 mt-10">CREATE</button>
		</div>
	</form>
</div>
