<script lang="ts">
	import '../app.pcss';
  import { DarkMode, Navbar } from 'flowbite-svelte';

  import { Tabs, TabItem } from 'flowbite-svelte';
	import Settings from './Settings.svelte';
	import ModsTable from './ModsTable.svelte';

</script>
  <div> 
    <DarkMode class="float-right mr-5" />
<!--   <div class="float-right"> -->
<!--     <DarkMode /> -->
<!--   </div> -->
<!-- </Navbar> -->
<Tabs style="underline">
<!-- <Navbar> -->
  <TabItem open title="Mods">
    <ModsTable/>
  </TabItem>
  <TabItem title="Settings">
    <Settings/>
  </TabItem>
</Tabs>
</div>

<slot />
