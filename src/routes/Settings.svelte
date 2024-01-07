<script lang="ts">
  import { Input, Label, Button, ButtonGroup, InputAddon } from 'flowbite-svelte';
  import { FolderSolid } from 'flowbite-svelte-icons';
  import { open } from '@tauri-apps/api/dialog';

  import { settings } from '$lib';
	import { onDestroy } from 'svelte';

  let gameDataPath = "";

  async function selectDirectory(): Promise<string | null> {
    const selectedDir = await open({
      directory: true,
      multiple: false,
      // defaultPath: gameDataPath
    });

    return selectedDir as string;
  }

  const setGamePath = async () => {
    const dir = await selectDirectory();
    if (dir) {
      gameDataPath = dir;
    }
  };

  const unsubscribe = settings.subscribe(val => gameDataPath = val.game_data_path);

  const handleSubmit = async () => {
    await settings.saveSettings({game_data_path: gameDataPath});
  };

  onDestroy(() => unsubscribe());
</script>

<form on:submit|preventDefault={handleSubmit}>
 <div class="grid gap-6 mb-6 md:grid-cols-2">
    <div>
      <Label for="game_data_path" class="mb-2">Game Data Path</Label>
      <ButtonGroup class="w-full" size="sm">
        <InputAddon><FolderSolid/></InputAddon>
        <Input type="text" id="game_data_path" bind:value="{gameDataPath}" required />
        <Button on:click="{setGamePath}">Choose</Button>
      </ButtonGroup>
    </div>
  </div>
  <Button type="submit">Save</Button>
</form>
