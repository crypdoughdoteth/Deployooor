<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  $: providerURL = "";
  $: etherscanAPIKey = "";
  // const handleConfig = () => {
  //   invoke("set_config", {
  //     providerURL: providerURL,
  //     etherscanAPIKey: etherscanAPIKey,
  //   });
  // };
</script>

<form class="w-1/3 m-auto mt-24">
  <label class="label">
    <span>Provider URL</span>
    <input class="input" type="text" />
  </label>
  <label class="label mt-4">
    <span>Etherscan API Key</span>
    <input class="input" type="text" />
  </label>
  <button type="button" class="btn rounded-lg bg-slate-950 mt-4"
    >Set Config</button
  >
</form>
