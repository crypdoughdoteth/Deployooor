<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { HDNodeWallet, JsonRpcProvider, Wallet, ethers, formatEther } from 'ethers';
	import { onMount } from 'svelte';

	type ReturnData = {
		abi: JSON;
		initcode: string;
	};
	type Config = {
		provider: string;
		keystore: string;
	};

	$: getKeys(password);

	let addy: string;
	let bal: string;
	let network: string; 
	let deploymentError: boolean;
	let deploymentErrorMsg: unknown;
	// I
	let contractFile: string;
	let password: string;
	let args: string;
	// O
	let res: ReturnData;
	let conf: Config;
	let configFound: boolean;
	let web3: JsonRpcProvider;
	let decryptedWallet: Wallet | HDNodeWallet;
	let contractAddress: string;
	async function getKeys(pass: string): Promise<Wallet | HDNodeWallet> {
		await invoke<JSON>('get_keys', { keyPath: conf.keystore })
			.then(async (message) => {
					const keys = JSON.stringify(message);
					decryptedWallet = (await ethers.Wallet.fromEncryptedJson(keys, pass)).connect(web3);
					addy = decryptedWallet.address
					bal = formatEther(await web3.getBalance(addy));
				}) 
			.catch((err) => {console.error(err)});
		return decryptedWallet;
	}
	
	async function onSubmit(): Promise<void> {
		event?.preventDefault();
		// callback to Rust code to
		// I.   Compile Vyper file
		// II.  Update provider

		console.log('calling rust code ..... ');
		await invoke<ReturnData>('fetch_data', { path: contractFile, keyPath: conf.keystore })
			.then((message) => {
				console.log(message);
				res = message;
			})
			.catch((error) => {
				deploymentError = true;
				deploymentErrorMsg = error;
				console.error(error);
			});
			await invoke<JSON>('get_keys', { keyPath: conf.keystore })
				.then(async (message) => {
						const keys = JSON.stringify(message);
						decryptedWallet = (await ethers.Wallet.fromEncryptedJson(keys, password)).connect(web3);
						addy = decryptedWallet.address
						bal = formatEther(await web3.getBalance(addy));
					}) 
				.catch((err) => {console.error(err)});
		let bytecode = res.initcode;
		const abi = new ethers.Interface(JSON.stringify(res.abi));
		const contract = new ethers.ContractFactory(abi, { object: bytecode }, decryptedWallet);
		try {
			let tx = await contract.deploy([args]);
			await tx.waitForDeployment();
			contractAddress = await tx.getAddress();
			console.log(contractAddress);
			deploymentError = false;
		} catch (e) {
			deploymentError = true;
			deploymentErrorMsg = e;
			setTimeout(() => (deploymentError = false), 3000);
		}
		configFound = true;
	}

	onMount(async () => {
		await invoke<Config>('get_config', {})
			.then(async (message) => {
				console.log(message);
				conf = message;
				web3 = new ethers.JsonRpcProvider(conf.provider);
				network = (await web3.getNetwork()).name;
			})
			.catch((error) => {
				configFound = false;
				console.error(error);
			});
	});
</script>

<div class="navbar rounded-xl place-content-center mt-5">
	<a href="./" class="btn btn-ghost normal-case text-xl">Home</a>
	<a href="/settings" class="btn btn-ghost normal-case text-xl">Settings</a>
</div>
<div class="flex flex-col justify-center items-center h-screen min-h-screen">
	
	<div class="card w-108 bg-neutral text-neutral-content mt-10 ">
		<div class="card-body items-center text-center">
			<div class="font-bold">
				<h3> Network: {network}</h3>
				<h3> Address: {addy}</h3>
				<h3> Gas Balance: {bal}</h3>
			</div>
		</div>
	  </div>	
	

	<form on:submit={() => onSubmit()}>
		<div class="form-control w-full max-w-xs mt-10 mb-5">
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
		<div class="form-control w-full max-w-xs mb-5">
			<!-- svelte-ignore a11y-label-has-associated-control -->
			<label class="label">
				<span class="label-text">Decrypt Wallet</span>
			</label>
			<input
				type="text"
				bind:value={password}
				placeholder="|> password"
				class="input input-bordered w-full max-w-xs"
				required
			/>
			<label for="my_modal_7" class="btn btn-primary mt-10 border-8 rounded-XL">Next</label>
			<input type="checkbox" id="my_modal_7" class="modal-toggle" />
			<div class="modal">
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
							<span>Error! {deploymentErrorMsg} </span>
						</div>
					{:else if deploymentError === false}
						<div class="alert alert-success flex flex-col mt-10 mb-10">
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
					<button type="submit" class="btn btn-primary rounded-xl border-8 mt-5">DEPLOY</button>
				</div>
				<label class="modal-backdrop" for="my_modal_7">Close</label>
			</div>
		</div>
	</form>
</div>
