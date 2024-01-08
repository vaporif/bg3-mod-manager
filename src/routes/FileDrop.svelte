<script lang="ts">
  import { event } from '@tauri-apps/api'
  import { onDestroy } from 'svelte'

  let hoveredElement: HTMLElement | null = null;
  function handleMouseEnter(event: MouseEvent) {
    hoveredElement = event.target as HTMLElement;
  }

  function handleMouseLeave() {
    hoveredElement = null;
  }

  export let extensions: null | string[] = null
  export let onDrop: (files: string[], target: EventTarget | null) => void = () => {
  }

  export let onHover: (files: string[], target: EventTarget | null) => void = () => {
  }

  function getValidPaths(paths: string[]) {
    if (extensions === null) {
      return paths;
    }
    let validPaths = []
    for (const path of paths) {
      for (const ext of extensions) {
        if (path.endsWith('.' + ext)) {
          validPaths.push(path);
          break;
        }
      }
    }
    return validPaths;
  };

  const fileDropHover = event.listen('tauri://file-drop-hover', (e) => {
    let files = getValidPaths(e.payload as string[]);
    onHover(files, hoveredElement);
  });
  
  onDestroy(async () => {
    const unlisten = await fileDropHover
    unlisten()
  });

  const fileDropPromise = event.listen('tauri://file-drop', (e) => {
    const payload = e.payload as string[]
    let files = getValidPaths(payload)
    if (files.length > 0) {
      onDrop(files, hoveredElement)
    }
  });

  onDestroy(async () => {
    const unlisten = await fileDropPromise
    unlisten()
  });

</script>

<div style="display: contents;" aria-roledescription="drag-n-drop-tracker" on:mouseenter={handleMouseEnter} on:mouseleave={handleMouseLeave}>
  <slot />
</div>

