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

	onMount(async () => {
		await getData();
		totalPosts = allContent!.length;
		totalPages = Math.ceil(totalPosts / postsPerPage);
		getItems();
		loading = false;
	});

	let allContent: Array<DeploymentData> | undefined;
	let currentPage = 1;
	const postsPerPage = 6;
	let totalPosts: number;
	let totalPages: number;
	let loading = true;
	let curr_items: DeploymentData[];
	//console.log(curr_items);
	function getItems(): DeploymentData[] {
		const startIndex = (currentPage - 1) * postsPerPage;
		const endIndex = startIndex + postsPerPage;
		const slice = allContent!.slice(startIndex, endIndex);
		curr_items = slice;
		curr_items = curr_items;
		return curr_items;
	}

	const setCurrentPage = (newPage: number) => {
		currentPage = newPage;
		getItems();
	};

	async function getData(): Promise<void> {
		// trigger DB dump of deployment details on Rust side
		await invoke<Array<DeploymentData>>('db_read', {})
			.then((message) => {
				allContent = message;
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
	<div>
		<table class="table shrink w-74">
			<!-- head -->
			<thead class="flex">
				<tr class="flex justify-center space-x-24">
					<th>Name</th>
					<!-- <th>Deployer Address</th> -->
					<th>Date</th>
					<th>Contract Address</th>
					<th>Network</th>
				</tr>
			</thead>
			<tbody class="flex flex-col">
				{#if loading === true}
					<tr>
						<h5 class="text-center mb-5 mt-5">Loading...</h5>
					</tr>
				{:else if loading === false && curr_items.length === 0}
					<tr>
						<h5 class="text-center mb-5 mt-5">Nothing to display</h5>
					</tr>
				{:else}
					{#each curr_items as row}
						<tr class="hover">
							<td>{row.sc_name}</td>
							<!-- <td>{row.deployer_address}</td> -->
							<td>{row.deploy_date}</td>
							<td>{row.sc_address}</td>
							<td>{row.network}</td>
						</tr>
					{/each}
				{/if}
			</tbody>
		</table>
		<div class="flex flex-col join items-center mt-10">
			<div>
				<button
					on:click|preventDefault={() => setCurrentPage(currentPage - 1)}
					class="join-item btn btn-next-prev">«</button
				>

				<button
					on:click|preventDefault={() => setCurrentPage(currentPage + 1)}
					class="join-item btn btn-next-prev">»</button
				>
			</div>
		</div>
	</div>
</div>
