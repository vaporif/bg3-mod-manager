<script>
  import { Label, Input } from 'flowbite-svelte';
</script>

<div class="mb-6">
  <Label for="large-input" class="block mb-2">Large input</Label>
  <Input id="large-input" size="lg" placeholder="Large input" />
</div>
<div class="mb-6">
  <Label for="default-input" class="block mb-2">Default input</Label>
  <Input id="default-input" placeholder="Default input" />
</div>
<div class="mb-6">
  <Label for="small-input" class="block mb-2">Small input</Label>
  <Input id="small-input" size="sm" placeholder="Small input" />
</div>
