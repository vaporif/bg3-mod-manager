<script lang="ts">
	import { Button } from 'flowbite-svelte';
	import { message } from '@tauri-apps/api/dialog';
	import {
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell
	} from 'flowbite-svelte';

	import { ArrowUpSolid, ArrowDownSolid } from 'flowbite-svelte-icons';
	import { slide } from 'svelte/transition';
	import FileDrop from './FileDrop.svelte';

	import { mods } from '$lib';

	$: items = $mods;

	let openRow: number | null = null;

	const toggleRow = (i: number) => {
		openRow = openRow === i ? null : i;
	};

	const changeOrder = (i: number, direction: 'up' | 'down') => {
		switch (direction) {
			case 'down':
				[items[i + 1], items[i]] = [items[i], items[i + 1]];
				break;
			case 'up':
				[items[i - 1], items[i]] = [items[i], items[i - 1]];
				break;
		}

		items = items;
	};

	const onDrop = async (filePaths: string[]) => {
		const dropResult = await mods.addModFiles(filePaths);
		for(const mod of dropResult.valid_files) {
			if(items.find((f) => f.uuid == mod.uuid)) {
				await message(`Mod already added`, "Mod issue");
				continue;
			}
			items.push(mod);
			items = items;
		}

		if(!!dropResult.invalid_files.length) {
			await message(`Some of the files were not included due to error`, { title: 'Mods invalid', type: 'error' });
		}
	};
</script>

<FileDrop {onDrop}>
	{#if items.length == 0}
		<button
			class="dark:hover:bg-bray-800 flex h-64 w-full cursor-pointer flex-col item	s-center justify-center rounded-lg border-2 border-dashed border-gray-300 bg-gray-50 hover:bg-gray-100 dark:border-gray-600 dark:bg-gray-700 dark:hover:border-gray-500 dark:hover:bg-gray-600"
		>
			<div class="flex flex-col items-center" tabIndex="0">
				<svg
					aria-hidden="true"
					class="mb-3 h-10 w-10 text-gray-400"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
					xmlns="http://www.w3.org/2000/svg"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
					/></svg
				>
				<p class="mb-2 text-sm text-gray-500 dark:text-gray-400">
					<span class="font-semibold">Drag and drop</span>
				</p>
				<p class="text-xs text-gray-500 dark:text-gray-400">zip file</p>
			</div>
		</button>
	{:else}
		<Table>
			<TableHead>
				<TableHeadCell>Name</TableHeadCell>
				<TableHeadCell>
					<span class="sr-only">Position</span>
				</TableHeadCell>
			</TableHead>
			<TableBody>
				{#each items as item, i (item.uuid)}
					<TableBodyRow on:click={() => toggleRow(i)}>
						<TableBodyCell>{item.name}</TableBodyCell>
						<TableBodyCell>
							{#if i != items.length - 1}
								<Button
									on:click={(e) => {
										e.stopPropagation();
										changeOrder(i, 'down');
									}}
								>
									<ArrowDownSolid size="xs" />
								</Button>
							{/if}

							{#if i != 0}
								<Button
									on:click={(e) => {
										e.stopPropagation();
										changeOrder(i, 'up');
									}}
								>
									<ArrowUpSolid size="xs" />
								</Button>
							{/if}
						</TableBodyCell>
					</TableBodyRow>
					{#if openRow === i}
						<TableBodyRow>
							<TableBodyCell colspan="2" class="p-0">
								<div class="px-2 py-3" transition:slide={{ duration: 300, axis: 'y' }}>
									<div class="overflow-hidden border shadow">
										<div class="border-t border-gray-200 px-4 py-5 sm:p-0">
											<dl class="sm:divide-y sm:divide-gray-200">
												<div class="py-3 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6 sm:py-5">
													<dt class="text-sm font-medium text-gray-500">Author</dt>
													<dd class="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0">
														{item.author}
													</dd>
												</div>
												<div class="py-3 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6 sm:py-5">
													<dt class="text-sm font-medium text-gray-500">Version</dt>
													<dd class="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0">
														{item.version || 'None'}
													</dd>
												</div>
												<div class="py-3 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6 sm:py-5">
													<dt class="text-sm font-medium text-gray-500">Dependencies</dt>
													<dd class="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0">[]</dd>
												</div>
												<div class="py-3 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6 sm:py-5">
													<dt class="text-sm font-medium text-gray-500">Description</dt>
													<dd class="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0">
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
</FileDrop>
