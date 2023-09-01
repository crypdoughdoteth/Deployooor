<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	type DeploymentData = {
		contract_name: string;
		deployer_address: string;
		date: string;
		contract_address: string;
		network: string;
	};

	let data: Array<DeploymentData>;

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

	onMount(async () => {
		await getData();
	});

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
			<thead>
				<tr>
					<th />
					<th>Name</th>
					<th>Deployer Address</th>
					<th>Date</th>
					<th>Contract Address</th>
					<th>Network</th>
					<th>Fee</th>
				</tr>
			</thead>
			<tbody>
				<!-- row 1 -->
				<tr class="hover">
					<th>1</th>
					<td>Cy Ganderton</td>
					<td>Quality Control Specialist</td>
					<td>Blue</td>
				</tr>
				<!-- row 2 -->
				<tr class="hover">
					<th>2</th>
					<td>Hart Hagerty</td>
					<td>Desktop Support Technician</td>
					<td>Purple</td>
				</tr>
				<!-- row 3 -->
				<tr class="hover">
					<th>3</th>
					<td>Brice Swyre</td>
					<td>Tax Accountant</td>
					<td>Red</td>
				</tr>
			</tbody>
		</table>
	</div>
</div>
