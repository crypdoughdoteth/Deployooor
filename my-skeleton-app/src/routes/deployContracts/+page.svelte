<script lang="ts">
</script>

<form class="w-1/3 m-auto mt-24">
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

  <p class="mt-4">Place holder for please fill form to see addr</p>

  <label class="label mt-4">
    <span>Arguments</span>
    <input class="input" type="text" placeholder="arguments" />
    <select class="select">
      <option value="1">placeholder</option>
    </select>
  </label>

  <p class="mt-4">Place holder for list all arguments</p>

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

  <p class="mt-4">placeholder for complete form to see gas</p>
  <div class="flex justify-between">
    <button type="button" class="btn rounded-lg bg-slate-950 mt-4"
      >Estimate Gas</button
    >
    <button type="button" class="bg-slate-50 btn rounded-lg text-slate-950 mt-4"
      >Deploy Contract</button
    >
  </div>
</form>
