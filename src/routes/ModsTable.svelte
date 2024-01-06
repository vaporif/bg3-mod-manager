<script lang="ts">
  import { Button } from 'flowbite-svelte';
  import { Dropzone } from 'flowbite-svelte';
  import {
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from 'flowbite-svelte';

  import { ArrowUpSolid, ArrowDownSolid } from 'flowbite-svelte-icons';
  import { slide } from 'svelte/transition';

  let items = [
    {
        "author": "Eldric Stormblade",
        "name": "Sorcerer's Grimoire",
        "folder": "ArcaneArtifacts",
        "version": null,
        "description": "Unleash the power of ancient spells with this mystical grimoire.",
        "uuid": "c9e5a97a-62ed-4f62-bf4a-8c94dbd6c28d",
        "created": "2023-10-15T08:30:45.1234567-06:00",
        "dependencies": [],
        "group": "57bacf0b-9ab7-4cd9-b7ed-40050ffa41df",
        "md5": "a1b2c3d4e5f6g7h8i9j0"
    },
    {
        "author": "Liliana Shadowdancer",
        "name": "Shadow's Veil",
        "folder": "StealthEssentials",
        "version": null,
        "description": "Become one with the shadows and vanish from sight with this stealth mod.",
        "uuid": "d8f7e6b5-a4c3-4f2e-9d1c-0b9a8f7e6d5c",
        "created": "2023-11-20T14:45:30.9876543-06:00",
        "dependencies": [],
        "group": "57bacf0b-9ab7-4cd9-b7ed-40050ffa41df",
        "md5": "b2a3c4d5e6f7g8h9i0j1k2l3"
    },
    {
        "author": "Dwarven Smithy",
        "name": "Forgemaster's Arsenal",
        "folder": "CraftingExpansions",
        "version": null,
        "description": "Craft legendary weapons and armor with the secrets of the dwarven forgemasters.",
        "uuid": "f1e2d3c4-b5a6-4e7d-8c9b-0a1b2c3d4e5f",
        "created": "2023-12-25T18:15:10.3456789-06:00",
        "dependencies": [],
        "group": "57bacf0b-9ab7-4cd9-b7ed-40050ffa41df",
        "md5": "c3d4e5f6g7h8i9j0k1l2m3"
    },
    {
        "author": "Elowen Moonshadow",
        "name": "Forest Guardian's Blessing",
        "folder": "NatureEnhancements",
        "version": null,
        "description": "Harness the power of the ancient forest with this mystical blessing.",
        "uuid": "e4f5d6c7-b8a9-4d0e-1c2b-3a4b5c6d7e8f",
        "created": "2023-08-05T10:20:35.5678901-06:00",
        "dependencies": [],
        "group": "57bacf0b-9ab7-4cd9-b7ed-40050ffa41df",
        "md5": "d4e5f6g7h8i9j0k1l2m3n4"
    },
    {
        "author": "Ragnar Thunderbeard",
        "name": "Dwarven Stronghold",
        "folder": "EpicFortresses",
        "version": null,
        "description": "Build a mighty dwarven stronghold and defend it from invaders.",
        "uuid": "1a2b3c4d-5e6f-7a8b-9c0d-1e2f3a4b5c6d",
        "created": "2023-07-10T12:55:20.2345678-06:00",
        "dependencies": [],
        "group": "57bacf0b-9ab7-4cd9-b7ed-40050ffa41df",
        "md5": "e5f6g7h8i9j0k1l2m3n4o5"
    }
];

  let openRow: number | null = null;

  const toggleRow = (i: number) => {
    openRow = openRow === i ? null : i
  }

  const changeOrder = (i: number, direction: "up" | 'down') => {
   switch(direction) {
      case 'down':
        [items[i + 1], items[i]] = [items[i], items[i + 1]];
         break;
      case 'up': 
        [items[i - 1], items[i]] = [items[i], items[i - 1]];
         break;
    } 

    items = items;
  }

</script>

{#if items.length == 0}
<Dropzone
  on:dragover={(event) => {
    event.preventDefault();
  }}>
  <svg aria-hidden="true" class="mb-3 w-10 h-10 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" /></svg>
  <p class="mb-2 text-sm text-gray-500 dark:text-gray-400"><span class="font-semibold">Click to </span> or drag and drop</p>
  <p class="text-xs text-gray-500 dark:text-gray-400">zip file</p>
</Dropzone>
{:else}
<Table>
    <TableHead>
      <TableHeadCell>Name</TableHeadCell>
      <TableHeadCell>
        <span class="sr-only">Position</span>
      </TableHeadCell>
    </TableHead>
    <TableBody class="divide-y">
      {#each items as item, i (item.uuid)}
        <TableBodyRow on:click="{() => toggleRow(i)}"> 
          <TableBodyCell>{item.name}</TableBodyCell>
          <TableBodyCell>
            {#if i != items.length - 1}
              <Button on:click={(e) => {e.stopPropagation(); changeOrder(i, 'down');} }>
                <ArrowDownSolid size="xs"/>
              </Button>
            {/if}
              
            {#if i != 0}
              <Button on:click={(e) => {e.stopPropagation(); changeOrder(i, 'up');}}>
                <ArrowUpSolid size="xs"/>
              </Button>
            {/if}
          </TableBodyCell>
        </TableBodyRow>
        {#if openRow === i}
          <TableBodyRow>
            <TableBodyCell colspan="2" class="p-0">
              <div class="px-2 py-3" transition:slide={{ duration: 300, axis: 'y' }}>
                <div class="overflow-hidden shadow border">
                    <div class="border-t border-gray-200 px-4 py-5 sm:p-0">
                        <dl class="sm:divide-y sm:divide-gray-200">
                            <div class="py-3 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                <dt class="text-sm font-medium text-gray-500">
                                    Author
                                </dt>
                                <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                                    {item.author}
                                </dd>
                            </div>
                            <div class="py-3 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                <dt class="text-sm font-medium text-gray-500">
                                    Version
                                </dt>
                                <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                                    {item.version || 'None'}
                                </dd>
                            </div>
                            <div class="py-3 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                <dt class="text-sm font-medium text-gray-500">
                                    Dependencies
                                </dt>
                                <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                                    []
                                </dd>
                            </div>
                            <div class="py-3 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                <dt class="text-sm font-medium text-gray-500">
                                    Description
                                </dt>
                                <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                                    <p>
                                      {item.description}
                                    </p>
                                </dd>
                            </div>
                        </dl>
                    </div>
                </div>
              </div>
            </TableBodyCell>
          </TableBodyRow>
        {/if}
      {/each}
    </TableBody>
  </Table>
{/if}
