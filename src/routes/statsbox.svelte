<script>
    export let jsonData = []; // Pass the JSON data as a prop
  
    // You can fetch the data here if needed, or pass it as a prop from the parent window
  </script>
  
  <table class="min-w-full table-auto">
    <thead>
      <tr>
        <th class="px-4 py-2">Item</th>
        <th class="px-4 py-2">Value</th>
      </tr>
    </thead>
    <tbody>
      {#each jsonData as row}
        <tr>
          <td class="border px-4 py-2">{row.key}</td>
          <td class="border px-4 py-2">{row.value}</td>
        </tr>
      {/each}
    </tbody>
  </table>