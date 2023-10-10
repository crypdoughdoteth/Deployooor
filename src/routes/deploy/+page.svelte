<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { HDNodeWallet, JsonRpcProvider, Wallet, ethers, formatEther } from 'ethers';
	import { onMount } from 'svelte';
	import { configuration, deployment } from '../../stores';
	import type { deploymentDetails } from '../../stores';

	type ReturnData = {
		abi: JSON;
		initcode: string;
	};

	$: getKeys(password);

	let version: string;
	let addy: string;
	let bal: string = '0';
	let network: string;
	let deploymentError: boolean;
	let deploymentErrorMsg: unknown;
	// I
	let contractFile: string;
	let password: string;
	let args: string;
	// O
	let res: ReturnData;

	let configFound: boolean;
	let web3: JsonRpcProvider;
	let decryptedWallet: Wallet | HDNodeWallet;
	let contractAddress: string;
	let success: boolean;
	let deployInit: boolean = false;
	let gasEstimated = false;
	let gas: string;

	async function getKeys(pass: string): Promise<Wallet | HDNodeWallet> {
		await invoke<JSON>('get_keys', { keyPath: $configuration.keystore })
			.then(async (message) => {
				const keys = JSON.stringify(message);
				decryptedWallet = (await ethers.Wallet.fromEncryptedJson(keys, pass)).connect(web3);
				addy = decryptedWallet.address;
				bal = formatEther(await web3.getBalance(addy));
			})
			.catch((err) => {
				console.error(err);
			});
		return decryptedWallet;
	}

	async function estimateGas() {
		await invoke<ReturnData>('compile_version', { version: version, path: contractFile })
			.then(async (message) => {
				console.log(message);
				let ret = message;
				let bytecode = ret.initcode;
				const abi = new ethers.Interface(JSON.stringify(ret.abi));
				const contract = new ethers.ContractFactory(abi, { object: bytecode }, decryptedWallet);
				//fetch gas price
				const deployTx = await contract.getDeployTransaction([args]);
				const estimateGas = await decryptedWallet.estimateGas(deployTx);
				const gasPrice = ethers.formatEther(estimateGas);
				gas = gasPrice;
				gasEstimated = true;
				return gas;
			})
			.catch((error) => {
				// deploymentError = true;
				// deploymentErrorMsg = error;
				gasEstimated = false;
				console.error(error);
			});
	}

	async function onSubmit(): Promise<void> {
		event?.preventDefault();
		// callback to Rust code to
		// I.   Compile Vyper file
		// II.  Update provider
		console.log(version);
		console.log('calling rust code ..... ');
		deployInit = true;
		await invoke<ReturnData>('compile_version', {
			path: contractFile,
			version: version
		})
			.then((message) => {
				console.log(message);
				res = message;
			})
			.catch((error) => {
				deploymentError = true;
				deploymentErrorMsg = error;
				console.error(error);
			});
		let bytecode = res.initcode;
		const abi = new ethers.Interface(JSON.stringify(res.abi));
		const contract = new ethers.ContractFactory(abi, { object: bytecode }, decryptedWallet);
		let tx;
		try {
			tx = await contract.deploy([args]);
			await tx.waitForDeployment();
			contractAddress = await tx.getAddress();
			let date = new Date().toLocaleDateString() + '@' + new Date().toLocaleTimeString();
			$deployment = {
				sc_name: contractFile,
				deployer_address: addy,
				deploy_date: date,
				sc_address: contractAddress,
				network: network
			};
			console.log(contractAddress);
			success = true;
			await recordDeployment({
				sc_name: contractFile,
				deployer_address: addy,
				deploy_date: date,
				sc_address: contractAddress,
				network: network
			});
		} catch (e) {
			deploymentError = true;
			deploymentErrorMsg = e;
			setTimeout(() => (deploymentError = false), 12000);
		}
		deployInit = false;
	}

	async function recordDeployment(deployment: deploymentDetails): Promise<void> {
		// trigger DB dump of deployment details on Rust side
		console.log('here we go!');
		await invoke('db_write', { deploymentData: deployment }).catch((err) => {
			console.error(err);
		});
	}

	onMount(async () => {
		console.log('Deploy Page State', $configuration.provider);
		web3 = new ethers.JsonRpcProvider($configuration.provider);
		$deployment.network = (await web3.getNetwork()).name;
		network = $deployment.network;
		if ($configuration.provider === '') {
			configFound = false;
		} else {
			configFound = true;
		}
	});
</script>

<div class="navbar rounded-xl place-content-center mt-5 mb-5">
	<a href="./" class="btn btn-ghost normal-case text-xl">Home</a>
	<a href="/settings" class="btn btn-ghost normal-case text-xl">Settings</a>
	<a href="/deployments" class="btn btn-ghost normal-case text-xl">Deployments</a>
</div>
<div class="flex flex-col justify-center items-center">
	<h1 class="text-3xl font-bold mt-5 mb-10 text-emerald-700">Deploy Your Contract!</h1>
</div>
<div class="flex flex-col justify-center items-center h-screen min-h-screen">
	<div class="card w-108 bg-neutral text-neutral-content mt-10">
		<div class="card-body items-center text-center">
			<div class="font-bold">
				{#if configFound === true}
					<h3>Network: {$deployment.network}</h3>
				{:else}
					<h3 class="text-red-600 font-bold underline">Config Not Found!</h3>
				{/if}
				{#if addy !== undefined}
					<h3>Address: {addy}</h3>
					<h3>Gas Balance: {bal}</h3>
				{/if}
				{#if gasEstimated === true}
					<h3>Gas Estimation: {gas}</h3>
				{/if}
			</div>
		</div>
	</div>

	<form on:submit={() => onSubmit()}>
		<div class="form-control w-full max-w-xs mt-10">
			<!-- svelte-ignore a11y-label-has-associated-control -->
			<label class="label">
				<span class="label-text">EVM Version</span>
			</label>
			<select bind:value={version} class="select select-bordered">
				<option selected>Shanghai</option>
				<option>Paris</option>
				<option>Berlin</option>
				<option>Istanbul</option>
				<option>Cancun</option>
			</select>
		</div>

		<div class="form-control w-full max-w-xs mt-5 mb-5">
			<!-- svelte-ignore a11y-label-has-associated-control -->
			<label class="label">
				<span class="label-text">Vyper Contract Path</span>
			</label>
			<input
				type="text"
				bind:value={contractFile}
				placeholder="|> path"
				class="input input-bordered w-full max-w-xs"
				required
			/>
		</div>
		<div class="form-control w-full max-w-xs mb-5">
			<!-- svelte-ignore a11y-label-has-associated-control -->
			<label class="label">
				<span class="label-text">Constructor Args</span>
			</label>
			<input
				type="text"
				bind:value={args}
				placeholder="|> args"
				class="input input-bordered w-full max-w-xs"
				required
			/>
		</div>
		<div class="form-control w-full max-w-xs mb-2">
			<!-- svelte-ignore a11y-label-has-associated-control -->
			<label class="label">
				<span class="label-text">Decrypt Wallet</span>
			</label>
			<input
				type="password"
				bind:value={password}
				placeholder="|> password"
				class="input input-bordered w-full max-w-xs"
				required
			/>
			<label for="my_modal_7" class="btn btn-primary border-8 rounded-XL mt-10">Next</label>
			<input type="checkbox" id="my_modal_7" class="modal-toggle" />
			<div class="modal backdrop-blur-md">
				<div class="modal-box flex flex-col">
					<h3 class="text-lg font-bold justify-center">Are You Ready To Deploy Your Contract?</h3>
					{#if configFound === false}
						<div class="alert alert-error mt-10 mb-10">
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="stroke-current shrink-0 h-6 w-6"
								fill="none"
								viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
								/></svg
							>
							<span>Error! Config Not Found.</span>
						</div>
					{/if}
					{#if deploymentError === true}
						<div class="alert alert-error mt-5 mb-5">
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="stroke-current shrink-0 h-6 w-6"
								fill="none"
								viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
								/></svg
							>
							<span>Error! {deploymentErrorMsg} </span>
						</div>
					{/if}
					{#if success === true}
						<div class="alert alert-success flex flex-col mt-5 mb-5">
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
							<span>Contract Deployed Successfully!</span>
							<span>Address: {contractAddress}</span>
						</div>
					{/if}

					<div class="card w-108 bg-neutral text-neutral-content mt-5 mb-5">
						<div class="card-body items-center text-center">
							<div class="font-bold">
								<h3>Deployment Network: {$deployment.network}</h3>
								<h3>Gas Estimation: {gas}</h3>
							</div>
						</div>
					</div>
					{#if deployInit === false}
						<button type="submit" class="btn btn-primary rounded-xl border-8">DEPLOY</button>
					{:else}
						<span class="loading loading-ring loading-lg mt-5 mb-5 self-center" />
						<button class="btn btn-primary rounded-xl border-8" disabled>DEPLOY</button>
					{/if}
				</div>
				<label class="modal-backdrop" for="my_modal_7">Close</label>
			</div>
		</div>
	</form>
	<button on:click={() => estimateGas()} class="btn btn-primary border-8 rounded-XL px-14">
		Estimate Gas
	</button>
</div>
