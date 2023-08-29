<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { JsonRpcProvider, ethers } from 'ethers';

	type ReturnData = {
		abi: JSON;
		keystore: JSON;
		initcode: string;
	};
	type Config = {
		provider: string;
		keystore: string;
	};

	// I
	let contractFile: string;
	let password: string;
	let args: string;
	// O
	let res: ReturnData;
	let conf: Config;

	let configFound: boolean; 
	async function onSubmit(): Promise<void> {
		event?.preventDefault();
		// callback to Rust code to
		// I.   Compile Vyper file
		// II.  Update provider

		console.log('calling rust code ..... ');

		await invoke<Config>('get_config', {})
			.then((message) => {
				console.log(message);
				conf = message;
			})
			.catch((error) =>{ 
				configFound = false;
				console.error(error);
			});

			
		await invoke<ReturnData>('fetch_data', { path: contractFile, keyPath: conf.keystore })
			.then((message) => {
				console.log(message);
				res = message;
			})
			.catch((error) => console.error(error));
		// console.log(console.log("********", res));
		let ks = res.keystore;
		let bytecode = res.initcode;
		let keys = JSON.stringify(ks);
		const web3: JsonRpcProvider = new ethers.JsonRpcProvider(conf.provider);
		const decryptedWallet = (await ethers.Wallet.fromEncryptedJson(keys, password)).connect(web3);
		const abi = new ethers.Interface(JSON.stringify(res.abi));
		const contract = new ethers.ContractFactory(abi, { object: bytecode }, decryptedWallet);
		let tx = await contract.deploy([args]);
		console.log(await tx.getAddress());
		await tx.waitForDeployment();
		configFound = true; 
	}
</script>

<div class="navbar rounded-xl place-content-center mt-5">
	<a href="./" class="btn btn-ghost normal-case text-xl ">Home</a>
    <a href="/settings" class="btn btn-ghost normal-case text-xl">Settings</a>
</div>
<div class="flex flex-col justify-center items-center h-screen min-h-screen">
	<form on:submit={() => onSubmit()}>
		<div class="form-control w-full max-w-xs mb-5">
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
						<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
						<span>Error! Config Not Found.</span>
					  </div>
					{/if}
					<button type="submit" class="btn btn-primary rounded-xl border-8 mt-5">DEPLOY</button>
				</div>
				<label class="modal-backdrop" for="my_modal_7">Close</label>
			</div>
		</div>
	</form>
</div>
