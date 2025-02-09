<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { open } from "@tauri-apps/api/dialog";
  import { writable } from "svelte/store";

  let filePath = writable<string | null>(null);
  let annotation_stats = writable<{ id: string; name: string; ontology: string }[]>([]);

  async function openFileDialog() {
    const selected = await open({
      multiple: false,
      filters: [{ name: "GO Annotation File", extensions: ["gaf"] }]
    });

    if (selected) {
      filePath.set(selected as string);
      processFile(selected as string);
    }
  }

  async function processFile(path: string) {
    try {
      const jsonData = await invoke<string>("process_file", { path });
        annotation_stats.set(JSON.parse(jsonData));
    } catch (error) {
      console.error("Error processing file:", error);
    }
  }
</script>


<div class="setupcard"></div>

<div class="p-5">
  <h1 class="text-gray-600 dark:text-gray-300 text-5xl">Setup</h1>
</div>
<button on:click={openFileDialog} class="text-white">Open File</button>

{#if $filePath}
  <p class="text-white">Selected file: {$filePath}</p>
  <div class="overflow-x-auto">
  <table class="min-w-full border border-gray-300 shadow-lg rounded-lg">
    <thead class="bg-gray-800 text-white uppercase text-sm">
      <tr>
        <th class="px-6 py-3 text-left">Item</th>
        <th class="px-6 py-3 text-left">Value</th>
      </tr>
    </thead>
    <tbody>
      {#each $annotation_stats as item }
      <tr class="border-b border-gray-300 odd:bg-gray-100 even:bg-white hover:bg-gray-200 transition">
        <td class="px-6 py-3 text-left">{item.key}</td>
        <td class="px-6 py-3 text-left">{item.value}</td>
      </tr>
    {/each}
  </tbody>
</table>
</div>

{/if}


