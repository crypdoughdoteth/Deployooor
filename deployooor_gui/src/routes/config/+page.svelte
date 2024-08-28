<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { type NetworkSettings } from "../../DeploymentTypes";

  $: providerURL = "";
  $: etherscanAPIKey = null;
  $: name_provider = "";

  const handleConfig = () => {
    const networkSettings: NetworkSettings = {
      name: name_provider,
      provider: providerURL,
      etherscan_api: etherscanAPIKey,
    };
    invoke("set_config", {
      networkSettings,
    }).catch((e) => {
      console.error(e);
    });
  };
</script>

<form class="w-1/3 m-auto mt-24">
  <label class="label">
    <span>Name</span>
    <input class="input" type="text" bind:value={name_provider} />
  </label>
  <label class="label">
    <span>Provider URL</span>
    <input class="input" type="text" bind:value={providerURL} />
  </label>
  <label class="label mt-4">
    <span>Etherscan API Key</span>
    <input class="input" type="text" bind:value={etherscanAPIKey} />
  </label>
  <button
    on:click={() => handleConfig()}
    type="button"
    class="btn rounded-lg bg-slate-950 mt-4">Set Config</button
  >
</form>
