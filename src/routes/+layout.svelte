<script lang="ts">
	import '../app.pcss';
  import { DarkMode } from 'flowbite-svelte';

  import { Tabs, TabItem } from 'flowbite-svelte';
	import Settings from './Settings.svelte';
	import ModsTable from './ModsTable.svelte';

</script>
<DarkMode class="float-right mr-5" />
<Tabs style="underline">
  <TabItem open title="Mods">
    <ModsTable/>
  </TabItem>
  <TabItem title="Settings">
    <Settings/>
  </TabItem>
</Tabs>

<slot />
