<script lang="ts">
	import { onMount } from 'svelte';
	import { configuration } from '../stores';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { Config } from '../stores';

	// on application start, load config in and set state stores
	onMount(async () => {
		await invoke<Config>('get_config', {})
			.then(async (message) => {
				console.log(message);
				$configuration = {
					provider: message.provider,
					keystore: message.keystore
				};
			})
			.catch((error) => {
				console.error(error);
			});
	});
</script>

<div class="flex justify-center items-center h-screen hero min-h-screen">
	<div class="hero-content text-center">
		<div class="max-w-md">
			<h1 class="text-5xl font-bold">Vyper Deployer</h1>
			<p class="py-6">
				It's easy! All you need is an encrypted keystore, a provider, and the path to your Vyper
				contract.
			</p>
			<div class="space-x-5">
				<button class="btn btn-primary rounded-lg">
					<a href="/settings">Settings </a>
				</button>
				<button class="btn btn-primary border-8 rounded-lg">
					<a href="/deploy">Get Started</a>
				</button>
				<button class="btn btn-primary border-8 rounded-lg">
					<a href="/deployments">Deployments</a>
				</button>
			</div>
		</div>
	</div>
</div>
