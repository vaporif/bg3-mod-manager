<script lang="ts">
	import { type InstalledMod } from '$lib/ipc';
	import { TableBodyCell, TableBodyRow } from 'flowbite-svelte';
	import { slide } from 'svelte/transition';

	export let item: InstalledMod | null = null;
</script>

<TableBodyRow>
	<TableBodyCell colspan="2" class="p-0">
		<div class="px-2 py-3" transition:slide={{ duration: 300, axis: 'y' }}>
			<div class="overflow-hidden border shadow">
				<div class="border-t border-gray-200 px-4 py-5 sm:p-0">
					<dl class="sm:divide-y sm:divide-gray-200">
						<div class="py-3 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6 sm:py-5">
							<dt class="text-sm font-medium text-gray-500">Author</dt>
							<dd class="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0">
								{item?.author}
							</dd>
						</div>
						<div class="py-3 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6 sm:py-5">
							<dt class="text-sm font-medium text-gray-500">Version</dt>
							<dd class="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0">
								{item?.version || 'None'}
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
									{item?.description}
								</p>
							</dd>
						</div>
					</dl>
				</div>
			</div>
		</div>
	</TableBodyCell>
</TableBodyRow>
