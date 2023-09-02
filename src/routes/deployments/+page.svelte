<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	type DeploymentData = {
		sc_name: string;
		deployer_address: string;
		deploy_date: string;
		sc_address: string;
		network: string;
	};

	let data: Array<DeploymentData>;

	onMount(async () => {
		await getData();

	});


	async function getData(): Promise<void> {
		// trigger DB dump of deployment details on Rust side
		await invoke<Array<DeploymentData>>('db_read', {})
			.then((message) => {
				data = message;
			})
			.catch((err) => {
				console.error(err);
			});
	}
</script>

<div class="navbar rounded-xl place-content-center mt-5">
	<a href="./" class="btn btn-ghost normal-case text-xl">Home</a>
	<a href="/deploy" class="btn btn-ghost normal-case text-xl">Deploy</a>
	<a href="/settings" class="btn btn-ghost normal-case text-xl">Settings</a>
</div>
<div class="flex flex-col justify-center items-center h-screen min-h-screen">
	<div class="overflow-x-auto">
		<table class="table">
			<!-- head -->
			<thead class="flex flex-row items-center mb-10">
				<tr>
					<th />
					<th>Name</th>
					<th>Deployer Address</th>
					<th>Date</th>
					<th>Contract Address</th>
					<th>Network</th>
				</tr>
			</thead>
			<tbody>
				<!-- svelte-ignore missing-declaration -->
				<!-- {#each _ as row}
				<tr class="hover">
					<th>{row.contract_name}</th>
					<th>{row.deployer_address}</th>
					<th>{row.date}</th>
					<th>{row.contract_address}</th>
					<th>{row.network}</th>
				</tr>
				{:else}
				<tr>
					<h5 class="text-center mb-10">There is no data to display here.</h5>
				</tr>
				{/each} -->
			</tbody>
		</table>
		<div class="flex flex-col join items-center mt-10">
			<div>
			<button on:click={() => {}} class="join-item btn btn-next-prev">«</button>
			<button on:click={() => {}} class="join-item btn btn-next-prev">»</button>
			</div>
		</div>
	</div>
</div>
