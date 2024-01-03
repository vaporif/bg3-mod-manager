<script>
  import { invoke } from '@tauri-apps/api/tauri'
  import { Alert } from 'flowbite-svelte';
  let name = ''
  let greetMsg = ''

  async function greet() {
    greetMsg = await invoke('greet', { name })
  }
</script>
<div class="p-8">
  <Alert>
    <span class="font-medium">Info alert!</span>
    Change a few things up and try submitting again.
  </Alert>
</div>
<div>
  <input id="greet-input" placeholder="Enter a name..." bind:value="{name}" />
<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
  <button on:click="{greet}">Greet</button>
  <p>{greetMsg}</p>
</div>
