<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { JsonRpcProvider, ethers } from 'ethers';

	type ReturnData = {
		abi: JSON;
		keystore: JSON;
		initcode: string;
	};
	// I
	let contractFile: string;
	let provider: string;
	let keystore: string;
	let password: string;
	let args: string;
	// O
	let res: ReturnData;

	async function onSubmit(): Promise<void> {
		event?.preventDefault();
		// callback to Rust code to
		// I.   Compile Vyper file
		// II.  Update provider
		// III. Decrypt keystore
		// IV.  Deploy contract
		console.log(console.log("--------", contractFile)); 
		console.log(console.log("--------", provider)); 
		console.log(console.log("--------", keystore)); 
		console.log(console.log("--------", password)); 
		console.log(console.log("--------", args)); 
		console.log('calling rust code ..... ');
		await invoke<ReturnData>('fetch_data', { path: contractFile, keyPath: keystore })
			.then((message) => {
				console.log(message);
				res = message
			})
			.catch((error) => console.error(error));
		// console.log(console.log("********", res)); 
		let ks = res.keystore;
		let bytecode = res.initcode;
		let keys = JSON.stringify(ks);
		const web3: JsonRpcProvider = new ethers.JsonRpcProvider(provider);
		const decryptedWallet = (await ethers.Wallet.fromEncryptedJson(keys, password)).connect(web3);
		const abi = new ethers.Interface(JSON.stringify(res.abi));
		const contract = new ethers.ContractFactory(abi, {object: bytecode}, decryptedWallet); 
		let tx = await contract.deploy([args]);
		console.log(await tx.getAddress());
		await tx.waitForDeployment();
	}
</script>

<div class="navbar rounded-xl">
	<a href="./" class="btn btn-ghost normal-case text-xl">Home</a>
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
				<span class="label-text">Keystore Path</span>
			</label>
			<input
				type="text"
				bind:value={keystore}
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
				<span class="label-text">JSON-RPC Provider</span>
			</label>
			<input
				type="text"
				bind:value={provider}
				placeholder="|> url"
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
			<button type="submit" class="btn btn-primary justify-center rounded-xl border-8 mt-5"
				>DEPLOY</button
			>
		</div>
	</form>
</div>
