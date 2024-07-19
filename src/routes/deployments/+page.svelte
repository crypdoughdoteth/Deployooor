<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  $: name = "";
  $: depAddr = "";
  $: date = "";
  $: SCAddr = "";
  $: fee = "";
  $: verified = "";
</script>

<div class="table-container w-1/2 m-auto mt-24">
  <table class="table table-hover">
    <thead>
      <tr>
        <th>SC Name</th>
        <th>Deployer Address</th>
        <th>Deploy Date</th>
        <th>SC Address</th>
        <th>Network</th>
        <th>Fee</th>
        <th>Verified</th>
      </tr>
    </thead>
  </table>
</div>
