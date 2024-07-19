<script lang="ts">
  import { Stepper, Step, type ToastSettings } from "@skeletonlabs/skeleton";
  import { getToastStore } from "@skeletonlabs/skeleton";
  import { getModalStore } from "@skeletonlabs/skeleton";
  const handleStep = () => {
    console.log("step");
  };
  const modalStore = getModalStore();
  const formData = {
    data: "placeholder",
    type: "placeholder",
  };

  const modal: ModalSettings = {
    type: "prompt",
    // Data
    title: "Enter Arguments",
    body: "Provide your Arguments",
    // Populates the input value and attributes
    value: "Value",
    valueAttr: { type: "text", minlength: 3, maxlength: 10, required: true },
    // Returns the updated response value
    response: (r: string) => console.log("response:", r),
  };
</script>

<Stepper on:next={() => handleStep()} class="w-1/3 m-auto mt-24 rounded-sm">
  <Step class="rounded-lg bg-slate-950/20 p-6">
    <svelte:fragment slot="header">Key</svelte:fragment>
    <form class="w-1/2 m-auto mt-20 h-80">
      <label class="label">
        <span class="ml-[7px]">Key To Use</span>
        <select class="select">
          <option value="1">placeholder</option>
        </select>
      </label>

      <label class="label mt-4">
        <span class="ml-[7px]">Input Wallet Password</span>
        <input class="input" type="password" placeholder="password" />
      </label>
      <!-- <button class="btn m-auto block mt-2 bg-slate-950">Show address</button> -->
      <ul class="list mt-4 h-30 overflow-hidden">
        <li>
          <span class="ml-[7px]">(blockies icon)</span>
          <span class="flex-auto ml-[7px]">Map Me</span>
        </li>
        <!-- ... -->
      </ul>
    </form>
  </Step>
  <Step class=" bg-slate-950/20 p-6 rounded-lg">
    <svelte:fragment slot="header">Contract</svelte:fragment>
    <form class="w-1/2 m-auto mt-20 h-80">
      <label class="label mt-4">
        <span class="ml-[7px]">Contract Type</span>
        <select class="select">
          <option value="1">Vyper</option>
          <option value="2">Stylus</option>
          <option value="3">Solidity</option>
        </select>
      </label>

      <label class="label mt-4">
        <span class="ml-[7px]">Path To Contract</span>
        <input class="input" type="file" multiple />
      </label>

      <label class="label mt-4">
        <span class="ml-[7px]">EVM Version</span>
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

    <form
      class="w-1/2 m-auto mt-20 h-80 flex flex-col align-center justify-around"
    >
      <!-- <label class="label mt-4"> -->
      <button
        on:click={() => modalStore.trigger(modal)}
        class="mx-auto p-6 block"
      >
        <i class="fa-solid fa-plus text-4xl"></i>
      </button>
      <!-- <label class="label mt-4 flex flex-col">
        <span class="w-fit mx-auto">Arguments</span>
        <select class="select">
          <option value="1">mapped args</option>
        </select>
      </label> -->
      <p class="ml-[7px]">Placeholder for mapped args</p>
      <!-- <span>Arguments</span>
        <input class="input mt-8" type="text" placeholder="arguments" />
      </label>
      <select class="select mt-10">
        <option value="1">placeholder</option>
      </select>
       -->
      <!-- </label> -->
    </form>
  </Step>
</Stepper>
