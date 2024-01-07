<script lang="ts">
	import '../app.pcss';
	import { subscribeToTauriEvents } from '$lib';

	import { DarkMode } from 'flowbite-svelte';
	import { Tabs, TabItem } from 'flowbite-svelte';
	import Settings from './Settings.svelte';
	import ModsTable from './ModsTable.svelte';
	import { onDestroy } from 'svelte';

	console.log('initted');

	const unsubscribePromise = subscribeToTauriEvents();

	onDestroy(async () => {
		let unsubscribe = await unsubscribePromise;
		unsubscribe();
	});
</script>

<DarkMode class="float-right mr-5" />
<Tabs style="underline">
	<TabItem open title="Mods">
		<ModsTable />
	</TabItem>
	<TabItem title="Settings">
		<Settings />
	</TabItem>
</Tabs>

<slot />
