<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	let prov: string;
	let keys: string;
	type Config = {
		provider: string;
		keystore: string;
	};

	async function onSubmit(): Promise<void> {
        event?.preventDefault();
        await invoke<Config>('set_config', { provider: prov, keystore: keys })
            .catch((error) => console.error(error)
		);
	}
</script>

<div class="navbar rounded-xl place-content-center mt-5">
	<a href="./" class="btn btn-ghost normal-case text-xl ">Home</a>
    <a href="/deploy" class="btn btn-ghost normal-case text-xl">Deploy</a>
</div>
<div class="flex flex-col justify-center items-center h-screen min-h-screen">
	<form on:submit={() => onSubmit()}>
			<div class="form-control w-full max-w-xs mb-5">
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<label class="label">
					<span class="label-text">Keystore Path</span>
				</label>
				<input
					type="text"
					bind:value={keys}
					placeholder="|> path"
					class="input input-bordered w-full max-w-xs"
					required
				/>
			</div>
			<div class="form-control w-full max-w-xs mb-5">
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<label class="label">
					<span class="label-text">JSON-RPC Provider</span>
				</label>
				<input
					type="text"
					bind:value={prov}
					placeholder="|> url"
					class="input input-bordered w-full max-w-xs"
					required
				/>
			</div>
			<div class="flex flex-col justify-center items-center">
                <button type="submit" class="btn btn-primary rounded-xl border-8 mt-5">Save
                </button>
            </div>
	</form>
</div>
