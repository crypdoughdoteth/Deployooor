<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { type NetworkSettings } from "../../DeploymentTypes";
  import { stringify } from "querystring";

  $: providerURL = "";
  $: etherscanAPIKey = "";
  $: _name = "";
  const handleConfig = () => {
    const NetworkSettings: NetworkSettings = {
      name: _name,
      provider: providerURL,
      etherscan_api: etherscanAPIKey,
    };
    invoke("set_config", {
      NetworkSettings,
    });
  };
</script>

<form class="w-1/3 m-auto mt-24">
  <label class="label">
    <label class="label">
      <span>Name</span>
      <input class="input" type="text" bind:value={_name} />
    </label>
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
