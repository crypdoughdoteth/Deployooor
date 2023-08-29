<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { JsonRpcProvider, ethers } from 'ethers';

	type ReturnData = {
		abi: JSON;
		keystore: JSON;
		bytecode: string;
	};
	// I

	let contractFile: string;
	let provider: string;
	let keystore: string;
	let password: string;
	let args: ethers.ContractMethodArgs<any[]>;
	// O
	let res: ReturnData;

	async function onSubmit(): Promise<void> {
		// callback to Rust code to
		// I.   Compile Vyper file
		// II.  Update provider
		// III. Decrypt keystore
		// IV.  Deploy contract
    console.log("calling rust code ..... ")
		invoke<ReturnData>('fetch_data', { path: contractFile, key_path: keystore })
			.then((message) => (res = message))
			.catch((error) => console.error(error));
		let ks = res.keystore;
		let keys = JSON.stringify(ks);
		const decryptedWallet = await ethers.Wallet.fromEncryptedJson(keys, password);
		const abi = new ethers.Interface(JSON.stringify(res.abi));
		const web3: JsonRpcProvider = new ethers.JsonRpcProvider(provider);
		const contract = new ethers.ContractFactory(abi, res.bytecode, decryptedWallet.connect(web3));
		let tx = await contract.deploy(args);
    console.log(await tx.getAddress());
    await tx.waitForDeployment();
	}
</script>

<div class="navbar rounded-xl">
	<a href="./" class="btn btn-ghost normal-case text-xl">Home</a>
</div>
<div class="flex flex-col justify-center items-center h-screen min-h-screen">
	<form on:submit|preventDefault={() => onSubmit()}>
		<div class="form-control w-full max-w-xs">
			<!-- svelte-ignore a11y-label-has-associated-control -->
			<label class="label">
				<span class="label-text">Vyper Contract</span>
			</label>
			<input
				type="file"
				bind:value={contractFile}
				class="file-input file-input-bordered w-full max-w-xs mb-5"
				required
			/>
		</div>

		<div class="form-control w-full max-w-xs">
			<!-- svelte-ignore a11y-label-has-associated-control -->
			<label class="label">
				<span class="label-text">Keystore</span>
			</label>
			<input
				type="file"
				bind:value={keystore}
				class="file-input file-input-bordered w-full max-w-xs mb-5"
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
				placeholder="|> constructor args"
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
				placeholder="|> url of provider"
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
