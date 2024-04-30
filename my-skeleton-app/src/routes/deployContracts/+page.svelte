<script lang="ts">
  import { Stepper, Step, type ToastSettings } from "@skeletonlabs/skeleton";
  import { getToastStore } from "@skeletonlabs/skeleton";
  const handleStep = () => {
    console.log("step");
  };
</script>

<Stepper on:next={() => handleStep()} class="w-1/3 m-auto mt-24 rounded-sm">
  <Step class="rounded-lg bg-slate-950/20 p-6">
    <svelte:fragment slot="header">Key</svelte:fragment>
    <form class="w-1/2 m-auto mt-20 h-80">
      <label class="label">
        <span>Key To Use</span>
        <select class="select">
          <option value="1">placeholder</option>
        </select>
      </label>

      <label class="label mt-4">
        <span>Input Wallet Password</span>
        <input class="input" type="password" placeholder="password" />
      </label>
      <button class="btn m-auto block mt-2 bg-slate-950">Show address</button>
    </form>
  </Step>
  <Step class=" bg-slate-950/20 p-6 rounded-lg">
    <svelte:fragment slot="header">Contract</svelte:fragment>
    <form class="w-1/2 m-auto mt-20 h-80">
      <label class="label mt-4">
        <span>Contract Type</span>
        <select class="select">
          <option value="1">Vyper</option>
          <option value="2">Stylus</option>
          <option value="3">Solidity</option>
        </select>
      </label>

      <label class="label mt-4">
        <span>Path To Contract</span>
        <input class="input" type="file" multiple />
      </label>

      <label class="label mt-4">
        <span>EVM Version</span>
        <select class="select">
          <option value="1">Cancun</option>
          <option value="2">Shanghai</option>
          <option value="3">Berlin</option>
          <option value="3">Paris</option>
          <option value="3">London</option>
        </select>
      </label>
    </form>
  </Step>
  <Step class="rounded-lg bg-slate-950/20 p-6">
    <svelte:fragment slot="header">Arguments</svelte:fragment>

    <form class="w-1/2 m-auto mt-20 h-80">
      <button> </button>
      <label class="label mt-4">
        <span>Arguments</span>
        <input class="input mt-8" type="text" placeholder="arguments" />
      </label>
      <select class="select mt-10">
        <option value="1">placeholder</option>
      </select>
      <button class="btn m-auto block mt-2 bg-slate-950">Show args</button>
    </form>
  </Step>
</Stepper>
