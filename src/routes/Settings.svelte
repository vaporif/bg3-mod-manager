<script>
  import { Input, Label, Button } from 'flowbite-svelte';
</script>

<form>
  <div>
    <Label for="game_data_path" class="mb-2">Game Data Path</Label>
    <Input type="text" id="game_data_path" required />
  </div>
  <Button type="submit">Save</Button>
</form>
