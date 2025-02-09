<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
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
        const icon = document.getElementById('icon');
        if (icon) {
          icon.innerHTML = `
          <circle cx="12" cy="12" r="10" class="fill-blue-500"/>
          <path d="M8 12L10 14L16 8" class="stroke-white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        `;
        }
        annotation_stats.set(JSON.parse(jsonData));
    } catch (error) {
      console.error("Error processing file:", error);
      const icon = document.getElementById('icon');
      if (icon) {
        icon.innerHTML = `
          <circle cx="12" cy="12" r="10" class="fill-red-500"/>
          <path d="M8 8L16 16M16 8L8 16" class="stroke-black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        `;
      }
    }
  }

  function openGoaStatsWindow() {
    // Call the Tauri API to open a new window
    invoke('open_stats_window');
  }
</script>

<div class="p-5">
  <h1 class="text-gray-600 dark:text-gray-300 text-5xl">Setup</h1>
</div>
<div class="card shadow-sm">
  <h2 class="text-lg">GO Annotations</h2>
  <div class="flex items-center space-x-3 mt-3">
    <button on:click={openFileDialog} class="cursor-pointer text-blue-500 hover:text-blue-600">Load GOA File</button>
   <!-- Icon with two states (Red Circle with Black X / Blue Checkmark) -->
   <button id="icon-toggle" class="p-2 rounded-full bg-transparent hover:bg-transparent">
    <svg id="icon" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
      <!-- Default icon: Red Circle with Black X -->
      <circle cx="12" cy="12" r="5" class="fill-red-500"/>
      <path d="M7 7L17 17M17 7L7 17" class="stroke-black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
  </button>

    <!-- Button to open a new component -->
    <button class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-100">
      Show GOA statistics
    </button>
  </div>
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


