<script lang="ts">
	import '../app.pcss';

	import { onDestroy } from 'svelte';
	import { DarkMode } from 'flowbite-svelte';
	import { Tabs, TabItem } from 'flowbite-svelte';

    import { subscribeToTauriEvents } from '$lib';
	import Settings from './Settings.svelte';
	import ModsTable from './ModsTable.svelte';

	const unsubscribePromise = subscribeToTauriEvents();

	onDestroy(async () => {
		const unsubscribe = await unsubscribePromise;
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
