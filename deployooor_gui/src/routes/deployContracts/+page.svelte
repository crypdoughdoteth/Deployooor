<script lang="ts">
  import { Stepper, Step, type ToastSettings } from "@skeletonlabs/skeleton";
  import { getToastStore } from "@skeletonlabs/skeleton";
  import { getModalStore } from "@skeletonlabs/skeleton";
  import { invoke } from "@tauri-apps/api/tauri";
  import { type ModalSettings } from "@skeletonlabs/skeleton";
  import { onMount } from "svelte";
  import { type ContractDeployment } from "../../DeploymentTypes";

  $: keyList = [];
  $: keyToUse = "";
  $: walletPassword = "";
  $: contractType = "";
  $: pathToContract = "";
  $: evmVersion = "";
  $: arguements = [];

  $: privateKey = "";
  //get key by name
  onMount(async () => {
    keyList = await invoke("list_keys");
    console.log(keyList);
  });

  const ContractDeployment: ContractDeployment = {
    provider: keyToUse,
    args: arguements,
    path: pathToContract,
    private_key: privateKey,
  };
  // console.log(invoke("get_config"));
  function onCompleteHandler(e: Event): void {
    e.preventDefault();
    invoke("deploy_contract", { ContractDeployment }).catch((e) => {
      console.error(e);
    });
  }
  const handleStep = (e: any) => {
    if (e.detail.step === 0) {
      invoke("get_key_by_name", {
        name: keyToUse,
        password: walletPassword,
      }).then((r) => {
        privateKey = r.pk;
      });
    }
    console.log(privateKey);
  };
  const modalStore = getModalStore();
  const formData = {
    data: "value",
    type: "placeholder",
  };

  const modal: ModalSettings = {
    type: "prompt",
    title: "Enter Arguments",
    body: "Provide your Arguments",
    value: "",
    valueAttr: { type: "text", minlength: 3, maxlength: 100, required: true },
    response: (r: string) => (arguements = r.split(" ")),
  };
</script>

<Stepper
  on:next={(e) => handleStep(e)}
  on:complete={onCompleteHandler}
  class="w-2/3 m-auto mt-24 rounded-sm"
>
  <Step class="rounded-lg bg-slate-950/20 p-6">
    <svelte:fragment slot="header">Key</svelte:fragment>
    <form class="m-auto mt-20 h-80">
      <label class="label">
        <span class="ml-[12px]">Key To Use</span>
        <select class="select" bind:value={keyToUse}>
          {#each keyList as key}
            <option value={key?.name}>{key?.name}</option>
          {/each}
        </select>
      </label>

      <label class="label mt-4">
        <span class="ml-[12px]">Input Wallet Password</span>
        <input
          class="input"
          type="password"
          placeholder="password"
          bind:value={walletPassword}
        />
      </label>
      <!-- <button class="btn m-auto block mt-2 bg-slate-950">Show address</button> -->
      <!-- <ul class="list mt-4 h-30 overflow-hidden">
        <li>
          <span class="ml-[12px]">(blockies icon)</span>
          <span class="flex-auto ml-[12px]">Map Me</span>
        </li>
      </ul> -->
    </form>
  </Step>
  <Step class=" bg-slate-950/20 p-6 rounded-lg">
    <svelte:fragment slot="header">Contract</svelte:fragment>
    <form class="m-auto mt-20 h-80">
      <label class="label mt-4">
        <span class="ml-[12px]">Contract Type</span>
        <select class="select" bind:value={contractType}>
          <option value="1">Vyper</option>
          <option value="2">Stylus</option>
          <option value="3">Solidity</option>
        </select>
      </label>

      <label class="label mt-4">
        <span class="ml-[12px]">Path To Contract</span>
        <input class="input" type="file" multiple bind:value={pathToContract} />
      </label>

      <label class="label mt-4">
        <span class="ml-[12px]">EVM Version</span>
        <select class="select" bind:value={evmVersion}>
          <option value="1">Cancun</option>
          <option value="2">Shanghai</option>
          <option value="3">Berlin</option>
          <option value="4">Paris</option>
          <option value="5">London</option>
        </select>
      </label>
    </form>
  </Step>
  <Step class="rounded-lg bg-slate-950/20 p-6">
    <svelte:fragment slot="header">Arguments</svelte:fragment>

    <form class="m-auto mt-20 h-80 flex flex-col align-center justify-around">
      <!-- <label class="label mt-4"> -->
      <button
        on:click={() => modalStore.trigger(modal)}
        class="mx-auto p-6 block"
      >
        <i class="fa-solid fa-plus text-4xl"></i>
      </button>
      <p class="ml-[12px]">Placeholder for mapped args</p>
    </form>
  </Step>
</Stepper>
