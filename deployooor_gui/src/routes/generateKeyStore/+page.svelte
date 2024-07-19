<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  $: name = "";
  $: password = "";
  const handleKeyGen = () => {
    invoke("create_key", {
      path: "./",
      nickname: name,
      password: password,
    });
  };
</script>

<form class="w-1/3 m-auto mt-24">
  <label class="label mt-4">
    <span>Name</span>
    <input bind:value={name} class="input" type="text" />
  </label>
  <label class="label mt-4">
    <span>Password</span>
    <input bind:value={password} class="input" type="password" />
  </label>

  <button
    on:click={() => handleKeyGen()}
    type="button"
    class="btn rounded-lg bg-slate-950 mt-4">Submit</button
  >
</form>
