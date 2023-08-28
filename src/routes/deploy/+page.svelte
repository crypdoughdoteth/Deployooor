<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri';

    type Args = {
        ty: Types, 
        value: String
    }
    enum Types {
        U256,
        U128, 
        U64,
        U8,
        I256,
        Address,
        Bytes, 
        Bool
    }

    let contractFile: File;
    let provider: URL;
    let keystore: File;
    let password: String;
    let args: Array<Args>;
    function handleSubmit() : void {
        // callback to Rust code to 
        // I.   Compile Vyper file
        // II.  Update provider
        // III. Decrypt keystore  
        // IV.  Deploy contract
        invoke('deploy', {provider_url: provider, password: password, path: contractFile, key_path: keystore})
    }
</script>

<div class="navbar rounded-xl">
	<a href="./" class="btn btn-ghost normal-case text-xl">Home</a>
</div>
<div class="flex flex-col justify-center items-center h-screen min-h-screen">
    <div class="form-control w-full max-w-xs">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="label">
          <span class="label-text">Vyper Contract</span>
        </label>
        <input type="file" bind:value={contractFile} class="file-input file-input-bordered w-full max-w-xs mb-5" />

      </div>
    
      <div class="form-control w-full max-w-xs">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="label">
          <span class="label-text">Keystore</span>
        </label>
        <input type="file" bind:value={keystore} class="file-input file-input-bordered w-full max-w-xs mb-5" />

      </div>
    <div class="form-control w-full max-w-xs mb-5">
		<!-- svelte-ignore a11y-label-has-associated-control -->
		<label class="label">
			<span class="label-text">JSON-RPC Provider</span>
		</label>
		<input type="text" bind:value={provider} placeholder="|> url of provider" class="input input-bordered w-full max-w-xs" />
	</div>
    <div class="form-control w-full max-w-xs mb-5">
		<!-- svelte-ignore a11y-label-has-associated-control -->
		<label class="label">
			<span class="label-text">Decrypt Wallet</span>
		</label>
		<input type="text" bind:value={password} placeholder="|> password" class="input input-bordered w-full max-w-xs" />
	</div>
    <button class="btn btn-primary justify-center rounded-xl border-8 mt-5" on:click={handleSubmit}>DEPLOY</button>
</div>
